use std::env;

fn main() {
    let mut build = cc::Build::new();

    let version = if env::var("CARGO_FEATURE_V3").is_ok() {
        "v3"
    } else {
        "v2"
    };

    build.include(format!("c_src/mimalloc/{version}/include"));
    build.include(format!("c_src/mimalloc/{version}/src"));
    build.file(format!("c_src/mimalloc/{version}/src/static.c"));

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target_os not defined!");
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").expect("target_family not defined!");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("target_arch not defined!");

    if target_family != "windows" {
        build.flag("-Wno-error=date-time");
    }

    if env::var_os("CARGO_FEATURE_OVERRIDE").is_some() {
        // Overriding malloc is only available on windows in shared mode, but we
        // only ever build a static lib.
        if target_family != "windows" {
            build.define("MI_MALLOC_OVERRIDE", None);
        }
    }

    if env::var_os("CARGO_FEATURE_SECURE").is_some() {
        build.define("MI_SECURE", "4");
    }

    let dynamic_tls = env::var("CARGO_FEATURE_LOCAL_DYNAMIC_TLS").is_ok();

    if target_family == "unix" && target_os != "haiku" {
        if dynamic_tls {
            build.flag_if_supported("-ftls-model=local-dynamic");
        } else {
            build.flag_if_supported("-ftls-model=initial-exec");
        }
    }

    if (target_os == "linux" || target_os == "android")
        && env::var_os("CARGO_FEATURE_NO_THP").is_some()
    {
        build.define("MI_NO_THP", "1");
    }

    if env::var_os("CARGO_FEATURE_DEBUG").is_some()
        || (env::var_os("CARGO_FEATURE_DEBUG_IN_DEBUG").is_some() && cfg!(debug_assertions))
    {
        build.define("MI_DEBUG", "3");
        build.define("MI_SHOW_ERRORS", "1");
    } else {
        // Remove heavy debug assertions etc
        build.define("MI_DEBUG", "0");
    }

    if build.get_compiler().is_like_msvc() {
        build.cpp(true);
    }

    build.compile("mimalloc");

    // on armv6 we need to link with libatomic
    if target_os == "linux" && target_arch == "arm" {
        // Embrace the atomic capability library across various platforms.
        // For instance, on certain platforms, llvm has relocated the atomic of the arm32 architecture to libclang_rt.builtins.a
        // while some use libatomic.a, and others use libatomic_ops.a.
        let atomic_name = env::var("DEP_ATOMIC").unwrap_or("atomic".to_owned());
        println!("cargo:rustc-link-lib={}", atomic_name);
    }

    // Link with libs needed on Windows
    if target_os == "windows" {
        // https://github.com/microsoft/mimalloc/blob/af21001f7a65eafb8fb16460b018ebf9d75e2ad8/CMakeLists.txt#L487
        let libs = ["psapi", "shell32", "user32", "advapi32", "bcrypt"];

        for lib in libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}
