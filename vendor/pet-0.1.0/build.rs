// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

fn main() {
    println!("cargo:rerun-if-env-changed=PET_BUILD_ID");
    println!("cargo:rerun-if-env-changed=BUILD_BUILDID");
    println!("cargo:rerun-if-env-changed=PET_COMMIT_SHA");
    println!("cargo:rerun-if-env-changed=BUILD_SOURCEVERSION");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");

    // Filter empties per-candidate so an explicitly-set-but-empty primary var
    // doesn't short-circuit the fallback chain (e.g., `PET_BUILD_ID=""`).
    if let Some(build_id) = std::env::var("PET_BUILD_ID")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var("BUILD_BUILDID")
                .ok()
                .filter(|value| !value.is_empty())
        })
    {
        println!("cargo:rustc-env=PET_BUILD_ID={build_id}");
    }

    // BUILD_SOURCEVERSION is set by Azure Pipelines; GITHUB_SHA by GitHub Actions.
    if let Some(commit_sha) = std::env::var("PET_COMMIT_SHA")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var("BUILD_SOURCEVERSION")
                .ok()
                .filter(|value| !value.is_empty())
        })
        .or_else(|| {
            std::env::var("GITHUB_SHA")
                .ok()
                .filter(|value| !value.is_empty())
        })
    {
        println!("cargo:rustc-env=PET_COMMIT_SHA={commit_sha}");
    }

    #[cfg(target_os = "windows")]
    {
        if std::env::var("CARGO_BIN_NAME").is_err() {
            return;
        }
        let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());

        let mut res = winresource::WindowsResource::new();
        res.set("ProductName", "Python Environment Tools");
        res.set("FileDescription", "Python Environment Tools");
        res.set("CompanyName", "Microsoft Corporation");
        res.set(
            "LegalCopyright",
            "Copyright (c) Microsoft Corporation. All rights reserved.",
        );
        res.set("OriginalFilename", "pet.exe");
        res.set("InternalName", "pet");
        res.set("FileVersion", &version);
        res.set("ProductVersion", &version);
        res.compile().expect("Failed to compile Windows resources");
    }
}
