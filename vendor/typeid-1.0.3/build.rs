use std::env;
use std::process::Command;
use std::str;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let compiler = match rustc_minor_version() {
        Some(compiler) => compiler,
        None => return,
    };

    if compiler >= 80 {
        println!("cargo:rustc-check-cfg=cfg(no_const_type_id)");
    }

    if compiler < 61 {
        // Function pointer casting in const fn.
        // https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html#more-capabilities-for-const-fn
        println!("cargo:rustc-cfg=no_const_type_id");
    }
}

fn rustc_minor_version() -> Option<u32> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    pieces.next()?.parse().ok()
}
