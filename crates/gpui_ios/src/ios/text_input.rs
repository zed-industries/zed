use super::window::{IosWindowState, WindowReference};
use gpui::{
    Autocapitalize, PlatformInputHandler, TextInputAction, TextInputConfiguration,
    TextInputStateChange, px,
};
use objc2::rc::{Retained, Weak};
use objc2::runtime::{AnyObject, ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{
    NSArray, NSAttributedStringKey, NSComparisonResult, NSCopying, NSDictionary,
    NSObjectNSDelayedPerforming, NSObjectProtocol, NSRange, NSString,
};
use objc2_ui_kit::*;
use std::{
    cell::{Cell, RefCell},
    ops::Range,
};

define_class!(
    #[unsafe(super = UITextPosition)]
    #[thread_kind = MainThreadOnly]
    #[name = "GPUITextPosition"]
    #[ivars = usize]
    struct TextPosition;
    unsafe impl NSObjectProtocol for TextPosition {}
);

impl TextPosition {
    fn new(offset: usize, main_thread: MainThreadMarker) -> Retained<UITextPosition> {
        let this = Self::alloc(main_thread).set_ivars(offset);
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        this.into_super()
    }

    fn offset(position: &UITextPosition) -> Option<usize> {
        position
            .downcast_ref::<Self>()
            .map(|position| *position.ivars())
    }
}

define_class!(
    #[unsafe(super = UITextRange)]
    #[thread_kind = MainThreadOnly]
    #[name = "GPUITextRange"]
    #[ivars = Range<usize>]
    struct TextRange;
    unsafe impl NSObjectProtocol for TextRange {}
    impl TextRange {
        #[unsafe(method_id(start))]
        fn start(&self) -> Retained<UITextPosition> {
            TextPosition::new(self.ivars().start, self.mtm())
        }
        #[unsafe(method_id(end))]
        fn end(&self) -> Retained<UITextPosition> {
            TextPosition::new(self.ivars().end, self.mtm())
        }
        #[unsafe(method(isEmpty))]
        fn is_empty(&self) -> bool { self.ivars().is_empty() }
    }
);

impl TextRange {
    fn new(range: Range<usize>, main_thread: MainThreadMarker) -> Retained<UITextRange> {
        let this = Self::alloc(main_thread).set_ivars(range);
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        this.into_super()
    }

    fn range(range: &UITextRange) -> Option<Range<usize>> {
        range
            .downcast_ref::<Self>()
            .map(|range| range.ivars().clone())
    }
}

struct SelectionRectIvars {
    rect: CGRect,
    contains_start: bool,
    contains_end: bool,
}

define_class!(
    #[unsafe(super = UITextSelectionRect)]
    #[thread_kind = MainThreadOnly]
    #[name = "GPUITextSelectionRect"]
    #[ivars = SelectionRectIvars]
    struct SelectionRect;
    unsafe impl NSObjectProtocol for SelectionRect {}
    impl SelectionRect {
        #[unsafe(method(rect))]
        fn rect(&self) -> CGRect { self.ivars().rect }
        #[unsafe(method(containsStart))]
        fn contains_start(&self) -> bool { self.ivars().contains_start }
        #[unsafe(method(containsEnd))]
        fn contains_end(&self) -> bool { self.ivars().contains_end }
        #[unsafe(method(isVertical))]
        fn is_vertical(&self) -> bool { false }
        #[unsafe(method(writingDirection))]
        fn writing_direction(&self) -> NSWritingDirection { NSWritingDirection::Natural }
    }
);

pub(super) struct TextInputIvars {
    window: WindowReference,
    configuration: RefCell<TextInputConfiguration>,
    delegate: RefCell<Weak<ProtocolObject<dyn UITextInputDelegate>>>,
    marked_style: RefCell<Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>>,
    keyboard_desired: Cell<bool>,
    keyboard_eligible: Cell<bool>,
    update_pending: Cell<bool>,
    configuration_changed: Cell<bool>,
    selection_changed: Cell<bool>,
    content_changed: Cell<bool>,
    native_edit: Cell<bool>,
}

define_class!(
    #[unsafe(super = UIView)]
    #[thread_kind = MainThreadOnly]
    #[name = "GPUITextInputView"]
    #[ivars = TextInputIvars]
    pub(super) struct TextInputView;

    unsafe impl NSObjectProtocol for TextInputView {}

    impl TextInputView {
        #[unsafe(method(canBecomeFirstResponder))]
        fn can_become_first_responder(&self) -> bool {
            self.ivars().keyboard_desired.get()
                && self.ivars().keyboard_eligible.get()
        }

        #[unsafe(method(reconcileTextInput))]
        fn reconcile_text_input(&self) {
            self.ivars().update_pending.set(false);
            let eligible = self.ivars().keyboard_desired.get()
                && self.with_handler(|handler| {
                    if !handler.query_accepts_focused_text_input() { return false; }
                    let Some(document) = document_range(handler) else { return false; };
                    handler.selected_text_range(false)
                        .is_some_and(|selection| contains_range(&document, &selection.range))
                }).unwrap_or(false);
            self.ivars().keyboard_eligible.set(eligible);
            if self.canBecomeFirstResponder() {
                if !self.isFirstResponder() { self.becomeFirstResponder(); }
            } else if self.isFirstResponder() {
                self.resignFirstResponder();
            }
            if self.ivars().configuration_changed.replace(false) && self.isFirstResponder() {
                self.reloadInputViews();
            }
            let content_changed = self.ivars().content_changed.replace(false);
            let selection_changed = self.ivars().selection_changed.replace(false);
            if content_changed || selection_changed {
                if let Some(delegate) = self.inputDelegate() {
                    let input = Some(ProtocolObject::from_ref(self));
                    // GPUI only reports post-edit invalidation. Defer until its
                    // Window borrow is released before UIKit queries the handler.
                    if content_changed { delegate.textWillChange(input); }
                    delegate.selectionWillChange(input);
                    delegate.selectionDidChange(input);
                    if content_changed { delegate.textDidChange(input); }
                }
            }
        }

        #[unsafe(method(canPerformAction:withSender:))]
        fn can_perform_action(&self, action: Sel, _sender: Option<&AnyObject>) -> bool {
            self.ivars().window.can_perform_action(action)
        }
    }

    unsafe impl UIKeyInput for TextInputView {
        #[unsafe(method(hasText))]
        fn has_text(&self) -> bool {
            self.document_range().is_some_and(|range| !range.is_empty())
        }
        #[unsafe(method(insertText:))]
        fn insert_text(&self, text: &NSString) {
            self.edit(|handler| handler.replace_text_in_range(None, &text.to_string()));
        }
        #[unsafe(method(deleteBackward))]
        fn delete_backward(&self) {
            self.edit(|handler| {
                let Some(selection) = handler.selected_text_range(false) else { return; };
                let mut range = selection.range;
                if range.is_empty() {
                    let start = handler.text_input_editable_range().map_or(0, |range| range.start);
                    if range.start <= start { return; }
                    let Some(text) = read_text(handler, start..range.start) else { return; };
                    let Some(character) = composed_character_range(&text, text.length().saturating_sub(1)) else { return; };
                    range.start = start + character.start;
                }
                if allowed_range(handler, &range) {
                    handler.replace_text_in_range(Some(range), "");
                }
            });
        }
    }

    unsafe impl UITextInputTraits for TextInputView {
        #[unsafe(method(keyboardType))]
        fn keyboard_type(&self) -> UIKeyboardType { UIKeyboardType::Default }
        #[unsafe(method(autocorrectionType))]
        fn autocorrection_type(&self) -> UITextAutocorrectionType {
            if self.ivars().configuration.borrow().autocorrect { UITextAutocorrectionType::Yes } else { UITextAutocorrectionType::No }
        }
        #[unsafe(method(autocapitalizationType))]
        fn autocapitalization_type(&self) -> UITextAutocapitalizationType {
            match self.ivars().configuration.borrow().autocapitalize {
                Autocapitalize::None => UITextAutocapitalizationType::None,
                Autocapitalize::Words => UITextAutocapitalizationType::Words,
                Autocapitalize::Sentences => UITextAutocapitalizationType::Sentences,
                Autocapitalize::Characters => UITextAutocapitalizationType::AllCharacters,
            }
        }
        #[unsafe(method(spellCheckingType))]
        fn spell_checking_type(&self) -> UITextSpellCheckingType {
            if self.ivars().configuration.borrow().suggestions { UITextSpellCheckingType::Yes } else { UITextSpellCheckingType::No }
        }
        #[unsafe(method(inlinePredictionType))]
        fn inline_prediction_type(&self) -> UITextInlinePredictionType {
            if self.ivars().configuration.borrow().suggestions { UITextInlinePredictionType::Yes } else { UITextInlinePredictionType::No }
        }
        #[unsafe(method(returnKeyType))]
        fn return_key_type(&self) -> UIReturnKeyType {
            match self.ivars().configuration.borrow().input_action {
                TextInputAction::Unspecified | TextInputAction::Enter | TextInputAction::Previous => UIReturnKeyType::Default,
                TextInputAction::Done => UIReturnKeyType::Done,
                TextInputAction::Go => UIReturnKeyType::Go,
                TextInputAction::Next => UIReturnKeyType::Next,
                TextInputAction::Search => UIReturnKeyType::Search,
                TextInputAction::Send => UIReturnKeyType::Send,
            }
        }
    }

    unsafe impl UITextInput for TextInputView {
        #[unsafe(method_id(textInRange:))]
        fn text_in_range(&self, range: &UITextRange) -> Option<Retained<NSString>> {
            (|| {
            let range = TextRange::range(range)?;
            self.with_handler(|handler| read_text(handler, range)).flatten()
            })()
        }
        #[unsafe(method(replaceRange:withText:))]
        fn replace_range(&self, range: &UITextRange, text: &NSString) {
            if let Some(range) = TextRange::range(range) {
                self.edit(|handler| {
                    if allowed_range(handler, &range) {
                        handler.replace_text_in_range(Some(range), &text.to_string());
                    }
                });
            }
        }
        #[unsafe(method_id(selectedTextRange))]
        fn selected_text_range(&self) -> Option<Retained<UITextRange>> {
            self.with_handler(|handler| {
                let selection = handler.selected_text_range(false)?;
                allowed_range(handler, &selection.range).then_some(selection)
            }).flatten().map(|selection| TextRange::new(selection.range, self.mtm()))
        }
        #[unsafe(method(setSelectedTextRange:))]
        fn set_selected_text_range(&self, range: Option<&UITextRange>) {
            if let Some(range) = range.and_then(TextRange::range) {
                self.edit(|handler| {
                    if allowed_range(handler, &range) { handler.set_selected_text_range(range); }
                });
            }
        }
        #[unsafe(method_id(markedTextRange))]
        fn marked_text_range(&self) -> Option<Retained<UITextRange>> {
            self.with_handler(|handler| {
                let range = handler.marked_text_range()?;
                allowed_range(handler, &range).then_some(range)
            }).flatten().map(|range| TextRange::new(range, self.mtm()))
        }
        #[unsafe(method_id(markedTextStyle))]
        fn marked_text_style(&self) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>> {
            self.ivars().marked_style.borrow().clone()
        }
        #[unsafe(method(setMarkedTextStyle:))]
        unsafe fn set_marked_text_style(&self, style: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>) {
            *self.ivars().marked_style.borrow_mut() = style.map(|style| style.copy());
        }
        #[unsafe(method(setMarkedText:selectedRange:))]
        fn set_marked_text(&self, text: Option<&NSString>, selection: NSRange) {
            let text = text.map(|text| text.to_string()).unwrap_or_default();
            let selection = checked_range(selection.location, selection.length, text.encode_utf16().count());
            self.edit(|handler| handler.replace_and_mark_text_in_range(None, &text, selection));
        }
        #[unsafe(method(unmarkText))]
        fn unmark_text(&self) { self.edit(PlatformInputHandler::unmark_text); }
        #[unsafe(method_id(beginningOfDocument))]
        fn beginning_of_document(&self) -> Retained<UITextPosition> {
            TextPosition::new(self.document_range().map_or(0, |range| range.start), self.mtm())
        }
        #[unsafe(method_id(endOfDocument))]
        fn end_of_document(&self) -> Retained<UITextPosition> {
            TextPosition::new(self.document_range().map_or(0, |range| range.end), self.mtm())
        }
        #[unsafe(method_id(textRangeFromPosition:toPosition:))]
        fn text_range(&self, start: &UITextPosition, end: &UITextPosition) -> Option<Retained<UITextRange>> {
            (|| {
            let start = TextPosition::offset(start)?;
            let end = TextPosition::offset(end)?;
            Some(TextRange::new(start.min(end)..start.max(end), self.mtm()))
            })()
        }
        #[unsafe(method_id(positionFromPosition:offset:))]
        fn position_offset(&self, position: &UITextPosition, offset: isize) -> Option<Retained<UITextPosition>> {
            (|| {
            let range = self.document_range()?;
            let offset = checked_offset(TextPosition::offset(position)?, offset, range.end)?;
            (offset >= range.start).then(|| TextPosition::new(offset, self.mtm()))
            })()
        }
        #[unsafe(method_id(positionFromPosition:inDirection:offset:))]
        fn position_direction(&self, position: &UITextPosition, direction: UITextLayoutDirection, offset: isize) -> Option<Retained<UITextPosition>> {
            (|| {
            if direction == UITextLayoutDirection::Left {
                return self.positionFromPosition_offset(position, offset.checked_neg()?);
            }
            if direction == UITextLayoutDirection::Right {
                return self.positionFromPosition_offset(position, offset);
            }
            let rectangle = self.caretRectForPosition(position);
            let sign = if direction == UITextLayoutDirection::Up { -1.0 } else { 1.0 };
            self.closestPositionToPoint(CGPoint::new(rectangle.origin.x, rectangle.origin.y + rectangle.size.height * (0.5 + sign * offset as f64)))
            })()
        }
        #[unsafe(method(comparePosition:toPosition:))]
        fn compare_position(&self, position: &UITextPosition, other: &UITextPosition) -> NSComparisonResult {
            match TextPosition::offset(position).cmp(&TextPosition::offset(other)) {
                std::cmp::Ordering::Less => NSComparisonResult::Ascending,
                std::cmp::Ordering::Equal => NSComparisonResult::Same,
                std::cmp::Ordering::Greater => NSComparisonResult::Descending,
            }
        }
        #[unsafe(method(offsetFromPosition:toPosition:))]
        fn offset_from_position(&self, start: &UITextPosition, end: &UITextPosition) -> isize {
            let start = TextPosition::offset(start).and_then(|offset| isize::try_from(offset).ok()).unwrap_or(0);
            let end = TextPosition::offset(end).and_then(|offset| isize::try_from(offset).ok()).unwrap_or(0);
            end.saturating_sub(start)
        }
        #[unsafe(method_id(inputDelegate))]
        fn input_delegate(&self) -> Option<Retained<ProtocolObject<dyn UITextInputDelegate>>> { self.ivars().delegate.borrow().load() }
        #[unsafe(method(setInputDelegate:))]
        fn set_input_delegate(&self, delegate: Option<&ProtocolObject<dyn UITextInputDelegate>>) {
            *self.ivars().delegate.borrow_mut() = delegate.map(Weak::new).unwrap_or_default();
        }
        #[unsafe(method_id(tokenizer))]
        fn input_tokenizer(&self) -> Retained<ProtocolObject<dyn UITextInputTokenizer>> {
            let tokenizer = unsafe { UITextInputStringTokenizer::initWithTextInput(UITextInputStringTokenizer::alloc(self.mtm()), self) };
            ProtocolObject::from_retained(tokenizer)
        }
        #[unsafe(method_id(positionWithinRange:farthestInDirection:))]
        fn farthest_position(&self, range: &UITextRange, direction: UITextLayoutDirection) -> Option<Retained<UITextPosition>> {
            (|| {
            let range = TextRange::range(range)?;
            let offset = if direction == UITextLayoutDirection::Left || direction == UITextLayoutDirection::Up { range.start } else { range.end };
            Some(TextPosition::new(offset, self.mtm()))
            })()
        }
        #[unsafe(method_id(characterRangeByExtendingPosition:inDirection:))]
        fn extending_range(&self, position: &UITextPosition, direction: UITextLayoutDirection) -> Option<Retained<UITextRange>> {
            (|| {
            let offset = TextPosition::offset(position)?;
            let backwards = direction == UITextLayoutDirection::Left || direction == UITextLayoutDirection::Up;
            self.character_range(if backwards { offset.checked_sub(1)? } else { offset })
            })()
        }
        #[unsafe(method(baseWritingDirectionForPosition:inDirection:))]
        fn base_writing_direction(&self, _position: &UITextPosition, _direction: UITextStorageDirection) -> NSWritingDirection { NSWritingDirection::Natural }
        #[unsafe(method(setBaseWritingDirection:forRange:))]
        fn set_base_writing_direction(&self, direction: NSWritingDirection, _range: &UITextRange) {
            if direction != NSWritingDirection::Natural {
                log::warn!("GPUI input handlers do not support overriding paragraph writing direction");
            }
        }
        #[unsafe(method(firstRectForRange:))]
        fn first_rect(&self, range: &UITextRange) -> CGRect {
            TextRange::range(range).and_then(|range| self.bounds(range)).unwrap_or(CGRect::ZERO)
        }
        #[unsafe(method(caretRectForPosition:))]
        fn caret_rect(&self, position: &UITextPosition) -> CGRect {
            TextPosition::offset(position)
                .and_then(|offset| self.bounds(offset..offset))
                .map(nonempty_caret_rect)
                .unwrap_or(CGRect::ZERO)
        }
        #[unsafe(method_id(selectionRectsForRange:))]
        fn selection_rects(&self, range: &UITextRange) -> Retained<NSArray<UITextSelectionRect>> {
            (|| {
            let Some(range) = TextRange::range(range) else { return NSArray::new(); };
            let bounds = self.with_handler(|handler| {
                if !allowed_range(handler, &range) { return Vec::new(); }
                handler.selection_bounds_for_range(range)
            }).unwrap_or_default();
            let count = bounds.len();
            let rectangles = bounds.into_iter().enumerate().map(|(index, bounds)| {
                    let this = SelectionRect::alloc(self.mtm()).set_ivars(SelectionRectIvars {
                        rect: native_rect(bounds),
                        contains_start: index == 0,
                        contains_end: index + 1 == count,
                    });
                    let this: Retained<SelectionRect> = unsafe { msg_send![super(this), init] };
                    this.into_super()
            }).collect::<Vec<_>>();
            NSArray::from_retained_slice(&rectangles)
            })()
        }
        #[unsafe(method_id(closestPositionToPoint:))]
        fn closest_position(&self, point: CGPoint) -> Option<Retained<UITextPosition>> {
            self.with_handler(|handler| {
                let offset = handler.character_index_for_point(gpui::point(px(point.x as f32), px(point.y as f32)))?;
                allowed_range(handler, &(offset..offset)).then_some(offset)
            }).flatten()
                .map(|offset| TextPosition::new(offset, self.mtm()))
        }
        #[unsafe(method_id(closestPositionToPoint:withinRange:))]
        fn closest_position_within_range(&self, point: CGPoint, range: &UITextRange) -> Option<Retained<UITextPosition>> {
            (|| {
            let range = TextRange::range(range)?;
            let position = self.closestPositionToPoint(point)?;
            Some(TextPosition::new(TextPosition::offset(&position)?.clamp(range.start, range.end), self.mtm()))
            })()
        }
        #[unsafe(method_id(characterRangeAtPoint:))]
        fn character_range_at_point(&self, point: CGPoint) -> Option<Retained<UITextRange>> {
            (|| {
                let position = self.closestPositionToPoint(point)?;
                self.character_range(TextPosition::offset(&position)?)
            })()
        }
        #[unsafe(method_id(textInputView))]
        fn text_input_view(&self) -> Retained<UIView> {
            // GPUI's bounds and hit testing use the rendering view's coordinate space,
            // not the hidden responder's one-point frame.
            self.superview().unwrap_or_else(|| self.retain().into_super())
        }
    }

    unsafe impl UIResponderStandardEditActions for TextInputView {
        #[unsafe(method(cut:))]
        unsafe fn cut(&self, _sender: Option<&AnyObject>) { self.ivars().window.dispatch_edit_menu_shortcut("x"); }
        #[unsafe(method(copy:))]
        unsafe fn copy(&self, _sender: Option<&AnyObject>) { self.ivars().window.dispatch_edit_menu_shortcut("c"); }
        #[unsafe(method(paste:))]
        unsafe fn paste(&self, _sender: Option<&AnyObject>) { self.ivars().window.dispatch_edit_menu_shortcut("v"); }
        #[unsafe(method(selectAll:))]
        unsafe fn select_all(&self, _sender: Option<&AnyObject>) { self.ivars().window.dispatch_edit_menu_shortcut("a"); }
    }
);

impl TextInputView {
    pub(super) fn new(frame: CGRect, main_thread: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(main_thread).set_ivars(TextInputIvars {
            window: WindowReference::default(),
            configuration: RefCell::new(TextInputConfiguration::default()),
            delegate: RefCell::new(Weak::default()),
            marked_style: RefCell::new(None),
            keyboard_desired: Cell::new(false),
            keyboard_eligible: Cell::new(false),
            update_pending: Cell::new(false),
            configuration_changed: Cell::new(false),
            selection_changed: Cell::new(false),
            content_changed: Cell::new(false),
            native_edit: Cell::new(false),
        });
        unsafe { msg_send![super(this), initWithFrame: frame] }
    }

    pub(super) fn set_window(&self, window: std::rc::Weak<IosWindowState>) {
        *self.ivars().window.0.borrow_mut() = window;
    }

    fn with_handler<R>(&self, callback: impl FnOnce(&mut PlatformInputHandler) -> R) -> Option<R> {
        self.ivars()
            .window
            .with_window(|window| window.with_input_handler(callback))
            .flatten()
    }

    fn edit(&self, callback: impl FnOnce(&mut PlatformInputHandler)) {
        let previous = self.ivars().native_edit.replace(true);
        self.with_handler(|handler| {
            let selected = handler.selected_text_range(false);
            let marked = handler.marked_text_range();
            let selection_allowed =
                selected.is_some_and(|selection| allowed_range(handler, &selection.range));
            let marked_allowed = marked.is_none_or(|range| allowed_range(handler, &range));
            if handler.query_accepts_focused_text_input() && selection_allowed && marked_allowed {
                callback(handler);
            }
        });
        self.ivars().native_edit.set(previous);
    }

    fn document_range(&self) -> Option<Range<usize>> {
        self.with_handler(document_range).flatten()
    }

    fn bounds(&self, range: Range<usize>) -> Option<CGRect> {
        self.with_handler(|handler| {
            if !allowed_range(handler, &range) {
                return None;
            }
            handler.bounds_for_range(range)
        })
        .flatten()
        .map(native_rect)
    }

    fn character_range(&self, offset: usize) -> Option<Retained<UITextRange>> {
        let (document_range, text) = self.document_text()?;
        let character = composed_character_range(&text, offset.checked_sub(document_range.start)?)?;
        Some(TextRange::new(
            (document_range.start + character.start)..(document_range.start + character.end),
            self.mtm(),
        ))
    }

    fn document_text(&self) -> Option<(Range<usize>, Retained<NSString>)> {
        self.with_handler(|handler| {
            let range = document_range(handler)?;
            let text = read_text(handler, range.clone())?;
            Some((range, text))
        })
        .flatten()
    }

    pub(super) fn set_keyboard_visible(&self, visible: bool) {
        self.ivars().keyboard_desired.set(visible);
        self.refresh_keyboard();
    }

    pub(super) fn refresh_keyboard(&self) {
        if self.ivars().update_pending.replace(true) {
            return;
        }
        // Draw temporarily takes the handler and owns the GPUI Window borrow.
        // A single deferred reconciliation observes its final state, never a
        // stale queued request to become first responder after focus was lost.
        unsafe {
            self.performSelector_withObject_afterDelay(sel!(reconcileTextInput), None, 0.0);
        }
    }

    pub(super) fn set_configuration(&self, configuration: &TextInputConfiguration) {
        if *self.ivars().configuration.borrow() == *configuration {
            return;
        }
        *self.ivars().configuration.borrow_mut() = configuration.clone();
        self.ivars().configuration_changed.set(true);
        self.refresh_keyboard();
    }

    pub(super) fn state_changed(&self, change: TextInputStateChange) {
        match change {
            TextInputStateChange::FocusGained => {
                self.ivars().content_changed.set(true);
                self.ivars().selection_changed.set(true);
                self.set_keyboard_visible(true);
            }
            TextInputStateChange::FocusLost => self.set_keyboard_visible(false),
            TextInputStateChange::SelectionChanged | TextInputStateChange::ContentChanged => {
                if self.ivars().native_edit.get() {
                    return;
                }
                self.ivars().selection_changed.set(true);
                if matches!(change, TextInputStateChange::ContentChanged) {
                    self.ivars().content_changed.set(true);
                }
                self.refresh_keyboard();
            }
        }
    }
}

fn native_rect(bounds: gpui::Bounds<gpui::Pixels>) -> CGRect {
    CGRect::new(
        CGPoint::new(f64::from(bounds.origin.x), f64::from(bounds.origin.y)),
        CGSize::new(f64::from(bounds.size.width), f64::from(bounds.size.height)),
    )
}

fn nonempty_caret_rect(mut rect: CGRect) -> CGRect {
    // GPUI can report a zero-width insertion location; UIKit needs a visible caret.
    rect.size.width = rect.size.width.max(1.);
    rect
}

fn contains_range(outer: &Range<usize>, inner: &Range<usize>) -> bool {
    outer.start <= inner.start && inner.start <= inner.end && inner.end <= outer.end
}

fn document_range(handler: &mut PlatformInputHandler) -> Option<Range<usize>> {
    handler
        .text_input_editable_range()
        .or_else(|| handler.text_length_utf16().map(|length| 0..length))
        .filter(|range| range.start <= range.end)
}

fn allowed_range(handler: &mut PlatformInputHandler, range: &Range<usize>) -> bool {
    document_range(handler).is_some_and(|editable| contains_range(&editable, range))
}

fn read_text(
    handler: &mut PlatformInputHandler,
    range: Range<usize>,
) -> Option<Retained<NSString>> {
    if !allowed_range(handler, &range) {
        return None;
    }
    let mut adjusted = None;
    let text = handler.text_for_range(range.clone(), &mut adjusted)?;
    // UIKit cannot represent an adjusted range alongside the returned string.
    // Do not shift the origin or expose text across an editable-region boundary.
    if adjusted.is_some_and(|adjusted| adjusted != range)
        || text.encode_utf16().count() != range.len()
    {
        return None;
    }
    Some(NSString::from_str(&text))
}

fn checked_range(location: usize, length: usize, document_length: usize) -> Option<Range<usize>> {
    let end = location.checked_add(length)?;
    (end <= document_length).then_some(location..end)
}

fn composed_character_range(text: &NSString, offset: usize) -> Option<Range<usize>> {
    // NSString uses UTF-16 and keeps surrogate pairs, combining marks, and
    // emoji sequences together. Check first: its API raises on an invalid index.
    if offset >= text.length() {
        return None;
    }
    let range = text.rangeOfComposedCharacterSequenceAtIndex(offset);
    checked_range(range.location, range.length, text.length())
}

fn checked_offset(position: usize, offset: isize, document_length: usize) -> Option<usize> {
    let position = position.checked_add_signed(offset)?;
    (position <= document_length).then_some(position)
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    #[test]
    fn insertion_geometry_has_a_visible_caret_width() {
        let rect = nonempty_caret_rect(CGRect::new(CGPoint::new(12., 20.), CGSize::new(0., 30.)));
        assert_eq!(rect.origin, CGPoint::new(12., 20.));
        assert_eq!(rect.size, CGSize::new(1., 30.));
    }

    define_class!(
        #[unsafe(super = objc2_foundation::NSObject)]
        #[thread_kind = MainThreadOnly]
        #[name = "GPUITextInputTestDelegate"]
        #[ivars = RefCell<Vec<&'static str>>]
        struct InputDelegate;

        unsafe impl NSObjectProtocol for InputDelegate {}
        unsafe impl UITextInputDelegate for InputDelegate {
            #[unsafe(method(selectionWillChange:))]
            fn selection_will_change(&self, _input: Option<&ProtocolObject<dyn UITextInput>>) {
                self.ivars().borrow_mut().push("selection-will");
            }
            #[unsafe(method(selectionDidChange:))]
            fn selection_did_change(&self, _input: Option<&ProtocolObject<dyn UITextInput>>) {
                self.ivars().borrow_mut().push("selection-did");
            }
            #[unsafe(method(textWillChange:))]
            fn text_will_change(&self, _input: Option<&ProtocolObject<dyn UITextInput>>) {
                self.ivars().borrow_mut().push("text-will");
            }
            #[unsafe(method(textDidChange:))]
            fn text_did_change(&self, _input: Option<&ProtocolObject<dyn UITextInput>>) {
                self.ivars().borrow_mut().push("text-did");
            }
            #[unsafe(method(conversationContext:didChange:))]
            fn conversation_context_did_change(
                &self,
                _context: Option<&UIConversationContext>,
                _input: Option<&ProtocolObject<dyn UITextInput>>,
            ) {
                self.ivars().borrow_mut().push("conversation-did");
            }
        }
    );

    #[allow(
        dead_code,
        reason = "called by the main-thread integration-test harness"
    )]
    pub(crate) fn native_protocol_smoke_test(main_thread: MainThreadMarker) {
        let position = TextPosition::new(3, main_thread);
        assert_eq!(TextPosition::offset(&position), Some(3));
        let range = TextRange::new(1..3, main_thread);
        assert_eq!(TextPosition::offset(&range.start()), Some(1));
        assert_eq!(TextPosition::offset(&range.end()), Some(3));
        assert!(!range.isEmpty());
        assert!(TextRange::new(3..3, main_thread).isEmpty());

        let text = NSString::from_str("a😀e\u{301}👨‍👩‍👧‍👦");
        assert_eq!(composed_character_range(&text, 1), Some(1..3));
        assert_eq!(composed_character_range(&text, 2), Some(1..3));
        assert_eq!(composed_character_range(&text, 4), Some(3..5));
        assert_eq!(composed_character_range(&text, 6), Some(5..16));
        assert_eq!(composed_character_range(&text, 16), None);

        let view = TextInputView::new(CGRect::ZERO, main_thread);
        view.set_window(std::rc::Weak::new());
        assert!(!view.canBecomeFirstResponder());
        view.set_keyboard_visible(true);
        view.reconcile_text_input(sel!(reconcileTextInput));
        assert!(!view.canBecomeFirstResponder());
        assert!(!view.hasText());
        assert!(view.selectedTextRange().is_none());
        assert!(view.markedTextRange().is_none());
        assert_eq!(TextPosition::offset(&view.beginningOfDocument()), Some(0));
        assert_eq!(TextPosition::offset(&view.endOfDocument()), Some(0));
        assert_eq!(
            view.offsetFromPosition_toPosition(&range.start(), &range.end()),
            2
        );
        assert_eq!(
            view.comparePosition_toPosition(&range.start(), &range.end()),
            NSComparisonResult::Ascending
        );
        assert!(
            view.textRangeFromPosition_toPosition(&range.end(), &range.start())
                .is_some()
        );
        view.tokenizer();
        let delegate = InputDelegate::alloc(main_thread).set_ivars(RefCell::default());
        let delegate: Retained<InputDelegate> = unsafe { msg_send![super(delegate), init] };
        view.setInputDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        view.state_changed(TextInputStateChange::ContentChanged);
        view.state_changed(TextInputStateChange::SelectionChanged);
        assert!(delegate.ivars().borrow().is_empty());
        objc2::rc::autoreleasepool(|_| {
            view.reconcile_text_input(sel!(reconcileTextInput));
        });
        assert_eq!(
            *delegate.ivars().borrow(),
            ["text-will", "selection-will", "selection-did", "text-did"]
        );
        drop(delegate);
        assert!(view.inputDelegate().is_none());
        view.setMarkedText_selectedRange(Some(&text), NSRange::new(1, 2));
        view.unmarkText();
        view.set_configuration(&TextInputConfiguration {
            autocorrect: true,
            autocapitalize: Autocapitalize::Sentences,
            suggestions: true,
            input_action: TextInputAction::Search,
        });
        assert_eq!(view.autocorrectionType(), UITextAutocorrectionType::Yes);
        assert_eq!(
            view.autocapitalizationType(),
            UITextAutocapitalizationType::Sentences
        );
        assert_eq!(view.spellCheckingType(), UITextSpellCheckingType::Yes);
        assert_eq!(view.returnKeyType(), UIReturnKeyType::Search);
        view.set_keyboard_visible(false);
        view.reconcile_text_input(sel!(reconcileTextInput));
        assert!(!view.canBecomeFirstResponder());
    }

    #[test]
    fn utf16_ranges_and_offsets() {
        assert!(contains_range(&(4..8), &(4..4)));
        assert!(contains_range(&(4..8), &(8..8)));
        assert!(!contains_range(&(4..8), &(3..5)));
        assert!(!contains_range(&(4..8), &(7..9)));
        assert!(!contains_range(&(4..8), &(6..5)));
        let length = "a😀e\u{301}".encode_utf16().count();
        assert_eq!(length, 5);
        assert_eq!(checked_range(1, 2, length), Some(1..3));
        assert_eq!(checked_range(length, 0, length), Some(length..length));
        assert_eq!(checked_range(usize::MAX, 1, length), None);
        assert_eq!(checked_range(4, 2, length), None);
        assert_eq!(checked_offset(3, -2, length), Some(1));
        assert_eq!(checked_offset(0, -1, length), None);
        assert_eq!(checked_offset(length, 1, length), None);
        assert_eq!(checked_offset(0, isize::MIN, length), None);
    }
}
