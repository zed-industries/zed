// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::cc_builder::CcBuilder;
use crate::OutputLib::{Crypto, Ssl};
use crate::{
    allow_prebuilt_nasm, cargo_env, effective_target, emit_warning, execute_command,
    get_crate_cflags, is_crt_static, is_fips_build, is_no_asm, is_no_pregenerated_src,
    optional_env, optional_env_optional_crate_target, sanitizer, set_env, set_env_for_target,
    should_build_jitter_entropy, target, target_arch, target_env, target_is_msvc, target_os,
    test_clang_cl_command, test_nasm_command, use_prebuilt_nasm, OutputLibType,
};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::is_lto_flag;

/// Strip LTO-related flags from a CFLAGS string. LTO flags interfere with the
/// FIPS delocator pipeline, which compiles C to assembly (`-S`) and processes it
/// with a Go tool. The combination of `-S` and `-flto` causes GCC to produce
/// GIMPLE-annotated output that the delocator cannot handle. Additionally, LTO
/// object files mixed with hand-written assembly in static libraries can cause
/// linker errors (e.g. `R_X86_64_32` relocations incompatible with PIE).
/// See also: aws-lc `CMakeLists.txt` which disables assembly when LTO is active.
fn strip_lto_flags(cflags: &str) -> String {
    cflags
        .split_whitespace()
        .filter(|flag| !is_lto_flag(flag))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) struct CmakeBuilder {
    manifest_dir: PathBuf,
    out_dir: PathBuf,
    build_prefix: Option<String>,
    output_lib_type: OutputLibType,
}

fn test_prebuilt_nasm_script(script_path: &Path) -> bool {
    // Call with no args - both scripts will exit with error, but we only care if they can execute
    execute_command(script_path.as_os_str(), &[]).executed
}

fn find_cmake_command() -> Option<OsString> {
    if let Some(cmake) = optional_env_optional_crate_target("CMAKE") {
        emit_warning(format!("CMAKE environment variable set: {}", cmake.clone()));
        if execute_command(cmake.as_ref(), &["--version".as_ref()]).status {
            Some(cmake.into())
        } else {
            None
        }
    } else if execute_command("cmake3".as_ref(), &["--version".as_ref()]).status {
        Some("cmake3".into())
    } else if execute_command("cmake".as_ref(), &["--version".as_ref()]).status {
        Some("cmake".into())
    } else {
        None
    }
}

impl CmakeBuilder {
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
        }
    }

    fn artifact_output_dir(&self) -> PathBuf {
        self.out_dir.join("build").join("artifacts")
    }

    fn get_cmake_config(&self) -> cmake::Config {
        cmake::Config::new(&self.manifest_dir)
    }

    fn apply_universal_build_options<'a>(
        &self,
        cmake_cfg: &'a mut cmake::Config,
    ) -> &'a cmake::Config {
        // Use the compiler options identified by CcBuilder
        let cc_builder = CcBuilder::new(
            self.manifest_dir.clone(),
            self.out_dir.clone(),
            self.build_prefix.clone(),
            self.output_lib_type,
        );
        let cc_build = cc::Build::new();
        let (is_cl_like, build_options) =
            cc_builder.collect_universal_build_options(&cc_build, true);
        for option in &build_options {
            option.apply_cmake(cmake_cfg, is_cl_like);
        }

        // CMake seeds `CMAKE_C_FLAGS` and `CMAKE_ASM_FLAGS` separately, so the
        // C-side prefix-map does not cover `.S` sources. In release-style
        // builds, mirror it with `asmflag()` when the compiler accepts
        // `-Wa,--debug-prefix-map=...`, and skip paths with spaces because the
        // flag must survive CMake and the assembler as one bare token.
        let opt_level = cargo_env("OPT_LEVEL");
        if !is_cl_like
            && (target_os() == "linux" || target_os().ends_with("bsd"))
            && !matches!(opt_level.as_str(), "0" | "1" | "2")
            && !self.manifest_dir.to_string_lossy().contains(' ')
        {
            let path_str = self.manifest_dir.display().to_string();
            let asm_flag = format!("-Wa,--debug-prefix-map={path_str}=");
            if cc_build.is_flag_supported(&asm_flag).unwrap_or(false) {
                cmake_cfg.asmflag(asm_flag);
            }
        }

        cmake_cfg
    }

    const GOCACHE_DIR_NAME: &'static str = "go-cache";

    #[allow(clippy::too_many_lines)]
    fn prepare_cmake_build(&self) -> cmake::Config {
        if is_fips_build() {
            unsafe {
                env::set_var("GOFLAGS", "-buildvcs=false");
            }
            if env::var("GOCACHE").is_err() {
                unsafe {
                    env::set_var(
                        "GOCACHE",
                        self.out_dir.join(Self::GOCACHE_DIR_NAME).as_os_str(),
                    );
                }
            }
        }

        let mut cmake_cfg = self.get_cmake_config();
        if let Some(generator) = optional_env_optional_crate_target("CMAKE_GENERATOR") {
            set_env("CMAKE_GENERATOR", generator);
        }

        if OutputLibType::default() == OutputLibType::Dynamic {
            cmake_cfg.define("BUILD_SHARED_LIBS", "1");
            if is_fips_build() {
                // The default flags include `-ffunction-sections` that can result in
                // dead code elimination dropping functions.
                cmake_cfg.no_default_flags(true);
            }
        } else {
            cmake_cfg.define("BUILD_SHARED_LIBS", "0");
        }

        if let Some(prefix) = &self.build_prefix {
            cmake_cfg.define("BORINGSSL_PREFIX", format!("{prefix}_"));
            let include_path = self.manifest_dir.join("generated-include");
            cmake_cfg.define(
                "BORINGSSL_PREFIX_HEADERS",
                include_path.display().to_string(),
            );
        }

        // Build flags that minimize our crate size.
        cmake_cfg.define("BUILD_TESTING", "OFF");
        cmake_cfg.define("BUILD_TOOL", "OFF");
        cmake_cfg.define("ENABLE_SOURCE_MODIFICATION", "OFF");
        if cfg!(feature = "ssl") {
            cmake_cfg.define("BUILD_LIBSSL", "ON");
        } else {
            cmake_cfg.define("BUILD_LIBSSL", "OFF");
        }
        if is_fips_build() {
            cmake_cfg.define("FIPS", "1");
        } else {
            if is_no_pregenerated_src() {
                // Go and Perl will be required.
                cmake_cfg.define("DISABLE_PERL", "OFF");
                cmake_cfg.define("DISABLE_GO", "OFF");
            } else {
                // Build flags that minimize our dependencies.
                cmake_cfg.define("DISABLE_PERL", "ON");
                cmake_cfg.define("DISABLE_GO", "ON");
            }
            if !should_build_jitter_entropy() {
                cmake_cfg.define("DISABLE_CPU_JITTER_ENTROPY", "ON");
            }

            if target_env() == "ohos" {
                Self::configure_open_harmony(&mut cmake_cfg);
            }
        }

        if is_no_asm() {
            let opt_level = cargo_env("OPT_LEVEL");
            if opt_level == "0" {
                cmake_cfg.define("OPENSSL_NO_ASM", "1");
            } else {
                panic!("AWS_LC_SYS_NO_ASM only allowed for debug builds!")
            }
        }

        Self::configure_sanitizer(&mut cmake_cfg);

        if let Some(cflags) = get_crate_cflags() {
            let cflags = if is_fips_build() {
                strip_lto_flags(&cflags)
            } else {
                cflags
            };
            // Set both the target-specific and base CFLAGS. cmake-rs reads
            // the base CFLAGS env var directly, so we must override it too.
            set_env_for_target("CFLAGS", &cflags);
            set_env("CFLAGS", &cflags);
        }

        // cmake-rs has logic that strips Optimization/Debug options that are passed via CFLAGS:
        // https://github.com/rust-lang/cmake-rs/issues/240
        // This breaks build configurations that generate warnings when optimizations
        // are disabled.
        Self::preserve_cflag_optimization_flags(&mut cmake_cfg);

        if target_os() == "windows" {
            if is_fips_build() {
                let opt_level = cargo_env("OPT_LEVEL");
                let build_type = if opt_level.eq("0") || opt_level.eq("1") || opt_level.eq("2") {
                    "RELWITHDEBINFO"
                } else {
                    "RELEASE"
                };
                cmake_cfg.define("CMAKE_BUILD_TYPE", build_type);
            } else if use_prebuilt_nasm() {
                self.configure_prebuilt_nasm(&mut cmake_cfg);
            }
            if target_is_msvc() {
                let mut msvcrt = String::from_str("MultiThreaded").unwrap();
                if is_crt_static() {
                    cmake_cfg.static_crt(true);
                    // When using static CRT on Windows MSVC, ignore missing PDB file warnings
                    // The static CRT libraries reference PDB files from Microsoft's build servers
                    // which are not available.
                    println!("cargo:rustc-link-arg=/ignore:4099");
                } else {
                    msvcrt.push_str("DLL");
                }
                cmake_cfg.define("CMAKE_MSVC_RUNTIME_LIBRARY", msvcrt.as_str());
            }
        }

        // Allow environment to specify CMake toolchain.
        if let Some(toolchain) = optional_env_optional_crate_target("CMAKE_TOOLCHAIN_FILE") {
            set_env_for_target("CMAKE_TOOLCHAIN_FILE", toolchain);

            return cmake_cfg;
        }
        // We only consider compiler CFLAGS when no cmake toolchain is set
        self.apply_universal_build_options(&mut cmake_cfg);

        // See issue: https://github.com/aws/aws-lc-rs/issues/453
        if target_os() == "windows" {
            self.configure_windows(&mut cmake_cfg);
        }

        // If the build environment vendor is Apple
        #[cfg(target_vendor = "apple")]
        {
            if target_arch() == "aarch64" {
                cmake_cfg.define("CMAKE_OSX_ARCHITECTURES", "arm64");
                cmake_cfg.define("CMAKE_SYSTEM_PROCESSOR", "arm64");
            }
            if target_arch() == "x86_64" {
                cmake_cfg.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
                cmake_cfg.define("CMAKE_SYSTEM_PROCESSOR", "x86_64");
            }
            if target_os().trim() == "ios" {
                cmake_cfg.define("CMAKE_SYSTEM_NAME", "iOS");
                if effective_target().ends_with("-ios-sim") || target_arch() == "x86_64" {
                    cmake_cfg.define("CMAKE_OSX_SYSROOT", "iphonesimulator");
                } else {
                    cmake_cfg.define("CMAKE_OSX_SYSROOT", "iphoneos");
                }
                cmake_cfg.define("CMAKE_THREAD_LIBS_INIT", "-lpthread");
            }
            if target_os().trim() == "macos" {
                cmake_cfg.define("CMAKE_SYSTEM_NAME", "Darwin");
                cmake_cfg.define("CMAKE_OSX_SYSROOT", "macosx");
            }
            if target_os().trim() == "tvos" {
                cmake_cfg.define("CMAKE_SYSTEM_NAME", "tvOS");
                if effective_target().ends_with("-tvos-sim") || target_arch() == "x86_64" {
                    cmake_cfg.define("CMAKE_OSX_SYSROOT", "appletvsimulator");
                } else {
                    cmake_cfg.define("CMAKE_OSX_SYSROOT", "appletvos");
                }
            }
        }

        if target_os() == "android" {
            self.configure_android(&mut cmake_cfg);
        }

        cmake_cfg
    }

    fn preserve_cflag_optimization_flags(cmake_cfg: &mut cmake::Config) {
        if let Some(cflags) = get_crate_cflags() {
            let split = cflags.split_whitespace();
            for arg in split {
                if arg.starts_with("-O") || arg.starts_with("/O") {
                    emit_warning(format!("Preserving optimization flag: {arg}"));
                    cmake_cfg.cflag(arg);
                }
            }
        }
    }

    #[allow(clippy::unused_self)]
    fn select_prebuilt_nasm_script(&self) -> PathBuf {
        let sh_script = self.manifest_dir.join("builder").join("prebuilt-nasm.sh");
        let bat_script = self.manifest_dir.join("builder").join("prebuilt-nasm.bat");

        // Test .sh first (more universal - works in MSYS2, WSL, native Unix)
        if test_prebuilt_nasm_script(&sh_script) {
            emit_warning("Selected prebuilt-nasm.sh (shell script can execute)");
            sh_script
        } else if test_prebuilt_nasm_script(&bat_script) {
            emit_warning(
                "Selected prebuilt-nasm.bat (batch script can execute, shell script cannot)",
            );
            bat_script
        } else {
            // Neither script could be verified as executable. This can happen in sandboxed or
            // restricted build environments where trial execution is blocked. We fall back to
            // selecting a script based on the host OS, which matches the behavior prior to
            // execution-based detection. If this fallback is incorrect, the build will fail
            // when CMake attempts to invoke the selected script.
            let fallback_script = if cfg!(target_os = "windows") {
                bat_script
            } else {
                sh_script
            };
            emit_warning(format!(
                "WARNING: Neither prebuilt-nasm.sh nor prebuilt-nasm.bat could be verified as \
                     executable. Falling back to target-based selection: {}. If the build fails \
                     during assembly, verify that the selected script is appropriate for your \
                     build environment.",
                fallback_script.file_name().unwrap().to_str().unwrap()
            ));
            fallback_script
        }
    }

    #[allow(clippy::unused_self)]
    fn configure_android(&self, _cmake_cfg: &mut cmake::Config) {
        // If we leave CMAKE_SYSTEM_PROCESSOR unset, then cmake-rs should handle properly setting
        // CMAKE_SYSTEM_NAME and CMAKE_SYSTEM_PROCESSOR:
        // https://github.com/rust-lang/cmake-rs/blob/b689783b5448966e810d515c798465f2e0ab56fd/src/lib.rs#L450-L499

        // Log relevant environment variables.
        if let Some(value) = optional_env_optional_crate_target("ANDROID_NDK_ROOT") {
            set_env("ANDROID_NDK_ROOT", value);
        } else {
            emit_warning("ANDROID_NDK_ROOT not set.");
        }
        if let Some(value) = optional_env_optional_crate_target("ANDROID_NDK") {
            set_env("ANDROID_NDK", value);
        } else {
            emit_warning("ANDROID_NDK not set.");
        }
        if let Some(value) = optional_env_optional_crate_target("ANDROID_STANDALONE_TOOLCHAIN") {
            set_env("ANDROID_STANDALONE_TOOLCHAIN", value);
        } else {
            emit_warning("ANDROID_STANDALONE_TOOLCHAIN not set.");
        }
    }

    #[allow(clippy::unused_self)]
    fn configure_windows(&self, cmake_cfg: &mut cmake::Config) {
        if is_fips_build() {
            cmake_cfg.generator("Ninja");
            let env_map = self
                .collect_vcvarsall_bat()
                .map_err(|x| panic!("{}", x))
                .unwrap();
            for (key, value) in env_map {
                cmake_cfg.env(key, value);
            }
        }

        match (target_env().as_str(), target_arch().as_str()) {
            ("msvc", "aarch64") => {
                // If CMAKE_GENERATOR is either not set or not set to "Ninja"
                let cmake_generator = optional_env("CMAKE_GENERATOR");
                if cmake_generator.is_none() || cmake_generator.unwrap().to_lowercase() != "ninja" {
                    // The following is not supported by the Ninja generator
                    cmake_cfg.generator_toolset(format!(
                        "ClangCL{}",
                        if cfg!(target_arch = "x86_64") {
                            ",host=x64"
                        } else {
                            ""
                        }
                    ));
                    cmake_cfg.define("CMAKE_GENERATOR_PLATFORM", "ARM64");
                }
                cmake_cfg.define("CMAKE_SYSTEM_NAME", "Windows");
                cmake_cfg.define("CMAKE_SYSTEM_PROCESSOR", "ARM64");
            }
            ("msvc", _) => {
                // No-op
            }
            (_, arch) => {
                cmake_cfg.define("CMAKE_SYSTEM_NAME", "Windows");
                cmake_cfg.define("CMAKE_SYSTEM_PROCESSOR", arch);
            }
        }

        // Workaround for win7-windows-gnu targets: the upstream C source gates the
        // Win7 compat path with `!defined(__MINGW32__)`. CMakeLists.txt already sets
        // _WIN32_WINNT for MinGW, but we must force AWSLC_WINDOWS_7_COMPAT until the
        // upstream fix lands: https://github.com/aws/aws-lc/pull/3239
        if target().contains("-win7-windows-gnu") {
            cmake_cfg.cflag("-DAWSLC_WINDOWS_7_COMPAT");
        }
    }

    /// Returns the architecture argument for `vcvarsall.bat`.
    ///
    /// In a Cargo build-script, `cfg!(target_arch)` is the **host** architecture
    /// (the machine running the build), while `target_arch()` (backed by
    /// `CARGO_CFG_TARGET_ARCH`) is the **target** architecture we are compiling for.
    ///
    /// `vcvarsall.bat` accepts a single token that encodes both:
    ///   - native   : `x64`, `arm64`, `x86`
    ///   - cross    : `<host>_<target>`, e.g. `x64_arm64`
    fn vcvarsall_arch() -> &'static str {
        let target = target_arch();
        match (
            cfg!(target_arch = "x86_64"),
            cfg!(target_arch = "aarch64"),
            target.as_str(),
        ) {
            // Host x64
            (true, _, "x86_64") => "x64",
            (true, _, "aarch64") => "x64_arm64",
            (true, _, "x86") => "x64_x86",
            // Host arm64
            (_, true, "aarch64") => "arm64",
            (_, true, "x86_64") => "arm64_x64",
            (_, true, "x86") => "arm64_x86",
            // Fallback
            _ => "x64",
        }
    }

    fn collect_vcvarsall_bat(&self) -> Result<HashMap<String, String>, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        let script_path = self.manifest_dir.join("builder").join("printenv.bat");
        let arch = OsString::from(Self::vcvarsall_arch());
        let result = execute_command(script_path.as_os_str(), &[arch.as_os_str()]);
        if !result.status {
            eprintln!("{}", result.stdout);
            return Err("Failed to run vcvarsall.bat.".to_owned());
        }
        eprintln!("{}", result.stdout);
        let lines = result.stdout.lines();
        for line in lines {
            if let Some((var, val)) = line.split_once('=') {
                map.insert(var.to_string(), val.to_string());
            }
        }
        Ok(map)
    }

    fn configure_prebuilt_nasm(&self, cmake_cfg: &mut cmake::Config) {
        emit_warning("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        emit_warning("!!!   Using pre-built NASM binaries   !!!");
        emit_warning("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

        let script_path = self.select_prebuilt_nasm_script();
        let script_path = script_path.display().to_string();
        let script_path = script_path.replace('\\', "/");

        cmake_cfg.define("CMAKE_ASM_NASM_COMPILER", script_path.as_str());
        // Without the following definition, the build fails with a message similar to the one
        // reported here: https://gitlab.kitware.com/cmake/cmake/-/issues/19453
        // The variables below were found in the associated fix:
        // https://gitlab.kitware.com/cmake/cmake/-/merge_requests/4257/diffs
        cmake_cfg.define(
            "CMAKE_ASM_NASM_COMPILE_OPTIONS_MSVC_RUNTIME_LIBRARY_MultiThreaded",
            "",
        );
        cmake_cfg.define(
            "CMAKE_ASM_NASM_COMPILE_OPTIONS_MSVC_RUNTIME_LIBRARY_MultiThreadedDLL",
            "",
        );
        cmake_cfg.define(
            "CMAKE_ASM_NASM_COMPILE_OPTIONS_MSVC_RUNTIME_LIBRARY_MultiThreadedDebug",
            "",
        );
        cmake_cfg.define(
            "CMAKE_ASM_NASM_COMPILE_OPTIONS_MSVC_RUNTIME_LIBRARY_MultiThreadedDebugDLL",
            "",
        );
        cmake_cfg.define(
            "CMAKE_ASM_NASM_COMPILE_OPTIONS_MSVC_DEBUG_INFORMATION_FORMAT_ProgramDatabase",
            "",
        );
    }

    /// Configure the `CMake` build for sanitizer instrumentation.
    ///
    /// Resolves the active sanitizer from either the `asan` Cargo feature flag
    /// (kept for backward compatibility) or the `AWS_LC_SYS_SANITIZER` /
    /// `AWS_LC_FIPS_SYS_SANITIZER` environment variable (preferred). Supported
    /// values: `asan`, `msan`, `tsan`.
    fn configure_sanitizer(cmake_cfg: &mut cmake::Config) {
        let feature_sanitizer = if cfg!(feature = "asan") {
            Some("asan")
        } else {
            None
        };
        let env_sanitizer = sanitizer();
        let env_sanitizer = env_sanitizer.as_deref();

        let active_sanitizer = match (feature_sanitizer, env_sanitizer) {
            (Some(f), Some(e)) if f != e => {
                panic!(
                    "Conflicting sanitizer configuration: feature flag '{f}' and \
                     AWS_LC_SYS_SANITIZER='{e}'. Remove one to resolve the conflict."
                );
            }
            (Some(f), _) => Some(f),
            (_, Some(e)) => Some(e),
            _ => None,
        };

        if let Some(san) = active_sanitizer {
            set_env_for_target("CC", "clang");
            set_env_for_target("CXX", "clang++");

            match san {
                "asan" => {
                    cmake_cfg.define("ASAN", "1");
                }
                "msan" => {
                    // AWS-LC's CMakeLists.txt correctly skips the FIPS
                    // delocator when MSAN is set, but bcm.c guards the
                    // integrity-test symbols with `#if !defined(OPENSSL_ASAN)`
                    // only — it does not check OPENSSL_MSAN. Until the
                    // upstream C guard is broadened, MSAN + FIPS will produce
                    // undefined-symbol link errors for BORINGSSL_bcm_text_*.
                    assert!(
                        !is_fips_build(),
                        "MemorySanitizer is not supported for FIPS builds. \
                         The FIPS integrity check in bcm.c lacks an \
                         OPENSSL_MSAN guard, causing undefined-symbol link \
                         errors (BORINGSSL_bcm_text_start/end/hash)."
                    );
                    cmake_cfg.define("MSAN", "1");
                }
                "tsan" => {
                    cmake_cfg.define("TSAN", "1");
                    // AWS-LC's CMakeLists.txt defines BORINGSSL_DISPATCH_TEST for
                    // non-FIPS, non-release builds. That macro enables non-atomic
                    // writes to a global BORINGSSL_function_hit[] array from ~15
                    // dispatch points (C and assembly), which TSAN flags as data
                    // races. The writes are benign (idempotent store of 1), but
                    // suppressing every function name is impractical. Force the C
                    // library to build in Release mode so the macro is never
                    // defined. This matches AWS-LC's own sanitizer CI, which
                    // always uses -DCMAKE_BUILD_TYPE=Release.
                    cmake_cfg.profile("Release");
                }
                other => {
                    panic!("Unsupported sanitizer: '{other}'. Supported values: asan, msan, tsan")
                }
            }
        }
    }

    fn configure_open_harmony(cmake_cfg: &mut cmake::Config) {
        let mut cflags = vec!["-Wno-unused-command-line-argument"];
        let mut asmflags = vec![];

        // If a toolchain is not specified by the environment
        if optional_env_optional_crate_target("CMAKE_TOOLCHAIN_FILE").is_none() {
            if let Ok(ndk) = env::var("OHOS_NDK_HOME") {
                set_env_for_target(
                    "CMAKE_TOOLCHAIN_FILE",
                    format!("{ndk}/native/build/cmake/ohos.toolchain.cmake"),
                );
            } else if let Ok(sdk) = env::var("OHOS_SDK_NATIVE") {
                set_env_for_target(
                    "CMAKE_TOOLCHAIN_FILE",
                    format!("{sdk}/build/cmake/ohos.toolchain.cmake"),
                );
            } else {
                emit_warning(
                    "Neither OHOS_NDK_HOME nor OHOS_SDK_NATIVE are set! No toolchain found.",
                );
            }
        }

        match effective_target().as_str() {
            "aarch64-unknown-linux-ohos" => {
                cmake_cfg.define("OHOS_ARCH", "arm64-v8a");
            }
            "armv7-unknown-linux-ohos" => {
                const ARM7_FLAGS: [&str; 6] = [
                    "-march=armv7-a",
                    "-mfloat-abi=softfp",
                    "-mtune=generic-armv7-a",
                    "-mthumb",
                    "-mfpu=neon",
                    "-DHAVE_NEON",
                ];
                cflags.extend(ARM7_FLAGS);
                asmflags.extend(ARM7_FLAGS);
                cmake_cfg.define("OHOS_ARCH", "armeabi-v7a");
            }
            "x86_64-unknown-linux-ohos" => {
                const X86_64_FLAGS: [&str; 3] = ["-msse4.1", "-DHAVE_NEON_X86", "-DHAVE_NEON"];
                cflags.extend(X86_64_FLAGS);
                asmflags.extend(X86_64_FLAGS);
                cmake_cfg.define("OHOS_ARCH", "x86_64");
            }
            ohos_target => {
                emit_warning(format!("Target: {ohos_target} is not support yet!").as_str());
            }
        }
        cmake_cfg
            .cflag(cflags.join(" ").as_str())
            .cxxflag(cflags.join(" ").as_str())
            .asmflag(asmflags.join(" ").as_str());
    }

    fn build_library(&self) -> PathBuf {
        self.prepare_cmake_build()
            .configure_arg("--no-warn-unused-cli")
            .build()
    }
}

impl crate::Builder for CmakeBuilder {
    fn check_dependencies(&self) -> Result<(), String> {
        let mut missing_dependency = false;
        if target_os() == "windows" && target_arch() == "x86_64" {
            if is_no_asm() && Some(true) == allow_prebuilt_nasm() {
                eprintln!(
                    "Build environment has both `AWS_LC_SYS_PREBUILT_NASM` and `AWS_LC_SYS_NO_ASM` set.\
                Please remove one of these environment variables.
                See User Guide: https://aws.github.io/aws-lc-rs/index.html"
                );
            }
            if !is_no_asm() && !test_nasm_command() && !use_prebuilt_nasm() {
                eprintln!(
                    "Consider installing NASM or setting `AWS_LC_SYS_PREBUILT_NASM` in the build environment.\
                See User Guide: https://aws.github.io/aws-lc-rs/index.html"
                );
                eprintln!("Missing dependency: nasm");
                missing_dependency = true;
            }
        }
        if target_os() == "windows"
            && target_arch() == "aarch64"
            && target_is_msvc()
            && !test_clang_cl_command()
        {
            eprintln!("Missing dependency: clang-cl");
            missing_dependency = true;
        }
        if let Some(cmake_cmd) = find_cmake_command() {
            unsafe {
                env::set_var("CMAKE", cmake_cmd);
            }
        } else {
            eprintln!("Missing dependency: cmake");
            missing_dependency = true;
        }

        if missing_dependency {
            return Err("Required build dependency is missing. Halting build.".to_owned());
        }

        Ok(())
    }
    fn build(&self) -> Result<(), String> {
        self.build_library();

        println!(
            "cargo:rustc-link-search=native={}",
            self.artifact_output_dir().display()
        );

        println!(
            "cargo:rustc-link-lib={}={}",
            self.output_lib_type.rust_link_lib_kind(),
            Crypto.libname(&self.build_prefix)
        );

        if cfg!(feature = "ssl") {
            println!(
                "cargo:rustc-link-lib={}={}",
                self.output_lib_type.rust_link_lib_kind(),
                Ssl.libname(&self.build_prefix)
            );
        }

        crate::emit_source_build_metadata(&self.manifest_dir);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "CMake"
    }
}

#[cfg(test)]
mod tests {
    use super::strip_lto_flags;

    #[test]
    fn test_strip_lto_flags() {
        // RPM-style CFLAGS with -flto=auto and -ffat-lto-objects
        let input = "-O2 -ftree-vectorize -flto=auto -ffat-lto-objects -fexceptions -g -fPIC";
        assert_eq!(
            strip_lto_flags(input),
            "-O2 -ftree-vectorize -fexceptions -g -fPIC"
        );

        // Various LTO flag forms
        assert_eq!(strip_lto_flags("-flto -O2"), "-O2");
        assert_eq!(strip_lto_flags("-flto=thin -O2"), "-O2");
        assert_eq!(strip_lto_flags("-flto=full -O2"), "-O2");
        assert_eq!(strip_lto_flags("-flto=4 -O2"), "-O2");
        assert_eq!(strip_lto_flags("-fno-fat-lto-objects -O2"), "-O2");

        // No LTO flags - passthrough
        let no_lto = "-O2 -fPIC -g";
        assert_eq!(strip_lto_flags(no_lto), no_lto);

        // Empty input
        assert_eq!(strip_lto_flags(""), "");
    }
}
