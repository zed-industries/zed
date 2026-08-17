use self::super::{ParameterBundle, apply_parameters};
use std::path::{PathBuf, Path, MAIN_SEPARATOR};
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::AtomicBool;
use std::process::Command;
use vswhom::VsFindResult;
use std::borrow::Cow;
use winreg::enums::*;
use std::ffi::OsStr;
use std::{env, fs};
use winreg;


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceCompiler;


impl ResourceCompiler {
    #[inline(always)]
    pub fn new() -> ResourceCompiler {
        ResourceCompiler
    }

    #[inline(always)]
    pub fn is_supported(&mut self) -> Option<Cow<'static, str>> {
        None
    }

    pub fn compile_resource<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>>(
        &self, out_dir: &str, prefix: &str, resource: &str, parameters: ParameterBundle<Ms, Mi, Is, Ii>)
        -> Result<String, Cow<'static, str>> {
        let out_file = format!("{}{}{}.lib", out_dir, MAIN_SEPARATOR, prefix);
        // `.res`es are linkable under MSVC as well as normal libraries.
        if !apply_parameters(Command::new(find_windows_sdk_tool_impl("rc.exe").as_ref().map_or(Path::new("rc.exe"), Path::new))
                                 .args(&["/fo", &out_file, "/I", out_dir]),
                             "/D",
                             "/I",
                             parameters)
            .arg(resource)
            .status()
            .map_err(|_| Cow::from("Are you sure you have RC.EXE in your $PATH?"))?
            .success() {
            return Err("RC.EXE failed to compile specified resource file".into());
        }
        Ok(out_file)
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Arch {
    X86,
    X64,
    AArch64,
}

pub fn find_windows_sdk_tool_impl(tool: &str) -> Option<PathBuf> {
    let arch = match env::var("HOST").expect("No HOST env var").as_bytes() {
        [b'x', b'8', b'6', b'_', b'6', b'4', ..] => Arch::X64, // "x86_64"
        [b'a', b'a', b'r', b'c', b'h', b'6', b'4', ..] => Arch::AArch64, // "aarch64"
        _ => Arch::X86,
    };

    find_windows_10_kits_tool("KitsRoot10", arch, tool)
        .or_else(|| find_windows_kits_tool("KitsRoot10", arch, tool))
        .or_else(|| find_windows_kits_tool("KitsRoot81", arch, tool))
        .or_else(|| find_windows_kits_tool("KitsRoot", arch, tool))
        .or_else(|| find_latest_windows_sdk_tool(arch, tool))
        .or_else(|| find_with_vswhom(arch, tool))
}


fn find_with_vswhom(arch: Arch, tool: &str) -> Option<PathBuf> {
    let res = VsFindResult::search();
    res.as_ref()
        .and_then(|res| res.windows_sdk_root.as_ref())
        .map(PathBuf::from)
        .and_then(|mut root| {
            let ver = root.file_name().expect("malformed vswhom-returned SDK root").to_os_string();
            root.pop();
            root.pop();
            root.push("bin");
            root.push(ver);
            try_bin_dir(root, "x86", "x64", "arm64", arch)
        })
        .and_then(|pb| try_tool(pb, tool))
        .or_else(move || {
            res.and_then(|res| res.windows_sdk_root)
                .map(PathBuf::from)
                .and_then(|mut root| {
                    root.pop();
                    root.pop();
                    try_bin_dir(root, "bin/x86", "bin/x64", "bin/arm64", arch)
                })
                .and_then(|pb| try_tool(pb, tool))
        })
}

// Windows 8 - 10
fn find_windows_kits_tool(key: &str, arch: Arch, tool: &str) -> Option<PathBuf> {
    winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(r"SOFTWARE\Microsoft\Windows Kits\Installed Roots", KEY_QUERY_VALUE)
        .and_then(|reg_key| reg_key.get_value::<String, _>(key))
        .ok()
        .and_then(|root_dir| try_bin_dir(root_dir, "bin/x86", "bin/x64", "bin/arm64", arch))
        .and_then(|pb| try_tool(pb, tool))
}

// Windows Vista - 7
fn find_latest_windows_sdk_tool(arch: Arch, tool: &str) -> Option<PathBuf> {
    winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(r"SOFTWARE\Microsoft\Microsoft SDKs\Windows", KEY_QUERY_VALUE)
        .and_then(|reg_key| reg_key.get_value::<String, _>("CurrentInstallFolder"))
        .ok()
        .and_then(|root_dir| try_bin_dir(root_dir, "Bin", "Bin/x64", "Bin/arm64", arch))
        .and_then(|pb| try_tool(pb, tool))
}

// Windows 10 with subdir support
fn find_windows_10_kits_tool(key: &str, arch: Arch, tool: &str) -> Option<PathBuf> {
    let kit_root = (winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(r"SOFTWARE\Microsoft\Windows Kits\Installed Roots", KEY_QUERY_VALUE)
        .and_then(|reg_key| reg_key.get_value::<String, _>(key))
        .ok())?;
    include_windows_10_kits(&kit_root);
    let root_dir = kit_root + "/bin";

    for entry in fs::read_dir(&root_dir).ok()?.filter(|d| d.is_ok()).map(Result::unwrap) {
        let fname = entry.file_name().into_string();
        let ftype = entry.file_type();
        if fname.is_err() || ftype.is_err() || ftype.unwrap().is_file() {
            continue;
        }

        let fname = entry.file_name().into_string().unwrap();
        if let Some(rc) = try_bin_dir(root_dir.clone(),
                                      &format!("{}/x86", fname),
                                      &format!("{}/x64", fname),
                                      &format!("{}/arm64", fname),
                                      arch)
            .and_then(|pb| try_tool(pb, tool)) {
            return Some(rc);
        }
    }

    None
}

/// Update %INCLUDE% to contain all \Include\<version>\ folders before invoking rc.exe
/// (https://github.com/nabijaczleweli/rust-embed-resource/pull/17),
/// fixing "Unable to find windows.h" errors (https://github.com/nabijaczleweli/rust-embed-resource/issues/11)
fn include_windows_10_kits(kit_root: &str) {
    static IS_INCLUDED: AtomicBool = AtomicBool::new(false);

    if !IS_INCLUDED.swap(true, SeqCst) {
        let mut include = env::var("INCLUDE").unwrap_or_default();
        if !include.ends_with(';') {
            include.push(';');
        }

        if let Ok(include_root) = fs::read_dir(kit_root.to_string() + r"\Include\") {
            get_dirs(include_root).filter_map(|dir| fs::read_dir(dir.path()).ok()).for_each(|dir| {
                get_dirs(dir).for_each(|sub_dir| if let Some(sub_dir) = sub_dir.path().to_str() {
                    if !include.contains(sub_dir) {
                        include.push_str(sub_dir);
                        include.push(';');
                    }
                })
            });
        }

        if let Some(cl) = cc::windows_registry::find_tool(env::var("TARGET").expect("No TARGET env var").as_str(), "cl.exe") {
            if let Some((_, ipaths)) = cl.env().iter().find(|(k, _)| k == "INCLUDE") {
                include.push_str(ipaths.to_str().expect("%INCLUDE% from cc nonrepresentable"));
                include.push(';');
            }
        }

        env::set_var("INCLUDE", include);
    }
}

fn get_dirs(read_dir: fs::ReadDir) -> impl Iterator<Item = fs::DirEntry> {
    read_dir.filter_map(|dir| dir.ok()).filter(|dir| dir.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
}

fn try_bin_dir<R: Into<PathBuf>>(root_dir: R, x86_bin: &str, x64_bin: &str, aarch64_bin: &str, arch: Arch) -> Option<PathBuf> {
    try_bin_dir_impl(root_dir.into(), x86_bin, x64_bin, aarch64_bin, arch)
}

fn try_bin_dir_impl(mut root_dir: PathBuf, x86_bin: &str, x64_bin: &str, aarch64_bin: &str, arch: Arch) -> Option<PathBuf> {
    match arch {
        Arch::X86 => root_dir.push(x86_bin),
        Arch::X64 => root_dir.push(x64_bin),
        Arch::AArch64 => root_dir.push(aarch64_bin),
    }

    if root_dir.is_dir() {
        Some(root_dir)
    } else {
        None
    }
}

fn try_tool(mut pb: PathBuf, tool: &str) -> Option<PathBuf> {
    pb.push(tool);
    if pb.exists() { Some(pb) } else { None }
}
