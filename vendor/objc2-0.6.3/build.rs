//! Helper script to work around MSRV being too low for `target_abi`.
use std::env;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    let target_abi = env::var("CARGO_CFG_TARGET_ABI");

    // Used to figure out when BOOL should be i8 vs. bool
    // Matches:
    // aarch64-apple-ios-macabi
    // x86_64-apple-ios-macabi
    println!("cargo:rustc-check-cfg=cfg(target_abi_macabi)");
    if target_abi.as_deref() == Ok("macabi")
        // Legacy check for Rust versions < 1.78 (still within MSRV)
        || target.ends_with("macabi")
    {
        println!("cargo:rustc-cfg=target_abi_macabi");
    }

    // Used to set correct image info
    // Matches:
    // aarch64-apple-ios-sim
    // aarch64-apple-watchos-sim
    // x86_64-apple-watchos-sim
    // i386-apple-ios
    // x86_64-apple-ios
    println!("cargo:rustc-check-cfg=cfg(target_simulator)");
    if target_abi.as_deref() == Ok("sim")
        // Legacy check for Rust versions < 1.78 (still within MSRV)
        || target.ends_with("sim")
        || target == "i386-apple-ios"
        || target == "x86_64-apple-ios"
    {
        println!("cargo:rustc-cfg=target_simulator");
    }
}
