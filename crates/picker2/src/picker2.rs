use editor::Editor;
use gpui::{
    div, prelude::*, uniform_list, AppContext, Component, Div, FocusHandle, FocusableView,
    MouseButton, Render, Task, UniformListScrollHandle, View, ViewContext, WindowContext,
};
use std::{cmp, sync::Arc};
use ui::{prelude::*, v_stack, Divider, Label, TextColor};

pub struct Picker<D: PickerDelegate> {
    pub delegate: D,
    scroll_handle: UniformListScrollHandle,
    editor: View<Editor>,
    pending_update_matches: Option<Task<()>>,
    confirm_on_update: Option<bool>,
}

pub trait PickerDelegate: Sized + 'static {
    type ListItem: Component<Picker<Self>>;

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
    ) -> Self::ListItem;
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

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        self.delegate.dismissed(cx);
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
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let editor::Event::BufferEdited = event {
            let query = self.editor.read(cx).text(cx);
            self.update_matches(query, cx);
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
}

impl<D: PickerDelegate> Render for Picker<D> {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .key_context("picker")
            .size_full()
            .elevation_2(cx)
            .on_action(Self::select_next)
            .on_action(Self::select_prev)
            .on_action(Self::select_first)
            .on_action(Self::select_last)
            .on_action(Self::cancel)
            .on_action(Self::confirm)
            .on_action(Self::secondary_confirm)
            .child(
                v_stack()
                    .py_0p5()
                    .px_1()
                    .child(div().px_1().py_0p5().child(self.editor.clone())),
            )
            .child(Divider::horizontal())
            .when(self.delegate.match_count() > 0, |el| {
                el.child(
                    v_stack()
                        .p_1()
                        .grow()
                        .child(
                            uniform_list("candidates", self.delegate.match_count(), {
                                move |this: &mut Self, visible_range, cx| {
                                    let selected_ix = this.delegate.selected_index();
                                    visible_range
                                        .map(|ix| {
                                            div()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    move |this: &mut Self, event, cx| {
                                                        this.handle_click(
                                                            ix,
                                                            event.modifiers.command,
                                                            cx,
                                                        )
                                                    },
                                                )
                                                .child(this.delegate.render_match(
                                                    ix,
                                                    ix == selected_ix,
                                                    cx,
                                                ))
                                        })
                                        .collect()
                                }
                            })
                            .track_scroll(self.scroll_handle.clone()),
                        )
                        .max_h_72()
                        .overflow_hidden(),
                )
            })
            .when(self.delegate.match_count() == 0, |el| {
                el.child(
                    v_stack().p_1().grow().child(
                        div()
                            .px_1()
                            .child(Label::new("No matches").color(TextColor::Muted)),
                    ),
                )
            })
    }
}
