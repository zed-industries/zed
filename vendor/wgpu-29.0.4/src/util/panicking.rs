#[cfg(feature = "std")]
pub fn is_panicking() -> bool {
    std::thread::panicking()
}

#[cfg(not(feature = "std"))]
pub fn is_panicking() -> bool {
    false
}
