use editor::Editor;
use gpui::{
    div, prelude::*, rems, uniform_list, AnyElement, AppContext, DismissEvent, Div, EventEmitter,
    FocusHandle, FocusableView, MouseButton, MouseDownEvent, Render, Task, UniformListScrollHandle,
    View, ViewContext, WindowContext,
};
use std::{cmp, sync::Arc};
use ui::{prelude::*, v_stack, Color, Divider, Label};

pub struct Picker<D: PickerDelegate> {
    pub delegate: D,
    scroll_handle: UniformListScrollHandle,
    editor: View<Editor>,
    pending_update_matches: Option<Task<()>>,
    confirm_on_update: Option<bool>,
}

pub trait PickerDelegate: Sized + 'static {
    type ListItem: IntoElement;
    fn match_count(&self) -> usize;
    fn selected_index(&self) -> usize;
    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>);

    fn placeholder_text(&self) -> Arc<str>;
    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()>;

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>);
    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>);

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem>;
}

impl<D: PickerDelegate> FocusableView for Picker<D> {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl<D: PickerDelegate> Picker<D> {
    pub fn new(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        let editor = cx.build_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text(delegate.placeholder_text(), cx);
            editor
        });
        cx.subscribe(&editor, Self::on_input_editor_event).detach();
        let mut this = Self {
            delegate,
            editor,
            scroll_handle: UniformListScrollHandle::new(),
            pending_update_matches: None,
            confirm_on_update: None,
        };
        this.update_matches("".to_string(), cx);
        this
    }

    pub fn focus(&self, cx: &mut WindowContext) {
        self.editor.update(cx, |editor, cx| editor.focus(cx));
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = cmp::min(index + 1, count - 1);
            self.delegate.set_selected_index(ix, cx);
            self.scroll_handle.scroll_to_item(ix);
            cx.notify();
        }
    }

    fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = index.saturating_sub(1);
            self.delegate.set_selected_index(ix, cx);
            self.scroll_handle.scroll_to_item(ix);
            cx.notify();
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.delegate.set_selected_index(0, cx);
            self.scroll_handle.scroll_to_item(0);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.delegate.set_selected_index(count - 1, cx);
            self.scroll_handle.scroll_to_item(count - 1);
            cx.notify();
        }
    }

    pub fn cycle_selection(&mut self, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        let index = self.delegate.selected_index();
        let new_index = if index + 1 == count { 0 } else { index + 1 };
        self.delegate.set_selected_index(new_index, cx);
        self.scroll_handle.scroll_to_item(new_index);
        cx.notify();
    }

    pub fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        self.delegate.dismissed(cx);
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if self.pending_update_matches.is_some() {
            self.confirm_on_update = Some(false)
        } else {
            self.delegate.confirm(false, cx);
        }
    }

    fn secondary_confirm(&mut self, _: &menu::SecondaryConfirm, cx: &mut ViewContext<Self>) {
        if self.pending_update_matches.is_some() {
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
        let index = self.delegate.selected_index();
        self.scroll_handle.scroll_to_item(index);
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
}

impl<D: PickerDelegate> EventEmitter<DismissEvent> for Picker<D> {}

impl<D: PickerDelegate> Render for Picker<D> {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let picker_editor = h_stack()
            .overflow_hidden()
            .flex_none()
            .h_9()
            .px_3()
            .child(self.editor.clone());

        let empty_state = div().p_1().child(
            h_stack()
                // TODO: This number matches the height of the uniform list items.
                // Align these two with a less magic number.
                .h(rems(1.4375))
                .px_2()
                .child(Label::new("No matches").color(Color::Muted)),
        );

        div()
            .key_context("picker")
            .size_full()
            .overflow_hidden()
            .elevation_3(cx)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::secondary_confirm))
            .child(
                picker_editor
            )
            .child(Divider::horizontal())
            .when(self.delegate.match_count() > 0, |el| {
                el.child(
                    v_stack()
                        .flex_grow()
                        .child(
                            uniform_list(
                                cx.view().clone(),
                                "candidates",
                                self.delegate.match_count(),
                                {
                                    let selected_index = self.delegate.selected_index();

                                    move |picker, visible_range, cx| {
                                        visible_range
                                            .map(|ix| {
                                                div()
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(move |this, event: &MouseDownEvent, cx| {
                                                            this.handle_click(
                                                                ix,
                                                                event.modifiers.command,
                                                                cx,
                                                            )
                                                        }),
                                                    )
                                                    .children(picker.delegate.render_match(
                                                        ix,
                                                        ix == selected_index,
                                                        cx,
                                                    ))
                                            })
                                            .collect()
                                    }
                                },
                            )
                            .track_scroll(self.scroll_handle.clone())
                            .p_1()
                        )
                        .max_h_72()
                        .overflow_hidden(),
                )
            })
            .when(self.delegate.match_count() == 0, |el| {
                el.child(
                    empty_state
                )
            })
    }
}

pub fn simple_picker_match(
    selected: bool,
    cx: &mut WindowContext,
    children: impl FnOnce(&mut WindowContext) -> AnyElement,
) -> AnyElement {
    let colors = cx.theme().colors();

    div()
        .px_1()
        .text_color(colors.text)
        .text_ui()
        .bg(colors.ghost_element_background)
        .rounded_md()
        .when(selected, |this| this.bg(colors.ghost_element_selected))
        .hover(|this| this.bg(colors.ghost_element_hover))
        .child((children)(cx))
        .into_any()
}
