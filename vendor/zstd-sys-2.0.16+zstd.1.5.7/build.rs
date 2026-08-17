use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

#[cfg(feature = "bindgen")]
fn generate_bindings(defs: Vec<&str>, headerpaths: Vec<PathBuf>) {
    use bindgen::RustTarget;

    let bindings = bindgen::Builder::default().header("zstd.h");

    #[cfg(feature = "zdict_builder")]
    let bindings = bindings.header("zdict.h");

    #[cfg(feature = "seekable")]
    let bindings = bindings.header("zstd_seekable.h");

    let bindings = bindings
        .layout_tests(false)
        .blocklist_type("max_align_t")
        .size_t_is_usize(true)
        .rust_target(
            RustTarget::stable(64, 0)
                .ok()
                .expect("Could not get 1.64.0 version"),
        )
        .use_core()
        .rustified_enum(".*")
        .clang_args(
            headerpaths
                .into_iter()
                .map(|path| format!("-I{}", path.display())),
        )
        .clang_args(defs.into_iter().map(|def| format!("-D{}", def)));

    #[cfg(feature = "experimental")]
    let bindings = bindings
        .clang_arg("-DZSTD_STATIC_LINKING_ONLY")
        .clang_arg("-DZDICT_STATIC_LINKING_ONLY")
        .clang_arg("-DZSTD_RUST_BINDINGS_EXPERIMENTAL");

    #[cfg(feature = "seekable")]
    let bindings = bindings.blocklist_function("ZSTD_seekable_initFile");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings(_: Vec<&str>, _: Vec<PathBuf>) {}

fn pkg_config() -> (Vec<&'static str>, Vec<PathBuf>) {
    let library = pkg_config::Config::new()
        .statik(true)
        .cargo_metadata(!cfg!(feature = "non-cargo"))
        .probe("libzstd")
        .expect("Can't probe for zstd in pkg-config");
    (vec!["PKG_CONFIG"], library.include_paths)
}

#[cfg(not(feature = "legacy"))]
fn set_legacy(_config: &mut cc::Build) {}

#[cfg(feature = "legacy")]
fn set_legacy(config: &mut cc::Build) {
    config.define("ZSTD_LEGACY_SUPPORT", Some("1"));
    config.include("zstd/lib/legacy");
}

#[cfg(feature = "zstdmt")]
fn set_pthread(config: &mut cc::Build) {
    config.flag("-pthread");
}

#[cfg(not(feature = "zstdmt"))]
fn set_pthread(_config: &mut cc::Build) {}

#[cfg(feature = "zstdmt")]
fn enable_threading(config: &mut cc::Build) {
    config.define("ZSTD_MULTITHREAD", Some(""));
}

#[cfg(not(feature = "zstdmt"))]
fn enable_threading(_config: &mut cc::Build) {}

/// This function would find the first flag in `flags` that is supported
/// and add that to `config`.
#[allow(dead_code)]
fn flag_if_supported_with_fallbacks(config: &mut cc::Build, flags: &[&str]) {
    let option = flags
        .iter()
        .find(|flag| config.is_flag_supported(flag).unwrap_or_default());

    if let Some(flag) = option {
        config.flag(flag);
    }
}

fn compile_zstd() {
    let mut config = cc::Build::new();

    // Search the following directories for C files to add to the compilation.
    for dir in &[
        "zstd/lib/common",
        "zstd/lib/compress",
        "zstd/lib/decompress",
        #[cfg(feature = "seekable")]
        "zstd/contrib/seekable_format",
        #[cfg(feature = "zdict_builder")]
        "zstd/lib/dictBuilder",
        #[cfg(feature = "legacy")]
        "zstd/lib/legacy",
    ] {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .unwrap()
            .map(Result::unwrap)
            .filter_map(|entry| {
                let filename = entry.file_name();

                if Path::new(&filename).extension() == Some(OsStr::new("c"))
                    // Skip xxhash*.c files: since we are using the "PRIVATE API"
                    // mode, it will be inlined in the headers.
                    && !filename.to_string_lossy().contains("xxhash")
                {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();
        entries.sort();

        config.files(entries);
    }

    // Either include ASM files, or disable ASM entirely.
    // Also disable it on windows, apparently it doesn't do well with these .S files at the moment.
    if cfg!(feature = "no_asm") || std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        config.define("ZSTD_DISABLE_ASM", Some(""));
    } else {
        config.file("zstd/lib/decompress/huf_decompress_amd64.S");
    }

    // List out the WASM targets that need wasm-shim.
    // Note that Emscripten already provides its own C standard library so
    // wasm32-unknown-emscripten should not be included here.
    // See: https://github.com/gyscos/zstd-rs/pull/209
    let need_wasm_shim = !cfg!(feature = "no_wasm_shim")
        && env::var("TARGET").map_or(false, |target| {
            target == "wasm32-unknown-unknown"
                || target.starts_with("wasm32-wasi")
        });

    if need_wasm_shim {
        cargo_print(&"rerun-if-changed=wasm-shim/stdlib.h");
        cargo_print(&"rerun-if-changed=wasm-shim/string.h");

        config.include("wasm-shim/");
    }

    // Some extra parameters
    config.include("zstd/lib/");
    config.include("zstd/lib/common");
    config.warnings(false);

    config.define("ZSTD_LIB_DEPRECATED", Some("0"));

    config
        .flag_if_supported("-ffunction-sections")
        .flag_if_supported("-fdata-sections")
        .flag_if_supported("-fmerge-all-constants");

    if cfg!(feature = "fat-lto") {
        config.flag_if_supported("-flto");
    } else if cfg!(feature = "thin-lto") {
        flag_if_supported_with_fallbacks(
            &mut config,
            &["-flto=thin", "-flto"],
        );
    }

    #[cfg(feature = "thin")]
    {
        // Here we try to build a lib as thin/small as possible.
        // We cannot use ZSTD_LIB_MINIFY since it is only
        // used in Makefile to define other options.

        config
            .define("HUF_FORCE_DECOMPRESS_X1", Some("1"))
            .define("ZSTD_FORCE_DECOMPRESS_SEQUENCES_SHORT", Some("1"))
            .define("ZSTD_NO_INLINE", Some("1"))
            // removes the error messages that are
            // otherwise returned by ZSTD_getErrorName
            .define("ZSTD_STRIP_ERROR_STRINGS", Some("1"));

        // Disable use of BMI2 instructions since it involves runtime checking
        // of the feature and fallback if no BMI2 instruction is detected.
        config.define("DYNAMIC_BMI2", Some("0"));

        // Disable support for all legacy formats
        #[cfg(not(feature = "legacy"))]
        config.define("ZSTD_LEGACY_SUPPORT", Some("0"));

        config.opt_level_str("z");
    }

    // Hide symbols from resulting library,
    // so we can be used with another zstd-linking lib.
    // See https://github.com/gyscos/zstd-rs/issues/58
    config.flag("-fvisibility=hidden");
    config.define("XXH_PRIVATE_API", Some(""));
    config.define("ZSTDLIB_VISIBILITY", Some(""));
    #[cfg(feature = "zdict_builder")]
    config.define("ZDICTLIB_VISIBILITY", Some(""));
    config.define("ZSTDERRORLIB_VISIBILITY", Some(""));

    // https://github.com/facebook/zstd/blob/d69d08ed6c83563b57d98132e1e3f2487880781e/lib/common/debug.h#L60
    /* recommended values for DEBUGLEVEL :
     * 0 : release mode, no debug, all run-time checks disabled
     * 1 : enables assert() only, no display
     * 2 : reserved, for currently active debug path
     * 3 : events once per object lifetime (CCtx, CDict, etc.)
     * 4 : events once per frame
     * 5 : events once per block
     * 6 : events once per sequence (verbose)
     * 7+: events at every position (*very* verbose)
     */
    #[cfg(feature = "debug")]
    if !need_wasm_shim {
        config.define("DEBUGLEVEL", Some("5"));
    }

    set_pthread(&mut config);
    set_legacy(&mut config);
    enable_threading(&mut config);

    // Compile!
    config.compile("libzstd.a");

    let src = env::current_dir().unwrap().join("zstd").join("lib");
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy(src.join("zstd.h"), include.join("zstd.h")).unwrap();
    fs::copy(src.join("zstd_errors.h"), include.join("zstd_errors.h"))
        .unwrap();
    #[cfg(feature = "zdict_builder")]
    fs::copy(src.join("zdict.h"), include.join("zdict.h")).unwrap();
    cargo_print(&format_args!("root={}", dst.display()));
}

/// Print a line for cargo.
///
/// If non-cargo is set, do not print anything.
fn cargo_print(content: &dyn fmt::Display) {
    if cfg!(not(feature = "non-cargo")) {
        println!("cargo:{}", content);
    }
}

fn main() {
    cargo_print(&"rerun-if-env-changed=ZSTD_SYS_USE_PKG_CONFIG");

    let target_arch =
        std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_arch == "wasm32" || target_os == "hermit" {
        cargo_print(&"rustc-cfg=feature=\"std\"");
    }

    // println!("cargo:rustc-link-lib=zstd");
    let (defs, headerpaths) = if cfg!(feature = "pkg-config")
        || env::var_os("ZSTD_SYS_USE_PKG_CONFIG").is_some()
    {
        pkg_config()
    } else {
        if !Path::new("zstd/lib").exists() {
            panic!("Folder 'zstd/lib' does not exists. Maybe you forgot to clone the 'zstd' submodule?");
        }

        let manifest_dir = PathBuf::from(
            env::var_os("CARGO_MANIFEST_DIR")
                .expect("Manifest dir is always set by cargo"),
        );

        compile_zstd();
        (vec![], vec![manifest_dir.join("zstd/lib")])
    };

    let includes: Vec<_> = headerpaths
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    cargo_print(&format_args!("include={}", includes.join(";")));

    generate_bindings(defs, headerpaths);
}
