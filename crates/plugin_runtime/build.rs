use std::path::Path;

fn main() {
    let base = Path::new("../../plugins");

    // println!("cargo:rerun-if-changed=../../plugins/*");
    println!("cargo:warning=Rebuilding plugins...");

    let _ = std::fs::remove_dir_all(base.join("bin"));
    let _ =
        std::fs::create_dir_all(base.join("bin")).expect("Could not make plugins bin directory");

    let build_successful = std::process::Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-wasi",
            "--manifest-path",
            base.join("Cargo.toml").to_str().unwrap(),
        ])
        .status()
        .expect("Could not build plugins")
        .success();
    assert!(build_successful);

    let binaries = std::fs::read_dir(base.join("target/wasm32-wasi/release"))
        .expect("Could not find compiled plugins in target");
    println!("cargo:warning={:?}", binaries);

    for file in binaries {
        let is_wasm = || {
            let path = file.ok()?.path();
            if path.extension()? == "wasm" {
                Some(path)
            } else {
                None
            }
        };

        if let Some(path) = is_wasm() {
            std::fs::copy(&path, base.join("bin").join(path.file_name().unwrap()))
                .expect("Could not copy compiled plugin to bin");
        }
    }

    // TODO: create .wat versions
    // TODO: optimize with wasm-opt
}
