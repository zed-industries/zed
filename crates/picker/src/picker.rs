use anyhow::Result;
use editor::{scroll::Autoscroll, Editor};
use gpui::{
    actions, div, impl_actions, list, prelude::*, uniform_list, AnyElement, AppContext, ClickEvent,
    DismissEvent, EventEmitter, FocusHandle, FocusableView, Length, ListSizingBehavior, ListState,
    MouseButton, MouseUpEvent, Render, Task, UniformListScrollHandle, View, ViewContext,
    WindowContext,
};
use head::Head;
use serde::Deserialize;
use std::{sync::Arc, time::Duration};
use ui::{prelude::*, v_flex, Color, Divider, Label, ListItem, ListItemSpacing};
use workspace::ModalView;

mod head;
pub mod highlighted_match_with_paths;

enum ElementContainer {
    List(ListState),
    UniformList(UniformListScrollHandle),
}

actions!(picker, [ConfirmCompletion]);

/// ConfirmInput is an alternative editor action which - instead of selecting active picker entry - treats pickers editor input literally,
/// performing some kind of action on it.
#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct ConfirmInput {
    pub secondary: bool,
}

impl_actions!(picker, [ConfirmInput]);

struct PendingUpdateMatches {
    delegate_update_matches: Option<Task<()>>,
    _task: Task<Result<()>>,
}

pub struct Picker<D: PickerDelegate> {
    pub delegate: D,
    element_container: ElementContainer,
    head: Head,
    pending_update_matches: Option<PendingUpdateMatches>,
    confirm_on_update: Option<bool>,
    width: Option<Length>,
    max_height: Option<Length>,

    /// Whether the `Picker` is rendered as a self-contained modal.
    ///
    /// Set this to `false` when rendering the `Picker` as part of a larger modal.
    is_modal: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PickerEditorPosition {
    #[default]
    /// Render the editor at the start of the picker. Usually the top
    Start,
    /// Render the editor at the end of the picker. Usually the bottom
    End,
}

pub trait PickerDelegate: Sized + 'static {
    type ListItem: IntoElement;

    fn match_count(&self) -> usize;
    fn selected_index(&self) -> usize;
    fn separators_after_indices(&self) -> Vec<usize> {
        Vec::new()
    }
    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>);
    // Allows binding some optional effect to when the selection changes.
    fn selected_index_changed(
        &self,
        _ix: usize,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Box<dyn Fn(&mut WindowContext) + 'static>> {
        None
    }
    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str>;
    fn no_matches_text(&self, _cx: &mut WindowContext) -> SharedString {
        "No matches".into()
    }
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

    /// Override if you want to have <enter> update the query instead of confirming.
    fn confirm_update_query(&mut self, _cx: &mut ViewContext<Picker<Self>>) -> Option<String> {
        None
    }
    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>);
    /// Instead of interacting with currently selected entry, treats editor input literally,
    /// performing some kind of action on it.
    fn confirm_input(&mut self, _secondary: bool, _: &mut ViewContext<Picker<Self>>) {}
    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>);
    fn should_dismiss(&self) -> bool {
        true
    }
    fn confirm_completion(&self, _query: String) -> Option<String> {
        None
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::default()
    }

    fn render_editor(&self, editor: &View<Editor>, _cx: &mut ViewContext<Picker<Self>>) -> Div {
        v_flex()
            .when(
                self.editor_position() == PickerEditorPosition::End,
                |this| this.child(Divider::horizontal()),
            )
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .h_9()
                    .px_3()
                    .child(editor.clone()),
            )
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| this.child(Divider::horizontal()),
            )
    }

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
        match &self.head {
            Head::Editor(editor) => editor.focus_handle(cx),
            Head::Empty(head) => head.focus_handle(cx),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContainerKind {
    List,
    UniformList,
}

impl<D: PickerDelegate> Picker<D> {
    /// A picker, which displays its matches using `gpui::uniform_list`, all matches should have the same height.
    /// The picker allows the user to perform search items by text.
    /// If `PickerDelegate::render_match` can return items with different heights, use `Picker::list`.
    pub fn uniform_list(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        let head = Head::editor(
            delegate.placeholder_text(cx),
            Self::on_input_editor_event,
            cx,
        );

        Self::new(delegate, ContainerKind::UniformList, head, cx)
    }

    /// A picker, which displays its matches using `gpui::uniform_list`, all matches should have the same height.
    /// If `PickerDelegate::render_match` can return items with different heights, use `Picker::list`.
    pub fn nonsearchable_uniform_list(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        let head = Head::empty(Self::on_empty_head_blur, cx);

        Self::new(delegate, ContainerKind::UniformList, head, cx)
    }

    /// A picker, which displays its matches using `gpui::list`, matches can have different heights.
    /// The picker allows the user to perform search items by text.
    /// If `PickerDelegate::render_match` only returns items with the same height, use `Picker::uniform_list` as its implementation is optimized for that.
    pub fn list(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        let head = Head::editor(
            delegate.placeholder_text(cx),
            Self::on_input_editor_event,
            cx,
        );

        Self::new(delegate, ContainerKind::List, head, cx)
    }

    fn new(delegate: D, container: ContainerKind, head: Head, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            delegate,
            head,
            element_container: Self::create_element_container(container, cx),
            pending_update_matches: None,
            confirm_on_update: None,
            width: None,
            max_height: Some(rems(18.).into()),
            is_modal: true,
        };
        this.update_matches("".to_string(), cx);
        // give the delegate 4ms to render the first set of suggestions.
        this.delegate
            .finalize_update_matches("".to_string(), Duration::from_millis(4), cx);
        this
    }

    fn create_element_container(
        container: ContainerKind,
        cx: &mut ViewContext<Self>,
    ) -> ElementContainer {
        match container {
            ContainerKind::UniformList => {
                ElementContainer::UniformList(UniformListScrollHandle::new())
            }
            ContainerKind::List => {
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
    }

    pub fn width(mut self, width: impl Into<gpui::Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn max_height(mut self, max_height: Option<gpui::Length>) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.is_modal = modal;
        self
    }

    pub fn focus(&self, cx: &mut WindowContext) {
        self.focus_handle(cx).focus(cx);
    }

    /// Handles the selecting an index, and passing the change to the delegate.
    /// If `scroll_to_index` is true, the new selected index will be scrolled into view.
    ///
    /// If some effect is bound to `selected_index_changed`, it will be executed.
    pub fn set_selected_index(
        &mut self,
        ix: usize,
        scroll_to_index: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let previous_index = self.delegate.selected_index();
        self.delegate.set_selected_index(ix, cx);
        let current_index = self.delegate.selected_index();

        if previous_index != current_index {
            if let Some(action) = self.delegate.selected_index_changed(ix, cx) {
                action(cx);
            }
            if scroll_to_index {
                self.scroll_to_item_index(ix);
            }
        }
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = if index == count - 1 { 0 } else { index + 1 };
            self.set_selected_index(ix, true, cx);
            cx.notify();
        }
    }

    fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = if index == 0 { count - 1 } else { index - 1 };
            self.set_selected_index(ix, true, cx);
            cx.notify();
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.set_selected_index(0, true, cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.set_selected_index(count - 1, true, cx);
            cx.notify();
        }
    }

    pub fn cycle_selection(&mut self, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        let index = self.delegate.selected_index();
        let new_index = if index + 1 == count { 0 } else { index + 1 };
        self.set_selected_index(new_index, true, cx);
        cx.notify();
    }

    pub fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        if self.delegate.should_dismiss() {
            self.delegate.dismissed(cx);
            cx.emit(DismissEvent);
        }
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
            self.do_confirm(false, cx);
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
            self.do_confirm(true, cx);
        }
    }

    fn confirm_input(&mut self, input: &ConfirmInput, cx: &mut ViewContext<Self>) {
        self.delegate.confirm_input(input.secondary, cx);
    }

    fn confirm_completion(&mut self, _: &ConfirmCompletion, cx: &mut ViewContext<Self>) {
        if let Some(new_query) = self.delegate.confirm_completion(self.query(cx)) {
            self.set_query(new_query, cx);
        } else {
            cx.propagate()
        }
    }

    fn handle_click(&mut self, ix: usize, secondary: bool, cx: &mut ViewContext<Self>) {
        cx.stop_propagation();
        cx.prevent_default();
        self.set_selected_index(ix, false, cx);
        self.do_confirm(secondary, cx)
    }

    fn do_confirm(&mut self, secondary: bool, cx: &mut ViewContext<Self>) {
        if let Some(update_query) = self.delegate.confirm_update_query(cx) {
            self.set_query(update_query, cx);
            self.delegate.set_selected_index(0, cx);
        } else {
            self.delegate.confirm(secondary, cx)
        }
    }

    fn on_input_editor_event(
        &mut self,
        _: View<Editor>,
        event: &editor::EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Head::Editor(ref editor) = &self.head else {
            panic!("unexpected call");
        };
        match event {
            editor::EditorEvent::BufferEdited => {
                let query = editor.read(cx).text(cx);
                self.update_matches(query, cx);
            }
            editor::EditorEvent::Blurred => {
                self.cancel(&menu::Cancel, cx);
            }
            _ => {}
        }
    }

    fn on_empty_head_blur(&mut self, cx: &mut ViewContext<Self>) {
        let Head::Empty(_) = &self.head else {
            panic!("unexpected call");
        };
        self.cancel(&menu::Cancel, cx);
    }

    pub fn refresh(&mut self, cx: &mut ViewContext<Self>) {
        let query = self.query(cx);
        self.update_matches(query, cx);
    }

    pub fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) {
        let delegate_pending_update_matches = self.delegate.update_matches(query, cx);

        self.matches_updated(cx);
        // This struct ensures that we can synchronously drop the task returned by the
        // delegate's `update_matches` method and the task that the picker is spawning.
        // If we simply capture the delegate's task into the picker's task, when the picker's
        // task gets synchronously dropped, the delegate's task would keep running until
        // the picker's task has a chance of being scheduled, because dropping a task happens
        // asynchronously.
        self.pending_update_matches = Some(PendingUpdateMatches {
            delegate_update_matches: Some(delegate_pending_update_matches),
            _task: cx.spawn(|this, mut cx| async move {
                let delegate_pending_update_matches = this.update(&mut cx, |this, _| {
                    this.pending_update_matches
                        .as_mut()
                        .unwrap()
                        .delegate_update_matches
                        .take()
                        .unwrap()
                })?;
                delegate_pending_update_matches.await;
                this.update(&mut cx, |this, cx| {
                    this.matches_updated(cx);
                })
            }),
        });
    }

    fn matches_updated(&mut self, cx: &mut ViewContext<Self>) {
        if let ElementContainer::List(state) = &mut self.element_container {
            state.reset(self.delegate.match_count());
        }

        let index = self.delegate.selected_index();
        self.scroll_to_item_index(index);
        self.pending_update_matches = None;
        if let Some(secondary) = self.confirm_on_update.take() {
            self.do_confirm(secondary, cx);
        }
        cx.notify();
    }

    pub fn query(&self, cx: &AppContext) -> String {
        match &self.head {
            Head::Editor(editor) => editor.read(cx).text(cx),
            Head::Empty(_) => "".to_string(),
        }
    }

    pub fn set_query(&self, query: impl Into<Arc<str>>, cx: &mut ViewContext<Self>) {
        if let Head::Editor(ref editor) = &self.head {
            editor.update(cx, |editor, cx| {
                editor.set_text(query, cx);
                let editor_offset = editor.buffer().read(cx).len(cx);
                editor.change_selections(Some(Autoscroll::Next), cx, |s| {
                    s.select_ranges(Some(editor_offset..editor_offset))
                });
            });
        }
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
            .cursor_pointer()
            .on_click(cx.listener(move |this, event: &ClickEvent, cx| {
                this.handle_click(ix, event.down.modifiers.secondary(), cx)
            }))
            // As of this writing, GPUI intercepts `ctrl-[mouse-event]`s on macOS
            // and produces right mouse button events. This matches platforms norms
            // but means that UIs which depend on holding ctrl down (such as the tab
            // switcher) can't be clicked on. Hence, this handler.
            .on_mouse_up(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseUpEvent, cx| {
                    // We specficially want to use the platform key here, as
                    // ctrl will already be held down for the tab switcher.
                    this.handle_click(ix, event.modifiers.platform, cx)
                }),
            )
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
                        .py(px(-1.0))
                },
            )
    }

    fn render_element_container(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let sizing_behavior = if self.max_height.is_some() {
            ListSizingBehavior::Infer
        } else {
            ListSizingBehavior::Auto
        };
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
            .with_sizing_behavior(sizing_behavior)
            .flex_grow()
            .py_1()
            .track_scroll(scroll_handle.clone())
            .into_any_element(),
            ElementContainer::List(state) => list(state.clone())
                .with_sizing_behavior(sizing_behavior)
                .flex_grow()
                .py_2()
                .into_any_element(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn logical_scroll_top_index(&self) -> usize {
        match &self.element_container {
            ElementContainer::List(state) => state.logical_scroll_top().item_ix,
            ElementContainer::UniformList(scroll_handle) => {
                scroll_handle.logical_scroll_top_index()
            }
        }
    }
}

impl<D: PickerDelegate> EventEmitter<DismissEvent> for Picker<D> {}
impl<D: PickerDelegate> ModalView for Picker<D> {}

impl<D: PickerDelegate> Render for Picker<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let editor_position = self.delegate.editor_position();

        v_flex()
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
            .on_action(cx.listener(Self::confirm_completion))
            .on_action(cx.listener(Self::confirm_input))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::Start {
                        Some(self.delegate.render_editor(&editor.clone(), cx))
                    } else {
                        None
                    }
                }
                Head::Empty(empty_head) => Some(div().child(empty_head.clone())),
            })
            .when(self.delegate.match_count() > 0, |el| {
                el.child(
                    v_flex()
                        .flex_grow()
                        .when_some(self.max_height, |div, max_h| div.max_h(max_h))
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
                            .child(
                                Label::new(self.delegate.no_matches_text(cx)).color(Color::Muted),
                            ),
                    ),
                )
            })
            .children(self.delegate.render_footer(cx))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::End {
                        Some(self.delegate.render_editor(&editor.clone(), cx))
                    } else {
                        None
                    }
                }
                Head::Empty(empty_head) => Some(div().child(empty_head.clone())),
            })
    }
}
