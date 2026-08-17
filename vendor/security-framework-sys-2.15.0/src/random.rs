use std::os::raw::{c_int, c_void};

pub enum __SecRandom {}
pub type SecRandomRef = *const __SecRandom;

extern "C" {
    pub static kSecRandomDefault: SecRandomRef;

    pub fn SecRandomCopyBytes(rnd: SecRandomRef, count: usize, bytes: *mut c_void) -> c_int;
}
