use std::env;

fn main() {
    let mut has_atomic32 = true;
    let mut has_atomic64 = true;

    let target = env::var("TARGET").unwrap();

    // Full target triples that have specific limitations:
    match target.as_str() {
        "arm-linux-androideabi"
        | "asmjs-unknown-emscripten"
        | "wasm32-unknown-emscripten"
        | "wasm32-unknown-unknown" => has_atomic64 = false,
        _ => {}
    }

    // Architecture-specific limitations:
    let arch = target.split("-").next().unwrap_or(&target);
    match arch {
        // NOTE: Not all ARMv7 variants are listed here, as certain variants do actually provide
        // 64-bit atomics. (`armv7`, `armv7a`, and `armv7s`, specifically)
        "armv5te" | "mips" | "mipsel" | "powerpc" | "riscv32imac" | "riscv32imafc"
        | "riscv32imafdc" | "thumbv7em" | "thumbv7m" | "thumbv8m.base" | "thumbv8m.main"
        | "armebv7r" | "armv7r" => {
            has_atomic64 = false;
        }
        "avr" | "riscv32i" | "riscv32im" | "riscv32imc" | "thumbv6m" => {
            has_atomic32 = false;
            has_atomic64 = false;
        }
        _ => {}
    }

    if has_atomic64 {
        println!("cargo:rustc-cfg=has_atomics_64");
    }

    if has_atomic32 {
        println!("cargo:rustc-cfg=has_atomics");
    }
}
