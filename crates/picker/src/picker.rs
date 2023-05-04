use editor::Editor;
use gpui::{
    elements::*,
    geometry::vector::{vec2f, Vector2F},
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, MouseButton},
    AnyElement, AnyViewHandle, AppContext, Axis, Entity, MouseState, Task, View, ViewContext,
    ViewHandle,
};
use menu::{Cancel, Confirm, SelectFirst, SelectLast, SelectNext, SelectPrev};
use parking_lot::Mutex;
use std::{cmp, sync::Arc};
use util::ResultExt;
use workspace::Modal;

pub enum PickerEvent {
    Dismiss,
}

pub struct Picker<D: PickerDelegate> {
    delegate: D,
    query_editor: ViewHandle<Editor>,
    list_state: UniformListState,
    max_size: Vector2F,
    theme: Arc<Mutex<Box<dyn Fn(&theme::Theme) -> theme::Picker>>>,
    confirmed: bool,
    pending_update_matches: Task<Option<()>>,
}

pub trait PickerDelegate: Sized + 'static {
    fn placeholder_text(&self) -> Arc<str>;
    fn match_count(&self) -> usize;
    fn selected_index(&self) -> usize;
    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>);
    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()>;
    fn confirm(&mut self, cx: &mut ViewContext<Picker<Self>>);
    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>);
    fn render_match(
        &self,
        ix: usize,
        state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> AnyElement<Picker<Self>>;
    fn center_selection_after_match_updates(&self) -> bool {
        false
    }
}

impl<D: PickerDelegate> Entity for Picker<D> {
    type Event = PickerEvent;
}

impl<D: PickerDelegate> View for Picker<D> {
    fn ui_name() -> &'static str {
        "Picker"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = (self.theme.lock())(&cx.global::<settings::Settings>().theme);
        let query = self.query(cx);
        let match_count = self.delegate.match_count();

        let container_style;
        let editor_style;
        if query.is_empty() && match_count == 0 {
            container_style = theme.empty_container;
            editor_style = theme.empty_input_editor.container;
        } else {
            container_style = theme.container;
            editor_style = theme.input_editor.container;
        };

        Flex::new(Axis::Vertical)
            .with_child(
                ChildView::new(&self.query_editor, cx)
                    .contained()
                    .with_style(editor_style),
            )
            .with_children(if match_count == 0 {
                if query.is_empty() {
                    None
                } else {
                    Some(
                        Label::new("No matches", theme.no_matches.label.clone())
                            .contained()
                            .with_style(theme.no_matches.container)
                            .into_any(),
                    )
                }
            } else {
                Some(
                    UniformList::new(
                        self.list_state.clone(),
                        match_count,
                        cx,
                        move |this, mut range, items, cx| {
                            let selected_ix = this.delegate.selected_index();
                            range.end = cmp::min(range.end, this.delegate.match_count());
                            items.extend(range.map(move |ix| {
                                MouseEventHandler::<D, _>::new(ix, cx, |state, cx| {
                                    this.delegate.render_match(ix, state, ix == selected_ix, cx)
                                })
                                // Capture mouse events
                                .on_down(MouseButton::Left, |_, _, _| {})
                                .on_up(MouseButton::Left, |_, _, _| {})
                                .on_click(MouseButton::Left, move |_, picker, cx| {
                                    picker.select_index(ix, cx);
                                })
                                .with_cursor_style(CursorStyle::PointingHand)
                                .into_any()
                            }));
                        },
                    )
                    .contained()
                    .with_margin_top(6.0)
                    .flex(1., false)
                    .into_any(),
                )
            })
            .contained()
            .with_style(container_style)
            .constrained()
            .with_max_width(self.max_size.x())
            .with_max_height(self.max_size.y())
            .into_any_named("picker")
    }

    fn update_keymap_context(&self, keymap: &mut KeymapContext, _: &AppContext) {
        Self::reset_to_default_keymap_context(keymap);
        keymap.add_identifier("menu");
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.query_editor);
        }
    }
}

impl<D: PickerDelegate> Modal for Picker<D> {
    fn dismiss_on_event(event: &Self::Event) -> bool {
        matches!(event, PickerEvent::Dismiss)
    }
}

impl<D: PickerDelegate> Picker<D> {
    pub fn init(cx: &mut AppContext) {
        cx.add_action(Self::select_first);
        cx.add_action(Self::select_last);
        cx.add_action(Self::select_next);
        cx.add_action(Self::select_prev);
        cx.add_action(Self::confirm);
        cx.add_action(Self::cancel);
    }

    pub fn new(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        let theme = Arc::new(Mutex::new(
            Box::new(|theme: &theme::Theme| theme.picker.clone())
                as Box<dyn Fn(&theme::Theme) -> theme::Picker>,
        ));
        let placeholder_text = delegate.placeholder_text();
        let query_editor = cx.add_view({
            let picker_theme = theme.clone();
            |cx| {
                let mut editor = Editor::single_line(
                    Some(Arc::new(move |theme| {
                        (picker_theme.lock())(theme).input_editor.clone()
                    })),
                    cx,
                );
                editor.set_placeholder_text(placeholder_text, cx);
                editor
            }
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();
        let mut this = Self {
            query_editor,
            list_state: Default::default(),
            delegate,
            max_size: vec2f(540., 420.),
            theme,
            confirmed: false,
            pending_update_matches: Task::ready(None),
        };
        this.update_matches(String::new(), cx);
        this
    }

    pub fn with_max_size(mut self, width: f32, height: f32) -> Self {
        self.max_size = vec2f(width, height);
        self
    }

    pub fn with_theme<F>(self, theme: F) -> Self
    where
        F: 'static + Fn(&theme::Theme) -> theme::Picker,
    {
        *self.theme.lock() = Box::new(theme);
        self
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn query(&self, cx: &AppContext) -> String {
        self.query_editor.read(cx).text(cx)
    }

    pub fn set_query(&self, query: impl Into<Arc<str>>, cx: &mut ViewContext<Self>) {
        self.query_editor
            .update(cx, |editor, cx| editor.set_text(query, cx));
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::BufferEdited { .. } => self.update_matches(self.query(cx), cx),
            editor::Event::Blurred if !self.confirmed => {
                self.dismiss(cx);
            }
            _ => {}
        }
    }

    pub fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) {
        let update = self.delegate.update_matches(query, cx);
        self.matches_updated(cx);
        self.pending_update_matches = cx.spawn(|this, mut cx| async move {
            update.await;
            this.update(&mut cx, |this, cx| this.matches_updated(cx))
                .log_err()
        });
    }

    fn matches_updated(&mut self, cx: &mut ViewContext<Self>) {
        let index = self.delegate.selected_index();
        let target = if self.delegate.center_selection_after_match_updates() {
            ScrollTarget::Center(index)
        } else {
            ScrollTarget::Show(index)
        };
        self.list_state.scroll_to(target);
        cx.notify();
    }

    pub fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        if self.delegate.match_count() > 0 {
            self.delegate.set_selected_index(0, cx);
            self.list_state.scroll_to(ScrollTarget::Show(0));
        }

        cx.notify();
    }

    pub fn select_index(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if self.delegate.match_count() > 0 {
            self.confirmed = true;
            self.delegate.set_selected_index(index, cx);
            self.delegate.confirm(cx);
        }
    }

    pub fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        let match_count = self.delegate.match_count();
        if match_count > 0 {
            let index = match_count - 1;
            self.delegate.set_selected_index(index, cx);
            self.list_state.scroll_to(ScrollTarget::Show(index));
        }
        cx.notify();
    }

    pub fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        let next_index = self.delegate.selected_index() + 1;
        if next_index < self.delegate.match_count() {
            self.delegate.set_selected_index(next_index, cx);
            self.list_state.scroll_to(ScrollTarget::Show(next_index));
        }

        cx.notify();
    }

    pub fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        let mut selected_index = self.delegate.selected_index();
        if selected_index > 0 {
            selected_index -= 1;
            self.delegate.set_selected_index(selected_index, cx);
            self.list_state
                .scroll_to(ScrollTarget::Show(selected_index));
        }

        cx.notify();
    }

    pub fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        self.confirmed = true;
        self.delegate.confirm(cx);
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        self.dismiss(cx);
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(PickerEvent::Dismiss);
        self.delegate.dismissed(cx);
    }
}
