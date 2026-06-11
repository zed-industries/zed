#![allow(
    clippy::disallowed_methods,
    reason = "build helper used only from build scripts"
)]
#![cfg(target_os = "windows")]

use std::process::Command;

fn git_sha() -> Option<String> {
    if let Ok(sha) = std::env::var("ZED_COMMIT_SHA") {
        return Some(sha);
    }

    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn product_version() -> String {
    let commit_sha = git_sha();
    let pkg_version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let channel = std::env::var("RELEASE_CHANNEL").unwrap_or_else(|_| "dev".into());
    let build_id = std::env::var("GITHUB_RUN_NUMBER").ok();

    let mut metadata = channel;
    if let Some(build_id) = &build_id {
        metadata.push('.');
        metadata.push_str(build_id);
    }
    if let Some(sha) = &commit_sha {
        metadata.push('.');
        metadata.push_str(sha);
    }

    format!("{pkg_version}+{metadata}")
}

const ICON_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../zed/resources/windows");
const MANIFEST_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/manifest.xml");

pub fn compile(manifest: bool) -> Result<(), Box<dyn std::error::Error>> {
    let channel = option_env!("RELEASE_CHANNEL").unwrap_or("dev");
    let (icon_filename, product_name) = match channel {
        "stable" => ("app-icon.ico", "Zed"),
        "preview" => ("app-icon-preview.ico", "Zed Preview"),
        "nightly" => ("app-icon-nightly.ico", "Zed Nightly"),
        _ => ("app-icon-dev.ico", "Zed Dev"),
    };
    let icon = std::path::PathBuf::from(ICON_DIR).join(icon_filename);
    let icon_escaped = icon.to_string_lossy().replace('\\', "\\\\");

    let manifest_line = if manifest {
        let escaped = MANIFEST_PATH.replace('\\', "\\\\");
        format!("1 24 \"{escaped}\"")
    } else {
        String::new()
    };

    let pkg_version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let product_version = product_version();
    let mut version_parts = pkg_version
        .split('.')
        .map(|part| part.parse::<u16>().unwrap_or(0))
        .chain(std::iter::repeat(0));
    let file_version = format!(
        "{},{},{},{}",
        version_parts.next().unwrap_or(0),
        version_parts.next().unwrap_or(0),
        version_parts.next().unwrap_or(0),
        version_parts.next().unwrap_or(0),
    );

    let rc_content = format!(
        r#"1 ICON "{icon_escaped}"
{manifest_line}

1 VERSIONINFO
FILEVERSION {file_version}
PRODUCTVERSION {file_version}
FILEFLAGSMASK 0x3fL
FILEFLAGS 0x0L
FILEOS 0x40004L
FILETYPE 0x1L
FILESUBTYPE 0x0L
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "FileDescription", "{product_name}\0"
            VALUE "FileVersion", "{pkg_version}\0"
            VALUE "ProductName", "{product_name}\0"
            VALUE "ProductVersion", "{product_version}\0"
            VALUE "CompanyName", "Zed Industries, Inc.\0"
            VALUE "LegalCopyright", "Copyright 2022 - 2025 Zed Industries, Inc.\0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x0409, 1200
    END
END
"#
    );

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let rc_path = out_dir.join("zed_resources.rc");
    std::fs::write(&rc_path, rc_content)?;

    if let Ok(toolkit_path) = std::env::var("ZED_RC_TOOLKIT_PATH") {
        let rc_exe = std::path::Path::new(&toolkit_path).join("rc.exe");
        unsafe {
            std::env::set_var("RC", rc_exe);
        }
    }

    embed_resource::compile(&rc_path, embed_resource::NONE)
        .manifest_optional()
        .unwrap();

    Ok(())
}
