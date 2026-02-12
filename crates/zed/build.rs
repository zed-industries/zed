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

        if let Some(build_identifier) = option_env!("GITHUB_RUN_NUMBER") {
            println!("cargo:rustc-env=ZED_BUILD_ID={build_identifier}");
        }

        if let Ok(build_profile) = std::env::var("PROFILE")
            && build_profile == "release"
        {
            // This is currently the best way to make `cargo build ...`'s build script
            // to print something to stdout without extra verbosity.
            println!("cargo::warning=Info: using '{git_sha}' hash for ZED_COMMIT_SHA env var");
        }
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(target_env = "msvc")]
        {
            // todo(windows): This is to avoid stack overflow. Remove it when solved.
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }

        if cfg!(target_arch = "x86_64") || cfg!(target_arch = "aarch64") {
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let out_dir: &std::path::Path = out_dir.as_ref();
            let target_dir = std::path::Path::new(&out_dir)
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .expect("Failed to find target directory");

            let conpty_dll_target = target_dir.join("conpty.dll");
            let open_console_target = target_dir.join("OpenConsole.exe");

            let conpty_url = "https://github.com/microsoft/terminal/releases/download/v1.23.13503.0/Microsoft.Windows.Console.ConPTY.1.23.251216003.nupkg";
            let nupkg_path = out_dir.join("conpty.nupkg.zip");
            let extract_dir = out_dir.join("conpty");

            let download_script = format!(
                "$ProgressPreference = 'SilentlyContinue'; [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                conpty_url,
                nupkg_path.display()
            );

            let download_result = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    &download_script,
                ])
                .output();

            match download_result {
                Ok(output) if output.status.success() => {
                    println!("Downloaded conpty nupkg successfully");

                    let extract_script = format!(
                        "$ProgressPreference = 'SilentlyContinue'; Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                        nupkg_path.display(),
                        extract_dir.display()
                    );

                    let extract_result = Command::new("powershell")
                        .args(["-NoProfile", "-NonInteractive", "-Command", &extract_script])
                        .output();

                    match extract_result {
                        Ok(output) if output.status.success() => {
                            let (conpty_dll_source, open_console_source) =
                                if cfg!(target_arch = "x86_64") {
                                    (
                                        extract_dir.join("runtimes/win-x64/native/conpty.dll"),
                                        extract_dir
                                            .join("build/native/runtimes/x64/OpenConsole.exe"),
                                    )
                                } else {
                                    (
                                        extract_dir.join("runtimes/win-arm64/native/conpty.dll"),
                                        extract_dir
                                            .join("build/native/runtimes/arm64/OpenConsole.exe"),
                                    )
                                };

                            match std::fs::copy(&conpty_dll_source, &conpty_dll_target) {
                                Ok(_) => {
                                    println!("Copied conpty.dll to {}", conpty_dll_target.display())
                                }
                                Err(e) => println!(
                                    "cargo::warning=Failed to copy conpty.dll from {}: {}",
                                    conpty_dll_source.display(),
                                    e
                                ),
                            }

                            match std::fs::copy(&open_console_source, &open_console_target) {
                                Ok(_) => println!(
                                    "Copied OpenConsole.exe to {}",
                                    open_console_target.display()
                                ),
                                Err(e) => println!(
                                    "cargo::warning=Failed to copy OpenConsole.exe from {}: {}",
                                    open_console_source.display(),
                                    e
                                ),
                            }
                        }
                        Ok(output) => {
                            println!(
                                "cargo::warning=Failed to extract conpty nupkg: {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                        }
                        Err(e) => {
                            println!(
                                "cargo::warning=Failed to run PowerShell for extraction: {}",
                                e
                            );
                        }
                    }
                }
                Ok(output) => {
                    println!(
                        "cargo::warning=Failed to download conpty nupkg: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                Err(e) => {
                    println!(
                        "cargo::warning=Failed to run PowerShell for download: {}",
                        e
                    );
                }
            }
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
}
