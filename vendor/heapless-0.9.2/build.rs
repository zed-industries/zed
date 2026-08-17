use std::{
    env,
    error::Error,
    fs,
    path::Path,
    process::{Command, ExitStatus, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rustc-check-cfg=cfg(arm_llsc)");
    println!("cargo::rustc-check-cfg=cfg(has_atomic_load_store)");

    let target = env::var("TARGET")?;

    // Manually list targets that have atomic load/store, but no CAS.
    // Remove when `cfg(target_has_atomic_load_store)` is stable.
    // last updated nightly-2023-10-28
    match &target[..] {
        "armv4t-none-eabi"
        | "armv5te-none-eabi"
        | "avr-unknown-gnu-atmega328"
        | "bpfeb-unknown-none"
        | "bpfel-unknown-none"
        | "thumbv4t-none-eabi"
        | "thumbv5te-none-eabi"
        | "thumbv6m-none-eabi" => println!("cargo:rustc-cfg=has_atomic_load_store"),
        _ => {}
    };

    // AArch64 instruction set contains `clrex` but not `ldrex` or `strex`; the
    // probe will succeed when we already know to deny this target from LLSC.
    if !target.starts_with("aarch64") {
        match compile_probe(ARM_LLSC_PROBE) {
            Some(status) if status.success() => println!("cargo:rustc-cfg=arm_llsc"),
            _ => {}
        }
    }

    Ok(())
}

const ARM_LLSC_PROBE: &str = r#"
#![no_std]

// `no_mangle` forces codegen, which makes llvm check the contents of the `asm!` macro
#[no_mangle]
unsafe fn asm() {
    core::arch::asm!("clrex");
}
"#;

// this function was taken from anyhow v1.0.63 build script
// https://crates.io/crates/anyhow/1.0.63 (last visited 2022-09-02)
// the code is licensed under 'MIT or APACHE-2.0'
fn compile_probe(source: &str) -> Option<ExitStatus> {
    let rustc = env::var_os("RUSTC")?;
    let out_dir = env::var_os("OUT_DIR")?;
    let probefile = Path::new(&out_dir).join("probe.rs");
    fs::write(&probefile, source).ok()?;

    // Make sure to pick up Cargo rustc configuration.
    let mut cmd = if let Some(wrapper) = env::var_os("RUSTC_WRAPPER") {
        let mut cmd = Command::new(wrapper);
        // The wrapper's first argument is supposed to be the path to rustc.
        cmd.arg(rustc);
        cmd
    } else {
        Command::new(rustc)
    };

    cmd.stderr(Stdio::null())
        .arg("--edition=2018")
        .arg("--crate-name=probe")
        .arg("--crate-type=lib")
        .arg("--out-dir")
        .arg(out_dir)
        .arg(probefile);

    if let Some(target) = env::var_os("TARGET") {
        cmd.arg("--target").arg(target);
    }

    // If Cargo wants to set RUSTFLAGS, use that.
    if let Ok(rustflags) = env::var("CARGO_ENCODED_RUSTFLAGS") {
        if !rustflags.is_empty() {
            for arg in rustflags.split('\x1f') {
                cmd.arg(arg);
            }
        }
    }

    cmd.status().ok()
}
