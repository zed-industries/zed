// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// NOTE: This module is intended to produce an equivalent "libcrypto" static library to the one
// produced by the CMake. Changes to CMake relating to compiler checks and/or special build flags
// may require modifications to the logic in this module.

mod apple_aarch64;
mod apple_x86_64;
mod linux_aarch64;
mod linux_arm;
mod linux_ppc64le;
mod linux_x86;
mod linux_x86_64;
mod universal;
mod win_aarch64;
mod win_x86;
mod win_x86_64;

use crate::nasm_builder::NasmBuilder;
use crate::{
    cargo_env, compiler_is_cl_like, emit_warning, env_var_to_bool, execute_command, find_clang_cl,
    get_crate_cc, get_crate_cflags, get_crate_cxx, is_link_whole_archive, is_no_asm, out_dir,
    requested_c_std, set_env_for_target, should_build_jitter_entropy, target, target_arch,
    target_env, target_is_msvc, target_os, target_vendor, CStdRequested, EnvGuard, OutputLibType,
};
use std::cell::Cell;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[non_exhaustive]
#[derive(PartialEq, Eq)]
pub(crate) enum CompilerFeature {
    NeonSha3,
}

pub(crate) struct CcBuilder {
    manifest_dir: PathBuf,
    out_dir: PathBuf,
    build_prefix: Option<String>,
    output_lib_type: OutputLibType,
    compiler_features: Cell<Vec<CompilerFeature>>,
}

use std::fs;

fn identify_sources() -> Vec<&'static str> {
    let mut source_files: Vec<&'static str> = vec![];
    source_files.append(&mut Vec::from(universal::CRYPTO_LIBRARY));
    let mut target_specific_source_found = true;
    if target_os() == "windows" {
        if target_arch() == "x86_64" {
            source_files.append(&mut Vec::from(win_x86_64::CRYPTO_LIBRARY));
        } else if target_arch() == "aarch64" {
            source_files.append(&mut Vec::from(win_aarch64::CRYPTO_LIBRARY));
        } else if target_arch() == "x86" {
            source_files.append(&mut Vec::from(win_x86::CRYPTO_LIBRARY));
        } else {
            target_specific_source_found = false;
        }
    } else if target_vendor() == "apple" {
        if target_arch() == "x86_64" {
            source_files.append(&mut Vec::from(apple_x86_64::CRYPTO_LIBRARY));
        } else if target_arch() == "aarch64" {
            source_files.append(&mut Vec::from(apple_aarch64::CRYPTO_LIBRARY));
        } else {
            target_specific_source_found = false;
        }
    } else if target_arch() == "x86_64" {
        source_files.append(&mut Vec::from(linux_x86_64::CRYPTO_LIBRARY));
    } else if target_arch() == "aarch64" {
        source_files.append(&mut Vec::from(linux_aarch64::CRYPTO_LIBRARY));
    } else if target_arch() == "arm" {
        source_files.append(&mut Vec::from(linux_arm::CRYPTO_LIBRARY));
    } else if target_arch() == "x86" {
        source_files.append(&mut Vec::from(linux_x86::CRYPTO_LIBRARY));
    } else if target_arch() == "powerpc64" {
        source_files.append(&mut Vec::from(linux_ppc64le::CRYPTO_LIBRARY));
    } else {
        target_specific_source_found = false;
    }
    if !target_specific_source_found {
        emit_warning(format!(
            "No target-specific source found: {}-{}",
            target_os(),
            target_arch()
        ));
    }

    source_files
}

#[allow(clippy::upper_case_acronyms)]
pub(crate) enum BuildOption {
    STD(String),
    FLAG(String),
    DEFINE(String, String),
    INCLUDE(PathBuf),
}
impl BuildOption {
    fn std<T: ToString + ?Sized>(val: &T) -> Self {
        Self::STD(val.to_string())
    }
    fn flag<T: ToString + ?Sized>(val: &T) -> Self {
        Self::FLAG(val.to_string())
    }
    fn flag_if_supported<T: ToString + ?Sized>(cc_build: &cc::Build, flag: &T) -> Option<Self> {
        if let Ok(true) = cc_build.is_flag_supported(flag.to_string()) {
            Some(Self::FLAG(flag.to_string()))
        } else {
            None
        }
    }

    fn define<K: ToString + ?Sized, V: ToString + ?Sized>(key: &K, val: &V) -> Self {
        Self::DEFINE(key.to_string(), val.to_string())
    }

    fn include<P: Into<PathBuf>>(path: P) -> Self {
        Self::INCLUDE(path.into())
    }

    fn apply_cc<'a>(&self, cc_build: &'a mut cc::Build) -> &'a mut cc::Build {
        match self {
            BuildOption::STD(val) => cc_build.std(val),
            BuildOption::FLAG(val) => cc_build.flag(val),
            BuildOption::DEFINE(key, val) => cc_build.define(key, Some(val.as_str())),
            BuildOption::INCLUDE(path) => cc_build.include(path.as_path()),
        }
    }

    pub(crate) fn apply_cmake<'a>(
        &self,
        cmake_cfg: &'a mut cmake::Config,
        is_cl_like: bool,
    ) -> &'a mut cmake::Config {
        if is_cl_like {
            match self {
                BuildOption::STD(val) => cmake_cfg.define(
                    "CMAKE_C_STANDARD",
                    val.to_ascii_lowercase().strip_prefix('c').unwrap_or("11"),
                ),
                BuildOption::FLAG(val) => cmake_cfg.cflag(val),
                BuildOption::DEFINE(key, val) => cmake_cfg.cflag(format!("/D{key}={val}")),
                BuildOption::INCLUDE(path) => cmake_cfg.cflag(format!("/I{}", path.display())),
            }
        } else {
            match self {
                BuildOption::STD(val) => cmake_cfg.define(
                    "CMAKE_C_STANDARD",
                    val.to_ascii_lowercase().strip_prefix('c').unwrap_or("11"),
                ),
                BuildOption::FLAG(val) => cmake_cfg.cflag(val),
                BuildOption::DEFINE(key, val) => cmake_cfg.cflag(format!("-D{key}={val}")),
                BuildOption::INCLUDE(path) => cmake_cfg.cflag(format!("-I{}", path.display())),
            }
        }
    }

    pub(crate) fn apply_nasm<'a>(&self, nasm_builder: &'a mut NasmBuilder) -> &'a mut NasmBuilder {
        match self {
            BuildOption::FLAG(val) => nasm_builder.flag(val),
            BuildOption::DEFINE(key, val) => nasm_builder.define(key, Some(val.as_str())),
            BuildOption::INCLUDE(path) => nasm_builder.include(path.as_path()),
            BuildOption::STD(_) => nasm_builder, // STD ignored for NASM
        }
    }
}

impl CcBuilder {
    pub(crate) fn new(
        manifest_dir: PathBuf,
        out_dir: PathBuf,
        build_prefix: Option<String>,
        output_lib_type: OutputLibType,
    ) -> Self {
        Self {
            manifest_dir,
            out_dir,
            build_prefix,
            output_lib_type,
            compiler_features: Cell::new(vec![]),
        }
    }

    pub(crate) fn collect_universal_build_options(
        &self,
        cc_build: &cc::Build,
        do_quote_paths: bool,
    ) -> (bool, Vec<BuildOption>) {
        let mut build_options: Vec<BuildOption> = Vec::new();

        let is_cl_like = compiler_is_cl_like(&cc_build.get_compiler());

        match requested_c_std() {
            CStdRequested::C99 => {
                build_options.push(BuildOption::std("c99"));
            }
            CStdRequested::C11 => {
                build_options.push(BuildOption::std("c11"));
            }
            CStdRequested::None => {
                if !is_cl_like {
                    if self.compiler_check("c11", Vec::<String>::new()) {
                        build_options.push(BuildOption::std("c11"));
                    } else {
                        build_options.push(BuildOption::std("c99"));
                    }
                }
            }
        }

        if let Some(cc) = get_crate_cc() {
            set_env_for_target("CC", &cc);
        }
        if let Some(cxx) = get_crate_cxx() {
            set_env_for_target("CXX", &cxx);
        }

        if target_arch() == "x86" && !is_cl_like {
            if let Some(option) = BuildOption::flag_if_supported(cc_build, "-msse2") {
                build_options.push(option);
            }
        }

        if target_os() == "macos" || target_os() == "darwin" {
            // Certain MacOS system headers are guarded by _POSIX_C_SOURCE and _DARWIN_C_SOURCE
            build_options.push(BuildOption::define("_DARWIN_C_SOURCE", "1"));
        }

        let opt_level = cargo_env("OPT_LEVEL");
        match opt_level.as_str() {
            "0" | "1" | "2" => {
                if is_no_asm() {
                    emit_warning("AWS_LC_SYS_NO_ASM found. Disabling assembly code usage.");
                    build_options.push(BuildOption::define("OPENSSL_NO_ASM", "1"));
                }
            }
            _ => {
                assert!(
                    !is_no_asm(),
                    "AWS_LC_SYS_NO_ASM only allowed for debug builds!"
                );
                if !is_cl_like {
                    let path_str = if do_quote_paths {
                        format!("\"{}\"", self.manifest_dir.display())
                    } else {
                        format!("{}", self.manifest_dir.display())
                    };

                    let flag = format!("-ffile-prefix-map={path_str}=");
                    if let Ok(true) = cc_build.is_flag_supported(&flag) {
                        emit_warning(format!("Using flag: {flag}"));
                        build_options.push(BuildOption::flag(&flag));
                    } else {
                        emit_warning("NOTICE: Build environment source paths might be visible in release binary.");
                        let flag = format!("-fdebug-prefix-map={path_str}=");
                        if let Ok(true) = cc_build.is_flag_supported(&flag) {
                            emit_warning(format!("Using flag: {flag}"));
                            build_options.push(BuildOption::flag(&flag));
                        }
                    }
                }
            }
        }

        if target_os() == "macos" {
            // This compiler error has only been seen on MacOS x86_64:
            // ```
            // clang: error: overriding '-mmacosx-version-min=13.7' option with '--target=x86_64-apple-macosx14.2' [-Werror,-Woverriding-t-option]
            // ```
            if let Some(option) =
                BuildOption::flag_if_supported(cc_build, "-Wno-overriding-t-option")
            {
                build_options.push(option);
            }
            if let Some(option) = BuildOption::flag_if_supported(cc_build, "-Wno-overriding-option")
            {
                build_options.push(option);
            }
        }
        (is_cl_like, build_options)
    }

    pub fn collect_cc_only_build_options(&self) -> Vec<BuildOption> {
        let mut build_options: Vec<BuildOption> = Vec::new();
        let is_cl_like = compiler_is_cl_like(&cc::Build::new().get_compiler());
        if !is_cl_like {
            build_options.push(BuildOption::flag("-Wno-unused-parameter"));
            // On emscripten, `-pthread` forces a shared-memory wasm module
            // (requires SharedArrayBuffer); build single-threaded instead.
            if target_os() != "emscripten" {
                build_options.push(BuildOption::flag("-pthread"));
            }
            if target_os() == "linux" || target_os() == "emscripten" {
                build_options.push(BuildOption::define("_XOPEN_SOURCE", "700"));
            } else if target_vendor() != "apple" {
                // Needed by illumos
                build_options.push(BuildOption::define("__EXTENSIONS__", "1"));
            }
        }
        if !should_build_jitter_entropy() {
            build_options.push(BuildOption::define("DISABLE_CPU_JITTER_ENTROPY", "1"));
        }
        self.add_includes(&mut build_options);
        self.add_defines(&mut build_options, is_cl_like);

        build_options
    }

    fn add_includes(&self, build_options: &mut Vec<BuildOption>) {
        // The order of includes matters
        if let Some(prefix) = &self.build_prefix {
            build_options.push(BuildOption::define("BORINGSSL_IMPLEMENTATION", "1"));
            build_options.push(BuildOption::define("BORINGSSL_PREFIX", prefix.as_str()));
            build_options.push(BuildOption::include(
                self.manifest_dir.join("generated-include"),
            ));
        }
        build_options.push(BuildOption::include(self.manifest_dir.join("include")));
        build_options.push(BuildOption::include(
            self.manifest_dir.join("aws-lc").join("include"),
        ));
        build_options.push(BuildOption::include(
            self.manifest_dir
                .join("aws-lc")
                .join("third_party")
                .join("s2n-bignum")
                .join("include"),
        ));
        build_options.push(BuildOption::include(
            self.manifest_dir
                .join("aws-lc")
                .join("third_party")
                .join("s2n-bignum")
                .join("s2n-bignum-imported")
                .join("include"),
        ));

        if should_build_jitter_entropy() {
            let jitterentropy_path = self
                .manifest_dir
                .join("aws-lc")
                .join("third_party")
                .join("jitterentropy")
                .join("jitterentropy-library");

            build_options.push(BuildOption::include(&jitterentropy_path));
            build_options.push(BuildOption::include(jitterentropy_path.join("src")));
        }
    }

    pub fn create_builder(&self) -> cc::Build {
        let mut cc_build = cc::Build::new();
        let build_options = self.collect_cc_only_build_options();
        for option in build_options {
            option.apply_cc(&mut cc_build);
        }
        cc_build
    }

    pub fn prepare_builder(&self) -> cc::Build {
        if let Some(cflags) = get_crate_cflags() {
            set_env_for_target("CFLAGS", cflags);
        }

        let mut cc_build = self.create_builder();
        let (is_cl_like, build_options) = self.collect_universal_build_options(&cc_build, false);
        for option in build_options {
            option.apply_cc(&mut cc_build);
        }

        // Add --noexecstack flag for assembly files to prevent executable stacks
        // This matches the behavior of AWS-LC's CMake build which uses -Wa,--noexecstack
        // See: https://github.com/aws/aws-lc/blob/main/crypto/CMakeLists.txt#L77
        if target_os() == "linux" || target_os().ends_with("bsd") {
            cc_build.asm_flag("-Wa,--noexecstack");
        }

        // `-ffile-prefix-map` does not reach GNU `as` for `.S` sources, so in
        // release-style builds mirror it with `-Wa,--debug-prefix-map=...`.
        // Probe first because clang's integrated assembler rejects the flag,
        // and skip paths with spaces because `-Wa,...` must stay a bare token.
        let opt_level = cargo_env("OPT_LEVEL");
        if (target_os() == "linux" || target_os().ends_with("bsd"))
            && !is_cl_like
            && !matches!(opt_level.as_str(), "0" | "1" | "2")
            && !self.manifest_dir.to_string_lossy().contains(' ')
        {
            let path_str = self.manifest_dir.display().to_string();
            let asm_flag = format!("-Wa,--debug-prefix-map={path_str}=");
            if cc_build.is_flag_supported(&asm_flag).unwrap_or(false) {
                cc_build.asm_flag(asm_flag);
            }
        }

        cc_build
    }

    #[allow(clippy::zero_sized_map_values)]
    fn build_s2n_bignum_source_feature_map() -> HashMap<String, CompilerFeature> {
        let mut source_feature_map: HashMap<String, CompilerFeature> = HashMap::new();
        source_feature_map.insert("sha3_keccak_f1600_alt.S".into(), CompilerFeature::NeonSha3);
        source_feature_map.insert("sha3_keccak2_f1600.S".into(), CompilerFeature::NeonSha3);
        source_feature_map.insert(
            "sha3_keccak4_f1600_alt2.S".into(),
            CompilerFeature::NeonSha3,
        );
        source_feature_map
    }

    #[allow(clippy::unused_self)]
    fn add_defines(&self, build_options: &mut Vec<BuildOption>, is_cl_like: bool) {
        // WIN32_LEAN_AND_MEAN and NOMINMAX are needed for all Windows targets to avoid
        // header type definition errors, no matter the compiler. This matches the behavior
        // in aws-lc/CMakeLists.txt, which defines these for all WIN32 targets
        if target_os() == "windows" {
            build_options.push(BuildOption::define("WIN32_LEAN_AND_MEAN", ""));
            build_options.push(BuildOption::define("NOMINMAX", ""));
        }

        // Suppress MSVC CRT deprecation warnings (fopen, getenv, strerror, etc.).
        // This applies to any compiler targeting the MSVC environment since the
        // warnings originate from the Windows SDK/CRT headers, not the compiler,
        // so it is keyed on the target ABI rather than the driver mode.
        if target_is_msvc() {
            build_options.push(BuildOption::define("_CRT_SECURE_NO_WARNINGS", "1"));
        }

        // MSVC STL macros: only meaningful when compiling in cl driver mode.
        if is_cl_like {
            build_options.push(BuildOption::define("_HAS_EXCEPTIONS", "0"));
            build_options.push(BuildOption::define(
                "_STL_EXTRA_DISABLED_WARNINGS",
                "4774 4987",
            ));
        }

        // Target Windows 7 (0x0601 == _WIN32_WINNT_WIN7) for any win7 target triple.
        if target().contains("-win7-windows-") {
            build_options.push(BuildOption::define("_WIN32_WINNT", "0x0601"));
            emit_warning(format!(
                "Setting _WIN32_WINNT to _WIN32_WINNT_WIN7 for {} target",
                target()
            ));

            // Additional workaround for MinGW: the upstream C source
            // (crypto/rand_extra/windows.c) gates the Win7 compat path
            // (BCryptGenRandom) with `!defined(__MINGW32__)`, which prevents MinGW
            // from using it even when `_WIN32_WINNT` targets Win7. We define
            // AWSLC_WINDOWS_7_COMPAT directly to bypass that guard until the
            // upstream fix lands: https://github.com/aws/aws-lc/pull/3239
            if !is_cl_like {
                build_options.push(BuildOption::define("AWSLC_WINDOWS_7_COMPAT", ""));
            }
        }
    }

    fn prepare_jitter_entropy_builder(&self, is_cl_like: bool) -> cc::Build {
        // See: https://github.com/aws/aws-lc/blob/2294510cd0ecb2d5946461e3dbb038363b7b94cb/third_party/jitterentropy/CMakeLists.txt#L19-L35
        let mut build_options: Vec<BuildOption> = Vec::new();
        self.add_includes(&mut build_options);
        self.add_defines(&mut build_options, is_cl_like);

        let mut je_builder = cc::Build::new();
        for option in build_options {
            option.apply_cc(&mut je_builder);
        }

        je_builder.define("AWSLC", "1");
        if target_os() == "macos" || target_os() == "darwin" {
            // Certain MacOS system headers are guarded by _POSIX_C_SOURCE and _DARWIN_C_SOURCE
            je_builder.define("_DARWIN_C_SOURCE", "1");
        }
        // Only enable PIC on non-Windows targets. Windows doesn't support -fPIC.
        if target_os() != "windows" {
            je_builder.pic(true);
        }
        for &flag in Self::jitter_entropy_dialect_flags(is_cl_like) {
            je_builder.flag(flag);
        }

        // Jitter uses a separate `cc::Build`, so it needs its own path-mapping
        // flags even when the main builder already has them. Apply them here
        // unconditionally because jitter is always compiled at `-O0`.
        for option in self.collect_path_reproducibility_options(&je_builder, is_cl_like) {
            option.apply_cc(&mut je_builder);
        }
        je_builder
    }

    /// Returns path-reproducibility flags for the configured compiler.
    /// These rewrite DWARF source paths and `__FILE__`; clang may also need
    /// extra stripping for `UBSan` metadata. Returns an empty `Vec` in cl
    /// driver mode, which has no equivalent flags.
    fn collect_path_reproducibility_options(
        &self,
        cc_build: &cc::Build,
        is_cl_like: bool,
    ) -> Vec<BuildOption> {
        let mut opts: Vec<BuildOption> = Vec::new();
        if is_cl_like {
            return opts;
        }

        let path_str = self.manifest_dir.display().to_string();

        // Prefer the flag that rewrites both `__FILE__` and DWARF; older GCC
        // may only support `-fdebug-prefix-map`.
        let file_flag = format!("-ffile-prefix-map={path_str}=");
        if cc_build.is_flag_supported(&file_flag).unwrap_or(false) {
            opts.push(BuildOption::flag(&file_flag));
        } else {
            let dbg_flag = format!("-fdebug-prefix-map={path_str}=");
            if cc_build.is_flag_supported(&dbg_flag).unwrap_or(false) {
                opts.push(BuildOption::flag(&dbg_flag));
            }
        }

        // Clang UBSan metadata can still carry the full source path at `-O0`.
        if let Some(opt) = BuildOption::flag_if_supported(
            cc_build,
            "-fsanitize-undefined-strip-path-components=-1",
        ) {
            opts.push(opt);
        }

        opts
    }

    /// Dialect-specific compiler flags for the jitterentropy sub-build.
    ///
    /// Keyed on compiler driver mode: `clang-cl` rejects the GNU-only
    /// `--param ssp-buffer-size=4` pair, while plain `clang` rejects the
    /// cl-style `-Od`/`-W4` flags. See: <https://github.com/aws/aws-lc-rs/issues/1146>
    fn jitter_entropy_dialect_flags(is_cl_like: bool) -> &'static [&'static str] {
        if is_cl_like {
            &["-Od", "-W4", "-DYNAMICBASE"]
        } else {
            &[
                "-fwrapv",
                "--param",
                "ssp-buffer-size=4",
                "-fvisibility=hidden",
                "-Wcast-align",
                "-Wmissing-field-initializers",
                "-Wshadow",
                "-Wswitch-enum",
                "-Wextra",
                "-Wall",
                "-pedantic",
                // Compilation will fail if optimizations are enabled.
                "-O0",
                "-fwrapv",
                "-Wconversion",
            ]
        }
    }

    /// The cc crate appends CFLAGS at the end of the compiler command line,
    /// which means CFLAGS optimization flags override build script flags.
    /// Jitterentropy MUST be compiled with -O0, so we temporarily override
    /// CFLAGS to replace any optimization flags with -O0.
    ///
    /// The cc crate collects flags from ALL matching CFLAGS env vars (not just
    /// the highest-priority one), so we must filter every variable it checks:
    ///   1. `CFLAGS_{target}` (e.g. `CFLAGS_x86_64_unknown_freebsd`)
    ///   2. `HOST_CFLAGS` or `TARGET_CFLAGS`
    ///   3. `CFLAGS`
    fn jitter_entropy_cflags_guards(is_cl_like: bool) -> Vec<EnvGuard> {
        let target_u = target().to_lowercase().replace('-', "_");

        let cflags_env_names: Vec<String> = vec![
            format!("CFLAGS_{target_u}"),
            "HOST_CFLAGS".to_string(),
            "TARGET_CFLAGS".to_string(),
            "CFLAGS".to_string(),
        ];

        let filter_cflags = |value: &str| -> String {
            let filtered: String = value
                .split_whitespace()
                .filter(|flag| !flag.starts_with("-O") && !flag.starts_with("/O"))
                .collect::<Vec<_>>()
                .join(" ");
            if is_cl_like {
                format!("{filtered} -Od").trim().to_string()
            } else {
                format!("{filtered} -O0 -U_FORTIFY_SOURCE")
                    .trim()
                    .to_string()
            }
        };

        cflags_env_names
            .into_iter()
            .filter_map(|name| {
                let value = env::var(&name).ok()?;
                Some(EnvGuard::new(&name, filter_cflags(&value)))
            })
            .collect()
    }

    fn add_all_files(&self, sources: &[&'static str], cc_build: &mut cc::Build) {
        let is_cl_like = compiler_is_cl_like(&cc_build.get_compiler());

        let force_include_option = if is_cl_like { "/FI" } else { "--include=" };
        // s2n-bignum is compiled separately due to needing extra flags
        let mut s2n_bignum_builder = cc_build.clone();
        s2n_bignum_builder.flag(format!(
            "{}{}",
            force_include_option,
            self.manifest_dir
                .join("generated-include")
                .join("openssl")
                .join("boringssl_prefix_symbols_asm.h")
                .display()
        ));
        s2n_bignum_builder.define("S2N_BN_HIDE_SYMBOLS", "1");

        // CPU Jitter Entropy is compiled separately due to needing specific flags.
        // Only set up the builder if jitter entropy is actually going to be built.
        let mut jitter_entropy_builder = should_build_jitter_entropy().then(|| {
            let mut jitter_entropy_builder = self.prepare_jitter_entropy_builder(is_cl_like);
            jitter_entropy_builder.flag(format!(
                "{}{}",
                force_include_option,
                self.manifest_dir
                    .join("generated-include")
                    .join("openssl")
                    .join("boringssl_prefix_symbols.h")
                    .display()
            ));
            jitter_entropy_builder
        });

        let mut build_options = vec![];
        self.add_includes(&mut build_options);
        let mut nasm_builder = NasmBuilder::new(self.manifest_dir.clone(), self.out_dir.clone());

        for option in &build_options {
            option.apply_nasm(&mut nasm_builder);
        }

        let s2n_bignum_source_feature_map = Self::build_s2n_bignum_source_feature_map();
        let compiler_features = self.compiler_features.take();
        for source in sources {
            let source_path = self.manifest_dir.join("aws-lc").join(source);
            let is_s2n_bignum = std::path::Path::new(source).starts_with("third_party/s2n-bignum");
            let is_jitter_entropy =
                std::path::Path::new(source).starts_with("third_party/jitterentropy");

            if !source_path.is_file() {
                emit_warning(format!("Not a file: {:?}", source_path.as_os_str()));
                continue;
            }
            if is_s2n_bignum {
                let filename: String = source_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                if let Some(compiler_feature) = s2n_bignum_source_feature_map.get(&filename) {
                    if compiler_features.contains(compiler_feature) {
                        s2n_bignum_builder.file(source_path);
                    } else {
                        emit_warning(format!(
                            "Skipping due to missing compiler features: {:?}",
                            source_path.as_os_str()
                        ));
                    }
                } else {
                    s2n_bignum_builder.file(source_path);
                }
            } else if is_jitter_entropy {
                // Only compile if not disabled.
                if let Some(builder) = jitter_entropy_builder.as_mut() {
                    builder.file(source_path);
                }
            } else if source_path.extension() == Some("asm".as_ref()) {
                nasm_builder.file(source_path);
            } else {
                cc_build.file(source_path);
            }
        }
        self.compiler_features.set(compiler_features);
        let s2n_bignum_object_files = s2n_bignum_builder.compile_intermediates();
        for object in s2n_bignum_object_files {
            cc_build.object(object);
        }
        if let Some(builder) = jitter_entropy_builder {
            let _je_cflags_guards = Self::jitter_entropy_cflags_guards(is_cl_like);
            let jitter_entropy_object_files = builder.compile_intermediates();
            for object in jitter_entropy_object_files {
                cc_build.object(object);
            }
        }
        let nasm_object_files = nasm_builder.compile_intermediates();
        for object in nasm_object_files {
            cc_build.object(object);
        }
    }

    fn build_library(&self, sources: &[&'static str]) {
        let mut cc_build = self.prepare_builder();
        self.run_compiler_checks(&mut cc_build);

        self.add_all_files(sources, &mut cc_build);

        let lib_name = if let Some(prefix) = &self.build_prefix {
            format!("{}_crypto", prefix.as_str())
        } else {
            "crypto".to_string()
        };

        // When whole-archive linking is requested, suppress cc-rs's automatic
        // emission of `cargo:rustc-link-{lib,search}=` so we can emit our own
        // directives with the `+whole-archive` modifier (which cc-rs has no
        // way to express). cc still performs the actual compilation and
        // archive creation; only the metadata output is silenced.
        if is_link_whole_archive() {
            cc_build.cargo_metadata(false);
        }
        cc_build.compile(&lib_name);
        if is_link_whole_archive() {
            emit_warning(format!(
                "AWS_LC_SYS_LINK_WHOLE_ARCHIVE set: linking '{lib_name}' with +whole-archive"
            ));
            println!("cargo:rustc-link-search=native={}", self.out_dir.display());
            println!(
                "cargo:rustc-link-lib={}={lib_name}",
                self.output_lib_type.rust_link_lib_kind()
            );
        }
    }

    // This performs basic checks of compiler capabilities and sets an appropriate flag on success.
    // This should be kept in alignment with the checks performed by AWS-LC's CMake build.
    // See: https://github.com/search?q=repo%3Aaws%2Faws-lc%20check_compiler&type=code
    fn compiler_check<T, S>(&self, basename: &str, extra_flags: T) -> bool
    where
        T: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut ret_val = false;
        let output_dir = self.out_dir.join(format!("out-{basename}"));
        let source_file = self
            .manifest_dir
            .join("aws-lc")
            .join("tests")
            .join("compiler_features_tests")
            .join(format!("{basename}.c"));
        if !source_file.exists() {
            emit_warning("######");
            emit_warning("###### WARNING: MISSING GIT SUBMODULE ######");
            emit_warning(format!(
                "  -- Did you initialize the repo's git submodules? Unable to find source file: {}.",
                source_file.display()
            ));
            emit_warning("  -- run 'git submodule update --init --recursive' to initialize.");
            emit_warning("######");
            emit_warning("######");
        }
        let mut cc_build = cc::Build::default();
        cc_build
            .file(source_file)
            .warnings_into_errors(true)
            .out_dir(&output_dir);
        for flag in extra_flags {
            let flag = flag.as_ref();
            cc_build.flag(flag);
        }

        // GNU-style flag, so gated on the actual compiler family.
        let compiler = cc_build.get_compiler();
        if compiler.is_like_gnu() || compiler.is_like_clang() {
            cc_build.flag("-Wno-unused-parameter");
        }
        let result = cc_build.try_compile_intermediates();

        if result.is_ok() {
            ret_val = true;
        }
        if fs::remove_dir_all(&output_dir).is_err() {
            emit_warning(format!("Failed to remove {}", output_dir.display()));
        }
        emit_warning(format!(
            "Compilation of '{basename}.c' {} - {:?}.",
            if ret_val { "succeeded" } else { "failed" },
            result
        ));
        ret_val
    }

    // This checks whether the compiler contains a critical bug that causes `memcmp` to erroneously
    // consider two regions of memory to be equal when they're not.
    // See GCC bug report: https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189
    // This should be kept in alignment with the same check performed by the CMake build.
    // See: https://github.com/search?q=repo%3Aaws%2Faws-lc%20check_run&type=code
    fn memcmp_check(&self) {
        // This check compiles, links, and executes a test program. When cross-compiling
        // (HOST != TARGET), we cannot execute the resulting binary, so we skip this check.
        // This also avoids linker configuration issues with cross-compilation toolchains
        // (e.g., cross-rs Darwin toolchains that set invalid -fuse-ld= flags in CFLAGS).
        if cargo_env("HOST") != target() {
            return;
        }

        let basename = "memcmp_invalid_stripped_check";
        let exec_path = out_dir().join(basename);
        let memcmp_build = cc::Build::default();
        let memcmp_compiler = memcmp_build.get_compiler();
        // The logic below assumes a Clang or GCC compiler; skip any other
        // family (keyed on the compiler, not the target ABI).
        if !memcmp_compiler.is_like_clang() && !memcmp_compiler.is_like_gnu() {
            return;
        }
        // Only pass -O3 to trigger the optimization bug. We intentionally ignore
        // CFLAGS here — this check is about compiler behavior at high optimization
        // levels, not about the user's build configuration. Arbitrary CFLAGS can
        // cause unrelated compile/link failures (e.g., -flto=thin on Windows
        // requires -fuse-ld=lld). This matches the CMake build which only passes
        // CMAKE_C_FLAGS_RELEASE (-O3) to check_run().
        let mut memcmp_compile_args: Vec<std::ffi::OsString> = vec!["-O3".into()];

        // Respect LDFLAGS for custom linker configurations. This check produces
        // an executable, so the linker must be reachable. LDFLAGS may contain
        // necessary flags like library search paths or linker selection.
        if let Ok(ldflags) = std::env::var("LDFLAGS") {
            for flag in ldflags.split_whitespace() {
                memcmp_compile_args.push(flag.into());
            }
        }

        memcmp_compile_args.push(
            self.manifest_dir
                .join("aws-lc")
                .join("tests")
                .join("compiler_features_tests")
                .join(format!("{basename}.c"))
                .into_os_string(),
        );
        memcmp_compile_args.push("-Wno-unused-parameter".into());
        memcmp_compile_args.push("-o".into());
        memcmp_compile_args.push(exec_path.clone().into_os_string());
        let memcmp_args: Vec<_> = memcmp_compile_args
            .iter()
            .map(std::ffi::OsString::as_os_str)
            .collect();
        let memcmp_compile_result =
            execute_command(memcmp_compiler.path().as_os_str(), memcmp_args.as_slice());
        assert!(
            memcmp_compile_result.status,
            "COMPILER: {}\
            ARGS: {:?}\
            EXECUTED: {}\
            ERROR: {}\
            OUTPUT: {}\
            Failed to compile {basename}
            ",
            memcmp_compiler.path().display(),
            memcmp_args.as_slice(),
            memcmp_compile_result.executed,
            memcmp_compile_result.stderr,
            memcmp_compile_result.stdout
        );

        let result = execute_command(exec_path.as_os_str(), &[]);
        assert!(
            result.status,
            "### COMPILER BUG DETECTED ###\nYour compiler ({}) is not supported due to a memcmp related bug reported in \
            https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189. \
            We strongly recommend against using this compiler. \n\
            EXECUTED: {}\n\
            ERROR: {}\n\
            OUTPUT: {}\n\
            ",
            memcmp_compiler.path().display(),
            memcmp_compile_result.executed,
            memcmp_compile_result.stderr,
            memcmp_compile_result.stdout
        );
        let _ = fs::remove_file(exec_path);
    }
    fn run_compiler_checks(&self, cc_build: &mut cc::Build) {
        if self.compiler_check("stdalign_check", Vec::<&'static str>::new()) {
            cc_build.define("AWS_LC_STDALIGN_AVAILABLE", Some("1"));
        }
        // Only run builtin_swap_check for GCC/Clang (matching CMake). On MSVC,
        // try_compile_intermediates succeeds but __builtin_bswap* are unresolved at
        // link time. Without this guard the define is set and crypto/internal.h skips
        // the correct _MSC_VER path that uses _byteswap_* intrinsics.
        if !target_is_msvc()
            && self.compiler_check("builtin_swap_check", Vec::<&'static str>::new())
        {
            cc_build.define("AWS_LC_BUILTIN_SWAP_SUPPORTED", Some("1"));
        }
        if target_arch() == "aarch64"
            && self.compiler_check("neon_sha3_check", vec!["-march=armv8.4-a+sha3"])
        {
            let mut compiler_features = self.compiler_features.take();
            compiler_features.push(CompilerFeature::NeonSha3);
            self.compiler_features.set(compiler_features);
            cc_build.define("MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION", Some("1"));
        }
        if target_os() == "linux" || target_os() == "android" {
            if self.compiler_check("linux_random_h", Vec::<&'static str>::new()) {
                cc_build.define("HAVE_LINUX_RANDOM_H", Some("1"));
            } else if self.compiler_check("linux_random_h", vec!["-DDEFINE_U32"]) {
                cc_build.define("HAVE_LINUX_RANDOM_H", Some("1"));
                cc_build.define("AWS_LC_URANDOM_NEEDS_U32", Some("1"));
            }
        }
        self.memcmp_check();
    }
}

impl crate::Builder for CcBuilder {
    fn check_dependencies(&self) -> Result<(), String> {
        if OutputLibType::Dynamic == self.output_lib_type {
            // https://github.com/rust-lang/cc-rs/issues/594
            return Err("CcBuilder only supports static builds".to_string());
        }

        if target_env() == "ohos" {
            return Err("OpenHarmony targets must be built with CMake.".to_string());
        }

        if Some(true) == env_var_to_bool("CARGO_FEATURE_SSL") {
            return Err("cc_builder for libssl not supported".to_string());
        }

        Ok(())
    }

    fn build(&self) -> Result<(), String> {
        if target_os() == "windows"
            && target_arch() == "aarch64"
            && target_is_msvc()
            && get_crate_cc().is_none()
        {
            if let Some(clang_cl) = find_clang_cl() {
                set_env_for_target("CC", clang_cl);
            } else {
                emit_warning(
                    "Windows ARM64 (aarch64-pc-windows-msvc) requires clang-cl. \
                     Install the 'C++ Clang Compiler for Windows' component in \
                     Visual Studio Build Tools, or set CC to a working clang-cl. \
                     See User Guide: https://aws.github.io/aws-lc-rs/index.html",
                );
            }
        }

        println!("cargo:root={}", self.out_dir.display());
        let sources = crate::cc_builder::identify_sources();
        self.build_library(sources.as_slice());

        crate::emit_source_build_metadata(&self.manifest_dir);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "CC"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EnvGuard, ENV_MUTEX};

    /// Runs `f` with `CARGO_CFG_TARGET_ENV` forced to `env_val`, restoring the
    /// previous value afterward. Holds the shared env lock so it doesn't race
    /// other env-mutating builder tests.
    fn with_target_env<R>(env_val: &str, f: impl FnOnce() -> R) -> R {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let _guard = EnvGuard::new("CARGO_CFG_TARGET_ENV", env_val);
        f()
    }

    #[test]
    fn test_target_is_msvc_matches_msvc_abi_only() {
        assert!(with_target_env("msvc", target_is_msvc));
        assert!(!with_target_env("gnu", target_is_msvc));
        assert!(!with_target_env("musl", target_is_msvc));
        assert!(!with_target_env("", target_is_msvc));
    }

    // Guards https://github.com/aws/aws-lc-rs/issues/1146: in cl driver mode
    // the GNU-only `--param ssp-buffer-size=4` pair (rejected by `clang-cl`)
    // must never be selected.
    #[test]
    fn test_jitter_entropy_flags_cl_dialect_omit_gnu_only() {
        let cl = CcBuilder::jitter_entropy_dialect_flags(true);
        assert!(
            !cl.contains(&"--param") && !cl.contains(&"ssp-buffer-size=4"),
            "cl-mode jitter flags must not contain the GNU-only ssp-buffer-size pair: {cl:?}"
        );
        assert!(cl.contains(&"-Od"), "expected cl-style -Od: {cl:?}");
    }

    // Guards the inverse of #1146: in GNU driver mode (e.g. plain `clang`, even
    // on a windows-msvc target) the cl-only `-Od`/`-W4` flags must not be
    // selected and the GNU hardening flags must be preserved.
    #[test]
    fn test_jitter_entropy_gnu_dialect_keeps_hardening_flags() {
        let gnu = CcBuilder::jitter_entropy_dialect_flags(false);
        assert!(gnu.contains(&"--param"));
        assert!(gnu.contains(&"ssp-buffer-size=4"));
        assert!(
            !gnu.contains(&"-Od"),
            "GNU dialect must not use cl-style -Od: {gnu:?}"
        );
        // jitterentropy must always be built unoptimized.
        assert!(gnu.contains(&"-O0"), "expected -O0: {gnu:?}");
    }

    // Driver mode is selected by the compiler program name (argv[0]); this is
    // the robust fallback when `cc`'s family probe misfires. `clang-cl`/`cl`
    // are cl mode; plain `clang`/`gcc` are not -- guarding both #1146 and its
    // inverse (plain clang on a windows-msvc target).
    #[test]
    fn test_program_name_is_cl_driver() {
        use crate::program_name_is_cl_driver;
        use std::path::Path;
        for cl in ["clang-cl", "clang-cl.exe", "CLANG-CL.EXE", "cl", "cl.exe"] {
            assert!(
                program_name_is_cl_driver(Path::new(cl)),
                "{cl} should be detected as cl driver mode"
            );
        }
        for gnu in ["clang", "clang.exe", "clang-18", "gcc", "cc"] {
            assert!(
                !program_name_is_cl_driver(Path::new(gnu)),
                "{gnu} should not be detected as cl driver mode"
            );
        }
    }
}
