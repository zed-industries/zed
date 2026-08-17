pub type Instant = std::time::Instant;
pub type SystemTime = std::time::SystemTime;

/// The current time, expressed in milliseconds since the Unix Epoch.
pub fn now() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH)
                                .expect("System clock was before 1970.")
                                .as_secs_f64() * 1000.0
}
