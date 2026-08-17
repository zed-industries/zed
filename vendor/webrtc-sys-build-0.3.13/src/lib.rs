// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::{self, BufRead, Write},
    path,
    process::Command,
};

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::StatusCode;

pub const SCRATH_PATH: &str = "livekit_webrtc";
pub const WEBRTC_TAG: &str = "webrtc-0001d84-4";
pub const IGNORE_DEFINES: [&str; 2] = ["CR_CLANG_REVISION", "CR_XCODE_VERSION"];

pub fn target_os() -> String {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target = env::var("TARGET").unwrap();
    let is_simulator = target.ends_with("-sim");

    match target_os.as_str() {
        "windows" => "win",
        "macos" => "mac",
        "ios" => {
            if is_simulator {
                "ios-simulator"
            } else {
                "ios-device"
            }
        }
        _ => &target_os,
    }
    .to_string()
}

pub fn target_arch() -> String {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match target_arch.as_str() {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        _ => &target_arch,
    }
    .to_owned()
}

/// The full name of the webrtc library
/// e.g. mac-x64-release (Same name on GH releases)
pub fn webrtc_triple() -> String {
    let profile = if use_debug() { "debug" } else { "release" };
    format!("{}-{}-{}", target_os(), target_arch(), profile)
}

/// Using debug builds of webrtc is still experimental for now
/// On Windows, Rust doesn't link against libcmtd on debug, which is an issue
/// Default to false (even on cargo debug)
pub fn use_debug() -> bool {
    let var = env::var("LK_DEBUG_WEBRTC");
    var.is_ok() && var.unwrap() == "true"
}

/// The location of the custom build is defined by the user
pub fn custom_dir() -> Option<path::PathBuf> {
    if let Ok(path) = env::var("LK_CUSTOM_WEBRTC") {
        return Some(path::PathBuf::from(path));
    }
    None
}

/// Location of the downloaded webrtc binaries
pub fn prebuilt_dir() -> path::PathBuf {
    path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join(format!(
        "{}/{}",
        WEBRTC_TAG,
        webrtc_triple()
    ))
}

pub fn download_url() -> String {
    format!(
        "https://github.com/zed-industries/livekit-rust-sdks/releases/download/{}/{}.zip",
        WEBRTC_TAG,
        format!("webrtc-{}", webrtc_triple())
    )
}

/// Used location of libwebrtc depending on whether it's a custom build or not
pub fn webrtc_dir() -> path::PathBuf {
    if let Some(path) = custom_dir() {
        return path;
    }

    prebuilt_dir()
}

pub fn webrtc_defines() -> Vec<(String, Option<String>)> {
    // read preprocessor definitions from webrtc.ninja
    let defines_re = Regex::new(r"-D(\w+)(?:=([^\s]+))?").unwrap();
    let mut files = vec![webrtc_dir().join("webrtc.ninja")];
    // include desktop_capture.ninja to avoid ABI mismatch for DesktopCaptureOptions due to WEBRTC_USE_X11 missing
    // libwebrtc does not implement desktop capture on Android
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "android" {
        files.push(webrtc_dir().join("desktop_capture.ninja"));
    }

    let mut seen = HashSet::new();
    let mut vec = Vec::new();

    for path in files {
        let gni = fs::File::open(&path)
            .unwrap_or_else(|e| panic!("Could not open ninja file: {path:?}\n{e:?}"));

        let mut defines_line = String::default();
        io::BufReader::new(gni).read_line(&mut defines_line).unwrap();
        for cap in defines_re.captures_iter(&defines_line) {
            let define_name = &cap[1];
            let define_value = cap.get(2).map(|m| m.as_str());
            if IGNORE_DEFINES.contains(&define_name) {
                continue;
            }
            let value = define_value.map(str::to_string);
            let name = define_name.to_owned();
            if seen.insert((name.clone(), value.clone())) {
                vec.push((name, value));
            }
        }
    }

    vec
}

pub fn configure_jni_symbols() -> Result<()> {
    download_webrtc().context("Failed to download WebRTC binaries for JNI configuration")?;

    let toolchain = android_ndk_toolchain().context("Failed to locate Android NDK toolchain")?;
    let toolchain_bin = toolchain.join("bin");

    let webrtc_dir = webrtc_dir();
    let webrtc_lib = webrtc_dir.join("lib");

    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());

    // Find JNI symbols
    let readelf_output = Command::new(toolchain_bin.join("llvm-readelf"))
        .arg("-Ws")
        .arg(webrtc_lib.join("libwebrtc.a"))
        .output()
        .expect("failed to run llvm-readelf");

    let jni_regex = Regex::new(r"(Java_org_webrtc.*)").unwrap();
    let content = String::from_utf8_lossy(&readelf_output.stdout);
    let jni_symbols: Vec<&str> =
        jni_regex.captures_iter(&content).map(|cap| cap.get(1).unwrap().as_str()).collect();

    if jni_symbols.is_empty() {
        return Err(anyhow!("No JNI symbols found")); // Shouldn't happen
    }

    // Keep JNI symbols
    for symbol in &jni_symbols {
        println!("cargo:rustc-link-arg=-Wl,--undefined={}", symbol);
    }

    // Version script
    let vs_path = out_dir.join("webrtc_jni.map");
    let mut vs_file = fs::File::create(&vs_path).context("Failed to create version script file")?;

    let jni_symbols = jni_symbols.join("; ");
    write!(vs_file, "JNI_WEBRTC {{\n\tglobal: {}; \n}};", jni_symbols)
        .context("Failed to write version script")?;

    println!("cargo:rustc-link-arg=-Wl,--version-script={}", vs_path.display());

    Ok(())
}

pub fn download_webrtc() -> Result<()> {
    if custom_dir().is_some() {
        return Ok(());
    }

    let webrtc_dir = prebuilt_dir();
    let _ = fs::remove_dir_all(&webrtc_dir);

    let mut resp = reqwest::blocking::get(download_url())
        .context("Failed to send HTTP request to download WebRTC")?;
    if resp.status() != StatusCode::OK {
        return Err(anyhow!("failed to download webrtc: {}", resp.status()));
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let tmp_path = PathBuf::from(out_dir).join("webrtc.zip");
    let mut file = fs::File::options()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(&tmp_path)
        .context("Failed to create temporary file for WebRTC download")?;
    resp.copy_to(&mut file).context("Failed to write WebRTC download to temporary file")?;

    let mut archive = zip::ZipArchive::new(file).context("Failed to open WebRTC zip archive")?;
    archive.extract(webrtc_dir.parent().unwrap()).context("Failed to extract WebRTC archive")?;
    drop(archive);

    fs::remove_file(&tmp_path).context("Failed to remove temporary WebRTC zip file")?;
    Ok(())
}

pub fn android_ndk_toolchain() -> Result<path::PathBuf> {
    let host_os = host_os();

    let home = env::var("HOME");
    let local = env::var("LOCALAPPDATA");

    let home = if host_os == Some("linux") {
        path::PathBuf::from(home.unwrap())
    } else if host_os == Some("darwin") {
        path::PathBuf::from(home.unwrap()).join("Library")
    } else if host_os == Some("windows") {
        path::PathBuf::from(local.unwrap())
    } else {
        return Err(anyhow!("Unsupported host OS"));
    };

    let ndk_dir = || -> Option<path::PathBuf> {
        let ndk_env = env::var("ANDROID_NDK_HOME");
        if let Ok(ndk_env) = ndk_env {
            return Some(path::PathBuf::from(ndk_env));
        }

        let ndk_dir = home.join("Android/sdk/ndk");
        if !ndk_dir.exists() {
            return None;
        }

        // Find the highest version
        let versions = fs::read_dir(ndk_dir.clone());
        if versions.is_err() {
            return None;
        }

        let version = versions
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|dir| dir.file_name().to_str().map(ToOwned::to_owned))
            .filter_map(|dir| semver::Version::parse(&dir).ok())
            .max_by(semver::Version::cmp);

        version.as_ref()?;

        let version = version.unwrap();
        Some(ndk_dir.join(version.to_string()))
    }();

    if let Some(ndk_dir) = ndk_dir {
        let llvm_dir = if host_os == Some("linux") {
            "linux-x86_64"
        } else if host_os == Some("darwin") {
            "darwin-x86_64"
        } else if host_os == Some("windows") {
            "windows-x86_64"
        } else {
            return Err(anyhow!("Unsupported host OS"));
        };

        Ok(ndk_dir.join(format!("toolchains/llvm/prebuilt/{}", llvm_dir)))
    } else {
        Err(anyhow!("Android NDK not found, please set ANDROID_NDK_HOME to your NDK path"))
    }
}

fn host_os() -> Option<&'static str> {
    let host = env::var("HOST").unwrap();
    if host.contains("darwin") {
        Some("darwin")
    } else if host.contains("linux") {
        Some("linux")
    } else if host.contains("windows") {
        Some("windows")
    } else {
        None
    }
}
