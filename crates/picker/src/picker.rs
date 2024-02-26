use editor::Editor;
use gpui::{
    div, list, prelude::*, uniform_list, AnyElement, AppContext, ClickEvent, DismissEvent,
    EventEmitter, FocusHandle, FocusableView, Length, ListState, Render, Task,
    UniformListScrollHandle, View, ViewContext, WindowContext,
};
use std::{sync::Arc, time::Duration};
use ui::{prelude::*, v_flex, Color, Divider, Label, ListItem, ListItemSpacing};
use workspace::ModalView;

enum ElementContainer {
    List(ListState),
    UniformList(UniformListScrollHandle),
}

pub struct Picker<D: PickerDelegate> {
    pub delegate: D,
    element_container: ElementContainer,
    editor: View<Editor>,
    pending_update_matches: Option<Task<()>>,
    confirm_on_update: Option<bool>,
    width: Option<Length>,
    max_height: Option<Length>,

    /// Whether the `Picker` is rendered as a self-contained modal.
    ///
    /// Set this to `false` when rendering the `Picker` as part of a larger modal.
    is_modal: bool,
}

pub trait PickerDelegate: Sized + 'static {
    type ListItem: IntoElement;
    fn match_count(&self) -> usize;
    fn selected_index(&self) -> usize;
    fn separators_after_indices(&self) -> Vec<usize> {
        Vec::new()
    }
    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>);

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str>;
    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()>;

    // Delegates that support this method (e.g. the CommandPalette) can chose to block on any background
    // work for up to `duration` to try and get a result synchronously.
    // This avoids a flash of an empty command-palette on cmd-shift-p, and lets workspace::SendKeystrokes
    // mostly work when dismissing a palette.
    fn finalize_update_matches(
        &mut self,
        _query: String,
        _duration: Duration,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> bool {
        false
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>);
    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>);

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem>;
    fn render_header(&self, _: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        None
    }
    fn render_footer(&self, _: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        None
    }
}

impl<D: PickerDelegate> FocusableView for Picker<D> {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

fn create_editor(placeholder: Arc<str>, cx: &mut WindowContext<'_>) -> View<Editor> {
    cx.new_view(|cx| {
        let mut editor = Editor::single_line(cx);
        editor.set_placeholder_text(placeholder, cx);
        editor
    })
}

impl<D: PickerDelegate> Picker<D> {
    /// A picker, which displays its matches using `gpui::uniform_list`, all matches should have the same height.
    /// If `PickerDelegate::render_match` can return items with different heights, use `Picker::list`.
    pub fn uniform_list(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        Self::new(delegate, cx, true)
    }

    /// A picker, which displays its matches using `gpui::list`, matches can have different heights.
    /// If `PickerDelegate::render_match` only returns items with the same height, use `Picker::uniform_list` as its implementation is optimized for that.
    pub fn list(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        Self::new(delegate, cx, false)
    }

    fn new(delegate: D, cx: &mut ViewContext<Self>, is_uniform: bool) -> Self {
        let editor = create_editor(delegate.placeholder_text(cx), cx);
        cx.subscribe(&editor, Self::on_input_editor_event).detach();
        let mut this = Self {
            delegate,
            editor,
            element_container: Self::create_element_container(is_uniform, cx),
            pending_update_matches: None,
            confirm_on_update: None,
            width: None,
            max_height: None,
            is_modal: true,
        };
        this.update_matches("".to_string(), cx);
        // give the delegate 4ms to renderthe first set of suggestions.
        this.delegate
            .finalize_update_matches("".to_string(), Duration::from_millis(4), cx);
        this
    }

    fn create_element_container(is_uniform: bool, cx: &mut ViewContext<Self>) -> ElementContainer {
        if is_uniform {
            ElementContainer::UniformList(UniformListScrollHandle::new())
        } else {
            let view = cx.view().downgrade();
            ElementContainer::List(ListState::new(
                0,
                gpui::ListAlignment::Top,
                px(1000.),
                move |ix, cx| {
                    view.upgrade()
                        .map(|view| {
                            view.update(cx, |this, cx| {
                                this.render_element(cx, ix).into_any_element()
                            })
                        })
                        .unwrap_or_else(|| div().into_any_element())
                },
            ))
        }
    }

    pub fn width(mut self, width: impl Into<gpui::Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn max_height(mut self, max_height: impl Into<gpui::Length>) -> Self {
        self.max_height = Some(max_height.into());
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.is_modal = modal;
        self
    }

    pub fn focus(&self, cx: &mut WindowContext) {
        self.editor.update(cx, |editor, cx| editor.focus(cx));
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = if index == count - 1 { 0 } else { index + 1 };
            self.delegate.set_selected_index(ix, cx);
            self.scroll_to_item_index(ix);
            cx.notify();
        }
    }

    fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = if index == 0 { count - 1 } else { index - 1 };
            self.delegate.set_selected_index(ix, cx);
            self.scroll_to_item_index(ix);
            cx.notify();
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.delegate.set_selected_index(0, cx);
            self.scroll_to_item_index(0);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.delegate.set_selected_index(count - 1, cx);
            self.scroll_to_item_index(count - 1);
            cx.notify();
        }
    }

    pub fn cycle_selection(&mut self, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        let index = self.delegate.selected_index();
        let new_index = if index + 1 == count { 0 } else { index + 1 };
        self.delegate.set_selected_index(new_index, cx);
        self.scroll_to_item_index(new_index);
        cx.notify();
    }

    pub fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        self.delegate.dismissed(cx);
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if self.pending_update_matches.is_some()
            && !self
                .delegate
                .finalize_update_matches(self.query(cx), Duration::from_millis(16), cx)
        {
            self.confirm_on_update = Some(false)
        } else {
            self.pending_update_matches.take();
            self.delegate.confirm(false, cx);
        }
    }

    fn secondary_confirm(&mut self, _: &menu::SecondaryConfirm, cx: &mut ViewContext<Self>) {
        if self.pending_update_matches.is_some()
            && !self
                .delegate
                .finalize_update_matches(self.query(cx), Duration::from_millis(16), cx)
        {
            self.confirm_on_update = Some(true)
        } else {
            self.delegate.confirm(true, cx);
        }
    }

    fn handle_click(&mut self, ix: usize, secondary: bool, cx: &mut ViewContext<Self>) {
        cx.stop_propagation();
        cx.prevent_default();
        self.delegate.set_selected_index(ix, cx);
        self.delegate.confirm(secondary, cx);
    }

    fn on_input_editor_event(
        &mut self,
        _: View<Editor>,
        event: &editor::EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::EditorEvent::BufferEdited => {
                let query = self.editor.read(cx).text(cx);
                self.update_matches(query, cx);
            }
            editor::EditorEvent::Blurred => {
                self.cancel(&menu::Cancel, cx);
            }
            _ => {}
        }
    }

    pub fn refresh(&mut self, cx: &mut ViewContext<Self>) {
        let query = self.editor.read(cx).text(cx);
        self.update_matches(query, cx);
    }

    pub fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) {
        let update = self.delegate.update_matches(query, cx);
        self.matches_updated(cx);
        self.pending_update_matches = Some(cx.spawn(|this, mut cx| async move {
            update.await;
            this.update(&mut cx, |this, cx| {
                this.matches_updated(cx);
            })
            .ok();
        }));
    }

    fn matches_updated(&mut self, cx: &mut ViewContext<Self>) {
        if let ElementContainer::List(state) = &mut self.element_container {
            state.reset(self.delegate.match_count());
        }

        let index = self.delegate.selected_index();
        self.scroll_to_item_index(index);
        self.pending_update_matches = None;
        if let Some(secondary) = self.confirm_on_update.take() {
            self.delegate.confirm(secondary, cx);
        }
        cx.notify();
    }

    pub fn query(&self, cx: &AppContext) -> String {
        self.editor.read(cx).text(cx)
    }

    pub fn set_query(&self, query: impl Into<Arc<str>>, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.set_text(query, cx));
    }

    fn scroll_to_item_index(&mut self, ix: usize) {
        match &mut self.element_container {
            ElementContainer::List(state) => state.scroll_to_reveal_item(ix),
            ElementContainer::UniformList(scroll_handle) => scroll_handle.scroll_to_item(ix),
        }
    }

    fn render_element(&self, cx: &mut ViewContext<Self>, ix: usize) -> impl IntoElement {
        div()
            .id(("item", ix))
            .on_click(cx.listener(move |this, event: &ClickEvent, cx| {
                this.handle_click(ix, event.down.modifiers.command, cx)
            }))
            .children(
                self.delegate
                    .render_match(ix, ix == self.delegate.selected_index(), cx),
            )
            .when(
                self.delegate.separators_after_indices().contains(&ix),
                |picker| {
                    picker
                        .border_color(cx.theme().colors().border_variant)
                        .border_b_1()
                        .pb(px(-1.0))
                },
            )
    }

    fn render_element_container(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        match &self.element_container {
            ElementContainer::UniformList(scroll_handle) => uniform_list(
                cx.view().clone(),
                "candidates",
                self.delegate.match_count(),
                move |picker, visible_range, cx| {
                    visible_range
                        .map(|ix| picker.render_element(cx, ix))
                        .collect()
                },
            )
            .py_2()
            .track_scroll(scroll_handle.clone())
            .into_any_element(),
            ElementContainer::List(state) => list(state.clone())
                .with_sizing_behavior(gpui::ListSizingBehavior::Infer)
                .py_2()
                .into_any_element(),
        }
    }
}

impl<D: PickerDelegate> EventEmitter<DismissEvent> for Picker<D> {}
impl<D: PickerDelegate> ModalView for Picker<D> {}

impl<D: PickerDelegate> Render for Picker<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let picker_editor = h_flex()
            .overflow_hidden()
            .flex_none()
            .h_9()
            .px_4()
            .child(self.editor.clone());

        div()
            .key_context("Picker")
            .size_full()
            .when_some(self.width, |el, width| el.w(width))
            .overflow_hidden()
            // This is a bit of a hack to remove the modal styling when we're rendering the `Picker`
            // as a part of a modal rather than the entire modal.
            //
            // We should revisit how the `Picker` is styled to make it more composable.
            .when(self.is_modal, |this| this.elevation_3(cx))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::secondary_confirm))
            .child(picker_editor)
            .child(Divider::horizontal())
            .when(self.delegate.match_count() > 0, |el| {
                el.child(
                    v_flex()
                        .flex_grow()
                        .max_h(self.max_height.unwrap_or(rems(18.).into()))
                        .overflow_hidden()
                        .children(self.delegate.render_header(cx))
                        .child(self.render_element_container(cx)),
                )
            })
            .when(self.delegate.match_count() == 0, |el| {
                el.child(
                    v_flex().flex_grow().py_2().child(
                        ListItem::new("empty_state")
                            .inset(true)
                            .spacing(ListItemSpacing::Sparse)
                            .disabled(true)
                            .child(Label::new("No matches").color(Color::Muted)),
                    ),
                )
            })
            .children(self.delegate.render_footer(cx))
    }
}
