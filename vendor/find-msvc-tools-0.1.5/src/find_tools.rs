// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An internal use crate to looking for windows-specific tools:
//! 1. On Windows host, probe the Windows Registry if needed;
//! 2. On non-Windows host, check specified environment variables.

#![allow(clippy::upper_case_acronyms)]

use std::{
    env,
    ffi::{OsStr, OsString},
    ops::Deref,
    path::PathBuf,
    process::Command,
    sync::Arc,
};

use crate::Tool;

/// The target provided by the user.
#[derive(Copy, Clone, PartialEq, Eq)]
enum TargetArch {
    X86,
    X64,
    Arm,
    Arm64,
    Arm64ec,
}
impl TargetArch {
    /// Parse the `TargetArch` from a str. Returns `None` if the arch is unrecognized.
    fn new(arch: &str) -> Option<Self> {
        // NOTE: Keep up to date with docs in [`find`].
        match arch {
            "x64" | "x86_64" => Some(Self::X64),
            "arm64" | "aarch64" => Some(Self::Arm64),
            "arm64ec" => Some(Self::Arm64ec),
            "x86" | "i686" | "i586" => Some(Self::X86),
            "arm" | "thumbv7a" => Some(Self::Arm),
            _ => None,
        }
    }

    #[cfg(windows)]
    /// Gets the Visual Studio name for the architecture.
    fn as_vs_arch(&self) -> &'static str {
        match self {
            Self::X64 => "x64",
            Self::Arm64 | Self::Arm64ec => "arm64",
            Self::X86 => "x86",
            Self::Arm => "arm",
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Env {
    Owned(OsString),
    Arced(Arc<OsStr>),
}

impl AsRef<OsStr> for Env {
    fn as_ref(&self) -> &OsStr {
        self.deref()
    }
}

impl Deref for Env {
    type Target = OsStr;

    fn deref(&self) -> &Self::Target {
        match self {
            Env::Owned(os_str) => os_str,
            Env::Arced(os_str) => os_str,
        }
    }
}

impl From<Env> for PathBuf {
    fn from(env: Env) -> Self {
        match env {
            Env::Owned(os_str) => PathBuf::from(os_str),
            Env::Arced(os_str) => PathBuf::from(os_str.deref()),
        }
    }
}

pub trait EnvGetter {
    fn get_env(&self, name: &'static str) -> Option<Env>;
}

struct StdEnvGetter;

impl EnvGetter for StdEnvGetter {
    #[allow(clippy::disallowed_methods)]
    fn get_env(&self, name: &'static str) -> Option<Env> {
        env::var_os(name).map(Env::Owned)
    }
}

/// Attempts to find a tool within an MSVC installation using the Windows
/// registry as a point to search from.
///
/// The `arch_or_target` argument is the architecture or the Rust target name
/// that the tool should work for (e.g. compile or link for). The supported
/// architecture names are:
/// - `"x64"` or `"x86_64"`
/// - `"arm64"` or `"aarch64"`
/// - `"arm64ec"`
/// - `"x86"`, `"i586"` or `"i686"`
/// - `"arm"` or `"thumbv7a"`
///
/// The `tool` argument is the tool to find. Supported tools include:
/// - MSVC tools: `cl.exe`, `link.exe`, `lib.exe`, etc.
/// - `MSBuild`: `msbuild.exe`
/// - Visual Studio IDE: `devenv.exe`
/// - Clang/LLVM tools: `clang.exe`, `clang++.exe`, `clang-*.exe`, `llvm-*.exe`, `lld.exe`, etc.
///
/// This function will return `None` if the tool could not be found, or it will
/// return `Some(cmd)` which represents a command that's ready to execute the
/// tool with the appropriate environment variables set.
///
/// To find MSVC tools, this function will first attempt to detect if we are
/// running in the context of a developer command prompt, and then use the tools
/// as found in the current `PATH`. If that fails, it will attempt to locate
/// the newest MSVC toolset in the newest installed version of Visual Studio.
/// To limit the search to a specific version of the MSVC toolset, set the
/// VCToolsVersion environment variable to the desired version (e.g. "14.44.35207").
///
/// Note that this function always returns `None` for non-MSVC targets (if a
/// full target name was specified).
pub fn find(arch_or_target: &str, tool: &str) -> Option<Command> {
    find_tool(arch_or_target, tool).map(|c| c.to_command())
}

/// Similar to the `find` function above, this function will attempt the same
/// operation (finding a MSVC tool in a local install) but instead returns a
/// `Tool` which may be introspected.
pub fn find_tool(arch_or_target: &str, tool: &str) -> Option<Tool> {
    let full_arch = if let Some((full_arch, rest)) = arch_or_target.split_once("-") {
        // The logic is all tailored for MSVC, if the target is not that then
        // bail out early.
        if !rest.contains("msvc") {
            return None;
        }
        full_arch
    } else {
        arch_or_target
    };
    find_tool_with_env(full_arch, tool, &StdEnvGetter)
}

pub fn find_tool_with_env(full_arch: &str, tool: &str, env_getter: &dyn EnvGetter) -> Option<Tool> {
    // We only need the arch.
    let target = TargetArch::new(full_arch)?;

    // Looks like msbuild isn't located in the same location as other tools like
    // cl.exe and lib.exe.
    if tool.contains("msbuild") {
        return impl_::find_msbuild(target, env_getter);
    }

    // Looks like devenv isn't located in the same location as other tools like
    // cl.exe and lib.exe.
    if tool.contains("devenv") {
        return impl_::find_devenv(target, env_getter);
    }

    // Clang/LLVM isn't located in the same location as other tools like
    // cl.exe and lib.exe.
    if ["clang", "lldb", "llvm", "ld", "lld"]
        .iter()
        .any(|&t| tool.contains(t))
    {
        return impl_::find_llvm_tool(tool, target, env_getter);
    }

    // Ok, if we're here, now comes the fun part of the probing. Default shells
    // or shells like MSYS aren't really configured to execute `cl.exe` and the
    // various compiler tools shipped as part of Visual Studio. Here we try to
    // first find the relevant tool, then we also have to be sure to fill in
    // environment variables like `LIB`, `INCLUDE`, and `PATH` to ensure that
    // the tool is actually usable.

    impl_::find_msvc_environment(tool, target, env_getter)
        .or_else(|| impl_::find_msvc_15plus(tool, target, env_getter))
        .or_else(|| impl_::find_msvc_14(tool, target, env_getter))
}

/// A version of Visual Studio
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum VsVers {
    /// Visual Studio 12 (2013)
    #[deprecated(
        note = "Visual Studio 12 is no longer supported. cc will never return this value."
    )]
    Vs12,
    /// Visual Studio 14 (2015)
    Vs14,
    /// Visual Studio 15 (2017)
    Vs15,
    /// Visual Studio 16 (2019)
    Vs16,
    /// Visual Studio 17 (2022)
    Vs17,
    /// Visual Studio 18 (2026)
    Vs18,
}

/// Find the most recent installed version of Visual Studio
///
/// This is used by the cmake crate to figure out the correct
/// generator.
#[allow(clippy::disallowed_methods)]
pub fn find_vs_version() -> Result<VsVers, String> {
    fn has_msbuild_version(version: &str) -> bool {
        impl_::has_msbuild_version(version, &StdEnvGetter)
    }

    match std::env::var("VisualStudioVersion") {
        Ok(version) => match &version[..] {
            "18.0" => Ok(VsVers::Vs18),
            "17.0" => Ok(VsVers::Vs17),
            "16.0" => Ok(VsVers::Vs16),
            "15.0" => Ok(VsVers::Vs15),
            "14.0" => Ok(VsVers::Vs14),
            vers => Err(format!(
                "\n\n\
                 unsupported or unknown VisualStudio version: {vers}\n\
                 if another version is installed consider running \
                 the appropriate vcvars script before building this \
                 crate\n\
                 "
            )),
        },
        _ => {
            // Check for the presence of a specific registry key
            // that indicates visual studio is installed.
            if has_msbuild_version("18.0") {
                Ok(VsVers::Vs18)
            } else if has_msbuild_version("17.0") {
                Ok(VsVers::Vs17)
            } else if has_msbuild_version("16.0") {
                Ok(VsVers::Vs16)
            } else if has_msbuild_version("15.0") {
                Ok(VsVers::Vs15)
            } else if has_msbuild_version("14.0") {
                Ok(VsVers::Vs14)
            } else {
                Err("\n\n\
                     couldn't determine visual studio generator\n\
                     if VisualStudio is installed, however, consider \
                     running the appropriate vcvars script before building \
                     this crate\n\
                     "
                .to_string())
            }
        }
    }
}

/// To find the Universal CRT we look in a specific registry key for where
/// all the Universal CRTs are located and then sort them asciibetically to
/// find the newest version. While this sort of sorting isn't ideal,  it is
/// what vcvars does so that's good enough for us.
///
/// Returns a pair of (root, version) for the ucrt dir if found
pub fn get_ucrt_dir() -> Option<(PathBuf, String)> {
    impl_::get_ucrt_dir()
}

/// Windows Implementation.
#[cfg(windows)]
mod impl_ {
    use crate::com;
    use crate::registry::{RegistryKey, LOCAL_MACHINE};
    use crate::setup_config::SetupConfiguration;
    use crate::vs_instances::{VsInstances, VswhereInstance};
    use crate::windows_sys::{
        GetMachineTypeAttributes, GetProcAddress, LoadLibraryA, UserEnabled, HMODULE,
        IMAGE_FILE_MACHINE_AMD64, MACHINE_ATTRIBUTES, S_OK,
    };
    use std::convert::TryFrom;
    use std::env;
    use std::ffi::OsString;
    use std::fs::File;
    use std::io::Read;
    use std::iter;
    use std::mem;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::str::FromStr;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Once;

    use super::{EnvGetter, TargetArch};
    use crate::Tool;

    struct MsvcTool {
        tool: PathBuf,
        libs: Vec<PathBuf>,
        path: Vec<PathBuf>,
        include: Vec<PathBuf>,
    }

    #[derive(Default)]
    struct SdkInfo {
        libs: Vec<PathBuf>,
        path: Vec<PathBuf>,
        include: Vec<PathBuf>,
    }

    struct LibraryHandle(HMODULE);

    impl LibraryHandle {
        fn new(name: &[u8]) -> Option<Self> {
            let handle = unsafe { LoadLibraryA(name.as_ptr() as _) };
            (!handle.is_null()).then_some(Self(handle))
        }

        /// Get a function pointer to a function in the library.
        /// # SAFETY
        ///
        /// The caller must ensure that the function signature matches the actual function.
        /// The easiest way to do this is to add an entry to `windows_sys_no_link.list` and use the
        /// generated function for `func_signature`.
        ///
        /// The function returned cannot be used after the handle is dropped.
        unsafe fn get_proc_address<F>(&self, name: &[u8]) -> Option<F> {
            let symbol = GetProcAddress(self.0, name.as_ptr() as _);
            symbol.map(|symbol| mem::transmute_copy(&symbol))
        }
    }

    type GetMachineTypeAttributesFuncType =
        unsafe extern "system" fn(u16, *mut MACHINE_ATTRIBUTES) -> i32;
    const _: () = {
        // Ensure that our hand-written signature matches the actual function signature.
        // We can't use `GetMachineTypeAttributes` outside of a const scope otherwise we'll end up statically linking to
        // it, which will fail to load on older versions of Windows.
        let _: GetMachineTypeAttributesFuncType = GetMachineTypeAttributes;
    };

    fn is_amd64_emulation_supported_inner() -> Option<bool> {
        // GetMachineTypeAttributes is only available on Win11 22000+, so dynamically load it.
        let kernel32 = LibraryHandle::new(b"kernel32.dll\0")?;
        // SAFETY: GetMachineTypeAttributesFuncType is checked to match the real function signature.
        let get_machine_type_attributes = unsafe {
            kernel32
                .get_proc_address::<GetMachineTypeAttributesFuncType>(b"GetMachineTypeAttributes\0")
        }?;
        let mut attributes = Default::default();
        if unsafe { get_machine_type_attributes(IMAGE_FILE_MACHINE_AMD64, &mut attributes) } == S_OK
        {
            Some((attributes & UserEnabled) != 0)
        } else {
            Some(false)
        }
    }

    fn is_amd64_emulation_supported() -> bool {
        // TODO: Replace with a OnceLock once MSRV is 1.70.
        static LOAD_VALUE: Once = Once::new();
        static IS_SUPPORTED: AtomicBool = AtomicBool::new(false);

        // Using Relaxed ordering since the Once is providing synchronization.
        LOAD_VALUE.call_once(|| {
            IS_SUPPORTED.store(
                is_amd64_emulation_supported_inner().unwrap_or(false),
                Ordering::Relaxed,
            );
        });
        IS_SUPPORTED.load(Ordering::Relaxed)
    }

    impl MsvcTool {
        fn new(tool: PathBuf) -> MsvcTool {
            MsvcTool {
                tool,
                libs: Vec::new(),
                path: Vec::new(),
                include: Vec::new(),
            }
        }

        fn add_sdk(&mut self, sdk_info: SdkInfo) {
            self.libs.extend(sdk_info.libs);
            self.path.extend(sdk_info.path);
            self.include.extend(sdk_info.include);
        }

        fn into_tool(self, env_getter: &dyn EnvGetter) -> Tool {
            let MsvcTool {
                tool,
                libs,
                path,
                include,
            } = self;
            let mut tool = Tool {
                tool,
                is_clang_cl: false,
                env: Vec::new(),
            };
            add_env(&mut tool, "LIB", libs, env_getter);
            add_env(&mut tool, "PATH", path, env_getter);
            add_env(&mut tool, "INCLUDE", include, env_getter);
            tool
        }
    }

    impl SdkInfo {
        fn find_tool(&self, tool: &str) -> Option<PathBuf> {
            self.path.iter().map(|p| p.join(tool)).find(|p| p.exists())
        }
    }

    /// Checks to see if the target's arch matches the VS environment. Returns `None` if the
    /// environment is unknown.
    fn is_vscmd_target(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<bool> {
        is_vscmd_target_env(target, env_getter).or_else(|| is_vscmd_target_cl(target, env_getter))
    }

    /// Checks to see if the `VSCMD_ARG_TGT_ARCH` environment variable matches the
    /// given target's arch. Returns `None` if the variable does not exist.
    fn is_vscmd_target_env(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<bool> {
        let vscmd_arch = env_getter.get_env("VSCMD_ARG_TGT_ARCH")?;
        Some(target.as_vs_arch() == vscmd_arch.as_ref())
    }

    /// Checks if the cl.exe target matches the given target's arch. Returns `None` if anything
    /// fails.
    fn is_vscmd_target_cl(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<bool> {
        let cmd_target = vscmd_target_cl(env_getter)?;
        Some(target.as_vs_arch() == cmd_target)
    }

    /// Detect the target architecture of `cl.exe` in the current path, and return `None` if this
    /// fails for any reason.
    fn vscmd_target_cl(env_getter: &dyn EnvGetter) -> Option<&'static str> {
        let cl_exe = env_getter.get_env("PATH").and_then(|path| {
            env::split_paths(&path)
                .map(|p| p.join("cl.exe"))
                .find(|p| p.exists())
        })?;
        let mut cl = Command::new(cl_exe);
        cl.stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null());

        let out = cl.output().ok()?;
        let cl_arch = out
            .stderr
            .split(|&b| b == b'\n' || b == b'\r')
            .next()?
            .rsplit(|&b| b == b' ')
            .next()?;

        match cl_arch {
            b"x64" => Some("x64"),
            b"x86" => Some("x86"),
            b"ARM64" => Some("arm64"),
            b"ARM" => Some("arm"),
            _ => None,
        }
    }

    /// Attempt to find the tool using environment variables set by vcvars.
    pub(super) fn find_msvc_environment(
        tool: &str,
        target: TargetArch,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        // Early return if the environment isn't one that is known to have compiler toolsets in PATH
        // `VCINSTALLDIR` is set from vcvarsall.bat (developer command prompt)
        // `VSTEL_MSBuildProjectFullPath` is set by msbuild when invoking custom build steps
        // NOTE: `VisualStudioDir` used to be used but this isn't set when invoking msbuild from the commandline
        if env_getter.get_env("VCINSTALLDIR").is_none()
            && env_getter.get_env("VSTEL_MSBuildProjectFullPath").is_none()
        {
            return None;
        }

        // If the vscmd target differs from the requested target then
        // attempt to get the tool using the VS install directory.
        if is_vscmd_target(target, env_getter) == Some(false) {
            // We will only get here with versions 15+.
            let vs_install_dir: PathBuf = env_getter.get_env("VSINSTALLDIR")?.into();
            tool_from_vs15plus_instance(tool, target, &vs_install_dir, env_getter)
        } else {
            // Fallback to simply using the current environment.
            env_getter
                .get_env("PATH")
                .and_then(|path| {
                    env::split_paths(&path)
                        .map(|p| p.join(tool))
                        .find(|p| p.exists())
                })
                .map(|path| Tool {
                    tool: path,
                    is_clang_cl: false,
                    env: Vec::new(),
                })
        }
    }

    fn find_msbuild_vs18(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<Tool> {
        find_tool_in_vs16plus_path(r"MSBuild\Current\Bin\MSBuild.exe", target, "18", env_getter)
    }

    fn find_msbuild_vs17(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<Tool> {
        find_tool_in_vs16plus_path(r"MSBuild\Current\Bin\MSBuild.exe", target, "17", env_getter)
    }

    #[allow(bare_trait_objects)]
    fn vs16plus_instances(
        target: TargetArch,
        version: &'static str,
        env_getter: &dyn EnvGetter,
    ) -> Box<Iterator<Item = PathBuf>> {
        let instances = if let Some(instances) = vs15plus_instances(target, env_getter) {
            instances
        } else {
            return Box::new(iter::empty());
        };
        Box::new(instances.into_iter().filter_map(move |instance| {
            let installation_name = instance.installation_name()?;
            if installation_name.starts_with(&format!("VisualStudio/{}.", version))
                || installation_name.starts_with(&format!("VisualStudioPreview/{}.", version))
            {
                Some(instance.installation_path()?)
            } else {
                None
            }
        }))
    }

    fn find_tool_in_vs16plus_path(
        tool: &str,
        target: TargetArch,
        version: &'static str,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        vs16plus_instances(target, version, env_getter)
            .filter_map(|path| {
                let path = path.join(tool);
                if !path.is_file() {
                    return None;
                }
                let mut tool = Tool {
                    tool: path,
                    is_clang_cl: false,
                    env: Vec::new(),
                };
                if target == TargetArch::X64 {
                    tool.env.push(("Platform".into(), "X64".into()));
                }
                if matches!(target, TargetArch::Arm64 | TargetArch::Arm64ec) {
                    tool.env.push(("Platform".into(), "ARM64".into()));
                }
                Some(tool)
            })
            .next()
    }

    fn find_msbuild_vs16(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<Tool> {
        find_tool_in_vs16plus_path(r"MSBuild\Current\Bin\MSBuild.exe", target, "16", env_getter)
    }

    pub(super) fn find_llvm_tool(
        tool: &str,
        target: TargetArch,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        find_llvm_tool_vs17plus(tool, target, env_getter, "18")
            .or_else(|| find_llvm_tool_vs17plus(tool, target, env_getter, "17"))
    }

    fn find_llvm_tool_vs17plus(
        tool: &str,
        target: TargetArch,
        env_getter: &dyn EnvGetter,
        version: &'static str,
    ) -> Option<Tool> {
        vs16plus_instances(target, version, env_getter)
            .filter_map(|mut base_path| {
                base_path.push(r"VC\Tools\LLVM");
                let host_folder = match host_arch() {
                    // The default LLVM bin folder is x86, and there's separate subfolders
                    // for the x64 and ARM64 host tools.
                    X86 => "",
                    X86_64 => "x64",
                    AARCH64 => "ARM64",
                    _ => return None,
                };
                if host_folder != "" {
                    // E.g. C:\...\VC\Tools\LLVM\x64
                    base_path.push(host_folder);
                }
                // E.g. C:\...\VC\Tools\LLVM\x64\bin\clang.exe
                base_path.push("bin");
                base_path.push(tool);
                let is_clang_cl = tool.contains("clang-cl");
                base_path.is_file().then(|| Tool {
                    tool: base_path,
                    is_clang_cl,
                    env: Vec::new(),
                })
            })
            .next()
    }

    // In MSVC 15 (2017) MS once again changed the scheme for locating
    // the tooling.  Now we must go through some COM interfaces, which
    // is super fun for Rust.
    //
    // Note that much of this logic can be found [online] wrt paths, COM, etc.
    //
    // [online]: https://blogs.msdn.microsoft.com/vcblog/2017/03/06/finding-the-visual-c-compiler-tools-in-visual-studio-2017/
    //
    // Returns MSVC 15+ instances (15, 16 right now), the order should be consider undefined.
    //
    // However, on ARM64 this method doesn't work because VS Installer fails to register COM component on ARM64.
    // Hence, as the last resort we try to use vswhere.exe to list available instances.
    fn vs15plus_instances(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<VsInstances> {
        vs15plus_instances_using_com()
            .or_else(|| vs15plus_instances_using_vswhere(target, env_getter))
    }

    fn vs15plus_instances_using_com() -> Option<VsInstances> {
        com::initialize().ok()?;

        let config = SetupConfiguration::new().ok()?;
        let enum_setup_instances = config.enum_all_instances().ok()?;

        Some(VsInstances::ComBased(enum_setup_instances))
    }

    fn vs15plus_instances_using_vswhere(
        target: TargetArch,
        env_getter: &dyn EnvGetter,
    ) -> Option<VsInstances> {
        let program_files_path = env_getter
            .get_env("ProgramFiles(x86)")
            .or_else(|| env_getter.get_env("ProgramFiles"))?;

        let program_files_path = Path::new(program_files_path.as_ref());

        let vswhere_path =
            program_files_path.join(r"Microsoft Visual Studio\Installer\vswhere.exe");

        if !vswhere_path.exists() {
            return None;
        }

        let tools_arch = match target {
            TargetArch::X86 | TargetArch::X64 => Some("x86.x64"),
            TargetArch::Arm => Some("ARM"),
            TargetArch::Arm64 | TargetArch::Arm64ec => Some("ARM64"),
        };

        let vswhere_output = Command::new(vswhere_path)
            .args([
                "-latest",
                "-products",
                "*",
                "-requires",
                &format!("Microsoft.VisualStudio.Component.VC.Tools.{}", tools_arch?),
                "-format",
                "text",
                "-nologo",
            ])
            .stderr(std::process::Stdio::inherit())
            .output()
            .ok()?;

        let vs_instances =
            VsInstances::VswhereBased(VswhereInstance::try_from(&vswhere_output.stdout).ok()?);

        Some(vs_instances)
    }

    // Inspired from official microsoft/vswhere ParseVersionString
    // i.e. at most four u16 numbers separated by '.'
    fn parse_version(version: &str) -> Option<[u16; 4]> {
        let mut iter = version.split('.').map(u16::from_str).fuse();
        let mut get_next_number = move || match iter.next() {
            Some(Ok(version_part)) => Some(version_part),
            Some(Err(_)) => None,
            None => Some(0),
        };
        Some([
            get_next_number()?,
            get_next_number()?,
            get_next_number()?,
            get_next_number()?,
        ])
    }

    pub(super) fn find_msvc_15plus(
        tool: &str,
        target: TargetArch,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        let iter = vs15plus_instances(target, env_getter)?;
        iter.into_iter()
            .filter_map(|instance| {
                let version = parse_version(&instance.installation_version()?)?;
                let instance_path = instance.installation_path()?;
                let tool = tool_from_vs15plus_instance(tool, target, &instance_path, env_getter)?;
                Some((version, tool))
            })
            .max_by(|(a_version, _), (b_version, _)| a_version.cmp(b_version))
            .map(|(_version, tool)| tool)
    }

    // While the paths to Visual Studio 2017's devenv and MSBuild could
    // potentially be retrieved from the registry, finding them via
    // SetupConfiguration has shown to be [more reliable], and is preferred
    // according to Microsoft. To help head off potential regressions though,
    // we keep the registry method as a fallback option.
    //
    // [more reliable]: https://github.com/rust-lang/cc-rs/pull/331
    fn find_tool_in_vs15_path(
        tool: &str,
        target: TargetArch,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        let mut path = match vs15plus_instances(target, env_getter) {
            Some(instances) => instances
                .into_iter()
                .filter_map(|instance| instance.installation_path())
                .map(|path| path.join(tool))
                .find(|path| path.is_file()),
            None => None,
        };

        if path.is_none() {
            let key = r"SOFTWARE\WOW6432Node\Microsoft\VisualStudio\SxS\VS7";
            path = LOCAL_MACHINE
                .open(key.as_ref())
                .ok()
                .and_then(|key| key.query_str("15.0").ok())
                .map(|path| PathBuf::from(path).join(tool))
                .and_then(|path| if path.is_file() { Some(path) } else { None });
        }

        path.map(|path| {
            let mut tool = Tool {
                tool: path,
                is_clang_cl: false,
                env: Vec::new(),
            };
            if target == TargetArch::X64 {
                tool.env.push(("Platform".into(), "X64".into()));
            } else if matches!(target, TargetArch::Arm64 | TargetArch::Arm64ec) {
                tool.env.push(("Platform".into(), "ARM64".into()));
            }
            tool
        })
    }

    fn tool_from_vs15plus_instance(
        tool: &str,
        target: TargetArch,
        instance_path: &Path,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        let (root_path, bin_path, host_dylib_path, lib_path, alt_lib_path, include_path) =
            vs15plus_vc_paths(target, instance_path, env_getter)?;
        let sdk_info = get_sdks(target, env_getter)?;
        let mut tool_path = bin_path.join(tool);
        if !tool_path.exists() {
            tool_path = sdk_info.find_tool(tool)?;
        };

        let mut tool = MsvcTool::new(tool_path);
        tool.path.push(bin_path.clone());
        tool.path.push(host_dylib_path);
        if let Some(alt_lib_path) = alt_lib_path {
            tool.libs.push(alt_lib_path);
        }
        tool.libs.push(lib_path);
        tool.include.push(include_path);

        if let Some((atl_lib_path, atl_include_path)) = atl_paths(target, &root_path) {
            tool.libs.push(atl_lib_path);
            tool.include.push(atl_include_path);
        }

        tool.add_sdk(sdk_info);

        Some(tool.into_tool(env_getter))
    }

    fn vs15plus_vc_paths(
        target_arch: TargetArch,
        instance_path: &Path,
        env_getter: &dyn EnvGetter,
    ) -> Option<(PathBuf, PathBuf, PathBuf, PathBuf, Option<PathBuf>, PathBuf)> {
        let version = vs15plus_vc_read_version(instance_path, env_getter)?;

        let hosts = match host_arch() {
            X86 => &["X86"],
            X86_64 => &["X64"],
            // Starting with VS 17.4, there is a natively hosted compiler on ARM64:
            // https://devblogs.microsoft.com/visualstudio/arm64-visual-studio-is-officially-here/
            // On older versions of VS, we use x64 if running under emulation is supported,
            // otherwise use x86.
            AARCH64 => {
                if is_amd64_emulation_supported() {
                    &["ARM64", "X64", "X86"][..]
                } else {
                    &["ARM64", "X86"]
                }
            }
            _ => return None,
        };
        let target_dir = target_arch.as_vs_arch();
        // The directory layout here is MSVC/bin/Host$host/$target/
        let path = instance_path.join(r"VC\Tools\MSVC").join(version);
        // We use the first available host architecture that can build for the target
        let (host_path, host) = hosts.iter().find_map(|&x| {
            let candidate = path.join("bin").join(format!("Host{}", x));
            if candidate.join(target_dir).exists() {
                Some((candidate, x))
            } else {
                None
            }
        })?;
        // This is the path to the toolchain for a particular target, running
        // on a given host
        let bin_path = host_path.join(target_dir);
        // But! we also need PATH to contain the target directory for the host
        // architecture, because it contains dlls like mspdb140.dll compiled for
        // the host architecture.
        let host_dylib_path = host_path.join(host.to_lowercase());
        let lib_fragment = if use_spectre_mitigated_libs(env_getter) {
            r"lib\spectre"
        } else {
            "lib"
        };
        let lib_path = path.join(lib_fragment).join(target_dir);
        let alt_lib_path =
            (target_arch == TargetArch::Arm64ec).then(|| path.join(lib_fragment).join("arm64ec"));
        let include_path = path.join("include");
        Some((
            path,
            bin_path,
            host_dylib_path,
            lib_path,
            alt_lib_path,
            include_path,
        ))
    }

    fn vs15plus_vc_read_version(dir: &Path, env_getter: &dyn EnvGetter) -> Option<String> {
        if let Some(version) = env_getter.get_env("VCToolsVersion") {
            // Restrict the search to a specific msvc version; if it doesn't exist then
            // our caller will fail to find the tool for this instance and move on.
            return version.to_str().map(ToString::to_string);
        }

        // Try to open the default version file.
        let mut version_path: PathBuf =
            dir.join(r"VC\Auxiliary\Build\Microsoft.VCToolsVersion.default.txt");
        let mut version_file = if let Ok(f) = File::open(&version_path) {
            f
        } else {
            // If the default doesn't exist, search for other version files.
            // These are in the form Microsoft.VCToolsVersion.v143.default.txt
            // where `143` is any three decimal digit version number.
            // This sorts versions by lexical order and selects the highest version.
            let mut version_file = String::new();
            version_path.pop();
            for file in version_path.read_dir().ok()? {
                let name = file.ok()?.file_name();
                let name = name.to_str()?;
                if name.starts_with("Microsoft.VCToolsVersion.v")
                    && name.ends_with(".default.txt")
                    && name > &version_file
                {
                    version_file.replace_range(.., name);
                }
            }
            if version_file.is_empty() {
                // If all else fails, manually search for bin directories.
                let tools_dir: PathBuf = dir.join(r"VC\Tools\MSVC");
                return tools_dir
                    .read_dir()
                    .ok()?
                    .filter_map(|file| {
                        let file = file.ok()?;
                        let name = file.file_name().into_string().ok()?;

                        file.path().join("bin").exists().then(|| {
                            let version = parse_version(&name);
                            (name, version)
                        })
                    })
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(version, _)| version);
            }
            version_path.push(version_file);
            File::open(version_path).ok()?
        };

        // Get the version string from the file we found.
        let mut version = String::new();
        version_file.read_to_string(&mut version).ok()?;
        version.truncate(version.trim_end().len());
        Some(version)
    }

    fn use_spectre_mitigated_libs(env_getter: &dyn EnvGetter) -> bool {
        env_getter
            .get_env("VSCMD_ARG_VCVARS_SPECTRE")
            .map(|env| env.as_ref() == "spectre")
            .unwrap_or_default()
    }

    fn atl_paths(target: TargetArch, path: &Path) -> Option<(PathBuf, PathBuf)> {
        let atl_path = path.join("atlmfc");
        let sub = target.as_vs_arch();
        if atl_path.exists() {
            Some((atl_path.join("lib").join(sub), atl_path.join("include")))
        } else {
            None
        }
    }

    // For MSVC 14 we need to find the Universal CRT as well as either
    // the Windows 10 SDK or Windows 8.1 SDK.
    pub(super) fn find_msvc_14(
        tool: &str,
        target: TargetArch,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        if env_getter.get_env("VCToolsVersion").is_some() {
            // VCToolsVersion is not set/supported for MSVC 14
            return None;
        }

        let vcdir = get_vc_dir("14.0")?;
        let sdk_info = get_sdks(target, env_getter)?;
        let mut tool = get_tool(tool, &vcdir, target, &sdk_info)?;
        tool.add_sdk(sdk_info);
        Some(tool.into_tool(env_getter))
    }

    fn get_sdks(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<SdkInfo> {
        let sub = target.as_vs_arch();
        let (ucrt, ucrt_version) = get_ucrt_dir()?;

        let host = match host_arch() {
            X86 => "x86",
            X86_64 => "x64",
            AARCH64 => "arm64",
            _ => return None,
        };

        let mut info = SdkInfo::default();

        info.path
            .push(ucrt.join("bin").join(&ucrt_version).join(host));

        let ucrt_include = ucrt.join("include").join(&ucrt_version);
        info.include.push(ucrt_include.join("ucrt"));

        let ucrt_lib = ucrt.join("lib").join(&ucrt_version);
        info.libs.push(ucrt_lib.join("ucrt").join(sub));

        if let Some((sdk, version)) = get_sdk10_dir(env_getter) {
            info.path.push(sdk.join("bin").join(host));
            let sdk_lib = sdk.join("lib").join(&version);
            info.libs.push(sdk_lib.join("um").join(sub));
            let sdk_include = sdk.join("include").join(&version);
            info.include.push(sdk_include.join("um"));
            info.include.push(sdk_include.join("cppwinrt"));
            info.include.push(sdk_include.join("winrt"));
            info.include.push(sdk_include.join("shared"));
        } else if let Some(sdk) = get_sdk81_dir() {
            info.path.push(sdk.join("bin").join(host));
            let sdk_lib = sdk.join("lib").join("winv6.3");
            info.libs.push(sdk_lib.join("um").join(sub));
            let sdk_include = sdk.join("include");
            info.include.push(sdk_include.join("um"));
            info.include.push(sdk_include.join("winrt"));
            info.include.push(sdk_include.join("shared"));
        }

        Some(info)
    }

    fn add_env(
        tool: &mut Tool,
        env: &'static str,
        paths: Vec<PathBuf>,
        env_getter: &dyn EnvGetter,
    ) {
        let prev = env_getter.get_env(env);
        let prev = prev.as_ref().map(AsRef::as_ref).unwrap_or_default();
        let prev = env::split_paths(&prev);
        let new = paths.into_iter().chain(prev);
        tool.env
            .push((env.to_string().into(), env::join_paths(new).unwrap()));
    }

    // Given a possible MSVC installation directory, we look for the linker and
    // then add the MSVC library path.
    fn get_tool(
        tool: &str,
        path: &Path,
        target: TargetArch,
        sdk_info: &SdkInfo,
    ) -> Option<MsvcTool> {
        bin_subdir(target)
            .into_iter()
            .map(|(sub, host)| {
                (
                    path.join("bin").join(sub).join(tool),
                    Some(path.join("bin").join(host)),
                )
            })
            .filter(|(path, _)| path.is_file())
            .chain(iter::once_with(|| Some((sdk_info.find_tool(tool)?, None))).flatten())
            .map(|(tool_path, host)| {
                let mut tool = MsvcTool::new(tool_path);
                tool.path.extend(host);
                let sub = vc_lib_subdir(target);
                tool.libs.push(path.join("lib").join(sub));
                tool.include.push(path.join("include"));
                let atlmfc_path = path.join("atlmfc");
                if atlmfc_path.exists() {
                    tool.libs.push(atlmfc_path.join("lib").join(sub));
                    tool.include.push(atlmfc_path.join("include"));
                }
                tool
            })
            .next()
    }

    // To find MSVC we look in a specific registry key for the version we are
    // trying to find.
    fn get_vc_dir(ver: &str) -> Option<PathBuf> {
        let key = r"SOFTWARE\Microsoft\VisualStudio\SxS\VC7";
        let key = LOCAL_MACHINE.open(key.as_ref()).ok()?;
        let path = key.query_str(ver).ok()?;
        Some(path.into())
    }

    // To find the Universal CRT we look in a specific registry key for where
    // all the Universal CRTs are located and then sort them asciibetically to
    // find the newest version. While this sort of sorting isn't ideal,  it is
    // what vcvars does so that's good enough for us.
    //
    // Returns a pair of (root, version) for the ucrt dir if found
    pub(super) fn get_ucrt_dir() -> Option<(PathBuf, String)> {
        let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
        let key = LOCAL_MACHINE.open(key.as_ref()).ok()?;
        let root = key.query_str("KitsRoot10").ok()?;
        let readdir = Path::new(&root).join("lib").read_dir().ok()?;
        let max_libdir = readdir
            .filter_map(|dir| dir.ok())
            .map(|dir| dir.path())
            .filter(|dir| {
                dir.components()
                    .last()
                    .and_then(|c| c.as_os_str().to_str())
                    .map(|c| c.starts_with("10.") && dir.join("ucrt").is_dir())
                    .unwrap_or(false)
            })
            .max()?;
        let version = max_libdir.components().last().unwrap();
        let version = version.as_os_str().to_str().unwrap().to_string();
        Some((root.into(), version))
    }

    // Vcvars finds the correct version of the Windows 10 SDK by looking
    // for the include `um\Windows.h` because sometimes a given version will
    // only have UCRT bits without the rest of the SDK. Since we only care about
    // libraries and not includes, we instead look for `um\x64\kernel32.lib`.
    // Since the 32-bit and 64-bit libraries are always installed together we
    // only need to bother checking x64, making this code a tiny bit simpler.
    // Like we do for the Universal CRT, we sort the possibilities
    // asciibetically to find the newest one as that is what vcvars does.
    // Before doing that, we check the "WindowsSdkDir" and "WindowsSDKVersion"
    // environment variables set by vcvars to use the environment sdk version
    // if one is already configured.
    fn get_sdk10_dir(env_getter: &dyn EnvGetter) -> Option<(PathBuf, String)> {
        if let (Some(root), Some(version)) = (
            env_getter.get_env("WindowsSdkDir"),
            env_getter
                .get_env("WindowsSDKVersion")
                .as_ref()
                .and_then(|version| version.as_ref().to_str()),
        ) {
            return Some((
                PathBuf::from(root),
                version.trim_end_matches('\\').to_string(),
            ));
        }

        let key = r"SOFTWARE\Microsoft\Microsoft SDKs\Windows\v10.0";
        let key = LOCAL_MACHINE.open(key.as_ref()).ok()?;
        let root = key.query_str("InstallationFolder").ok()?;
        let readdir = Path::new(&root).join("lib").read_dir().ok()?;
        let mut dirs = readdir
            .filter_map(|dir| dir.ok())
            .map(|dir| dir.path())
            .collect::<Vec<_>>();
        dirs.sort();
        let dir = dirs
            .into_iter()
            .rev()
            .find(|dir| dir.join("um").join("x64").join("kernel32.lib").is_file())?;
        let version = dir.components().last().unwrap();
        let version = version.as_os_str().to_str().unwrap().to_string();
        Some((root.into(), version))
    }

    // Interestingly there are several subdirectories, `win7` `win8` and
    // `winv6.3`. Vcvars seems to only care about `winv6.3` though, so the same
    // applies to us. Note that if we were targeting kernel mode drivers
    // instead of user mode applications, we would care.
    fn get_sdk81_dir() -> Option<PathBuf> {
        let key = r"SOFTWARE\Microsoft\Microsoft SDKs\Windows\v8.1";
        let key = LOCAL_MACHINE.open(key.as_ref()).ok()?;
        let root = key.query_str("InstallationFolder").ok()?;
        Some(root.into())
    }

    const PROCESSOR_ARCHITECTURE_INTEL: u16 = 0;
    const PROCESSOR_ARCHITECTURE_AMD64: u16 = 9;
    const PROCESSOR_ARCHITECTURE_ARM64: u16 = 12;
    const X86: u16 = PROCESSOR_ARCHITECTURE_INTEL;
    const X86_64: u16 = PROCESSOR_ARCHITECTURE_AMD64;
    const AARCH64: u16 = PROCESSOR_ARCHITECTURE_ARM64;

    // When choosing the tool to use, we have to choose the one which matches
    // the target architecture. Otherwise we end up in situations where someone
    // on 32-bit Windows is trying to cross compile to 64-bit and it tries to
    // invoke the native 64-bit compiler which won't work.
    //
    // For the return value of this function, the first member of the tuple is
    // the folder of the tool we will be invoking, while the second member is
    // the folder of the host toolchain for that tool which is essential when
    // using a cross linker. We return a Vec since on x64 there are often two
    // linkers that can target the architecture we desire. The 64-bit host
    // linker is preferred, and hence first, due to 64-bit allowing it more
    // address space to work with and potentially being faster.
    fn bin_subdir(target: TargetArch) -> Vec<(&'static str, &'static str)> {
        match (target, host_arch()) {
            (TargetArch::X86, X86) => vec![("", "")],
            (TargetArch::X86, X86_64) => vec![("amd64_x86", "amd64"), ("", "")],
            (TargetArch::X64, X86) => vec![("x86_amd64", "")],
            (TargetArch::X64, X86_64) => vec![("amd64", "amd64"), ("x86_amd64", "")],
            (TargetArch::Arm, X86) => vec![("x86_arm", "")],
            (TargetArch::Arm, X86_64) => vec![("amd64_arm", "amd64"), ("x86_arm", "")],
            _ => vec![],
        }
    }

    // MSVC's x86 libraries are not in a subfolder
    fn vc_lib_subdir(target: TargetArch) -> &'static str {
        match target {
            TargetArch::X86 => "",
            TargetArch::X64 => "amd64",
            TargetArch::Arm => "arm",
            TargetArch::Arm64 | TargetArch::Arm64ec => "arm64",
        }
    }

    #[allow(bad_style)]
    fn host_arch() -> u16 {
        type DWORD = u32;
        type WORD = u16;
        type LPVOID = *mut u8;
        type DWORD_PTR = usize;

        #[repr(C)]
        struct SYSTEM_INFO {
            wProcessorArchitecture: WORD,
            _wReserved: WORD,
            _dwPageSize: DWORD,
            _lpMinimumApplicationAddress: LPVOID,
            _lpMaximumApplicationAddress: LPVOID,
            _dwActiveProcessorMask: DWORD_PTR,
            _dwNumberOfProcessors: DWORD,
            _dwProcessorType: DWORD,
            _dwAllocationGranularity: DWORD,
            _wProcessorLevel: WORD,
            _wProcessorRevision: WORD,
        }

        extern "system" {
            fn GetNativeSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
        }

        unsafe {
            let mut info = mem::zeroed();
            GetNativeSystemInfo(&mut info);
            info.wProcessorArchitecture
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::path::Path;
        // Import the find function from the module level
        use crate::find_tools::find;

        fn host_arch_to_string(host_arch_value: u16) -> &'static str {
            match host_arch_value {
                X86 => "x86",
                X86_64 => "x64",
                AARCH64 => "arm64",
                _ => panic!("Unsupported host architecture: {}", host_arch_value),
            }
        }

        #[test]
        fn test_find_cl_exe() {
            // Test that we can find cl.exe for common target architectures
            // and validate the correct host-target combination paths
            // This should pass on Windows CI with Visual Studio installed

            let target_architectures = ["x64", "x86", "arm64"];
            let mut found_any = false;

            // Determine the host architecture
            let host_arch_value = host_arch();
            let host_name = host_arch_to_string(host_arch_value);

            for &target_arch in &target_architectures {
                if let Some(cmd) = find(target_arch, "cl.exe") {
                    // Verify the command looks valid
                    assert!(
                        !cmd.get_program().is_empty(),
                        "cl.exe program path should not be empty"
                    );
                    assert!(
                        Path::new(cmd.get_program()).exists(),
                        "cl.exe should exist at: {:?}",
                        cmd.get_program()
                    );

                    // Verify the path contains the correct host-target combination
                    // Use case-insensitive comparison since VS IDE uses "Hostx64" while Build Tools use "HostX64"
                    let path_str = cmd.get_program().to_string_lossy();
                    let path_str_lower = path_str.to_lowercase();
                    let expected_host_target_path =
                        format!("\\bin\\host{host_name}\\{target_arch}");
                    let expected_host_target_path_unix =
                        expected_host_target_path.replace("\\", "/");

                    assert!(
                        path_str_lower.contains(&expected_host_target_path) || path_str_lower.contains(&expected_host_target_path_unix),
                        "cl.exe path should contain host-target combination (case-insensitive) '{}' for {} host targeting {}, but found: {}",
                        expected_host_target_path,
                        host_name,
                        target_arch,
                        path_str
                    );

                    found_any = true;
                }
            }

            assert!(found_any, "Expected to find cl.exe for at least one target architecture (x64, x86, or arm64) on Windows CI with Visual Studio installed");
        }

        #[test]
        #[cfg(not(disable_clang_cl_tests))]
        fn test_find_llvm_tools() {
            // Import StdEnvGetter from the parent module
            use crate::find_tools::StdEnvGetter;

            // Test the actual find_llvm_tool function with various LLVM tools
            // This test assumes CI environment has Visual Studio + Clang installed
            // We test against x64 target since clang can cross-compile to any target
            let target_arch = TargetArch::new("x64").expect("Should support x64 architecture");
            let llvm_tools = ["clang.exe", "clang++.exe", "lld.exe", "llvm-ar.exe"];

            // Determine expected host-specific path based on host architecture
            let host_arch_value = host_arch();
            let expected_host_path = match host_arch_value {
                X86 => "LLVM\\bin",            // x86 host
                X86_64 => "LLVM\\x64\\bin",    // x64 host
                AARCH64 => "LLVM\\ARM64\\bin", // arm64 host
                _ => panic!("Unsupported host architecture: {}", host_arch_value),
            };

            let host_name = host_arch_to_string(host_arch_value);

            let mut found_tools_count = 0;

            for &tool in &llvm_tools {
                // Test finding LLVM tools using the standard environment getter
                let env_getter = StdEnvGetter;
                let result = find_llvm_tool(tool, target_arch, &env_getter);

                match result {
                    Some(found_tool) => {
                        found_tools_count += 1;

                        // Verify the found tool has a valid, non-empty path
                        assert!(
                            !found_tool.path().as_os_str().is_empty(),
                            "Found LLVM tool '{}' should have a non-empty path",
                            tool
                        );

                        // Verify the tool path actually exists on filesystem
                        assert!(
                            found_tool.path().exists(),
                            "LLVM tool '{}' path should exist: {:?}",
                            tool,
                            found_tool.path()
                        );

                        // Verify the tool path contains the expected tool name
                        let path_str = found_tool.path().to_string_lossy();
                        assert!(
                            path_str.contains(tool.trim_end_matches(".exe")),
                            "Tool path '{}' should contain tool name '{}'",
                            path_str,
                            tool
                        );

                        // Verify it's in the correct host-specific VS LLVM directory
                        assert!(
                            path_str.contains(expected_host_path) || path_str.contains(&expected_host_path.replace("\\", "/")),
                            "LLVM tool should be in host-specific VS LLVM directory '{}' for {} host, but found: {}",
                            expected_host_path,
                            host_name,
                            path_str
                        );
                    }
                    None => {}
                }
            }

            // On CI with VS + Clang installed, we should find at least some LLVM tools
            assert!(
                found_tools_count > 0,
                "Expected to find at least one LLVM tool on CI with Visual Studio + Clang installed for {} host. Found: {}",
                host_name,
                found_tools_count
            );
        }
    }

    // Given a registry key, look at all the sub keys and find the one which has
    // the maximal numeric value.
    //
    // Returns the name of the maximal key as well as the opened maximal key.
    fn max_version(key: &RegistryKey) -> Option<(OsString, RegistryKey)> {
        let mut max_vers = 0;
        let mut max_key = None;
        for subkey in key.iter().filter_map(|k| k.ok()) {
            let val = subkey
                .to_str()
                .and_then(|s| s.trim_start_matches('v').replace('.', "").parse().ok());
            let val = match val {
                Some(s) => s,
                None => continue,
            };
            if val > max_vers {
                if let Ok(k) = key.open(&subkey) {
                    max_vers = val;
                    max_key = Some((subkey, k));
                }
            }
        }
        max_key
    }

    #[inline(always)]
    pub(super) fn has_msbuild_version(version: &str, env_getter: &dyn EnvGetter) -> bool {
        match version {
            "18.0" => {
                find_msbuild_vs18(TargetArch::X64, env_getter).is_some()
                    || find_msbuild_vs18(TargetArch::X86, env_getter).is_some()
                    || find_msbuild_vs18(TargetArch::Arm64, env_getter).is_some()
            }
            "17.0" => {
                find_msbuild_vs17(TargetArch::X64, env_getter).is_some()
                    || find_msbuild_vs17(TargetArch::X86, env_getter).is_some()
                    || find_msbuild_vs17(TargetArch::Arm64, env_getter).is_some()
            }
            "16.0" => {
                find_msbuild_vs16(TargetArch::X64, env_getter).is_some()
                    || find_msbuild_vs16(TargetArch::X86, env_getter).is_some()
                    || find_msbuild_vs16(TargetArch::Arm64, env_getter).is_some()
            }
            "15.0" => {
                find_msbuild_vs15(TargetArch::X64, env_getter).is_some()
                    || find_msbuild_vs15(TargetArch::X86, env_getter).is_some()
                    || find_msbuild_vs15(TargetArch::Arm64, env_getter).is_some()
            }
            "14.0" => LOCAL_MACHINE
                .open(&OsString::from(format!(
                    "SOFTWARE\\Microsoft\\MSBuild\\ToolsVersions\\{}",
                    version
                )))
                .is_ok(),
            _ => false,
        }
    }

    pub(super) fn find_devenv(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<Tool> {
        find_devenv_vs15(target, env_getter)
    }

    fn find_devenv_vs15(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<Tool> {
        find_tool_in_vs15_path(r"Common7\IDE\devenv.exe", target, env_getter)
    }

    // see http://stackoverflow.com/questions/328017/path-to-msbuild
    pub(super) fn find_msbuild(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<Tool> {
        // VS 15 (2017) changed how to locate msbuild
        if let Some(r) = find_msbuild_vs18(target, env_getter) {
            Some(r)
        } else if let Some(r) = find_msbuild_vs17(target, env_getter) {
            Some(r)
        } else if let Some(r) = find_msbuild_vs16(target, env_getter) {
            return Some(r);
        } else if let Some(r) = find_msbuild_vs15(target, env_getter) {
            return Some(r);
        } else {
            find_old_msbuild(target)
        }
    }

    fn find_msbuild_vs15(target: TargetArch, env_getter: &dyn EnvGetter) -> Option<Tool> {
        find_tool_in_vs15_path(r"MSBuild\15.0\Bin\MSBuild.exe", target, env_getter)
    }

    fn find_old_msbuild(target: TargetArch) -> Option<Tool> {
        let key = r"SOFTWARE\Microsoft\MSBuild\ToolsVersions";
        LOCAL_MACHINE
            .open(key.as_ref())
            .ok()
            .and_then(|key| {
                max_version(&key).and_then(|(_vers, key)| key.query_str("MSBuildToolsPath").ok())
            })
            .map(|path| {
                let mut path = PathBuf::from(path);
                path.push("MSBuild.exe");
                let mut tool = Tool {
                    tool: path,
                    is_clang_cl: false,
                    env: Vec::new(),
                };
                if target == TargetArch::X64 {
                    tool.env.push(("Platform".into(), "X64".into()));
                }
                tool
            })
    }
}

/// Non-Windows Implementation.
#[cfg(not(windows))]
mod impl_ {
    use std::{env, ffi::OsStr, path::PathBuf};

    use super::{EnvGetter, TargetArch};
    use crate::Tool;

    /// Finding msbuild.exe tool under unix system is not currently supported.
    /// Maybe can check it using an environment variable looks like `MSBUILD_BIN`.
    #[inline(always)]
    pub(super) fn find_msbuild(_target: TargetArch, _: &dyn EnvGetter) -> Option<Tool> {
        None
    }

    // Finding devenv.exe tool under unix system is not currently supported.
    // Maybe can check it using an environment variable looks like `DEVENV_BIN`.
    #[inline(always)]
    pub(super) fn find_devenv(_target: TargetArch, _: &dyn EnvGetter) -> Option<Tool> {
        None
    }

    // Finding Clang/LLVM-related tools on unix systems is not currently supported.
    #[inline(always)]
    pub(super) fn find_llvm_tool(
        _tool: &str,
        _target: TargetArch,
        _: &dyn EnvGetter,
    ) -> Option<Tool> {
        None
    }

    /// Attempt to find the tool using environment variables set by vcvars.
    pub(super) fn find_msvc_environment(
        tool: &str,
        _target: TargetArch,
        env_getter: &dyn EnvGetter,
    ) -> Option<Tool> {
        // Early return if the environment doesn't contain a VC install.
        let vc_install_dir = env_getter.get_env("VCINSTALLDIR")?;
        let vs_install_dir = env_getter.get_env("VSINSTALLDIR")?;

        let get_tool = |install_dir: &OsStr| {
            env::split_paths(install_dir)
                .map(|p| p.join(tool))
                .find(|p| p.exists())
                .map(|path| Tool {
                    tool: path,
                    is_clang_cl: false,
                    env: Vec::new(),
                })
        };

        // Take the path of tool for the vc install directory.
        get_tool(vc_install_dir.as_ref())
            // Take the path of tool for the vs install directory.
            .or_else(|| get_tool(vs_install_dir.as_ref()))
            // Take the path of tool for the current path environment.
            .or_else(|| {
                env_getter
                    .get_env("PATH")
                    .as_ref()
                    .map(|path| path.as_ref())
                    .and_then(get_tool)
            })
    }

    #[inline(always)]
    pub(super) fn find_msvc_15plus(
        _tool: &str,
        _target: TargetArch,
        _: &dyn EnvGetter,
    ) -> Option<Tool> {
        None
    }

    // For MSVC 14 we need to find the Universal CRT as well as either
    // the Windows 10 SDK or Windows 8.1 SDK.
    #[inline(always)]
    pub(super) fn find_msvc_14(
        _tool: &str,
        _target: TargetArch,
        _: &dyn EnvGetter,
    ) -> Option<Tool> {
        None
    }

    #[inline(always)]
    pub(super) fn has_msbuild_version(_version: &str, _: &dyn EnvGetter) -> bool {
        false
    }

    #[inline(always)]
    pub(super) fn get_ucrt_dir() -> Option<(PathBuf, String)> {
        None
    }
}
