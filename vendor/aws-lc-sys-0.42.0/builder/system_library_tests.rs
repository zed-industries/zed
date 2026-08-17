// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use super::*;
use crate::fips_probe::verify_fips_install;
use crate::{EnvGuard, ENV_MUTEX};
use std::sync::MutexGuard;

/// RAII bundle that pairs a set of `EnvGuard`s (which restore env vars on
/// drop) with the global `ENV_MUTEX` guard. Drop order is field-declaration
/// order, so `_guards` runs first — env vars are restored *before* the lock
/// is released, preventing another test from observing partially-restored
/// state.
struct TestEnvGuard<'a> {
    _guards: Vec<EnvGuard>,
    _lock: MutexGuard<'a, ()>,
}

/// Temporary directory tree that mimics a minimal AWS-LC CMake install.
///
/// Provides a builder-style API so tests read as a concise declarative setup
/// rather than a series of `create_dir_all` / `write` calls:
///
/// ```
/// let fx = FakeInstall::new().touch_lib("crypto.lib").touch_bin("crypto.dll");
/// ```
struct FakeInstall {
    _temp: tempfile::TempDir,
    root: PathBuf,
}

impl FakeInstall {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().to_path_buf();
        Self { _temp: temp, root }
    }

    /// Returns the `lib/` subdirectory path (does not require it to exist).
    fn lib_dir(&self) -> PathBuf {
        self.root.join("lib")
    }

    /// Creates an empty file in `lib/`, creating `lib/` itself if needed.
    fn touch_lib(self, filename: &str) -> Self {
        let dir = self.root.join("lib");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(filename), b"").unwrap();
        self
    }

    /// Creates an empty file in `bin/`, creating `bin/` itself if needed.
    fn touch_bin(self, filename: &str) -> Self {
        let dir = self.root.join("bin");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(filename), b"").unwrap();
        self
    }

    /// Creates a subdirectory under root (e.g. `"lib64"`).
    fn mkdir(self, subdir: &str) -> Self {
        std::fs::create_dir_all(self.root.join(subdir)).unwrap();
        self
    }
}

/// Sets up Cargo environment variables needed for tests that call target_os(),
/// target_env(), etc. Returns a guard that restores the original environment
/// when dropped. The guard also holds a mutex lock to prevent parallel test
/// execution.
fn setup_test_env() -> TestEnvGuard<'static> {
    setup_test_env_with_target(std::env::consts::OS, "")
}

/// Like `setup_test_env`, but forces specific `CARGO_CFG_TARGET_OS` and
/// `CARGO_CFG_TARGET_ENV` values so tests can exercise platform-specific
/// resolution logic (notably MSVC) on any host.
fn setup_test_env_with_target(os: &str, env: &str) -> TestEnvGuard<'static> {
    // Acquire lock first to ensure exclusive access to env vars.
    //
    // WARNING: Do not call setup_test_env() multiple times in the same
    // scope/thread. std::sync::Mutex is not reentrant, so a second call from
    // the same thread will deadlock waiting for the lock held by the first.
    let lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let arch = std::env::consts::ARCH;
    let vars: &[(&str, &str)] = &[
        ("CARGO_CFG_TARGET_OS", os),
        ("CARGO_CFG_TARGET_ENV", env),
        ("CARGO_CFG_TARGET_FEATURE", ""),
        ("CARGO_CFG_TARGET_ARCH", arch),
        ("CARGO_CFG_TARGET_POINTER_WIDTH", "64"),
        ("TARGET", arch),
        ("CARGO_PKG_NAME", "aws-lc-sys"),
    ];

    let guards: Vec<EnvGuard> = vars
        .iter()
        .map(|(key, val)| EnvGuard::new(key, *val))
        .collect();

    TestEnvGuard {
        _guards: guards,
        _lock: lock,
    }
}

// -------------------------------------------------------------------------
// version_at_least
// -------------------------------------------------------------------------

#[test]
fn test_version_at_least_valid() {
    assert!(version_at_least("1.2.3", "1.2.3").unwrap());
    assert!(version_at_least("2.0.0", "1.9.9").unwrap());
    assert!(version_at_least("1.3.0", "1.2.9").unwrap());
    assert!(version_at_least("1.2.4", "1.2.3").unwrap());
    assert!(!version_at_least("1.9.9", "2.0.0").unwrap());
    assert!(!version_at_least("1.2.3", "1.3.0").unwrap());
    assert!(!version_at_least("1.2.2", "1.2.3").unwrap());
}

#[test]
fn test_version_at_least_invalid() {
    assert!(version_at_least("1.2", "1.2.3").is_err());
    assert!(version_at_least("1.2.3.4", "1.2.3").is_err());
    assert!(version_at_least("a.b.c", "1.2.3").is_err());
    assert!(version_at_least("1.2.3", "").is_err());
}

// -------------------------------------------------------------------------
// validate_and_extract_version
// -------------------------------------------------------------------------

fn write_base_h(dir: &Path, body: &str) -> PathBuf {
    let openssl_dir = dir.join("openssl");
    std::fs::create_dir_all(&openssl_dir).unwrap();
    let base_h = openssl_dir.join("base.h");
    std::fs::write(&base_h, body).unwrap();
    base_h
}

#[test]
fn test_validate_and_extract_version_valid() {
    let temp_dir = tempfile::tempdir().unwrap();
    write_base_h(
        temp_dir.path(),
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"1.35.0\"\n",
    );
    assert_eq!(
        validate_and_extract_version(temp_dir.path()).unwrap(),
        "1.35.0"
    );
}

#[test]
fn test_validate_and_extract_version_not_awslc() {
    let temp_dir = tempfile::tempdir().unwrap();
    write_base_h(temp_dir.path(), "// Not AWS-LC\n");
    let err = validate_and_extract_version(temp_dir.path()).unwrap_err();
    assert!(err.contains("not valid AWS-LC headers"), "{err}");
}

#[test]
fn test_validate_and_extract_version_ignores_comment_false_match() {
    // A comment that mentions the macro name shouldn't false-match.
    let temp_dir = tempfile::tempdir().unwrap();
    let mut body = String::from("#define OPENSSL_IS_AWSLC 1\n");
    body.push_str("// AWSLC_VERSION_NUMBER_STRING is defined below\n");
    body.push_str("#define AWSLC_VERSION_NUMBER_STRING \"2.0.0\"\n");
    write_base_h(temp_dir.path(), &body);
    assert_eq!(
        validate_and_extract_version(temp_dir.path()).unwrap(),
        "2.0.0"
    );
}

#[test]
fn test_validate_and_extract_version_missing_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    assert!(validate_and_extract_version(temp_dir.path()).is_err());
}

#[test]
fn test_validate_and_extract_version_missing_version_string() {
    let temp_dir = tempfile::tempdir().unwrap();
    write_base_h(temp_dir.path(), "#define OPENSSL_IS_AWSLC 1\n");
    let err = validate_and_extract_version(temp_dir.path()).unwrap_err();
    assert!(err.contains("Could not find AWSLC_VERSION_NUMBER_STRING"));
}

// -------------------------------------------------------------------------
// FIPS module version resolution
// -------------------------------------------------------------------------

#[test]
fn test_extract_fips_version_number_present() {
    let content = "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_FIPS_VERSION_NUMBER 4\n";
    assert_eq!(extract_fips_version_number(content).unwrap(), Some(4));
}

#[test]
fn test_extract_fips_version_number_absent() {
    let content = "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"3.3.0\"\n";
    assert_eq!(extract_fips_version_number(content).unwrap(), None);
}

#[test]
fn test_extract_fips_version_number_ignores_comment_false_match() {
    // A comment mentioning the macro shouldn't false-match.
    let content =
        "// AWSLC_FIPS_VERSION_NUMBER is defined below\n#define AWSLC_FIPS_VERSION_NUMBER 7\n";
    assert_eq!(extract_fips_version_number(content).unwrap(), Some(7));
}

#[test]
fn test_extract_fips_version_number_present_but_malformed_is_error() {
    // A present-but-unparseable macro must not silently fall back to the
    // (post-decoupling unreliable) library version.
    let content = "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_FIPS_VERSION_NUMBER notanumber\n";
    let err = extract_fips_version_number(content).unwrap_err();
    assert!(err.contains("Malformed AWSLC_FIPS_VERSION_NUMBER"), "{err}");
}

#[test]
fn test_version_major() {
    assert_eq!(version_major("3.3.0").unwrap(), 3);
    assert_eq!(version_major("12.1.4").unwrap(), 12);
    assert!(version_major("").is_err());
    assert!(version_major("x.y.z").is_err());
}

#[test]
fn test_resolve_fips_version_prefers_macro() {
    // The macro is authoritative and wins over the legacy version string.
    let content =
        "#define AWSLC_VERSION_NUMBER_STRING \"3.3.0\"\n#define AWSLC_FIPS_VERSION_NUMBER 4\n";
    assert_eq!(
        resolve_fips_version(Path::new("base.h"), content).unwrap(),
        4
    );
}

#[test]
fn test_resolve_fips_version_legacy_fallback() {
    // No macro (legacy FIPS branch <= 3.x): infer from the version major.
    let content = "#define AWSLC_VERSION_NUMBER_STRING \"3.3.0\"\n";
    assert_eq!(
        resolve_fips_version(Path::new("base.h"), content).unwrap(),
        3
    );
}

#[test]
fn test_validate_and_resolve_fips_version_requires_awslc_marker() {
    // Missing OPENSSL_IS_AWSLC marker is rejected even if a FIPS macro exists.
    let temp_dir = tempfile::tempdir().unwrap();
    write_base_h(temp_dir.path(), "#define AWSLC_FIPS_VERSION_NUMBER 4\n");
    let err = validate_and_resolve_fips_version(temp_dir.path()).unwrap_err();
    assert!(err.contains("not valid AWS-LC headers"), "{err}");
}

#[test]
fn test_validate_and_resolve_fips_version_macro() {
    let temp_dir = tempfile::tempdir().unwrap();
    write_base_h(
        temp_dir.path(),
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_FIPS_VERSION_NUMBER 4\n",
    );
    assert_eq!(
        validate_and_resolve_fips_version(temp_dir.path()).unwrap(),
        4
    );
}

#[test]
fn test_validate_and_resolve_fips_version_legacy() {
    let temp_dir = tempfile::tempdir().unwrap();
    write_base_h(
        temp_dir.path(),
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"3.3.0\"\n",
    );
    assert_eq!(
        validate_and_resolve_fips_version(temp_dir.path()).unwrap(),
        3
    );
}

// -------------------------------------------------------------------------
// Declared minimums must not exceed what we vendor
// -------------------------------------------------------------------------
//
// Guards against declaring a minimum newer than the bundled submodule. Skips
// when the submodule is absent; submodule-initialized CI enforces it.

/// Path to a sibling sys crate's bundled `base.h`, relative to `builder-test`.
fn bundled_base_h(sys_crate: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("builder-test should have a parent directory")
        .join(sys_crate)
        .join("aws-lc")
        .join("include")
        .join("openssl")
        .join("base.h")
}

#[test]
fn test_minimum_aws_lc_version_not_newer_than_bundled() {
    let base_h = bundled_base_h("aws-lc-sys");
    let Ok(content) = std::fs::read_to_string(&base_h) else {
        eprintln!(
            "skipping: bundled base.h not found at {} (submodule not initialized)",
            base_h.display()
        );
        return;
    };
    let bundled = extract_version(&base_h, &content).unwrap();
    assert!(
        version_at_least(&bundled, MINIMUM_AWS_LC_VERSION).unwrap(),
        "MINIMUM_AWS_LC_VERSION ({MINIMUM_AWS_LC_VERSION}) must not exceed the bundled \
         aws-lc-sys version ({bundled})",
    );
}

#[test]
fn test_minimum_fips_version_not_newer_than_bundled() {
    let base_h = bundled_base_h("aws-lc-fips-sys");
    let Ok(content) = std::fs::read_to_string(&base_h) else {
        eprintln!(
            "skipping: bundled FIPS base.h not found at {} (submodule not initialized)",
            base_h.display()
        );
        return;
    };
    let bundled = resolve_fips_version(&base_h, &content).unwrap();
    assert!(
        bundled >= MINIMUM_FIPS_VERSION,
        "MINIMUM_FIPS_VERSION ({MINIMUM_FIPS_VERSION}) must not exceed the bundled \
         aws-lc-fips-sys FIPS version ({bundled})",
    );
}

// -------------------------------------------------------------------------
// resolve_library — no explicit preference
// -------------------------------------------------------------------------

#[test]
#[cfg(not(all(target_os = "windows", target_env = "msvc")))]
fn test_resolve_library_no_explicit_pref() {
    let _env = setup_test_env();

    // Vanilla `crypto`.
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libcrypto.a"), b"").unwrap();
    let resolved = resolve_library(temp.path(), None, "crypto", CRYPTO_LIB_CANDIDATES).unwrap();
    assert!(matches!(resolved.lib_type, OutputLibType::Static));
    assert_eq!(resolved.name, "crypto");
    assert_eq!(resolved.path, temp.path().join("libcrypto.a"));

    // ENABLE_DIST_PKG-style `crypto-awslc` is recognized.
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libcrypto-awslc.a"), b"").unwrap();
    let resolved = resolve_library(temp.path(), None, "crypto", CRYPTO_LIB_CANDIDATES).unwrap();
    assert_eq!(resolved.name, "crypto-awslc");

    // Plain `crypto` wins over `crypto-awslc` when both are present.
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libcrypto.a"), b"").unwrap();
    std::fs::write(temp.path().join("libcrypto-awslc.a"), b"").unwrap();
    let resolved = resolve_library(temp.path(), None, "crypto", CRYPTO_LIB_CANDIDATES).unwrap();
    assert_eq!(resolved.name, "crypto");

    // A symbol-prefixed filename (`libfoo_crypto.a`) is NOT a match — we
    // resolve the file by base name, not by symbol prefix.
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libfoo_crypto.a"), b"").unwrap();
    assert!(resolve_library(temp.path(), None, "crypto", CRYPTO_LIB_CANDIDATES).is_err());

    // Empty directory => not found.
    let temp = tempfile::tempdir().unwrap();
    let Err(err) = resolve_library(temp.path(), None, "crypto", CRYPTO_LIB_CANDIDATES) else {
        panic!("expected Err for empty lib dir");
    };
    assert!(err.contains("No crypto library found"), "{err}");
}

// -------------------------------------------------------------------------
// resolve_library — explicit AWS_LC_SYS_STATIC=0/1 preference
// -------------------------------------------------------------------------

#[test]
#[cfg(not(all(target_os = "windows", target_env = "msvc")))]
fn test_resolve_library_explicit_pref_satisfied() {
    let _env = setup_test_env();
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libcrypto.a"), b"").unwrap();
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "tvos")))]
    std::fs::write(temp.path().join("libcrypto.so"), b"").unwrap();
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
    std::fs::write(temp.path().join("libcrypto.dylib"), b"").unwrap();

    let resolved = resolve_library(
        temp.path(),
        Some(OutputLibType::Static),
        "crypto",
        CRYPTO_LIB_CANDIDATES,
    )
    .unwrap();
    assert!(matches!(resolved.lib_type, OutputLibType::Static));

    let resolved = resolve_library(
        temp.path(),
        Some(OutputLibType::Dynamic),
        "crypto",
        CRYPTO_LIB_CANDIDATES,
    )
    .unwrap();
    assert!(matches!(resolved.lib_type, OutputLibType::Dynamic));
}

#[test]
#[cfg(not(all(target_os = "windows", target_env = "msvc")))]
fn test_resolve_library_explicit_pref_unsatisfied_is_hard_error() {
    let _env = setup_test_env();

    // Static requested, only dynamic present.
    let temp = tempfile::tempdir().unwrap();
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "tvos")))]
    std::fs::write(temp.path().join("libcrypto.so"), b"").unwrap();
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
    std::fs::write(temp.path().join("libcrypto.dylib"), b"").unwrap();
    let Err(err) = resolve_library(
        temp.path(),
        Some(OutputLibType::Static),
        "crypto",
        CRYPTO_LIB_CANDIDATES,
    ) else {
        panic!("expected Err: static requested, only dynamic present");
    };
    assert!(
        err.contains("only a dynamic crypto library was found"),
        "{err}"
    );

    // Dynamic requested, only static present.
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libcrypto.a"), b"").unwrap();
    let Err(err) = resolve_library(
        temp.path(),
        Some(OutputLibType::Dynamic),
        "crypto",
        CRYPTO_LIB_CANDIDATES,
    ) else {
        panic!("expected Err: dynamic requested, only static present");
    };
    assert!(
        err.contains("only a static crypto library was found"),
        "{err}"
    );
}

#[test]
#[cfg(not(all(target_os = "windows", target_env = "msvc")))]
fn test_resolve_library_explicit_pref_neither_form_present() {
    // When the user explicitly requests a form but the directory contains
    // *neither* form, the error must say "no library found" — not
    // "only a {got} library was found", which would be a lie.
    let _env = setup_test_env();
    let temp = tempfile::tempdir().unwrap();
    let Err(err) = resolve_library(
        temp.path(),
        Some(OutputLibType::Static),
        "crypto",
        CRYPTO_LIB_CANDIDATES,
    ) else {
        panic!("expected Err: empty dir");
    };
    assert!(
        err.contains("no crypto library was found"),
        "expected 'no crypto library was found' phrasing, got: {err}"
    );
    assert!(
        !err.contains("only a"),
        "must not claim a library was found when none is present: {err}"
    );
}

#[test]
#[cfg(not(all(target_os = "windows", target_env = "msvc")))]
fn test_resolve_library_explicit_static_satisfied_by_awslc_suffix() {
    // AWS_LC_SYS_STATIC=1 with only `libcrypto-awslc.a` present should still
    // succeed via the candidate fallback within the same lib_type.
    let _env = setup_test_env();
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libcrypto-awslc.a"), b"").unwrap();

    let resolved = resolve_library(
        temp.path(),
        Some(OutputLibType::Static),
        "crypto",
        CRYPTO_LIB_CANDIDATES,
    )
    .unwrap();
    assert!(matches!(resolved.lib_type, OutputLibType::Static));
    assert_eq!(resolved.name, "crypto-awslc");
}

// -------------------------------------------------------------------------
// resolve_library — SSL candidates
// -------------------------------------------------------------------------

#[test]
#[cfg(not(all(target_os = "windows", target_env = "msvc")))]
fn test_resolve_library_ssl_candidates() {
    let _env = setup_test_env();

    // Vanilla `ssl`.
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libssl.a"), b"").unwrap();
    let resolved = resolve_library(temp.path(), None, "ssl", SSL_LIB_CANDIDATES).unwrap();
    assert!(matches!(resolved.lib_type, OutputLibType::Static));
    assert_eq!(resolved.name, "ssl");

    // Suffixed `ssl-awslc` is recognized.
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libssl-awslc.a"), b"").unwrap();
    let resolved = resolve_library(temp.path(), None, "ssl", SSL_LIB_CANDIDATES).unwrap();
    assert_eq!(resolved.name, "ssl-awslc");
}

// -------------------------------------------------------------------------
// probe_lib — Unix
// -------------------------------------------------------------------------

#[test]
#[cfg(not(all(target_os = "windows", target_env = "msvc")))]
fn test_probe_lib_unix() {
    let _env = setup_test_env();

    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("libcrypto.a"), b"").unwrap();
    assert!(probe_lib(temp.path(), "crypto", OutputLibType::Static).is_some());

    #[cfg(target_os = "linux")]
    {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join("libcrypto.so"), b"").unwrap();
        assert!(probe_lib(temp.path(), "crypto", OutputLibType::Dynamic).is_some());
    }
}

// -------------------------------------------------------------------------
// MSVC import-library vs static-archive disambiguation
// -------------------------------------------------------------------------

#[test]
fn test_msvc_static_archive_without_sibling_dll() {
    let _env = setup_test_env_with_target("windows", "msvc");
    let fx = FakeInstall::new().touch_lib("crypto.lib");
    let lib_dir = fx.lib_dir();

    assert_eq!(
        probe_lib(&lib_dir, "crypto", OutputLibType::Static),
        Some(lib_dir.join("crypto.lib")),
    );
    assert_eq!(probe_lib(&lib_dir, "crypto", OutputLibType::Dynamic), None);
}

#[test]
fn test_msvc_import_library_with_sibling_dll() {
    let _env = setup_test_env_with_target("windows", "msvc");
    let fx = FakeInstall::new()
        .touch_lib("crypto.lib")
        .touch_bin("crypto.dll");
    let lib_dir = fx.lib_dir();

    assert_eq!(probe_lib(&lib_dir, "crypto", OutputLibType::Static), None);
    assert_eq!(
        probe_lib(&lib_dir, "crypto", OutputLibType::Dynamic),
        Some(lib_dir.join("crypto.lib")),
    );
}

#[test]
fn test_msvc_resolve_shared_install_honors_dynamic_preference() {
    let _env = setup_test_env_with_target("windows", "msvc");
    let fx = FakeInstall::new()
        .touch_lib("crypto.lib")
        .touch_bin("crypto.dll");
    let lib_dir = fx.lib_dir();

    let resolved = resolve_library(&lib_dir, None, "crypto", CRYPTO_LIB_CANDIDATES).unwrap();
    assert!(matches!(resolved.lib_type, OutputLibType::Dynamic));
    assert_eq!(resolved.name, "crypto");
    assert_eq!(resolved.path, lib_dir.join("crypto.lib"));
}

// -------------------------------------------------------------------------
// check_dependencies — lib dir resolution (lib64 vs lib)
// -------------------------------------------------------------------------

#[test]
fn test_check_dependencies_prefers_lib64_on_64bit() {
    // setup_test_env forces CARGO_CFG_TARGET_POINTER_WIDTH=64, so the
    // resolution must pick lib64 regardless of the test binary's actual
    // host pointer width. Don't switch on `cfg!(target_pointer_width)` here
    // — that's the host build target, which can disagree with the env var
    // on a (rare) 32-bit host and would spuriously fail.
    let _env = setup_test_env();
    let fx = FakeInstall::new()
        .mkdir("lib")
        .mkdir("lib64")
        .mkdir("include/openssl");

    // Place libcrypto.a (and libssl.a, for the `ssl` feature) in both lib/
    // and lib64/ so resolution succeeds in either location.
    std::fs::write(fx.root.join("lib").join("libcrypto.a"), b"").unwrap();
    std::fs::write(fx.root.join("lib64").join("libcrypto.a"), b"").unwrap();
    std::fs::write(fx.root.join("lib").join("libssl.a"), b"").unwrap();
    std::fs::write(fx.root.join("lib64").join("libssl.a"), b"").unwrap();

    let sys = SystemLib::new(
        PathBuf::from("."),
        fx.root.clone(),
        None,
        true, // skip version check
    );
    sys.check_dependencies().unwrap();

    let crypto = sys.crypto_lib.borrow();
    let crypto = crypto.as_ref().unwrap();
    assert!(
        crypto.path.to_str().unwrap().contains("lib64"),
        "Expected lib64 path, got: {}",
        crypto.path.display()
    );
}

#[test]
fn test_check_dependencies_missing_lib_dir() {
    let _env = setup_test_env();
    let fx = FakeInstall::new(); // no lib dir created

    let sys = SystemLib::new(PathBuf::from("."), fx.root.clone(), None, true);
    let err = sys.check_dependencies().unwrap_err();
    // The error is now propagated from `resolve_library` rather than the
    // generic "AWS-LC libcrypto not found" fallback, so it points at the
    // specific lib_dir that was probed and lists the expected filenames.
    assert!(
        err.contains("No crypto library found"),
        "expected propagated resolver error, got: {err}"
    );
    assert!(
        err.contains("Expected one of:"),
        "expected list of probed filenames, got: {err}"
    );
}

// -------------------------------------------------------------------------
// resolve_bindings
// -------------------------------------------------------------------------

#[test]
fn test_resolve_bindings_conventional() {
    let temp = tempfile::tempdir().unwrap();
    let conventional = temp.path().join("share").join("rust");
    std::fs::create_dir_all(&conventional).unwrap();
    let bindings = conventional.join("aws_lc_bindings.rs");
    std::fs::write(&bindings, b"// bindings").unwrap();

    let _env = setup_test_env();
    assert_eq!(
        resolve_bindings(Some(temp.path()), &None).unwrap(),
        bindings
    );
}

#[test]
fn test_resolve_bindings_override_takes_priority() {
    let temp = tempfile::tempdir().unwrap();
    // Conventional bindings exist…
    let conventional = temp.path().join("share").join("rust");
    std::fs::create_dir_all(&conventional).unwrap();
    std::fs::write(conventional.join("aws_lc_bindings.rs"), b"// conventional").unwrap();
    // …but the override should win.
    let override_path = temp.path().join("custom-bindings.rs");
    std::fs::write(&override_path, b"// override").unwrap();

    let _env = setup_test_env();
    assert_eq!(
        resolve_bindings(Some(temp.path()), &Some(override_path.clone())).unwrap(),
        override_path
    );
}

#[test]
fn test_resolve_bindings_override_missing_is_hard_error() {
    let temp = tempfile::tempdir().unwrap();
    let bogus = temp.path().join("does-not-exist.rs");

    let _env = setup_test_env();
    let err = resolve_bindings(Some(temp.path()), &Some(bogus)).unwrap_err();
    assert!(err.contains("does not point to a file"), "{err}");
}

#[test]
fn test_resolve_bindings_missing_returns_helpful_error() {
    let temp = tempfile::tempdir().unwrap();
    let _env = setup_test_env();
    let err = resolve_bindings(Some(temp.path()), &None).unwrap_err();
    assert!(err.contains("No pre-generated bindings found"), "{err}");
}

#[test]
fn test_resolve_bindings_no_prefix_no_override_is_error() {
    let _env = setup_test_env();
    // Discovery couldn't determine a prefix and no override was supplied.
    let err = resolve_bindings(None, &None).unwrap_err();
    assert!(err.contains("No pre-generated bindings found"), "{err}");
}

#[test]
fn test_resolve_bindings_no_prefix_uses_override() {
    let temp = tempfile::tempdir().unwrap();
    let override_path = temp.path().join("custom-bindings.rs");
    std::fs::write(&override_path, b"// override").unwrap();

    let _env = setup_test_env();
    // With no prefix, an explicit override still resolves.
    assert_eq!(
        resolve_bindings(None, &Some(override_path.clone())).unwrap(),
        override_path
    );
}

// -------------------------------------------------------------------------
// SystemLib::probe — non-fatal validation for auto-detection
// -------------------------------------------------------------------------
//
// `probe` runs the same checks as the fatal `link` path but returns `Err`
// instead of erroring the build, so auto-detection can fall back to a source
// build. These tests are gated to non-FIPS builds because the FIPS path inside
// `validate` compiles and runs a probe binary against a real FIPS module,
// which a fake install can't satisfy.

/// Builds a `FakeInstall` that looks like a minimal, valid AWS-LC: a static
/// libcrypto, an `include/` directory, and conventional pre-generated bindings
/// under `share/rust/`.
#[cfg(not(feature = "fips"))]
fn with_required_ssl_libs(fx: FakeInstall) -> FakeInstall {
    #[cfg(feature = "ssl")]
    {
        fx.touch_lib("libssl.a")
    }
    #[cfg(not(feature = "ssl"))]
    {
        fx
    }
}

#[cfg(not(feature = "fips"))]
fn fake_valid_install() -> FakeInstall {
    let fx = with_required_ssl_libs(
        FakeInstall::new()
            .mkdir("include/openssl")
            .mkdir("share/rust")
            .touch_lib("libcrypto.a"),
    );
    write_base_h(
        &fx.root.join("include"),
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"99.0.0\"\n",
    );
    std::fs::write(
        fx.root
            .join("share")
            .join("rust")
            .join("aws_lc_bindings.rs"),
        b"// bindings",
    )
    .unwrap();
    fx
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_probe_succeeds_on_valid_install() {
    let _env = setup_test_env();
    let fx = fake_valid_install();

    // skip_version_check=true keeps this independent of the bundled submodule
    // headers; the marker/version path is exercised by the tests below and by
    // the `validate_and_extract_version` tests.
    let sys = SystemLib::new(PathBuf::from("."), fx.root.clone(), None, true);
    sys.probe()
        .expect("valid install should probe successfully");

    // `probe` memoizes its result; a second call must still succeed.
    sys.probe().expect("probe should be idempotent");
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_probe_falls_back_when_bindings_missing() {
    let _env = setup_test_env();
    // Valid library + headers, but no pre-generated bindings present.
    let fx = with_required_ssl_libs(
        FakeInstall::new()
            .mkdir("include/openssl")
            .touch_lib("libcrypto.a"),
    );
    write_base_h(
        &fx.root.join("include"),
        "#define OPENSSL_IS_AWSLC 1\n#define AWSLC_VERSION_NUMBER_STRING \"99.0.0\"\n",
    );

    let sys = SystemLib::new(PathBuf::from("."), fx.root.clone(), None, true);
    // Must return Err (so auto-detection can fall back), never panic.
    let err = sys.probe().unwrap_err();
    assert!(err.contains("No pre-generated bindings found"), "{err}");
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_probe_falls_back_when_not_awslc_headers() {
    let _env = setup_test_env();
    let fx = fake_valid_install();
    // Overwrite the headers so the AWS-LC marker is absent (e.g. OpenSSL).
    write_base_h(&fx.root.join("include"), "// Not AWS-LC\n");

    // Version checking ON so the marker check runs. The marker check happens
    // before the minimum-version comparison, so this stays independent of the
    // bundled submodule.
    let sys = SystemLib::new(PathBuf::from("."), fx.root.clone(), None, false);
    let err = sys.probe().unwrap_err();
    assert!(err.contains("not valid AWS-LC headers"), "{err}");
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_probe_falls_back_when_skip_version_check_but_not_awslc_headers() {
    let _env = setup_test_env();
    let fx = fake_valid_install();
    write_base_h(&fx.root.join("include"), "// Not AWS-LC\n");

    // SYSTEM_SKIP_VERSION_CHECK skips the version floor, not the AWS-LC marker.
    let sys = SystemLib::new(PathBuf::from("."), fx.root.clone(), None, true);
    let err = sys.probe().unwrap_err();
    assert!(err.contains("not valid AWS-LC headers"), "{err}");
}

#[test]
#[cfg(not(feature = "fips"))]
fn test_probe_falls_back_when_libs_missing() {
    let _env = setup_test_env();
    let fx = FakeInstall::new().mkdir("include/openssl"); // headers but no libs

    let sys = SystemLib::new(PathBuf::from("."), fx.root.clone(), None, true);
    let err = sys.probe().unwrap_err();
    assert!(err.contains("No crypto library found"), "{err}");
}

// -------------------------------------------------------------------------
// FIPS verification (fixture-based integration tests)
//
// Skipped unless the matching env vars point at install prefixes containing
// `include/` and `lib/`.
// -------------------------------------------------------------------------

/// Env-var name pointing at a real FIPS AWS-LC install (positive fixture).
const FIXTURE_FIPS_ENV: &str = "AWS_LC_FIPS_SYS_FIXTURE_FIPS";
/// Env-var name pointing at a real non-FIPS AWS-LC install (negative fixture).
const FIXTURE_NONFIPS_ENV: &str = "AWS_LC_FIPS_SYS_FIXTURE_NONFIPS";

/// Cargo env vars the FIPS verifier reads, with HOST == TARGET so the runtime
/// probe path runs. `_temp_out_dir` keeps OUT_DIR alive; `_env` restores the
/// env vars and releases the global mutex on drop.
struct FixtureEnvGuard<'a> {
    _env: TestEnvGuard<'a>,
    _temp_out_dir: tempfile::TempDir,
}

fn repo_manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("builder-test lives under the repo root")
}

fn setup_fixture_env() -> FixtureEnvGuard<'static> {
    // Acquire the env mutex first; layered EnvGuards below take effect under it.
    let lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let arch = std::env::consts::ARCH;
    let host_triple = format!("{arch}-fixture-host");
    let temp_out = tempfile::tempdir().unwrap();

    let vars: Vec<(&str, String)> = vec![
        ("CARGO_CFG_TARGET_OS", std::env::consts::OS.to_string()),
        ("CARGO_CFG_TARGET_ENV", String::new()),
        ("CARGO_CFG_TARGET_FEATURE", String::new()),
        ("CARGO_CFG_TARGET_ARCH", arch.to_string()),
        ("CARGO_CFG_TARGET_POINTER_WIDTH", "64".to_string()),
        ("HOST", host_triple.clone()),
        ("TARGET", host_triple),
        ("CARGO_PKG_NAME", "aws-lc-fips-sys".to_string()),
        ("OUT_DIR", temp_out.path().to_string_lossy().into_owned()),
        // cc::Build::get_compiler() requires these to derive flags.
        ("OPT_LEVEL", "0".to_string()),
        ("DEBUG", "true".to_string()),
    ];
    let guards: Vec<EnvGuard> = vars
        .iter()
        .map(|(key, val)| EnvGuard::new(key, val.as_str()))
        .collect();

    FixtureEnvGuard {
        _env: TestEnvGuard {
            _guards: guards,
            _lock: lock,
        },
        _temp_out_dir: temp_out,
    }
}

/// Resolves `crypto` for an install rooted at `install_dir`, mirroring what
/// `SystemLib::check_dependencies` does so the FIPS verifier can be invoked
/// directly without the full builder setup.
fn resolve_crypto_for_fixture(install_dir: &Path) -> ResolvedLib {
    for sub in ["lib64", "lib"] {
        let lib_dir = install_dir.join(sub);
        if !lib_dir.is_dir() {
            continue;
        }
        if let Ok(lib) = resolve_library(&lib_dir, None, "crypto", CRYPTO_LIB_CANDIDATES) {
            return lib;
        }
    }
    panic!(
        "fixture {} does not contain a resolvable libcrypto",
        install_dir.display()
    );
}

#[test]
fn test_verify_fips_library_accepts_fips_install() {
    let Ok(install) = std::env::var(FIXTURE_FIPS_ENV) else {
        eprintln!("skip: {FIXTURE_FIPS_ENV} not set");
        return;
    };
    let install_dir = PathBuf::from(install);
    assert!(
        install_dir.is_dir(),
        "{FIXTURE_FIPS_ENV}={} is not a directory",
        install_dir.display()
    );

    let _env = setup_fixture_env();
    let crypto_lib = resolve_crypto_for_fixture(&install_dir);
    let lib_dir = crypto_lib
        .path
        .parent()
        .expect("resolved libcrypto has no parent")
        .to_path_buf();
    let include_dir = install_dir.join("include");

    verify_fips_install(repo_manifest_dir(), &include_dir, &crypto_lib, &lib_dir).expect(
        "verify_fips_library should accept a real FIPS install (build-time + runtime probes)",
    );
}

#[test]
fn test_verify_fips_library_rejects_nonfips_install() {
    let Ok(install) = std::env::var(FIXTURE_NONFIPS_ENV) else {
        eprintln!("skip: {FIXTURE_NONFIPS_ENV} not set");
        return;
    };
    let install_dir = PathBuf::from(install);
    assert!(
        install_dir.is_dir(),
        "{FIXTURE_NONFIPS_ENV}={} is not a directory",
        install_dir.display()
    );

    let _env = setup_fixture_env();
    let crypto_lib = resolve_crypto_for_fixture(&install_dir);
    let lib_dir = crypto_lib
        .path
        .parent()
        .expect("resolved libcrypto has no parent")
        .to_path_buf();
    let include_dir = install_dir.join("include");

    let err = verify_fips_install(repo_manifest_dir(), &include_dir, &crypto_lib, &lib_dir)
        .expect_err("verify_fips_library must reject a non-FIPS install");
    assert!(
        err.contains("FIPS verification failed"),
        "unexpected error message: {err}"
    );
    // Assert the failure is specifically the FIPS link-probe rejection, not an
    // earlier stage (missing probe source, unusable compiler) or the runtime
    // FIPS_mode() check. This is the unforgeable part of the check: a non-FIPS
    // libcrypto does not export BORINGSSL_integrity_test, so the probe cannot
    // link against it.
    assert!(
        err.contains("linker rejected the probe against"),
        "error should be the FIPS link-probe rejection: {err}"
    );
}
