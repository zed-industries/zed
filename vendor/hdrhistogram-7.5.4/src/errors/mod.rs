//! Error types used throughout this library
use std::error::Error;
use std::fmt;

/// Errors that can occur when creating a histogram.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CreationError {
    /// Lowest discernible value must be >= 1.
    LowIsZero,
    /// Lowest discernible value must be <= `u64::max_value() / 2` because the highest value is
    /// a `u64` and the lowest value must be no bigger than half the highest.
    LowExceedsMax,
    /// Highest trackable value must be >= 2 * lowest discernible value for some internal
    /// calculations to work out. In practice, high is typically much higher than 2 * low.
    HighLessThanTwiceLow,
    /// Number of significant digits must be in the range `[0, 5]`. It is capped at 5 because 5
    /// significant digits is already more than almost anyone needs, and memory usage scales
    /// exponentially as this increases.
    SigFigExceedsMax,
    /// Cannot represent sigfig worth of values beyond the lowest discernible value. Decrease the
    /// significant figures, lowest discernible value, or both.
    ///
    /// This could happen if low is very large (like 2^60) and sigfigs is 5, which requires 18
    /// additional bits, which would then require more bits than will fit in a u64. Specifically,
    /// the exponent of the largest power of two that is smaller than the lowest value and the bits
    /// needed to represent the requested significant figures must sum to 63 or less.
    CannotRepresentSigFigBeyondLow,
    /// The `usize` type is too small to represent the desired configuration. Use fewer significant
    /// figures or a lower max.
    UsizeTypeTooSmall,
}

// TODO like RecordError, this is also an awkward split along resizing.
/// Errors that can occur when adding another histogram.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AdditionError {
    /// The other histogram includes values that do not fit in this histogram's range.
    /// Only possible when auto resize is disabled.
    OtherAddendValueExceedsRange,
    /// The other histogram includes values that would map to indexes in this histogram that are
    /// not expressible for `usize`. Configure this histogram to use fewer significant digits. Only
    /// possible when resize is enabled.
    ResizeFailedUsizeTypeTooSmall,
}

/// Errors that can occur when subtracting another histogram.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SubtractionError {
    /// The other histogram includes values that do not fit in this histogram's range.
    /// Only possible when auto resize is disabled.
    SubtrahendValueExceedsMinuendRange,
    /// The other histogram includes counts that are higher than the current count for a value, and
    /// counts cannot go negative. The subtraction may have been partially applied to some counts as
    /// this error is returned when the first impossible subtraction is detected.
    SubtrahendCountExceedsMinuendCount,
}

// TODO the error conditions here are awkward: one only possible when resize is disabled, the other
// only when resize is enabled.
/// Errors that can occur while recording a value and its associated count.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RecordError {
    /// The value to record is not representable in this histogram and resizing is disabled.
    /// Configure a higher maximum value or enable resizing. Only possible when resizing is
    /// disabled.
    ValueOutOfRangeResizeDisabled,
    /// Auto resizing is enabled and must be used to represent the provided value, but the histogram
    /// cannot be resized because `usize` cannot represent sufficient length. Configure this
    /// histogram to use fewer significant digits. Only possible when resizing is enabled.
    ResizeFailedUsizeTypeTooSmall,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct UsizeTypeTooSmall;

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreationError::LowIsZero => write!(f, "Lowest discernible value must be >= 1"),
            CreationError::LowExceedsMax => write!(f, "Lowest discernible value must be <= `u64::max_value() / 2`"),
            CreationError::HighLessThanTwiceLow => write!(f, "Highest trackable value must be >= 2 * lowest discernible value for some internal calculations"),
            CreationError::SigFigExceedsMax => write!(f, "Number of significant digits must be in the range `[0, 5]`"),
            CreationError::CannotRepresentSigFigBeyondLow => write!(f, "Cannot represent sigfig worth of values beyond the lowest discernible value"),
            CreationError::UsizeTypeTooSmall =>  write!(f, "The `usize` type is too small to represent the desired configuration"),
        }
    }
}

impl Error for CreationError {}

impl fmt::Display for AdditionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdditionError::OtherAddendValueExceedsRange => write!(f, "The other histogram includes values that do not fit in this histogram's range"),
            AdditionError::ResizeFailedUsizeTypeTooSmall => write!(f, "The other histogram includes values that would map to indexes in this histogram that are not expressible for `usize`"),
        }
    }
}

impl Error for AdditionError {}

impl fmt::Display for SubtractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubtractionError::SubtrahendValueExceedsMinuendRange => write!(f, "The other histogram includes values that do not fit in this histogram's range"),
            SubtractionError::SubtrahendCountExceedsMinuendCount => write!(f, "The other histogram includes counts that are higher than the current count for a value, and counts cannot go negative"),
        }
    }
}

impl Error for SubtractionError {}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordError::ValueOutOfRangeResizeDisabled  => write!(f, "The value to record is not representable in this histogram and resizing is disabled"),
            RecordError::ResizeFailedUsizeTypeTooSmall => write!(f, "Auto resizing is enabled and must be used to represent the provided value, but the histogram cannot be resized because `usize` cannot represent sufficient length"),
        }
    }
}

impl Error for RecordError {}

impl fmt::Display for UsizeTypeTooSmall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The `usize` type is too small to represent the desired configuration"
        )
    }
}

impl Error for UsizeTypeTooSmall {}
