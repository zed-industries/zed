#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
use std::process::Command;

fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");

        // Weakly link ReplayKit to ensure Zed can be used on macOS 10.15+.
        println!("cargo:rustc-link-arg=-Wl,-weak_framework,ReplayKit");

        // Seems to be required to enable Swift concurrency
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

        // Register exported Objective-C selectors, protocols, etc
        println!("cargo:rustc-link-arg=-Wl,-ObjC");

        // weak link to support Catalina
        println!("cargo:rustc-link-arg=-Wl,-weak_framework,ScreenCaptureKit");
    }

    // Populate git sha environment variable if git is available
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output()
        && output.status.success()
    {
        let git_sha = String::from_utf8_lossy(&output.stdout);
        let git_sha = git_sha.trim();

        println!("cargo:rustc-env=ZED_COMMIT_SHA={git_sha}");

        if let Ok(build_profile) = std::env::var("PROFILE")
            && build_profile == "release"
        {
            // This is currently the best way to make `cargo build ...`'s build script
            // to print something to stdout without extra verbosity.
            println!("cargo:warning=Info: using '{git_sha}' hash for ZED_COMMIT_SHA env var");
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(target_env = "msvc")]
        {
            // todo(windows): This is to avoid stack overflow. Remove it when solved.
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }

        let release_channel = option_env!("RELEASE_CHANNEL").unwrap_or("dev");
        let icon = match release_channel {
            "stable" => "resources/windows/app-icon.ico",
            "preview" => "resources/windows/app-icon-preview.ico",
            "nightly" => "resources/windows/app-icon-nightly.ico",
            "dev" => "resources/windows/app-icon-dev.ico",
            _ => "resources/windows/app-icon-dev.ico",
        };
        let icon = std::path::Path::new(icon);

        println!("cargo:rerun-if-env-changed=RELEASE_CHANNEL");
        println!("cargo:rerun-if-changed={}", icon.display());

        let mut res = winresource::WindowsResource::new();

        // Depending on the security applied to the computer, winresource might fail
        // fetching the RC path. Therefore, we add a way to explicitly specify the
        // toolkit path, allowing winresource to use a valid RC path.
        if let Some(explicit_rc_toolkit_path) = std::env::var("ZED_RC_TOOLKIT_PATH").ok() {
            res.set_toolkit_path(explicit_rc_toolkit_path.as_str());
        }
        res.set_icon(icon.to_str().unwrap());
        res.set("FileDescription", "Zed");
        res.set("ProductName", "Zed");

        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    #[cfg(target_os = "linux")]
    {
        use image::{DynamicImage, ImageReader, ImageResult, imageops};
        use std::env;
        use std::path::Path;

        let out_dir = env::var("OUT_DIR").unwrap();

        let release_channel = option_env!("RELEASE_CHANNEL").unwrap_or("dev");
        let icon = match release_channel {
            "stable" => "resources/app-icon.png",
            "preview" => "resources/app-icon-preview.png",
            "nightly" => "resources/app-icon-nightly.png",
            "dev" | _ => "resources/app-icon-dev.png",
        };

        let icon_src = Path::new(icon);

        let resized_image = match || -> ImageResult<DynamicImage> {
            Ok(ImageReader::open(icon_src)?.decode()?)
        }() {
            Err(msg) => {
                eprintln!("failed to read or decode {}: {msg}", icon_src.display());
                std::process::exit(1);
            }
            Ok(image) => imageops::resize(&image, 256, 256, imageops::FilterType::Nearest),
        };

        // name should match include_bytes! call in src/zed.rs
        let icon_out_path = Path::new(&out_dir).join("app_icon.png");
        resized_image.save(&icon_out_path).expect("saving app icon");

        // verify icon can be read and decoded
        if let Err(msg) = ImageReader::open(&icon_out_path).unwrap().decode() {
            eprintln!(
                "error verifying {}: {msg} (resized from {icon})",
                icon_out_path.display(),
            );
            std::process::exit(1);
        }

        println!("cargo:rerun-if-env-changed=RELEASE_CHANNEL");
        println!("cargo:rerun-if-changed={}", icon_src.to_string_lossy());
    }
}
