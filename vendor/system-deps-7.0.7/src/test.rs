use itertools::Itertools;
use std::cell::Cell;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Mutex;

use assert_matches::assert_matches;

use crate::Dependencies;

use super::{
    BuildFlags, BuildInternalClosureError, Config, EnvVariables, Error, InternalLib, Library,
};

lazy_static! {
    static ref LOCK: Mutex<()> = Mutex::new(());
}

fn create_config(path: &str, env: Vec<(&'static str, &'static str)>) -> Config {
    {
        // PKG_CONFIG_PATH is read by pkg-config, so we need to actually change the env
        let _l = LOCK.lock();
        env::set_var(
            "PKG_CONFIG_PATH",
            env::current_dir().unwrap().join("src").join("tests"),
        );
    }

    let mut hash = HashMap::new();
    hash.insert(
        "CARGO_MANIFEST_DIR",
        env::current_dir()
            .unwrap()
            .join("src")
            .join("tests")
            .join(path)
            .to_string_lossy()
            .to_string(),
    );

    hash.insert("CARGO_FEATURE_TEST_FEATURE", "".to_string());
    env.iter().for_each(|(k, v)| {
        hash.insert(k, v.to_string());
    });

    Config::new_with_env(EnvVariables::Mock(hash))
}

fn toml(
    path: &str,
    env: Vec<(&'static str, &'static str)>,
) -> Result<(Dependencies, BuildFlags), Error> {
    let libs = create_config(path, env).probe_full()?;
    let flags = libs.gen_flags()?;
    Ok((libs, flags))
}

fn assert_flags(flags: BuildFlags, expected: &str) {
    // flags ordering isn't guaranteed so sort them out before comparing
    let flags = flags.to_string().split('\n').sorted().join("\n");
    let expected = expected.split('\n').sorted().join("\n");
    assert_eq!(flags, expected);
}

#[test]
fn good() {
    let (libraries, flags) = toml("toml-good", vec![]).unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.version, "1.2.3");
    assert_eq!(
        testlib.defines.get("BADGER").unwrap().as_deref(),
        Some("yes")
    );
    assert!(testlib.defines.get("AWESOME").unwrap().is_none());

    let testdata = libraries.get_by_name("testdata").unwrap();
    assert_eq!(testdata.version, "4.5.6");
    assert!(libraries.get_by_name("testmore").is_none());

    assert_eq!(libraries.iter().len(), 2);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
"#,
    );
}

#[test]
fn version_range() {
    let (libraries, _flags) = toml("toml-version-range", vec![]).unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.version, "1.2.3");
    assert_eq!(
        testlib.defines.get("BADGER").unwrap().as_deref(),
        Some("yes")
    );
    assert!(testlib.defines.get("AWESOME").unwrap().is_none());

    let testdata = libraries.get_by_name("testdata").unwrap();
    assert_eq!(testdata.version, "4.5.6");

    assert_eq!(libraries.iter().len(), 2);
}

#[test]
#[ignore]
fn version_range_unsatisfied() {
    let err = toml_err("toml-version-range-unsatisfied");

    assert_matches!(err, Error::PkgConfig(_));

    let err_msg = err.to_string();
    // pkgconf and pkg-config give different error messages
    if !err_msg.contains("Package 'testlib' has version '1.2.3', required version is '< 1.2'")
        && !err_msg.contains("Requested 'testlib < 1.2' but version of Test Library is 1.2.3")
    {
        panic!("Error not as expected: {:?}", err);
    }
}

fn toml_err(path: &str) -> Error {
    toml(path, vec![]).unwrap_err()
}

fn toml_err_invalid(path: &str, err_ends_with: &str) {
    let err = toml_err(path);
    assert_matches!(err, Error::InvalidMetadata(_));

    if !err.to_string().ends_with(err_ends_with) {
        panic!(
            "Expected error to end with: {:?}\nGot error: {:?}",
            err_ends_with, err
        );
    }
}

// Assert a PkgConfig error because requested lib version cannot be found
fn toml_pkg_config_err_version(
    path: &str,
    expected_version: &str,
    env_vars: Vec<(&'static str, &'static str)>,
) {
    let err = toml(path, env_vars).unwrap_err();
    match err {
        Error::PkgConfig(e) => match e {
            pkg_config::Error::ProbeFailure {
                command: cmd,
                output: _,
                name: _,
            } => {
                let s = format!(">= {expected_version}");
                // remove trailing " and ', if any
                let cmd = cmd.strip_suffix('"').unwrap_or(&cmd);
                let cmd = cmd.strip_suffix('\'').unwrap_or(cmd);
                assert!(cmd.ends_with(&s));
            }
            _ => panic!("Wrong pkg-config error type"),
        },
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn missing_file() {
    assert_matches!(toml_err("toml-missing-file"), Error::FailToRead(_, _));
}

#[test]
fn missing_key() {
    toml_err_invalid(
        "toml-missing-key",
        "missing key `package.metadata.system-deps`",
    );
}

#[test]
fn not_table() {
    toml_err_invalid(
        "toml-not-table",
        "`package.metadata.system-deps` is not a table",
    );
}

#[test]
fn version_missing() {
    toml_err_invalid("toml-version-missing", "No version defined for testlib");
}

#[test]
fn version_not_string() {
    toml_err_invalid(
        "toml-version-not-string",
        "`package.metadata.system-deps.testlib`: not a string or a table",
    );
}

#[test]
fn version_in_table_not_string() {
    toml_err_invalid(
        "toml-version-in-table-not-string",
        "metadata.system-deps.testlib: unexpected key version type integer",
    );
}

#[test]
fn feature_not_string() {
    toml_err_invalid(
        "toml-feature-not-string",
        "metadata.system-deps.testlib: unexpected key feature type integer",
    );
}

#[test]
fn unexpected_key() {
    toml_err_invalid(
        "toml-unexpected-key",
        "metadata.system-deps.testlib: unexpected key color type string",
    );
}

#[test]
fn override_name() {
    let (libraries, _) = toml("toml-override-name", vec![]).unwrap();
    let testlib = libraries.get_by_name("test_lib").unwrap();
    assert_eq!(testlib.name, "testlib");
    assert_eq!(testlib.version, "1.2.3");

    // Enable feature 1.2
    let (libraries, _) = toml("toml-override-name", vec![("CARGO_FEATURE_V1_2", "")]).unwrap();
    let testlib = libraries.get_by_name("test_lib").unwrap();
    assert_eq!(testlib.name, "testlib");
    assert_eq!(testlib.version, "1.2.3");
}

#[test]
fn fallback_names() {
    let (libraries, flags) = toml("toml-fallback-names", vec![]).unwrap();
    let testlib = libraries.get_by_name("test_lib").unwrap();
    assert_eq!(testlib.name, "testlib");
    assert_eq!(testlib.version, "1.2.3");

    eprintln!();
    eprintln!("{flags}");
    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LINK
"#,
    );
}

#[test]
fn version_fallback_names() {
    let (libraries, flags) = toml("toml-version-fallback-names", vec![]).unwrap();
    let testlib = libraries.get_by_name("test_lib").unwrap();
    assert_eq!(testlib.name, "testlib");
    assert_eq!(testlib.version, "1.2.3");
    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LINK
"#,
    );

    // Now try with v2 feature enabled.
    let (libraries, flags) = toml(
        "toml-version-fallback-names",
        vec![("CARGO_FEATURE_V2", "")],
    )
    .unwrap();
    let testlib = libraries.get_by_name("test_lib").unwrap();
    assert_eq!(testlib.name, "testlib-2.0");
    assert_eq!(testlib.version, "2.0.0");

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-lib=test
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TEST_LIB_LINK
"#,
    );
}

#[test]
fn feature_versions() {
    let (libraries, _) = toml("toml-feature-versions", vec![]).unwrap();
    let testdata = libraries.get_by_name("testdata").unwrap();
    assert_eq!(testdata.name, "testdata");
    assert_eq!(testdata.version, "4.5.6");

    // version 5 is not available
    toml_pkg_config_err_version("toml-feature-versions", "5", vec![("CARGO_FEATURE_V5", "")]);

    // We check the highest version enabled by features
    toml_pkg_config_err_version("toml-feature-versions", "6", vec![("CARGO_FEATURE_V6", "")]);

    let (libraries, _) = toml("toml-version-names", vec![]).unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.name, "testlib");
    assert_eq!(testlib.version, "1.2.3");

    // Enable feature v2
    let (libraries, _) = toml("toml-version-names", vec![("CARGO_FEATURE_V2", "")]).unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.name, "testlib-2.0");
    assert_eq!(testlib.version, "2.0.0");

    // Takes the higher feature
    let (libraries, _) = toml(
        "toml-version-names",
        vec![("CARGO_FEATURE_V2", ""), ("CARGO_FEATURE_V3", "")],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.name, "testlib-3.0");
}

#[test]
fn override_search_native() {
    #[cfg(target_os = "windows")]
    let paths_env = "/custom/path;/other/path";
    #[cfg(not(target_os = "windows"))]
    let paths_env = "/custom/path:/other/path";

    let (libraries, flags) = toml(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE", paths_env)],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(
        testlib.link_paths,
        vec![Path::new("/custom/path"), Path::new("/other/path")]
    );

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/custom/path
cargo:rustc-link-search=native=/other/path
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
"#,
    );
}

#[test]
fn override_search_framework() {
    let (libraries, flags) = toml(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK", "/custom/path")],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.framework_paths, vec![Path::new("/custom/path")]);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/custom/path
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
"#,
    );
}

#[test]
fn override_lib() {
    let (libraries, flags) = toml(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_LIB", "overridden-test other-test")],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(
        testlib.libs,
        vec!["overridden-test", "other-test"]
            .into_iter()
            .map(|name| InternalLib::new(name.to_string(), false))
            .collect::<Vec<InternalLib>>()
    );

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=overridden-test
cargo:rustc-link-lib=other-test
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
"#,
    );
}

#[test]
fn override_framework() {
    let (libraries, flags) = toml(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK", "overridden-framework")],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.frameworks, vec!["overridden-framework"]);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=overridden-framework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
"#,
    );
}

#[test]
fn override_include() {
    let (libraries, flags) = toml(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_INCLUDE", "/other/include")],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.include_paths, vec![Path::new("/other/include")]);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=someframework
cargo:include=/other/include
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
"#,
    );
}

#[test]
fn override_unset() {
    let (libraries, flags) = toml(
        "toml-good",
        vec![
            ("SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE", ""),
            ("SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK", ""),
            ("SYSTEM_DEPS_TESTLIB_LIB", ""),
            ("SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK", ""),
            ("SYSTEM_DEPS_TESTLIB_INCLUDE", ""),
        ],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.link_paths, Vec::<PathBuf>::new());
    assert_eq!(testlib.framework_paths, Vec::<PathBuf>::new());
    assert_eq!(testlib.libs, Vec::<InternalLib>::new());
    assert_eq!(testlib.frameworks, Vec::<String>::new());
    assert_eq!(testlib.include_paths, Vec::<PathBuf>::new());

    assert_flags(
        flags,
        r"cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
",
    );
}

#[test]
fn override_no_pkg_config() {
    let (libraries, flags) = toml(
        "toml-good",
        vec![
            ("SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG", "1"),
            ("SYSTEM_DEPS_TESTLIB_LIB", "custom-lib"),
        ],
    )
    .unwrap();
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert_eq!(testlib.link_paths, Vec::<PathBuf>::new());
    assert_eq!(testlib.framework_paths, Vec::<PathBuf>::new());
    assert_eq!(
        testlib.libs,
        vec![InternalLib::new("custom-lib".to_string(), false)]
    );
    assert_eq!(testlib.frameworks, Vec::<String>::new());
    assert_eq!(testlib.include_paths, Vec::<PathBuf>::new());

    assert_flags(
        flags,
        r"cargo:rustc-link-lib=custom-lib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
",
    );
}

#[test]
fn override_no_pkg_config_error() {
    let err = toml(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG", "1")],
    )
    .unwrap_err();
    assert_eq!(
        err.to_string(),
        "You should define at least one lib using SYSTEM_DEPS_TESTLIB_LIB or SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK"
    );
}

fn test_build_internal(
    path: &'static str,
    env: Vec<(&'static str, &'static str)>,
    expected_lib: &'static str,
) -> Result<(Dependencies, bool), (Error, bool)> {
    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();
    let config = create_config(path, env).add_build_internal(expected_lib, move |lib, version| {
        called_clone.replace(true);
        assert_eq!(lib, expected_lib);
        let mut pkg_lib = pkg_config::Config::new()
            .print_system_libs(false)
            .cargo_metadata(false)
            .probe(lib)
            .unwrap();
        pkg_lib.version = version.to_string();
        Ok(Library::from_pkg_config(lib, pkg_lib))
    });

    match config.probe_full() {
        Ok(libraries) => Ok((libraries, called.get())),
        Err(e) => Err((e, called.get())),
    }
}

#[test]
fn build_internal_always() {
    let (libraries, called) = test_build_internal(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL", "always")],
        "testlib",
    )
    .unwrap();

    assert!(called);
    assert!(libraries.get_by_name("testlib").is_some());
}

#[test]
fn build_internal_auto_not_called() {
    // No need to build the lib as the existing version is new enough
    let (libraries, called) = test_build_internal(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL", "auto")],
        "testlib",
    )
    .unwrap();

    assert!(!called);
    assert!(libraries.get_by_name("testlib").is_some());
}

#[test]
fn build_internal_auto_called() {
    // Version 5 is not available, so we should try building
    let (libraries, called) = test_build_internal(
        "toml-feature-versions",
        vec![
            ("SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL", "auto"),
            ("CARGO_FEATURE_V5", ""),
        ],
        "testdata",
    )
    .unwrap();

    assert!(called);
    assert!(libraries.get_by_name("testdata").is_some());
}

#[test]
fn build_internal_auto_never() {
    // Version 5 is not available, but we forbid to build the lib
    let (err, called) = test_build_internal(
        "toml-feature-versions",
        vec![
            ("SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL", "never"),
            ("CARGO_FEATURE_V5", ""),
        ],
        "testdata",
    )
    .unwrap_err();

    assert!(matches!(err, Error::PkgConfig(..)));
    assert!(!called);
}

#[test]
fn build_internal_always_no_closure() {
    let config = create_config(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL", "always")],
    );

    let err = config.probe_full().unwrap_err();
    assert!(matches!(err, Error::BuildInternalNoClosure(..)));
}

#[test]
fn build_internal_invalid() {
    let config = create_config(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL", "badger")],
    );

    let err = config.probe_full().unwrap_err();
    assert!(matches!(err, Error::BuildInternalInvalid(..)));
}

#[test]
fn build_internal_wrong_version() {
    // Require version 5
    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();
    let config = create_config(
        "toml-feature-versions",
        vec![
            ("SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL", "auto"),
            ("CARGO_FEATURE_V5", ""),
        ],
    )
    .add_build_internal("testdata", move |lib, _version| {
        called_clone.replace(true);
        assert_eq!(lib, "testdata");
        let pkg_lib = pkg_config::Config::new()
            .print_system_libs(false)
            .cargo_metadata(false)
            .probe(lib)
            .unwrap();
        Ok(Library::from_pkg_config(lib, pkg_lib))
    });

    let err = config.probe_full().unwrap_err();
    assert!(matches!(err, Error::BuildInternalWrongVersion(..)));
    assert!(called.get());
}

#[test]
fn build_internal_fail() {
    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();
    let config = create_config(
        "toml-good",
        vec![("SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL", "always")],
    )
    .add_build_internal("testlib", move |lib, _version| {
        called_clone.replace(true);
        assert_eq!(lib, "testlib");
        Err(BuildInternalClosureError::failed("Something went wrong"))
    });

    let err = config.probe_full().unwrap_err();
    assert!(matches!(err, Error::BuildInternalClosureError(..)));
    assert!(called.get());
}

#[test]
fn build_internal_always_global() {
    let called = Rc::new(Cell::new((false, false)));
    let called_clone = called.clone();
    let called_clone2 = called.clone();
    let config = create_config("toml-good", vec![("SYSTEM_DEPS_BUILD_INTERNAL", "always")])
        .add_build_internal("testlib", move |lib, version| {
            let (_, b) = called_clone.get();
            called_clone.replace((true, b));
            let mut pkg_lib = pkg_config::Config::new()
                .print_system_libs(false)
                .cargo_metadata(false)
                .probe(lib)
                .unwrap();
            pkg_lib.version = version.to_string();
            Ok(Library::from_pkg_config(lib, pkg_lib))
        })
        .add_build_internal("testdata", move |lib, version| {
            let (a, _) = called_clone2.get();
            called_clone2.replace((a, true));
            let mut pkg_lib = pkg_config::Config::new()
                .print_system_libs(false)
                .cargo_metadata(false)
                .probe(lib)
                .unwrap();
            pkg_lib.version = version.to_string();
            Ok(Library::from_pkg_config(lib, pkg_lib))
        });

    let libraries = config.probe_full().unwrap();
    assert_eq!(called.get(), (true, true));
    assert!(libraries.get_by_name("testlib").is_some());
    assert!(libraries.get_by_name("testdata").is_some());
}

#[test]
fn build_internal_global_override() {
    // Request to build all libs using global var but disable it for a specific one
    let called = Rc::new(Cell::new((false, false)));
    let called_clone = called.clone();
    let called_clone2 = called.clone();
    let config = create_config(
        "toml-good",
        vec![
            ("SYSTEM_DEPS_BUILD_INTERNAL", "always"),
            ("SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL", "never"),
        ],
    )
    .add_build_internal("testlib", move |lib, version| {
        let (_, b) = called_clone.get();
        called_clone.replace((true, b));
        let mut pkg_lib = pkg_config::Config::new()
            .print_system_libs(false)
            .cargo_metadata(false)
            .probe(lib)
            .unwrap();
        pkg_lib.version = version.to_string();
        Ok(Library::from_pkg_config(lib, pkg_lib))
    })
    .add_build_internal("testdata", move |lib, version| {
        let (a, _) = called_clone2.get();
        called_clone2.replace((a, true));
        let mut pkg_lib = pkg_config::Config::new()
            .print_system_libs(false)
            .cargo_metadata(false)
            .probe(lib)
            .unwrap();
        pkg_lib.version = version.to_string();
        Ok(Library::from_pkg_config(lib, pkg_lib))
    });

    let libraries = config.probe_full().unwrap();
    assert_eq!(called.get(), (false, true));
    assert!(libraries.get_by_name("testlib").is_some());
    assert!(libraries.get_by_name("testdata").is_some());
}

#[test]
fn build_internal_override_name() {
    let (libraries, called) = test_build_internal(
        "toml-override-name",
        vec![("SYSTEM_DEPS_BUILD_INTERNAL", "always")],
        "testlib",
    )
    .unwrap();

    assert!(called);
    assert!(libraries.get_by_name("test_lib").is_some());
}

#[test]
fn optional() {
    // without any feature, testmore is not optional
    toml_pkg_config_err_version("toml-optional", "2", vec![]);

    // when enabling v3 testmore is now optional
    let config = create_config("toml-optional", vec![("CARGO_FEATURE_V3", "")]);
    let libs = config.probe_full().unwrap();
    assert!(libs.get_by_name("testlib").is_some());
    assert!(libs.get_by_name("testmore").is_none());
    assert!(libs.get_by_name("testbadger").is_none());

    // testlib is no longer optional if enabling v5
    toml_pkg_config_err_version("toml-optional", "5.0", vec![("CARGO_FEATURE_V5", "")]);
}

#[test]
fn aggregate() {
    let (libraries, _) = toml("toml-two-libs", vec![]).unwrap();

    assert_eq!(libraries.all_libs(), vec!["test", "test2"]);
    assert_eq!(
        libraries.all_link_paths(),
        vec![Path::new("/usr/lib"), Path::new("/usr/lib64")]
    );
    assert_eq!(
        libraries.all_frameworks(),
        vec!["someframework", "someotherframework"]
    );
    assert_eq!(
        libraries.all_framework_paths(),
        vec![Path::new("/usr/lib"), Path::new("/usr/lib64")]
    );
    assert_eq!(
        libraries.all_include_paths(),
        vec![
            Path::new("/usr/include/testanotherlib"),
            Path::new("/usr/include/testlib")
        ]
    );
    assert_eq!(
        libraries.all_defines(),
        vec![
            ("AWESOME", &None),
            ("BADGER", &Some("yes".into())),
            ("GREAT", &None)
        ]
    );
}

#[test]
fn os_specific() {
    let (libraries, _) = toml(
        "toml-os-specific",
        vec![("TARGET", "x86_64-alpine-linux-musl")],
    )
    .unwrap();
    assert!(libraries.get_by_name("testdata").is_some());
    assert!(libraries.get_by_name("testlib").is_some());
    assert!(libraries.get_by_name("testanotherlib").is_some());

    let (libraries, _) = toml("toml-os-specific", vec![("TARGET", "x86_64-apple-darwin")]).unwrap();
    assert!(libraries.get_by_name("testdata").is_none());
    assert!(libraries.get_by_name("testlib").is_none());
    assert!(libraries.get_by_name("testanotherlib").is_some());

    let (libraries, _) = toml(
        "toml-os-specific",
        vec![("TARGET", "x86_64-pc-windows-gnu")],
    )
    .unwrap();
    assert!(libraries.get_by_name("testdata").is_none());
    assert!(libraries.get_by_name("testlib").is_some());
    assert!(libraries.get_by_name("testanotherlib").is_none());
}

#[test]
fn invalid_cfg() {
    let err = toml(
        "toml-invalid-cfg",
        vec![("TARGET", "x86_64-unknown-linux-gnu")],
    )
    .unwrap_err();

    assert_matches!(err, Error::UnsupportedCfg(_));
}

#[test]
fn static_one_lib() {
    let (libraries, flags) = toml(
        "toml-static",
        vec![("SYSTEM_DEPS_TESTSTATICLIB_LINK", "static")],
    )
    .unwrap();

    let testdata = libraries.get_by_name("testdata").unwrap();
    assert!(!testdata.statik);

    let testlib = libraries.get_by_name("teststaticlib").unwrap();
    assert!(testlib.statik);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=./src/tests/lib/
cargo:rustc-link-search=framework=./src/tests/lib/
cargo:rustc-link-lib=static=teststatic
cargo:rustc-link-lib=framework=someframework
cargo:include=./src/tests/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
"#
        .to_string()
        .as_str(),
    );
}

#[test]
fn override_static_no_pkg_config() {
    let (libraries, flags) = toml(
        "toml-static",
        vec![
            ("SYSTEM_DEPS_TESTSTATICLIB_NO_PKG_CONFIG", "1"),
            ("SYSTEM_DEPS_TESTSTATICLIB_LIB", "custom-lib"),
            ("SYSTEM_DEPS_TESTSTATICLIB_LINK", "static"),
        ],
    )
    .unwrap();
    let testlib = libraries.get_by_name("teststaticlib").unwrap();
    assert_eq!(testlib.link_paths, Vec::<PathBuf>::new());
    assert!(testlib.statik);
    assert_eq!(testlib.framework_paths, Vec::<PathBuf>::new());
    assert_eq!(
        testlib.libs,
        vec![InternalLib::new("custom-lib".to_string(), true)]
    );
    assert_eq!(testlib.frameworks, Vec::<String>::new());
    assert_eq!(testlib.include_paths, Vec::<PathBuf>::new());

    assert_flags(
        flags,
        r"cargo:rustc-link-lib=static=custom-lib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
",
    );
}

#[test]
fn static_all_libs() {
    let (libraries, flags) = toml("toml-static", vec![("SYSTEM_DEPS_LINK", "static")]).unwrap();

    let testdata = libraries.get_by_name("testdata").unwrap();
    assert!(testdata.statik);

    let testlib = libraries.get_by_name("teststaticlib").unwrap();
    assert!(testlib.statik);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=./src/tests/lib/
cargo:rustc-link-search=framework=./src/tests/lib/
cargo:rustc-link-lib=static=teststatic
cargo:rustc-link-lib=framework=someframework
cargo:include=./src/tests/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTSTATICLIB_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
"#,
    );
}

#[test]
fn static_lib_not_available() {
    let (libraries, flags) = toml("toml-good", vec![("SYSTEM_DEPS_LINK", "static")]).unwrap();

    let testdata = libraries.get_by_name("testdata").unwrap();
    assert!(testdata.statik);

    // testlib is not available as static library, which is why it is linked dynamically,
    // as seen below
    let testlib = libraries.get_by_name("testlib").unwrap();
    assert!(testlib.statik);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIB_LINK
"#,
    );
}

#[test]
fn has_link_flags() {
    let (libraries, flags) = toml("toml-rpath", vec![]).unwrap();
    let testlib = libraries.get_by_name("testlibwithrpath").unwrap();
    assert_eq!(testlib.version, "1.2.3");
    assert_eq!(
        testlib.defines.get("BADGER").unwrap().as_deref(),
        Some("yes")
    );
    assert!(testlib.defines.get("AWESOME").unwrap().is_none());

    let testdata = libraries.get_by_name("testdata").unwrap();
    assert_eq!(testdata.version, "4.5.6");
    assert!(libraries.get_by_name("testmore").is_none());

    assert_eq!(libraries.iter().len(), 2);

    assert_flags(
        flags,
        r#"cargo:rustc-link-search=native=/usr/lib/
cargo:rustc-link-search=framework=/usr/lib/
cargo:rustc-link-lib=test
cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/
cargo:rustc-link-lib=framework=someframework
cargo:include=/usr/include/testlib
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_INCLUDE
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_LDFLAGS
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_LIB
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_LIB_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_NO_PKG_CONFIG
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_SEARCH_FRAMEWORK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_SEARCH_NATIVE
cargo:rerun-if-env-changed=SYSTEM_DEPS_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_BUILD_INTERNAL
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTDATA_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_TESTLIBWITHRPATH_LINK
cargo:rerun-if-env-changed=SYSTEM_DEPS_LINK
"#,
    );
}
