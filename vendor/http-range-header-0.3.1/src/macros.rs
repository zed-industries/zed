#[cfg(feature = "with_error_cause")]
macro_rules! invalid {
    ($create:expr) => {
        Err(RangeUnsatisfiableError::new($create))
    };
}

#[cfg(not(feature = "with_error_cause"))]
macro_rules! invalid {
    ($create:expr) => {
        Err(RangeUnsatisfiableError)
    };
}
