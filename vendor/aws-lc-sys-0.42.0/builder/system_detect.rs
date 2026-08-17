// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Auto-detection of a pre-existing, system-installed AWS-LC.
//!
//! Unlike the explicit `<crate>_SYSTEM_DIR` path (which treats any problem as a
//! hard error), detection is best-effort: a candidate that isn't a usable
//! AWS-LC is ignored with a diagnostic and the build falls back to compiling
//! AWS-LC from source. The `<crate>_USE_SYSTEM` control variable (handled by the
//! caller in `get_builder`) can turn "nothing found" into a hard error, or
//! disable detection entirely.
//!
//! Discovery mirrors the mechanism `openssl-sys` uses, so that an environment
//! which provides AWS-LC to `openssl-sys` (for example, a build environment
//! that supplies AWS-LC to its entire dependency closure) provides it here too,
//! in the same precedence:
//!
//! 1. `OPENSSL_DIR` — a single install prefix.
//! 2. `OPENSSL_INCLUDE_DIR` / `OPENSSL_LIB_DIR` — independent header/library
//!    directories. Either may be set on its own, with the unset half derived
//!    from `OPENSSL_DIR` (matching `openssl-sys`).
//! 3. pkg-config (`openssl`, `aws-lc`, `libcrypto`, then `libcrypto-awslc`) —
//!    only when the `OPENSSL_*` variables above didn't already point somewhere,
//!    matching `openssl-sys`'s "env vars take precedence over pkg-config"
//!    ordering while also recognizing AWS-LC's native package names.
//!
//! Probing pkg-config on the default path is safe despite not being opt-in:
//! a candidate is only *adopted* if it carries the `OPENSSL_IS_AWSLC` marker
//! and ships usable bindings, so a stray system OpenSSL (or a binding-less
//! AWS-LC) is rejected and the build falls back to source. Cross-compilation is
//! left to the `pkg_config` crate, which refuses to run unless
//! `PKG_CONFIG_ALLOW_CROSS=1` (again matching `openssl-sys`).

use crate::system_library::{InstallLayout, SystemLib};
use crate::{
    emit_warning, get_system_bindings_path, get_system_skip_version_check, optional_env_target,
    use_system,
};
use std::path::{Path, PathBuf};

#[cfg(unix)]
const PKG_CONFIG_MODULES: &[&str] = &["openssl", "aws-lc", "libcrypto", "libcrypto-awslc"];

/// Returns candidate install layouts discovered from generic locations, in
/// precedence order. Each entry is only a *candidate*: the caller probes it and
/// moves on if it isn't a usable AWS-LC.
///
/// `optional_env_target` consults the target-suffixed variant first
/// (e.g. `OPENSSL_DIR_x86_64_unknown_linux_gnu`) before the bare name, matching
/// this build script's target-env convention, and emits the appropriate
/// `rerun-if-env-changed` directives.
fn detect_candidate_layouts() -> Vec<InstallLayout> {
    let mut candidates = Vec::new();

    // Read OPENSSL_DIR once; it's a prefix candidate on its own and supplies
    // the unset half (and bindings prefix) of the include/lib split below.
    let openssl_dir = optional_env_target("OPENSSL_DIR").map(PathBuf::from);

    // 1. OPENSSL_DIR: a single install prefix.
    if let Some(prefix) = &openssl_dir {
        candidates.push(InstallLayout::from_prefix(prefix.clone()));
    }

    // 2. OPENSSL_INCLUDE_DIR / OPENSSL_LIB_DIR: independent locations. Either
    //    may be set alone; the unset half comes from OPENSSL_DIR, matching
    //    openssl-sys. A lone var with no OPENSSL_DIR is skipped.
    let include_env = optional_env_target("OPENSSL_INCLUDE_DIR").map(PathBuf::from);
    let lib_env = optional_env_target("OPENSSL_LIB_DIR").map(PathBuf::from);
    if include_env.is_some() || lib_env.is_some() {
        let include_dir = include_env.or_else(|| openssl_dir.as_ref().map(|p| p.join("include")));
        let lib_dir = lib_env.or_else(|| openssl_dir.as_ref().map(|p| p.join("lib")));
        if let (Some(include_dir), Some(lib_dir)) = (include_dir, lib_dir) {
            // Bindings come from OPENSSL_DIR when set, else from the prefix
            // inferred from the include dir (`<prefix>/include` or the
            // cohabiting `<prefix>/include/aws-lc` layout).
            let bindings_prefix = openssl_dir
                .clone()
                .or_else(|| bindings_prefix_from_include_dir(&include_dir));
            candidates.push(InstallLayout::from_paths(
                include_dir,
                vec![lib_dir],
                bindings_prefix,
            ));
        }
    }

    // 3. pkg-config, only as a fallback when the user hasn't pointed us
    //    anywhere explicitly (parity with openssl-sys: env vars over pkg-config).
    if candidates.is_empty() {
        candidates.extend(pkg_config_candidates());
    }

    candidates
}

/// Probes pkg-config for AWS-LC, trying the `openssl` module first (what
/// `openssl-sys` keys on), followed by AWS-LC's native package names, and
/// returns a candidate layout for each that resolves. The bindings prefix is
/// derived from the include path (`<prefix>/include` or
/// `<prefix>/include/aws-lc` → `<prefix>`).
///
/// `cargo_metadata(false)` is essential: this is only a *probe*, so it must not
/// emit link directives for a library we may yet reject. On adoption we re-emit
/// the link flags ourselves, so a `.pc`'s `Libs.private` is dropped, as on the
/// explicit path. Note this disables only link directives, not the pkg-config
/// crate's separate `env_metadata` (left enabled), so the probe still emits
/// `rerun-if-env-changed` for `PKG_CONFIG_PATH` and its target variants — those
/// stay tracked for rebuilds alongside the `OPENSSL_*`/`*_USE_SYSTEM` inputs.
#[cfg(unix)]
fn pkg_config_candidates() -> Vec<InstallLayout> {
    PKG_CONFIG_MODULES
        .iter()
        .filter_map(|module| {
            let library = pkg_config::Config::new()
                .cargo_metadata(false)
                .probe(module)
                .ok()?;
            // Need an explicit include path to validate the headers; a `.pc`
            // that relies on the compiler's default include dir isn't enough to
            // locate base.h reliably.
            let include_dir = library.include_paths.first()?.clone();
            let bindings_prefix = bindings_prefix_from_include_dir(&include_dir);
            Some(InstallLayout::from_paths(
                include_dir,
                library.link_paths.clone(),
                bindings_prefix,
            ))
        })
        .collect()
}

#[cfg(not(unix))]
fn pkg_config_candidates() -> Vec<InstallLayout> {
    Vec::new()
}

fn bindings_prefix_from_include_dir(include_dir: &Path) -> Option<PathBuf> {
    let parent = include_dir.parent()?;
    if include_dir.file_name().and_then(|name| name.to_str()) == Some("aws-lc")
        && parent.file_name().and_then(|name| name.to_str()) == Some("include")
    {
        return parent.parent().map(Path::to_path_buf);
    }
    Some(parent.to_path_buf())
}

/// Detects and validates a system-installed AWS-LC suitable for linking.
///
/// On success returns a fully *probed* `SystemLib` whose validation result is
/// memoized, so the subsequent `check_dependencies`/`link` performed by the
/// build script does not repeat the work (notably the FIPS probe). Returns
/// `None` when no candidate is a usable AWS-LC, signaling the caller to fall
/// back to a source build.
pub(crate) fn detect_system_awslc(manifest_dir: &Path) -> Option<SystemLib> {
    // Rejecting a candidate and falling back to source is the normal, healthy
    // outcome, so only escalate to a cargo:warning when a system install is
    // *required* (USE_SYSTEM=1); otherwise emit a plain note (shown under `-vv`)
    // to avoid spamming builds that merely have a non-AWS-LC OpenSSL around.
    let report_rejection = |message: String| {
        if use_system() == Some(true) {
            emit_warning(message);
        } else {
            println!("{message}");
        }
    };

    for layout in detect_candidate_layouts() {
        let sys = SystemLib::from_layout(
            manifest_dir.to_path_buf(),
            layout,
            get_system_bindings_path(),
            get_system_skip_version_check(),
        );
        match sys.probe() {
            Ok(()) => {
                emit_warning(format!(
                    "Auto-detected system-installed AWS-LC at: {}",
                    sys.marker_dir().display()
                ));
                return Some(sys);
            }
            Err(reason) => {
                report_rejection(format!(
                    "Ignoring candidate AWS-LC at {}: {reason}",
                    sys.marker_dir().display()
                ));
            }
        }
    }
    None
}

#[cfg(test)]
#[path = "system_detect_tests.rs"]
mod tests;
