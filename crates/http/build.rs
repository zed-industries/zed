use std::env;

fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS");
    match target.as_deref() {
        Ok("macos") => {
            #[cfg(target_os = "macos")]
            macos::build();
        }
        _ => (),
    };
}

#[cfg(target_os = "macos")]
mod macos {
    use std::{env, path::PathBuf};

    pub(super) fn build() {
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");
        gen_system_configuration();
    }

    fn gen_system_configuration() {
        println!("cargo:rerun-if-changed=src/proxy.macos/system_configuration.h");
        let bindings = bindgen::Builder::default()
            .generate_comments(true)
            .clang_arg(format!("-isysroot{}", sdk_path()))
            .header("src/proxy/macos/system_configuration.h")
            .allowlist_var("kSCPropNetProxiesHTTP.*")
            .allowlist_var("kSCPropNetProxiesSOCKS.*")
            .allowlist_var("kSCDynamicStoreUseSessionKeys")
            .allowlist_function("SCDynamicStoreCopyProxies")
            .allowlist_function("SCDynamicStoreCreateRunLoopSource")
            .allowlist_function("SCDynamicStoreCreateWithOptions")
            .allowlist_function("SCDynamicStoreGetTypeID")
            .allowlist_function("SCDynamicStoreSetNotificationKeys")
            .allowlist_function("SCDynamicStoreKeyCreateProxies")
            .blocklist_item("CF.*")
            .blocklist_item("__CF.*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .layout_tests(false)
            .generate()
            .expect("unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("system_configuration_sys.rs"))
            .expect("couldn't write system configuration_sys bindings");
    }

    fn sdk_path() -> String {
        let output = std::process::Command::new("xcrun")
            .args("--sdk macosx --show-sdk-path".split(' '))
            .output()
            .expect("failed to execute xcrun");

        if !output.status.success() {
            panic!("xcrun command failed");
        }

        std::str::from_utf8(&output.stdout)
            .expect("Invalid UTF-8 sequence")
            .trim()
            .to_string()
    }
}
