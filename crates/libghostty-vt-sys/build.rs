use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Pinned ghostty commit. Update this to pull a newer version.
const GHOSTTY_REPO: &str = "https://github.com/ghostty-org/ghostty.git";
const GHOSTTY_COMMIT: &str = "bebca84668947bfc92b9a30ed58712e1c34eee1d";
const VENDORED_GHOSTTY_DIR: &str = "vendor/ghostty";
const VENDORED_ZIG_SYSTEM_DIR: &str = "vendor/zig";

enum GhosttySource {
    Vendored(PathBuf),
    Fetched(PathBuf),
    External(PathBuf),
}

impl GhosttySource {
    fn path(&self) -> &Path {
        match self {
            Self::Vendored(path) | Self::Fetched(path) | Self::External(path) => path,
        }
    }

    fn can_patch(&self) -> bool {
        matches!(self, Self::Fetched(_))
    }
}

fn main() {
    // docs.rs has no Zig toolchain. The checked-in bindings in src/bindings.rs
    // are enough for generating documentation, so skip the entire native
    // build when running under docs.rs.
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-env-changed=GHOSTTY_SOURCE_DIR");
    println!("cargo:rerun-if-env-changed=GHOSTTY_ZIG_SYSTEM_DIR");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=HOST");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));
    let target = env::var("TARGET").expect("TARGET must be set");
    let host = env::var("HOST").expect("HOST must be set");

    let ghostty_source = locate_ghostty_source(&manifest_dir, &out_dir);
    let ghostty_dir = ghostty_source.path();
    rerun_if_ghostty_source_changed(ghostty_dir);

    if target.contains("darwin") {
        ensure_ghostty_macos_deployment_target(ghostty_dir, ghostty_source.can_patch());
    }

    // Build libghostty-vt via zig.
    let install_prefix = out_dir.join("ghostty-install");

    let mut build = Command::new("zig");
    build
        .arg("build")
        .arg("-Demit-lib-vt")
        .arg("--prefix")
        .arg(&install_prefix)
        .current_dir(&ghostty_dir);

    if let Some(zig_system_dir) = zig_system_dir(&manifest_dir) {
        build.arg("--system").arg(zig_system_dir);
    }

    // Only pass -Dtarget when cross-compiling. For native builds, let zig
    // auto-detect the host (matches how ghostty's own CMakeLists.txt works).
    if target != host {
        let zig_target = zig_target(&target);
        build.arg(format!("-Dtarget={zig_target}"));
    }

    run(build, "zig build");

    let lib_dir = install_prefix.join("lib");
    let include_dir = install_prefix.join("include");

    let ghostty_library_file_name = static_library_file_name("ghostty-vt", &target);
    let simdutf_library_file_name = static_library_file_name("simdutf", &target);
    let highway_library_file_name = static_library_file_name("highway", &target);

    assert!(
        lib_dir.join(&ghostty_library_file_name).exists(),
        "expected static library at {}",
        lib_dir.join(&ghostty_library_file_name).display()
    );
    assert!(
        include_dir.join("ghostty").join("vt.h").exists(),
        "expected header at {}",
        include_dir.join("ghostty").join("vt.h").display()
    );

    let simdutf_dir = static_dependency_dir(&ghostty_dir, &simdutf_library_file_name);
    let highway_dir = static_dependency_dir(&ghostty_dir, &highway_library_file_name);

    if target.contains("darwin") && host.contains("darwin") {
        normalize_macos_static_archive(&lib_dir.join(&ghostty_library_file_name));
        normalize_macos_static_archive(&simdutf_dir.join(&simdutf_library_file_name));
        normalize_macos_static_archive(&highway_dir.join(&highway_library_file_name));
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-search=native={}", simdutf_dir.display());
    println!("cargo:rustc-link-search=native={}", highway_dir.display());
    println!("cargo:rustc-link-lib=static=ghostty-vt");
    println!("cargo:rustc-link-lib=static=simdutf");
    println!("cargo:rustc-link-lib=static=highway");
    if target.contains("darwin") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    println!("cargo:include={}", include_dir.display());
}

fn locate_ghostty_source(manifest_dir: &Path, out_dir: &Path) -> GhosttySource {
    if let Ok(dir) = env::var("GHOSTTY_SOURCE_DIR") {
        let path = PathBuf::from(dir);
        assert!(
            path.join("build.zig").exists(),
            "GHOSTTY_SOURCE_DIR does not contain build.zig: {}",
            path.display()
        );
        return GhosttySource::External(path);
    }

    let vendored_path = manifest_dir.join(VENDORED_GHOSTTY_DIR);
    if vendored_path.join("build.zig").exists() {
        return GhosttySource::Vendored(vendored_path);
    }

    GhosttySource::Fetched(fetch_ghostty(out_dir))
}

fn zig_system_dir(manifest_dir: &Path) -> Option<PathBuf> {
    if let Ok(dir) = env::var("GHOSTTY_ZIG_SYSTEM_DIR") {
        return Some(PathBuf::from(dir));
    }

    let vendored_path = manifest_dir.join(VENDORED_ZIG_SYSTEM_DIR);
    if vendored_path.exists() {
        return Some(vendored_path);
    }

    None
}

fn rerun_if_ghostty_source_changed(ghostty_dir: &Path) {
    for path in ["build.zig", "build.zig.zon", "src", "pkg"] {
        println!(
            "cargo:rerun-if-changed={}",
            ghostty_dir.join(path).display()
        );
    }
}

fn ensure_ghostty_macos_deployment_target(ghostty_dir: &Path, can_patch: bool) {
    let config_path = ghostty_dir.join("src").join("build").join("Config.zig");
    let source = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", config_path.display()));

    let macos_13 = ".macos => .{ .semver = .{\n            .major = 13,\n            .minor = 0,\n            .patch = 0,\n        } },";
    let previous_zed_macos_11_target = ".macos => .{ .semver = .{\n            .major = 11,\n            .minor = 0,\n            .patch = 0,\n        } },";
    let macos_10_15_7 = ".macos => .{ .semver = .{\n            .major = 10,\n            .minor = 15,\n            .patch = 7,\n        } },";

    if source.contains(macos_10_15_7) {
        return;
    }

    let source_target = [macos_13, previous_zed_macos_11_target]
        .into_iter()
        .find(|target| source.contains(target))
        .unwrap_or_else(|| {
            panic!(
                "failed to find macOS deployment target in {}",
                config_path.display()
            )
        });
    assert!(
        can_patch,
        "Ghostty source does not use Zed's macOS deployment target; provide an already-patched \
         source with a macOS 10.15.7 deployment target or let build.rs fetch the pinned source"
    );

    let patched = source.replace(source_target, macos_10_15_7);
    std::fs::write(&config_path, patched)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", config_path.display()));
}

fn static_dependency_dir(ghostty_dir: &Path, file_name: &str) -> PathBuf {
    let cache_dir = ghostty_dir.join(".zig-cache").join("o");
    let entries = std::fs::read_dir(&cache_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", cache_dir.display()));

    for entry in entries {
        let entry = entry.unwrap_or_else(|error| {
            panic!("failed to read entry in {}: {error}", cache_dir.display())
        });
        let path = entry.path();
        if path.join(file_name).exists() {
            return path;
        }
    }

    panic!("failed to find {file_name} under {}", cache_dir.display());
}

fn static_library_file_name(name: &str, target: &str) -> String {
    if target.contains("windows-msvc") {
        format!("{name}.lib")
    } else {
        format!("lib{name}.a")
    }
}

fn normalize_macos_static_archive(path: &Path) {
    let mut ranlib = Command::new("ranlib");
    ranlib.arg("-c").arg(path);
    run(ranlib, "ranlib static library");
}

fn fetch_ghostty(out_dir: &Path) -> PathBuf {
    let src_dir = out_dir.join("ghostty-src");
    let stamp = src_dir.join(".ghostty-commit");

    if stamp.exists()
        && let Ok(existing) = std::fs::read_to_string(&stamp)
        && existing.trim() == GHOSTTY_COMMIT
    {
        return src_dir;
    }

    if src_dir.exists() {
        std::fs::remove_dir_all(&src_dir)
            .unwrap_or_else(|error| panic!("failed to remove {}: {error}", src_dir.display()));
    }

    eprintln!("Fetching ghostty {GHOSTTY_COMMIT} ...");

    let mut clone = Command::new("git");
    clone
        .arg("clone")
        .arg("--filter=blob:none")
        .arg("--no-checkout")
        .arg(GHOSTTY_REPO)
        .arg(&src_dir);
    run(clone, "git clone ghostty");

    let mut checkout = Command::new("git");
    checkout
        .arg("checkout")
        .arg(GHOSTTY_COMMIT)
        .current_dir(&src_dir);
    run(checkout, "git checkout ghostty commit");

    std::fs::write(&stamp, GHOSTTY_COMMIT)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", stamp.display()));

    src_dir
}

fn run(mut command: Command, context: &str) {
    let status = command
        .status()
        .unwrap_or_else(|error| panic!("failed to execute {context}: {error}"));
    assert!(status.success(), "{context} failed with status {status}");
}

fn zig_target(target: &str) -> String {
    let value = match target {
        "x86_64-unknown-linux-gnu" => "x86_64-linux-gnu",
        "x86_64-unknown-linux-musl" => "x86_64-linux-musl",
        "aarch64-unknown-linux-gnu" => "aarch64-linux-gnu",
        "aarch64-unknown-linux-musl" => "aarch64-linux-musl",
        "aarch64-apple-darwin" => "aarch64-macos-none",
        "x86_64-apple-darwin" => "x86_64-macos-none",
        "aarch64-pc-windows-msvc" => "aarch64-windows-msvc",
        "x86_64-pc-windows-msvc" => "x86_64-windows-msvc",
        other => panic!("unsupported Rust target for vendored build: {other}"),
    };
    value.to_owned()
}
