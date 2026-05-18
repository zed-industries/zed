#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use cargo_toml::Manifest;
use clap::Parser;
use regex::Regex;
use toml_edit::{DocumentMut, Item, Table, value};

use crate::workspace::load_workspace;

const GITIGNORE_ENTRY: &str = ".webrtc-sys/";
const LOCAL_DIR_NAME: &str = ".webrtc-sys";
const ENV_VAR: &str = "LK_CUSTOM_WEBRTC";

#[derive(Parser)]
pub struct SetupWebrtcArgs {
    /// Re-download even if the target directory already exists.
    #[arg(long)]
    force: bool,

    /// Override the host triple component used for the release artifact
    /// (e.g. `mac-arm64-release`). Defaults to the current host.
    #[arg(long)]
    triple: Option<String>,

    /// Skip writing to `~/.cargo/config.toml`. Useful when you only want the
    /// archive on disk and intend to set `LK_CUSTOM_WEBRTC` yourself.
    #[arg(long)]
    no_cargo_config: bool,
}

pub fn run_setup_webrtc(args: SetupWebrtcArgs) -> Result<()> {
    let metadata = load_workspace()?;
    let workspace_root = metadata.workspace_root.as_std_path().to_path_buf();

    let rev = read_webrtc_sys_rev(&workspace_root)?;
    eprintln!("Pinned livekit-rust-sdks rev: {rev}");

    let tag = fetch_webrtc_tag(&rev)?;
    eprintln!("WEBRTC_TAG for that rev: {tag}");

    let triple = match args.triple {
        Some(triple) => triple,
        None => host_webrtc_triple()?,
    };
    eprintln!("Target triple: {triple}");

    let local_root = workspace_root.join(LOCAL_DIR_NAME);
    let tag_dir = local_root.join(&tag);
    let extracted_dir = tag_dir.join(&triple);

    if extracted_dir.exists() && !args.force {
        eprintln!(
            "Already present at {}, skipping download.",
            extracted_dir.display()
        );
    } else {
        if extracted_dir.exists() {
            fs::remove_dir_all(&extracted_dir)
                .with_context(|| format!("removing stale {}", extracted_dir.display()))?;
        }
        fs::create_dir_all(&tag_dir).with_context(|| format!("creating {}", tag_dir.display()))?;
        download_and_extract(&tag, &triple, &tag_dir)?;
    }

    let absolute = extracted_dir
        .canonicalize()
        .with_context(|| format!("canonicalizing {}", extracted_dir.display()))?;

    ensure_gitignore_entry(&workspace_root)?;

    if args.no_cargo_config {
        eprintln!(
            "Skipping ~/.cargo/config.toml update. Set {ENV_VAR}={} yourself.",
            absolute.display()
        );
    } else {
        update_cargo_config(&absolute)?;
    }

    eprintln!();
    eprintln!("Done. {ENV_VAR} -> {}", absolute.display());
    Ok(())
}

fn read_webrtc_sys_rev(workspace_root: &Path) -> Result<String> {
    let manifest_path = workspace_root.join("Cargo.toml");
    let manifest = Manifest::from_path(&manifest_path)
        .with_context(|| format!("parsing {}", manifest_path.display()))?;

    let patch = manifest
        .patch
        .get("crates-io")
        .context("workspace Cargo.toml has no [patch.crates-io] section")?;
    let dep = patch
        .get("webrtc-sys")
        .context("[patch.crates-io] is missing webrtc-sys")?;
    let detail = dep
        .detail()
        .context("webrtc-sys patch entry is not a table")?;
    detail
        .git
        .as_ref()
        .context("webrtc-sys patch is missing a git source")?;
    detail
        .rev
        .clone()
        .context("webrtc-sys patch is missing a `rev`")
}

fn fetch_webrtc_tag(rev: &str) -> Result<String> {
    let url = format!(
        "https://raw.githubusercontent.com/zed-industries/livekit-rust-sdks/{rev}/webrtc-sys/build/src/lib.rs"
    );
    let body = curl_text(&url).with_context(|| format!("fetching {url}"))?;

    let re =
        Regex::new(r#"pub\s+const\s+WEBRTC_TAG\s*:\s*&str\s*=\s*"([^"]+)""#).expect("static regex");
    let captures = re
        .captures(&body)
        .with_context(|| format!("could not find WEBRTC_TAG in {url}"))?;
    Ok(captures[1].to_string())
}

fn host_webrtc_triple() -> Result<String> {
    let os = match std::env::consts::OS {
        "macos" => "mac",
        "linux" => "linux",
        "windows" => "win",
        other => bail!("unsupported host OS: {other}"),
    };
    let arch = match std::env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        other => bail!("unsupported host arch: {other}"),
    };
    Ok(format!("{os}-{arch}-release"))
}

fn download_and_extract(tag: &str, triple: &str, into: &Path) -> Result<()> {
    let url = format!(
        "https://github.com/zed-industries/livekit-rust-sdks/releases/download/{tag}/webrtc-{triple}.zip"
    );
    let zip_path = into.join(format!("webrtc-{triple}.zip"));

    eprintln!("Downloading {url}");
    let status = Command::new("curl")
        .args(["-fL", "--retry", "3", "--progress-bar", "-o"])
        .arg(&zip_path)
        .arg(&url)
        .status()
        .context("running curl")?;
    if !status.success() {
        bail!("curl exited with {status} while downloading {url}");
    }

    eprintln!("Extracting into {}", into.display());
    let status = Command::new("unzip")
        .arg("-q")
        .arg("-o")
        .arg(&zip_path)
        .arg("-d")
        .arg(into)
        .status()
        .context("running unzip")?;
    if !status.success() {
        bail!(
            "unzip exited with {status} while extracting {}",
            zip_path.display()
        );
    }

    fs::remove_file(&zip_path).ok();
    Ok(())
}

fn curl_text(url: &str) -> Result<String> {
    let output = Command::new("curl")
        .args(["-fsSL", url])
        .output()
        .context("running curl")?;
    if !output.status.success() {
        bail!(
            "curl failed for {url} (exit {}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim(),
        );
    }
    String::from_utf8(output.stdout).context("curl returned non-UTF-8 body")
}

fn ensure_gitignore_entry(workspace_root: &Path) -> Result<()> {
    let path = workspace_root.join(".gitignore");
    let existing =
        fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    if existing
        .lines()
        .any(|line| line.trim() == GITIGNORE_ENTRY || line.trim() == LOCAL_DIR_NAME)
    {
        return Ok(());
    }
    let mut updated = existing;
    if !updated.ends_with('\n') {
        updated.push('\n');
    }
    updated.push_str(GITIGNORE_ENTRY);
    updated.push('\n');
    fs::write(&path, updated).with_context(|| format!("writing {}", path.display()))?;
    eprintln!("Added {GITIGNORE_ENTRY} to .gitignore");
    Ok(())
}

fn update_cargo_config(webrtc_path: &Path) -> Result<()> {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .context("could not determine home directory")?;
    let config_path = PathBuf::from(home).join(".cargo").join("config.toml");
    if config_path.exists() {
        bail!(
            "{} already exists; refusing to modify it. \
             Add `[env]\\n{ENV_VAR} = \"{}\"` yourself, \
             or re-run with --no-cargo-config.",
            config_path.display(),
            webrtc_path.display(),
        );
    }

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }

    let mut doc = DocumentMut::new();
    let mut env_table = Table::new();
    env_table.set_implicit(false);
    let path_str = webrtc_path
        .to_str()
        .context("webrtc path is not valid UTF-8")?;
    env_table.insert(ENV_VAR, value(path_str));
    doc.insert("env", Item::Table(env_table));

    fs::write(&config_path, doc.to_string())
        .with_context(|| format!("writing {}", config_path.display()))?;
    eprintln!("Wrote {} with {ENV_VAR}={path_str}", config_path.display());
    Ok(())
}
