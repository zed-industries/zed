use std::{io::Write, path::Path};
use wasmtime::{Config, Engine};

fn main() {
    let base = Path::new("../../plugins");

    // Find all files and folders that don't change when rebuilt
    let crates = std::fs::read_dir(base).expect("Could not find plugin directory");
    for dir in crates {
        let path = dir.unwrap().path();
        let name = path.file_name().and_then(|x| x.to_str());
        let is_dir = path.is_dir();
        if is_dir && name != Some("target") && name != Some("bin") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Clear out and recreate the plugin bin directory
    let _ = std::fs::remove_dir_all(base.join("bin"));
    let _ =
        std::fs::create_dir_all(base.join("bin")).expect("Could not make plugins bin directory");

    // Compile the plugins using the same profile as the current Zed build
    let (profile_flags, profile_target) = match std::env::var("PROFILE").unwrap().as_str() {
        "debug" => (&[][..], "debug"),
        "release" => (&["--release"][..], "release"),
        unknown => panic!("unknown profile `{}`", unknown),
    };

    // Get the target architecture for pre-cross-compilation of plugins
    // and write it to disk to be used when embedding plugins
    let target_triple = std::env::var("TARGET").unwrap().to_string();
    println!("cargo:rerun-if-env-changed=TARGET");

    // Invoke cargo to build the plugins
    let build_successful = std::process::Command::new("cargo")
        .args([
            "build",
            "--target",
            "wasm32-wasi",
            "--manifest-path",
            base.join("Cargo.toml").to_str().unwrap(),
        ])
        .args(profile_flags)
        .status()
        .expect("Could not build plugins")
        .success();
    assert!(build_successful);

    // Find all compiled binaries
    let engine = create_default_engine(&target_triple);
    let binaries = std::fs::read_dir(base.join("target/wasm32-wasi").join(profile_target))
        .expect("Could not find compiled plugins in target");

    // Copy and precompile all compiled plugins we can find
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
            precompile(&out_path, &engine, &target_triple);
        }
    }
}

/// Creates an engine with the default configuration.
/// N.B. This must create an engine with the same config as the one
/// in `plugin_runtime/src/plugin.rs`.
fn create_default_engine(target_triple: &str) -> Engine {
    let mut config = Config::default();
    config
        .target(target_triple)
        .expect(&format!("Could not set target to `{}`", target_triple));
    config.async_support(true);
    config.consume_fuel(true);
    Engine::new(&config).expect("Could not create precompilation engine")
}

fn precompile(path: &Path, engine: &Engine, target_triple: &str) {
    let bytes = std::fs::read(path).expect("Could not read wasm module");
    let compiled = engine
        .precompile_module(&bytes)
        .expect("Could not precompile module");
    let out_path = path.parent().unwrap().join(&format!(
        "{}.{}",
        path.file_name().unwrap().to_string_lossy(),
        target_triple,
    ));
    let mut out_file = std::fs::File::create(out_path)
        .expect("Could not create output file for precompiled module");
    out_file.write_all(&compiled).unwrap();
}
