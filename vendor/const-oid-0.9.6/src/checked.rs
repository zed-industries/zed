//! Checked arithmetic helpers.

/// `const fn`-friendly checked addition helper.
macro_rules! checked_add {
    ($a:expr, $b:expr) => {
        match $a.checked_add($b) {
            Some(n) => n,
            None => return Err(Error::Length),
        }
    };
}
