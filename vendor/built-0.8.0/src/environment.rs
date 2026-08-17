use crate::util::{self, ArrayDisplay};
use crate::{fmt_option_str, write_str_variable, write_variable};
use std::{cell, collections, env, ffi, fmt, fs, io, process};

const BUILT_OVERRIDE_PREFIX: &str = "BUILT_OVERRIDE_";

#[derive(Debug, Default)]
enum EnvironmentValue {
    #[default]
    Unused,
    Queried,
    Used,
}

impl EnvironmentValue {
    fn upgrade_to_queried(&mut self) {
        if matches!(self, EnvironmentValue::Unused) {
            *self = EnvironmentValue::Queried;
        }
    }

    fn upgrade_to_used(&mut self) {
        *self = EnvironmentValue::Used;
    }

    pub fn is_unused(&self) -> bool {
        matches!(self, EnvironmentValue::Unused)
    }

    pub fn is_used(&self) -> bool {
        matches!(self, EnvironmentValue::Used)
    }
}

pub struct EnvironmentMap {
    map: collections::HashMap<String, (String, cell::RefCell<EnvironmentValue>)>,
    override_prefix: String,
}

fn get_version_from_cmd(executable: &ffi::OsStr) -> io::Result<String> {
    let output = process::Command::new(executable).arg("-V").output()?;
    let mut v = String::from_utf8(output.stdout).unwrap();
    v.pop(); // remove newline
    Ok(v)
}

impl EnvironmentMap {
    pub fn new() -> Self {
        let map = env::vars_os()
            .filter_map(|(k, v)| match (k.into_string(), v.into_string()) {
                (Ok(k), Ok(v)) => Some((k, (v, cell::RefCell::default()))),
                _ => None,
            })
            .collect::<collections::HashMap<_, _>>();
        let override_prefix = format!("{}{}_", BUILT_OVERRIDE_PREFIX, map["CARGO_PKG_NAME"].0);
        Self {
            map,
            override_prefix,
        }
    }

    fn override_key(&self, key: &str) -> String {
        let mut prefixed_key = self.override_prefix.clone();
        prefixed_key.push_str(key);
        prefixed_key
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|v| {
            v.1.borrow_mut().upgrade_to_queried();
            v.0.as_str()
        })
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn filter_map_keys<F>(&self, mut f: F) -> impl Iterator<Item = &str>
    where
        F: FnMut(&str) -> Option<&str>,
    {
        self.map.iter().filter_map(move |v| match f(v.0.as_str()) {
            Some(w) => {
                v.1 .1.borrow_mut().upgrade_to_queried();
                Some(w)
            }
            None => None,
        })
    }

    pub fn get_override_var<'a, T>(&'a self, key: &str) -> Option<T>
    where
        T: util::ParseFromEnv<'a>,
    {
        self.map.get(&self.override_key(key)).map(|v| {
            v.1.borrow_mut().upgrade_to_queried();
            match T::parse_from_env(v.0.as_str()) {
                Ok(t) => {
                    v.1.borrow_mut().upgrade_to_used();
                    t
                }
                Err(e) => {
                    panic!("Failed to parse `{key}`=`{0}`: {e:?}", v.0);
                }
            }
        })
    }

    pub fn unused_override_vars(&self) -> impl Iterator<Item = &str> {
        self.map.iter().filter_map(|(k, v)| {
            if k.starts_with(&self.override_prefix) && v.1.borrow().is_unused() {
                Some(k.as_str())
            } else {
                None
            }
        })
    }

    pub fn used_override_vars(&self) -> impl Iterator<Item = &str> {
        self.map.iter().filter_map(|(k, v)| {
            if v.1.borrow().is_used() {
                Some(k.strip_prefix(&self.override_prefix).unwrap())
            } else {
                None
            }
        })
    }

    pub fn write_ci(&self, mut w: &fs::File) -> io::Result<()> {
        use io::Write;

        let ci = match self.get_override_var("CI_PLATFORM") {
            Some(v) => v,
            None => self.detect_ci().map(|ci| ci.to_string()),
        };
        write_variable!(
            w,
            "CI_PLATFORM",
            "Option<&str>",
            fmt_option_str(ci),
            "The Continuous Integration platform detected during compilation."
        );
        Ok(())
    }

    pub fn write_env(&self, mut w: &fs::File) -> io::Result<()> {
        use io::Write;
        macro_rules! write_env_str {
            ($(($name:ident, $env_name:expr, $doc:expr)),*) => {$(
                let v = match self.get_override_var(stringify!($name)) {
                    Some(v) => v,
                    None => self.get($env_name).expect(stringify!(Missing expected environment variable $env_name)),
                };
                write_str_variable!(
                    w,
                    stringify!($name),
                    v,
                        $doc
                );
            )*}
        }

        write_env_str!(
            (PKG_VERSION, "CARGO_PKG_VERSION", "The full version."),
            (
                PKG_VERSION_MAJOR,
                "CARGO_PKG_VERSION_MAJOR",
                "The major version."
            ),
            (
                PKG_VERSION_MINOR,
                "CARGO_PKG_VERSION_MINOR",
                "The minor version."
            ),
            (
                PKG_VERSION_PATCH,
                "CARGO_PKG_VERSION_PATCH",
                "The patch version."
            ),
            (
                PKG_VERSION_PRE,
                "CARGO_PKG_VERSION_PRE",
                "The pre-release version."
            ),
            (
                PKG_AUTHORS,
                "CARGO_PKG_AUTHORS",
                "A colon-separated list of authors."
            ),
            (PKG_NAME, "CARGO_PKG_NAME", "The name of the package."),
            (PKG_DESCRIPTION, "CARGO_PKG_DESCRIPTION", "The description."),
            (PKG_HOMEPAGE, "CARGO_PKG_HOMEPAGE", "The homepage."),
            (PKG_LICENSE, "CARGO_PKG_LICENSE", "The license."),
            (
                PKG_REPOSITORY,
                "CARGO_PKG_REPOSITORY",
                "The source repository as advertised in Cargo.toml."
            ),
            (
                TARGET,
                "TARGET",
                "The target triple that was being compiled for."
            ),
            (HOST, "HOST", "The host triple of the rust compiler."),
            (
                PROFILE,
                "PROFILE",
                "`release` for release builds, `debug` for other builds."
            ),
            (RUSTC, "RUSTC", "The compiler that cargo resolved to use."),
            (
                RUSTDOC,
                "RUSTDOC",
                "The documentation generator that cargo resolved to use."
            )
        );

        write_str_variable!(
            w,
            "OPT_LEVEL",
            self.get_override_var("OPT_LEVEL")
                .unwrap_or_else(|| env::var("OPT_LEVEL").unwrap()),
            "Value of OPT_LEVEL for the profile used during compilation."
        );

        write_variable!(
            w,
            "NUM_JOBS",
            "u32",
            self.get_override_var("NUM_JOBS").unwrap_or_else(|| {
                if env::var(crate::SOURCE_DATE_EPOCH).is_ok() {
                    1u32
                } else {
                    env::var("NUM_JOBS").unwrap().parse().unwrap()
                }
            }),
            "The parallelism that was specified during compilation."
        );

        write_variable!(
            w,
            "DEBUG",
            "bool",
            self.get_override_var("DEBUG")
                .unwrap_or_else(|| env::var("DEBUG").unwrap() == "true"),
            "Value of DEBUG for the profile used during compilation."
        );
        Ok(())
    }

    pub fn write_features(&self, mut w: &fs::File) -> io::Result<()> {
        use io::Write;

        let mut features = self.get_override_var("FEATURES").unwrap_or_else(|| {
            self.filter_map_keys(|k| k.strip_prefix("CARGO_FEATURE_"))
                .map(|f| f.to_owned())
                .collect::<Vec<_>>()
        });
        features.sort_unstable();

        write_variable!(
            w,
            "FEATURES",
            format_args!("[&str; {}]", features.len()),
            ArrayDisplay(&features, |t, f| write!(f, "\"{}\"", t.escape_default())),
            "The features that were enabled during compilation."
        );
        let features_str = features.join(", ");
        write_str_variable!(
            w,
            "FEATURES_STR",
            features_str,
            "The features as a comma-separated string."
        );

        let mut lowercase_features = features
            .iter()
            .map(|name| name.to_lowercase())
            .collect::<Vec<_>>();
        lowercase_features.sort_unstable();

        write_variable!(
            w,
            "FEATURES_LOWERCASE",
            format_args!("[&str; {}]", lowercase_features.len()),
            ArrayDisplay(&lowercase_features, |val, fmt| write!(
                fmt,
                "\"{}\"",
                val.escape_default()
            )),
            "The features as above, as lowercase strings."
        );
        let lowercase_features_str = lowercase_features.join(", ");
        write_str_variable!(
            w,
            "FEATURES_LOWERCASE_STR",
            lowercase_features_str,
            "The feature-string as above, from lowercase strings."
        );

        Ok(())
    }

    pub fn write_cfg(&self, mut w: &fs::File) -> io::Result<()> {
        use io::Write;

        write_str_variable!(
            w,
            "CFG_TARGET_ARCH",
            self.get_override_var("CFG_TARGET_ARCH")
                .unwrap_or_else(|| self.get("CARGO_CFG_TARGET_ARCH").unwrap()),
            "The target architecture, given by `CARGO_CFG_TARGET_ARCH`."
        );

        write_str_variable!(
            w,
            "CFG_ENDIAN",
            self.get_override_var("CFG_ENDIAN")
                .unwrap_or_else(|| self.get("CARGO_CFG_TARGET_ENDIAN").unwrap()),
            "The endianness, given by `CARGO_CFG_TARGET_ENDIAN`."
        );

        write_str_variable!(
            w,
            "CFG_ENV",
            self.get_override_var("CFG_ENV")
                .unwrap_or_else(|| self.get("CARGO_CFG_TARGET_ENV").unwrap()),
            "The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`."
        );

        write_str_variable!(
            w,
            "CFG_FAMILY",
            self.get_override_var("CFG_FAMILY")
                .unwrap_or_else(|| self.get("CARGO_CFG_TARGET_FAMILY").unwrap_or_default()),
            "The OS-family, given by `CARGO_CFG_TARGET_FAMILY`."
        );

        write_str_variable!(
            w,
            "CFG_OS",
            self.get_override_var("CFG_OS")
                .unwrap_or_else(|| self.get("CARGO_CFG_TARGET_OS").unwrap()),
            "The operating system, given by `CARGO_CFG_TARGET_OS`."
        );

        write_str_variable!(
            w,
            "CFG_POINTER_WIDTH",
            self.get_override_var("CFG_POINTER_WIDTH")
                .unwrap_or_else(|| self.get("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap()),
            "The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`."
        );

        Ok(())
    }

    pub fn write_compiler_version(&self, mut w: &fs::File) -> io::Result<()> {
        use std::io::Write;

        let rustc;
        let rustc_version;
        match self.get_override_var("RUSTC") {
            Some(v) => {
                rustc = v;
                rustc_version = self
                    .get_override_var("RUSTC_VERSION")
                    .expect("RUSTC_VERSION must be overridden if RUSTC is")
            }
            None => {
                rustc = self.get("RUSTC").unwrap();
                rustc_version = get_version_from_cmd(rustc.as_ref())?;
            }
        }

        let rustdoc;
        let rustdoc_version;
        match self.get_override_var("RUSTDOC") {
            Some(v) => {
                rustdoc = v;
                rustdoc_version = self.get_override_var("RUSTDOC_VERSION").unwrap_or_default();
            }
            None => {
                rustdoc = self.get("RUSTDOC").unwrap();
                rustdoc_version = get_version_from_cmd(rustdoc.as_ref()).unwrap_or_default();
            }
        }

        write_str_variable!(
            w,
            "RUSTC_VERSION",
            rustc_version,
            format_args!("The output of `{rustc} -V`")
        );

        write_str_variable!(
            w,
            "RUSTDOC_VERSION",
            rustdoc_version,
            format_args!(
                "The output of `{rustdoc} -V`; empty string if `{rustdoc} -V` failed to execute"
            )
        );
        Ok(())
    }

    pub fn detect_ci(&self) -> Option<CIPlatform> {
        macro_rules! detect {
            ($(($k:expr, $v:expr, $i:ident)),*) => {$(
                    if self.get($k).map_or(false, |v| v == $v) {
                        return Some(CIPlatform::$i);
                    }
                    )*};
            ($(($k:expr, $i:ident)),*) => {$(
                    if self.contains_key($k) {
                        return Some(CIPlatform::$i);
                    }
                    )*};
            ($($k:expr),*) => {$(
                if self.contains_key($k) {
                    return Some(CIPlatform::Generic);
                }
            )*};
        }
        // Variable names collected by watson/ci-info
        detect!(
            ("TRAVIS", Travis),
            ("CIRCLECI", Circle),
            ("GITLAB_CI", GitLab),
            ("APPVEYOR", AppVeyor),
            ("DRONE", Drone),
            ("MAGNUM", Magnum),
            ("SEMAPHORE", Semaphore),
            ("JENKINS_URL", Jenkins),
            ("bamboo_planKey", Bamboo),
            ("TF_BUILD", TFS),
            ("TEAMCITY_VERSION", TeamCity),
            ("BUILDKITE", Buildkite),
            ("HUDSON_URL", Hudson),
            ("GO_PIPELINE_LABEL", GoCD),
            ("BITBUCKET_COMMIT", BitBucket),
            ("GITHUB_ACTIONS", GitHubActions)
        );

        if self.contains_key("TASK_ID") && self.contains_key("RUN_ID") {
            return Some(CIPlatform::TaskCluster);
        }

        detect!(("CI_NAME", "codeship", Codeship));

        detect!(
            "CI",                     // Could be Travis, Circle, GitLab, AppVeyor or CodeShip
            "CONTINUOUS_INTEGRATION", // Probably Travis
            "BUILD_NUMBER"            // Jenkins, TeamCity
        );
        None
    }
}

/// Various Continuous Integration platforms whose presence can be detected.
pub enum CIPlatform {
    /// <https://travis-ci.org>
    Travis,
    /// <https://circleci.com>
    Circle,
    /// <https://about.gitlab.com/gitlab-ci>
    GitLab,
    /// <https://www.appveyor.com>
    AppVeyor,
    /// <https://codeship.com>
    Codeship,
    /// <https://github.com/drone/drone>
    Drone,
    /// <https://magnum-ci.com>
    Magnum,
    /// <https://semaphoreci.com>
    Semaphore,
    /// <https://jenkins.io>
    Jenkins,
    /// <https://www.atlassian.com/software/bamboo>
    Bamboo,
    /// <https://www.visualstudio.com/de/tfs>
    TFS,
    /// <https://www.jetbrains.com/teamcity>
    TeamCity,
    /// <https://buildkite.com>
    Buildkite,
    /// <http://hudson-ci.org>
    Hudson,
    /// <https://github.com/taskcluster>
    TaskCluster,
    /// <https://www.gocd.io>
    GoCD,
    /// <https://bitbucket.org>
    BitBucket,
    /// <https://github.com/features/actions>
    GitHubActions,
    /// Unspecific
    Generic,
}

impl fmt::Display for CIPlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            CIPlatform::Travis => "Travis CI",
            CIPlatform::Circle => "CircleCI",
            CIPlatform::GitLab => "GitLab",
            CIPlatform::AppVeyor => "AppVeyor",
            CIPlatform::Codeship => "CodeShip",
            CIPlatform::Drone => "Drone",
            CIPlatform::Magnum => "Magnum",
            CIPlatform::Semaphore => "Semaphore",
            CIPlatform::Jenkins => "Jenkins",
            CIPlatform::Bamboo => "Bamboo",
            CIPlatform::TFS => "Team Foundation Server",
            CIPlatform::TeamCity => "TeamCity",
            CIPlatform::Buildkite => "Buildkite",
            CIPlatform::Hudson => "Hudson",
            CIPlatform::TaskCluster => "TaskCluster",
            CIPlatform::GoCD => "GoCD",
            CIPlatform::BitBucket => "BitBucket",
            CIPlatform::GitHubActions => "GitHub Actions",
            CIPlatform::Generic => "Generic CI",
        })
    }
}
