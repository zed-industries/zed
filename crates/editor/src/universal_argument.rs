use crate::actions::{
    MoveLeft, MoveRight, MoveToEndOfParagraph, MoveToNextWordEnd, MoveToPreviousWordStart,
    MoveToStartOfParagraph, UniversalArgument, UniversalArgumentDigit, UniversalArgumentMinus,
};
use gpui::{Action, App, Global, KeystrokeEvent, Window};
use zed_actions::editor::{MoveDown, MoveUp};

pub const MAX_UNIVERSAL_REPEAT: i32 = 1_000_000;

#[derive(Default)]
pub struct UniversalArgumentGlobals {
    pub(crate) state: Option<UniversalArgumentState>,
    pub(crate) consumed_this_dispatch: bool,
    pub(crate) replaying_dispatch: bool,
}

impl UniversalArgumentGlobals {
    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    pub fn accepts_minus(&self) -> bool {
        self.state
            .is_some_and(UniversalArgumentState::accepts_minus)
    }

    pub(crate) fn take(&mut self) -> Option<ResolvedUniversalArgument> {
        let argument = self.state.take().map(UniversalArgumentState::resolve);
        if argument.is_some() {
            self.consumed_this_dispatch = true;
        }
        argument
    }

    pub(crate) fn clear(&mut self) -> bool {
        if self.state.take().is_some() {
            self.consumed_this_dispatch = false;
            true
        } else {
            false
        }
    }
}

impl Global for UniversalArgumentGlobals {}

pub(crate) fn universal_argument(cx: &mut App) -> bool {
    let universal_argument_globals = cx.default_global::<UniversalArgumentGlobals>();
    let next_argument = universal_argument_globals.state.map_or_else(
        UniversalArgumentState::new_plain,
        UniversalArgumentState::multiply,
    );

    if universal_argument_globals.state != Some(next_argument)
        || universal_argument_globals.consumed_this_dispatch
    {
        universal_argument_globals.state = Some(next_argument);
        universal_argument_globals.consumed_this_dispatch = false;
        cx.refresh_windows();
        true
    } else {
        false
    }
}

pub(crate) fn universal_argument_digit(digit: usize, cx: &mut App) -> bool {
    let universal_argument_globals = cx.default_global::<UniversalArgumentGlobals>();
    let Some(argument) = universal_argument_globals.state.take() else {
        return false;
    };

    universal_argument_globals.state = Some(argument.push_digit(digit));
    universal_argument_globals.consumed_this_dispatch = false;
    cx.refresh_windows();
    true
}

pub(crate) fn universal_argument_minus(cx: &mut App) -> bool {
    let universal_argument_globals = cx.default_global::<UniversalArgumentGlobals>();
    let Some(argument) = universal_argument_globals.state.take() else {
        return false;
    };

    universal_argument_globals.state = Some(argument.apply_minus());
    universal_argument_globals.consumed_this_dispatch = false;
    cx.refresh_windows();
    true
}

pub(crate) fn intercept_keystrokes(event: &KeystrokeEvent, window: &mut Window, cx: &mut App) {
    if window.has_pending_keystrokes() || event.keystroke.is_ime_in_progress() {
        return;
    }

    let Some(count) = ({
        let Some(universal_argument_globals) = cx.try_global::<UniversalArgumentGlobals>() else {
            return;
        };
        if universal_argument_globals.replaying_dispatch
            || universal_argument_globals.consumed_this_dispatch
        {
            return;
        }
        universal_argument_globals
            .state
            .map(UniversalArgumentState::resolve)
            .map(ResolvedUniversalArgument::numeric_value)
    }) else {
        return;
    };

    if count >= 0 {
        return;
    }

    let Some(action) = action_for_keystroke(event, cx) else {
        return;
    };
    if is_universal_argument_action(action.as_ref()) {
        return;
    }

    let Some(inverse_action) = inverse_action(action.as_ref()) else {
        return;
    };

    let dispatch_count = count.unsigned_abs() as usize;
    cx.stop_propagation();
    {
        let universal_argument_globals = cx.default_global::<UniversalArgumentGlobals>();
        universal_argument_globals.state = None;
        universal_argument_globals.consumed_this_dispatch = false;
        universal_argument_globals.replaying_dispatch = true;
    }
    for _ in 0..dispatch_count {
        window.dispatch_action(inverse_action.boxed_clone(), cx);
    }
    cx.default_global::<UniversalArgumentGlobals>()
        .replaying_dispatch = false;
    cx.refresh_windows();
}

pub(crate) fn observe_keystrokes(event: &KeystrokeEvent, window: &mut Window, cx: &mut App) {
    if window.has_pending_keystrokes() || event.keystroke.is_ime_in_progress() {
        return;
    }

    let Some(action) = event.action.as_ref() else {
        if event.keystroke.key_char.is_some() {
            if !is_editor_context(event) {
                repeat_text_input(event, window, cx);
            }
        } else {
            cx.default_global::<UniversalArgumentGlobals>().clear();
        }
        return;
    };

    if is_universal_argument_action(action.as_ref()) {
        return;
    }

    let Some(argument) = ({
        let universal_argument_globals = cx.default_global::<UniversalArgumentGlobals>();
        if universal_argument_globals.replaying_dispatch {
            return;
        }
        if universal_argument_globals.consumed_this_dispatch {
            universal_argument_globals.state = None;
            universal_argument_globals.consumed_this_dispatch = false;
            return;
        }
        universal_argument_globals
            .state
            .take()
            .map(UniversalArgumentState::resolve)
    }) else {
        return;
    };

    let count = argument.numeric_value();
    let Some((repeat_action, dispatch_count)) = repeat_action(action.as_ref(), count) else {
        return;
    };
    if dispatch_count == 0 {
        return;
    }

    cx.default_global::<UniversalArgumentGlobals>()
        .replaying_dispatch = true;
    for _ in 0..dispatch_count {
        window.dispatch_action(repeat_action.boxed_clone(), cx);
    }
    cx.default_global::<UniversalArgumentGlobals>()
        .replaying_dispatch = false;
}

fn repeat_text_input(event: &KeystrokeEvent, window: &mut Window, cx: &mut App) {
    let Some(count) = ({
        let Some(universal_argument_globals) = cx.try_global::<UniversalArgumentGlobals>() else {
            return;
        };
        if universal_argument_globals.replaying_dispatch
            || universal_argument_globals.consumed_this_dispatch
        {
            return;
        }
        universal_argument_globals
            .state
            .map(UniversalArgumentState::resolve)
            .map(ResolvedUniversalArgument::numeric_value)
    }) else {
        return;
    };
    {
        let universal_argument_globals = cx.default_global::<UniversalArgumentGlobals>();
        universal_argument_globals.state = None;
        universal_argument_globals.consumed_this_dispatch = false;
    }

    if count == 0 {
        cx.stop_propagation();
        return;
    }

    let dispatch_count = (count.unsigned_abs() as usize).saturating_sub(1);
    if dispatch_count == 0 {
        return;
    }

    cx.default_global::<UniversalArgumentGlobals>()
        .replaying_dispatch = true;
    for _ in 0..dispatch_count {
        window.dispatch_keystroke(event.keystroke.clone(), cx);
    }
    cx.default_global::<UniversalArgumentGlobals>()
        .replaying_dispatch = false;
}

fn action_for_keystroke(event: &KeystrokeEvent, cx: &App) -> Option<Box<dyn Action>> {
    let input = [event.keystroke.clone()];
    let mut selected_action = None;
    let mut selected_depth = None;
    for binding in cx.all_bindings_for_input(&input) {
        let Some(depth) = binding
            .predicate()
            .map_or(Some(event.context_stack.len()), |predicate| {
                predicate.depth_of(&event.context_stack)
            })
        else {
            continue;
        };

        if selected_depth.is_none_or(|selected_depth| depth > selected_depth) {
            selected_depth = Some(depth);
            selected_action = Some(binding.action().boxed_clone());
        }
    }
    selected_action
}

fn is_editor_context(event: &KeystrokeEvent) -> bool {
    event
        .context_stack
        .iter()
        .any(|context| context.contains("Editor"))
}

pub(crate) fn is_universal_argument_action(action: &dyn Action) -> bool {
    action.as_any().is::<UniversalArgument>()
        || action.as_any().is::<UniversalArgumentDigit>()
        || action.as_any().is::<UniversalArgumentMinus>()
}

fn repeat_action(action: &dyn Action, count: i32) -> Option<(Box<dyn Action>, usize)> {
    if count == 0 {
        return None;
    }

    if count < 0 {
        let magnitude = count.unsigned_abs() as usize;
        if let Some(inverse_action) = inverse_action(action) {
            Some((inverse_action, magnitude.saturating_add(1)))
        } else {
            Some((action.boxed_clone(), magnitude.saturating_sub(1)))
        }
    } else {
        Some((action.boxed_clone(), (count as usize).saturating_sub(1)))
    }
}

fn inverse_action(action: &dyn Action) -> Option<Box<dyn Action>> {
    if action.as_any().is::<MoveRight>() {
        Some(Box::new(MoveLeft))
    } else if action.as_any().is::<MoveLeft>() {
        Some(Box::new(MoveRight))
    } else if action.as_any().is::<MoveDown>() {
        Some(Box::new(MoveUp))
    } else if action.as_any().is::<MoveUp>() {
        Some(Box::new(MoveDown))
    } else if action.as_any().is::<MoveToNextWordEnd>() {
        Some(Box::new(MoveToPreviousWordStart))
    } else if action.as_any().is::<MoveToPreviousWordStart>() {
        Some(Box::new(MoveToNextWordEnd))
    } else if action.as_any().is::<MoveToEndOfParagraph>() {
        Some(Box::new(MoveToStartOfParagraph))
    } else if action.as_any().is::<MoveToStartOfParagraph>() {
        Some(Box::new(MoveToEndOfParagraph))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniversalArgumentState {
    Plain { value: i32 },
    Numeric(UniversalArgumentNumericState),
}

impl UniversalArgumentState {
    pub fn new_plain() -> Self {
        Self::Plain { value: 4 }
    }

    pub fn multiply(self) -> Self {
        match self {
            Self::Plain { value } => Self::Plain {
                value: cap_universal_argument_magnitude(value.saturating_mul(4)),
            },
            Self::Numeric(_) => Self::new_plain(),
        }
    }

    pub fn push_digit(self, digit: usize) -> Self {
        let digit = digit.min(9) as i32;
        match self {
            Self::Plain { .. } => Self::Numeric(UniversalArgumentNumericState {
                magnitude: digit,
                is_negative: false,
                has_digits: true,
            }),
            Self::Numeric(numeric) => Self::Numeric(numeric.push_digit(digit)),
        }
    }

    pub fn apply_minus(self) -> Self {
        match self {
            Self::Plain { .. } => Self::Numeric(UniversalArgumentNumericState {
                magnitude: 0,
                is_negative: true,
                has_digits: false,
            }),
            Self::Numeric(numeric) => Self::Numeric(numeric.apply_minus()),
        }
    }

    pub fn accepts_minus(self) -> bool {
        match self {
            Self::Plain { .. } => true,
            Self::Numeric(numeric) => !numeric.has_digits(),
        }
    }

    pub fn resolve(self) -> ResolvedUniversalArgument {
        match self {
            Self::Plain { value } => ResolvedUniversalArgument::Plain(value),
            Self::Numeric(numeric) => ResolvedUniversalArgument::Numeric(numeric.value()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniversalArgumentNumericState {
    magnitude: i32,
    is_negative: bool,
    has_digits: bool,
}

impl UniversalArgumentNumericState {
    fn push_digit(mut self, digit: i32) -> Self {
        self.magnitude = cap_universal_argument_magnitude(
            self.magnitude.saturating_mul(10).saturating_add(digit),
        );
        self.has_digits = true;
        self
    }

    fn apply_minus(mut self) -> Self {
        self.is_negative = !self.is_negative;
        self
    }

    fn has_digits(self) -> bool {
        self.has_digits
    }

    fn value(self) -> i32 {
        let magnitude = if self.has_digits { self.magnitude } else { 1 };
        if self.is_negative {
            -magnitude
        } else {
            magnitude
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolvedUniversalArgument {
    Plain(i32),
    Numeric(i32),
}

impl ResolvedUniversalArgument {
    pub fn numeric_value(self) -> i32 {
        match self {
            Self::Plain(value) | Self::Numeric(value) => value,
        }
    }
}

fn cap_universal_argument_magnitude(value: i32) -> i32 {
    value.clamp(-MAX_UNIVERSAL_REPEAT, MAX_UNIVERSAL_REPEAT)
}
