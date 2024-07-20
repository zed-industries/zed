use std::env;

fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS");
    println!("cargo::rustc-check-cfg=cfg(gles)");
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
        generate_dispatch_bindings();
    }

    fn generate_dispatch_bindings() {
        println!("cargo:rustc-link-lib=framework=System");
        println!("cargo:rerun-if-changed=src/platform/mac/dispatch.h");

        let bindings = bindgen::Builder::default()
            .header("src/proxy/dispatch.h")
            .allowlist_var("DISPATCH_QUEUE_PRIORITY_HIGH")
            .allowlist_function("dispatch_get_global_queue")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .layout_tests(false)
            .generate()
            .expect("unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("dispatch_sys.rs"))
            .expect("couldn't write dispatch bindings");
    }
}
