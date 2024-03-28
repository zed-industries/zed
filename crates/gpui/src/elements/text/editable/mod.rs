mod actions;
mod blink_manager;
// mod history;
mod selection;
mod transaction;

use std::{any::TypeId, ops::Range, sync::Arc, time::Duration};

use crate::{
    outline, rgb, Action, Bounds, ClipboardItem, Context, CursorStyle, DispatchPhase, Element,
    ElementContext, ElementId, FocusHandle, HighlightStyle, Hitbox, Hsla, InputHandler,
    InteractiveText, IntoElement, LayoutId, Model, MouseMoveEvent, Pixels, SharedString, Size,
    StyledText, TextState, WindowContext,
};

use actions::{
    Backspace, Copy, Cut, Delete, MoveDown, MoveLeft, MoveRight, MoveToBeginning, MoveToEnd,
    MoveToNextWordEnd, MoveToPreviousWordStart, MoveUp, Newline, Paste, Redo, SelectAll,
    SelectLeft, SelectRight, SelectToBeginning, SelectToEnd, SelectToNextWordEnd,
    SelectToPreviousWordStart, Tab, TabPrev, Undo,
};
use blink_manager::BlinkManager;
use parking_lot::Mutex;
use selection::Selection;

use self::transaction::{History, ReplaceTextInRange, ReplaceTextInRangeAndSelect, Transaction};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

/// Construct an element with editable text.
pub fn editable_text(id: ElementId, value: Model<String>) -> EditableText {
    EditableText {
        id,
        value,
        kind: Kind::Singline,
        cursor_color: rgb(0x348feb).into(),
        selection_color: None,
        enter_listener: None,
        focus_next_listener: None,
        focus_prev_listener: None,
    }
}

/// Some editable text.
pub struct EditableText {
    id: ElementId,
    value: Model<String>,
    kind: Kind,
    cursor_color: Hsla,
    selection_color: Option<Hsla>,
    enter_listener: Option<Arc<dyn Fn(&mut WindowContext<'_>)>>,
    focus_next_listener: Option<Arc<dyn Fn(&mut WindowContext<'_>)>>,
    focus_prev_listener: Option<Arc<dyn Fn(&mut WindowContext<'_>)>>,
}

#[derive(Clone)]
/// The kind of editable text element, either single or multiline.
pub enum Kind {
    /// This editable text can only be one line. Like an input of type text on the web.
    // TODO: there is nothing done to handle multiline text that is passed to a single line
    // element. Currently this flag just changes functionality like calling `on_enter` instead of
    // adding a newline to the content.
    Singline,
    /// This editable text can have multiple line. Like a textarea on the web.
    Multiline,
}

impl EditableText {
    /// Set the editable text's cursor color.
    pub fn cursor_color(mut self, cursor_color: impl Into<Hsla>) -> Self {
        self.cursor_color = cursor_color.into();
        self
    }

    /// Set the editable text's selection background color.
    pub fn selection_color(mut self, selection_color: impl Into<Hsla>) -> Self {
        self.selection_color = Some(selection_color.into());
        self
    }

    /// Set the editable text to multiline.
    pub fn multiline(mut self) -> Self {
        self.kind = Kind::Multiline;
        self
    }

    /// A function to be called whenever `enter` is pressed on a single line editable text element.
    pub fn on_enter(mut self, listener: impl Fn(&mut WindowContext<'_>) + 'static) -> Self {
        self.enter_listener = Some(Arc::new(listener));
        self
    }

    /// A function to be called whenever `tab` is pressed on a single line editable text element.
    pub fn on_focus_next(mut self, listener: impl Fn(&mut WindowContext<'_>) + 'static) -> Self {
        self.focus_next_listener = Some(Arc::new(listener));
        self
    }

    /// A function to be called whenever `shift-tab` is pressed on a single line editable text element.
    pub fn on_focus_prev(mut self, listener: impl Fn(&mut WindowContext<'_>) + 'static) -> Self {
        self.focus_prev_listener = Some(Arc::new(listener));
        self
    }
}

impl IntoElement for EditableText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for EditableText {
    type BeforeLayout = (InteractiveText, TextState);
    type AfterLayout = Hitbox;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        cx.with_element_state::<EditableTextState, _>(Some(self.id.clone()), |state, cx| {
            let state = state.unwrap().unwrap_or_else(|| EditableTextState::new(cx));

            let mut styled_text = self.render_text(&state, cx);

            let (layout_id, text_state) = styled_text.before_layout(cx);

            ((layout_id, (styled_text, text_state)), Some(state))
        })
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<crate::Pixels>,
        (styled_text, text_state): &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Self::AfterLayout {
        styled_text.after_layout(bounds, text_state, cx);
        cx.insert_hitbox(bounds, false)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        (text, text_state): &mut Self::BeforeLayout,
        hitbox: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        cx.with_element_state::<EditableTextState, _>(Some(self.id.clone()), |state, cx| {
            let mut state = state.unwrap().unwrap();
            state.bounds = Some(bounds);
            state.text_state = Some(Arc::new(text_state.clone()));

            cx.set_focus_handle(&state.focus_handle);
            cx.set_key_context(actions::key_context());
            cx.handle_input(&state.focus_handle, self.into_input_handler(&state));

            self.register_paint_mouse_listeners(&hitbox, &state, cx);
            self.register_paint_action_listeners(
                &self.value.read(cx).to_owned().into(),
                &state,
                cx,
            );

            text.paint(bounds, text_state, hitbox, cx);

            {
                let selection = state.selection.read(cx);

                if selection.is_some() {
                    if !state.focus_handle.is_focused(cx) {
                        state.blur(cx);
                    } else {
                        if state.blink_manager.read(cx).visible() {
                            let selection = selection.as_ref().unwrap();

                            let index = selection.position();

                            if let Some(origin) = text_state.position_for_index(bounds, index) {
                                cx.paint_quad(outline(
                                    Bounds {
                                        origin,
                                        size: Size {
                                            width: Pixels(2.),
                                            height: text_state.line_height(),
                                        },
                                    },
                                    self.cursor_color,
                                ));
                            }
                        }
                    }
                }
            }

            ((), Some(state))
        })
    }
}

impl EditableText {
    fn render_text(&self, state: &EditableTextState, cx: &mut ElementContext) -> InteractiveText {
        let selection_color = self.selection_color.unwrap_or_else(|| {
            let mut cursor_color = self.cursor_color;
            cursor_color.fade_out(0.5);
            cursor_color
        });

        InteractiveText::new(
            self.id.clone(),
            StyledText::new(format!("{} ", self.value.read(cx))).with_highlights(
                &cx.text_style(),
                state
                    .selection
                    .read(cx)
                    .clone()
                    .filter(Selection::not_empty)
                    .map(|selection| {
                        (
                            selection.span,
                            HighlightStyle {
                                background_color: Some(selection_color),
                                ..Default::default()
                            },
                        )
                    }),
            ),
        )
        .cursor_style(CursorStyle::IBeam)
        .on_selection_change({
            let state = state.clone();

            move |from, to, cx| {
                state.on_selection_change(from, to, cx);
            }
        })
    }
}

#[derive(Clone)]
struct EditableTextState {
    selection: Model<Option<Selection>>,
    // TODO: confirm this should be separate from selection
    marked: Model<Option<Range<usize>>>,
    history: Model<History>,
    blink_manager: Model<BlinkManager>,
    focus_handle: FocusHandle,
    text_state: Option<Arc<TextState>>,
    bounds: Option<Bounds<Pixels>>,
}

impl EditableTextState {
    fn new(cx: &mut ElementContext) -> Self {
        let blink_manager = cx.new_model(|cx| BlinkManager::new(CURSOR_BLINK_INTERVAL, cx));
        let selection = cx.new_model(|_| None);

        // TODO: make selection a model too so we can show cursor when it updates via observe.
        cx.observe(&blink_manager, |_, cx| cx.refresh()).detach();
        cx.observe(&selection, {
            let blink_manager = blink_manager.clone();

            move |_, cx| {
                blink_manager.update(cx, |this, cx| {
                    this.show_cursor(cx);
                });
            }
        })
        .detach();

        actions::bind_keys(cx);

        Self {
            blink_manager,
            selection,
            marked: cx.new_model(|_| None),
            history: cx.new_model(|_| History::default()),
            focus_handle: cx.focus_handle(),
            text_state: None,
            bounds: None,
        }
    }

    fn focus(&self, cx: &mut WindowContext) {
        cx.focus(&self.focus_handle);

        self.blink_manager.update(cx, |this, cx| {
            this.enable(cx);
        });
    }

    fn blur(&self, cx: &mut WindowContext) {
        self.blink_manager.update(cx, |this, cx| {
            this.disable(cx);
        });

        self.selection.update(cx, |this, cx| {
            *this = None;
            cx.notify();
        });
    }

    fn on_selection_change(&self, from_index: usize, to_index: usize, cx: &mut WindowContext) {
        self.selection.update(cx, |this, cx| {
            *this = Some(Selection::new(from_index, to_index));
            cx.notify();
        });
    }
}

impl EditableText {
    fn register_paint_mouse_listeners(
        &self,
        hitbox: &Hitbox,
        state: &EditableTextState,
        cx: &mut ElementContext,
    ) {
        cx.on_mouse_event({
            let hitbox = hitbox.clone();
            let state = state.clone();

            move |_event: &MouseMoveEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && hitbox.is_hovered(cx) {
                    state.focus(cx);
                }
            }
        });
    }

    fn register_paint_action_listeners(
        &self,
        text: &SharedString,
        state: &EditableTextState,
        cx: &mut ElementContext,
    ) {
        self.register_movement_action(cx, text, state, EditableTextState::move_down);
        self.register_movement_action(cx, text, state, EditableTextState::move_left);
        self.register_movement_action(cx, text, state, EditableTextState::move_right);
        self.register_movement_action(cx, text, state, EditableTextState::move_up);
        self.register_movement_action(cx, text, state, EditableTextState::move_to_beginning);
        self.register_movement_action(cx, text, state, EditableTextState::move_to_end);
        self.register_movement_action(cx, text, state, EditableTextState::move_to_next_word_end);
        self.register_movement_action(
            cx,
            text,
            state,
            EditableTextState::move_to_previous_word_start,
        );
        self.register_movement_action(cx, text, state, EditableTextState::select_all);
        self.register_movement_action(cx, text, state, EditableTextState::select_left);
        self.register_movement_action(cx, text, state, EditableTextState::select_right);
        self.register_movement_action(cx, text, state, EditableTextState::select_to_beginning);
        self.register_movement_action(cx, text, state, EditableTextState::select_to_end);
        self.register_movement_action(cx, text, state, EditableTextState::select_to_next_word_end);
        self.register_movement_action(
            cx,
            text,
            state,
            EditableTextState::select_to_previous_word_start,
        );

        self.register_input_action(cx, state, EditableTextInputHandler::backspace);
        self.register_input_action(cx, state, EditableTextInputHandler::copy);
        self.register_input_action(cx, state, EditableTextInputHandler::cut);
        self.register_input_action(cx, state, EditableTextInputHandler::newline);
        self.register_input_action(cx, state, EditableTextInputHandler::paste);
        self.register_input_action(cx, state, EditableTextInputHandler::redo);
        self.register_input_action(cx, state, EditableTextInputHandler::tab);
        self.register_input_action(cx, state, EditableTextInputHandler::tab_prev);
        self.register_input_action(cx, state, EditableTextInputHandler::undo);
    }

    fn register_movement_action<T: Action>(
        &self,
        cx: &mut WindowContext,
        text: &SharedString,
        state: &EditableTextState,
        listener: impl Fn(&EditableTextState, &T, &SharedString, &mut WindowContext) + 'static,
    ) {
        cx.on_action(TypeId::of::<T>(), {
            let text = text.clone();
            let state = state.clone();

            move |action, phase, cx| {
                let action = action.downcast_ref().unwrap();

                if phase == DispatchPhase::Bubble {
                    listener(&state, action, &text, cx);

                    cx.refresh();
                }
            }
        });
    }

    fn register_input_action<T: Action>(
        &self,
        cx: &mut WindowContext,
        state: &EditableTextState,
        listener: impl Fn(&mut EditableTextInputHandler, &T, &mut WindowContext) + 'static,
    ) {
        cx.on_action(TypeId::of::<T>(), {
            let input_handler = Arc::new(Mutex::new(self.into_input_handler(state)));

            move |action, phase, cx| {
                let action = action.downcast_ref().unwrap();

                if phase == DispatchPhase::Bubble {
                    listener(&mut input_handler.lock(), action, cx);

                    cx.refresh();
                }
            }
        });
    }
}

macro_rules! impl_selection_modifier {
    {$fn_name:ident, $action:ident} => {
        fn $fn_name(&self, _: &$action, text: &SharedString, cx: &mut WindowContext) {
            self.selection.update(cx, |this, cx| {
                if let Some(this) = this.as_mut() {
                    this.$fn_name(text.as_ref());
                    cx.notify();
                }
            });
        }
    };
}

macro_rules! impl_all_selection_modifiers {
    ($(($fn_name:ident, $action:ident)),+) => {
        impl EditableTextState {
            $(
                impl_selection_modifier!{$fn_name, $action}
            )+
        }
    };
}

impl_all_selection_modifiers!(
    (move_left, MoveLeft),
    (move_right, MoveRight),
    (move_to_beginning, MoveToBeginning),
    (move_to_end, MoveToEnd),
    (move_to_next_word_end, MoveToNextWordEnd),
    (move_to_previous_word_start, MoveToPreviousWordStart),
    (select_all, SelectAll),
    (select_left, SelectLeft),
    (select_right, SelectRight),
    (select_to_beginning, SelectToBeginning),
    (select_to_end, SelectToEnd),
    (select_to_next_word_end, SelectToNextWordEnd),
    (select_to_previous_word_start, SelectToPreviousWordStart)
);

impl EditableTextState {
    fn move_down(&self, _: &MoveDown, text: &SharedString, cx: &mut WindowContext) {
        let Some(selection) = self.selection.read(cx).clone() else {
            return;
        };

        if self.text_state.is_none() || self.bounds.is_none() {
            return;
        }

        let text_state = self.text_state.clone().unwrap();
        let bounds = self.bounds.unwrap();

        let Some(mut position) = self
            .text_state
            .clone()
            .unwrap()
            .position_for_index(bounds, selection.span.start)
        else {
            return;
        };

        position.y += text_state.line_height() * Pixels(1.5);

        let index = if let Some(mut index) = text_state.index_for_position(bounds, position) {
            index
        } else {
            // Either current is last line or line below is a newline.
            selection.get_next_line_start(text)
        };

        self.selection.update(cx, |this, cx| {
            if let Some(this) = this.as_mut() {
                this.span = index..index;
                cx.notify();
            }
        });
    }

    fn move_up(&self, _: &MoveUp, text: &SharedString, cx: &mut WindowContext) {
        let Some(selection) = self.selection.read(cx).clone() else {
            return;
        };

        if self.text_state.is_none() || self.bounds.is_none() {
            return;
        }

        let text_state = self.text_state.clone().unwrap();
        let bounds = self.bounds.unwrap();

        let Some(mut position) = text_state.position_for_index(bounds, selection.span.start) else {
            return;
        };

        position.y -= text_state.line_height() / 2.;

        let index = if let Some(mut index) = text_state.index_for_position(bounds, position) {
            index
        } else {
            // Either current is first line or line above is a newline.
            selection.get_previous_line_end(text)
        };

        self.selection.update(cx, |this, cx| {
            if let Some(this) = this.as_mut() {
                this.span = index..index;
                cx.notify();
            }
        });
    }
}

impl EditableText {
    fn into_input_handler(&self, state: &EditableTextState) -> EditableTextInputHandler {
        EditableTextInputHandler::new(self, state)
    }
}

struct EditableTextInputHandler {
    state: EditableTextState,
    kind: Kind,
    value: Model<String>,
    enter_listener: Option<Arc<dyn Fn(&mut WindowContext<'_>)>>,
    focus_next_listener: Option<Arc<dyn Fn(&mut WindowContext<'_>)>>,
    focus_prev_listener: Option<Arc<dyn Fn(&mut WindowContext<'_>)>>,
}

impl EditableTextInputHandler {
    fn new(current: &EditableText, state: &EditableTextState) -> Self {
        Self {
            state: state.clone(),
            kind: current.kind.clone(),
            value: current.value.clone(),
            enter_listener: current.enter_listener.clone(),
            focus_next_listener: current.focus_next_listener.clone(),
            focus_prev_listener: current.focus_prev_listener.clone(),
        }
    }
}

impl EditableTextInputHandler {
    fn backspace(&mut self, _: &Backspace, cx: &mut WindowContext) {
        let mut selection = self.state.selection.read(cx).clone().unwrap();

        if selection.is_empty() {
            let mut selection = selection.clone();
            selection.select_left(self.value.read(cx).as_ref());
            let mut new_selection = selection.clone();
            new_selection.span.end = new_selection.span.start;
            self.handle_transaction(
                ReplaceTextInRangeAndSelect::new(
                    "".to_string().into(),
                    Some(selection.span.clone()),
                    new_selection,
                ),
                cx,
            );
        } else {
            let mut selection = selection.clone();
            selection.move_left(self.value.read(cx).as_ref());
            self.handle_transaction(
                ReplaceTextInRangeAndSelect::new("".to_string().into(), None, selection.clone()),
                cx,
            );
        }
    }

    fn copy(&mut self, _: &Copy, cx: &mut WindowContext) {
        if let Some(selection) = self.state.selection.read(cx) {
            let text = &self.value.read(cx)[selection.span.clone()];

            cx.write_to_clipboard(ClipboardItem::new(text.into()));
        }
    }

    fn cut(&mut self, _: &Cut, cx: &mut WindowContext) {
        if let Some(selection) = self.state.selection.read(cx) {
            cx.write_to_clipboard(ClipboardItem::new(
                self.value.read(cx)[selection.span.clone()].into(),
            ));

            self.delete(&Delete, cx);
        }
    }

    fn delete(&mut self, _: &Delete, cx: &mut WindowContext) {
        if let Some(selection) = self.state.selection.read(cx) {
            self.replace_text_in_range(Some(selection.span.clone()), "", cx);
        }
    }

    fn newline(&mut self, _: &Newline, cx: &mut WindowContext) {
        if matches!(self.kind, Kind::Singline) {
            if let Some(listener) = &self.enter_listener {
                listener(cx);
            }
        } else {
            if let Some(selection) = self.state.selection.read(cx) {
                self.replace_text_in_range(Some(selection.span.clone()), "\n", cx);
            }
        }
    }

    fn paste(&mut self, _: &Paste, cx: &mut WindowContext) {
        if let Some(value) = cx.read_from_clipboard() {
            let selection = self.state.selection.read(cx).as_ref().unwrap();

            self.replace_text_in_range(Some(selection.span.clone()), value.text(), cx);
        }
    }

    fn redo(&mut self, _: &Redo, cx: &mut WindowContext) {
        self.state.history.update(cx, |history, cx| {
            self.state.selection.update(cx, |selection, cx| {
                if let Some(selection) = selection {
                    self.value.update(cx, |value, _cx| {
                        history.redo(value, selection);
                    });
                }
            });
        });
    }

    fn tab(&mut self, _: &Tab, cx: &mut WindowContext) {
        if let Some(listener) = &self.focus_next_listener {
            listener(cx);
        }
    }

    fn tab_prev(&mut self, _: &TabPrev, cx: &mut WindowContext) {
        if let Some(listener) = &self.focus_prev_listener {
            listener(cx);
        }
    }

    fn undo(&mut self, _: &Undo, cx: &mut WindowContext) {
        self.state.history.update(cx, |history, cx| {
            self.state.selection.update(cx, |selection, cx| {
                if let Some(selection) = selection {
                    self.value.update(cx, |value, _cx| {
                        history.undo(value, selection);
                    });
                }
            });
        });
    }
}

impl EditableTextInputHandler {
    fn handle_transaction(&mut self, tx: impl Transaction + 'static, cx: &mut WindowContext) {
        self.state.history.update(cx, |history, cx| {
            self.state.selection.update(cx, |selection, cx| {
                if let Some(selection) = selection {
                    self.value.update(cx, |value, _cx| {
                        history.apply(tx, value, selection);
                    });
                }
            });
        });

        cx.refresh();
    }
}

impl InputHandler for EditableTextInputHandler {
    fn selected_text_range(&mut self, cx: &mut WindowContext) -> Option<Range<usize>> {
        self.state
            .selection
            .read(cx)
            .clone()
            .map(|selection| selection.span)
    }

    fn marked_text_range(&mut self, cx: &mut WindowContext) -> Option<Range<usize>> {
        self.state.marked.read(cx).clone()
    }

    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        cx: &mut WindowContext,
    ) -> Option<String> {
        self.value.read(cx).get(range_utf16).map(ToOwned::to_owned)
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
    ) {
        self.handle_transaction(
            ReplaceTextInRange::new(text.to_owned().into(), replacement_range),
            cx,
        );
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut WindowContext,
    ) {
        // TODO: confirm this is correct functionality.

        self.replace_text_in_range(range_utf16, new_text, cx);

        self.state.marked.update(cx, |this, cx| {
            *this = new_selected_range;
            cx.notify();
        });
    }

    fn unmark_text(&mut self, cx: &mut WindowContext) {
        self.state.marked.update(cx, |this, cx| {
            *this = None;
            cx.notify();
        })
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _cx: &mut WindowContext,
    ) -> Option<Bounds<Pixels>> {
        if self.state.text_state.is_none() || self.state.bounds.is_none() {
            return None;
        }

        let text_state = self.state.text_state.clone().unwrap();
        let containing_bounds = self.state.bounds.unwrap();

        let Some(start_position) =
            text_state.position_for_index(containing_bounds, range_utf16.start)
        else {
            return None;
        };

        let Some(end_position) =
            text_state.position_for_index(containing_bounds, range_utf16.start)
        else {
            return None;
        };

        // TODO: confirm this works. Just wanna get a draft up and don't know much about IME.
        // - Might need to adjust for containing bounds
        // - Might need to adjust for width of final glyph

        Some(Bounds {
            origin: start_position,
            size: Size {
                width: end_position.x - start_position.x,
                height: text_state.line_height(),
            },
        })
    }
}
