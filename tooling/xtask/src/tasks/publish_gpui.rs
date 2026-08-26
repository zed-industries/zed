#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]
use std::io::{self, Write};
use std::process::{Command, Output, Stdio};

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Parser)]
pub struct PublishGpuiArgs {
    /// Perform a dry-run and wait for user confirmation before each publish
    #[arg(long)]
    dry_run: bool,

    /// Skip to a specific package (by package name or crate name) and start from there
    #[arg(long)]
    skip_to: Option<String>,
}

pub fn run_publish_gpui(args: PublishGpuiArgs) -> Result<()> {
    println!(
        "Starting GPUI publish process{}...",
        if args.dry_run { " (with dry-run)" } else { "" }
    );

    let start_time = std::time::Instant::now();
    check_workspace_root()?;

    if args.skip_to.is_none() {
        check_git_clean()?;
    } else {
        println!("Skipping git clean check due to --skip-to flag");
    }

    let version = read_gpui_version()?;
    println!("Updating GPUI to version: {}", version);
    publish_dependencies(&version, args.dry_run, args.skip_to.as_deref())?;
    publish_gpui(&version, args.dry_run)?;
    println!("GPUI published in {}s", start_time.elapsed().as_secs_f32());
    Ok(())
}

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

fn publish_dependencies(new_version: &str, dry_run: bool, skip_to: Option<&str>) -> Result<()> {
    let gpui_dependencies = vec![
        ("collections", "gpui_collections", "crates"),
        ("perf", "gpui_perf", "tooling"),
        ("util_macros", "gpui_util_macros", "crates"),
        ("util", "gpui_util", "crates"),
        ("gpui_macros", "gpui-macros", "crates"),
        ("http_client", "gpui_http_client", "crates"),
        (
            "derive_refineable",
            "gpui_derive_refineable",
            "crates/refineable",
        ),
        ("refineable", "gpui_refineable", "crates"),
        ("semantic_version", "gpui_semantic_version", "crates"),
        ("sum_tree", "gpui_sum_tree", "crates"),
        ("media", "gpui_media", "crates"),
    ];

    let mut should_skip = skip_to.is_some();
    let skip_target = skip_to.unwrap_or("");

    for (package_name, crate_name, package_dir) in gpui_dependencies {
        if should_skip {
            if package_name == skip_target || crate_name == skip_target {
                println!("Found skip target: {} ({})", crate_name, package_name);
                should_skip = false;
            } else {
                println!("Skipping: {} ({})", crate_name, package_name);
                continue;
            }
        }

        println!(
            "Publishing dependency: {} (package: {})",
            crate_name, package_name
        );

        update_crate_cargo_toml(package_name, crate_name, package_dir, new_version)?;
        update_workspace_dependency_version(package_name, crate_name, new_version)?;
        publish_crate(crate_name, dry_run)?;
    }

    if should_skip {
        bail!(
            "Could not find package or crate named '{}' to skip to",
            skip_target
        );
    }

    Ok(())
}

fn publish_gpui(new_version: &str, dry_run: bool) -> Result<()> {
    update_crate_cargo_toml("gpui", "gpui", "crates", new_version)?;

    publish_crate("gpui", dry_run)?;

    Ok(())
}

fn update_crate_cargo_toml(
    package_name: &str,
    crate_name: &str,
    package_dir: &str,
    new_version: &str,
) -> Result<()> {
    let cargo_toml_path = format!("{}/{}/Cargo.toml", package_dir, package_name);
    let contents = std::fs::read_to_string(&cargo_toml_path)
        .context(format!("Failed to read {}", cargo_toml_path))?;

    let updated = update_crate_package_fields(&contents, crate_name, new_version)?;

    std::fs::write(&cargo_toml_path, updated)
        .context(format!("Failed to write {}", cargo_toml_path))?;

    Ok(())
}

fn update_crate_package_fields(
    toml_contents: &str,
    crate_name: &str,
    new_version: &str,
) -> Result<String> {
    let mut doc = toml_contents
        .parse::<toml_edit::DocumentMut>()
        .context("Failed to parse TOML")?;

    let package = doc
        .get_mut("package")
        .and_then(|p| p.as_table_like_mut())
        .context("Failed to find [package] section")?;

    package.insert("name", toml_edit::value(crate_name));
    package.insert("version", toml_edit::value(new_version));
    package.insert("publish", toml_edit::value(true));

    Ok(doc.to_string())
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

fn update_workspace_dependency_version(
    package_name: &str,
    crate_name: &str,
    new_version: &str,
) -> Result<()> {
    let workspace_cargo_toml_path = "Cargo.toml";
    let contents = std::fs::read_to_string(workspace_cargo_toml_path)
        .context("Failed to read workspace Cargo.toml")?;

    let mut doc = contents
        .parse::<toml_edit::DocumentMut>()
        .context("Failed to parse TOML")?;

    update_dependency_version_in_doc(&mut doc, package_name, crate_name, new_version)?;
    update_profile_override_in_doc(&mut doc, package_name, crate_name)?;

    std::fs::write(workspace_cargo_toml_path, doc.to_string())
        .context("Failed to write workspace Cargo.toml")?;

    Ok(())
}

fn update_dependency_version_in_doc(
    doc: &mut toml_edit::DocumentMut,
    package_name: &str,
    crate_name: &str,
    new_version: &str,
) -> Result<()> {
    let dependency = doc
        .get_mut("workspace")
        .and_then(|w| w.get_mut("dependencies"))
        .and_then(|d| d.get_mut(package_name))
        .context(format!(
            "Failed to find {} in workspace dependencies",
            package_name
        ))?;

    if let Some(dep_table) = dependency.as_table_like_mut() {
        dep_table.insert("version", toml_edit::value(new_version));
        dep_table.insert("package", toml_edit::value(crate_name));
    } else {
        bail!("{} is not a table in workspace dependencies", package_name);
    }

    Ok(())
}

fn update_profile_override_in_doc(
    doc: &mut toml_edit::DocumentMut,
    package_name: &str,
    crate_name: &str,
) -> Result<()> {
    if let Some(profile_dev_package) = doc
        .get_mut("profile")
        .and_then(|p| p.get_mut("dev"))
        .and_then(|d| d.get_mut("package"))
        .and_then(|p| p.as_table_like_mut())
    {
        if let Some(old_entry) = profile_dev_package.get(package_name) {
            let old_entry_clone = old_entry.clone();
            profile_dev_package.remove(package_name);
            profile_dev_package.insert(crate_name, old_entry_clone);
        }
    }

    Ok(())
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

    if !output.status.success() {
        bail!("Command failed with status {}", output.status);
    }

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
            collections = { path = "crates/collections" }

            util = { path = "crates/util", package = "zed-util", version = "0.1.0" }
        "#};

        let mut doc = input.parse::<toml_edit::DocumentMut>().unwrap();

        update_dependency_version_in_doc(&mut doc, "collections", "gpui_collections", "0.2.0")
            .unwrap();

        let result = doc.to_string();

        let output = indoc! {r#"
            [workspace]
            resolver = "2"

            [workspace.dependencies]
            # here's a comment
            collections = { path = "crates/collections" , version = "0.2.0", package = "gpui_collections" }

            util = { path = "crates/util", package = "zed-util", version = "0.1.0" }
        "#};

        assert_eq!(result, output);
    }

    #[test]
    fn test_update_crate_package_fields() {
        let input = indoc! {r#"
            [package]
            name = "collections"
            version = "0.1.0"
            edition = "2021"
            publish = false
            # some comment about the license
            license = "GPL-3.0-or-later"

            [dependencies]
            serde = "1.0"
        "#};

        let result = update_crate_package_fields(input, "gpui_collections", "0.2.0").unwrap();

        let output = indoc! {r#"
            [package]
            name = "gpui_collections"
            version = "0.2.0"
            edition = "2021"
            publish = true
            # some comment about the license
            license = "GPL-3.0-or-later"

            [dependencies]
            serde = "1.0"
        "#};

        assert_eq!(result, output);
    }

    #[test]
    fn test_update_profile_override_in_toml() {
        let input = indoc! {r#"
            [profile.dev]
            split-debuginfo = "unpacked"

            [profile.dev.package]
            taffy = { opt-level = 3 }
            collections = { codegen-units = 256 }
            refineable = { codegen-units = 256 }
            util = { codegen-units = 256 }
        "#};

        let mut doc = input.parse::<toml_edit::DocumentMut>().unwrap();

        update_profile_override_in_doc(&mut doc, "collections", "gpui_collections").unwrap();

        let result = doc.to_string();

        let output = indoc! {r#"
            [profile.dev]
            split-debuginfo = "unpacked"

            [profile.dev.package]
            taffy = { opt-level = 3 }
            refineable = { codegen-units = 256 }
            util = { codegen-units = 256 }
            gpui_collections = { codegen-units = 256 }
        "#};

        assert_eq!(result, output);
    }
}
