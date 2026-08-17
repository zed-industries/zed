use std::env;

fn main() {
    cc::Build::new()
        .warnings(false)
        .cpp(true)
        .file("src/implementation.cc")
        .flag_if_supported("-std=c++11")
        .compile("tz_haiku");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/implementation.cc");
    println!("cargo:rerun-if-changed=src/interface.h");

    let target = env::var_os("TARGET").expect("cargo should set TARGET env var");
    let target = target
        .to_str()
        .expect("TARGET env var should be valid UTF-8");
    if target.contains("haiku") {
        println!("cargo:rustc-link-lib=be");
    }
}
