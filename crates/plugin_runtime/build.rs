use std::{io::Write, path::Path};
use wasmtime::{Config, Engine};

fn main() {
    let base = Path::new("../../plugins");

    // println!("cargo:rerun-if-changed=../../plugins/*");
    println!("cargo:warning=Precompiling plugins...");

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

    let engine = create_engine();

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
            let out_path = base.join("bin").join(path.file_name().unwrap());
            std::fs::copy(&path, &out_path).expect("Could not copy compiled plugin to bin");
            precompile(&out_path, &engine);
        }
    }
}

fn create_engine() -> Engine {
    let mut config = Config::default();
    config.async_support(true);
    // config.epoch_interruption(true);
    Engine::new(&config).expect("Could not create engine")
}

fn precompile(path: &Path, engine: &Engine) {
    let bytes = std::fs::read(path).expect("Could not read wasm module");
    let compiled = engine
        .precompile_module(&bytes)
        .expect("Could not precompile module");
    let out_path = path.parent().unwrap().join(&format!(
        "{}.pre",
        path.file_name().unwrap().to_string_lossy()
    ));
    let mut out_file = std::fs::File::create(out_path)
        .expect("Could not create output file for precompiled module");
    out_file.write_all(&compiled).unwrap();
}
