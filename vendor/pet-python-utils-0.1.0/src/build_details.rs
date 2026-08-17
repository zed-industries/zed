// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Reads `build-details.json` ([PEP 739]) installed alongside a Python
//! interpreter so we can extract version (and other build metadata) without
//! having to spawn the interpreter itself.
//!
//! Starting with Python 3.14 the file is shipped in the platform-independent
//! standard library directory:
//!
//! * Unix: `<prefix>/lib/python<X.Y>/build-details.json`
//! * Windows: `<prefix>/Lib/build-details.json`
//!
//! See <https://packaging.python.org/en/latest/specifications/build-details/>.
//!
//! [PEP 739]: https://peps.python.org/pep-0739/

use lazy_static::lazy_static;
use log::{trace, warn};
use regex::Regex;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

const BUILD_DETAILS_FILE: &str = "build-details.json";

lazy_static! {
    /// Matches stdlib directory names like `python3.14`, `python3.14t`, `pypy3.10`.
    /// Deliberately does *not* match `python3` (versionless) or `python-config`.
    static ref STDLIB_DIRNAME: Regex =
        Regex::new(r"^(python|pypy)(\d+)\.(\d+)t?$").expect("invalid stdlib regex");
}

/// Subset of the `build-details.json` schema (v1.0) that we currently consume.
///
/// Fields we don't use are intentionally omitted. `serde` ignores unknown
/// fields by default, so adding more later is non-breaking.
#[derive(Debug, Deserialize)]
pub struct BuildDetails {
    pub schema_version: String,
    #[serde(default)]
    pub base_prefix: Option<String>,
    #[serde(default)]
    pub base_interpreter: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    pub language: Language,
    #[serde(default)]
    pub implementation: Option<Implementation>,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    /// `X.Y` style version, e.g. `"3.14"`.
    pub version: String,
    pub version_info: VersionInfo,
}

#[derive(Debug, Deserialize)]
pub struct Implementation {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionInfo {
    pub major: u64,
    pub minor: u64,
    pub micro: u64,
    /// One of `alpha`, `beta`, `candidate`, `final`.
    pub releaselevel: String,
    pub serial: u64,
}

impl BuildDetails {
    /// Find and parse `build-details.json` for the given sys.prefix-like
    /// directory. The argument may be the prefix root or its `bin`/`Scripts`
    /// child; both are accepted for convenience.
    pub fn find(prefix: &Path) -> Option<Self> {
        Self::find_with_hint(prefix, None)
    }

    /// Like [`Self::find`], but if multiple candidate stdlib directories exist
    /// (e.g. `lib/python3.10` and `lib/python3.14`), prefers the one whose
    /// `(major, minor)` matches the supplied hint. Useful when callers already
    /// know the expected version (e.g. from `pyvenv.cfg`).
    pub fn find_with_hint(prefix: &Path, pyver: Option<(u64, u64)>) -> Option<Self> {
        let file = find_file(prefix, pyver)?;
        parse(&file)
    }

    /// Returns a CPython-style version string built from `version_info`.
    ///
    /// Examples: `"3.14.0"`, `"3.14.0a0"`, `"3.14.0b1"`, `"3.14.0rc2"`.
    /// This matches the `PY_VERSION` literal that `patchlevel.h` exposes.
    pub fn version_string(&self) -> String {
        let v = &self.language.version_info;
        let suffix = match v.releaselevel.as_str() {
            "alpha" => format!("a{}", v.serial),
            "beta" => format!("b{}", v.serial),
            "candidate" => format!("rc{}", v.serial),
            "final" => String::new(),
            other => {
                // Schema documents only the four levels above. Be loud if the
                // upstream spec ever grows a new one so we can update this code
                // instead of silently producing a misleading version string.
                warn!(
                    "build-details.json has unknown releaselevel {:?}; treating as final",
                    other
                );
                String::new()
            }
        };
        format!("{}.{}.{}{}", v.major, v.minor, v.micro, suffix)
    }
}

fn find_file(prefix: &Path, pyver: Option<(u64, u64)>) -> Option<PathBuf> {
    let prefix = strip_bin(prefix);

    // Windows-style: <prefix>/Lib/build-details.json
    let win_path = prefix.join("Lib").join(BUILD_DETAILS_FILE);
    if win_path.is_file() {
        return Some(win_path);
    }

    let lib_dir = prefix.join("lib");

    // Fast path: if we have a (major, minor) hint, probe the expected paths
    // directly and skip the directory scan entirely.
    if let Some((major, minor)) = pyver {
        for impl_prefix in ["python", "pypy"] {
            for suffix in ["", "t"] {
                let candidate = lib_dir
                    .join(format!("{impl_prefix}{major}.{minor}{suffix}"))
                    .join(BUILD_DETAILS_FILE);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }

    // Slow path: enumerate `lib/` and pick deterministically (highest
    // `(major, minor)`) so multi-version prefixes don't depend on `read_dir`
    // iteration order.
    let entries = fs::read_dir(&lib_dir).ok()?;
    let mut best: Option<(u64, u64, PathBuf)> = None;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let captures = match STDLIB_DIRNAME.captures(name) {
            Some(c) => c,
            None => continue,
        };
        let candidate = path.join(BUILD_DETAILS_FILE);
        if !candidate.is_file() {
            continue;
        }
        // `unwrap()`s here are safe: the regex guarantees both groups are
        // present and contain only digits, so `parse::<u64>` cannot fail.
        let major: u64 = captures[2].parse().unwrap();
        let minor: u64 = captures[3].parse().unwrap();
        if best
            .as_ref()
            .is_none_or(|(bm, bn, _)| (major, minor) > (*bm, *bn))
        {
            best = Some((major, minor, candidate));
        }
    }
    best.map(|(_, _, path)| path)
}

fn strip_bin(prefix: &Path) -> PathBuf {
    if let Some(name) = prefix.file_name().and_then(|n| n.to_str()) {
        if name.eq_ignore_ascii_case("bin") || name.eq_ignore_ascii_case("Scripts") {
            if let Some(parent) = prefix.parent() {
                return parent.to_path_buf();
            }
        }
    }
    prefix.to_path_buf()
}

fn parse(file: &Path) -> Option<BuildDetails> {
    let contents = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to read {} at {:?}: {}", BUILD_DETAILS_FILE, file, e);
            return None;
        }
    };
    let bd: BuildDetails = match serde_json::from_str(&contents) {
        Ok(bd) => bd,
        Err(e) => {
            warn!(
                "Failed to parse {} at {:?}: {}",
                BUILD_DETAILS_FILE, file, e
            );
            return None;
        }
    };
    // We only understand schema 1.x. Reject anything else so a future,
    // potentially-incompatible schema bump doesn't cause us to silently emit
    // wrong version strings.
    if !is_supported_schema(&bd.schema_version) {
        warn!(
            "Unsupported {} schema version {:?} at {:?}; ignoring",
            BUILD_DETAILS_FILE, bd.schema_version, file
        );
        return None;
    }
    trace!("Parsed {} at {:?}", BUILD_DETAILS_FILE, file);
    Some(bd)
}

fn is_supported_schema(version: &str) -> bool {
    matches!(version.split('.').next(), Some("1"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    fn write(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut f = fs::File::create(path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }

    const SAMPLE_FINAL: &str = r#"{
        "schema_version": "1.0",
        "base_prefix": "/usr",
        "base_interpreter": "/usr/bin/python",
        "platform": "linux-x86_64",
        "language": {
            "version": "3.14",
            "version_info": {
                "major": 3,
                "minor": 14,
                "micro": 1,
                "releaselevel": "final",
                "serial": 0
            }
        },
        "implementation": {
            "name": "cpython"
        }
    }"#;

    const SAMPLE_ALPHA: &str = r#"{
        "schema_version": "1.0",
        "language": {
            "version": "3.14",
            "version_info": {
                "major": 3,
                "minor": 14,
                "micro": 0,
                "releaselevel": "alpha",
                "serial": 0
            }
        }
    }"#;

    fn sample_with_minor(minor: u64) -> String {
        format!(
            r#"{{
                "schema_version": "1.0",
                "language": {{
                    "version": "3.{minor}",
                    "version_info": {{
                        "major": 3,
                        "minor": {minor},
                        "micro": 0,
                        "releaselevel": "final",
                        "serial": 0
                    }}
                }}
            }}"#
        )
    }

    #[test]
    fn finds_unix_layout() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        let stdlib = prefix.join("lib").join("python3.14");
        write(&stdlib.join("build-details.json"), SAMPLE_FINAL);

        let bd = BuildDetails::find(prefix).expect("should find file");
        assert_eq!(bd.schema_version, "1.0");
        assert_eq!(bd.language.version, "3.14");
        assert_eq!(bd.version_string(), "3.14.1");
        assert_eq!(
            bd.implementation.as_ref().map(|i| i.name.as_str()),
            Some("cpython")
        );
    }

    #[test]
    fn finds_windows_layout() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(&prefix.join("Lib").join("build-details.json"), SAMPLE_FINAL);

        let bd = BuildDetails::find(prefix).expect("should find file");
        assert_eq!(bd.version_string(), "3.14.1");
    }

    #[test]
    fn accepts_bin_subdirectory() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(
            &prefix
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            SAMPLE_FINAL,
        );

        let bd = BuildDetails::find(&prefix.join("bin")).expect("should find via bin");
        assert_eq!(bd.version_string(), "3.14.1");
    }

    #[test]
    fn accepts_scripts_subdirectory() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(&prefix.join("Lib").join("build-details.json"), SAMPLE_FINAL);

        let bd = BuildDetails::find(&prefix.join("Scripts")).expect("should find via Scripts");
        assert_eq!(bd.version_string(), "3.14.1");
    }

    #[test]
    fn version_string_handles_pre_releases() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(
            &prefix
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            SAMPLE_ALPHA,
        );

        let bd = BuildDetails::find(prefix).unwrap();
        assert_eq!(bd.version_string(), "3.14.0a0");
    }

    #[test]
    fn returns_none_when_missing() {
        let dir = tempdir().unwrap();
        assert!(BuildDetails::find(dir.path()).is_none());
    }

    #[test]
    fn returns_none_for_invalid_json() {
        let dir = tempdir().unwrap();
        write(
            &dir.path()
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            "{ not valid json",
        );
        assert!(BuildDetails::find(dir.path()).is_none());
    }

    #[test]
    fn ignores_non_python_lib_dirs() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(
            &prefix
                .join("lib")
                .join("ruby3.3")
                .join("build-details.json"),
            SAMPLE_FINAL,
        );
        assert!(BuildDetails::find(prefix).is_none());
    }

    #[test]
    fn ignores_versionless_python_dir() {
        // `python3` (no minor) and other look-alikes such as `python-config`
        // must not be picked up.
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(
            &prefix
                .join("lib")
                .join("python3")
                .join("build-details.json"),
            SAMPLE_FINAL,
        );
        write(
            &prefix
                .join("lib")
                .join("python-config")
                .join("build-details.json"),
            SAMPLE_FINAL,
        );
        assert!(BuildDetails::find(prefix).is_none());
    }

    #[test]
    fn hint_picks_matching_minor() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(
            &prefix
                .join("lib")
                .join("python3.10")
                .join("build-details.json"),
            &sample_with_minor(10),
        );
        write(
            &prefix
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            &sample_with_minor(14),
        );

        let bd = BuildDetails::find_with_hint(prefix, Some((3, 14))).unwrap();
        assert_eq!(bd.version_string(), "3.14.0");
        let bd = BuildDetails::find_with_hint(prefix, Some((3, 10))).unwrap();
        assert_eq!(bd.version_string(), "3.10.0");
    }

    #[test]
    fn no_hint_picks_highest_version_deterministically() {
        // Multiple stdlib dirs side by side: pick the highest (major, minor)
        // regardless of `read_dir` ordering.
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(
            &prefix
                .join("lib")
                .join("python3.10")
                .join("build-details.json"),
            &sample_with_minor(10),
        );
        write(
            &prefix
                .join("lib")
                .join("python3.15")
                .join("build-details.json"),
            &sample_with_minor(15),
        );
        write(
            &prefix
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            &sample_with_minor(14),
        );

        let bd = BuildDetails::find(prefix).unwrap();
        assert_eq!(bd.version_string(), "3.15.0");
    }

    #[test]
    fn rejects_unsupported_schema_version() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        let payload =
            SAMPLE_FINAL.replace(r#""schema_version": "1.0""#, r#""schema_version": "2.0""#);
        write(
            &prefix
                .join("lib")
                .join("python3.14")
                .join("build-details.json"),
            &payload,
        );
        assert!(BuildDetails::find(prefix).is_none());
    }

    #[test]
    fn accepts_free_threaded_dirname() {
        let dir = tempdir().unwrap();
        let prefix = dir.path();
        write(
            &prefix
                .join("lib")
                .join("python3.14t")
                .join("build-details.json"),
            SAMPLE_FINAL,
        );
        assert!(BuildDetails::find(prefix).is_some());
    }
}
