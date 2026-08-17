/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::{Path, PathBuf};

use crate::bindgen::cargo::cargo_expand;
use crate::bindgen::cargo::cargo_lock::{self, Lock};
pub(crate) use crate::bindgen::cargo::cargo_metadata::PackageRef;
use crate::bindgen::cargo::cargo_metadata::{self, Metadata};
use crate::bindgen::cargo::cargo_toml;
use crate::bindgen::config::Profile;
use crate::bindgen::error::Error;
use crate::bindgen::ir::Cfg;

/// Parse a dependency string used in Cargo.lock
fn parse_dep_string(dep_string: &str) -> (&str, Option<&str>) {
    let split: Vec<&str> = dep_string.split_whitespace().collect();

    (split[0], split.get(1).cloned())
}

/// A collection of metadata for a library from cargo.
#[derive(Clone, Debug)]
pub(crate) struct Cargo {
    manifest_path: PathBuf,
    binding_crate_name: String,
    lock: Option<Lock>,
    metadata: Metadata,
    clean: bool,
}

impl Cargo {
    /// Gather metadata from cargo for a specific library and binding crate
    /// name. If dependency finding isn't needed then Cargo.lock files don't
    /// need to be parsed.
    pub(crate) fn load(
        crate_dir: &Path,
        lock_file: Option<&Path>,
        binding_crate_name: Option<&str>,
        use_cargo_lock: bool,
        clean: bool,
        only_target_dependencies: bool,
        existing_metadata_file: Option<&Path>,
    ) -> Result<Cargo, Error> {
        let toml_path = crate_dir.join("Cargo.toml");
        let metadata =
            cargo_metadata::metadata(&toml_path, existing_metadata_file, only_target_dependencies)
                .map_err(|x| Error::CargoMetadata(toml_path.to_str().unwrap().to_owned(), x))?;
        let lock_path = lock_file
            .map(PathBuf::from)
            .unwrap_or_else(|| Path::new(&metadata.workspace_root).join("Cargo.lock"));

        let lock = if use_cargo_lock {
            match cargo_lock::lock(&lock_path) {
                Ok(lock) => Some(lock),
                Err(x) => {
                    warn!("Couldn't load lock file {:?}: {:?}", lock_path, x);
                    None
                }
            }
        } else {
            None
        };

        // Use the specified binding crate name or infer it from the manifest
        let binding_crate_name = match binding_crate_name {
            Some(s) => s.to_owned(),
            None => {
                let manifest = cargo_toml::manifest(&toml_path)
                    .map_err(|x| Error::CargoToml(toml_path.to_str().unwrap().to_owned(), x))?;
                manifest.package.name
            }
        };

        Ok(Cargo {
            manifest_path: toml_path,
            binding_crate_name,
            lock,
            metadata,
            clean,
        })
    }

    pub(crate) fn binding_crate_name(&self) -> &str {
        &self.binding_crate_name
    }

    pub(crate) fn binding_crate_ref(&self) -> PackageRef {
        match self.find_pkg_to_generate_bindings_ref(&self.binding_crate_name) {
            Some(pkg_ref) => pkg_ref,
            None => panic!(
                "Unable to find {} for {:?}",
                self.binding_crate_name, self.manifest_path
            ),
        }
    }

    pub(crate) fn dependencies(&self, package: &PackageRef) -> Vec<(PackageRef, Option<Cfg>)> {
        let lock = match self.lock {
            Some(ref lock) => lock,
            None => return vec![],
        };

        let mut dependencies = None;

        // Find the dependencies listing in the lockfile
        if let Some(ref root) = lock.root {
            // If the version is not on the lockfile then it shouldn't be
            // ambiguous.
            if root.name == package.name
                && package
                    .version
                    .as_ref()
                    .map_or(true, |v| *v == root.version)
            {
                dependencies = root.dependencies.as_ref();
            }
        }
        if dependencies.is_none() {
            if let Some(ref lock_packages) = lock.package {
                for lock_package in lock_packages {
                    if lock_package.name == package.name
                        && package
                            .version
                            .as_ref()
                            .map_or(true, |v| *v == lock_package.version)
                    {
                        dependencies = lock_package.dependencies.as_ref();
                        break;
                    }
                }
            }
        }
        if dependencies.is_none() {
            return vec![];
        }

        dependencies
            .unwrap()
            .iter()
            .map(|dep| {
                let (dep_name, dep_version) = parse_dep_string(dep);

                // If a version was not specified find the only package with the name of the dependency
                let dep_version = dep_version.or_else(|| {
                    let mut versions = self.metadata.packages.iter().filter_map(|package| {
                        if package.name_and_version.name != dep_name {
                            return None;
                        }
                        package.name_and_version.version.as_deref()
                    });

                    // If the iterator contains more items, meaning multiple versions of the same
                    // package are present, warn! amd abort.
                    let version = versions.next();
                    if versions.next().is_none() {
                        version
                    } else {
                        warn!("when looking for a version for package {}, multiple versions where found", dep_name);
                        None
                    }
                });

                // Try to find the cfgs in the Cargo.toml
                let cfg = self
                    .metadata
                    .packages
                    .get(package)
                    .and_then(|meta_package| meta_package.dependencies.get(dep_name))
                    .and_then(Cfg::load_metadata);

                let package_ref = PackageRef {
                    name: dep_name.to_owned(),
                    version: dep_version.map(|v| v.to_owned()),
                };

                (package_ref, cfg)
            })
            .collect()
    }

    /// Finds the package reference for which we want to generate bindings in `cargo metadata`
    /// matching on `package_name` and verifying the manifest path matches so that we don't get a
    /// a dependency with the same name (fix for https://github.com/mozilla/cbindgen/issues/900)
    fn find_pkg_to_generate_bindings_ref(&self, package_name: &str) -> Option<PackageRef> {
        // Keep a list of candidates in case the manifest check fails, so that the old behavior
        // still applies, returning the first package that was found
        let mut candidates = vec![];
        for package in &self.metadata.packages {
            if package.name_and_version.name == package_name {
                // If we are sure it is the right package return it
                if Path::new(package.manifest_path.as_str()) == self.manifest_path {
                    return Some(package.name_and_version.clone());
                }
                // Otherwise note that a package was found
                candidates.push(package.name_and_version.clone());
            }
        }

        // If we could not verify the manifest path but we found candidates return the first one if
        // any, this is the old behavior which did not check for manifest path, kept for backwards
        // compatibility
        candidates.into_iter().next()
    }

    /// Finds the directory for a specified package reference.
    #[allow(unused)]
    pub(crate) fn find_crate_dir(&self, package: &PackageRef) -> Option<PathBuf> {
        self.metadata
            .packages
            .get(package)
            .and_then(|meta_package| {
                Path::new(&meta_package.manifest_path)
                    .parent()
                    .map(|x| x.to_owned())
            })
    }

    /// Finds `src/lib.rs` for a specified package reference.
    pub(crate) fn find_crate_src(&self, package: &PackageRef) -> Option<PathBuf> {
        let kind_lib = String::from("lib");
        let kind_staticlib = String::from("staticlib");
        let kind_rlib = String::from("rlib");
        let kind_cdylib = String::from("cdylib");
        let kind_dylib = String::from("dylib");

        self.metadata
            .packages
            .get(package)
            .and_then(|meta_package| {
                for target in &meta_package.targets {
                    if target.kind.contains(&kind_lib)
                        || target.kind.contains(&kind_staticlib)
                        || target.kind.contains(&kind_rlib)
                        || target.kind.contains(&kind_cdylib)
                        || target.kind.contains(&kind_dylib)
                    {
                        return Some(PathBuf::from(&target.src_path));
                    }
                }
                None
            })
    }

    pub(crate) fn expand_crate(
        &self,
        package: &PackageRef,
        expand_all_features: bool,
        expand_default_features: bool,
        expand_features: &Option<Vec<String>>,
        profile: Profile,
    ) -> Result<String, cargo_expand::Error> {
        cargo_expand::expand(
            &self.manifest_path,
            &package.name,
            package.version.as_deref(),
            self.clean,
            expand_all_features,
            expand_default_features,
            expand_features,
            profile,
        )
    }
}
