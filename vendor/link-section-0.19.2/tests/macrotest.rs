#![cfg(not(miri))]

/*

To overwrite the Linux expansion tests on macOS, run:

docker run --rm -v "$(pwd):/src" -w /src rust:1.88 \
  bash -lc 'export CARGO_TARGET_DIR=/src/target/target-docker && export PATH="/usr/local/cargo/bin:$PATH" && cargo install cargo-expand && MACROTEST=overwrite cargo test -p link-section --test macrotest'
*/

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use tempfile::tempdir;

/// Macrotest sometimes leaves files empty when things fail to compile.
fn ensure_no_empty_files(path: impl AsRef<Path>) {
    if ensure_no_empty_files_recurse(path) {
        panic!("Empty files found");
    }
}

fn ensure_no_empty_files_recurse(path: impl AsRef<Path>) -> bool {
    if !std::fs::exists(path.as_ref()).unwrap() {
        return false;
    }
    let mut empty_files = false;
    let files = std::fs::read_dir(path).unwrap();
    for file in files.flatten() {
        if file.file_type().unwrap().is_dir() {
            ensure_no_empty_files_recurse(file.path());
        } else {
            let content = std::fs::read_to_string(file.path()).unwrap();
            if content.trim().is_empty() {
                eprintln!("Empty file found: {}", file.path().display());
                empty_files = true;
            }
            if content.contains("/*ERROR*/") {
                panic!("Error found in file: {}", file.path().display());
            }
        }
    }
    empty_files
}

#[test]
#[cfg(not(linktime_used_linker))]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
    ensure_no_empty_files("tests/expand");
}

#[cfg(target_vendor = "apple")]
#[cfg(not(linktime_used_linker))]
#[test]
pub fn pass_darwin() {
    macrotest::expand("tests/expand-darwin/*.rs");
    ensure_no_empty_files("tests/expand-darwin");
}

#[cfg(target_os = "linux")]
#[cfg(not(linktime_used_linker))]
#[test]
pub fn pass_linux() {
    macrotest::expand("tests/expand-linux/*.rs");
    ensure_no_empty_files("tests/expand-linux");
}

#[test]
pub fn trybuild() {
    let t = trybuild::TestCases::new();
    // TODO: whitespace issue (tabs?) in error tests
    #[cfg(not(target_os = "openbsd"))]
    t.compile_fail("tests/errors/*.rs");
    t.pass("tests/pass/*.rs");
}

#[test]
pub fn target_test() {
    let Some(toolchain) = std::env::var_os("TOOLCHAIN") else {
        return;
    };
    if toolchain != "nightly" {
        return;
    }
    let cases_dir = Path::new("tests/target-test");
    let overwrite = std::env::var_os("MACROTEST")
        .map(|v| v == "overwrite")
        .unwrap_or(false);

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let mut count = 0;
    let mut success = 0;

    let target_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("target")
        .join("target-test");

    // Each testcase is:
    // - input:  tests/target-test/<case>.rs
    // - outputs: tests/target-test/<case>/<target-triple>.rs
    for entry in fs::read_dir(cases_dir).unwrap().flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let case_name = path.file_stem().unwrap().to_string_lossy().to_string();
        let outputs_dir = cases_dir.join(&case_name);
        if !outputs_dir.is_dir() {
            panic!(
                "expected output directory {} for testcase {}",
                outputs_dir.display(),
                case_name
            );
        }

        // Allow a testcase-specific Cargo.toml at tests/target-test/<case>/Cargo.toml
        // (useful for additional deps or feature flags).
        let case_cargo_toml_path = outputs_dir.join("Cargo.toml");
        let base_cargo_toml = if case_cargo_toml_path.exists() {
            fs::read_to_string(&case_cargo_toml_path).unwrap()
        } else {
            format!(
                r#"[package]
name = "expand_probe"
version = "0.0.0"
edition = "2021"

[dependencies]
link-section = {{ path = "{repo_root}/link-section", default-features = false }}
"#,
                repo_root = repo_root.display()
            )
        };

        for out_file in fs::read_dir(&outputs_dir).unwrap().flatten() {
            let out_path = out_file.path();
            if out_path.extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }

            count += 1;

            let target = out_path.file_stem().unwrap().to_string_lossy().to_string();
            eprintln!("testing testcase={case_name}, target={target}");

            let dir = tempdir().unwrap();
            fs::create_dir_all(dir.path().join("src")).unwrap();

            fs::write(dir.path().join("Cargo.toml"), &base_cargo_toml).unwrap();
            // Ensure the probe crate doesn't require `std` for exotic targets.
            let input_src = fs::read_to_string(&path).unwrap();
            fs::write(
                dir.path().join("src/lib.rs"),
                format!("#![no_std]\n// ***START MARKER***\n{input_src}"),
            )
            .unwrap();

            let out = Command::new("cargo")
                .current_dir(dir.path())
                .env("CARGO_TERM_COLOR", "never")
                .env("CARGO_TARGET_DIR", target_dir.join(&target))
                .env_remove("RUSTFLAGS")
                .args([
                    "+nightly",
                    "rustc",
                    // Build core from source so we can target triples without prebuilt std.
                    "-Zbuild-std=core,alloc",
                    "-Zbuild-std-features=compiler-builtins-mem",
                    "--target",
                    &target,
                    "--lib",
                    "--quiet",
                    "--",
                    "-Zunpretty=expanded",
                ])
                .output()
                .unwrap();

            // Even when compilation fails later, -Zunpretty=expanded often emits output.
            // For debugging, include stderr on mismatch.
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            eprintln!("{stderr}");

            if stderr.contains("error") {
                panic!("compilation failed for testcase={case_name}, target={target}\n\n--- stderr ---\n{stderr}\n");
            }

            if stdout.trim().is_empty() {
                panic!(
                    "no expansion output for testcase={case_name}, target={target}\n\n--- stderr ---\n{stderr}\n"
                );
            }

            if !stdout.contains("// ***START MARKER***") {
                panic!(
                    "no start marker in expansion output for testcase={case_name}, target={target}\n\n--- stdout ---\n{stdout}\n"
                );
            }

            let stdout = stdout
                .split("// ***START MARKER***\n")
                .nth(1)
                .unwrap()
                .to_string();

            if overwrite {
                fs::write(&out_path, &stdout).unwrap();
                success += 1;
            } else {
                let expected = fs::read_to_string(&out_path).unwrap();
                if expected != stdout {
                    eprintln!(
                        "target-test mismatch for testcase={case_name}, target={target}\n\
                         (set MACROTEST=overwrite to update)\n\n\
                         --- expected file ---\n{expected}\n\n\
                         --- actual stdout ---\n{stdout}\n\n\
                         --- stderr ---\n{stderr}\n"
                    );
                } else {
                    success += 1;
                }
            }
        }
    }

    if count == 0 {
        panic!("no testcases found");
    } else if count == success {
        println!("all {} testcases passed", count);
    } else {
        panic!("{} of {} testcases passed", success, count);
    }

    ensure_no_empty_files("tests/target-test");
}
