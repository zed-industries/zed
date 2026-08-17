use std::env;

#[rustversion::nightly]
const NIGHTLY: bool = true;

#[rustversion::not(nightly)]
const NIGHTLY: bool = false;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let external_assembly_required = target_arch == "powerpc64";
    println!("target: {target_arch}, ext: {external_assembly_required}");

    println!("cargo:rustc-check-cfg=cfg(nightly)");
    if NIGHTLY {
        println!("cargo:rustc-cfg=nightly");
    }

    if external_assembly_required {
        cc::Build::new()
            .file("src/detail/asm/asm_ppc64le_elf.S")
            .compile("ppc64le-asm-lib");
    }
}
