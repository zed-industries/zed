use gpui::Global;

pub const MAX_UNIVERSAL_REPEAT: i32 = 1_000_000;

#[derive(Default)]
pub struct UniversalArgumentGlobals {
    pub(crate) state: Option<UniversalArgumentState>,
    pub(crate) consumed_this_dispatch: bool,
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
