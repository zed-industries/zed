//! A [`Cargo` build script](http://doc.crates.io/build-script.html) library to handle compilation and inclusion of Windows
//! resources in the most resilient fashion imaginable
//!
//! # Background
//!
//! Including Windows resources seems very easy at first, despite the build scripts' abhorrent documentation:
//! [compile with `windres`, then make linkable with
//! `ar`](https://github.com/nabijaczleweli/cargo-update/commit/ef4346c#diff-a7b0a2dee0126cddf994326e705a91ea).
//!
//! I was very happy with that solution until it was brought to my attention, that [MSVC uses something
//! different](https://github.com/nabijaczleweli/cargo-update/commit/f57e9c3#diff-a7b0a2dee0126cddf994326e705a91ea),
//! and now either `windres`-`ar` combo or `RC.EXE` would be used, which was OK.
//!
//! Later it transpired, that [MSVC is even more incompatible with everything
//! else](https://github.com/nabijaczleweli/cargo-update/commit/39fa758#diff-a7b0a2dee0126cddf994326e705a91ea)
//! by way of not having `RC.EXE` in `$PATH` (because it would only be reasonable to do so),
//! so another MSVC artisan made the script [find the most likely places for `RC.EXE` to
//! be](https://github.com/nabijaczleweli/cargo-update/pull/22), and the script grew yet again,
//! now standing at 100 lines and 3.2 kB.
//!
//! After [copying the build script in its
//! entirety](https://github.com/thecoshman/http/commit/98205a4#diff-a7b0a2dee0126cddf994326e705a91ea)
//! and realising how error-prone that was, then being [nudged by
//! Shepmaster](https://chat.stackoverflow.com/transcript/message/35378953#35378953)
//! to extract it to a crate, here we are.
//!
//! # Usage
//!
//! For the purposes of the demonstration we will assume that the resource file's name
//! is "checksums.rc", but it can be any name relative to the crate root.
//!
//! In `Cargo.toml`:
//!
//! ```toml
//! # The general section with crate name, license, etc.
//! build = "build.rs"
//!
//! [build-dependencies]
//! embed-resource = "3.0"
//! ```
//!
//! In `build.rs`:
//!
//! ```rust,no_run
//! extern crate embed_resource;
//!
//! fn main() {
//!     embed_resource::compile("checksums.rc", embed_resource::NONE).manifest_optional().unwrap();
//!     // or
//!     embed_resource::compile("checksums.rc", &["VERSION=000901"]).manifest_required().unwrap();
//!     // or
//!     embed_resource::compile("checksums.rc", embed_resource::ParamsMacrosAndIncludeDirs(
//!         &["VERSION=000901"], &["src/include"])).manifest_required().unwrap();
//!     // or
//!     embed_resource::compile("checksums.rc", embed_resource::ParamsIncludeDirs(
//!         &["src/include"])).manifest_required().unwrap();
//! }
//! ```
//!
//! Use `.manifest_optional().unwrap()` if the manifest is cosmetic (like an icon).<br />
//! Use `.manifest_required().unwrap()` if the manifest is required (security, entry point, &c.).
//!
//! Parameters that look like `&["string"]` or `embed_resource::NONE` in the example above
//! can be anything that satisfies `IntoIterator<AsRef<OsStr>>`:
//! `&[&str]`, of course, but also `Option<PathBuf>`, `Vec<OsString>`, `BTreeSet<&Path>`, &c.
//!
//! ## Errata
//!
//! If no `cargo:rerun-if-changed` annotations are generated, Cargo scans the entire build root by default.
//! Because the first step in building a manifest is an unspecified C preprocessor step with-out the ability to generate the
//! equivalent of `cc -MD`, we do *not* output said annotation.
//!
//! If scanning is prohibitively expensive, or you have something else that generates the annotations, you may want to spec the
//! full non-system dependency list for your manifest manually, so:
//! ```rust,no_run
//! println!("cargo:rerun-if-changed=app-name-manifest.rc");
//! embed_resource::compile("app-name-manifest.rc", embed_resource::NONE);
//! ```
//! for the above example (cf. [#41](https://github.com/nabijaczleweli/rust-embed-resource/issues/41)).
//!
//! # Cross-compilation
//!
//! It is possible to embed resources in Windows executables built on non-Windows hosts. There are two ways to do this:
//!
//! When targetting `*-pc-windows-gnu`, `*-w64-mingw32-windres` is attempted by default, for `*-pc-windows-msvc` it's `llvm-rc`,
//! this can be overriden by setting `RC_$TARGET`, `RC_${TARGET//-/_}`, or `RC` environment variables.
//!
//! When compiling with LLVM-RC, an external C compiler is used to preprocess the resource,
//! preloaded with configuration from
//! [`cc`](https://github.com/alexcrichton/cc-rs#external-configuration-via-environment-variables).
//!
//! ## Migration
//! ### 2.x
//!
//! Add `embed_resource::NONE` as the last argument to `embed_resource::compile()` and `embed_resource::compile_for()`.
//!
//! ### 3.x
//!
//! Add `.manifest_optional().unwrap()` or `.manifest_required().unwrap()` to all [`compile()`] and `compile_for*()` calls.
//! `CompilationResult` is `#[must_use]` so should be highlighted automatically.
//!
//! Embed-resource <3.x always behaves like `.manifest_optional().unwrap()`.
//!
//! # Credit
//!
//! In chronological order:
//!
//! [@liigo](https://github.com/liigo) -- persistency in pestering me and investigating problems where I have failed
//!
//! [@mzji](https://github.com/mzji) -- MSVC lab rat
//!
//! [@TheCatPlusPlus](https://github.com/TheCatPlusPlus) -- knowledge and providing first iteration of manifest-embedding code
//!
//! [@azyobuzin](https://github.com/azyobuzin) -- providing code for finding places where RC.EXE could hide
//!
//! [@retep998](https://github.com/retep998) -- fixing MSVC support
//!
//! [@SonnyX](https://github.com/SonnyX) -- Windows cross-compilation support and testing
//!
//! [@MSxDOS](https://github.com/MSxDOS) -- finding and supplying RC.EXE its esoteric header include paths
//!
//! [@roblabla](https://github.com/roblabla) -- cross-compilation to Windows MSVC via LLVM-RC
//!
//! # Special thanks
//!
//! To all who support further development on [Patreon](https://patreon.com/nabijaczleweli), in particular:
//!
//!   * ThePhD
//!   * Embark Studios
//!   * Lars Strojny
//!   * EvModder

#![allow(private_bounds)]


#[cfg(any(not(target_os = "windows"), all(target_os = "windows", target_env = "msvc")))]
extern crate cc;
#[cfg(not(target_os = "windows"))]
extern crate memchr;
#[cfg(all(target_os = "windows", target_env = "msvc"))]
extern crate vswhom;
#[cfg(all(target_os = "windows", target_env = "msvc"))]
extern crate winreg;
extern crate rustc_version;
extern crate toml;

#[cfg(not(target_os = "windows"))]
mod non_windows;
#[cfg(all(target_os = "windows", target_env = "msvc"))]
mod windows_msvc;
#[cfg(all(target_os = "windows", not(target_env = "msvc")))]
mod windows_not_msvc;

#[cfg(not(target_os = "windows"))]
use self::non_windows::*;
#[cfg(all(target_os = "windows", target_env = "msvc"))]
use self::windows_msvc::*;
#[cfg(all(target_os = "windows", not(target_env = "msvc")))]
use self::windows_not_msvc::*;

use std::{env, fs};
use std::ffi::OsStr;
use std::borrow::Cow;
use std::process::Command;
use toml::Table as TomlTable;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};


/// Empty slice, properly-typed for [`compile()`] and `compile_for*()` to mean "no additional parameters".
///
/// Rust helpfully forbids default type parameters on functions, so just passing `[]` doesn't work :)
pub const NONE: &[&OsStr] = &[];


// This is all of the parameters and it's non-public:
// the only way users can construct this is via From<Mi> (same as From<ParamsMacros>), From<ParamsIncludeDirs>,
// and From<ParamsMacrosAndIncludeDirs>
#[derive(PartialEq, Eq, Debug)] // only for tests
struct ParameterBundle<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>> {
    macros: Mi,
    include_dirs: Ii,
}

impl<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>> From<Mi> for ParameterBundle<Ms, Mi, &'static &'static OsStr, &'static [&'static OsStr]> {
    fn from(macros: Mi) -> Self {
        ParamsMacros(macros).into()
    }
}

/// Give this to [`compile()`] or `compile_for*()` to add some macro definitions (`-D`/`/D`).
///
/// Every value must be in the form `MACRO=value` or `MACRO`. An empty iterator is a no-op.
pub struct ParamsMacros<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>>(pub Mi);
impl<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>> From<ParamsMacros<Ms, Mi>> for ParameterBundle<Ms, Mi, &'static &'static OsStr, &'static [&'static OsStr]> {
    fn from(macros: ParamsMacros<Ms, Mi>) -> Self {
        ParamsMacrosAndIncludeDirs(macros.0, NONE).into()
    }
}

/// Give this to [`compile()`] or `compile_for*()` to add include directories (`-I`/`/I`).
///
/// An empty iterator is a no-op.
pub struct ParamsIncludeDirs<Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>>(pub Ii);
impl<Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>> From<ParamsIncludeDirs<Is, Ii>>
    for ParameterBundle<&'static &'static OsStr, &'static [&'static OsStr], Is, Ii> {
    fn from(include_dirs: ParamsIncludeDirs<Is, Ii>) -> Self {
        ParamsMacrosAndIncludeDirs(NONE, include_dirs.0).into()
    }
}

/// Give this to [`compile()`] or `compile_for*()` to add some macro definitions (`-D`/`/D`) and include directories
/// (`-I`/`/I`).
///
/// Every macro value must be in the form `MACRO=value` or `MACRO`.
///
/// Empty iterators are no-ops.
pub struct ParamsMacrosAndIncludeDirs<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>>(pub Mi, pub Ii);
impl<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>> From<ParamsMacrosAndIncludeDirs<Ms, Mi, Is, Ii>>
    for ParameterBundle<Ms, Mi, Is, Ii> {
    fn from(maid: ParamsMacrosAndIncludeDirs<Ms, Mi, Is, Ii>) -> Self {
        Self {
            macros: maid.0,
            include_dirs: maid.1,
        }
    }
}


/// https://101010.pl/@nabijaczleweli/115226665478478763
#[cfg(test)]
#[allow(dead_code)]
fn compat_3_0_5() {
    use std::collections::BTreeSet;

    // these spellings of the macros argument taken from GitHub "embed_resource::" search
    //                                               and https://crates.io/crates/embed-resource/reverse_dependencies on 2025-09-18
    let _ = compile("", std::iter::empty::<&str>());
    let _ = compile("", None::<&str>);
    let marcos = &[format!("VERSION_PATCH={}", env!("CARGO_PKG_VERSION_PATCH"))];
    let _ = compile("", marcos);
    let marcos = vec![format!("VERSION_PATCH={}", env!("CARGO_PKG_VERSION_PATCH"))];
    let _ = compile("", marcos);

    // these weren't
    let _ = compile("", [""]);
    let _ = compile("", &[""]);
    let _ = compile("", vec![""]);
    let _ = compile("", vec![Path::new("gaming=baming")].into_iter().collect::<BTreeSet<_>>());
    let _ = compile("", vec![Path::new("gaming=baming").to_owned()].into_iter().collect::<BTreeSet<_>>());
    let _ = compile("", [PathBuf::from("gaming=baming")].iter());
    let _ = compile("", [PathBuf::from("gaming=baming")].iter().collect::<BTreeSet<_>>());

    // this is new
    let _ = compile("", ParamsIncludeDirs(&[Path::new("include_dir")]));
    let _ = compile("", ParamsIncludeDirs([PathBuf::from("include_dir")]));
    let _ = compile("", ParamsIncludeDirs(vec![Path::new("include_dir1"), Path::new("include_dir2")]));

    let _ = compile("", ParamsMacrosAndIncludeDirs(NONE, NONE));
    let _ = compile("", ParamsMacrosAndIncludeDirs([""], [""]));
}

#[test]
fn argument_bundle_into() {
    assert_eq!(ParameterBundle::from(NONE),
               ParameterBundle {
                   macros: NONE,
                   include_dirs: NONE,
               });
    assert_eq!(ParameterBundle::from([""]),
               ParameterBundle {
                   macros: [""],
                   include_dirs: NONE,
               });

    assert_eq!(ParameterBundle::from(ParamsMacros(NONE)),
               ParameterBundle {
                   macros: NONE,
                   include_dirs: NONE,
               });
    assert_eq!(ParameterBundle::from(ParamsMacros([""])),
               ParameterBundle {
                   macros: [""],
                   include_dirs: NONE,
               });

    assert_eq!(ParameterBundle::from(ParamsIncludeDirs(NONE)),
               ParameterBundle {
                   macros: NONE,
                   include_dirs: NONE,
               });
    assert_eq!(ParameterBundle::from(ParamsIncludeDirs([""])),
               ParameterBundle {
                   macros: NONE,
                   include_dirs: [""],
               });

    assert_eq!(ParameterBundle::from(ParamsMacrosAndIncludeDirs(NONE, NONE)),
               ParameterBundle {
                   macros: NONE,
                   include_dirs: NONE,
               });
    assert_eq!(ParameterBundle::from(ParamsMacrosAndIncludeDirs([""], NONE)),
               ParameterBundle {
                   macros: [""],
                   include_dirs: NONE,
               });
    assert_eq!(ParameterBundle::from(ParamsMacrosAndIncludeDirs(NONE, [""])),
               ParameterBundle {
                   macros: NONE,
                   include_dirs: [""],
               });
    assert_eq!(ParameterBundle::from(ParamsMacrosAndIncludeDirs([""], [""])),
               ParameterBundle {
                   macros: [""],
                   include_dirs: [""],
               });
}


/// Result of [`compile()`] and `compile_for*()`
///
/// Turn this into a `Result` with `manifest_optional()` if the manifest is nice, but isn't required, like when embedding an
/// icon or some other cosmetic.
///
/// Turn this into a `Result` with `manifest_required()` if the manifest is mandatory, like when configuring entry points or
/// security.
#[must_use]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompilationResult {
    /// not building for windows
    NotWindows,
    /// built, linked
    Ok,
    /// building for windows, but the environment can't compile a resource (most likely due to a missing compiler)
    NotAttempted(Cow<'static, str>),
    /// environment can compile a resource, but has failed to do so
    Failed(Cow<'static, str>),
}
impl CompilationResult {
    /// `Ok(())` if `NotWindows`, `Ok`, or `NotAttempted`; `Err(self)` if `Failed`
    pub fn manifest_optional(self) -> Result<(), CompilationResult> {
        match self {
            CompilationResult::NotWindows |
            CompilationResult::Ok |
            CompilationResult::NotAttempted(..) => Ok(()),
            err @ CompilationResult::Failed(..) => Err(err),
        }
    }

    /// `Ok(())` if `NotWindows`, `Ok`; `Err(self)` if `NotAttempted` or `Failed`
    pub fn manifest_required(self) -> Result<(), CompilationResult> {
        match self {
            CompilationResult::NotWindows |
            CompilationResult::Ok => Ok(()),
            err @ CompilationResult::NotAttempted(..) |
            err @ CompilationResult::Failed(..) => Err(err),
        }
    }
}
impl Display for CompilationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("embed-resource: ")?;
        match self {
            CompilationResult::NotWindows => f.write_str("not building for windows"),
            CompilationResult::Ok => f.write_str("OK"),
            CompilationResult::NotAttempted(why) => {
                f.write_str("compilation not attempted: ")?;
                if !why.contains(' ') {
                    f.write_str("missing compiler: ")?;
                }
                f.write_str(why)
            }
            CompilationResult::Failed(err) => f.write_str(err),
        }
    }
}
impl std::error::Error for CompilationResult {}

macro_rules! try_compile_impl {
    ($expr:expr) => {
        match $expr {
            Result::Ok(val) => val,
            Result::Err(err) => return err,
        }
    };
}


/// Compile the Windows resource file and update the cargo search path if building for Windows.
///
/// On non-Windows non-Windows-cross-compile-target this does nothing, on non-MSVC Windows and Windows cross-compile targets,
/// this chains `windres` with `ar`,
/// but on MSVC Windows, this will try its hardest to find `RC.EXE` in Windows Kits and/or SDK directories,
/// falling back to [Jon Blow's VS discovery script](https://pastebin.com/3YvWQa5c),
/// and on Windows 10 `%INCLUDE%` will be updated to help `RC.EXE` find `windows.h` and friends.
///
/// `$OUT_DIR` is added to the include search path.
///
/// Note that this does *nothing* if building with rustc before 1.50.0 and there's a library in the crate,
/// since the resource is linked to the library, if any, instead of the binaries.
///
/// Since rustc 1.50.0, the resource is linked only to the binaries
/// (unless there are none, in which case it's also linked to the library).
///
/// `parameters` are a list of macros to define (directly or via [`ParamsMacros`]), in standard `NAME`/`NAME=VALUE` format,
/// [`ParamsIncludeDirs`], or [`ParamsMacrosAndIncludeDirs`].
///
/// # Examples
///
/// In your build script, assuming the crate's name is "checksums":
///
/// ```rust,no_run
/// extern crate embed_resource;
///
/// fn main() {
///     // Compile and link checksums.rc
///     embed_resource::compile("checksums.rc", embed_resource::NONE);
/// }
/// ```
pub fn compile<T: AsRef<Path>,
               Ms: AsRef<OsStr>,
               Mi: IntoIterator<Item = Ms>,
               Is: AsRef<OsStr>,
               Ii: IntoIterator<Item = Is>,
               P: Into<ParameterBundle<Ms, Mi, Is, Ii>>>(
    resource_file: T, parameters: P)
    -> CompilationResult {
    let (prefix, out_dir, out_file) = try_compile_impl!(compile_impl(resource_file.as_ref(), parameters.into()));
    let hasbins = fs::read_to_string("Cargo.toml")
        .unwrap_or_else(|err| {
            eprintln!("Couldn't read Cargo.toml: {}; assuming src/main.rs or S_ISDIR(src/bin/)", err);
            String::new()
        })
        .parse::<TomlTable>()
        .unwrap_or_else(|err| {
            eprintln!("Couldn't parse Cargo.toml: {}; assuming src/main.rs or S_ISDIR(src/bin/)", err);
            TomlTable::new()
        })
        .contains_key("bin") || (Path::new("src/main.rs").exists() || Path::new("src/bin").is_dir());
    eprintln!("Final verdict: crate has binaries: {}", hasbins);

    if hasbins && rustc_version::version().expect("couldn't get rustc version") >= rustc_version::Version::new(1, 50, 0) {
        println!("cargo:rustc-link-arg-bins={}", out_file);
    } else {
        // Cargo pre-0.51.0 (rustc pre-1.50.0) compat
        // Only links to the calling crate's library
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=dylib={}", prefix);
    }
    CompilationResult::Ok
}

/// Likewise, but only for select binaries.
///
/// Only available since rustc 1.55.0, does nothing before.
///
/// # Examples
///
/// ```rust,no_run
/// extern crate embed_resource;
///
/// fn main() {
/// embed_resource::compile_for("assets/poke-a-mango.rc", &["poke-a-mango", "poke-a-mango-installer"],
///                             &["VERSION=\"0.5.0\""]);
///     embed_resource::compile_for("assets/uninstaller.rc", &["unins001"], embed_resource::NONE);
/// }
/// ```
pub fn compile_for<T: AsRef<Path>,
                   J: Display,
                   I: IntoIterator<Item = J>,
                   Ms: AsRef<OsStr>,
                   Mi: IntoIterator<Item = Ms>,
                   Is: AsRef<OsStr>,
                   Ii: IntoIterator<Item = Is>,
                   P: Into<ParameterBundle<Ms, Mi, Is, Ii>>>(
    resource_file: T, for_bins: I, parameters: P)
    -> CompilationResult {
    let (_, _, out_file) = try_compile_impl!(compile_impl(resource_file.as_ref(), parameters.into()));
    for bin in for_bins {
        println!("cargo:rustc-link-arg-bin={}={}", bin, out_file);
    }
    CompilationResult::Ok
}

/// Likewise, but only link the resource to test binaries (select types only. unclear which (and likely to change). you may
/// prefer [`compile_for_everything()`]).
///
/// Only available since rustc 1.60.0, does nothing before.
pub fn compile_for_tests<T: AsRef<Path>,
                         Ms: AsRef<OsStr>,
                         Mi: IntoIterator<Item = Ms>,
                         Is: AsRef<OsStr>,
                         Ii: IntoIterator<Item = Is>,
                         P: Into<ParameterBundle<Ms, Mi, Is, Ii>>>(
    resource_file: T, parameters: P)
    -> CompilationResult {
    let (_, _, out_file) = try_compile_impl!(compile_impl(resource_file.as_ref(), parameters.into()));
    println!("cargo:rustc-link-arg-tests={}", out_file);
    CompilationResult::Ok
}

/// Likewise, but only link the resource to benchmarks.
///
/// Only available since rustc 1.60.0, does nothing before.
pub fn compile_for_benchmarks<T: AsRef<Path>,
                              Ms: AsRef<OsStr>,
                              Mi: IntoIterator<Item = Ms>,
                              Is: AsRef<OsStr>,
                              Ii: IntoIterator<Item = Is>,
                              P: Into<ParameterBundle<Ms, Mi, Is, Ii>>>(
    resource_file: T, parameters: P)
    -> CompilationResult {
    let (_, _, out_file) = try_compile_impl!(compile_impl(resource_file.as_ref(), parameters.into()));
    println!("cargo:rustc-link-arg-benches={}", out_file);
    CompilationResult::Ok
}

/// Likewise, but only link the resource to examples.
///
/// Only available since rustc 1.60.0, does nothing before.
pub fn compile_for_examples<T: AsRef<Path>,
                            Ms: AsRef<OsStr>,
                            Mi: IntoIterator<Item = Ms>,
                            Is: AsRef<OsStr>,
                            Ii: IntoIterator<Item = Is>,
                            P: Into<ParameterBundle<Ms, Mi, Is, Ii>>>(
    resource_file: T, parameters: P)
    -> CompilationResult {
    let (_, _, out_file) = try_compile_impl!(compile_impl(resource_file.as_ref(), parameters.into()));
    println!("cargo:rustc-link-arg-examples={}", out_file);
    CompilationResult::Ok
}

/// Likewise, but link the resource into *every* artifact: binaries, cdylibs, examples, tests (`[[test]]`/`#[test]`/doctest),
/// benchmarks, &c.
///
/// Only available since rustc 1.50.0, does nothing before.
pub fn compile_for_everything<T: AsRef<Path>,
                              Ms: AsRef<OsStr>,
                              Mi: IntoIterator<Item = Ms>,
                              Is: AsRef<OsStr>,
                              Ii: IntoIterator<Item = Is>,
                              P: Into<ParameterBundle<Ms, Mi, Is, Ii>>>(
    resource_file: T, parameters: P)
    -> CompilationResult {
    let (_, _, out_file) = try_compile_impl!(compile_impl(resource_file.as_ref(), parameters.into()));
    println!("cargo:rustc-link-arg={}", out_file);
    CompilationResult::Ok
}

fn compile_impl<Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>, P: Into<ParameterBundle<Ms, Mi, Is, Ii>>>(
    resource_file: &Path, parameters: P)
    -> Result<(&str, String, String), CompilationResult> {
    let mut comp = ResourceCompiler::new();
    if let Some(missing) = comp.is_supported() {
        if missing.is_empty() {
            Err(CompilationResult::NotWindows)
        } else {
            Err(CompilationResult::NotAttempted(missing))
        }
    } else {
        let prefix = &resource_file.file_stem().expect("resource_file has no stem").to_str().expect("resource_file's stem not UTF-8");
        let out_dir = env::var("OUT_DIR").expect("No OUT_DIR env var");

        let out_file = comp.compile_resource(&out_dir, &prefix, resource_file.to_str().expect("resource_file not UTF-8"), parameters.into())
            .map_err(CompilationResult::Failed)?;
        Ok((prefix, out_dir, out_file))
    }
}

fn apply_parameters<'t, Ms: AsRef<OsStr>, Mi: IntoIterator<Item = Ms>, Is: AsRef<OsStr>, Ii: IntoIterator<Item = Is>>(to: &'t mut Command, macro_pref: &str,
                                                                                                                      include_dir_pref: &str,
                                                                                                                      parameters: ParameterBundle<Ms,
                                                                                                                                                  Mi,
                                                                                                                                                  Is,
                                                                                                                                                  Ii>)
                                                                                                                      -> &'t mut Command {
    for m in parameters.macros {
        to.arg(macro_pref).arg(m);
    }
    for id in parameters.include_dirs {
        to.arg(include_dir_pref).arg(id);
    }
    to
}


/// Find MSVC build tools other than the compiler and linker
///
/// On Windows + MSVC this can be used try to find tools such as `MIDL.EXE` in Windows Kits and/or SDK directories.
///
/// The compilers and linkers can be better found with the `cc` or `vswhom` crates.
/// This always returns `None` on non-MSVC targets.
///
/// # Examples
///
/// In your build script, find `midl.exe` and use it to compile an IDL file:
///
/// ```rust,no_run
/// # #[cfg(all(target_os = "windows", target_env = "msvc"))]
/// # {
/// extern crate embed_resource;
/// extern crate vswhom;
/// # use std::env;
/// # use std::process::Command;
///
/// let midl = embed_resource::find_windows_sdk_tool("midl.exe").unwrap();
///
/// // midl.exe uses cl.exe as a preprocessor, so it needs to be in PATH
/// let vs_locations = vswhom::VsFindResult::search().unwrap();
/// let output = Command::new(midl)
///     .env("PATH", vs_locations.vs_exe_path.unwrap())
///     .args(&["/out", &env::var("OUT_DIR").unwrap()])
///     .arg("haka.pfx.idl").output().unwrap();
///
/// assert!(output.status.success());
/// # }
/// ```
pub fn find_windows_sdk_tool<T: AsRef<str>>(tool: T) -> Option<PathBuf> {
    find_windows_sdk_tool_impl(tool.as_ref())
}
