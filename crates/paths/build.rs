#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

use std::{env, fs, path::Path, process};

fn main() {
    if let Err(error) = generate_app_name() {
        eprintln!("paths build.rs: {error}");
        process::exit(1);
    }
}

fn generate_app_name() -> Result<(), String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|error| format!("failed to read CARGO_MANIFEST_DIR: {error}"))?;
    let app_name_path = Path::new(&manifest_dir).join("APP_NAME");
    println!("cargo:rerun-if-changed={}", app_name_path.display());

    let contents = fs::read_to_string(&app_name_path).map_err(|error| {
        format!(
            "failed to read app name from {}: {error}",
            app_name_path.display()
        )
    })?;
    let app_name = validate_app_name(&contents)
        .map_err(|error| format!("invalid app name in {}: {error}", app_name_path.display()))?;

    let out_dir =
        env::var("OUT_DIR").map_err(|error| format!("failed to read OUT_DIR: {error}"))?;
    let generated_path = Path::new(&out_dir).join("app_name.rs");
    fs::write(
        &generated_path,
        format!(
            "/// The application name, used to derive platform-specific data, config, cache, and state directory paths.\n\
             ///\n\
             /// Generated from the workspace `APP_NAME` file by `crates/paths/build.rs`.\n\
             pub const APP_NAME: &str = {app_name:?};\n"
        ),
    )
    .map_err(|error| {
        format!(
            "failed to write generated app name to {}: {error}",
            generated_path.display()
        )
    })?;

    Ok(())
}

fn validate_app_name(contents: &str) -> Result<&str, &'static str> {
    let app_name = contents
        .strip_suffix('\n')
        .map(|contents| contents.strip_suffix('\r').unwrap_or(contents))
        .unwrap_or(contents);

    if app_name.trim().is_empty() {
        return Err("APP_NAME must contain a non-empty app name");
    }

    if app_name != app_name.trim() {
        return Err("APP_NAME must not contain leading or trailing whitespace");
    }

    if app_name.contains('\n') || app_name.contains('\r') {
        return Err(
            "APP_NAME must contain exactly one non-empty line, with at most one trailing newline",
        );
    }

    if app_name.contains('/') || app_name.contains('\\') {
        return Err("APP_NAME must not contain '/' or '\\' path separators");
    }

    if app_name.chars().any(char::is_control) {
        return Err("APP_NAME must not contain control characters");
    }

    Ok(app_name)
}

#[cfg(test)]
mod tests {
    use super::validate_app_name;

    #[test]
    fn accepts_single_line_with_optional_trailing_newline() {
        assert_eq!(validate_app_name("Zed"), Ok("Zed"));
        assert_eq!(validate_app_name("Zed\n"), Ok("Zed"));
        assert_eq!(validate_app_name("Zed\r\n"), Ok("Zed"));
    }

    #[test]
    fn rejects_invalid_names() {
        assert!(validate_app_name("").is_err());
        assert!(validate_app_name("\n").is_err());
        assert!(validate_app_name("Zed/Fork").is_err());
        assert!(validate_app_name("Zed\\Fork").is_err());
        assert!(validate_app_name(" Zed").is_err());
        assert!(validate_app_name("Zed ").is_err());
        assert!(validate_app_name("Zed\u{7}").is_err());
        assert!(validate_app_name("Zed\nFork").is_err());
        assert!(validate_app_name("Zed\n\n").is_err());
    }
}
