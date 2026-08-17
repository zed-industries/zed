use std::ptr;

use clang_sys::*;

use libc::c_char;

fn parse() {
    unsafe {
        let index = clang_createIndex(0, 0);
        assert!(!index.is_null());

        let tu = clang_parseTranslationUnit(
            index,
            "tests/header.h\0".as_ptr() as *const c_char,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            0,
            0,
        );
        assert!(!tu.is_null());
    }
}

#[cfg(feature = "runtime")]
#[test]
fn test() {
    load().unwrap();
    let library = get_library().unwrap();
    println!("{:?} ({:?})", library.version(), library.path());
    parse();
    unload().unwrap();
}

#[cfg(not(feature = "runtime"))]
#[test]
fn test() {
    parse();
}

#[test]
fn test_support() {
    let clang = support::Clang::find(None, &[]).unwrap();
    println!("{:?}", clang);
}

#[test]
fn test_support_target() {
    let args = &["--target".into(), "x86_64-unknown-linux-gnu".into()];
    let clang = support::Clang::find(None, args).unwrap();
    println!("{:?}", clang);
}
