use std::path::PathBuf;

#[allow(dead_code)]
const RUST_TOOLCHAIN_TOML: &str = r#"[toolchain]
channel = "nightly-2026-08-06"
components = ["rust-src"]
"#;

#[allow(dead_code)]
const GITIGNORE: &str = "/build/\n/rust/target/\n";

#[allow(dead_code)]
const CLAUDE_MD: &str = r#"# Citadel project — Rust/C boundary rule

This project follows Citadel's architecture rule:

- `cpp/` may only perform direct, linear I/O hand-off: reading a pin, writing a pin, sending a byte, declaring `pinMode`/board constants. No `if`, no `for`/`while`, no ternaries, no computed intermediate variables.
- All logic — state transitions, calculations, control decisions — must live in `rust/` (a `#![no_std]` crate), never in `cpp/`.
- The two sides only exchange plain data across `extern "C"`: `cpp/` calls into `extern "C"` Rust functions, and Rust may read `extern "C"` variables/constants defined in `cpp/`.

If asked to add a decision or calculation to a file in `cpp/`, implement it in `rust/src/lib.rs` instead and expose it via an `extern "C"` function.
"#;

#[allow(dead_code)]
const DOCS_README: &str = r#"# docs

Schematics, pin assignment notes, and other project documentation go here.
"#;

#[allow(dead_code)]
const RUST_LIB_RS: &str = r#"#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Add your logic here, exposed via `extern "C"` for cpp/io.cpp to call.
"#;

#[allow(dead_code)]
const CPP_IO_CPP: &str = r#"#include <Arduino.h>

void setup() {
    // pinMode(...) calls go here
}

void loop() {
    // straight-line I/O only — put decisions and calculations in rust/src/lib.rs
}
"#;

/// Sanitizes an arbitrary project directory name into a valid Cargo package
/// name fragment: lowercase, non-alphanumeric runs collapsed to a single
/// `_`, leading/trailing `_` trimmed. Falls back to `project` if the result
/// would be empty (e.g. the input has no alphanumeric characters at all).
pub fn sanitize_crate_name(project_name: &str) -> String {
    let mut result = String::new();
    let mut last_was_separator = false;
    for ch in project_name.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator {
            result.push('_');
            last_was_separator = true;
        }
    }
    let trimmed = result.trim_matches('_');
    if trimmed.is_empty() {
        "project".to_string()
    } else {
        trimmed.to_string()
    }
}

fn rust_cargo_toml(project_name: &str) -> String {
    let crate_name = sanitize_crate_name(project_name);
    format!(
        r#"[workspace]

[package]
name = "{crate_name}_logic"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
crate-type = ["staticlib"]

[profile.release]
panic = "abort"
opt-level = "s"
lto = true
"#
    )
}

/// Returns the full Citadel project scaffold as (relative path, file
/// contents) pairs. `project_name` is typically the selected directory's
/// name; it is only used to derive `rust/Cargo.toml`'s package name (via
/// [`sanitize_crate_name`]) — it does not affect any other file's content
/// or path.
pub fn scaffold_files(project_name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (PathBuf::from(".gitignore"), GITIGNORE.to_string()),
        (PathBuf::from(".claude/CLAUDE.md"), CLAUDE_MD.to_string()),
        (PathBuf::from(".claude/skills/.gitkeep"), String::new()),
        (PathBuf::from("docs/README.md"), DOCS_README.to_string()),
        (
            PathBuf::from("rust-toolchain.toml"),
            RUST_TOOLCHAIN_TOML.to_string(),
        ),
        (
            PathBuf::from("rust/Cargo.toml"),
            rust_cargo_toml(project_name),
        ),
        (PathBuf::from("rust/src/lib.rs"), RUST_LIB_RS.to_string()),
        (PathBuf::from("cpp/io.cpp"), CPP_IO_CPP.to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_files_includes_all_expected_paths() {
        let files = scaffold_files("my-project");
        let paths: Vec<&PathBuf> = files.iter().map(|(p, _)| p).collect();
        assert_eq!(paths.len(), 8);
        assert!(paths.contains(&&PathBuf::from(".gitignore")));
        assert!(paths.contains(&&PathBuf::from(".claude/CLAUDE.md")));
        assert!(paths.contains(&&PathBuf::from(".claude/skills/.gitkeep")));
        assert!(paths.contains(&&PathBuf::from("docs/README.md")));
        assert!(paths.contains(&&PathBuf::from("rust-toolchain.toml")));
        assert!(paths.contains(&&PathBuf::from("rust/Cargo.toml")));
        assert!(paths.contains(&&PathBuf::from("rust/src/lib.rs")));
        assert!(paths.contains(&&PathBuf::from("cpp/io.cpp")));
    }

    #[test]
    fn claude_md_states_boundary_rule() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from(".claude/CLAUDE.md"))
            .unwrap();
        assert!(content.contains("extern \"C\""));
        assert!(content.contains("cpp/"));
        assert!(content.contains("rust/"));
    }

    #[test]
    fn io_cpp_has_setup_and_loop_and_no_control_flow() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from("cpp/io.cpp"))
            .unwrap();
        assert!(content.contains("void setup()"));
        assert!(content.contains("void loop()"));
        assert!(!content.contains("if ("));
        assert!(!content.contains("for ("));
        assert!(!content.contains("while ("));
    }

    #[test]
    fn rust_lib_rs_is_no_std_with_panic_handler() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from("rust/src/lib.rs"))
            .unwrap();
        assert!(content.contains("#![no_std]"));
        assert!(content.contains("#[panic_handler]"));
    }

    #[test]
    fn rust_cargo_toml_uses_sanitized_name() {
        let files = scaffold_files("My Project!");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from("rust/Cargo.toml"))
            .unwrap();
        assert!(content.contains("name = \"my_project_logic\""));
        assert!(content.contains("crate-type = [\"staticlib\"]"));
    }

    #[test]
    fn gitkeep_is_empty() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from(".claude/skills/.gitkeep"))
            .unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn sanitize_crate_name_lowercases_and_collapses_separators() {
        assert_eq!(sanitize_crate_name("My Project!"), "my_project");
    }

    #[test]
    fn sanitize_crate_name_keeps_digits() {
        assert_eq!(sanitize_crate_name("123"), "123");
    }

    #[test]
    fn sanitize_crate_name_trims_leading_and_trailing_separators() {
        assert_eq!(sanitize_crate_name("__hello__"), "hello");
    }

    #[test]
    fn sanitize_crate_name_falls_back_when_nothing_alphanumeric() {
        assert_eq!(sanitize_crate_name("!!!"), "project");
        assert_eq!(sanitize_crate_name(""), "project");
    }
}

