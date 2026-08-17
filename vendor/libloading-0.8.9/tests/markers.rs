extern crate libloading;

#[cfg(test)]
fn assert_send<T: Send>() {}
#[cfg(test)]
fn assert_sync<T: Sync>() {}
#[cfg(test)]
fn assert_display<T: std::fmt::Display>() {}

#[test]
fn check_error_send() {
    assert_send::<libloading::Error>();
}

#[test]
fn check_error_sync() {
    assert_sync::<libloading::Error>();
}

#[test]
fn check_error_display() {
    assert_display::<libloading::Error>();
}

#[test]
fn check_library_send() {
    assert_send::<libloading::Library>();
}

#[cfg(unix)]
#[test]
fn check_unix_library_send() {
    assert_send::<libloading::os::unix::Library>();
}

#[cfg(windows)]
#[test]
fn check_windows_library_send() {
    assert_send::<libloading::os::windows::Library>();
}

#[test]
fn check_library_sync() {
    assert_sync::<libloading::Library>();
}

#[cfg(unix)]
#[test]
fn check_unix_library_sync() {
    assert_sync::<libloading::os::unix::Library>();
}

#[cfg(windows)]
#[test]
fn check_windows_library_sync() {
    assert_sync::<libloading::os::windows::Library>();
}

#[test]
fn check_symbol_send() {
    assert_send::<libloading::Symbol<fn() -> ()>>();
    // assert_not_send::<libloading::Symbol<*const ()>>();
}

#[cfg(unix)]
#[test]
fn check_unix_symbol_send() {
    assert_send::<libloading::os::unix::Symbol<fn() -> ()>>();
    // assert_not_send::<libloading::os::unix::Symbol<*const ()>>();
}

#[cfg(windows)]
#[test]
fn check_windows_symbol_send() {
    assert_send::<libloading::os::windows::Symbol<fn() -> ()>>();
}

#[test]
fn check_symbol_sync() {
    assert_sync::<libloading::Symbol<fn() -> ()>>();
    // assert_not_sync::<libloading::Symbol<*const ()>>();
}

#[cfg(unix)]
#[test]
fn check_unix_symbol_sync() {
    assert_sync::<libloading::os::unix::Symbol<fn() -> ()>>();
    // assert_not_sync::<libloading::os::unix::Symbol<*const ()>>();
}

#[cfg(windows)]
#[test]
fn check_windows_symbol_sync() {
    assert_sync::<libloading::os::windows::Symbol<fn() -> ()>>();
    // assert_not_sync::<libloading::os::windows::Symbol<*const ()>>();
}
