use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Pinned ghostty commit. Update this to pull a newer version.
const GHOSTTY_REPO: &str = "https://github.com/ghostty-org/ghostty.git";
const GHOSTTY_COMMIT: &str = "bebca84668947bfc92b9a30ed58712e1c34eee1d";

enum GhosttySource {
    Fetched(PathBuf),
    External(PathBuf),
}

impl GhosttySource {
    fn path(&self) -> &Path {
        match self {
            Self::Fetched(path) | Self::External(path) => path,
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

    println!("cargo:rerun-if-env-changed=LIBGHOSTTY_VT_SYS_NO_VENDOR");
    println!("cargo:rerun-if-env-changed=GHOSTTY_SOURCE_DIR");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=HOST");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));
    let target = env::var("TARGET").expect("TARGET must be set");
    let host = env::var("HOST").expect("HOST must be set");

    // Locate ghostty source: env override > fetch into OUT_DIR.
    let ghostty_source = match env::var("GHOSTTY_SOURCE_DIR") {
        Ok(dir) => {
            let path = PathBuf::from(dir);
            assert!(
                path.join("build.zig").exists(),
                "GHOSTTY_SOURCE_DIR does not contain build.zig: {}",
                path.display()
            );
            GhosttySource::External(path)
        }
        Err(_) => GhosttySource::Fetched(fetch_ghostty(&out_dir)),
    };
    let ghostty_dir = ghostty_source.path();

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

    // Only pass -Dtarget when cross-compiling. For native builds, let zig
    // auto-detect the host (matches how ghostty's own CMakeLists.txt works).
    if target != host {
        let zig_target = zig_target(&target);
        build.arg(format!("-Dtarget={zig_target}"));
    }

    run(build, "zig build");

    let lib_dir = install_prefix.join("lib");
    let include_dir = install_prefix.join("include");

    let lib_name = "libghostty-vt.a";

    assert!(
        lib_dir.join(lib_name).exists(),
        "expected static library at {}",
        lib_dir.join(lib_name).display()
    );
    assert!(
        include_dir.join("ghostty").join("vt.h").exists(),
        "expected header at {}",
        include_dir.join("ghostty").join("vt.h").display()
    );

    let simdutf_dir = static_dependency_dir(&ghostty_dir, "libsimdutf.a");
    let highway_dir = static_dependency_dir(&ghostty_dir, "libhighway.a");

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

fn ensure_ghostty_macos_deployment_target(ghostty_dir: &Path, can_patch: bool) {
    let config_path = ghostty_dir.join("src").join("build").join("Config.zig");
    let source = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", config_path.display()));

    let macos_13 = ".macos => .{ .semver = .{\n            .major = 13,\n            .minor = 0,\n            .patch = 0,\n        } },";
    let macos_11 = ".macos => .{ .semver = .{\n            .major = 11,\n            .minor = 0,\n            .patch = 0,\n        } },";

    if source.contains(macos_11) {
        return;
    }

    assert!(
        source.contains(macos_13),
        "failed to find macOS deployment target in {}",
        config_path.display()
    );
    assert!(
        can_patch,
        "GHOSTTY_SOURCE_DIR uses Ghostty's default macOS 13 deployment target; unset \
         GHOSTTY_SOURCE_DIR to use the vendored source, or point it at an already-patched source"
    );

    let patched = source.replace(macos_13, macos_11);
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

/// Clone ghostty at the pinned commit into OUT_DIR/ghostty-src.
/// Reuses an existing clone if the commit matches.
fn fetch_ghostty(out_dir: &Path) -> PathBuf {
    let src_dir = out_dir.join("ghostty-src");
    let stamp = src_dir.join(".ghostty-commit");

    // Skip fetch if we already have the right commit.
    if stamp.exists()
        && let Ok(existing) = std::fs::read_to_string(&stamp)
        && existing.trim() == GHOSTTY_COMMIT
    {
        return src_dir;
    }

    // Clean and clone fresh.
    if src_dir.exists() {
        std::fs::remove_dir_all(&src_dir)
            .unwrap_or_else(|e| panic!("failed to remove {}: {e}", src_dir.display()));
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

    std::fs::write(&stamp, GHOSTTY_COMMIT).unwrap_or_else(|e| panic!("failed to write stamp: {e}"));

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
        other => panic!("unsupported Rust target for vendored build: {other}"),
    };
    value.to_owned()
}
