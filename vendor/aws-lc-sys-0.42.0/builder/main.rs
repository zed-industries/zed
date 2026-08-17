// Copyright (c) 2022, Google Inc.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Needed until MSRV >= 1.70
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::ref_option)]
// Clippy can only be run on nightly toolchain
#![cfg_attr(clippy, feature(custom_inner_attributes))]
#![cfg_attr(clippy, clippy::msrv = "1.77")]

use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::{env, fmt};

use cc_builder::CcBuilder;
use cmake_builder::CmakeBuilder;
use system_library::SystemLib;

// These should generally match those found in aws-lc/include/openssl/opensslconf.h
const OSSL_CONF_DEFINES: &[&str] = &[
    "OPENSSL_NO_ASYNC",
    "OPENSSL_NO_BF",
    "OPENSSL_NO_BLAKE2",
    "OPENSSL_NO_BUF_FREELISTS",
    "OPENSSL_NO_CAMELLIA",
    "OPENSSL_NO_CAPIENG",
    "OPENSSL_NO_CAST",
    "OPENSSL_NO_CMS",
    "OPENSSL_NO_COMP",
    "OPENSSL_NO_CRYPTO_MDEBUG",
    "OPENSSL_NO_CT",
    "OPENSSL_NO_DANE",
    "OPENSSL_NO_DEPRECATED",
    "OPENSSL_NO_DGRAM",
    "OPENSSL_NO_DYNAMIC_ENGINE",
    "OPENSSL_NO_EC_NISTP_64_GCC_128",
    "OPENSSL_NO_EC2M",
    "OPENSSL_NO_EGD",
    "OPENSSL_NO_ENGINE",
    "OPENSSL_NO_GMP",
    "OPENSSL_NO_GOST",
    "OPENSSL_NO_HEARTBEATS",
    "OPENSSL_NO_HW",
    "OPENSSL_NO_IDEA",
    "OPENSSL_NO_JPAKE",
    "OPENSSL_NO_KRB5",
    "OPENSSL_NO_MD2",
    "OPENSSL_NO_MDC2",
    "OPENSSL_NO_OCB",
    "OPENSSL_NO_RC2",
    "OPENSSL_NO_RC5",
    "OPENSSL_NO_RFC3779",
    "OPENSSL_NO_RIPEMD",
    "OPENSSL_NO_RMD160",
    "OPENSSL_NO_SCTP",
    "OPENSSL_NO_SEED",
    "OPENSSL_NO_SM2",
    "OPENSSL_NO_SM3",
    "OPENSSL_NO_SM4",
    "OPENSSL_NO_SRP",
    "OPENSSL_NO_SSL_TRACE",
    "OPENSSL_NO_SSL2",
    "OPENSSL_NO_SSL3",
    "OPENSSL_NO_SSL3_METHOD",
    "OPENSSL_NO_STATIC_ENGINE",
    "OPENSSL_NO_STORE",
    "OPENSSL_NO_TS",
    "OPENSSL_NO_WHIRLPOOL",
];

mod cc_builder;
mod cmake_builder;
mod fips_probe;
mod nasm_builder;
#[cfg(any(feature = "bindgen", feature = "fips"))]
mod sys_bindgen;
mod system_detect;
mod system_library;

pub(crate) struct EnvGuard {
    key: String,
    original_value: Option<String>,
}

impl EnvGuard {
    fn new<T: AsRef<OsStr>>(key: &str, new_value: T) -> Self {
        let original_value = env::var(key).ok();
        env::set_var(key, new_value);
        Self {
            key: key.to_string(),
            original_value,
        }
    }

    #[cfg(test)]
    fn remove(key: &str) -> Self {
        let original_value = env::var(key).ok();
        env::remove_var(key);
        Self {
            key: key.to_string(),
            original_value,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        if let Some(ref value) = self.original_value {
            env::set_var(&self.key, value);
        } else {
            env::remove_var(&self.key);
        }
    }
}

/// Single process-wide lock serializing all tests that mutate env vars. Every
/// builder test module must share this one; a per-module mutex lets tests race
/// across modules and observe each other's partially-restored env state.
#[cfg(test)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(crate) fn is_fips_build() -> bool {
    is_fips_crate() || cfg!(feature = "fips")
}

fn is_fips_crate() -> bool {
    crate_name().contains("fips")
}

fn is_all_bindings() -> bool {
    is_fips_crate() || env::var("CARGO_FEATURE_ALL_BINDINGS").is_ok()
}

fn is_prebuilt_nasm() -> bool {
    !is_fips_build() && env::var("CARGO_FEATURE_PREBUILT_NASM").is_ok()
}

fn is_disable_prebuilt_nasm() -> bool {
    is_fips_build() || env::var("CARGO_FEATURE_DISABLE_PREBUILT_NASM").is_ok()
}

pub(crate) fn get_aws_lc_include_path(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("aws-lc").join("include")
}

pub(crate) fn get_rust_include_path(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("include")
}

pub(crate) fn get_generated_include_path(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("generated-include")
}

#[allow(unknown_lints)]
#[allow(static_mut_refs)]
pub(crate) fn get_env_includes_path() -> Option<Vec<PathBuf>> {
    unsafe { SYS_INCLUDES.clone() }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum OutputLib {
    Crypto,
    Ssl,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum OutputLibType {
    Static,
    Dynamic,
}

fn cargo_env<N: AsRef<str>>(name: N) -> String {
    let name = name.as_ref();
    env::var(name).unwrap_or_else(|_| panic!("missing env var {name:?}"))
}

fn env_name_for_target<K: AsRef<OsStr>>(env_var: K) -> String {
    let target = target().to_lowercase();
    let target = target.replace('-', "_");
    format!("{}_{target}", env_var.as_ref().to_str().unwrap())
}

// "CFLAGS" =>
// "SYS_CFLAGS_aarch64_unknown_linux_gnu" OR "SYS_CFLAGS"
//    OR "CFLAGS_aarch64_unknown_linux_gnu" OR "CFLAGS"
fn optional_env_optional_crate_target<N: AsRef<str>>(name: N) -> Option<String> {
    let name = name.as_ref();
    optional_env_crate_target(name).or(optional_env_target(name))
}

// "EFFECTIVE_TARGET" => "SYS_EFFECTIVE_TARGET_aarch64_unknown_linux_gnu" + "SYS_EFFECTIVE_TARGET"
fn optional_env_crate_target<N: AsRef<str>>(name: N) -> Option<String> {
    let name = name.as_ref();
    let crate_name = crate_name().to_uppercase().replace('-', "_");
    let name_for_crate = format!("{crate_name}_{name}");
    let name_for_crate_target = env_name_for_target(&name_for_crate);
    optional_env(name_for_crate_target).or(optional_env(name_for_crate))
}

pub(crate) fn optional_env_target<N: AsRef<str>>(name: N) -> Option<String> {
    let name_for_target = env_name_for_target(name.as_ref());
    optional_env(name_for_target).or(optional_env(name))
}

fn optional_env<N: AsRef<str>>(name: N) -> Option<String> {
    let name = name.as_ref();
    println!("cargo:rerun-if-env-changed={name}");
    if let Ok(value) = env::var(name) {
        emit_warning(format!("Environment Variable found '{name}': '{value}'"));
        return Some(value);
    }
    None
}

fn set_env_for_target<K, V>(env_var: K, value: V)
where
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let env_var = env_name_for_target(env_var);
    #[allow(unused_unsafe)]
    unsafe {
        env::set_var(&env_var, &value);
    }
    emit_warning(format!(
        "Setting {env_var}: {}",
        value.as_ref().to_str().unwrap()
    ));
}

fn set_env<K, V>(env_var: K, value: V)
where
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    #[allow(unused_unsafe)]
    unsafe {
        env::set_var(&env_var, &value);
    }
    emit_warning(format!(
        "Setting {}: {}",
        env_var.as_ref().to_str().unwrap(),
        value.as_ref().to_str().unwrap()
    ));
}

fn env_var_to_bool(name: &str) -> Option<bool> {
    if let Some(value) = optional_env(name) {
        return parse_to_bool(&value);
    }
    None
}

pub(crate) fn env_crate_var_to_bool(name: &str) -> Option<bool> {
    if let Some(value) = optional_env_crate_target(name) {
        return parse_to_bool(&value);
    }
    None
}

fn parse_to_bool(env_var_value: &str) -> Option<bool> {
    let env_var_value = env_var_value.to_lowercase();
    if env_var_value.starts_with('0')
        || env_var_value.starts_with('n')
        || env_var_value.starts_with("off")
        || env_var_value.starts_with('f')
    {
        emit_warning(format!("Value: {env_var_value} is false."));
        return Some(false);
    }
    if env_var_value.starts_with(|c: char| c.is_ascii_digit())
        || env_var_value.starts_with('y')
        || env_var_value.starts_with("on")
        || env_var_value.starts_with('t')
    {
        emit_warning(format!("Value: {env_var_value} is true."));
        return Some(true);
    }
    None
}

impl Default for OutputLibType {
    fn default() -> Self {
        if let Some(stc_lib) = is_static_library() {
            if stc_lib {
                OutputLibType::Static
            } else {
                OutputLibType::Dynamic
            }
        } else if !is_fips_build()
            || (is_fips_build()
                && (target_os() == "linux" || target_os().ends_with("bsd"))
                && (target_arch() == "x86_64" || target_arch() == "aarch64"))
        {
            OutputLibType::Static
        } else {
            OutputLibType::Dynamic
        }
    }
}

impl OutputLibType {
    fn rust_lib_type(&self) -> &str {
        match self {
            OutputLibType::Static => "static",
            OutputLibType::Dynamic => "dylib",
        }
    }

    /// Returns the `kind` portion of a `cargo:rustc-link-lib=<kind>=<name>`
    /// directive, including the `+whole-archive` modifier when whole-archive
    /// linking has been requested for a static library.
    ///
    /// Whole-archive linking forces the linker to include every object file
    /// from the static archive, which surfaces unresolved symbol references
    /// inside object files that no consumer happens to call. We use it in CI
    /// to catch missing assembly source files in the `cc_builder` source lists.
    fn rust_link_lib_kind(self) -> String {
        if matches!(self, OutputLibType::Static) && is_link_whole_archive() {
            format!("{}:+whole-archive", self.rust_lib_type())
        } else {
            self.rust_lib_type().to_string()
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Static => Self::Dynamic,
            Self::Dynamic => Self::Static,
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Dynamic => "dynamic",
        }
    }
}

impl OutputLib {
    fn libname(self, prefix: &Option<String>) -> String {
        let name = match self {
            OutputLib::Crypto => "crypto",
            OutputLib::Ssl => "ssl",
        };
        if let Some(prefix) = prefix {
            format!("{prefix}_{name}")
        } else {
            name.to_string()
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn prefix_string() -> String {
    format!(
        "aws_lc{}_{}",
        if is_fips_crate() { "_fips" } else { "" },
        VERSION.to_string().replace('.', "_")
    )
}

fn target_has_prefixed_symbols() -> bool {
    target_vendor() == "apple" || (target_arch() == "x86" && target_os() == "windows")
}

fn is_cranelift_backend() -> bool {
    // CARGO_ENCODED_RUSTFLAGS contains flags separated by 0x1f (ASCII Unit Separator)
    if let Some(rustflags) = optional_env("CARGO_ENCODED_RUSTFLAGS") {
        for flag in rustflags.split('\x1f') {
            if flag.contains("codegen-backend") && flag.contains("cranelift") {
                return true;
            }
        }
    }
    false
}

fn target_chokes_on_u1() -> bool {
    target_arch() == "mips" || target_arch() == "mips64" || is_cranelift_backend()
}

#[cfg(any(feature = "bindgen", feature = "fips"))]
fn target_platform_prefix(name: &str) -> String {
    if is_all_bindings() {
        format!("{}_{}", effective_target().replace('-', "_"), name)
    } else if target_has_prefixed_symbols() {
        format!("universal_prefixed_{}", name.replace('-', "_"))
    } else {
        format!("universal_{}", name.replace('-', "_"))
    }
}

pub(crate) struct CommandResult {
    #[allow(dead_code)]
    stderr: Box<str>,
    #[allow(dead_code)]
    stdout: Box<str>,
    spawn_error: Option<Box<str>>,
    exit_code: Option<i32>,
    executed: bool,
    status: bool,
}

/// Returns true if the given flag is an LTO-related compiler flag.
/// Used to filter out LTO flags from checks/builds where they are unnecessary
/// or cause toolchain-specific failures (e.g., Clang on Windows requires
/// `-fuse-ld=lld` for LTO linking).
pub(crate) fn is_lto_flag(flag: &str) -> bool {
    flag.starts_with("-flto") || flag == "-ffat-lto-objects" || flag == "-fno-fat-lto-objects"
}

const MAX_CMD_OUTPUT_SIZE: usize = 1 << 15;
fn execute_command(executable: &OsStr, args: &[&OsStr]) -> CommandResult {
    match Command::new(executable).args(args).output() {
        Ok(mut result) => {
            let exit_code = result.status.code();
            result.stderr.truncate(MAX_CMD_OUTPUT_SIZE);
            let stderr = String::from_utf8(result.stderr)
                .unwrap_or_default()
                .into_boxed_str();
            result.stdout.truncate(MAX_CMD_OUTPUT_SIZE);
            let stdout = String::from_utf8(result.stdout)
                .unwrap_or_default()
                .into_boxed_str();
            CommandResult {
                stderr,
                stdout,
                spawn_error: None,
                exit_code,
                executed: true,
                status: result.status.success(),
            }
        }
        Err(err) => CommandResult {
            stderr: String::new().into_boxed_str(),
            stdout: String::new().into_boxed_str(),
            spawn_error: Some(err.to_string().into_boxed_str()),
            exit_code: None,
            executed: false,
            status: false,
        },
    }
}

#[cfg(any(feature = "bindgen", feature = "fips"))]
fn generate_bindings(manifest_dir: &Path, prefix: &Option<String>, bindings_path: &PathBuf) {
    let options = BindingOptions {
        build_prefix: prefix.clone(),
        include_ssl: cfg!(feature = "ssl"),
        disable_prelude: true,
    };

    let bindings = sys_bindgen::generate_bindings(manifest_dir, &options);

    bindings
        .write(Box::new(std::fs::File::create(bindings_path).unwrap()))
        .expect("written bindings");
}

#[cfg(any(feature = "bindgen", feature = "fips"))]
fn generate_src_bindings(manifest_dir: &Path, prefix: &Option<String>, src_bindings_path: &Path) {
    sys_bindgen::generate_bindings(
        manifest_dir,
        &BindingOptions {
            build_prefix: prefix.clone(),
            include_ssl: false,
            ..Default::default()
        },
    )
    .write_to_file(src_bindings_path)
    .expect("write bindings");
}

pub(crate) fn emit_rustc_cfg(cfg: &str) {
    let cfg = cfg.replace('-', "_");
    emit_warning(format!("Emitting configuration: cargo:rustc-cfg={cfg}"));
    println!("cargo:rustc-cfg={cfg}");
}

fn emit_warning<T: AsRef<str>>(message: T) {
    println!("cargo:warning={}", message.as_ref());
}

#[allow(dead_code)]
fn target_family() -> String {
    cargo_env("CARGO_CFG_TARGET_FAMILY")
}

fn target_os() -> String {
    cargo_env("CARGO_CFG_TARGET_OS")
}

fn target_arch() -> String {
    cargo_env("CARGO_CFG_TARGET_ARCH")
}

#[allow(unused)]
fn target_env() -> String {
    cargo_env("CARGO_CFG_TARGET_ENV")
}

/// Whether the target ABI is MSVC (a `*-pc-windows-msvc` target).
///
/// This reflects the target ABI only and is the right signal for ABI/CRT/SDK
/// decisions (MSVC CRT linkage, `_CRT_SECURE_NO_WARNINGS`, the `clang-cl`
/// dependency checks). It is *not* a reliable signal for the compiler's flag
/// *dialect*: a `*-pc-windows-msvc` target can be built with plain `clang` in
/// GNU driver mode (e.g. `CC=clang`), which needs GNU-style flags. For dialect
/// decisions use `compiler_is_cl_like` instead.
fn target_is_msvc() -> bool {
    target_env() == "msvc"
}

/// Whether the active C compiler is driven in MSVC "cl" mode (`cl.exe` or
/// `clang-cl`), which is what determines cl-style flag *dialect* (`/Fo`, `/FI`,
/// `/Od`, no `--param ...`).
///
/// Driver mode is a property of the *compiler*, not the target ABI, and the two
/// can diverge in both directions:
/// - plain `clang` targeting `*-pc-windows-msvc` runs in GNU mode and needs
///   GNU-style flags (so the target ABI alone is not enough), and
/// - `cc::Tool::is_like_msvc()` can misclassify `clang-cl` as plain Clang when
///   its probe misfires in a sandbox (e.g. cargo-xwin), emitting GNU-only flags
///   to a cl-mode driver (so `is_like_msvc()` alone is not enough).
///
/// `clang` selects its driver mode from `argv[0]`, so a `clang-cl`/`cl` program
/// name is an authoritative fallback when `cc`'s family probe is unreliable.
/// See: <https://github.com/aws/aws-lc-rs/issues/1146>
pub(crate) fn compiler_is_cl_like(compiler: &cc::Tool) -> bool {
    compiler.is_like_msvc() || program_name_is_cl_driver(compiler.path())
}

/// Whether a compiler program name selects clang's cl driver mode (`clang-cl`)
/// or is `cl` itself. Robust fallback for `compiler_is_cl_like`.
pub(crate) fn program_name_is_cl_driver(path: &Path) -> bool {
    match path.file_name().and_then(OsStr::to_str) {
        Some(name) => {
            let name = name.to_ascii_lowercase();
            name.contains("clang-cl") || name == "cl" || name == "cl.exe"
        }
        None => false,
    }
}

#[allow(unused)]
fn target_vendor() -> String {
    cargo_env("CARGO_CFG_TARGET_VENDOR")
}

fn target() -> String {
    cargo_env("TARGET")
}

fn crate_name() -> String {
    cargo_env("CARGO_PKG_NAME")
}

/// Formats the full environment variable name for the current crate + given
/// suffix (e.g. `"SYSTEM_DIR"` → `"AWS_LC_SYS_SYSTEM_DIR"`).
pub(crate) fn crate_env_var_name(name: &str) -> String {
    let crate_name = crate_name().to_uppercase().replace('-', "_");
    format!("{crate_name}_{name}")
}

fn effective_target() -> String {
    #[allow(unknown_lints)]
    #[allow(static_mut_refs)]
    unsafe {
        if !SYS_EFFECTIVE_TARGET.is_empty() {
            return SYS_EFFECTIVE_TARGET.clone();
        }
    }
    let target = target();
    match target.as_str() {
        "x86_64-alpine-linux-musl" => "x86_64-unknown-linux-musl".to_string(),
        "aarch64-alpine-linux-musl" => "aarch64-unknown-linux-musl".to_string(),
        _ => target,
    }
}

#[allow(unused)]
fn target_underscored() -> String {
    effective_target().replace('-', "_")
}

pub(crate) fn out_dir() -> PathBuf {
    let out = PathBuf::from(cargo_env("OUT_DIR"));
    #[cfg(windows)]
    let out = to_short_path(&out);
    out
}

/// On Windows, convert a path to its 8.3 short form to avoid MAX_PATH (260 char) limits
/// when cl.exe is invoked with deeply nested source trees (e.g. Bazel runfiles).
#[cfg(windows)]
fn to_short_path(path: &Path) -> PathBuf {
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    extern "system" {
        fn GetShortPathNameW(
            lpszLongPath: *const u16,
            lpszShortPath: *mut u16,
            cchBuffer: u32,
        ) -> u32;
    }
    let wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let len = unsafe { GetShortPathNameW(wide.as_ptr(), std::ptr::null_mut(), 0) };
    if len == 0 {
        return path.to_path_buf();
    }
    let mut buf = vec![0u16; len as usize];
    let result = unsafe { GetShortPathNameW(wide.as_ptr(), buf.as_mut_ptr(), len) };
    if result == 0 {
        return path.to_path_buf();
    }
    buf.truncate(result as usize);
    let short_path = PathBuf::from(std::ffi::OsString::from_wide(&buf));

    const MAX_PATH: usize = 260;
    let original_len = wide.len() - 1;
    if original_len >= MAX_PATH && (result as usize) >= MAX_PATH {
        emit_warning(format!(
            "Path length ({}) exceeds MAX_PATH ({}) and 8.3 short name conversion was ineffective. \
             8.3 short names may be disabled on this volume. \
             See: https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/fsutil-8dot3name",
            original_len, MAX_PATH,
        ));
    }

    short_path
}

fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

fn get_builder(prefix: &Option<String>, manifest_dir: &Path, out_dir: &Path) -> Box<dyn Builder> {
    let cmake_builder_builder = || {
        Box::new(CmakeBuilder::new(
            manifest_dir.to_path_buf(),
            out_dir.to_path_buf(),
            prefix.clone(),
            OutputLibType::default(),
        ))
    };

    if let Some(install_dir) = get_system_dir_path() {
        // SYSTEM_DIR and USE_SYSTEM=0 are contradictory ("use this install" vs
        // "never use a system install"); fail rather than silently honoring one.
        assert!(
            use_system() != Some(false),
            "{system_dir} is set but {use_system}=0 forbids a system install.\n\
             Unset one: keep {system_dir} to link it, or {use_system}=0 to build from source.",
            system_dir = crate_env_var_name("SYSTEM_DIR"),
            use_system = crate_env_var_name("USE_SYSTEM"),
        );
        return Box::new(SystemLib::new(
            manifest_dir.to_path_buf(),
            install_dir,
            get_system_bindings_path(),
            get_system_skip_version_check(),
        ));
    }

    // No explicit SYSTEM_DIR. Unless the user opted out with USE_SYSTEM=0, try
    // to auto-detect a usable system AWS-LC. Detection is non-fatal: an
    // unsuitable (or absent) install falls through to the source build, unless
    // USE_SYSTEM=1 demands one.
    if use_system() != Some(false) {
        if let Some(system_lib) = system_detect::detect_system_awslc(manifest_dir) {
            // Mirror the explicit path's global state so is_bindgen_required()
            // and the other get_system_dir_path() consumers agree we are
            // linking a system library.
            set_system_dir(system_lib.marker_dir().to_path_buf());
            return Box::new(system_lib);
        }
        // Detection found nothing usable. Fall through to a source build unless
        // USE_SYSTEM=1 demands a system install, in which case fail.
        assert!(
            use_system() != Some(true),
            "{use_system}=1 requires a system-installed AWS-LC, but none was found.\n\
             Set {system_dir} to point at an install, provide OPENSSL_DIR, or unset \
             {use_system} to build from source.\n\
             Re-run with `-vv` to see why each discovered candidate was rejected.",
            use_system = crate_env_var_name("USE_SYSTEM"),
            system_dir = crate_env_var_name("SYSTEM_DIR"),
        );
    }

    if is_fips_build() {
        return cmake_builder_builder();
    }

    let cc_builder_builder = || {
        Box::new(CcBuilder::new(
            manifest_dir.to_path_buf(),
            out_dir.to_path_buf(),
            prefix.clone(),
            OutputLibType::default(),
        ))
    };

    if let Some(val) = is_cmake_builder() {
        let builder: Box<dyn Builder> = if val {
            cmake_builder_builder()
        } else {
            cc_builder_builder()
        };
        builder.check_dependencies().unwrap();
        return builder;
    } else if is_no_asm() || sanitizer().is_some() {
        let builder = cmake_builder_builder();
        builder.check_dependencies().unwrap();
        return builder;
    } else if !is_bindgen_required() {
        let cc_builder = cc_builder_builder();
        if cc_builder.check_dependencies().is_ok() {
            return cc_builder;
        }
    }
    let cmake_builder = cmake_builder_builder();
    cmake_builder.check_dependencies().unwrap();
    cmake_builder
}

trait Builder {
    fn check_dependencies(&self) -> Result<(), String>;
    fn build(&self) -> Result<(), String>;
    fn name(&self) -> &str;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CStdRequested {
    C99,
    C11,
    None,
}

impl CStdRequested {
    fn from_env() -> Self {
        if let Some(val) = optional_env_crate_target("C_STD") {
            let cstd = match val.as_str() {
                "99" => CStdRequested::C99,
                "11" => CStdRequested::C11,
                _ => CStdRequested::None,
            };
            emit_warning(format!("SYS_C_STD environment variable set: {cstd:?}"));
            return cstd;
        }
        CStdRequested::None
    }
}

static mut PREGENERATED: bool = false;
static mut SYS_NO_PREFIX: bool = false;
static mut SYS_PREGENERATING_BINDINGS: bool = false;
static mut SYS_EXTERNAL_BINDGEN: Option<bool> = None;
static mut SYS_NO_ASM: bool = false;
static mut SYS_PREBUILT_NASM: Option<bool> = None;
static mut SYS_CMAKE_BUILDER: Option<bool> = None;
static mut SYS_NO_PREGENERATED_SRC: bool = false;
static mut SYS_EFFECTIVE_TARGET: String = String::new();
static mut SYS_NO_JITTER_ENTROPY: Option<bool> = None;
static mut SYS_NO_U1_BINDINGS: Option<bool> = None;
static mut SYS_INCLUDES: Option<Vec<PathBuf>> = None;
static mut SYS_SANITIZER: Option<String> = None;
static mut SYS_LINK_WHOLE_ARCHIVE: bool = false;
static mut SYS_STATIC: Option<bool> = None;
static mut SYS_SYSTEM_DIR: Option<PathBuf> = None;
static mut SYS_USE_SYSTEM: Option<bool> = None;
static mut SYS_SYSTEM_BINDINGS: Option<PathBuf> = None;
static mut SYS_SYSTEM_SKIP_VERSION_CHECK: bool = false;
static mut SYS_C_STD: CStdRequested = CStdRequested::None;

fn initialize() {
    unsafe {
        SYS_NO_PREFIX = env_crate_var_to_bool("NO_PREFIX").unwrap_or(false);
        SYS_PREGENERATING_BINDINGS =
            env_crate_var_to_bool("PREGENERATING_BINDINGS").unwrap_or(false);
        SYS_EXTERNAL_BINDGEN = env_crate_var_to_bool("EXTERNAL_BINDGEN");
        SYS_NO_ASM = env_crate_var_to_bool("NO_ASM").unwrap_or(false);
        SYS_PREBUILT_NASM = env_crate_var_to_bool("PREBUILT_NASM");
        SYS_C_STD = CStdRequested::from_env();
        SYS_CMAKE_BUILDER = env_crate_var_to_bool("CMAKE_BUILDER");
        SYS_NO_PREGENERATED_SRC = env_crate_var_to_bool("NO_PREGENERATED_SRC").unwrap_or(false);
        SYS_EFFECTIVE_TARGET = optional_env_crate_target("EFFECTIVE_TARGET").unwrap_or_default();
        SYS_NO_JITTER_ENTROPY = env_crate_var_to_bool("NO_JITTER_ENTROPY");
        SYS_NO_U1_BINDINGS = env_crate_var_to_bool("NO_U1_BINDINGS");
        SYS_INCLUDES =
            optional_env_crate_target("INCLUDES").map(|v| std::env::split_paths(&v).collect());
        SYS_SANITIZER = optional_env_crate_target("SANITIZER").map(|v| v.to_lowercase());
        SYS_LINK_WHOLE_ARCHIVE = env_crate_var_to_bool("LINK_WHOLE_ARCHIVE").unwrap_or(false);
        SYS_STATIC = env_crate_var_to_bool("STATIC");
        SYS_SYSTEM_DIR = optional_env_crate_target("SYSTEM_DIR").map(PathBuf::from);
        SYS_USE_SYSTEM = env_crate_var_to_bool("USE_SYSTEM");
        SYS_SYSTEM_BINDINGS = optional_env_crate_target("SYSTEM_BINDINGS").map(PathBuf::from);
        SYS_SYSTEM_SKIP_VERSION_CHECK =
            env_crate_var_to_bool("SYSTEM_SKIP_VERSION_CHECK").unwrap_or(false);
    }

    assert!(
        !is_fips_crate() || is_fips_build(),
        "aws-lc-fips-sys requires 'fips' feature to be enabled.",
    );

    if !is_external_bindgen_requested().unwrap_or(false)
        && (is_pregenerating_bindings() || !has_bindgen_feature())
        && !cfg!(feature = "ssl")
    {
        if is_all_bindings() {
            assert!(
                use_no_u1_bindings() != Some(true),
                "Bindgen currently cannot generate prefixed bindings w/o the \\x01 prefix.",
            );
            let target = effective_target();
            let supported_platform = match (is_fips_crate(), target.as_str()) {
                (
                    _,
                    "aarch64-apple-darwin"
                    | "aarch64-unknown-linux-gnu"
                    | "aarch64-unknown-linux-musl"
                    | "x86_64-apple-darwin"
                    | "x86_64-unknown-linux-gnu"
                    | "x86_64-unknown-linux-musl",
                )
                | (
                    false,
                    "aarch64-linux-android"
                    | "aarch64-pc-windows-msvc"
                    | "i686-pc-windows-msvc"
                    | "i686-unknown-linux-gnu"
                    | "riscv64gc-unknown-linux-gnu"
                    | "x86_64-pc-windows-gnu"
                    | "x86_64-pc-windows-msvc",
                ) => Some(target),
                _ => None,
            };
            if let Some(platform) = supported_platform {
                emit_rustc_cfg(platform.as_str());
                unsafe {
                    PREGENERATED = true;
                }
            }
        } else if !is_fips_crate() {
            if use_no_u1_bindings() == Some(true)
                || (target_chokes_on_u1() && use_no_u1_bindings().is_none())
            {
                if is_cranelift_backend() {
                    emit_warning(
                        "Cranelift codegen backend detected. Using universal_no_u1 bindings.",
                    );
                }
                if target_has_prefixed_symbols() {
                    emit_rustc_cfg("universal-no-u1-prefixed");
                } else {
                    emit_rustc_cfg("universal-no-u1");
                }
            } else if target_has_prefixed_symbols() {
                emit_rustc_cfg("universal-prefixed");
            } else {
                emit_rustc_cfg("universal");
            }
            unsafe {
                PREGENERATED = true;
            }
        }
    }
}

fn is_bindgen_required() -> bool {
    // The system library path uses pre-generated bindings shipped with the
    // install; bindgen is never needed.
    if get_system_dir_path().is_some() {
        return false;
    }
    is_no_prefix()
        || is_pregenerating_bindings()
        || is_external_bindgen_requested().unwrap_or(false)
        || has_bindgen_feature()
        || !has_pregenerated()
}

#[cfg(any(feature = "bindgen", feature = "fips"))]
fn internal_bindgen_supported() -> bool {
    let cv = bindgen::clang_version();
    emit_warning(format!("Clang version: {}", cv.full));
    true
}

fn is_no_prefix() -> bool {
    unsafe { SYS_NO_PREFIX }
}

fn is_pregenerating_bindings() -> bool {
    unsafe { SYS_PREGENERATING_BINDINGS }
}

fn is_external_bindgen_requested() -> Option<bool> {
    unsafe { SYS_EXTERNAL_BINDGEN }
}

fn is_no_asm() -> bool {
    unsafe { SYS_NO_ASM }
}

#[allow(static_mut_refs)]
fn sanitizer() -> Option<String> {
    unsafe { SYS_SANITIZER.clone() }
}

fn is_cmake_builder() -> Option<bool> {
    if is_no_pregenerated_src() {
        Some(true)
    } else {
        unsafe { SYS_CMAKE_BUILDER }
    }
}

fn is_static_library() -> Option<bool> {
    unsafe { SYS_STATIC }
}

fn is_no_pregenerated_src() -> bool {
    unsafe { SYS_NO_PREGENERATED_SRC }
}

/// Returns true when the build script should ask rustc to link the produced
/// `libcrypto` archive with the `+whole-archive` modifier. This is intended
/// for CI use only and is enabled via `AWS_LC_SYS_LINK_WHOLE_ARCHIVE=1`.
///
/// `+whole-archive` on its own is necessary but not sufficient to catch
/// missing internal symbol references: the linker may still strip
/// unreferenced sections (Linux/LLD: `--gc-sections`; Apple ld:
/// `-dead_strip`) before final symbol resolution, so unresolved references
/// inside dead-stripped objects never produce link errors. To make this
/// flag effective, callers should also set `RUSTFLAGS="-C link-dead-code=yes"`
/// to tell rustc not to enable section GC. The CI job that exercises this
/// flag does both.
fn is_link_whole_archive() -> bool {
    unsafe { SYS_LINK_WHOLE_ARCHIVE }
}

fn disable_jitter_entropy() -> Option<bool> {
    unsafe { SYS_NO_JITTER_ENTROPY }
}

#[allow(static_mut_refs)]
pub(crate) fn get_system_dir_path() -> Option<PathBuf> {
    unsafe { SYS_SYSTEM_DIR.clone() }
}

/// Records an auto-detected install prefix as if it had been supplied via
/// `<crate>_SYSTEM_DIR`. This makes every downstream consumer of
/// `get_system_dir_path` — most importantly `is_bindgen_required` — observe
/// the same state as the explicit path, so detection can never both link a
/// system library and run bindgen.
fn set_system_dir(install_dir: PathBuf) {
    unsafe {
        SYS_SYSTEM_DIR = Some(install_dir);
    }
}

/// The tri-state `<crate>_USE_SYSTEM` control:
/// - `None` (unset): detect, adopt a valid system AWS-LC if found, else source.
/// - `Some(false)`: skip detection entirely; build from source.
/// - `Some(true)`: require a system install; hard-fail if none is found.
fn use_system() -> Option<bool> {
    unsafe { SYS_USE_SYSTEM }
}

#[allow(static_mut_refs)]
pub(crate) fn get_system_bindings_path() -> Option<PathBuf> {
    unsafe { SYS_SYSTEM_BINDINGS.clone() }
}

pub(crate) fn get_system_skip_version_check() -> bool {
    unsafe { SYS_SYSTEM_SKIP_VERSION_CHECK }
}

/// Returns true if jitter entropy should be built. This is false when the user
/// explicitly set `AWS_LC_SYS_NO_JITTER_ENTROPY=1`, or when auto-detection
/// determined that required headers are unavailable.
fn should_build_jitter_entropy() -> bool {
    static AVAILABLE: OnceLock<bool> = OnceLock::new();

    // If the user explicitly set the env var, respect it unconditionally.
    if let Some(val) = disable_jitter_entropy() {
        return !val;
    }

    *AVAILABLE.get_or_init(|| {
        // wasm/emscripten targets do not support CPU jitter entropy.
        if target_arch().starts_with("wasm") {
            return false;
        }
        // The current FIPS branch does not include jitter entropy.
        if is_fips_build() {
            return true;
        }
        // Only Apple targets need the CoreServices header.
        if target_vendor() != "apple" {
            return true;
        }
        probe_apple_core_services()
    })
}

/// Probes whether `CoreServices/CoreServices.h` is available for the current
/// target toolchain. On Apple targets, jitterentropy requires this header which
/// is only present when the macOS SDK is installed. When cross-compiling to
/// Apple targets from non-macOS hosts (e.g., Linux → macOS via cargo-zigbuild),
/// this header is typically missing.
fn probe_apple_core_services() -> bool {
    let probe_dir = out_dir().join("out-apple_core_services_check");
    let src_dir = probe_dir.join("src");
    let _ = std::fs::create_dir_all(&src_dir);
    let source_file = src_dir.join("apple_core_services_check.c");
    if std::fs::write(&source_file, "#include <CoreServices/CoreServices.h>\n").is_err() {
        emit_warning("Failed to write CoreServices probe file; assuming available.");
        return true;
    }

    let mut cc_build = cc::Build::default();
    cc_build.file(&source_file).out_dir(&probe_dir);
    let available = cc_build.try_compile_intermediates().is_ok();
    let _ = std::fs::remove_dir_all(&probe_dir);

    if !available {
        emit_warning(
            "CoreServices/CoreServices.h not available; \
             automatically disabling CPU jitter entropy.",
        );
    }
    available
}

fn use_no_u1_bindings() -> Option<bool> {
    unsafe { SYS_NO_U1_BINDINGS }
}

fn get_crate_cc() -> Option<String> {
    optional_env_optional_crate_target("TARGET_CC").or(optional_env_optional_crate_target("CC"))
}

fn get_crate_cxx() -> Option<String> {
    optional_env_optional_crate_target("TARGET_CXX").or(optional_env_optional_crate_target("CXX"))
}

fn get_crate_cflags() -> Option<String> {
    optional_env_optional_crate_target("TARGET_CFLAGS")
        .or(optional_env_optional_crate_target("CFLAGS"))
}

fn use_prebuilt_nasm() -> bool {
    target_os() == "windows"
        && target_arch() == "x86_64"
        && !is_no_asm()
        && !test_nasm_command() // NASM not found in environment
        && Some(false) != allow_prebuilt_nasm() // not prevented by environment
        && !is_disable_prebuilt_nasm() // not prevented by feature
        // permitted by environment or by feature
        && (Some(true) == allow_prebuilt_nasm() || is_prebuilt_nasm())
}

fn allow_prebuilt_nasm() -> Option<bool> {
    unsafe { SYS_PREBUILT_NASM }
}

fn requested_c_std() -> CStdRequested {
    unsafe { SYS_C_STD }
}

fn has_bindgen_feature() -> bool {
    cfg!(feature = "bindgen")
}

fn has_pregenerated() -> bool {
    unsafe { PREGENERATED }
}

fn test_nasm_command() -> bool {
    let status = execute_command("nasm".as_ref(), &["-version".as_ref()]).status;
    if !status {
        emit_warning("NASM command not found or failed to execute.");
    }
    status
}

fn find_clang_cl() -> Option<OsString> {
    static CACHE: OnceLock<Option<OsString>> = OnceLock::new();
    CACHE
        .get_or_init(|| {
            // Check if clang-cl is directly available (e.g., in PATH)
            if execute_command("clang-cl".as_ref(), &["--version".as_ref()]).status {
                return Some("clang-cl".into());
            }

            // Try to find clang-cl in a Visual Studio installation
            find_clang_cl_in_vs()
        })
        .clone()
}

fn find_clang_cl_in_vs() -> Option<OsString> {
    // Use the cc crate's VS discovery (which calls vswhere internally) to
    // locate the VS installation, then look for clang-cl relative to it.
    let tool = cc::windows_registry::find_tool(&target(), "cl.exe")?;
    let cl_path = tool.path().to_path_buf();

    // cl.exe lives deep inside the VS installation:
    //   <VS>/VC/Tools/MSVC/<ver>/bin/<host>/<target>/cl.exe
    // clang-cl lives at:
    //   <VS>/VC/Tools/Llvm/<arch>/bin/clang-cl.exe
    // Walk up from cl.exe until we find the VS root.
    for ancestor in cl_path.ancestors() {
        let vc_llvm = ancestor.join("VC").join("Tools").join("Llvm");
        if vc_llvm.exists() {
            for arch_dir in &["ARM64", "x64"] {
                let clang_cl = vc_llvm.join(arch_dir).join("bin").join("clang-cl.exe");
                if clang_cl.exists()
                    && execute_command(clang_cl.as_os_str(), &["--version".as_ref()]).status
                {
                    emit_warning(format!("Found clang-cl at: {}", clang_cl.display()));
                    return Some(clang_cl.into_os_string());
                }
            }
            break;
        }
    }

    None
}

fn test_clang_cl_command() -> bool {
    find_clang_cl().is_some()
}

fn prepare_cargo_cfg() {
    if cfg!(clippy) {
        println!("cargo:rustc-check-cfg=cfg(use_bindgen_pregenerated)");
        println!("cargo:rustc-check-cfg=cfg(aarch64_linux_android)");
        println!("cargo:rustc-check-cfg=cfg(aarch64_apple_darwin)");
        println!("cargo:rustc-check-cfg=cfg(aarch64_pc_windows_msvc)");
        println!("cargo:rustc-check-cfg=cfg(aarch64_unknown_linux_gnu)");
        println!("cargo:rustc-check-cfg=cfg(aarch64_unknown_linux_musl)");
        println!("cargo:rustc-check-cfg=cfg(cpu_jitter_entropy)");
        println!("cargo:rustc-check-cfg=cfg(i686_pc_windows_msvc)");
        println!("cargo:rustc-check-cfg=cfg(i686_unknown_linux_gnu)");
        println!("cargo:rustc-check-cfg=cfg(riscv64gc_unknown_linux_gnu)");
        println!("cargo:rustc-check-cfg=cfg(x86_64_apple_darwin)");
        println!("cargo:rustc-check-cfg=cfg(x86_64_pc_windows_gnu)");
        println!("cargo:rustc-check-cfg=cfg(x86_64_pc_windows_msvc)");
        println!("cargo:rustc-check-cfg=cfg(x86_64_unknown_linux_gnu)");
        println!("cargo:rustc-check-cfg=cfg(x86_64_unknown_linux_musl)");
        println!("cargo:rustc-check-cfg=cfg(universal)");
        println!("cargo:rustc-check-cfg=cfg(universal_no_u1)");
        println!("cargo:rustc-check-cfg=cfg(universal_no_u1_prefixed)");
        println!("cargo:rustc-check-cfg=cfg(universal_prefixed)");
    }
}

fn is_crt_static() -> bool {
    let features = cargo_env("CARGO_CFG_TARGET_FEATURE");
    features.contains("crt-static")
}

#[cfg(any(feature = "bindgen", feature = "fips"))]
fn handle_bindgen(manifest_dir: &Path, prefix: &Option<String>) -> bool {
    if internal_bindgen_supported() && !is_external_bindgen_requested().unwrap_or(false) {
        emit_warning(format!(
            "Generating bindings - internal bindgen. Platform: {}",
            effective_target()
        ));
        let gen_bindings_path = out_dir().join("bindings.rs");
        generate_bindings(manifest_dir, prefix, &gen_bindings_path);
        emit_rustc_cfg("use_bindgen_pregenerated");
        true
    } else {
        false
    }
}

#[cfg(not(any(feature = "bindgen", feature = "fips")))]
fn handle_bindgen(_manifest_dir: &Path, _prefix: &Option<String>) -> bool {
    false
}

fn canonicalized_manifest_dir() -> PathBuf {
    let manifest_dir = current_dir();
    let manifest_dir = dunce::canonicalize(Path::new(&manifest_dir)).unwrap();
    #[cfg(windows)]
    let manifest_dir = to_short_path(&manifest_dir);
    manifest_dir
}

/// Links the startup FIPS-mode self-check for the system-library FIPS path.
pub(crate) fn link_fips_runtime_check(
    manifest_dir: &Path,
    include_dir: &Path,
) -> Result<(), String> {
    let source = manifest_dir.join("builder").join("fips_runtime_check.c");
    if !source.is_file() {
        return Err(format!(
            "FIPS runtime check source missing: {}",
            source.display()
        ));
    }
    println!("cargo:rerun-if-changed={}", source.display());

    let mut cc_build = cc::Build::default();
    cc_build
        .file(&source)
        .warnings(false)
        // We emit the link directives ourselves (with +whole-archive), so
        // suppress cc-rs's automatic emission.
        .cargo_metadata(false)
        // Silence cc-rs's `ar cqD` probe, which Apple's `ar` rejects (and
        // cc-rs leaks as warnings before retrying). Real compile errors still
        // surface from the build-time probe in verify_fips_install().
        .cargo_warnings(false)
        .include(include_dir);

    let lib_name = "aws_lc_fips_runtime_check";
    cc_build.compile(lib_name);

    // Keep the constructor object in the final link.
    println!("cargo:rustc-link-search=native={}", out_dir().display());
    println!("cargo:rustc-link-lib=static:+whole-archive={lib_name}");
    Ok(())
}

#[cfg(not(test))]
fn main() {
    initialize();
    prepare_cargo_cfg();

    let manifest_dir = canonicalized_manifest_dir();

    let prefix = (!is_no_prefix()).then(prefix_string);

    let builder = get_builder(&prefix, &manifest_dir, &out_dir());
    emit_warning(format!("Building with: {}", builder.name()));
    emit_warning(format!("Symbol Prefix: {prefix:?}"));

    builder.check_dependencies().unwrap();

    #[allow(unused_assignments)]
    let mut bindings_available = false;
    emit_warning(format!("Target platform: '{}'", target()));
    if is_pregenerating_bindings() {
        #[cfg(any(feature = "bindgen", feature = "fips"))]
        {
            let src_bindings_path = Path::new(&manifest_dir)
                .join("src")
                .join(format!("{}.rs", target_platform_prefix("crypto")));
            if is_external_bindgen_requested().unwrap_or(false) {
                assert!(
                    !is_pregenerating_bindings(),
                    "Pregenerated bindings not supported using external bindgen.",
                );
                invoke_external_bindgen(&manifest_dir, &prefix, &src_bindings_path).unwrap();
            } else {
                generate_src_bindings(&manifest_dir, &prefix, &src_bindings_path);
            }
            bindings_available = true;
        }
    } else if is_bindgen_required() {
        assert!(
            use_no_u1_bindings() != Some(true),
            "Bindgen currently cannot generate prefixed bindings w/o the \\x01 prefix.",
        );
        emit_warning("######");
        emit_warning(
            "If bindgen is unable to locate a header file, use the \
            BINDGEN_EXTRA_CLANG_ARGS environment variable to specify additional include paths.",
        );
        emit_warning(
            "See: https://github.com/rust-lang/rust-bindgen?tab=readme-ov-file#environment-variables",
        );
        emit_warning("######");
        let aws_lc_crypto_dir = Path::new(&manifest_dir).join("aws-lc").join("crypto");
        if !aws_lc_crypto_dir.exists() {
            emit_warning("######");
            emit_warning("###### WARNING: MISSING GIT SUBMODULE ######");
            emit_warning(format!(
                "  -- Did you initialize the repo's git submodules? Unable to find crypto directory: {}.",
                aws_lc_crypto_dir.display()
            ));
            emit_warning("  -- run 'git submodule update --init --recursive' to initialize.");
            emit_warning("######");
            emit_warning("######");
        }
        bindings_available = handle_bindgen(&manifest_dir, &prefix);
    } else {
        bindings_available = true;
    }

    if !bindings_available {
        if cfg!(feature = "ssl") {
            emit_warning(
                "External bindgen required, but external bindgen unable to produce SSL bindings.",
            );
        } else {
            assert!(
                use_no_u1_bindings() != Some(true),
                "Bindgen currently cannot generate prefixed bindings w/o the \\x01 prefix.",
            );
            let gen_bindings_path = out_dir().join("bindings.rs");
            let result = invoke_external_bindgen(&manifest_dir, &prefix, &gen_bindings_path);
            match result {
                Ok(()) => {
                    emit_rustc_cfg("use_bindgen_pregenerated");
                    bindings_available = true;
                }
                Err(msg) => eprintln!("Failure invoking external bindgen! {msg}"),
            }
        }
    }

    assert!(
        bindings_available,
        "aws-lc-sys build failed. Please enable the 'bindgen' feature on aws-lc-rs or aws-lc-sys.\
        For more information, see the aws-lc-rs User Guide: https://aws.github.io/aws-lc-rs/index.html"
    );
    builder.build().unwrap();

    // Emitted unconditionally regardless of which builder ran: the OpenSSL
    // compatibility-conf macro list is a property of AWS-LC's API surface,
    // not of how it was built or located. The builder/ rerun trigger is
    // likewise common because every code path runs through this build script.
    println!("cargo:conf={}", OSSL_CONF_DEFINES.join(","));
    println!("cargo:rerun-if-changed=builder/");
}

/// Emits the shared post-build cargo metadata for source-based builders
/// (`CMake` and CC). This sets up include paths, exports library and
/// configuration names for downstream crates, and registers rerun triggers.
pub(crate) fn emit_source_build_metadata(manifest_dir: &Path) {
    // MinGW/GCC ignores `#pragma comment(lib, "bcrypt.lib")`, so we must
    // link explicitly. The upstream CMakeLists.txt forces _WIN32_WINNT_WIN7
    // for MINGW+GCC, activating the BCryptGenRandom codepath.
    // See: https://github.com/aws/aws-lc/pull/3239
    if target().contains("-windows-gnu") {
        println!("cargo:rustc-link-lib=bcrypt");
    }

    println!(
        "cargo:include={}",
        setup_include_paths(&out_dir(), manifest_dir).display()
    );

    // export the artifact names
    println!("cargo:libcrypto={}_crypto", prefix_string());
    if cfg!(feature = "ssl") {
        println!("cargo:libssl={}_ssl", prefix_string());
    }

    println!("cargo:rerun-if-changed=aws-lc/");
}

fn setup_include_paths(out_dir: &Path, manifest_dir: &Path) -> PathBuf {
    let mut include_paths = vec![
        get_rust_include_path(manifest_dir),
        get_generated_include_path(manifest_dir),
        get_aws_lc_include_path(manifest_dir),
    ];

    if let Some(extra_paths) = get_env_includes_path() {
        include_paths.extend(extra_paths);
    }

    let include_dir = out_dir.join("include");
    std::fs::create_dir_all(&include_dir).unwrap();

    // iterate over all the include paths and copy them into the final output
    for path in include_paths {
        for child in std::fs::read_dir(path).into_iter().flatten().flatten() {
            if child.path().is_file() {
                std::fs::copy(
                    child.path(),
                    include_dir.join(child.path().file_name().unwrap()),
                )
                .expect("Failed to copy include file during build setup");
                continue;
            }

            // prefer the earliest paths
            let options = fs_extra::dir::CopyOptions::new()
                .skip_exist(true)
                .copy_inside(true);
            fs_extra::dir::copy(child.path(), &include_dir, &options)
                .expect("Failed to copy include directory during build setup");
        }
    }

    include_dir
}

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct BindingOptions {
    pub build_prefix: Option<String>,
    pub include_ssl: bool,
    pub disable_prelude: bool,
}

impl Debug for BindingOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BindingOptions")
            .field("build_prefix", &self.build_prefix)
            .field("include_ssl", &self.include_ssl)
            .field("disable_prelude", &self.disable_prelude)
            .finish()
    }
}

fn verify_bindgen() -> Result<(), String> {
    if !is_external_bindgen_requested().unwrap_or(true) {
        return Err("external bindgen usage prevented by SYS_EXTERNAL_BINDGEN=0".to_string());
    }

    let result = execute_command("bindgen".as_ref(), &["--version".as_ref()]);
    if !result.status {
        if result.executed {
            eprintln!(
                "bindgen-cli exited with an error status:\nSTDOUT: {}\n\nSTDERR: {}",
                result.stdout, result.stderr
            );
        } else {
            eprintln!(
                "Consider installing the bindgen-cli: \
            `cargo install --force --locked bindgen-cli`\
            \n\
            See our User Guide for more information about bindgen:\
            https://aws.github.io/aws-lc-rs/index.html"
            );
        }
        return Err("External bindgen command failed.".to_string());
    }
    let mut major_version: u32 = 0;
    let mut minor_version: u32 = 0;
    let mut patch_version: u32 = 0;
    let bindgen_version = result.stdout.split(' ').nth(1);
    if let Some(version) = bindgen_version {
        let version_parts: Vec<&str> = version.trim().split('.').collect();
        if version_parts.len() == 3 {
            major_version = version_parts[0].parse::<u32>().unwrap_or(0);
            minor_version = version_parts[1].parse::<u32>().unwrap_or(0);
            patch_version = version_parts[2].parse::<u32>().unwrap_or(0);
        }
    }
    // We currently expect to support all bindgen versions >= 0.69.5
    if major_version == 0 && (minor_version < 69 || (minor_version == 69 && patch_version < 5)) {
        eprintln!(
            "bindgen-cli was used. Detected version was: \
            {major_version}.{minor_version}.{patch_version} \n\
        Consider upgrading : \
        `cargo install --force --locked bindgen-cli`\
        \n\
        See our User Guide for more information about bindgen:\
        https://aws.github.io/aws-lc-rs/index.html"
        );
    }
    Ok(())
}

const PRELUDE: &str = r"
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::missing_safety_doc,
    clippy::must_use_candidate,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::ptr_as_ptr,
    clippy::ptr_offset_with_cast,
    clippy::pub_underscore_fields,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::useless_transmute,
    dead_code,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unpredictable_function_pointer_comparisons,
    unused_imports
)]
";

fn invoke_external_bindgen(
    manifest_dir: &Path,
    prefix: &Option<String>,
    gen_bindings_path: &Path,
) -> Result<(), String> {
    verify_bindgen()?;

    emit_warning(format!(
        "Generating bindings - external bindgen. Platform: {}",
        effective_target()
    ));

    let _guard_target = EnvGuard::new("TARGET", effective_target());

    let options = BindingOptions {
        // We collect the symbols w/o the prefix added
        build_prefix: None,
        include_ssl: false,
        disable_prelude: true,
    };

    let clang_args = prepare_clang_args(manifest_dir, &options);
    let header = get_rust_include_path(manifest_dir)
        .join("rust_wrapper.h")
        .display()
        .to_string();

    let sym_prefix: String;
    let mut bindgen_params = vec![];
    if let Some(prefix_str) = prefix {
        sym_prefix = if target_os().to_lowercase() == "macos"
            || target_os().to_lowercase() == "ios"
            || target_os().to_lowercase() == "tvos"
            || (target_os().to_lowercase() == "windows" && target_arch() == "x86")
        {
            format!("_{prefix_str}_")
        } else {
            format!("{prefix_str}_")
        };
        bindgen_params.extend(vec!["--prefix-link-name", sym_prefix.as_str()]);
    }

    // These flags needs to be kept in sync with the setup in bindgen::prepare_bindings_builder
    // If `bindgen-cli` makes backwards incompatible changes, we will update the parameters below
    // to conform with the most recent release. We will guide consumers to likewise use the
    // latest version of bindgen-cli.
    bindgen_params.extend(vec![
        "--allowlist-file",
        r".*(/|\\)openssl((/|\\)[^/\\]+)+\.h",
        "--allowlist-file",
        r".*(/|\\)rust_wrapper\.h",
        "--rustified-enum",
        r"point_conversion_form_t",
        "--default-macro-constant-type",
        r"signed",
        "--with-derive-default",
        "--with-derive-partialeq",
        "--with-derive-eq",
        "--raw-line",
        COPYRIGHT,
        "--generate",
        "functions,types,vars,methods,constructors,destructors",
        header.as_str(),
        "--rust-target",
        r"1.70",
        "--output",
        gen_bindings_path.to_str().unwrap(),
        "--formatter",
        r"rustfmt",
    ]);
    if !options.disable_prelude {
        bindgen_params.extend(["--raw-line", PRELUDE]);
    }
    bindgen_params.push("--");

    for x in &clang_args {
        bindgen_params.push(x.as_str());
    }
    let cmd_params: Vec<OsString> = bindgen_params.iter().map(OsString::from).collect();
    let cmd_params: Vec<&OsStr> = cmd_params.iter().map(OsString::as_os_str).collect();

    let result = execute_command("bindgen".as_ref(), cmd_params.as_ref());
    if !result.status {
        return Err(format!(
            "\n\n\
            bindgen-PARAMS: {}\n\
            bindgen-STDOUT: {}\n\
            bindgen-STDERR: {}",
            bindgen_params.join(" "),
            result.stdout.as_ref(),
            result.stderr.as_ref()
        ));
    }
    Ok(())
}

fn add_header_include_path(args: &mut Vec<String>, path: String) {
    args.push("-I".to_string());
    args.push(path);
}

fn prepare_clang_args(manifest_dir: &Path, options: &BindingOptions) -> Vec<String> {
    let mut clang_args: Vec<String> = Vec::new();

    add_header_include_path(
        &mut clang_args,
        get_rust_include_path(manifest_dir).display().to_string(),
    );

    if options.build_prefix.is_some() {
        // NOTE: It's possible that the prefix embedded in the header files doesn't match the prefix
        // specified. This only happens when the version number as changed in Cargo.toml, but the
        // new headers have not yet been generated.
        add_header_include_path(
            &mut clang_args,
            get_generated_include_path(manifest_dir)
                .display()
                .to_string(),
        );
    }

    add_header_include_path(
        &mut clang_args,
        get_aws_lc_include_path(manifest_dir).display().to_string(),
    );

    if let Some(include_paths) = get_env_includes_path() {
        for path in include_paths {
            add_header_include_path(&mut clang_args, path.display().to_string());
        }
    }

    clang_args
}

const COPYRIGHT: &str = r"
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
";

// =============================================================================
// Tests - run via: rustc --test builder/main.rs --edition 2021 -o /tmp/builder_test && /tmp/builder_test
// Or via the Makefile target: make test-builder
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // parse_to_bool tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_to_bool_true_values() {
        assert_eq!(parse_to_bool("1"), Some(true));
        assert_eq!(parse_to_bool("2"), Some(true));
        assert_eq!(parse_to_bool("9"), Some(true));
        assert_eq!(parse_to_bool("ON"), Some(true));
        assert_eq!(parse_to_bool("on"), Some(true));
        assert_eq!(parse_to_bool("On"), Some(true));
        assert_eq!(parse_to_bool("YES"), Some(true));
        assert_eq!(parse_to_bool("yes"), Some(true));
        assert_eq!(parse_to_bool("TRUE"), Some(true));
        assert_eq!(parse_to_bool("true"), Some(true));
        assert_eq!(parse_to_bool("y"), Some(true));
        assert_eq!(parse_to_bool("Y"), Some(true));
        assert_eq!(parse_to_bool("t"), Some(true));
        assert_eq!(parse_to_bool("T"), Some(true));
    }

    #[test]
    fn test_parse_to_bool_false_values() {
        assert_eq!(parse_to_bool("0"), Some(false));
        assert_eq!(parse_to_bool("OFF"), Some(false));
        assert_eq!(parse_to_bool("off"), Some(false));
        assert_eq!(parse_to_bool("NO"), Some(false));
        assert_eq!(parse_to_bool("no"), Some(false));
        assert_eq!(parse_to_bool("FALSE"), Some(false));
        assert_eq!(parse_to_bool("false"), Some(false));
        assert_eq!(parse_to_bool("n"), Some(false));
        assert_eq!(parse_to_bool("N"), Some(false));
        assert_eq!(parse_to_bool("f"), Some(false));
        assert_eq!(parse_to_bool("F"), Some(false));
    }

    #[test]
    fn test_parse_to_bool_none_values() {
        assert_eq!(parse_to_bool(""), None);
        assert_eq!(parse_to_bool("invalid"), None);
        assert_eq!(parse_to_bool("maybe"), None);
        assert_eq!(parse_to_bool("   "), None);
    }

    // -------------------------------------------------------------------------
    // OutputLib tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_output_lib_crypto_libname_with_prefix() {
        let prefix = Some("test_prefix".to_string());
        let name = OutputLib::Crypto.libname(&prefix);
        assert_eq!(name, "test_prefix_crypto");
    }

    #[test]
    fn test_output_lib_crypto_libname_without_prefix() {
        let name = OutputLib::Crypto.libname(&None);
        assert_eq!(name, "crypto");
    }

    #[test]
    fn test_output_lib_ssl_libname_with_prefix() {
        let prefix = Some("my_prefix".to_string());
        let name = OutputLib::Ssl.libname(&prefix);
        assert_eq!(name, "my_prefix_ssl");
    }

    #[test]
    fn test_output_lib_ssl_libname_without_prefix() {
        let name = OutputLib::Ssl.libname(&None);
        assert_eq!(name, "ssl");
    }

    // -------------------------------------------------------------------------
    // OutputLibType tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_output_lib_type_rust_lib_type() {
        assert_eq!(OutputLibType::Static.rust_lib_type(), "static");
        assert_eq!(OutputLibType::Dynamic.rust_lib_type(), "dylib");
    }

    // -------------------------------------------------------------------------
    // Version/Prefix tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_version_constant_exists() {
        assert!(!VERSION.is_empty(), "VERSION should not be empty");
    }

    // -------------------------------------------------------------------------
    // is_lto_flag tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_lto_flag() {
        assert!(is_lto_flag("-flto"));
        assert!(is_lto_flag("-flto=thin"));
        assert!(is_lto_flag("-flto=full"));
        assert!(is_lto_flag("-flto=auto"));
        assert!(is_lto_flag("-flto=4"));
        assert!(is_lto_flag("-ffat-lto-objects"));
        assert!(is_lto_flag("-fno-fat-lto-objects"));

        assert!(!is_lto_flag("-O3"));
        assert!(!is_lto_flag("-march=native"));
        assert!(!is_lto_flag("-ffunction-sections"));
        assert!(!is_lto_flag("-fdata-sections"));
        assert!(!is_lto_flag("-fuse-ld=lld"));
    }
}
