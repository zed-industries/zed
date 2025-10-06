#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use std::io::{self, Write};
use std::process::{Command, Output, Stdio};

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Parser)]
pub struct PublishGpuiArgs {
    /// Optional metadata to append to the version (e.g., build.123). If not provided, bumps the minor version.
    #[arg(long)]
    metadata: Option<String>,

    /// Perform a dry-run and wait for user confirmation before each publish
    #[arg(long)]
    dry_run: bool,
}

pub fn run_publish_gpui(args: PublishGpuiArgs) -> Result<()> {
    println!(
        "Starting GPUI publish process{}...",
        if args.dry_run { " (with dry-run)" } else { "" }
    );

    check_workspace_root()?;
    ensure_cargo_set_version()?;
    check_git_clean()?;

    let current_version = read_gpui_version()?;
    let new_version = bump_version(&current_version, args.metadata.as_deref())?;
    println!(
        "Updating GPUI version: {} -> {}",
        current_version, new_version
    );
    publish_dependencies(&new_version, args.dry_run)?;
    publish_gpui(&new_version, args.dry_run)?;
    println!("GPUI publish complete!");
    Ok(())
}

/// Helper for running shell commands with optional verbose output

fn read_gpui_version() -> Result<String> {
    let gpui_cargo_toml_path = "crates/gpui/Cargo.toml";
    let contents = std::fs::read_to_string(gpui_cargo_toml_path)
        .context("Failed to read crates/gpui/Cargo.toml")?;

    let cargo_toml: toml::Value =
        toml::from_str(&contents).context("Failed to parse crates/gpui/Cargo.toml")?;

    let version = cargo_toml
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .context("Failed to find version in crates/gpui/Cargo.toml")?;

    Ok(version.to_string())
}

fn bump_version(current_version: &str, metadata: Option<&str>) -> Result<String> {
    let without_metadata = current_version.split('+').next().unwrap();

    let new_version = if let Some(meta) = metadata {
        format!("{}+{}", without_metadata, meta)
    } else {
        let base_version = without_metadata.split('-').next().unwrap();
        let parts: Vec<&str> = base_version.split('.').collect();
        if parts.len() != 3 {
            bail!("Invalid version format: {}", current_version);
        }

        let major: u32 = parts[0].parse().context("Failed to parse major version")?;
        let minor: u32 = parts[1].parse().context("Failed to parse minor version")?;
        format!("{}.{}.0", major, minor + 1)
    };

    Ok(new_version)
}

fn publish_dependencies(new_version: &str, dry_run: bool) -> Result<()> {
    let gpui_dependencies = vec![
        ("zed-collections", "collections"),
        ("zed-perf", "perf"),
        ("zed-util-macros", "util_macros"),
        ("zed-util", "util"),
        ("gpui-macros", "gpui_macros"),
        ("zed-http-client", "http_client"),
        ("zed-derive-refinable", "derive_refineable"),
        ("zed-refineable", "refineable"),
        ("zed-semantic-version", "semantic_version"),
        ("zed-sum-tree", "sum_tree"),
        ("zed-media", "media"),
    ];

    for (crate_name, package_name) in gpui_dependencies {
        println!(
            "Publishing dependency: {} (package: {})",
            crate_name, package_name
        );

        // Bump version in dependency's Cargo.toml
        update_crate_version(crate_name, new_version)?;

        publish_crate(crate_name, dry_run)?;

        // Update version in workspace Cargo.toml
        update_workspace_dependency_version(package_name, new_version)?;
    }

    Ok(())
}

fn publish_gpui(new_version: &str, dry_run: bool) -> Result<()> {
    update_crate_version("gpui", new_version)?;

    publish_crate("gpui", dry_run)?;

    Ok(())
}

fn update_crate_version(package_name: &str, new_version: &str) -> Result<()> {
    let output = run_command(
        Command::new("cargo")
            .arg("set-version")
            .arg("--package")
            .arg(package_name)
            .arg(new_version),
    )?;

    if !output.status.success() {
        bail!("Failed to set version for package {}", package_name);
    }

    Ok(())
}

fn publish_crate(crate_name: &str, dry_run: bool) -> Result<()> {
    let publish_crate_impl = |crate_name, dry_run| {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

        let mut command = Command::new(&cargo);
        command
            .arg("publish")
            .arg("--allow-dirty")
            .args(["-p", crate_name]);

        if dry_run {
            command.arg("--dry-run");
        }

        run_command(&mut command)?;

        anyhow::Ok(())
    };

    if dry_run {
        publish_crate_impl(crate_name, true)?;

        print!("Press Enter to publish for real (or ctrl-c to abort)...");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
    }

    publish_crate_impl(crate_name, false)?;

    Ok(())
}

fn update_workspace_dependency_version(package_name: &str, new_version: &str) -> Result<()> {
    let workspace_cargo_toml_path = "Cargo.toml";
    let contents = std::fs::read_to_string(workspace_cargo_toml_path)
        .context("Failed to read workspace Cargo.toml")?;

    let updated = update_dependency_version_in_toml(&contents, package_name, new_version)?;

    std::fs::write(workspace_cargo_toml_path, updated)
        .context("Failed to write workspace Cargo.toml")?;

    Ok(())
}

fn update_dependency_version_in_toml(
    toml_contents: &str,
    package_name: &str,
    new_version: &str,
) -> Result<String> {
    let mut doc = toml_contents
        .parse::<toml_edit::DocumentMut>()
        .context("Failed to parse TOML")?;

    // Navigate to workspace.dependencies.<package_name>
    let dependency = doc
        .get_mut("workspace")
        .and_then(|w| w.get_mut("dependencies"))
        .and_then(|d| d.get_mut(package_name))
        .context(format!(
            "Failed to find {} in workspace dependencies",
            package_name
        ))?;

    // Update the version field if it exists
    if let Some(dep_table) = dependency.as_table_like_mut() {
        if dep_table.contains_key("version") {
            dep_table.insert("version", toml_edit::value(new_version));
        } else {
            bail!(
                "No version field found for {} in workspace dependencies",
                package_name
            );
        }
    } else {
        bail!("{} is not a table in workspace dependencies", package_name);
    }

    Ok(doc.to_string())
}

fn check_workspace_root() -> Result<()> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;

    // Check if Cargo.toml exists in the current directory
    let cargo_toml_path = cwd.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        bail!(
            "Cargo.toml not found in current directory. Please run this command from the workspace root."
        );
    }

    // Check if it's a workspace by looking for [workspace] section
    let contents =
        std::fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;

    if !contents.contains("[workspace]") {
        bail!(
            "Current directory does not appear to be a workspace root. Please run this command from the workspace root."
        );
    }

    Ok(())
}

fn ensure_cargo_set_version() -> Result<()> {
    let output = run_command(
        Command::new("which")
            .arg("cargo-set-version")
            .stdout(Stdio::piped()),
    )
    .context("Failed to check for cargo-set-version")?;

    if !output.status.success() {
        println!("cargo-set-version not found. Installing cargo-edit...");

        let install_output = run_command(Command::new("cargo").arg("install").arg("cargo-edit"))?;

        if !install_output.status.success() {
            bail!("Failed to install cargo-edit");
        }
    }

    Ok(())
}

fn check_git_clean() -> Result<()> {
    let output = run_command(
        Command::new("git")
            .args(["status", "--porcelain"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
    )?;

    if !output.status.success() {
        bail!("git status command failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        bail!(
            "Working directory is not clean. Please commit or stash your changes before publishing."
        );
    }

    Ok(())
}

fn run_command(command: &mut Command) -> Result<Output> {
    let command_str = {
        let program = command.get_program().to_string_lossy();
        let args = command
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        if args.is_empty() {
            program.to_string()
        } else {
            format!("{} {}", program, args)
        }
    };
    eprintln!("+ {}", command_str);

    let output = command
        .spawn()
        .context("failed to spawn child process")?
        .wait_with_output()
        .context("failed to wait for child process")?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_update_dependency_version_in_toml() {
        let input = indoc! {r#"
            [workspace]
            resolver = "2"

            [workspace.dependencies]
            # here's a comment
            collections = { path = "crates/collections", package = "zed-collections", version = "0.1.0" }

            util = { path = "crates/util", package = "zed-util", version = "0.1.0" }
        "#};

        let result = update_dependency_version_in_toml(input, "collections", "0.2.0").unwrap();

        let output = indoc! {r#"
            [workspace]
            resolver = "2"

            [workspace.dependencies]
            # here's a comment
            collections = { path = "crates/collections", package = "zed-collections", version = "0.2.0" }

            util = { path = "crates/util", package = "zed-util", version = "0.1.0" }
        "#};

        assert_eq!(result, output);
    }

    #[test]
    fn test_bump_version() {
        // Test bumping minor version (default behavior)
        assert_eq!(bump_version("0.1.0", None).unwrap(), "0.2.0");
        assert_eq!(bump_version("0.1.5", None).unwrap(), "0.2.0");
        assert_eq!(bump_version("1.42.7", None).unwrap(), "1.43.0");

        // Test stripping pre-release and bumping minor
        assert_eq!(bump_version("0.1.0-alpha.1", None).unwrap(), "0.2.0");
        assert_eq!(bump_version("0.1.0-beta", None).unwrap(), "0.2.0");

        // Test stripping existing metadata and bumping
        assert_eq!(bump_version("0.1.0+old.metadata", None).unwrap(), "0.2.0");

        // Test adding metadata (keeps current version)
        assert_eq!(
            bump_version("0.1.0", Some("build.123")).unwrap(),
            "0.1.0+build.123"
        );

        // Test adding metadata retains pre-release
        assert_eq!(
            bump_version("0.1.0-alpha", Some("build.456")).unwrap(),
            "0.1.0-alpha+build.456"
        );

        // Test adding metadata retains pre-release with complex identifier
        assert_eq!(
            bump_version("0.1.0-beta.2", Some("build.789")).unwrap(),
            "0.1.0-beta.2+build.789"
        );

        // Test replacing existing metadata
        assert_eq!(
            bump_version("0.1.0+old.metadata", Some("new.metadata")).unwrap(),
            "0.1.0+new.metadata"
        );

        // Test replacing metadata while retaining pre-release
        assert_eq!(
            bump_version("0.1.0-alpha+old.metadata", Some("new.metadata")).unwrap(),
            "0.1.0-alpha+new.metadata"
        );
    }
}
