use std::{
    collections, env, ffi, fs,
    io::{self, Write},
    path, process,
};

struct Project {
    root: tempfile::TempDir,
    files: Vec<(path::PathBuf, Vec<u8>)>,
    env: collections::HashMap<ffi::OsString, ffi::OsString>,
}

impl Project {
    fn new() -> Project {
        Project {
            root: tempfile::tempdir().unwrap(),
            files: Default::default(),
            env: Default::default(),
        }
    }

    fn add_file<N: Into<path::PathBuf>, C: Into<Vec<u8>>>(
        &mut self,
        name: N,
        content: C,
    ) -> &mut Self {
        self.files.push((name.into(), content.into()));
        self
    }

    fn set_env<K, V>(&mut self, k: K, v: V) -> &mut Self
    where
        K: Into<ffi::OsString>,
        V: Into<ffi::OsString>,
    {
        self.env.insert(k.into(), v.into());
        self
    }

    fn bootstrap(&mut self) -> &mut Self {
        let built_root = get_built_root();
        let features = if cfg!(feature = "git2") {
            r#"["git2"]"#
        } else {
            "[]"
        };

        self.add_file(
            "Cargo.toml",
            format!(
                r#"
[package]
name = "testbox"
version = "0.0.1"
build = "build.rs"

[build-dependencies]
built = {{ path = "{}", features = {} }}"#,
                built_root.display().to_string().escape_default(),
                &features,
            ),
        )
        .add_file(
            "build.rs",
            r#"
fn main() {
    built::write_built_file().expect("writing failed");
}"#,
        );
        self
    }

    /// Hold on to the tempdir, it will be removed when dropped!
    fn create(self) -> io::Result<tempfile::TempDir> {
        fs::DirBuilder::new()
            .create(self.root.path().join("src"))
            .unwrap();
        for (name, content) in self.files {
            let fname = self.root.path().join(name);
            assert!(fname.is_absolute());
            fs::create_dir_all(fname.parent().unwrap()).unwrap();
            let mut file = fs::File::create(fname)?;
            file.write_all(&content)?;
        }
        Ok(self.root)
    }

    fn create_and_run(mut self, extra_args: &[&str]) -> (String, String) {
        let env = std::mem::take(&mut self.env);
        let root = self.create().expect("Creating the project failed");
        Self::run(root.as_ref(), extra_args, env)
    }

    fn create_and_build(self, extra_args: &[&str]) {
        let root = self.create().expect("Creating the project failed");
        Self::build(root.as_ref(), extra_args);
    }

    fn run<I, K, V>(root: &std::path::Path, extra_args: &[&str], env: I) -> (String, String)
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<ffi::OsStr>,
        V: AsRef<ffi::OsStr>,
    {
        let cargo_result = process::Command::new("cargo")
            .current_dir(root)
            .arg("run")
            .args(extra_args)
            .envs(env)
            .output()
            .expect("cargo failed");
        let stderr = String::from_utf8_lossy(&cargo_result.stderr);
        let stdout = String::from_utf8_lossy(&cargo_result.stdout);
        assert!(
            cargo_result.status.success(),
            "cargo failed with {}",
            stderr
        );
        assert!(stdout.contains("builttestsuccess"));
        (stderr.into_owned(), stdout.into_owned())
    }

    fn build(root: &std::path::Path, extra_args: &[&str]) {
        let cargo_result = process::Command::new("cargo")
            .current_dir(root)
            .arg("build")
            .args(extra_args)
            .output()
            .expect("cargo failed");
        assert!(
            cargo_result.status.success(),
            "cargo failed with {}",
            String::from_utf8_lossy(&cargo_result.stderr)
        );
    }

    #[cfg(feature = "git2")]
    fn init_git(&self) -> git2::Repository {
        git2::Repository::init_opts(
            &self.root,
            git2::RepositoryInitOptions::new()
                .external_template(false)
                .mkdir(false)
                .no_reinit(true)
                .mkpath(false),
        )
        .expect("git-init failed")
    }
}

/// Tries to find built's Cargo.toml, panics if it ends up in /
fn get_built_root() -> path::PathBuf {
    env::current_exe()
        .map(|path| {
            let mut path = path;
            loop {
                if path.join("Cargo.toml").exists() {
                    break;
                }
                path = path::PathBuf::from(path.parent().unwrap());
            }
            path
        })
        .unwrap()
}

#[test]
#[ignore = "requires target x86_64-unknown-none"]
fn nostd_testbox() {
    let mut p = Project::new();

    let built_root = get_built_root();

    p.add_file(
        "Cargo.toml",
        format!(
            r#"
[package]
name = "nostd_testbox"
version = "1.2.3-rc1"
authors = ["Joe", "Bob"]
build = "build.rs"
description = "xobtset"
homepage = "localhost"
repository = "https://dev.example.com/sources/testbox/"
license = "MIT"

[build-dependencies]
built = {{ path = "{}", default_features=false }}

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[features]
default = ["SuperAwesome", "MegaAwesome"]
SuperAwesome = []
MegaAwesome = []"#,
            built_root.display().to_string().escape_default(),
        ),
    )
    .add_file(
        "build.rs",
        r#"
fn main() {
    built::write_built_file().unwrap();
}"#,
    )
    .add_file(
        "src/main.rs",
        r#"
#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    loop {}
}
"#,
    );

    p.create_and_build(&["--target", "x86_64-unknown-none"]);
}

#[test]
fn unused_override() {
    let mut p = Project::new();
    p.bootstrap()
        .add_file(
            "src/main.rs",
            r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    println!("builttestsuccess");
}
"#,
        )
        .set_env("BUILT_OVERRIDE_testbox_FOO", "BAR")
        .set_env("BUILT_OVERRIDE_testbox1_FOO", "BAR");
    let (stderr, _) = p.create_and_run(&[]);
    assert!(stderr.contains("but was ignored by built: `BUILT_OVERRIDE_testbox_FOO`."));
    assert!(!stderr.contains("but was ignored by built: `BUILT_OVERRIDE_testbox1_FOO`."));
}

#[test]
fn minimal_testbox() {
    let mut p = Project::new();

    let built_root = get_built_root();

    p.add_file(
        "Cargo.toml",
        format!(
            r#"
[package]
name = "minimal_testbox"
version = "1.2.3-rc1"
authors = ["Joe", "Bob"]
build = "build.rs"
description = "xobtset"
homepage = "localhost"
repository = "https://dev.example.com/sources/testbox/"
license = "MIT"

[dependencies]
built = {{ path = "{built_root}", default_features=false }}

[build-dependencies]
built = {{ path = "{built_root}", default_features=false }}

[features]
default = ["SuperAwesome", "MegaAwesome"]
SuperAwesome = []
MegaAwesome = []"#,
            built_root = built_root.display().to_string().escape_default()
        ),
    );

    p.add_file(
        "build.rs",
        r#"

fn main() {
    built::write_built_file().unwrap();
}"#,
    );

    p.add_file(
        "src/main.rs",
        r#"
//! The minimal testbox.

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::PKG_VERSION, "1.2.3-rc1");
    assert_eq!(built_info::PKG_VERSION_MAJOR, "1");
    assert_eq!(built_info::PKG_VERSION_MINOR, "2");
    assert_eq!(built_info::PKG_VERSION_PATCH, "3");
    assert_eq!(built_info::PKG_VERSION_PRE, "rc1");
    assert_eq!(built_info::PKG_AUTHORS, "Joe:Bob");
    assert_eq!(built_info::PKG_NAME, "minimal_testbox");
    assert_eq!(built_info::PKG_DESCRIPTION, "xobtset");
    assert_eq!(built_info::PKG_HOMEPAGE, "localhost");
    assert_eq!(built_info::PKG_LICENSE, "MIT");
    assert_eq!(built_info::PKG_REPOSITORY, "https://dev.example.com/sources/testbox/");
    assert!(built_info::NUM_JOBS > 0);
    assert!(built_info::OPT_LEVEL == "0");
    assert!(built_info::DEBUG);
    assert_eq!(built_info::PROFILE, "debug");
    assert_eq!(built_info::FEATURES,
               ["DEFAULT", "MEGAAWESOME", "SUPERAWESOME"]);
    assert_eq!(built_info::FEATURES_STR,
               "DEFAULT, MEGAAWESOME, SUPERAWESOME");
    assert_eq!(built_info::FEATURES_LOWERCASE,
               ["default", "megaawesome", "superawesome"]);
    assert_eq!(built_info::FEATURES_LOWERCASE_STR,
               "default, megaawesome, superawesome");
    assert_ne!(built_info::RUSTC_VERSION, "");
    assert_ne!(built_info::RUSTDOC_VERSION, "");
    assert_ne!(built_info::HOST, "");
    assert_ne!(built_info::TARGET, "");
    assert_ne!(built_info::RUSTC, "");
    assert_ne!(built_info::RUSTDOC, "");
    assert_ne!(built_info::CFG_TARGET_ARCH, "");
    assert_ne!(built_info::CFG_ENDIAN, "");
    assert_ne!(built_info::CFG_FAMILY, "");
    assert_ne!(built_info::CFG_OS, "");
    assert_ne!(built_info::CFG_POINTER_WIDTH, "");
    // For CFG_ENV, empty string is a possible value.
    let _: &'static str = built_info::CFG_ENV;
    println!("builttestsuccess");
}"#,
    );

    p.create_and_run(&[]);
}

#[test]
fn simple_workspace() {
    let mut p = Project::new();
    let built_root = get_built_root();

    p.add_file("Cargo.toml", "[workspace]\nmembers = ['foobar']");
    p.add_file(
        "foobar/Cargo.toml",
        format!(
            r#"[package]
name = "foobar"
version = "5.6.7"
build = "build.rs"

[build-dependencies]
built = {{ path = "{}" }}"#,
            built_root.display().to_string().escape_default()
        ),
    );
    p.add_file(
        "foobar/build.rs",
        "fn main() { built::write_built_file().unwrap(); }",
    );
    p.add_file(
        "foobar/src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
fn main() {
    assert_eq!(built_info::PKG_VERSION, "5.6.7");
    println!("builttestsuccess");
}
"#,
    );
    p.create_and_run(&[]);
}

#[test]
#[cfg(all(
    feature = "cargo-lock",
    feature = "dependency-tree",
    feature = "git2",
    feature = "chrono",
    feature = "semver"
))]
fn full_testbox() {
    let mut p = Project::new();

    let built_root = get_built_root();

    p.add_file(
        "Cargo.toml",
        format!(
            r#"
[package]
name = "testbox"
version = "1.2.3-rc1"
authors = ["Joe", "Bob", "Harry:Potter"]
build = "build.rs"
description = "xobtset"
homepage = "localhost"
repository = "https://dev.example.com/sources/testbox/"
license = "MIT"

[dependencies]
built = {{ path = "{built_root}", features=["cargo-lock", "dependency-tree", "git2", "chrono", "semver"] }}

[build-dependencies]
built = {{ path = "{built_root}", features=["cargo-lock", "dependency-tree", "git2", "chrono", "semver"] }}

[features]
default = ["SuperAwesome", "MegaAwesome"]
SuperAwesome = []
MegaAwesome = []"#,
            built_root = built_root.display().to_string().escape_default()
        ),
    )
    .add_file(
        "build.rs",
        r#"
fn main() {
    built::write_built_file().unwrap();
}"#,
    )
    .set_env("CONTINUOUS_INTEGRATION", "1")
    .add_file(
        "src/main.rs",
        r#"
//! The testbox.

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_VERSION, None);
    assert_eq!(built_info::GIT_DIRTY, None);
    assert_eq!(built_info::GIT_COMMIT_HASH, None);
    assert_eq!(built_info::GIT_COMMIT_HASH_SHORT, None);
    assert_eq!(built_info::GIT_HEAD_REF, None);
    assert!(built_info::CI_PLATFORM.is_some());
    assert_eq!(built_info::PKG_VERSION, "1.2.3-rc1");
    assert_eq!(built_info::PKG_VERSION_MAJOR, "1");
    assert_eq!(built_info::PKG_VERSION_MINOR, "2");
    assert_eq!(built_info::PKG_VERSION_PATCH, "3");
    assert_eq!(built_info::PKG_VERSION_PRE, "rc1");
    assert_eq!(built_info::PKG_AUTHORS, "Joe:Bob:Harry:Potter");
    assert_eq!(built_info::PKG_NAME, "testbox");
    assert_eq!(built_info::PKG_DESCRIPTION, "xobtset");
    assert_eq!(built_info::PKG_HOMEPAGE, "localhost");
    assert_eq!(built_info::PKG_LICENSE, "MIT");
    assert_eq!(built_info::PKG_REPOSITORY, "https://dev.example.com/sources/testbox/");
    assert!(built_info::NUM_JOBS > 0);
    assert!(built_info::OPT_LEVEL == "0");
    assert!(built_info::DEBUG);
    assert_eq!(built_info::PROFILE, "debug");
    assert_eq!(built_info::FEATURES,
               ["DEFAULT", "MEGAAWESOME", "SUPERAWESOME"]);
    assert_eq!(built_info::FEATURES_STR,
               "DEFAULT, MEGAAWESOME, SUPERAWESOME");
    assert_eq!(built_info::FEATURES_LOWERCASE,
               ["default", "megaawesome", "superawesome"]);
    assert_eq!(built_info::FEATURES_LOWERCASE_STR,
               "default, megaawesome, superawesome");
    assert_ne!(built_info::RUSTC_VERSION, "");
    assert_ne!(built_info::RUSTDOC_VERSION, "");
    assert_ne!(built_info::DEPENDENCIES_STR, "");
    assert_ne!(built_info::DIRECT_DEPENDENCIES_STR, "");
    assert_ne!(built_info::INDIRECT_DEPENDENCIES_STR, "");
    assert_ne!(built_info::HOST, "");
    assert_ne!(built_info::TARGET, "");
    assert_ne!(built_info::RUSTC, "");
    assert_ne!(built_info::RUSTDOC, "");
    assert_ne!(built_info::CFG_TARGET_ARCH, "");
    assert_ne!(built_info::CFG_ENDIAN, "");
    assert_ne!(built_info::CFG_FAMILY, "");
    assert_ne!(built_info::CFG_OS, "");
    assert_ne!(built_info::CFG_POINTER_WIDTH, "");
    // For CFG_ENV, empty string is a possible value.
    let _: &'static str = built_info::CFG_ENV;

    assert!(built::util::parse_versions(built_info::DEPENDENCIES.iter())
        .any(|(name, ver)| name == "toml" && ver >= built::semver::Version::parse("0.1.0").unwrap()));

    assert_eq!(built_info::DIRECT_DEPENDENCIES.len(), 1);
    assert_eq!(built_info::DIRECT_DEPENDENCIES[0].0, "built");

    assert!((built::chrono::offset::Utc::now() - built::util::strptime(built_info::BUILT_TIME_UTC)).num_days() <= 1);

    assert!(built_info::OVERRIDE_VARIABLES_USED.is_empty());

    println!("builttestsuccess");
}"#,
    );
    p.create_and_run(&[]);
}

#[test]
#[cfg(all(feature = "git2", feature = "chrono", feature = "semver"))]
fn overridden_testbox() {
    let mut p = Project::new();

    let built_root = get_built_root();

    p.add_file(
        "Cargo.toml",
        format!(
            r#"
[package]
name = "testbox"
version = "1.2.3-rc1"
authors = ["Joe", "Bob", "Harry:Potter"]
build = "build.rs"
description = "xobtset"
homepage = "localhost"
repository = "https://dev.example.com/sources/testbox/"
license = "MIT"

[dependencies]
built = {{ path = "{built_root}", features=["git2", "chrono", "semver"] }}

[build-dependencies]
built = {{ path = "{built_root}", features=["git2", "chrono", "semver"] }}

[features]
default = ["SuperAwesome", "MegaAwesome"]
SuperAwesome = []
MegaAwesome = []"#,
            built_root = built_root.display().to_string().escape_default()
        ),
    )
    .add_file(
        "build.rs",
        r#"
fn main() {
    built::write_built_file().unwrap();
}"#,
    )
    .set_env("BUILT_OVERRIDE_testbox_GIT_VERSION", "GIT_VERSION1")
    .set_env("BUILT_OVERRIDE_testbox_GIT_DIRTY", "false")
    .set_env("BUILT_OVERRIDE_testbox_GIT_COMMIT_HASH", "1234567890")
    .set_env("BUILT_OVERRIDE_testbox_GIT_HEAD_REF", "GIT_REF")
    .set_env("BUILT_OVERRIDE_testbox_CI_PLATFORM", "TESTBOXCI")
    .set_env("BUILT_OVERRIDE_testbox_PKG_VERSION", "1.2.3.4")
    .set_env("BUILT_OVERRIDE_testbox_PKG_VERSION_MAJOR", "abc")
    .set_env("BUILT_OVERRIDE_testbox_PKG_VERSION_MINOR", "def")
    .set_env("BUILT_OVERRIDE_testbox_PKG_VERSION_PATCH", "ghi")
    .set_env("BUILT_OVERRIDE_testbox_PKG_VERSION_PRE", "jkl")
    .set_env("BUILT_OVERRIDE_testbox_PKG_AUTHORS", "The council")
    .set_env("BUILT_OVERRIDE_testbox_PKG_NAME", "OVERRIDEBOX")
    .set_env("BUILT_OVERRIDE_testbox_PKG_DESCRIPTION", "TEST")
    .set_env("BUILT_OVERRIDE_testbox_PKG_HOMEPAGE", "foreignhost")
    .set_env("BUILT_OVERRIDE_testbox_PKG_LICENSE", "MITv2")
    .set_env("BUILT_OVERRIDE_testbox_PKG_REPOSITORY", "8.8.8.8")
    .set_env("BUILT_OVERRIDE_testbox_NUM_JOBS", "999")
    .set_env(
        "BUILT_OVERRIDE_testbox_OPT_LEVEL",
        "not_too_fast_not_too_slow",
    )
    .set_env("BUILT_OVERRIDE_testbox_DEBUG", "false")
    .set_env("BUILT_OVERRIDE_testbox_PROFILE", "MEDIUM")
    .set_env(
        "BUILT_OVERRIDE_testbox_FEATURES",
        "cup_holder,Stereo Sound, dynamic range",
    )
    .set_env("BUILT_OVERRIDE_testbox_RUSTC", "overridec")
    .set_env("BUILT_OVERRIDE_testbox_RUSTC_VERSION", "overridec v1")
    .set_env("BUILT_OVERRIDE_testbox_RUSTDOC", "overridedoc")
    .set_env("BUILT_OVERRIDE_testbox_RUSTDOC_VERSION", "overridedoc v1")
    .set_env("BUILT_OVERRIDE_testbox_HOST", "overridehost")
    .set_env("BUILT_OVERRIDE_testbox_TARGET", "potato")
    .set_env("BUILT_OVERRIDE_testbox_CFG_TARGET_ARCH", "potatoes")
    .set_env("BUILT_OVERRIDE_testbox_CFG_ENDIAN", "random")
    .set_env("BUILT_OVERRIDE_testbox_CFG_FAMILY", "v0")
    .set_env("BUILT_OVERRIDE_testbox_CFG_OS", "none")
    .set_env("BUILT_OVERRIDE_testbox_CFG_POINTER_WIDTH", "63.9998")
    .set_env("BUILT_OVERRIDE_testbox_CFG_ENV", "calm")
    .set_env(
        "BUILT_OVERRIDE_testbox_BUILT_TIME_UTC",
        "Sat, 25 May 2024 12:15:59 +0000",
    )
    .add_file(
        "src/main.rs",
        r#"
//! The testbox.

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_VERSION, Some("GIT_VERSION1"));
    assert_eq!(built_info::GIT_DIRTY, Some(false));
    assert_eq!(built_info::GIT_COMMIT_HASH, Some("1234567890"));
    assert_eq!(built_info::GIT_COMMIT_HASH_SHORT, Some("12345678"));
    assert_eq!(built_info::GIT_HEAD_REF, Some("GIT_REF"));
    assert_eq!(built_info::CI_PLATFORM, Some("TESTBOXCI"));
    assert_eq!(built_info::PKG_VERSION, "1.2.3.4");
    assert_eq!(built_info::PKG_VERSION_MAJOR, "abc");
    assert_eq!(built_info::PKG_VERSION_MINOR, "def");
    assert_eq!(built_info::PKG_VERSION_PATCH, "ghi");
    assert_eq!(built_info::PKG_VERSION_PRE, "jkl");
    assert_eq!(built_info::PKG_AUTHORS, "The council");
    assert_eq!(built_info::PKG_NAME, "OVERRIDEBOX");
    assert_eq!(built_info::PKG_DESCRIPTION, "TEST");
    assert_eq!(built_info::PKG_HOMEPAGE, "foreignhost");
    assert_eq!(built_info::PKG_LICENSE, "MITv2");
    assert_eq!(built_info::PKG_REPOSITORY, "8.8.8.8");
    assert_eq!(built_info::NUM_JOBS, 999);
    assert_eq!(built_info::OPT_LEVEL, "not_too_fast_not_too_slow");
    assert!(!built_info::DEBUG);
    assert_eq!(built_info::PROFILE, "MEDIUM");
    assert_eq!(built_info::FEATURES,
               ["Stereo Sound", "cup_holder", "dynamic range"]);
    assert_eq!(built_info::FEATURES_STR,
               "Stereo Sound, cup_holder, dynamic range");
    assert_eq!(built_info::FEATURES_LOWERCASE,
               ["cup_holder", "dynamic range", "stereo sound"]);
    assert_eq!(built_info::FEATURES_LOWERCASE_STR,
               "cup_holder, dynamic range, stereo sound");
    assert_eq!(built_info::RUSTC, "overridec");
    assert_eq!(built_info::RUSTC_VERSION, "overridec v1");
    assert_eq!(built_info::RUSTDOC, "overridedoc");
    assert_eq!(built_info::RUSTDOC_VERSION, "overridedoc v1");
    assert_eq!(built_info::HOST, "overridehost");
    assert_eq!(built_info::TARGET, "potato");
    assert_eq!(built_info::CFG_TARGET_ARCH, "potatoes");
    assert_eq!(built_info::CFG_ENDIAN, "random");
    assert_eq!(built_info::CFG_FAMILY, "v0");
    assert_eq!(built_info::CFG_OS, "none");
    assert_eq!(built_info::CFG_POINTER_WIDTH, "63.9998");
    assert_eq!(built_info::CFG_ENV, "calm");

    assert_eq!(built::util::strptime(built_info::BUILT_TIME_UTC).to_rfc2822(),
              "Sat, 25 May 2024 12:15:59 +0000");
    assert_eq!(built_info::OVERRIDE_VARIABLES_USED, [
        "BUILT_TIME_UTC",
        "CFG_ENDIAN",
        "CFG_ENV",
        "CFG_FAMILY",
        "CFG_OS",
        "CFG_POINTER_WIDTH",
        "CFG_TARGET_ARCH",
        "CI_PLATFORM",
        "DEBUG",
        "FEATURES",
        "GIT_COMMIT_HASH",
        "GIT_DIRTY",
        "GIT_HEAD_REF",
        "GIT_VERSION",
        "HOST",
        "NUM_JOBS",
        "OPT_LEVEL",
        "PKG_AUTHORS",
        "PKG_DESCRIPTION",
        "PKG_HOMEPAGE",
        "PKG_LICENSE",
        "PKG_NAME",
        "PKG_REPOSITORY",
        "PKG_VERSION",
        "PKG_VERSION_MAJOR",
        "PKG_VERSION_MINOR",
        "PKG_VERSION_PATCH",
        "PKG_VERSION_PRE",
        "PROFILE",
        "RUSTC",
        "RUSTC_VERSION",
        "RUSTDOC",
        "RUSTDOC_VERSION",
        "TARGET",
    ]);
    println!("builttestsuccess");
}"#,
    );
    p.create_and_run(&[]);
}

#[test]
#[cfg(feature = "chrono")]
fn source_date_epoch() {
    let mut p = Project::new();
    let built_root = get_built_root();

    p.add_file(
        "Cargo.toml",
        format!(
            r#"
[package]
name = "testbox"
version = "1.2.3-rc1"
authors = ["Joe", "Bob", "Harry:Potter"]
build = "build.rs"
description = "xobtset"

[dependencies]
built = {{ path = "{built_root}", features=["chrono"] }}

[build-dependencies]
built = {{ path = "{built_root}", features=["chrono"] }}"#,
            built_root = built_root.display().to_string().escape_default()
        ),
    )
    .add_file(
        "build.rs",
        r#"
fn main() {
    built::write_built_file().unwrap();
}"#,
    )
    .add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built::util::strptime(built_info::BUILT_TIME_UTC).to_rfc2822(),
              "Sat, 25 May 2024 12:15:59 +0000");
    assert_eq!(built_info::NUM_JOBS, 1);
    println!("builttestsuccess");
}"#,
    )
    .set_env("SOURCE_DATE_EPOCH", "1716639359");
    p.create_and_run(&[]);
}

#[test]
#[cfg(feature = "git2")]
fn git_no_git() {
    // `root` isn't even a git-repo
    let mut p = Project::new();
    p.bootstrap().add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_DIRTY, None);
    println!("builttestsuccess");
}
"#,
    );

    p.create_and_run(&[]);
}

#[test]
#[cfg(feature = "git2")]
fn clean_then_dirty_git() {
    let mut p = Project::new();
    p.bootstrap().add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_DIRTY, Some(false));
    println!("builttestsuccess");
}
"#,
    );
    let repo = p.init_git();
    let root = p.create().expect("Creating the project failed");

    let sig = git2::Signature::now("foo", "bar").unwrap();

    let mut idx = repo.index().unwrap();
    for p in &["src/main.rs", "build.rs"] {
        idx.add_path(path::Path::new(p)).unwrap();
    }
    idx.write().unwrap();
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Testing testing 1 2 3",
        &repo.find_tree(idx.write_tree().unwrap()).unwrap(),
        &[],
    )
    .unwrap();
    Project::run(
        root.as_ref(),
        &[],
        std::iter::empty::<(ffi::OsString, ffi::OsString)>(),
    );

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(root.path().join("src/main.rs"))
        .unwrap();
    f.write_all(
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_DIRTY, Some(true));
    assert!(built_info::GIT_COMMIT_HASH.is_some());
    assert!(built_info::GIT_COMMIT_HASH_SHORT.is_some());
    assert!(built_info::GIT_COMMIT_HASH.unwrap().starts_with(built_info::GIT_COMMIT_HASH_SHORT.unwrap()));
    println!("builttestsuccess");
}
"#
        .as_bytes(),
    )
    .unwrap();

    Project::run(
        root.as_ref(),
        &[],
        std::iter::empty::<(ffi::OsString, ffi::OsString)>(),
    );
}

#[test]
#[cfg(feature = "git2")]
fn empty_git() {
    // Issue #7, git can be there and still fail
    let mut p = Project::new();
    p.bootstrap().add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    println!("builttestsuccess");
}
"#,
    );
    p.init_git();
    p.create_and_run(&[]);
}

#[cfg(target_os = "windows")]
#[test]
fn absolute_paths() {
    // Issue #35. Usually binaries we refer to are simply executables names but sometimes they are
    // absolute paths, containing backslashes, and everything gets sad on this devilish platform.

    let mut p = Project::new();
    p.bootstrap().add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    println!("builttestsuccess");
}
"#,
    );

    let rustc_exe_buf = String::from_utf8(
        process::Command::new("where")
            .arg("rustc")
            .output()
            .expect("Unable to locate absolute path to rustc using `where`")
            .stdout,
    )
    .unwrap();
    let rustc_exe = rustc_exe_buf.split("\r\n").next().unwrap();

    // There should at least be `C:\`
    assert!(rustc_exe.contains('\\'));

    let root = p.create().expect("Creating the project failed");
    let cargo_result = process::Command::new("cargo")
        .current_dir(&root)
        .arg("run")
        .env("RUSTC", &rustc_exe)
        .output()
        .expect("cargo failed");
    if !cargo_result.status.success() {
        panic!(
            "cargo failed with {}",
            String::from_utf8_lossy(&cargo_result.stderr)
        );
    }
}
