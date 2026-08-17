// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use super::*;
use crate::{EnvGuard, ENV_MUTEX};
use std::sync::MutexGuard;

/// RAII bundle pairing the env-var guards with the global `ENV_MUTEX` lock, so
/// detection tests (which read process-wide env vars) never run concurrently.
struct DetectEnvGuard<'a> {
    _guards: Vec<EnvGuard>,
    _lock: MutexGuard<'a, ()>,
}

/// Locks the global env mutex and sets the Cargo target vars the detection path
/// reads, plus any `extra` vars (notably the `OPENSSL_*` family). `TARGET` is
/// fixed so the target-suffixed variants are deterministic.
///
/// All `OPENSSL_*` discovery vars are removed by default so an ambient value on
/// the developer's machine can't make a test flaky; `extra` overrides the
/// specific ones a test needs. Keys are deduplicated (last value wins) so each
/// var gets exactly one `EnvGuard` and is restored correctly on drop.
fn detect_env(extra: &[(&str, &str)]) -> DetectEnvGuard<'static> {
    let lock = ENV_MUTEX
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut vars: Vec<(&str, Option<&str>)> = vec![
        ("CARGO_CFG_TARGET_OS", Some(std::env::consts::OS)),
        ("CARGO_CFG_TARGET_ENV", Some("")),
        ("CARGO_CFG_TARGET_ARCH", Some(std::env::consts::ARCH)),
        ("CARGO_CFG_TARGET_POINTER_WIDTH", Some("64")),
        ("TARGET", Some("testarch")),
        ("CARGO_PKG_NAME", Some("aws-lc-sys")),
        ("OPENSSL_DIR", None),
        ("OPENSSL_DIR_testarch", None),
        ("OPENSSL_INCLUDE_DIR", None),
        ("OPENSSL_INCLUDE_DIR_testarch", None),
        ("OPENSSL_LIB_DIR", None),
        ("OPENSSL_LIB_DIR_testarch", None),
    ];
    for &(key, val) in extra {
        if let Some(slot) = vars.iter_mut().find(|(k, _)| *k == key) {
            slot.1 = Some(val);
        } else {
            vars.push((key, Some(val)));
        }
    }
    let guards = vars
        .iter()
        .map(|(key, val)| match val {
            Some(val) => EnvGuard::new(key, *val),
            None => EnvGuard::remove(key),
        })
        .collect();
    DetectEnvGuard {
        _guards: guards,
        _lock: lock,
    }
}

fn write_base_h(include_dir: &Path, body: &str) {
    let openssl = include_dir.join("openssl");
    std::fs::create_dir_all(&openssl).unwrap();
    std::fs::write(openssl.join("base.h"), body).unwrap();
}

/// Points both `OPENSSL_DIR` and its target-suffixed variant at the same dir.
fn openssl_dir_vars(dir: &str) -> [(&str, &str); 2] {
    [("OPENSSL_DIR", dir), ("OPENSSL_DIR_testarch", dir)]
}

#[test]
fn test_bindings_prefix_from_include_dir() {
    let root = PathBuf::from("install");
    assert_eq!(
        bindings_prefix_from_include_dir(&root.join("include")).unwrap(),
        root
    );
    assert_eq!(
        bindings_prefix_from_include_dir(&root.join("include").join("aws-lc")).unwrap(),
        root
    );
}

#[test]
#[cfg(unix)]
fn test_pkg_config_modules_include_awslc_package_names() {
    assert_eq!(
        PKG_CONFIG_MODULES,
        ["openssl", "aws-lc", "libcrypto", "libcrypto-awslc"]
    );
}

#[test]
fn test_detect_rejects_non_awslc_headers() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::create_dir_all(root.join("lib")).unwrap();
    std::fs::write(root.join("lib").join("libcrypto.a"), b"").unwrap();
    write_base_h(&root.join("include"), "// Not AWS-LC\n");

    let dir = root.to_str().unwrap();
    let _env = detect_env(&openssl_dir_vars(dir));

    // A real library is present, but the headers lack the AWS-LC marker, so
    // detection must decline (return None) rather than error the build.
    assert!(detect_system_awslc(Path::new(".")).is_none());
}

#[test]
fn test_detect_returns_none_when_no_library() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    // include/ exists but there is no library to link against.
    std::fs::create_dir_all(root.join("include").join("openssl")).unwrap();

    let dir = root.to_str().unwrap();
    let _env = detect_env(&openssl_dir_vars(dir));

    assert!(detect_system_awslc(Path::new(".")).is_none());
}

#[test]
fn test_detect_returns_none_when_openssl_dir_points_nowhere() {
    let temp = tempfile::tempdir().unwrap();
    let missing = temp.path().join("does-not-exist");

    let dir = missing.to_str().unwrap();
    let _env = detect_env(&openssl_dir_vars(dir));

    assert!(detect_system_awslc(Path::new(".")).is_none());
}

#[test]
fn test_detect_include_lib_split_reaches_probe() {
    // Headers and libraries in *separate* directories (no shared prefix),
    // exercising the OPENSSL_INCLUDE_DIR + OPENSSL_LIB_DIR / from_paths route.
    let temp = tempfile::tempdir().unwrap();
    let include_dir = temp.path().join("headers");
    let lib_dir = temp.path().join("libs");
    std::fs::create_dir_all(&lib_dir).unwrap();
    std::fs::write(lib_dir.join("libcrypto.a"), b"").unwrap();
    write_base_h(&include_dir, "// Not AWS-LC\n");

    let inc = include_dir.to_str().unwrap();
    let lib = lib_dir.to_str().unwrap();
    // OPENSSL_DIR intentionally unset so only the split vars drive detection.
    let _env = detect_env(&[
        ("OPENSSL_INCLUDE_DIR", inc),
        ("OPENSSL_INCLUDE_DIR_testarch", inc),
        ("OPENSSL_LIB_DIR", lib),
        ("OPENSSL_LIB_DIR_testarch", lib),
    ]);

    // The library resolves from OPENSSL_LIB_DIR and the headers from
    // OPENSSL_INCLUDE_DIR, so the candidate reaches validation; the missing
    // AWS-LC marker then makes detection decline rather than error.
    assert!(detect_system_awslc(Path::new(".")).is_none());
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_detect_include_lib_split_finds_bindings_from_include_prefix() {
    let temp = tempfile::tempdir().unwrap();
    let include_prefix = temp.path().join("include-prefix");
    let lib_prefix = temp.path().join("lib-prefix");
    let include_dir = include_prefix.join("include");
    let lib_dir = lib_prefix.join("lib");
    let bindings_dir = include_prefix.join("share").join("rust");

    std::fs::create_dir_all(&lib_dir).unwrap();
    std::fs::write(lib_dir.join("libcrypto.a"), b"").unwrap();
    #[cfg(feature = "ssl")]
    std::fs::write(lib_dir.join("libssl.a"), b"").unwrap();
    write_base_h(
        &include_dir,
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"99.0.0\"\n",
    );
    std::fs::create_dir_all(&bindings_dir).unwrap();
    std::fs::write(bindings_dir.join("aws_lc_bindings.rs"), b"// bindings").unwrap();

    let inc = include_dir.to_str().unwrap();
    let lib = lib_dir.to_str().unwrap();
    let _env = detect_env(&[
        ("OPENSSL_INCLUDE_DIR", inc),
        ("OPENSSL_INCLUDE_DIR_testarch", inc),
        ("OPENSSL_LIB_DIR", lib),
        ("OPENSSL_LIB_DIR_testarch", lib),
    ]);

    let detected = detect_system_awslc(Path::new("."));
    assert!(detected.is_some());
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_detect_include_lib_split_finds_bindings_from_cohabiting_include_prefix() {
    let temp = tempfile::tempdir().unwrap();
    let install_prefix = temp.path().join("install");
    let lib_prefix = temp.path().join("lib-prefix");
    let include_dir = install_prefix.join("include").join("aws-lc");
    let lib_dir = lib_prefix.join("lib");
    let bindings_dir = install_prefix.join("share").join("rust");

    std::fs::create_dir_all(&lib_dir).unwrap();
    std::fs::write(lib_dir.join("libcrypto-awslc.a"), b"").unwrap();
    #[cfg(feature = "ssl")]
    std::fs::write(lib_dir.join("libssl-awslc.a"), b"").unwrap();
    write_base_h(
        &include_dir,
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"99.0.0\"\n",
    );
    std::fs::create_dir_all(&bindings_dir).unwrap();
    std::fs::write(bindings_dir.join("aws_lc_bindings.rs"), b"// bindings").unwrap();

    let inc = include_dir.to_str().unwrap();
    let lib = lib_dir.to_str().unwrap();
    let _env = detect_env(&[
        ("OPENSSL_INCLUDE_DIR", inc),
        ("OPENSSL_INCLUDE_DIR_testarch", inc),
        ("OPENSSL_LIB_DIR", lib),
        ("OPENSSL_LIB_DIR_testarch", lib),
    ]);

    let detected = detect_system_awslc(Path::new("."));
    assert!(detected.is_some());
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_detect_derives_include_from_openssl_dir_when_only_lib_set() {
    // OPENSSL_DIR + OPENSSL_LIB_DIR, no OPENSSL_INCLUDE_DIR: the include half
    // is derived from OPENSSL_DIR while the lib half comes from OPENSSL_LIB_DIR.
    // The library lives only in the separate lib dir, so adoption proves
    // OPENSSL_LIB_DIR was used.
    let temp = tempfile::tempdir().unwrap();
    let prefix = temp.path().join("prefix");
    let lib_dir = temp.path().join("separate-libs");
    let include_dir = prefix.join("include");
    let bindings_dir = prefix.join("share").join("rust");

    std::fs::create_dir_all(&lib_dir).unwrap();
    std::fs::write(lib_dir.join("libcrypto.a"), b"").unwrap();
    #[cfg(feature = "ssl")]
    std::fs::write(lib_dir.join("libssl.a"), b"").unwrap();
    write_base_h(
        &include_dir,
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"99.0.0\"\n",
    );
    std::fs::create_dir_all(&bindings_dir).unwrap();
    std::fs::write(bindings_dir.join("aws_lc_bindings.rs"), b"// bindings").unwrap();

    let lib = lib_dir.to_str().unwrap();
    let mut env: Vec<(&str, &str)> = openssl_dir_vars(prefix.to_str().unwrap()).to_vec();
    env.push(("OPENSSL_LIB_DIR", lib));
    env.push(("OPENSSL_LIB_DIR_testarch", lib));
    let _env = detect_env(&env);

    assert!(detect_system_awslc(Path::new(".")).is_some());
}
