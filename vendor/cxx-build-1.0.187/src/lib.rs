//! The CXX code generator for constructing and compiling C++ code.
//!
//! This is intended to be used from Cargo build scripts to execute CXX's
//! C++ code generator, set up any additional compiler flags depending on
//! the use case, and make the C++ compiler invocation.
//!
//! <br>
//!
//! # Example
//!
//! Example of a canonical Cargo build script that builds a CXX bridge:
//!
//! ```no_run
//! // build.rs
//!
//! fn main() {
//!     cxx_build::bridge("src/main.rs")
//!         .file("src/demo.cc")
//!         .std("c++11")
//!         .compile("cxxbridge-demo");
//!
//!     println!("cargo:rerun-if-changed=src/demo.cc");
//!     println!("cargo:rerun-if-changed=include/demo.h");
//! }
//! ```
//!
//! A runnable working setup with this build script is shown in the *demo*
//! directory of [https://github.com/dtolnay/cxx].
//!
//! [https://github.com/dtolnay/cxx]: https://github.com/dtolnay/cxx
//!
//! <br>
//!
//! # Alternatives
//!
//! For use in non-Cargo builds like Bazel or Buck, CXX provides an
//! alternate way of invoking the C++ code generator as a standalone command
//! line tool. The tool is packaged as the `cxxbridge-cmd` crate.
//!
//! ```bash
//! $ cargo install cxxbridge-cmd  # or build it from the repo
//!
//! $ cxxbridge src/main.rs --header > path/to/mybridge.h
//! $ cxxbridge src/main.rs > path/to/mybridge.cc
//! ```

#![doc(html_root_url = "https://docs.rs/cxx-build/1.0.187")]
#![cfg_attr(not(check_cfg), allow(unexpected_cfgs))]
#![allow(
    clippy::cast_sign_loss,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::enum_glob_use,
    clippy::expl_impl_clone_on_copy, // https://github.com/rust-lang/rust-clippy/issues/15842
    clippy::explicit_auto_deref,
    clippy::inherent_to_string,
    clippy::items_after_statements,
    clippy::match_bool,
    clippy::match_like_matches_macro,
    clippy::match_same_arms,
    clippy::needless_continue,
    clippy::needless_doctest_main,
    clippy::needless_lifetimes,
    clippy::needless_pass_by_value,
    clippy::nonminimal_bool,
    clippy::redundant_else,
    clippy::ref_as_ptr,
    clippy::ref_option,
    clippy::similar_names,
    clippy::single_match_else,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::toplevel_ref_arg,
    clippy::uninlined_format_args,
    clippy::upper_case_acronyms
)]
#![allow(unknown_lints, mismatched_lifetime_syntaxes)]

mod cargo;
mod cfg;
mod deps;
mod error;
mod gen;
mod intern;
mod out;
mod paths;
mod syntax;
mod target;
mod vec;

use crate::cargo::CargoEnvCfgEvaluator;
use crate::deps::{Crate, HeaderDir};
use crate::error::{Error, Result};
use crate::gen::error::report;
use crate::gen::Opt;
use crate::paths::PathExt;
use crate::syntax::map::{Entry, UnorderedMap};
use crate::target::TargetDir;
use cc::Build;
use std::collections::BTreeSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::iter;
use std::path::{Path, PathBuf};
use std::process;

pub use crate::cfg::{Cfg, CFG};

/// This returns a [`cc::Build`] on which you should continue to set up any
/// additional source files or compiler flags, and lastly call its [`compile`]
/// method to execute the C++ build.
///
/// [`compile`]: cc::Build::compile
#[must_use]
pub fn bridge(rust_source_file: impl AsRef<Path>) -> Build {
    bridges(iter::once(rust_source_file))
}

/// `cxx_build::bridge` but for when more than one file contains a
/// #\[cxx::bridge\] module.
///
/// ```no_run
/// let source_files = vec!["src/main.rs", "src/path/to/other.rs"];
/// cxx_build::bridges(source_files)
///     .file("src/demo.cc")
///     .std("c++11")
///     .compile("cxxbridge-demo");
/// ```
#[must_use]
pub fn bridges(rust_source_files: impl IntoIterator<Item = impl AsRef<Path>>) -> Build {
    let ref mut rust_source_files = rust_source_files.into_iter();
    build(rust_source_files).unwrap_or_else(|err| {
        let _ = writeln!(io::stderr(), "\n\ncxxbridge error: {}\n\n", report(err));
        process::exit(1);
    })
}

struct Project {
    include_prefix: PathBuf,
    manifest_dir: PathBuf,
    // The `links = "..."` value from Cargo.toml.
    links_attribute: Option<OsString>,
    // Output directory as received from Cargo.
    out_dir: PathBuf,
    // Directory into which to symlink all generated code.
    //
    // This is *not* used for an #include path, only as a debugging convenience.
    // Normally available at target/cxxbridge/ if we are able to know where the
    // target dir is, otherwise under a common scratch dir.
    //
    // The reason this isn't the #include dir is that we do not want builds to
    // have access to headers from arbitrary other parts of the dependency
    // graph. Using a global directory for all builds would be both a race
    // condition depending on what order Cargo randomly executes the build
    // scripts, as well as semantically undesirable for builds not to have to
    // declare their real dependencies.
    shared_dir: PathBuf,
}

impl Project {
    fn init() -> Result<Self> {
        let include_prefix = Path::new(CFG.include_prefix);
        assert!(include_prefix.is_relative());
        let include_prefix = include_prefix.components().collect();

        let links_attribute = env::var_os("CARGO_MANIFEST_LINKS");

        let manifest_dir = paths::manifest_dir()?;
        let out_dir = paths::out_dir()?;

        let shared_dir = match target::find_target_dir(&out_dir) {
            TargetDir::Path(target_dir) => target_dir.join("cxxbridge"),
            TargetDir::Unknown => scratch::path("cxxbridge"),
        };

        Ok(Project {
            include_prefix,
            manifest_dir,
            links_attribute,
            out_dir,
            shared_dir,
        })
    }
}

// We lay out the OUT_DIR as follows. Everything is namespaced under a cxxbridge
// subdirectory to avoid stomping on other things that the caller's build script
// might be doing inside OUT_DIR.
//
//     $OUT_DIR/
//        cxxbridge/
//           crate/
//              $CARGO_PKG_NAME -> $CARGO_MANIFEST_DIR
//           include/
//              rust/
//                 cxx.h
//              $CARGO_PKG_NAME/
//                 .../
//                    lib.rs.h
//           sources/
//              $CARGO_PKG_NAME/
//                 .../
//                    lib.rs.cc
//
// The crate/ and include/ directories are placed on the #include path for the
// current build as well as for downstream builds that have a direct dependency
// on the current crate.
fn build(rust_source_files: &mut dyn Iterator<Item = impl AsRef<Path>>) -> Result<Build> {
    let ref prj = Project::init()?;
    validate_cfg(prj)?;
    let this_crate = make_this_crate(prj)?;

    let mut build = Build::new();
    build.cpp(true);
    build.cpp_link_stdlib(None); // linked via link-cplusplus crate

    for path in rust_source_files {
        generate_bridge(prj, &mut build, path.as_ref())?;
    }

    this_crate.print_to_cargo();
    eprintln!("\nCXX include path:");
    for header_dir in this_crate.header_dirs {
        build.include(&header_dir.path);
        if header_dir.exported {
            eprintln!("  {}", header_dir.path.display());
        } else {
            eprintln!("  {} (private)", header_dir.path.display());
        }
    }

    Ok(build)
}

fn validate_cfg(prj: &Project) -> Result<()> {
    for exported_dir in &CFG.exported_header_dirs {
        if !exported_dir.is_absolute() {
            return Err(Error::ExportedDirNotAbsolute(exported_dir));
        }
    }

    for prefix in &CFG.exported_header_prefixes {
        if prefix.is_empty() {
            return Err(Error::ExportedEmptyPrefix);
        }
    }

    if prj.links_attribute.is_none() {
        if !CFG.exported_header_dirs.is_empty() {
            return Err(Error::ExportedDirsWithoutLinks);
        }
        if !CFG.exported_header_prefixes.is_empty() {
            return Err(Error::ExportedPrefixesWithoutLinks);
        }
        if !CFG.exported_header_links.is_empty() {
            return Err(Error::ExportedLinksWithoutLinks);
        }
    }

    Ok(())
}

fn make_this_crate(prj: &Project) -> Result<Crate> {
    let crate_dir = make_crate_dir(prj);
    let include_dir = make_include_dir(prj)?;

    let mut this_crate = Crate {
        include_prefix: Some(prj.include_prefix.clone()),
        links: prj.links_attribute.clone(),
        header_dirs: Vec::new(),
    };

    // The generated code directory (include_dir) is placed in front of
    // crate_dir on the include line so that `#include "path/to/file.rs"` from
    // C++ "magically" works and refers to the API generated from that Rust
    // source file.
    this_crate.header_dirs.push(HeaderDir {
        exported: true,
        path: include_dir,
    });

    this_crate.header_dirs.push(HeaderDir {
        exported: true,
        path: crate_dir,
    });

    for exported_dir in &CFG.exported_header_dirs {
        this_crate.header_dirs.push(HeaderDir {
            exported: true,
            path: PathBuf::from(exported_dir),
        });
    }

    let mut header_dirs_index = UnorderedMap::new();
    let mut used_header_links = BTreeSet::new();
    let mut used_header_prefixes = BTreeSet::new();
    for krate in deps::direct_dependencies() {
        let mut is_link_exported = || match &krate.links {
            None => false,
            Some(links_attribute) => CFG.exported_header_links.iter().any(|&exported| {
                let matches = links_attribute == exported;
                if matches {
                    used_header_links.insert(exported);
                }
                matches
            }),
        };

        let mut is_prefix_exported = || match &krate.include_prefix {
            None => false,
            Some(include_prefix) => CFG.exported_header_prefixes.iter().any(|&exported| {
                let matches = include_prefix.starts_with(exported);
                if matches {
                    used_header_prefixes.insert(exported);
                }
                matches
            }),
        };

        let exported = is_link_exported() || is_prefix_exported();

        for dir in krate.header_dirs {
            // Deduplicate dirs reachable via multiple transitive dependencies.
            match header_dirs_index.entry(dir.path.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(this_crate.header_dirs.len());
                    this_crate.header_dirs.push(HeaderDir {
                        exported,
                        path: dir.path,
                    });
                }
                Entry::Occupied(entry) => {
                    let index = *entry.get();
                    this_crate.header_dirs[index].exported |= exported;
                }
            }
        }
    }

    if let Some(unused) = CFG
        .exported_header_links
        .iter()
        .find(|&exported| !used_header_links.contains(exported))
    {
        return Err(Error::UnusedExportedLinks(unused));
    }

    if let Some(unused) = CFG
        .exported_header_prefixes
        .iter()
        .find(|&exported| !used_header_prefixes.contains(exported))
    {
        return Err(Error::UnusedExportedPrefix(unused));
    }

    Ok(this_crate)
}

fn make_crate_dir(prj: &Project) -> PathBuf {
    if prj.include_prefix.as_os_str().is_empty() {
        return prj.manifest_dir.clone();
    }
    let crate_dir = prj.out_dir.join("cxxbridge").join("crate");
    let ref link = crate_dir.join(&prj.include_prefix);
    let ref manifest_dir = prj.manifest_dir;
    if out::relative_symlink_dir(manifest_dir, link).is_err() && cfg!(not(unix)) {
        let cachedir_tag = "\
        Signature: 8a477f597d28d172789f06886806bc55\n\
        # This file is a cache directory tag created by cxx.\n\
        # For information about cache directory tags see https://bford.info/cachedir/\n";
        let _ = out::write(crate_dir.join("CACHEDIR.TAG"), cachedir_tag.as_bytes());
        let max_depth = 6;
        best_effort_copy_headers(manifest_dir, link, max_depth);
    }
    crate_dir
}

fn make_include_dir(prj: &Project) -> Result<PathBuf> {
    let include_dir = prj.out_dir.join("cxxbridge").join("include");
    let cxx_h = include_dir.join("rust").join("cxx.h");
    let ref shared_cxx_h = prj.shared_dir.join("rust").join("cxx.h");
    if let Some(ref original) = env::var_os("DEP_CXXBRIDGE1_HEADER") {
        out::absolute_symlink_file(original, cxx_h)?;
        out::absolute_symlink_file(original, shared_cxx_h)?;
    } else {
        out::write(shared_cxx_h, gen::include::HEADER.as_bytes())?;
        out::relative_symlink_file(shared_cxx_h, cxx_h)?;
    }
    Ok(include_dir)
}

fn generate_bridge(prj: &Project, build: &mut Build, rust_source_file: &Path) -> Result<()> {
    let opt = Opt {
        allow_dot_includes: false,
        cfg_evaluator: Box::new(CargoEnvCfgEvaluator),
        doxygen: CFG.doxygen,
        ..Opt::default()
    };
    println!("cargo:rerun-if-changed={}", rust_source_file.display());
    let generated = gen::generate_from_path(rust_source_file, &opt);
    let ref rel_path = paths::local_relative_path(rust_source_file);

    let cxxbridge = prj.out_dir.join("cxxbridge");
    let include_dir = cxxbridge.join("include").join(&prj.include_prefix);
    let sources_dir = cxxbridge.join("sources").join(&prj.include_prefix);

    let ref rel_path_h = rel_path.with_appended_extension(".h");
    let ref header_path = include_dir.join(rel_path_h);
    out::write(header_path, &generated.header)?;

    let ref link_path = include_dir.join(rel_path);
    let _ = out::relative_symlink_file(header_path, link_path);

    let ref rel_path_cc = rel_path.with_appended_extension(".cc");
    let ref implementation_path = sources_dir.join(rel_path_cc);
    out::write(implementation_path, &generated.implementation)?;
    build.file(implementation_path);

    let shared_h = prj.shared_dir.join(&prj.include_prefix).join(rel_path_h);
    let shared_cc = prj.shared_dir.join(&prj.include_prefix).join(rel_path_cc);
    let _ = out::relative_symlink_file(header_path, shared_h);
    let _ = out::relative_symlink_file(implementation_path, shared_cc);
    Ok(())
}

fn best_effort_copy_headers(src: &Path, dst: &Path, max_depth: usize) {
    // Not using crate::gen::fs because we aren't reporting the errors.
    use std::fs;

    let mut dst_created = false;
    let Ok(mut entries) = fs::read_dir(src) else {
        return;
    };

    while let Some(Ok(entry)) = entries.next() {
        let file_name = entry.file_name();
        if file_name.to_string_lossy().starts_with('.') {
            continue;
        }
        match entry.file_type() {
            Ok(file_type) if file_type.is_dir() && max_depth > 0 => {
                let src = entry.path();
                if src.join("Cargo.toml").exists() || src.join("CACHEDIR.TAG").exists() {
                    continue;
                }
                let dst = dst.join(file_name);
                best_effort_copy_headers(&src, &dst, max_depth - 1);
            }
            Ok(file_type) if file_type.is_file() => {
                let src = entry.path();
                match src.extension().and_then(OsStr::to_str) {
                    Some("h" | "hh" | "hpp") => {}
                    _ => continue,
                }
                if !dst_created && fs::create_dir_all(dst).is_err() {
                    return;
                }
                dst_created = true;
                let dst = dst.join(file_name);
                let _ = fs::remove_file(&dst);
                let _ = fs::copy(src, dst);
            }
            _ => {}
        }
    }
}

fn env_os(key: impl AsRef<OsStr>) -> Result<OsString> {
    let key = key.as_ref();
    env::var_os(key).ok_or_else(|| Error::NoEnv(key.to_owned()))
}
