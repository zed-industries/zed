extern crate libloading;
use libloading::library_filename;
use std::path::Path;

#[cfg(any(target_os = "windows", target_os = "cygwin"))]
const EXPECTED: &str = "audioengine.dll";
#[cfg(target_os = "linux")]
const EXPECTED: &str = "libaudioengine.so";
#[cfg(target_os = "macos")]
const EXPECTED: &str = "libaudioengine.dylib";

#[test]
fn test_library_filename() {
    let name = "audioengine";
    let resolved = library_filename(name);
    assert!(Path::new(&resolved).ends_with(EXPECTED));
}
