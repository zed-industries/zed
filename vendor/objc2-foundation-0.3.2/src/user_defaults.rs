use crate::NSUserDefaults;

// Documented to be thread-safe, and should be, it's a singleton that's
// accessible from any thread using `standardUserDefaults`.
unsafe impl Send for NSUserDefaults {}
unsafe impl Sync for NSUserDefaults {}
