#![deny(warnings)]

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Some(channel) = version_check::Channel::read() {
        if channel.supports_features() {
            println!("cargo:rustc-cfg=feature=\"specialize\"");
            if version_check::Version::read().map_or(false, |v| v.at_most("1.77.9")) {
                println!("cargo:rustc-cfg=feature=\"stdsimd\"");
            }
        }
    }
    let os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS was not set");
    if os.eq_ignore_ascii_case("linux")
        || os.eq_ignore_ascii_case("android")
        || os.eq_ignore_ascii_case("windows")
        || os.eq_ignore_ascii_case("macos")
        || os.eq_ignore_ascii_case("ios")
        || os.eq_ignore_ascii_case("freebsd")
        || os.eq_ignore_ascii_case("openbsd")
        || os.eq_ignore_ascii_case("dragonfly")
        || os.eq_ignore_ascii_case("solaris")
        || os.eq_ignore_ascii_case("illumos")
        || os.eq_ignore_ascii_case("fuchsia")
        || os.eq_ignore_ascii_case("redox")
        || os.eq_ignore_ascii_case("cloudabi")
        || os.eq_ignore_ascii_case("haiku")
        || os.eq_ignore_ascii_case("vxworks")
        || os.eq_ignore_ascii_case("emscripten")
        || os.eq_ignore_ascii_case("wasi")
    {
        println!("cargo:rustc-cfg=feature=\"runtime-rng\"");
    }
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH was not set");
    if arch.eq_ignore_ascii_case("x86_64")
        || arch.eq_ignore_ascii_case("aarch64")
        || arch.eq_ignore_ascii_case("mips64")
        || arch.eq_ignore_ascii_case("powerpc64")
        || arch.eq_ignore_ascii_case("riscv64gc")
        || arch.eq_ignore_ascii_case("s390x")
    {
        println!("cargo:rustc-cfg=feature=\"folded_multiply\"");
    }

}
