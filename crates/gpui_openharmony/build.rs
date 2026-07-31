fn main() {
    println!("cargo:rerun-if-changed=src/clipboard.rs");
    println!("cargo:rerun-if-changed=src/display_info.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if target_env == "ohos" {
        // Add the OHOS sysroot lib directory to the linker search path.
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let target_triple = format!("{}-unknown-linux-ohos", target_arch);

        // Try CC_<triple> env var first (set by user for cross-compilation).
        let cc_env = format!("CC_{}", target_triple.replace('-', "_"));
        let mut search_path = None;
        if let Ok(cc) = std::env::var(&cc_env) {
            if let Ok(output) = std::process::Command::new(&cc)
                .arg("--print-sysroot")
                .output()
            {
                let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !sysroot.is_empty() {
                    search_path = Some(format!("{}/usr/lib/{}", sysroot, target_triple));
                }
            }
        }
        // Fallback: OHOS_NDK_HOME.
        if search_path.is_none() {
            if let Ok(ndk) = std::env::var("OHOS_NDK_HOME") {
                search_path = Some(format!(
                    "{}/sysroot/usr/lib/{}",
                    ndk, target_triple
                ));
            }
        }
        if let Some(path) = search_path {
            println!("cargo:rustc-link-search=native={}", path);
        }

        // Clipboard: libpasteboard.so (system pasteboard) + libudmf.so (UDMF)
        println!("cargo:rustc-link-lib=dylib=pasteboard");
        println!("cargo:rustc-link-lib=dylib=udmf");
        // Display: libnative_display_manager.so
        println!("cargo:rustc-link-lib=dylib=native_display_manager");
    }
}
