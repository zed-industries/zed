use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

fn rerun_dir<P: AsRef<Path>>(dir: P) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("cargo:rerun-if-changed={}", path.to_string_lossy());

        if path.is_dir() {
            rerun_dir(path);
        }
    }
}

fn hash_changed(files: &[&str], out_dir: &str, config: &Path) -> Option<([u8; 8], PathBuf)> {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    let mut hasher = DefaultHasher::new();

    let paths = files
        .iter()
        .map(Path::new)
        .chain(std::iter::once(config))
        .chain(std::iter::once(Path::new("build.rs")));

    for path in paths {
        if let Ok(buf) = std::fs::read(path) {
            hasher.write(&buf);
        } else {
            panic!("Cannot open {}", path.display());
        }
    }

    if let Some(cmd) = strip_command() {
        hasher.write(cmd.as_bytes());
    }

    let hash = hasher.finish().to_be_bytes();

    let hash_path = Path::new(&out_dir).join("asm.hash");

    if let Ok(old_hash) = std::fs::read(&hash_path) {
        if old_hash == hash {
            return None;
        }
    }

    Some((hash, hash_path))
}

#[cfg(feature = "asm")]
fn build_nasm_files() {
    let mut config = "
%pragma preproc sane_empty_expansion true
%define private_prefix avsc
%define ARCH_X86_32 0
%define ARCH_X86_64 1
%define PIC 1
%define STACK_ALIGNMENT 16
%define HAVE_AVX512ICL 1
"
    .to_owned();

    if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
        config += "%define PREFIX 1\n";
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.asm");
    std::fs::write(&dest_path, config).expect("can write config.asm");

    let asm_files = &[
        // "src/asm/x86/cdef_avx2.asm",
        // "src/asm/x86/cdef_avx512.asm",
        // "src/asm/x86/cdef_dist.asm",
        // "src/asm/x86/cdef_rav1e.asm",
        // "src/asm/x86/cdef_sse.asm",
        // "src/asm/x86/cdef16_avx2.asm",
        // "src/asm/x86/cdef16_avx512.asm",
        // "src/asm/x86/cdef16_sse.asm",
        "src/asm/x86/ipred_avx2.asm",
        "src/asm/x86/ipred_avx512.asm",
        "src/asm/x86/ipred_sse.asm",
        "src/asm/x86/ipred16_avx2.asm",
        "src/asm/x86/ipred16_avx512.asm",
        "src/asm/x86/ipred16_sse.asm",
        // "src/asm/x86/itx_avx2.asm",
        // "src/asm/x86/itx_avx512.asm",
        // "src/asm/x86/itx_sse.asm",
        // "src/asm/x86/itx16_avx2.asm",
        // "src/asm/x86/itx16_avx512.asm",
        // "src/asm/x86/itx16_sse.asm",
        // "src/asm/x86/looprestoration_avx2.asm",
        // "src/asm/x86/looprestoration_avx512.asm",
        // "src/asm/x86/looprestoration_sse.asm",
        // "src/asm/x86/looprestoration16_avx2.asm",
        // "src/asm/x86/looprestoration16_avx512.asm",
        // "src/asm/x86/looprestoration16_sse.asm",
        "src/asm/x86/mc_avx2.asm",
        "src/asm/x86/mc_avx512.asm",
        "src/asm/x86/mc_sse.asm",
        "src/asm/x86/mc16_avx2.asm",
        "src/asm/x86/mc16_avx512.asm",
        "src/asm/x86/mc16_sse.asm",
        // "src/asm/x86/me.asm",
        "src/asm/x86/sad_avx.asm",
        "src/asm/x86/sad_plane.asm",
        "src/asm/x86/sad_sse2.asm",
        "src/asm/x86/satd.asm",
        "src/asm/x86/satd16_avx2.asm",
        // "src/asm/x86/sse.asm",
        "src/asm/x86/tables.asm",
    ];

    if let Some((hash, hash_path)) = hash_changed(asm_files, &out_dir, &dest_path) {
        let obj = nasm_rs::Build::new()
      .min_version(2, 15, 0)
      .include(&out_dir)
      .include("src")
      .files(asm_files)
      .compile_objects()
      .unwrap_or_else(|e| {
        panic!("NASM build failed. Make sure you have nasm installed or disable the \"asm\" feature.\n\
                You can get NASM from https://nasm.us or your system's package manager.\n\
                \n\
                error: {e}");
    });

        // cc is better at finding the correct archiver
        let mut cc = cc::Build::new();
        for o in obj {
            cc.object(o);
        }
        cc.compile("avscasm");

        // Strip local symbols from the asm library since they
        // confuse the debugger.
        if let Some(strip) = strip_command() {
            let _ = std::process::Command::new(strip)
                .arg("-x")
                .arg(Path::new(&out_dir).join("libavscasm.a"))
                .status();
        }

        std::fs::write(hash_path, &hash[..]).unwrap();
    } else {
        println!("cargo:rustc-link-search={out_dir}");
    }
    println!("cargo:rustc-link-lib=static=avscasm");
    rerun_dir("src/asm/x86");
}

fn strip_command() -> Option<String> {
    let target = env::var("TARGET").expect("TARGET");
    // follows Cargo's naming convention for the linker setting
    let normalized_target = target.replace('-', "_").to_uppercase();
    let explicit_strip = env::var(format!("CARGO_TARGET_{normalized_target}_STRIP"))
        .ok()
        .or_else(|| env::var("STRIP").ok());
    if explicit_strip.is_some() {
        return explicit_strip;
    }

    // strip command is target-specific, e.g. macOS's strip breaks MUSL's archives
    let host = env::var("HOST").expect("HOST");
    if host != target {
        return None;
    }

    Some("strip".into())
}

#[cfg(feature = "asm")]
fn build_neon_asm_files() {
    let mut config = "
#define PRIVATE_PREFIX avsc_
#define ARCH_AARCH64 1
#define ARCH_ARM 0
#define CONFIG_LOG 1
#define HAVE_ASM 1
"
    .to_owned();

    if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
        config += "#define PREFIX 1\n";
    }
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.h");
    std::fs::write(&dest_path, config).expect("can write config.h");

    let asm_files = &[
        // "src/asm/arm/64/cdef.S",
        // "src/asm/arm/64/cdef16.S",
        // "src/asm/arm/64/cdef_dist.S",
        "src/asm/arm/64/mc.S",
        "src/asm/arm/64/mc16.S",
        // "src/asm/arm/64/itx.S",
        // "src/asm/arm/64/itx16.S",
        "src/asm/arm/64/ipred.S",
        "src/asm/arm/64/ipred16.S",
        // "src/asm/arm/64/sad.S",
        "src/asm/arm/64/satd.S",
        // "src/asm/arm/64/sse.S",
        "src/asm/arm/tables.S",
    ];

    if let Some((hash, hash_path)) = hash_changed(asm_files, &out_dir, &dest_path) {
        cc::Build::new()
            .files(asm_files)
            .include(".")
            .include(&out_dir)
            .compile("avsc-aarch64");

        std::fs::write(hash_path, &hash[..]).unwrap();
    } else {
        println!("cargo:rustc-link-search={out_dir}");
        println!("cargo:rustc-link-lib=static=avsc-aarch64");
    }
    rerun_dir("src/asm/arm");
}

#[allow(unused_variables)]
fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    // let env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    #[cfg(feature = "asm")]
    {
        if arch == "x86_64" {
            println!("cargo:rustc-cfg=asm_x86_64");
            build_nasm_files()
        }
        if arch == "aarch64" {
            println!("cargo:rustc-cfg=asm_neon");
            build_neon_asm_files()
        }
    }

    println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap());
    if let Ok(value) = env::var("CARGO_CFG_TARGET_FEATURE") {
        println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE={value}");
    }
    println!(
        "cargo:rustc-env=CARGO_ENCODED_RUSTFLAGS={}",
        env::var("CARGO_ENCODED_RUSTFLAGS").unwrap()
    );
}
