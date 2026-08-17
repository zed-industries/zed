//! This is ignored on mac when targeting arm as the way sadness-generator does
//! this on that arch is to explicitly raise SIGFPE, however we don't actually
//! handle that signal and rely on the exception port instead
#![cfg(not(all(target_os = "macos", any(target_arch = "arm", target_arch = "aarch64"))))]

mod shared;

#[test]
fn handles_fpe() {
    shared::handles_crash(shared::SadnessFlavor::DivideByZero);
}
