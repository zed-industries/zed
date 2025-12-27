use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    emit_wasmtime_version();
    copy_extension_api_rust_files()
}

/// Emit the wasmtime crate version as a compile-time environment variable.
/// This allows us to use the version for cache invalidation without hardcoding.
fn emit_wasmtime_version() {
    // Try to read version from Cargo.lock
    let cargo_lock_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("Cargo.lock"));

    if let Some(lock_path) = cargo_lock_path {
        println!("cargo:rerun-if-changed={}", lock_path.display());
        if let Ok(content) = fs::read_to_string(&lock_path) {
            // Parse Cargo.lock to find wasmtime version
            // Format: [[package]]\nname = "wasmtime"\nversion = "X.Y.Z"
            let mut in_wasmtime = false;
            for line in content.lines() {
                if line.starts_with("name = \"wasmtime\"") {
                    in_wasmtime = true;
                } else if in_wasmtime && line.starts_with("version = ") {
                    if let Some(version) = line
                        .strip_prefix("version = \"")
                        .and_then(|s| s.strip_suffix('"'))
                    {
                        println!("cargo:rustc-env=WASMTIME_VERSION={}", version);
                        return;
                    }
                } else if line.starts_with("[[package]]") && in_wasmtime {
                    break;
                }
            }
        }
    }

    // Fallback if we can't parse Cargo.lock
    println!("cargo:rustc-env=WASMTIME_VERSION=unknown");
}

/// rust-analyzer doesn't support include! for files from outside the crate.
/// Copy them to the OUT_DIR, so we can include them from there, which is supported.
fn copy_extension_api_rust_files() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let input_dir = PathBuf::from("../extension_api/wit");
    let output_dir = PathBuf::from(out_dir);

    println!("cargo:rerun-if-changed={}", input_dir.display());

    for entry in fs::read_dir(&input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("cargo:rerun-if-changed={}", path.display());

            for subentry in fs::read_dir(&path)? {
                let subentry = subentry?;
                let subpath = subentry.path();
                if subpath.extension() == Some(std::ffi::OsStr::new("rs")) {
                    let relative_path = subpath.strip_prefix(&input_dir)?;
                    let destination = output_dir.join(relative_path);

                    fs::create_dir_all(destination.parent().unwrap())?;
                    fs::copy(&subpath, &destination)?;
                }
            }
        } else if path.extension() == Some(std::ffi::OsStr::new("rs")) {
            let relative_path = path.strip_prefix(&input_dir)?;
            let destination = output_dir.join(relative_path);

            fs::create_dir_all(destination.parent().unwrap())?;
            fs::copy(&path, &destination)?;
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    Ok(())
}
