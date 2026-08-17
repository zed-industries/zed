// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Support for linking against a system-installed AWS-LC library.
//!
//! Activated when `<crate>_SYSTEM_DIR` (e.g. `AWS_LC_SYS_SYSTEM_DIR`) is set.
//! Two entry points are invoked from the build script: `check_dependencies`
//! locates and resolves the libraries, and `link()` then validates the
//! include directory, checks the installed version against the bundled
//! one, locates the bindings, copies them into `OUT_DIR`, and emits the
//! appropriate `cargo:` directives.

use crate::fips_probe::verify_fips_install;
use crate::{
    crate_env_var_name, emit_rustc_cfg, emit_warning, is_fips_build, is_static_library,
    link_fips_runtime_check, out_dir, target_env, target_os, Builder, OutputLibType,
};
use std::cell::RefCell;
use std::path::{Path, PathBuf};

/// Resolved on-disk locations for a system AWS-LC.
///
/// Prefix-based discovery (explicit `*_SYSTEM_DIR`, `OPENSSL_DIR`, default
/// prefixes) derives every field from a single install prefix via
/// [`InstallLayout::from_prefix`]. Sources that yield *independent* locations
/// — pkg-config, the `OPENSSL_INCLUDE_DIR`/`OPENSSL_LIB_DIR` split, multiarch
/// library directories such as `/usr/lib/<triple>` — populate the fields
/// directly via [`InstallLayout::from_paths`].
pub(crate) struct InstallLayout {
    /// Directory passed to the compiler with `-I`; contains `openssl/base.h`.
    include_dir: PathBuf,
    /// Library search directories, highest priority first.
    lib_dirs: Vec<PathBuf>,
    /// Prefix under which conventional bindings live
    /// (`<prefix>/share/rust/aws_lc_bindings.rs`), when discovery could
    /// determine one. `None` means only an explicit `SYSTEM_BINDINGS` override
    /// can supply bindings.
    bindings_prefix: Option<PathBuf>,
    /// Representative directory used in diagnostics and recorded as the active
    /// system install once adopted: the install prefix when known, otherwise
    /// the include directory.
    marker_dir: PathBuf,
}

impl InstallLayout {
    /// Layout for a conventional single-prefix install: headers at
    /// `<prefix>/include`, libraries under `<prefix>/lib64` (preferred on
    /// 64-bit targets when present) then `<prefix>/lib`, bindings under
    /// `<prefix>/share/rust`.
    pub(crate) fn from_prefix(prefix: PathBuf) -> Self {
        let include_dir = prefix.join("include");
        let mut lib_dirs = Vec::new();
        // Cargo always sets CARGO_CFG_TARGET_POINTER_WIDTH for build scripts.
        let target_is_64bit =
            std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH").is_ok_and(|w| w == "64");
        let lib64 = prefix.join("lib64");
        if target_is_64bit && lib64.exists() {
            lib_dirs.push(lib64);
        }
        lib_dirs.push(prefix.join("lib"));
        Self {
            include_dir,
            lib_dirs,
            bindings_prefix: Some(prefix.clone()),
            marker_dir: prefix,
        }
    }

    /// Layout for independently-located headers and libraries. `bindings_prefix`
    /// is where `share/rust/aws_lc_bindings.rs` is expected (e.g. derived from a
    /// pkg-config include path or the `OPENSSL_INCLUDE_DIR` install layout).
    pub(crate) fn from_paths(
        include_dir: PathBuf,
        lib_dirs: Vec<PathBuf>,
        bindings_prefix: Option<PathBuf>,
    ) -> Self {
        let marker_dir = bindings_prefix
            .clone()
            .unwrap_or_else(|| include_dir.clone());
        Self {
            include_dir,
            lib_dirs,
            bindings_prefix,
            marker_dir,
        }
    }
}

pub(crate) struct SystemLib {
    pub manifest_dir: PathBuf,
    layout: InstallLayout,
    pub bindings_file: Option<PathBuf>,
    pub skip_version_check: bool,
    pub crypto_lib: RefCell<Option<ResolvedLib>>,
    pub ssl_lib: RefCell<Option<ResolvedLib>>,
    /// Memoizes the bindings path resolved by `validate`, which doubles as a
    /// "validation already succeeded" marker. Auto-detection probes an install
    /// (running `validate`) and, on success, links the *same* instance; this
    /// cache prevents the subsequent `link` from repeating the version check,
    /// bindings lookup, and — most importantly — re-compiling and re-running
    /// the FIPS probe binary.
    validated_bindings: RefCell<Option<PathBuf>>,
}

impl SystemLib {
    /// Constructs a `SystemLib` from a single install prefix. Used by the
    /// explicit `*_SYSTEM_DIR` path and by prefix-based auto-detection.
    pub(crate) fn new(
        manifest_dir: PathBuf,
        install_dir: PathBuf,
        bindings_file: Option<PathBuf>,
        skip_version_check: bool,
    ) -> Self {
        Self::from_layout(
            manifest_dir,
            InstallLayout::from_prefix(install_dir),
            bindings_file,
            skip_version_check,
        )
    }

    /// Constructs a `SystemLib` from an already-resolved [`InstallLayout`].
    /// Used by auto-detection sources that locate headers and libraries
    /// independently (pkg-config, `OPENSSL_INCLUDE_DIR`/`OPENSSL_LIB_DIR`).
    pub(crate) fn from_layout(
        manifest_dir: PathBuf,
        layout: InstallLayout,
        bindings_file: Option<PathBuf>,
        skip_version_check: bool,
    ) -> Self {
        Self {
            manifest_dir,
            layout,
            bindings_file,
            skip_version_check,
            crypto_lib: RefCell::new(None),
            ssl_lib: RefCell::new(None),
            validated_bindings: RefCell::new(None),
        }
    }

    /// The directory to record (via `set_system_dir`) once this install is
    /// adopted, so downstream consumers of `get_system_dir_path` agree a system
    /// library is in use.
    pub(crate) fn marker_dir(&self) -> &Path {
        &self.layout.marker_dir
    }
}

/// A resolved library file: the name to pass to the linker and its on-disk path.
pub(crate) struct ResolvedLib {
    pub lib_type: OutputLibType,
    /// The bare library name passed to `rustc-link-lib` (e.g. `"crypto"`).
    pub name: String,
    /// Full path to the library file (used for `rerun-if-changed`).
    pub path: PathBuf,
}

impl Builder for SystemLib {
    fn check_dependencies(&self) -> Result<(), String> {
        let requested_lib_type = is_static_library().map(|stc| {
            if stc {
                OutputLibType::Static
            } else {
                OutputLibType::Dynamic
            }
        });
        // Resolve crypto and (optionally) ssl in the same lib_dir, so we don't
        // commit to a lib_dir for crypto only to discover ssl is missing there
        // when it lives in the alternate dir.
        //
        // Track the most-informative error we encounter so that when no
        // candidate lib_dir works we surface the actionable resolver error
        let mut last_err: Option<String> = None;
        for lib_dir in &self.layout.lib_dirs {
            let crypto_lib = match resolve_library(
                lib_dir.as_path(),
                requested_lib_type,
                "crypto",
                CRYPTO_LIB_CANDIDATES,
            ) {
                Ok(lib) => lib,
                Err(e) => {
                    last_err = Some(e);
                    continue;
                }
            };
            if cfg!(feature = "ssl") {
                let ssl_lib = match resolve_library(
                    lib_dir.as_path(),
                    requested_lib_type,
                    "ssl",
                    SSL_LIB_CANDIDATES,
                ) {
                    Ok(lib) => lib,
                    Err(e) => {
                        last_err = Some(e);
                        continue;
                    }
                };
                self.ssl_lib.replace(Some(ssl_lib));
            }
            self.crypto_lib.replace(Some(crypto_lib));
            return Ok(());
        }
        Err(last_err.unwrap_or_else(|| {
            format!(
                "AWS-LC libcrypto{ssl} not found under {dirs}",
                ssl = if cfg!(feature = "ssl") { "/libssl" } else { "" },
                dirs = self
                    .layout
                    .lib_dirs
                    .iter()
                    .map(|d| d.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }))
    }

    fn build(&self) -> Result<(), String> {
        self.link()
    }

    fn name(&self) -> &'static str {
        "System Library"
    }
}

impl SystemLib {
    /// Performs every *non-emitting* check required before linking against the
    /// resolved install, returning the path to the pre-generated bindings to
    /// use. In order, it: confirms the include directory exists, verifies the
    /// AWS-LC marker and enforces the version / FIPS-version floor (unless
    /// skipped), locates the bindings, and — for FIPS builds — verifies the
    /// FIPS module.
    ///
    /// This is the single source of truth for "is this install suitable?".
    /// Both the fatal explicit path (via `link`) and the non-fatal
    /// auto-detection path (via `probe`) route through here, so the two can
    /// never diverge — a probe that succeeds guarantees the subsequent link
    /// will not fail validation. The result is memoized so a probe followed by
    /// a link does not repeat the work (notably the FIPS probe).
    ///
    /// `check_dependencies` must have run first to resolve the libraries.
    fn validate(&self) -> Result<PathBuf, String> {
        if let Some(bindings) = self.validated_bindings.borrow().clone() {
            return Ok(bindings);
        }

        let crypto_lib = self.crypto_lib.borrow();
        let crypto_lib = crypto_lib
            .as_ref()
            .expect("check_dependencies must run first");
        let lib_dir = crypto_lib
            .path
            .parent()
            .ok_or("libcrypto parent directory not found")?;
        let include_dir = &self.layout.include_dir;
        if !include_dir.exists() {
            return Err(format!(
                "Include directory not found: {}\n\
                 Verify the AWS-LC headers exist (via {}, OPENSSL_DIR, or \
                 OPENSSL_INCLUDE_DIR).",
                include_dir.display(),
                crate_env_var_name("SYSTEM_DIR"),
            ));
        }

        if self.skip_version_check {
            emit_warning("Skipping AWS-LC version compatibility check.");
            read_and_validate_base_h(include_dir)?;
        } else if is_fips_build() {
            // FIPS gates on the FIPS module version (see MINIMUM_FIPS_VERSION),
            // not the library version string.
            let fips_version = validate_and_resolve_fips_version(include_dir)?;
            if fips_version < MINIMUM_FIPS_VERSION {
                return Err(format!(
                    "AWS-LC FIPS module version too old: installed FIPS version {fips_version} < \
                     minimum supported {MINIMUM_FIPS_VERSION}.\n\
                     Please use a newer AWS-LC FIPS build or unset {} to build from source.\n\
                     To bypass this check (not recommended), set {}=1",
                    crate_env_var_name("SYSTEM_DIR"),
                    crate_env_var_name("SYSTEM_SKIP_VERSION_CHECK"),
                ));
            }
        } else {
            let version = validate_and_extract_version(include_dir)?;
            if !version_at_least(&version, MINIMUM_AWS_LC_VERSION)? {
                return Err(format!(
                    "AWS-LC version too old: installed {version} < minimum supported \
                     {MINIMUM_AWS_LC_VERSION}.\n\
                     Please upgrade AWS-LC or unset {} to build from source.\n\
                     To bypass this check (not recommended), set {}=1",
                    crate_env_var_name("SYSTEM_DIR"),
                    crate_env_var_name("SYSTEM_SKIP_VERSION_CHECK"),
                ));
            }
        }

        let bindings =
            resolve_bindings(self.layout.bindings_prefix.as_deref(), &self.bindings_file)?;

        if is_fips_build() {
            // Verify the supplied FIPS library is a usable FIPS module before
            // committing to it. The matching startup self-check link directive
            // is emitted later, in `link`.
            verify_fips_install(
                self.manifest_dir.as_path(),
                include_dir,
                crypto_lib,
                lib_dir,
            )?;
        }

        self.validated_bindings.replace(Some(bindings.clone()));
        Ok(bindings)
    }

    /// Non-fatally checks whether this install is suitable to link against. It
    /// resolves the libraries and runs the exact same validation `link` relies
    /// on, but emits no `cargo:` directives. Auto-detection treats any `Err`
    /// as "not suitable — fall back to a source build" rather than a hard
    /// error.
    pub(crate) fn probe(&self) -> Result<(), String> {
        self.check_dependencies()?;
        self.validate().map(|_| ())
    }

    /// Emits the `cargo:` directives needed to link against the
    /// system-installed AWS-LC.
    pub(crate) fn link(&self) -> Result<(), String> {
        let bindings = self.validate()?;

        let crypto_lib = self.crypto_lib.borrow();
        let crypto_lib = crypto_lib
            .as_ref()
            .expect("check_dependencies must run first");
        let lib_dir = crypto_lib
            .path
            .parent()
            .ok_or("libcrypto parent directory not found")?;
        let include_dir = &self.layout.include_dir;

        std::fs::copy(&bindings, out_dir().join("bindings.rs"))
            .map_err(|e| format!("Failed to copy bindings from {}: {}", bindings.display(), e))?;
        emit_warning(format!(
            "Using pre-generated bindings from: {}",
            bindings.display()
        ));
        emit_rustc_cfg("use_bindgen_pregenerated");

        emit_warning(format!(
            "Using system-installed AWS-LC from: {}",
            self.layout.marker_dir.display()
        ));

        if is_fips_build() {
            // The FIPS module itself was already verified in `validate`; here
            // we only add the startup self-check.
            link_fips_runtime_check(self.manifest_dir.as_path(), include_dir)?;
        }

        self.emit_link_directives(crypto_lib, include_dir, lib_dir);
        Ok(())
    }

    fn emit_link_directives(&self, crypto_lib: &ResolvedLib, include_dir: &Path, lib_dir: &Path) {
        let optional_ssl_lib = self.ssl_lib.borrow();
        let kind = crypto_lib.lib_type.rust_lib_type();
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:libcrypto={}", crypto_lib.name);
        println!("cargo:rustc-link-lib={kind}={}", crypto_lib.name);
        if let Some(ssl_lib) = optional_ssl_lib.as_ref() {
            println!("cargo:rustc-link-lib={kind}={}", ssl_lib.name);
            println!("cargo:libssl={}", ssl_lib.name);
            println!("cargo:rerun-if-changed={}", ssl_lib.path.display());
        }

        println!("cargo:include={}", include_dir.display());

        println!("cargo:rerun-if-changed={}", include_dir.display());
        println!("cargo:rerun-if-changed={}", crypto_lib.path.display());
    }
}

// =============================================================================
// Header validation / version extraction
// =============================================================================

/// Minimum AWS-LC (mainline) version supported via the system-library path,
/// for the non-FIPS `aws-lc-sys` crate. `1.68.0` is the first release that can
/// emit Rust bindings (`-DGENERATE_RUST_BINDINGS=ON`), which the
/// system-library path consumes. A declared floor (not derived from the
/// bundled submodule) whose support is proven by CI; bump it and the CI pin
/// together. See `.github/workflows/system-lib-tests.yml`.
const MINIMUM_AWS_LC_VERSION: &str = "1.68.0";

/// Minimum AWS-LC FIPS *module* version supported via the system-library path,
/// for `aws-lc-fips-sys`. The FIPS version (aws/aws-lc#3211) is decoupled from
/// the library version and increases monotonically across FIPS branches, so it
/// is a stable basis for comparison.
///
/// TODO: bump to `4` when switching to the `fips-2025-09-12-lts` branch.
const MINIMUM_FIPS_VERSION: u32 = 3;

/// Finds the first `#define <name> ...` line in `base.h` content and returns
/// the value token following the macro name (or `""` for a bare `#define
/// <name>` with no value), or `None` if the macro is not `#define`d. Matching
/// on the leading whitespace-separated tokens — rather than a bare `contains`
/// — prevents a comment that merely mentions the macro name from
/// false-matching.
fn find_define<'a>(content: &'a str, name: &str) -> Option<&'a str> {
    content.lines().find_map(|line| {
        let mut tokens = line.split_whitespace();
        if tokens.next() == Some("#define") && tokens.next() == Some(name) {
            // Map a valueless `#define <name>` to "" so callers still
            // distinguish "defined" (`Some`) from "absent" (`None`).
            Some(tokens.next().unwrap_or(""))
        } else {
            None
        }
    })
}

/// Reads `<include_dir>/openssl/base.h`, verifies the `OPENSSL_IS_AWSLC` marker
/// (rejecting `OpenSSL`/`BoringSSL`), and returns the header path and contents.
fn read_and_validate_base_h(include_dir: &Path) -> Result<(PathBuf, String), String> {
    let base_h = include_dir.join("openssl").join("base.h");
    let content = std::fs::read_to_string(&base_h)
        .map_err(|e| format!("Failed to read {}: {}", base_h.display(), e))?;

    let has_awslc_marker = find_define(&content, "OPENSSL_IS_AWSLC").is_some();
    if !has_awslc_marker {
        return Err(format!(
            "Headers at {} are not valid AWS-LC headers.\n\
             The OPENSSL_IS_AWSLC marker was not found in base.h.\n\
             Ensure the path contains AWS-LC headers, not OpenSSL or BoringSSL.",
            include_dir.display()
        ));
    }

    Ok((base_h, content))
}

/// Validates the headers are AWS-LC and returns the library version string
/// (`AWSLC_VERSION_NUMBER_STRING`).
fn validate_and_extract_version(include_dir: &Path) -> Result<String, String> {
    let (base_h, content) = read_and_validate_base_h(include_dir)?;
    extract_version(&base_h, &content)
}

/// Validates the headers are AWS-LC and returns the FIPS module version.
fn validate_and_resolve_fips_version(include_dir: &Path) -> Result<u32, String> {
    let (base_h, content) = read_and_validate_base_h(include_dir)?;
    resolve_fips_version(&base_h, &content)
}

/// Extracts `AWSLC_VERSION_NUMBER_STRING` from already-loaded `base.h`
/// content. Kept separate from `validate_and_extract_version` so the
/// bundled-version guard test can call it directly, skipping the
/// `OPENSSL_IS_AWSLC` marker check (the bundled headers are AWS-LC by
/// construction).
fn extract_version(base_h: &Path, content: &str) -> Result<String, String> {
    let Some(value) = find_define(content, "AWSLC_VERSION_NUMBER_STRING") else {
        return Err(format!(
            "Could not find AWSLC_VERSION_NUMBER_STRING in {}.",
            base_h.display()
        ));
    };
    let trimmed = value.trim_matches('"');
    if trimmed != value && !trimmed.is_empty() {
        Ok(trimmed.to_string())
    } else {
        Err(format!(
            "Malformed AWSLC_VERSION_NUMBER_STRING in {}",
            base_h.display()
        ))
    }
}

fn version_at_least(installed: &str, required: &str) -> Result<bool, String> {
    let parse = |s: &str| -> Result<(u32, u32, u32), String> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid version format: {s}"));
        }
        Ok((
            parts[0]
                .parse()
                .map_err(|_| format!("Invalid major: {}", parts[0]))?,
            parts[1]
                .parse()
                .map_err(|_| format!("Invalid minor: {}", parts[1]))?,
            parts[2]
                .parse()
                .map_err(|_| format!("Invalid patch: {}", parts[2]))?,
        ))
    };
    Ok(parse(installed)? >= parse(required)?)
}

/// Resolves the FIPS module version from `base.h`: prefers the
/// `AWSLC_FIPS_VERSION_NUMBER` macro (FIPS 4.x+ / mainline, aws/aws-lc#3211),
/// falling back to the major of `AWSLC_VERSION_NUMBER_STRING` on legacy FIPS
/// branches (<= 3.x) that predate the macro.
fn resolve_fips_version(base_h: &Path, content: &str) -> Result<u32, String> {
    if let Some(version) = extract_fips_version_number(content)? {
        return Ok(version);
    }
    let version = extract_version(base_h, content)?;
    version_major(&version)
}

/// Extracts the integer `AWSLC_FIPS_VERSION_NUMBER` macro.
///
/// Returns `Ok(None)` when the macro is absent (a legacy FIPS branch that
/// predates it; callers fall back to the version-string major). A macro that is
/// *present but unparseable* is a hard error rather than a silent fallback:
/// post-decoupling (aws/aws-lc#3211) the library version is not a reliable
/// substitute, so masking a malformed authoritative value would be wrong.
fn extract_fips_version_number(content: &str) -> Result<Option<u32>, String> {
    let Some(value) = find_define(content, "AWSLC_FIPS_VERSION_NUMBER") else {
        return Ok(None);
    };
    value.parse::<u32>().map(Some).map_err(|_| {
        let shown = if value.is_empty() { "<empty>" } else { value };
        format!("Malformed AWSLC_FIPS_VERSION_NUMBER: expected an unsigned integer, found {shown}")
    })
}

/// Parses the MAJOR component of a `MAJOR.MINOR.PATCH` version string.
fn version_major(version: &str) -> Result<u32, String> {
    version
        .split('.')
        .next()
        .and_then(|major| major.parse::<u32>().ok())
        .ok_or_else(|| format!("Invalid version format: {version}"))
}

// =============================================================================
// Library resolution
// =============================================================================

/// Candidate bare library names for libcrypto, in preference order.
///
/// `crypto` covers plain AWS-LC installs (the common case). `crypto-awslc`
/// covers installs produced with `ENABLE_DIST_PKG=ON` or a non-Apple Unix
/// shared build that disables `ENABLE_PRE_SONAME_BUILD`, where AWS-LC's
/// `CMake` appends the `-awslc` SONAME suffix to the library file name.
///
/// Note: the `BORINGSSL_PREFIX` symbol-prefixing feature does *not* rename
/// the library file in upstream AWS-LC's build — it only renames the
/// symbols inside. Do not try to apply the symbol prefix here.
const CRYPTO_LIB_CANDIDATES: &[&str] = &["crypto", "crypto-awslc"];

/// Candidate bare library names for libssl, in preference order.
/// See `CRYPTO_LIB_CANDIDATES` for the rationale.
const SSL_LIB_CANDIDATES: &[&str] = &["ssl", "ssl-awslc"];

/// Resolves a library, choosing the linkage form based on `requested_lib_type`.
///
/// `lib_kind` is the human-readable name used in error messages
/// (e.g. `"crypto"` or `"ssl"`).
///
/// When `requested_lib_type` is `Some`, the user has set `AWS_LC_SYS_STATIC`
/// explicitly and the requested form is required: if only the other form
/// is present we return an error rather than silently falling back. When
/// `requested_lib_type` is `None`, we prefer the form picked by
/// `OutputLibType::default()` but fall back to whichever form is available.
fn resolve_library(
    lib_dir: &Path,
    requested_lib_type: Option<OutputLibType>,
    lib_kind: &str,
    candidate_names: &[&str],
) -> Result<ResolvedLib, String> {
    let preferred = requested_lib_type.unwrap_or_default();

    if let Some(found) = find_candidate(lib_dir, preferred, candidate_names) {
        return Ok(found);
    }

    // Preferred form not present. Probe the opposite form so we can give
    // an accurate diagnostic: "wrong form on disk" vs "nothing on disk".
    let opposite = preferred.opposite();
    let opposite_match = find_candidate(lib_dir, opposite, candidate_names);

    if requested_lib_type.is_some() {
        // User explicitly requested a form that isn't available; do not
        // silently fall back.
        if opposite_match.is_some() {
            return Err(format!(
                "{var} is set to request {requested} linking, but only a {got} {lib_kind} library was found in {}.\n\
                 Expected one of: {}\n\
                 Provide the requested form at this location, or unset {var} to allow \
                 the available form.",
                lib_dir.display(),
                expected_lib_filenames(candidate_names, Some(preferred)),
                requested = preferred.description(),
                got = opposite.description(),
                var = crate_env_var_name("STATIC"),
            ));
        }
        return Err(format!(
            "{var} is set to request {requested} linking, but no {lib_kind} library was found in {}.\n\
             Expected one of: {}",
            lib_dir.display(),
            expected_lib_filenames(candidate_names, Some(preferred)),
            requested = preferred.description(),
            var = crate_env_var_name("STATIC"),
        ));
    }

    if let Some(found) = opposite_match {
        let desc = opposite.description();
        emit_warning(format!(
            "Only {desc} {lib_kind} library found in system install; using {desc} linking."
        ));
        return Ok(found);
    }

    Err(format!(
        "No {lib_kind} library found in {}\n\
         Expected one of: {}",
        lib_dir.display(),
        expected_lib_filenames(candidate_names, None),
    ))
}

/// Returns the first candidate in `candidates` whose library of the given
/// `lib_type` exists in `lib_dir`.
fn find_candidate(
    lib_dir: &Path,
    lib_type: OutputLibType,
    candidates: &[&str],
) -> Option<ResolvedLib> {
    candidates.iter().find_map(|name| {
        probe_lib(lib_dir, name, lib_type).map(|path| ResolvedLib {
            lib_type,
            name: (*name).to_string(),
            path,
        })
    })
}

/// Returns `true` when the target is Windows with the MSVC toolchain.
fn is_msvc() -> bool {
    matches!(
        (target_os().as_str(), target_env().as_str()),
        ("windows", "msvc")
    )
}

/// Probes for a library named `name` of the requested `lib_type` in `lib_dir`.
///
/// On MSVC, both static archives and DLL import libraries are named
/// `{name}.lib`. The two are disambiguated by `msvc_has_sibling_dll`: if
/// `../bin/{name}.dll` exists, the `.lib` is an import library (dynamic);
/// otherwise it is a real static archive.
fn probe_lib(lib_dir: &Path, name: &str, lib_type: OutputLibType) -> Option<PathBuf> {
    let want_dynamic = matches!(lib_type, OutputLibType::Dynamic);
    let path = lib_dir.join(lib_filename(name, lib_type));
    if !path.exists() {
        return None;
    }
    // MSVC: `{name}.lib` could be either a real static archive or a DLL
    // import library. Classify by sibling-DLL presence.
    if is_msvc() && msvc_has_sibling_dll(lib_dir, name) != want_dynamic {
        return None;
    }
    Some(path)
}

/// Returns the platform-specific filename for a library named `base` of the
/// given linkage type.
///
/// On MSVC, both real static archives and DLL import libraries are named
/// `{base}.lib`, so this function returns that name for either linkage. The
/// two are disambiguated at probe time inside `probe_lib` via the sibling
/// `../bin/{base}.dll` check.
fn lib_filename(base: &str, lib_type: OutputLibType) -> String {
    if is_msvc() {
        return format!("{base}.lib");
    }

    match lib_type {
        OutputLibType::Static => format!("lib{base}.a"),
        OutputLibType::Dynamic => match target_os().as_str() {
            // MinGW: the runtime DLL lives in bin/, not lib_dir. The import
            // library lib{base}.dll.a is the only artifact in lib_dir.
            "windows" => format!("lib{base}.dll.a"),
            "macos" | "ios" | "tvos" => format!("lib{base}.dylib"),
            _ => format!("lib{base}.so"),
        },
    }
}

/// Comma-separated list of platform-appropriate filenames the resolver looks
/// for. Used purely for error messages. Note that on MSVC `lib_filename`
/// produces the same `{base}.lib` for either linkage, so the `filter`
/// argument doesn't change the resulting list there (deduplicated below).
fn expected_lib_filenames(candidates: &[&str], filter: Option<OutputLibType>) -> String {
    let lib_types: &[OutputLibType] = match filter {
        Some(OutputLibType::Static) => &[OutputLibType::Static],
        Some(OutputLibType::Dynamic) => &[OutputLibType::Dynamic],
        None => &[OutputLibType::Static, OutputLibType::Dynamic],
    };
    let mut names: Vec<String> = Vec::new();
    for &lt in lib_types {
        for base in candidates {
            let name = lib_filename(base, lt);
            if !names.contains(&name) {
                names.push(name);
            }
        }
    }
    names.join(", ")
}

/// Returns `true` when a sibling `../bin/{name}.dll` exists relative to
/// `lib_dir`. `CMake` installs the runtime DLL under `bin/` while the import
/// library goes to `lib/` on MSVC, so this is the canonical way to detect
/// that the `.lib` in `lib_dir` is an import library rather than a real
/// static archive.
fn msvc_has_sibling_dll(lib_dir: &Path, name: &str) -> bool {
    lib_dir
        .parent()
        .is_some_and(|root| root.join("bin").join(format!("{name}.dll")).exists())
}

// =============================================================================
// Bindings resolution
// =============================================================================

/// Locates pre-generated bindings, either from the explicit override or the
/// conventional install location populated by AWS-LC's `CMake` install (see
/// <https://github.com/aws/aws-lc/pull/2999>, AWS-LC v1.68.0+).
///
/// `bindings_prefix` is the install prefix under which `share/rust/` is
/// searched; it is `None` for discovery sources that couldn't determine a
/// prefix (in which case only an explicit override can supply bindings).
///
/// A misconfigured override is a hard error rather than a silent fall-through,
/// since the user clearly asked for those specific bindings.
fn resolve_bindings(
    bindings_prefix: Option<&Path>,
    bindings_override: &Option<PathBuf>,
) -> Result<PathBuf, String> {
    if let Some(path) = bindings_override {
        if path.is_file() {
            return Ok(path.clone());
        }
        return Err(format!(
            "{} does not point to a file: {}",
            crate_env_var_name("SYSTEM_BINDINGS"),
            path.display()
        ));
    }

    let Some(prefix) = bindings_prefix else {
        return Err(format!(
            "No pre-generated bindings found for system-installed AWS-LC.\n\
             The detected install did not expose an install prefix from which to\n\
             locate share/rust/aws_lc_bindings.rs. Set {} to point at a bindings file.",
            crate_env_var_name("SYSTEM_BINDINGS"),
        ));
    };

    let conventional = prefix.join("share").join("rust").join("aws_lc_bindings.rs");
    if conventional.is_file() {
        return Ok(conventional);
    }

    Err(format!(
        "No pre-generated bindings found for system-installed AWS-LC.\n\n\
         To resolve this, either:\n\
         1. Install AWS-LC with Rust bindings (share/rust/aws_lc_bindings.rs) under {}, or\n\
         2. Set {} to point to a bindings file",
        prefix.display(),
        crate_env_var_name("SYSTEM_BINDINGS"),
    ))
}

#[cfg(test)]
#[path = "system_library_tests.rs"]
mod tests;
