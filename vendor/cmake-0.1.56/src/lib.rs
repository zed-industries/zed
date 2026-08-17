//! A build dependency for running `cmake` to build a native library
//!
//! This crate provides some necessary boilerplate and shim support for running
//! the system `cmake` command to build a native library. It will add
//! appropriate cflags for building code to link into Rust, handle cross
//! compilation, and use the necessary generator for the platform being
//! targeted.
//!
//! The builder-style configuration allows for various variables and such to be
//! passed down into the build as well.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! cmake = "0.1"
//! ```
//!
//! ## Examples
//!
//! ```no_run
//! use cmake;
//!
//! // Builds the project in the directory located in `libfoo`, installing it
//! // into $OUT_DIR
//! let dst = cmake::build("libfoo");
//!
//! println!("cargo:rustc-link-search=native={}", dst.display());
//! println!("cargo:rustc-link-lib=static=foo");
//! ```
//!
//! ```no_run
//! use cmake::Config;
//!
//! let dst = Config::new("libfoo")
//!                  .define("FOO", "BAR")
//!                  .cflag("-foo")
//!                  .build();
//! println!("cargo:rustc-link-search=native={}", dst.display());
//! println!("cargo:rustc-link-lib=static=foo");
//! ```

#![deny(missing_docs)]

extern crate cc;

use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Builder style configuration for a pending CMake build.
pub struct Config {
    path: PathBuf,
    generator: Option<OsString>,
    generator_toolset: Option<OsString>,
    cflags: OsString,
    cxxflags: OsString,
    asmflags: OsString,
    defines: Vec<(OsString, OsString)>,
    deps: Vec<String>,
    target: Option<String>,
    host: Option<String>,
    out_dir: Option<PathBuf>,
    profile: Option<String>,
    configure_args: Vec<OsString>,
    build_args: Vec<OsString>,
    cmake_target: Option<String>,
    env: Vec<(OsString, OsString)>,
    static_crt: Option<bool>,
    uses_cxx11: bool,
    always_configure: bool,
    no_build_target: bool,
    no_default_flags: bool,
    verbose_cmake: bool,
    verbose_make: bool,
    pic: Option<bool>,
    c_cfg: Option<cc::Build>,
    cxx_cfg: Option<cc::Build>,
    env_cache: HashMap<String, Option<OsString>>,
}

/// Builds the native library rooted at `path` with the default cmake options.
/// This will return the directory in which the library was installed.
///
/// # Examples
///
/// ```no_run
/// use cmake;
///
/// // Builds the project in the directory located in `libfoo`, installing it
/// // into $OUT_DIR
/// let dst = cmake::build("libfoo");
///
/// println!("cargo:rustc-link-search=native={}", dst.display());
/// println!("cargo:rustc-link-lib=static=foo");
/// ```
///
pub fn build<P: AsRef<Path>>(path: P) -> PathBuf {
    Config::new(path.as_ref()).build()
}

impl Config {
    /// Return explicitly set profile or infer `CMAKE_BUILD_TYPE` from Rust's compilation profile.
    ///
    /// * if `opt-level=0` then `CMAKE_BUILD_TYPE=Debug`,
    /// * if `opt-level={1,2,3}` and:
    ///   * `debug=false` then `CMAKE_BUILD_TYPE=Release`
    ///   * otherwise `CMAKE_BUILD_TYPE=RelWithDebInfo`
    /// * if `opt-level={s,z}` then `CMAKE_BUILD_TYPE=MinSizeRel`
    pub fn get_profile(&self) -> &str {
        if let Some(profile) = self.profile.as_ref() {
            profile
        } else {
            // Determine Rust's profile, optimization level, and debug info:
            #[derive(PartialEq)]
            enum RustProfile {
                Debug,
                Release,
            }
            #[derive(PartialEq, Debug)]
            enum OptLevel {
                Debug,
                Release,
                Size,
            }

            let rust_profile = match &getenv_unwrap("PROFILE")[..] {
                "debug" => RustProfile::Debug,
                "release" | "bench" => RustProfile::Release,
                unknown => {
                    eprintln!(
                        "Warning: unknown Rust profile={}; defaulting to a release build.",
                        unknown
                    );
                    RustProfile::Release
                }
            };

            let opt_level = match &getenv_unwrap("OPT_LEVEL")[..] {
                "0" => OptLevel::Debug,
                "1" | "2" | "3" => OptLevel::Release,
                "s" | "z" => OptLevel::Size,
                unknown => {
                    let default_opt_level = match rust_profile {
                        RustProfile::Debug => OptLevel::Debug,
                        RustProfile::Release => OptLevel::Release,
                    };
                    eprintln!(
                        "Warning: unknown opt-level={}; defaulting to a {:?} build.",
                        unknown, default_opt_level
                    );
                    default_opt_level
                }
            };

            let debug_info: bool = match &getenv_unwrap("DEBUG")[..] {
                "false" => false,
                "true" => true,
                unknown => {
                    eprintln!("Warning: unknown debug={}; defaulting to `true`.", unknown);
                    true
                }
            };

            match (opt_level, debug_info) {
                (OptLevel::Debug, _) => "Debug",
                (OptLevel::Release, false) => "Release",
                (OptLevel::Release, true) => "RelWithDebInfo",
                (OptLevel::Size, _) => "MinSizeRel",
            }
        }
    }

    /// Creates a new blank set of configuration to build the project specified
    /// at the path `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Config {
        Config {
            path: env::current_dir().unwrap().join(path),
            generator: None,
            generator_toolset: None,
            no_default_flags: false,
            cflags: OsString::new(),
            cxxflags: OsString::new(),
            asmflags: OsString::new(),
            defines: Vec::new(),
            deps: Vec::new(),
            profile: None,
            out_dir: None,
            target: None,
            host: None,
            configure_args: Vec::new(),
            build_args: Vec::new(),
            cmake_target: None,
            env: Vec::new(),
            static_crt: None,
            uses_cxx11: false,
            always_configure: true,
            no_build_target: false,
            verbose_cmake: false,
            verbose_make: false,
            pic: None,
            c_cfg: None,
            cxx_cfg: None,
            env_cache: HashMap::new(),
        }
    }

    /// Sets flag for PIC. Otherwise use cc::Build platform default
    pub fn pic(&mut self, explicit_flag: bool) -> &mut Config {
        self.pic = Some(explicit_flag);
        self
    }

    /// Sets the build-tool generator (`-G`) for this compilation.
    ///
    /// If unset, this crate will use the `CMAKE_GENERATOR` environment variable
    /// if set. Otherwise, it will guess the best generator to use based on the
    /// build target.
    pub fn generator<T: AsRef<OsStr>>(&mut self, generator: T) -> &mut Config {
        self.generator = Some(generator.as_ref().to_owned());
        self
    }

    /// Sets the toolset name (-T) if supported by generator.
    /// Can be used to compile with Clang/LLVM instead of msvc when Visual Studio generator is selected.
    ///
    /// If unset, will use the default toolset of the selected generator.
    pub fn generator_toolset<T: AsRef<OsStr>>(&mut self, toolset_name: T) -> &mut Config {
        self.generator_toolset = Some(toolset_name.as_ref().to_owned());
        self
    }

    /// Adds a custom flag to pass down to the C compiler, supplementing those
    /// that this library already passes.
    pub fn cflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut Config {
        self.cflags.push(" ");
        self.cflags.push(flag.as_ref());
        self
    }

    /// Adds a custom flag to pass down to the C++ compiler, supplementing those
    /// that this library already passes.
    pub fn cxxflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut Config {
        self.cxxflags.push(" ");
        self.cxxflags.push(flag.as_ref());
        self
    }

    /// Adds a custom flag to pass down to the ASM compiler, supplementing those
    /// that this library already passes.
    pub fn asmflag<P: AsRef<OsStr>>(&mut self, flag: P) -> &mut Config {
        self.asmflags.push(" ");
        self.asmflags.push(flag.as_ref());
        self
    }

    /// Adds a new `-D` flag to pass to cmake during the generation step.
    pub fn define<K, V>(&mut self, k: K, v: V) -> &mut Config
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.defines
            .push((k.as_ref().to_owned(), v.as_ref().to_owned()));
        self
    }

    /// Registers a dependency for this compilation on the native library built
    /// by Cargo previously.
    ///
    /// This registration will update the `CMAKE_PREFIX_PATH` environment
    /// variable for the [`build`][Self::build] system generation step.  The
    /// path will be updated to include the content of the environment
    /// variable `DEP_XXX_ROOT`, where `XXX` is replaced with the uppercased
    /// value of `dep` (if that variable exists).
    pub fn register_dep(&mut self, dep: &str) -> &mut Config {
        self.deps.push(dep.to_string());
        self
    }

    /// Sets the target triple for this compilation.
    ///
    /// This is automatically scraped from `$TARGET` which is set for Cargo
    /// build scripts so it's not necessary to call this from a build script.
    pub fn target(&mut self, target: &str) -> &mut Config {
        self.target = Some(target.to_string());
        self
    }

    /// Disables the cmake target option for this compilation.
    ///
    /// Note that this isn't related to the target triple passed to the compiler!
    pub fn no_build_target(&mut self, no_build_target: bool) -> &mut Config {
        self.no_build_target = no_build_target;
        self
    }

    /// Disables the generation of default compiler flags. The default compiler
    /// flags may cause conflicts in some cross compiling scenarios.
    pub fn no_default_flags(&mut self, no_default_flags: bool) -> &mut Config {
        self.no_default_flags = no_default_flags;
        self
    }

    /// Sets the host triple for this compilation.
    ///
    /// This is automatically scraped from `$HOST` which is set for Cargo
    /// build scripts so it's not necessary to call this from a build script.
    pub fn host(&mut self, host: &str) -> &mut Config {
        self.host = Some(host.to_string());
        self
    }

    /// Sets the output directory for this compilation.
    ///
    /// This is automatically scraped from `$OUT_DIR` which is set for Cargo
    /// build scripts so it's not necessary to call this from a build script.
    pub fn out_dir<P: AsRef<Path>>(&mut self, out: P) -> &mut Config {
        self.out_dir = Some(out.as_ref().to_path_buf());
        self
    }

    /// Sets the `CMAKE_BUILD_TYPE=build_type` variable.
    ///
    /// By default, this value is automatically inferred from Rust's compilation
    /// profile as follows:
    ///
    /// * if `opt-level=0` then `CMAKE_BUILD_TYPE=Debug`,
    /// * if `opt-level={1,2,3}` and:
    ///   * `debug=false` then `CMAKE_BUILD_TYPE=Release`
    ///   * otherwise `CMAKE_BUILD_TYPE=RelWithDebInfo`
    /// * if `opt-level={s,z}` then `CMAKE_BUILD_TYPE=MinSizeRel`
    pub fn profile(&mut self, profile: &str) -> &mut Config {
        self.profile = Some(profile.to_string());
        self
    }

    /// Configures whether the /MT flag or the /MD flag will be passed to msvc build tools.
    ///
    /// This option defaults to `false`, and affect only msvc targets.
    pub fn static_crt(&mut self, static_crt: bool) -> &mut Config {
        self.static_crt = Some(static_crt);
        self
    }

    /// Add an argument to the `cmake` configure step
    pub fn configure_arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut Config {
        self.configure_args.push(arg.as_ref().to_owned());
        self
    }

    /// Add an argument to the final `cmake` build step
    pub fn build_arg<A: AsRef<OsStr>>(&mut self, arg: A) -> &mut Config {
        self.build_args.push(arg.as_ref().to_owned());
        self
    }

    /// Configure an environment variable for the `cmake` processes spawned by
    /// this crate in the `build` step.
    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut Config
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.env
            .push((key.as_ref().to_owned(), value.as_ref().to_owned()));
        self
    }

    /// Sets the build target for the final `cmake` build step, this will
    /// default to "install" if not specified.
    pub fn build_target(&mut self, target: &str) -> &mut Config {
        self.cmake_target = Some(target.to_string());
        self
    }

    /// Alters the default target triple on OSX to ensure that c++11 is
    /// available. Does not change the target triple if it is explicitly
    /// specified.
    ///
    /// This does not otherwise affect any CXX flags, i.e. it does not set
    /// -std=c++11 or -stdlib=libc++.
    #[deprecated = "no longer does anything, C++ is determined based on `cc::Build`, and the macOS issue has been fixed upstream"]
    pub fn uses_cxx11(&mut self) -> &mut Config {
        self.uses_cxx11 = true;
        self
    }

    /// Forces CMake to always run before building the custom target.
    ///
    /// In some cases, when you have a big project, you can disable
    /// subsequents runs of cmake to make `cargo build` faster.
    pub fn always_configure(&mut self, always_configure: bool) -> &mut Config {
        self.always_configure = always_configure;
        self
    }

    /// Sets very verbose output.
    pub fn very_verbose(&mut self, value: bool) -> &mut Config {
        self.verbose_cmake = value;
        self.verbose_make = value;
        self
    }

    // Simple heuristic to determine if we're cross-compiling using the Android
    // NDK toolchain file.
    fn uses_android_ndk(&self) -> bool {
        // `ANDROID_ABI` is the only required flag:
        // https://developer.android.com/ndk/guides/cmake#android_abi
        self.defined("ANDROID_ABI")
            && self.defines.iter().any(|(flag, value)| {
                flag == "CMAKE_TOOLCHAIN_FILE"
                    && Path::new(value).file_name() == Some("android.toolchain.cmake".as_ref())
            })
    }

    /// Initializes the C build configuration.
    pub fn init_c_cfg(&mut self, c_cfg: cc::Build) -> &mut Config {
        self.c_cfg = Some(c_cfg);
        self
    }

    /// Initializes the C++ build configuration.
    pub fn init_cxx_cfg(&mut self, cxx_cfg: cc::Build) -> &mut Config {
        self.cxx_cfg = Some(cxx_cfg);
        self
    }

    /// Run this configuration, compiling the library with all the configured
    /// options.
    ///
    /// This will run both the build system generator command as well as the
    /// command to build the library.
    pub fn build(&mut self) -> PathBuf {
        let target = match self.target.clone() {
            Some(t) => t,
            None => getenv_unwrap("TARGET"),
        };
        let host = self.host.clone().unwrap_or_else(|| getenv_unwrap("HOST"));

        // Some decisions later on are made if CMAKE_TOOLCHAIN_FILE is defined,
        // so we need to read it from the environment variables from the beginning.
        if !self.defined("CMAKE_TOOLCHAIN_FILE") {
            if let Some(s) = self.getenv_target_os("CMAKE_TOOLCHAIN_FILE") {
                self.define("CMAKE_TOOLCHAIN_FILE", s);
            } else if target.contains("redox") {
                if !self.defined("CMAKE_SYSTEM_NAME") {
                    self.define("CMAKE_SYSTEM_NAME", "Generic");
                }
            } else if target != host && !self.defined("CMAKE_SYSTEM_NAME") {
                // Set CMAKE_SYSTEM_NAME and CMAKE_SYSTEM_PROCESSOR when cross compiling
                let os = getenv_unwrap("CARGO_CFG_TARGET_OS");
                let arch = getenv_unwrap("CARGO_CFG_TARGET_ARCH");
                // CMAKE_SYSTEM_NAME list
                // https://gitlab.kitware.com/cmake/cmake/-/issues/21489#note_1077167
                //
                // CMAKE_SYSTEM_PROCESSOR
                // some of the values come from https://en.wikipedia.org/wiki/Uname
                let (system_name, system_processor) = match (os.as_str(), arch.as_str()) {
                    ("android", "arm") => ("Android", "armv7-a"),
                    ("android", "x86") => ("Android", "i686"),
                    ("android", arch) => ("Android", arch),
                    ("dragonfly", arch) => ("DragonFly", arch),
                    ("macos", "aarch64") => ("Darwin", "arm64"),
                    ("macos", arch) => ("Darwin", arch),
                    ("freebsd", "x86_64") => ("FreeBSD", "amd64"),
                    ("freebsd", arch) => ("FreeBSD", arch),
                    ("fuchsia", arch) => ("Fuchsia", arch),
                    ("haiku", arch) => ("Haiku", arch),
                    ("ios", "aarch64") => ("iOS", "arm64"),
                    ("ios", arch) => ("iOS", arch),
                    ("linux", arch) => {
                        let name = "Linux";
                        match arch {
                            "powerpc" => (name, "ppc"),
                            "powerpc64" => (name, "ppc64"),
                            "powerpc64le" => (name, "ppc64le"),
                            _ => (name, arch),
                        }
                    }
                    ("netbsd", arch) => ("NetBSD", arch),
                    ("openbsd", "x86_64") => ("OpenBSD", "amd64"),
                    ("openbsd", arch) => ("OpenBSD", arch),
                    ("solaris", arch) => ("SunOS", arch),
                    ("tvos", "aarch64") => ("tvOS", "arm64"),
                    ("tvos", arch) => ("tvOS", arch),
                    ("visionos", "aarch64") => ("visionOS", "arm64"),
                    ("visionos", arch) => ("visionOS", arch),
                    ("watchos", "aarch64") => ("watchOS", "arm64"),
                    ("watchos", arch) => ("watchOS", arch),
                    ("windows", "x86_64") => ("Windows", "AMD64"),
                    ("windows", "x86") => ("Windows", "X86"),
                    ("windows", "aarch64") => ("Windows", "ARM64"),
                    ("none", arch) => ("Generic", arch),
                    // Others
                    (os, arch) => (os, arch),
                };
                self.define("CMAKE_SYSTEM_NAME", system_name);
                self.define("CMAKE_SYSTEM_PROCESSOR", system_processor);
            }
        }

        let generator = self
            .generator
            .clone()
            .or_else(|| self.getenv_target_os("CMAKE_GENERATOR"));

        let msvc = target.contains("msvc");
        let ndk = self.uses_android_ndk();
        let mut c_cfg = self.c_cfg.clone().unwrap_or_default();
        c_cfg
            .cargo_metadata(false)
            .cpp(false)
            .opt_level(0)
            .debug(false)
            .warnings(false)
            .host(&host)
            .no_default_flags(ndk || self.no_default_flags);
        if !ndk {
            c_cfg.target(&target);
        }
        let mut cxx_cfg = self.cxx_cfg.clone().unwrap_or_default();
        cxx_cfg
            .cargo_metadata(false)
            .cpp(true)
            .opt_level(0)
            .debug(false)
            .warnings(false)
            .host(&host)
            .no_default_flags(ndk || self.no_default_flags);
        if !ndk {
            cxx_cfg.target(&target);
        }
        if let Some(static_crt) = self.static_crt {
            c_cfg.static_crt(static_crt);
            cxx_cfg.static_crt(static_crt);
        }
        if let Some(explicit_flag) = self.pic {
            c_cfg.pic(explicit_flag);
            cxx_cfg.pic(explicit_flag);
        }
        let c_compiler = c_cfg.get_compiler();
        let cxx_compiler = cxx_cfg.get_compiler();
        let asm_compiler = c_cfg.get_compiler();

        let dst = self
            .out_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(getenv_unwrap("OUT_DIR")));

        let build_dir = try_canonicalize(&dst.join("build"));

        self.maybe_clear(&build_dir);
        let _ = fs::create_dir_all(&build_dir);

        // Add all our dependencies to our cmake paths
        let mut cmake_prefix_path = Vec::new();
        for dep in &self.deps {
            let dep = dep.to_uppercase().replace('-', "_");
            if let Some(root) = env::var_os(format!("DEP_{}_ROOT", dep)) {
                cmake_prefix_path.push(PathBuf::from(root));
            }
        }
        let system_prefix = self
            .getenv_target_os("CMAKE_PREFIX_PATH")
            .unwrap_or_default();
        cmake_prefix_path.extend(env::split_paths(&system_prefix));
        let cmake_prefix_path = env::join_paths(&cmake_prefix_path).unwrap();

        // Build up the first cmake command to build the build system.
        let mut cmd = self.cmake_configure_command(&target);

        let version = Version::from_command(cmd.get_program()).unwrap_or_default();

        if self.verbose_cmake {
            cmd.arg("-Wdev");
            cmd.arg("--debug-output");
        }

        cmd.arg(&self.path).current_dir(&build_dir);

        if version >= Version::new(3, 13) {
            cmd.arg("-B").arg(&build_dir);
        }

        let mut is_ninja = false;
        if let Some(ref generator) = generator {
            is_ninja = generator.to_string_lossy().contains("Ninja");
        }
        if target.contains("windows-gnu") {
            if host.contains("windows") {
                // On MinGW we need to coerce cmake to not generate a visual
                // studio build system but instead use makefiles that MinGW can
                // use to build.
                if generator.is_none() {
                    // If make.exe isn't found, that means we may be using a MinGW
                    // toolchain instead of a MSYS2 toolchain. If neither is found,
                    // the build cannot continue.
                    let has_msys2 = Command::new("make")
                        .arg("--version")
                        .output()
                        .err()
                        .map(|e| e.kind() != ErrorKind::NotFound)
                        .unwrap_or(true);
                    let has_mingw32 = Command::new("mingw32-make")
                        .arg("--version")
                        .output()
                        .err()
                        .map(|e| e.kind() != ErrorKind::NotFound)
                        .unwrap_or(true);

                    let generator = match (has_msys2, has_mingw32) {
                        (true, _) => "MSYS Makefiles",
                        (false, true) => "MinGW Makefiles",
                        (false, false) => fail("no valid generator found for GNU toolchain; MSYS or MinGW must be installed")
                    };

                    cmd.arg("-G").arg(generator);
                }
            } else {
                // If we're cross compiling onto windows, then set some
                // variables which will hopefully get things to succeed. Some
                // systems may need the `windres` or `dlltool` variables set, so
                // set them if possible.
                if !self.defined("CMAKE_RC_COMPILER") {
                    let exe = find_exe(c_compiler.path());
                    if let Some(name) = exe.file_name().unwrap().to_str() {
                        let name = name.replace("gcc", "windres");
                        let windres = exe.with_file_name(name);
                        if windres.is_file() {
                            let mut arg = OsString::from("-DCMAKE_RC_COMPILER=");
                            arg.push(&windres);
                            cmd.arg(arg);
                        }
                    }
                }
            }
        } else if msvc {
            // If we're on MSVC we need to be sure to use the right generator or
            // otherwise we won't get 32/64 bit correct automatically.
            // This also guarantees that NMake generator isn't chosen implicitly.
            let using_nmake_generator = if let Some(g) = &generator {
                g == "NMake Makefiles" || g == "NMake Makefiles JOM"
            } else {
                cmd.arg("-G").arg(self.visual_studio_generator(&target));
                false
            };
            if !is_ninja && !using_nmake_generator {
                if target.contains("x86_64") {
                    if self.generator_toolset.is_none() {
                        cmd.arg("-Thost=x64");
                    }
                    cmd.arg("-Ax64");
                } else if target.contains("thumbv7a") {
                    if self.generator_toolset.is_none() {
                        cmd.arg("-Thost=x64");
                    }
                    cmd.arg("-Aarm");
                } else if target.contains("aarch64") {
                    if self.generator_toolset.is_none() {
                        cmd.arg("-Thost=x64");
                    }
                    cmd.arg("-AARM64");
                } else if target.contains("i686") {
                    if self.generator_toolset.is_none() {
                        cmd.arg("-Thost=x86");
                    }
                    cmd.arg("-AWin32");
                } else {
                    panic!("unsupported msvc target: {}", target);
                }
            }
        } else if target.contains("darwin") && !self.defined("CMAKE_OSX_ARCHITECTURES") {
            if target.contains("x86_64") {
                cmd.arg("-DCMAKE_OSX_ARCHITECTURES=x86_64");
            } else if target.contains("aarch64") {
                cmd.arg("-DCMAKE_OSX_ARCHITECTURES=arm64");
            } else {
                panic!("unsupported darwin target: {}", target);
            }
        }
        if let Some(ref generator) = generator {
            cmd.arg("-G").arg(generator);
        }
        if let Some(ref generator_toolset) = self.generator_toolset {
            cmd.arg("-T").arg(generator_toolset);
        }
        let profile = self.get_profile().to_string();
        for (k, v) in &self.defines {
            let mut os = OsString::from("-D");
            os.push(k);
            os.push("=");
            os.push(v);
            cmd.arg(os);
        }

        if !self.defined("CMAKE_INSTALL_PREFIX") {
            let mut dstflag = OsString::from("-DCMAKE_INSTALL_PREFIX=");
            dstflag.push(&dst);
            cmd.arg(dstflag);
        }

        let build_type = self
            .defines
            .iter()
            .find(|&(a, _)| a == "CMAKE_BUILD_TYPE")
            .map(|x| x.1.to_str().unwrap())
            .unwrap_or(&profile);
        let build_type_upcase = build_type
            .chars()
            .flat_map(|c| c.to_uppercase())
            .collect::<String>();

        {
            // let cmake deal with optimization/debuginfo
            let skip_arg = |arg: &OsStr| match arg.to_str() {
                Some(s) => s.starts_with("-O") || s.starts_with("/O") || s == "-g",
                None => false,
            };
            let mut set_compiler = |kind: &str, compiler: &cc::Tool, extra: &OsString| {
                let flag_var = format!("CMAKE_{}_FLAGS", kind);
                let tool_var = format!("CMAKE_{}_COMPILER", kind);
                if !self.defined(&flag_var) {
                    let mut flagsflag = OsString::from("-D");
                    flagsflag.push(&flag_var);
                    flagsflag.push("=");
                    flagsflag.push(extra);
                    for arg in compiler.args() {
                        if skip_arg(arg) {
                            continue;
                        }
                        flagsflag.push(" ");
                        flagsflag.push(arg);
                    }
                    cmd.arg(flagsflag);
                }

                // The visual studio generator apparently doesn't respect
                // `CMAKE_C_FLAGS` but does respect `CMAKE_C_FLAGS_RELEASE` and
                // such. We need to communicate /MD vs /MT, so set those vars
                // here.
                //
                // Note that for other generators, though, this *overrides*
                // things like the optimization flags, which is bad.
                if generator.is_none() && msvc {
                    let flag_var_alt = format!("CMAKE_{}_FLAGS_{}", kind, build_type_upcase);
                    if !self.defined(&flag_var_alt) {
                        let mut flagsflag = OsString::from("-D");
                        flagsflag.push(&flag_var_alt);
                        flagsflag.push("=");
                        flagsflag.push(extra);
                        for arg in compiler.args() {
                            if skip_arg(arg) {
                                continue;
                            }
                            flagsflag.push(" ");
                            flagsflag.push(arg);
                        }
                        cmd.arg(flagsflag);
                    }
                }

                // Apparently cmake likes to have an absolute path to the
                // compiler as otherwise it sometimes thinks that this variable
                // changed as it thinks the found compiler, /usr/bin/cc,
                // differs from the specified compiler, cc. Not entirely sure
                // what's up, but at least this means cmake doesn't get
                // confused?
                //
                // Also specify this on Windows only if we use MSVC with Ninja,
                // as it's not needed for MSVC with Visual Studio generators and
                // for MinGW it doesn't really vary.
                if !self.defined("CMAKE_TOOLCHAIN_FILE")
                    && !self.defined(&tool_var)
                    && (env::consts::FAMILY != "windows" || (msvc && is_ninja))
                {
                    let mut ccompiler = OsString::from("-D");
                    ccompiler.push(&tool_var);
                    ccompiler.push("=");
                    ccompiler.push(find_exe(compiler.path()));
                    #[cfg(windows)]
                    {
                        // CMake doesn't like unescaped `\`s in compiler paths
                        // so we either have to escape them or replace with `/`s.
                        use std::os::windows::ffi::{OsStrExt, OsStringExt};
                        let wchars = ccompiler
                            .encode_wide()
                            .map(|wchar| {
                                if wchar == b'\\' as u16 {
                                    '/' as u16
                                } else {
                                    wchar
                                }
                            })
                            .collect::<Vec<_>>();
                        ccompiler = OsString::from_wide(&wchars);
                    }
                    cmd.arg(ccompiler);
                }
            };

            set_compiler("C", &c_compiler, &self.cflags);
            set_compiler("CXX", &cxx_compiler, &self.cxxflags);
            set_compiler("ASM", &asm_compiler, &self.asmflags);
        }

        if !self.defined("CMAKE_BUILD_TYPE") {
            cmd.arg(format!("-DCMAKE_BUILD_TYPE={}", profile));
        }

        if self.verbose_make {
            cmd.arg("-DCMAKE_VERBOSE_MAKEFILE:BOOL=ON");
        }

        for (k, v) in c_compiler.env().iter().chain(&self.env) {
            cmd.env(k, v);
        }

        if self.always_configure || !build_dir.join("CMakeCache.txt").exists() {
            cmd.args(&self.configure_args);
            run(cmd.env("CMAKE_PREFIX_PATH", cmake_prefix_path), "cmake");
        } else {
            println!("CMake project was already configured. Skipping configuration step.");
        }

        // And build!
        let mut cmd = self.cmake_build_command(&target);
        cmd.current_dir(&build_dir);

        for (k, v) in c_compiler.env().iter().chain(&self.env) {
            cmd.env(k, v);
        }

        // If the generated project is Makefile based we should carefully transfer corresponding CARGO_MAKEFLAGS
        let mut use_jobserver = false;
        if fs::metadata(build_dir.join("Makefile")).is_ok() {
            match env::var_os("CARGO_MAKEFLAGS") {
                // Only do this on non-windows, non-bsd, and non-macos (unless a named pipe
                // jobserver is available)
                // * On Windows, we could be invoking make instead of
                //   mingw32-make which doesn't work with our jobserver
                // * bsdmake also does not work with our job server
                // * On macOS, CMake blocks propagation of the jobserver's file descriptors to make
                //   However, if the jobserver is based on a named pipe, this will be available to
                //   the build.
                Some(ref makeflags)
                    if !(cfg!(windows)
                        || cfg!(target_os = "openbsd")
                        || cfg!(target_os = "netbsd")
                        || cfg!(target_os = "freebsd")
                        || cfg!(target_os = "dragonfly")
                        || (cfg!(target_os = "macos")
                            && !uses_named_pipe_jobserver(makeflags))) =>
                {
                    use_jobserver = true;
                    cmd.env("MAKEFLAGS", makeflags);
                }
                _ => {}
            }
        }

        cmd.arg("--build").arg(&build_dir);

        if !self.no_build_target {
            let target = self
                .cmake_target
                .clone()
                .unwrap_or_else(|| "install".to_string());
            cmd.arg("--target").arg(target);
        }

        cmd.arg("--config").arg(&profile);

        // --parallel requires CMake 3.12:
        // https://cmake.org/cmake/help/latest/release/3.12.html#command-line
        if version >= Version::new(3, 12) && !use_jobserver {
            if let Ok(s) = env::var("NUM_JOBS") {
                // See https://cmake.org/cmake/help/v3.12/manual/cmake.1.html#build-tool-mode
                cmd.arg("--parallel").arg(s);
            }
        }

        if !&self.build_args.is_empty() {
            cmd.arg("--").args(&self.build_args);
        }

        run(&mut cmd, "cmake");

        println!("cargo:root={}", dst.display());
        dst
    }

    fn cmake_executable(&mut self, target: &str) -> OsString {
        self.getenv_target_os("CMAKE")
            .or_else(|| find_cmake_executable(target))
            .unwrap_or_else(|| OsString::from("cmake"))
    }

    // If we are building for Emscripten, wrap the calls to CMake
    // as "emcmake cmake ..." and "emmake cmake --build ...".
    // https://emscripten.org/docs/compiling/Building-Projects.html

    fn cmake_configure_command(&mut self, target: &str) -> Command {
        if target.contains("emscripten") {
            let emcmake = self
                .getenv_target_os("EMCMAKE")
                .unwrap_or_else(|| OsString::from("emcmake"));
            let mut cmd = Command::new(emcmake);
            cmd.arg(self.cmake_executable(target));
            cmd
        } else {
            Command::new(self.cmake_executable(target))
        }
    }

    fn cmake_build_command(&mut self, target: &str) -> Command {
        if target.contains("emscripten") {
            let emmake = self
                .getenv_target_os("EMMAKE")
                .unwrap_or_else(|| OsString::from("emmake"));
            let mut cmd = Command::new(emmake);
            cmd.arg(self.cmake_executable(target));
            cmd
        } else {
            Command::new(self.cmake_executable(target))
        }
    }

    fn getenv_os(&mut self, v: &str) -> Option<OsString> {
        if let Some(val) = self.env_cache.get(v) {
            return val.clone();
        }
        let r = env::var_os(v);
        println!("{} = {:?}", v, r);
        self.env_cache.insert(v.to_string(), r.clone());
        r
    }

    /// Gets a target-specific environment variable.
    fn getenv_target_os(&mut self, var_base: &str) -> Option<OsString> {
        let host = self.host.clone().unwrap_or_else(|| getenv_unwrap("HOST"));
        let target = self
            .target
            .clone()
            .unwrap_or_else(|| getenv_unwrap("TARGET"));

        let kind = if host == target { "HOST" } else { "TARGET" };
        let target_u = target.replace('-', "_");
        self.getenv_os(&format!("{}_{}", var_base, target))
            .or_else(|| self.getenv_os(&format!("{}_{}", var_base, target_u)))
            .or_else(|| self.getenv_os(&format!("{}_{}", kind, var_base)))
            .or_else(|| self.getenv_os(var_base))
    }

    fn visual_studio_generator(&self, target: &str) -> String {
        use cc::windows_registry::{find_vs_version, VsVers};

        let base = match find_vs_version() {
            Ok(VsVers::Vs18) => "Visual Studio 18 2026",
            Ok(VsVers::Vs17) => "Visual Studio 17 2022",
            Ok(VsVers::Vs16) => "Visual Studio 16 2019",
            Ok(VsVers::Vs15) => "Visual Studio 15 2017",
            Ok(VsVers::Vs14) => "Visual Studio 14 2015",
            // This was deprecated recently (2024-07). Ignore the warning for now.
            #[allow(deprecated)]
            Ok(VsVers::Vs12) => "Visual Studio 12 2013",
            Ok(_) => panic!(
                "Visual studio version detected but this crate \
                 doesn't know how to generate cmake files for it, \
                 can the `cmake` crate be updated?"
            ),
            Err(msg) => panic!("{}", msg),
        };
        if ["i686", "x86_64", "thumbv7a", "aarch64"]
            .iter()
            .any(|t| target.contains(t))
        {
            base.to_string()
        } else {
            panic!("unsupported msvc target: {}", target);
        }
    }

    fn defined(&self, var: &str) -> bool {
        self.defines.iter().any(|(a, _)| a == var)
    }

    // If a cmake project has previously been built (e.g. CMakeCache.txt already
    // exists), then cmake will choke if the source directory for the original
    // project being built has changed. Detect this situation through the
    // `CMAKE_HOME_DIRECTORY` variable that cmake emits and if it doesn't match
    // we blow away the build directory and start from scratch (the recommended
    // solution apparently [1]).
    //
    // [1]: https://cmake.org/pipermail/cmake/2012-August/051545.html
    fn maybe_clear(&self, dir: &Path) {
        // CMake will apparently store canonicalized paths which normally
        // isn't relevant to us but we canonicalize it here to ensure
        // we're both checking the same thing.
        let path = try_canonicalize(&self.path);

        let mut f = match File::open(dir.join("CMakeCache.txt")) {
            Ok(f) => f,
            Err(..) => return,
        };
        let mut u8contents = Vec::new();
        match f.read_to_end(&mut u8contents) {
            Ok(f) => f,
            Err(..) => return,
        };
        let contents = String::from_utf8_lossy(&u8contents);
        drop(f);
        for line in contents.lines() {
            if line.starts_with("CMAKE_HOME_DIRECTORY") {
                let needs_cleanup = match line.split('=').next_back() {
                    Some(cmake_home) => fs::canonicalize(cmake_home)
                        .ok()
                        .map(|cmake_home| cmake_home != path)
                        .unwrap_or(true),
                    None => true,
                };
                if needs_cleanup {
                    println!(
                        "detected home dir change, cleaning out entire build \
                         directory"
                    );
                    fs::remove_dir_all(dir).unwrap();
                }
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
}

impl Version {
    fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    fn parse(s: &str) -> Option<Self> {
        // As of 3.22, the format of the version output is "cmake version <major>.<minor>.<patch>".
        // ```
        // $ cmake --version
        // cmake version 3.22.2
        //
        // CMake suite maintained and supported by Kitware (kitware.com/cmake).
        // ```
        let version = s.lines().next()?.strip_prefix("cmake version ")?;
        let mut digits = version.splitn(3, '.'); // split version string to major minor patch
        let major = digits.next()?.parse::<u32>().ok()?;
        let minor = digits.next()?.parse::<u32>().ok()?;
        // Ignore the patch version because it does not change the API.
        Some(Version::new(major, minor))
    }

    fn from_command(executable: &OsStr) -> Option<Self> {
        let output = Command::new(executable).arg("--version").output().ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = core::str::from_utf8(&output.stdout).ok()?;
        Self::parse(stdout)
    }
}

impl Default for Version {
    fn default() -> Self {
        // If the version parsing fails, we assume that it is the latest known
        // version. This is because the failure of version parsing may be due to
        // the version output being changed.
        Self::new(3, 22)
    }
}

fn run(cmd: &mut Command, program: &str) {
    eprintln!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!(
                "failed to execute command: {}\nis `{}` not installed?",
                e, program
            ));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        if status.code() == Some(127) {
            fail(&format!(
                "command did not execute successfully, got: {}, is `{}` not installed?",
                status, program
            ));
        }
        fail(&format!(
            "command did not execute successfully, got: {}",
            status
        ));
    }
}

fn find_exe(path: &Path) -> PathBuf {
    env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .map(|p| p.join(path))
        .find(|p| fs::metadata(p).is_ok())
        .unwrap_or_else(|| path.to_owned())
}

fn getenv_unwrap(v: &str) -> String {
    match env::var(v) {
        Ok(s) => s,
        Err(..) => fail(&format!("environment variable `{}` not defined", v)),
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

/// Returns whether the given MAKEFLAGS indicate that there is an available
/// jobserver that uses a named pipe (fifo)
fn uses_named_pipe_jobserver(makeflags: &OsStr) -> bool {
    makeflags
        .to_string_lossy()
        // auth option as defined in
        // https://www.gnu.org/software/make/manual/html_node/POSIX-Jobserver.html#POSIX-Jobserver
        .contains("--jobserver-auth=fifo:")
}

/// Attempt to canonicalize; fall back to the original path if unsuccessful, in case `cmake` knows
/// something we don't.
fn try_canonicalize(path: &Path) -> PathBuf {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_owned());
    // On Windows, attempt to remove the verbatim prefix from the canonicalized path.
    // FIXME(ChrisDenton): once MSRV is >=1.79 use `std::path::absolute` instead of canonicalize.
    // That will avoid the need for this hack.
    #[cfg(windows)]
    {
        use std::os::windows::ffi::{OsStrExt, OsStringExt};
        let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
        if wide.starts_with(&[b'\\' as u16, b'\\' as u16, b'?' as u16, b'\\' as u16]) {
            if wide.get(5..7) == Some(&[b':' as u16, b'\\' as u16]) {
                // Convert \\?\C:\ to C:\
                wide.copy_within(4.., 0);
                wide.truncate(wide.len() - 4);
            } else if wide.get(4..8) == Some(&[b'U' as u16, b'N' as u16, b'C' as u16, b'\\' as u16])
            {
                // Convert \\?\UNC\ to \\
                wide.copy_within(8.., 2);
                wide.truncate(wide.len() - (8 - 2));
            }
            return OsString::from_wide(&wide).into();
        }
    }
    path
}

#[cfg(windows)]
fn find_cmake_executable(target: &str) -> Option<OsString> {
    use cc::windows_registry::find_tool;

    // Try to find cmake.exe bundled with MSVC, but only if there isn't another one in path
    let cmake_in_path = env::split_paths(&env::var_os("PATH").unwrap_or(OsString::new()))
        .any(|p| p.join("cmake.exe").exists());
    if cmake_in_path {
        None
    } else {
        find_tool(target, "devenv").and_then(|t| {
            t.path()
                .join("..\\CommonExtensions\\Microsoft\\CMake\\CMake\\bin\\cmake.exe")
                .canonicalize()
                .ok()
                .map(OsString::from)
        })
    }
}

#[cfg(not(windows))]
fn find_cmake_executable(_target: &str) -> Option<OsString> {
    None
}

#[cfg(test)]
mod tests {
    use super::uses_named_pipe_jobserver;
    use super::Version;

    #[test]
    fn test_cmake_version() {
        let text = "cmake version 3.22.2

CMake suite maintained and supported by Kitware (kitware.com/cmake).
";
        let v = Version::parse(text).unwrap();
        assert_eq!(v, Version::new(3, 22));
        assert!(Version::new(3, 22) > Version::new(3, 21));
        assert!(Version::new(3, 22) < Version::new(3, 23));

        let _v = Version::from_command("cmake".as_ref()).unwrap();
    }

    #[test]
    fn test_uses_fifo_jobserver() {
        assert!(uses_named_pipe_jobserver(
            "-j --jobserver-auth=fifo:/foo".as_ref()
        ));
        assert!(!uses_named_pipe_jobserver(
            "-j --jobserver-auth=8:9".as_ref()
        ));
    }
}
