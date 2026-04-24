#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
use std::process::Command;

fn main() {
    if std::env::var("ZED_UPDATE_EXPLANATION").is_ok() {
        println!(r#"cargo:rustc-cfg=feature="no-bundled-uninstall""#);
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");
    }

    // Populate git sha environment variable if git is available
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    if let Some(output) = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
    {
        let git_sha = String::from_utf8_lossy(&output.stdout);
        let git_sha = git_sha.trim();

        println!("cargo:rustc-env=ZED_COMMIT_SHA={git_sha}");
    }
    if let Some(build_identifier) = option_env!("GITHUB_RUN_NUMBER") {
        println!("cargo:rustc-env=ZED_BUILD_ID={build_identifier}");
    }

    if cfg!(windows) {
        let release_channel = option_env!("RELEASE_CHANNEL").unwrap_or("dev");
        let icon_path = match release_channel {
            "stable" => "../zed/resources/windows/app-icon.ico",
            "preview" => "../zed/resources/windows/app-icon-preview.ico",
            "nightly" => "../zed/resources/windows/app-icon-nightly.ico",
            "dev" => "../zed/resources/windows/app-icon-dev.ico",
            _ => "../zed/resources/windows/app-icon-dev.ico",
        };
        let icon_path = std::path::Path::new(icon_path);

        println!("cargo:rerun-if-env-changed=RELEASE_CHANNEL");
        println!("cargo:rerun-if-changed={}", icon_path.display());

        #[cfg(windows)]
        {
            let manifest_dir =
                std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
            let icon_path = manifest_dir.join(icon_path);
            let icon_path = icon_path.to_string_lossy().replace('\\', "\\\\");
            let package_version = std::env::var("CARGO_PKG_VERSION").unwrap();
            let mut version = package_version
                .split('.')
                .map(|part| part.parse::<u16>().unwrap_or(0))
                .chain(std::iter::repeat(0));
            let file_version = format!(
                "{},{},{},{}",
                version.next().unwrap(),
                version.next().unwrap(),
                version.next().unwrap(),
                version.next().unwrap()
            );
            let rc = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("cli.rc");

            let rc_contents = format!(
                r#"1 ICON "{icon_path}"

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
            VALUE "FileDescription", "Zed\0"
            VALUE "FileVersion", "{package_version}\0"
            VALUE "ProductName", "Zed\0"
            VALUE "ProductVersion", "{package_version}\0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x0409, 1200
    END
END
"#
            );
            std::fs::write(&rc, rc_contents).unwrap();

            if let Some(explicit_rc_toolkit_path) = std::env::var("ZED_RC_TOOLKIT_PATH").ok() {
                let rc_path = std::path::Path::new(&explicit_rc_toolkit_path).join("rc.exe");
                unsafe {
                    std::env::set_var("RC", rc_path);
                }
            }

            embed_resource::compile_for(&rc, ["cli"], embed_resource::NONE)
                .manifest_optional()
                .unwrap();
        }
    }
}
