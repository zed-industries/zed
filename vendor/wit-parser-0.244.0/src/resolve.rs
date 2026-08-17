use std::cmp::Ordering;
use std::collections::hash_map;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::mem;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use id_arena::{Arena, Id};
use indexmap::{IndexMap, IndexSet};
use semver::Version;
#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::ast::lex::Span;
use crate::ast::{ParsedUsePath, parse_use_path};
#[cfg(feature = "serde")]
use crate::serde_::{serialize_arena, serialize_id_map};
use crate::{
    AstItem, Docs, Error, Function, FunctionKind, Handle, IncludeName, Interface, InterfaceId,
    InterfaceSpan, LiftLowerAbi, ManglingAndAbi, PackageName, PackageNotFoundError, SourceMap,
    Stability, Type, TypeDef, TypeDefKind, TypeId, TypeIdVisitor, TypeOwner, UnresolvedPackage,
    UnresolvedPackageGroup, World, WorldId, WorldItem, WorldKey, WorldSpan,
};

pub use clone::CloneMaps;

mod clone;

/// Representation of a fully resolved set of WIT packages.
///
/// This structure contains a graph of WIT packages and all of their contents
/// merged together into the contained arenas. All items are sorted
/// topologically and everything here is fully resolved, so with a `Resolve` no
/// name lookups are necessary and instead everything is index-based.
///
/// Working with a WIT package requires inserting it into a `Resolve` to ensure
/// that all of its dependencies are satisfied. This will give the full picture
/// of that package's types and such.
///
/// Each item in a `Resolve` has a parent link to trace it back to the original
/// package as necessary.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Resolve {
    /// All known worlds within this `Resolve`.
    ///
    /// Each world points at a `PackageId` which is stored below. No ordering is
    /// guaranteed between this list of worlds.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub worlds: Arena<World>,

    /// All known interfaces within this `Resolve`.
    ///
    /// Each interface points at a `PackageId` which is stored below. No
    /// ordering is guaranteed between this list of interfaces.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub interfaces: Arena<Interface>,

    /// All known types within this `Resolve`.
    ///
    /// Types are topologically sorted such that any type referenced from one
    /// type is guaranteed to be defined previously. Otherwise though these are
    /// not sorted by interface for example.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub types: Arena<TypeDef>,

    /// All known packages within this `Resolve`.
    ///
    /// This list of packages is not sorted. Sorted packages can be queried
    /// through [`Resolve::topological_packages`].
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub packages: Arena<Package>,

    /// A map of package names to the ID of the package with that name.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub package_names: IndexMap<PackageName, PackageId>,

    /// Activated features for this [`Resolve`].
    ///
    /// This set of features is empty by default. This is consulted for
    /// `@unstable` annotations in loaded WIT documents. Any items with
    /// `@unstable` are filtered out unless their feature is present within this
    /// set.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub features: IndexSet<String>,

    /// Activate all features for this [`Resolve`].
    #[cfg_attr(feature = "serde", serde(skip))]
    pub all_features: bool,
}

/// A WIT package within a `Resolve`.
///
/// A package is a collection of interfaces and worlds. Packages additionally
/// have a unique identifier that affects generated components and uniquely
/// identifiers this particular package.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Package {
    /// A unique name corresponding to this package.
    pub name: PackageName,

    /// Documentation associated with this package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,

    /// All interfaces contained in this packaged, keyed by the interface's
    /// name.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id_map"))]
    pub interfaces: IndexMap<String, InterfaceId>,

    /// All worlds contained in this package, keyed by the world's name.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id_map"))]
    pub worlds: IndexMap<String, WorldId>,
}

pub type PackageId = Id<Package>;

/// All the sources used during resolving a directory or path.
#[derive(Clone, Debug)]
pub struct PackageSourceMap {
    sources: Vec<Vec<PathBuf>>,
    package_id_to_source_map_idx: BTreeMap<PackageId, usize>,
}

impl PackageSourceMap {
    fn from_single_source(package_id: PackageId, source: &Path) -> Self {
        Self {
            sources: vec![vec![source.to_path_buf()]],
            package_id_to_source_map_idx: BTreeMap::from([(package_id, 0)]),
        }
    }

    fn from_source_maps(
        source_maps: Vec<SourceMap>,
        package_id_to_source_map_idx: BTreeMap<PackageId, usize>,
    ) -> PackageSourceMap {
        for (package_id, idx) in &package_id_to_source_map_idx {
            if *idx >= source_maps.len() {
                panic!(
                    "Invalid source map index: {}, package id: {:?}, source maps size: {}",
                    idx,
                    package_id,
                    source_maps.len()
                )
            }
        }

        Self {
            sources: source_maps
                .into_iter()
                .map(|source_map| {
                    source_map
                        .source_files()
                        .map(|path| path.to_path_buf())
                        .collect()
                })
                .collect(),
            package_id_to_source_map_idx,
        }
    }

    /// All unique source paths.
    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        // Usually any two source map should not have duplicated source paths,
        // but it can happen, e.g. with using [`Resolve::push_str`] directly.
        // To be sure we use a set for deduplication here.
        self.sources
            .iter()
            .flatten()
            .map(|path_buf| path_buf.as_ref())
            .collect::<IndexSet<&Path>>()
            .into_iter()
    }

    /// Source paths for package
    pub fn package_paths(&self, id: PackageId) -> Option<impl Iterator<Item = &Path>> {
        self.package_id_to_source_map_idx
            .get(&id)
            .map(|&idx| self.sources[idx].iter().map(|path_buf| path_buf.as_ref()))
    }
}

enum ParsedFile {
    #[cfg(feature = "decoding")]
    Package(PackageId),
    Unresolved(UnresolvedPackageGroup),
}

/// Visitor helper for performing topological sort on a group of packages.
fn visit<'a>(
    pkg: &'a UnresolvedPackage,
    pkg_details_map: &'a BTreeMap<PackageName, (UnresolvedPackage, usize)>,
    order: &mut IndexSet<PackageName>,
    visiting: &mut HashSet<&'a PackageName>,
    source_maps: &[SourceMap],
) -> Result<()> {
    if order.contains(&pkg.name) {
        return Ok(());
    }

    match pkg_details_map.get(&pkg.name) {
        Some(pkg_details) => {
            let (_, source_maps_index) = pkg_details;
            source_maps[*source_maps_index].rewrite_error(|| {
                for (i, (dep, _)) in pkg.foreign_deps.iter().enumerate() {
                    let span = pkg.foreign_dep_spans[i];
                    if !visiting.insert(dep) {
                        bail!(Error::new(span, "package depends on itself"));
                    }
                    if let Some(dep) = pkg_details_map.get(dep) {
                        let (dep_pkg, _) = dep;
                        visit(dep_pkg, pkg_details_map, order, visiting, source_maps)?;
                    }
                    assert!(visiting.remove(dep));
                }
                assert!(order.insert(pkg.name.clone()));
                Ok(())
            })
        }
        None => panic!("No pkg_details found for package when doing topological sort"),
    }
}

impl Resolve {
    /// Creates a new [`Resolve`] with no packages/items inside of it.
    pub fn new() -> Resolve {
        Resolve::default()
    }

    /// Parse WIT packages from the input `path`.
    ///
    /// The input `path` can be one of:
    ///
    /// * A directory containing a WIT package with an optional `deps` directory
    ///   for any dependent WIT packages it references.
    /// * A single standalone WIT file.
    /// * A wasm-encoded WIT package as a single file in the wasm binary format.
    /// * A wasm-encoded WIT package as a single file in the wasm text format.
    ///
    /// In all of these cases packages are allowed to depend on previously
    /// inserted packages into this `Resolve`. Resolution for packages is based
    /// on the name of each package and reference.
    ///
    /// This method returns a `PackageId` and additionally a `PackageSourceMap`.
    /// The `PackageId` represent the main package that was parsed. For example if a single WIT
    /// file was specified  this will be the main package found in the file. For a directory this
    /// will be all the main package in the directory itself. The `PackageId` value is useful
    /// to pass to [`Resolve::select_world`] to take a user-specified world in a
    /// conventional fashion and select which to use for bindings generation.
    ///
    /// The returned [`PackageSourceMap`] contains all the sources used during this operation.
    /// This can be useful for systems that want to rebuild or regenerate bindings based on files modified,
    /// or for ones which like to identify the used files for a package.
    ///
    /// More information can also be found at [`Resolve::push_dir`] and
    /// [`Resolve::push_file`].
    pub fn push_path(&mut self, path: impl AsRef<Path>) -> Result<(PackageId, PackageSourceMap)> {
        self._push_path(path.as_ref())
    }

    fn _push_path(&mut self, path: &Path) -> Result<(PackageId, PackageSourceMap)> {
        if path.is_dir() {
            self.push_dir(path).with_context(|| {
                format!(
                    "failed to resolve directory while parsing WIT for path [{}]",
                    path.display()
                )
            })
        } else {
            let id = self.push_file(path)?;
            Ok((id, PackageSourceMap::from_single_source(id, path)))
        }
    }

    fn sort_unresolved_packages(
        &mut self,
        main: UnresolvedPackageGroup,
        deps: Vec<UnresolvedPackageGroup>,
    ) -> Result<(PackageId, PackageSourceMap)> {
        let mut pkg_details_map = BTreeMap::new();
        let mut source_maps = Vec::new();

        let mut insert = |group: UnresolvedPackageGroup| {
            let UnresolvedPackageGroup {
                main,
                nested,
                source_map,
            } = group;
            let i = source_maps.len();
            source_maps.push(source_map);

            for pkg in nested.into_iter().chain([main]) {
                let name = pkg.name.clone();
                let my_span = pkg.package_name_span;
                let (prev_pkg, prev_i) = match pkg_details_map.insert(name.clone(), (pkg, i)) {
                    Some(pair) => pair,
                    None => continue,
                };
                let loc1 = source_maps[i].render_location(my_span);
                let loc2 = source_maps[prev_i].render_location(prev_pkg.package_name_span);
                bail!(
                    "\
package {name} is defined in two different locations:\n\
  * {loc1}\n\
  * {loc2}\n\
                     "
                )
            }
            Ok(())
        };

        let main_name = main.main.name.clone();
        insert(main)?;
        for dep in deps {
            insert(dep)?;
        }

        // Perform a simple topological sort which will bail out on cycles
        // and otherwise determine the order that packages must be added to
        // this `Resolve`.
        let mut order = IndexSet::new();
        let mut visiting = HashSet::new();
        for pkg_details in pkg_details_map.values() {
            let (pkg, _) = pkg_details;
            visit(
                pkg,
                &pkg_details_map,
                &mut order,
                &mut visiting,
                &source_maps,
            )?;
        }

        // Ensure that the final output is topologically sorted. Use a set to ensure that we render
        // the buffers for each `SourceMap` only once, even though multiple packages may reference
        // the same `SourceMap`.
        let mut package_id_to_source_map_idx = BTreeMap::new();
        let mut main_pkg_id = None;
        for name in order {
            let (pkg, source_map_index) = pkg_details_map.remove(&name).unwrap();
            let source_map = &source_maps[source_map_index];
            let is_main = pkg.name == main_name;
            let id = self.push(pkg, source_map)?;
            if is_main {
                assert!(main_pkg_id.is_none());
                main_pkg_id = Some(id);
            }
            package_id_to_source_map_idx.insert(id, source_map_index);
        }

        Ok((
            main_pkg_id.unwrap(),
            PackageSourceMap::from_source_maps(source_maps, package_id_to_source_map_idx),
        ))
    }

    /// Parses the filesystem directory at `path` as a WIT package and returns
    /// a fully resolved [`PackageId`] list as a result.
    ///
    /// The directory itself is parsed with [`UnresolvedPackageGroup::parse_dir`]
    /// and then all packages found are inserted into this `Resolve`. The `path`
    /// specified may have a `deps` subdirectory which is probed automatically
    /// for any other WIT dependencies.
    ///
    /// The `deps` folder may contain:
    ///
    /// * `$path/deps/my-package/*.wit` - a directory that may contain multiple
    ///   WIT files. This is parsed with [`UnresolvedPackageGroup::parse_dir`]
    ///   and then inserted into this [`Resolve`]. Note that cannot recursively
    ///   contain a `deps` directory.
    /// * `$path/deps/my-package.wit` - a single-file WIT package. This is
    ///   parsed with [`Resolve::push_file`] and then added to `self` for
    ///   name resolution.
    /// * `$path/deps/my-package.{wasm,wat}` - a wasm-encoded WIT package either
    ///   in the text for binary format.
    ///
    /// In all cases entries in the `deps` folder are added to `self` first
    /// before adding files found in `path` itself. All WIT packages found are
    /// candidates for name-based resolution that other packages may use.
    ///
    /// This function returns a tuple of two values. The first value is a
    /// [`PackageId`], which represents the main WIT package found within
    /// `path`. This argument is useful for passing to [`Resolve::select_world`]
    /// for choosing something to bindgen with.
    ///
    /// The second value returned is a [`PackageSourceMap`], which contains all the sources
    /// that were parsed during resolving. This can be useful for:
    /// * build systems that want to rebuild bindings whenever one of the files changed
    /// * or other tools, which want to identify the sources for the resolved packages
    pub fn push_dir(&mut self, path: impl AsRef<Path>) -> Result<(PackageId, PackageSourceMap)> {
        self._push_dir(path.as_ref())
    }

    fn _push_dir(&mut self, path: &Path) -> Result<(PackageId, PackageSourceMap)> {
        let top_pkg = UnresolvedPackageGroup::parse_dir(path)
            .with_context(|| format!("failed to parse package: {}", path.display()))?;
        let deps = path.join("deps");
        let deps = self
            .parse_deps_dir(&deps)
            .with_context(|| format!("failed to parse dependency directory: {}", deps.display()))?;

        self.sort_unresolved_packages(top_pkg, deps)
    }

    fn parse_deps_dir(&mut self, path: &Path) -> Result<Vec<UnresolvedPackageGroup>> {
        let mut ret = Vec::new();
        if !path.exists() {
            return Ok(ret);
        }
        let mut entries = path
            .read_dir()
            .and_then(|i| i.collect::<std::io::Result<Vec<_>>>())
            .context("failed to read directory")?;
        entries.sort_by_key(|e| e.file_name());
        for dep in entries {
            let path = dep.path();
            let pkg = if dep.file_type()?.is_dir() || path.metadata()?.is_dir() {
                // If this entry is a directory or a symlink point to a
                // directory then always parse it as an `UnresolvedPackage`
                // since it's intentional to not support recursive `deps`
                // directories.
                UnresolvedPackageGroup::parse_dir(&path)
                    .with_context(|| format!("failed to parse package: {}", path.display()))?
            } else {
                // If this entry is a file then we may want to ignore it but
                // this may also be a standalone WIT file or a `*.wasm` or
                // `*.wat` encoded package.
                let filename = dep.file_name();
                match Path::new(&filename).extension().and_then(|s| s.to_str()) {
                    Some("wit") | Some("wat") | Some("wasm") => match self._push_file(&path)? {
                        #[cfg(feature = "decoding")]
                        ParsedFile::Package(_) => continue,
                        ParsedFile::Unresolved(pkg) => pkg,
                    },

                    // Other files in deps dir are ignored for now to avoid
                    // accidentally including things like `.DS_Store` files in
                    // the call below to `parse_dir`.
                    _ => continue,
                }
            };
            ret.push(pkg);
        }
        Ok(ret)
    }

    /// Parses the contents of `path` from the filesystem and pushes the result
    /// into this `Resolve`.
    ///
    /// The `path` referenced here can be one of:
    ///
    /// * A WIT file. Note that in this case this single WIT file will be the
    ///   entire package and any dependencies it has must already be in `self`.
    /// * A WIT package encoded as WebAssembly, either in text or binary form.
    ///   In this the package and all of its dependencies are automatically
    ///   inserted into `self`.
    ///
    /// In both situations the `PackageId`s of the resulting resolved packages
    /// are returned from this method. The return value is mostly useful in
    /// conjunction with [`Resolve::select_world`].
    pub fn push_file(&mut self, path: impl AsRef<Path>) -> Result<PackageId> {
        match self._push_file(path.as_ref())? {
            #[cfg(feature = "decoding")]
            ParsedFile::Package(id) => Ok(id),
            ParsedFile::Unresolved(pkg) => self.push_group(pkg),
        }
    }

    fn _push_file(&mut self, path: &Path) -> Result<ParsedFile> {
        let contents = std::fs::read(path)
            .with_context(|| format!("failed to read path for WIT [{}]", path.display()))?;

        // If decoding is enabled at compile time then try to see if this is a
        // wasm file.
        #[cfg(feature = "decoding")]
        {
            use crate::decoding::{DecodedWasm, decode};

            #[cfg(feature = "wat")]
            let is_wasm = wat::Detect::from_bytes(&contents).is_wasm();
            #[cfg(not(feature = "wat"))]
            let is_wasm = wasmparser::Parser::is_component(&contents);

            if is_wasm {
                #[cfg(feature = "wat")]
                let contents = wat::parse_bytes(&contents).map_err(|mut e| {
                    e.set_path(path);
                    e
                })?;

                match decode(&contents)? {
                    DecodedWasm::Component(..) => {
                        bail!("found an actual component instead of an encoded WIT package in wasm")
                    }
                    DecodedWasm::WitPackage(resolve, pkg) => {
                        let remap = self.merge(resolve)?;
                        return Ok(ParsedFile::Package(remap.packages[pkg.index()]));
                    }
                }
            }
        }

        // If this wasn't a wasm file then assume it's a WIT file.
        let text = match std::str::from_utf8(&contents) {
            Ok(s) => s,
            Err(_) => bail!("input file is not valid utf-8 [{}]", path.display()),
        };
        let pkgs = UnresolvedPackageGroup::parse(path, text)?;
        Ok(ParsedFile::Unresolved(pkgs))
    }

    /// Appends a new [`UnresolvedPackage`] to this [`Resolve`], creating a
    /// fully resolved package with no dangling references.
    ///
    /// All the dependencies of `unresolved` must already have been loaded
    /// within this `Resolve` via previous calls to `push` or other methods such
    /// as [`Resolve::push_path`].
    ///
    /// Any dependency resolution error or otherwise world-elaboration error
    /// will be returned here, if successful a package identifier is returned
    /// which corresponds to the package that was just inserted.
    pub fn push(
        &mut self,
        unresolved: UnresolvedPackage,
        source_map: &SourceMap,
    ) -> Result<PackageId> {
        let ret = source_map.rewrite_error(|| Remap::default().append(self, unresolved));
        if ret.is_ok() {
            #[cfg(debug_assertions)]
            self.assert_valid();
        }
        ret
    }

    /// Appends new [`UnresolvedPackageGroup`] to this [`Resolve`], creating a
    /// fully resolved package with no dangling references.
    ///
    /// Any dependency resolution error or otherwise world-elaboration error
    /// will be returned here, if successful a package identifier is returned
    /// which corresponds to the package that was just inserted.
    ///
    /// The returned [`PackageId`]s are listed in topologically sorted order.
    pub fn push_group(&mut self, unresolved_group: UnresolvedPackageGroup) -> Result<PackageId> {
        let (pkg_id, _) = self.sort_unresolved_packages(unresolved_group, Vec::new())?;
        Ok(pkg_id)
    }

    /// Convenience method for combining [`UnresolvedPackageGroup::parse`] and
    /// [`Resolve::push_group`].
    ///
    /// The `path` provided is used for error messages but otherwise is not
    /// read. This method does not touch the filesystem. The `contents` provided
    /// are the contents of a WIT package.
    pub fn push_str(&mut self, path: impl AsRef<Path>, contents: &str) -> Result<PackageId> {
        self.push_group(UnresolvedPackageGroup::parse(path.as_ref(), contents)?)
    }

    pub fn all_bits_valid(&self, ty: &Type) -> bool {
        match ty {
            Type::U8
            | Type::S8
            | Type::U16
            | Type::S16
            | Type::U32
            | Type::S32
            | Type::U64
            | Type::S64
            | Type::F32
            | Type::F64 => true,

            Type::Bool | Type::Char | Type::String | Type::ErrorContext => false,

            Type::Id(id) => match &self.types[*id].kind {
                TypeDefKind::List(_)
                | TypeDefKind::Map(_, _)
                | TypeDefKind::Variant(_)
                | TypeDefKind::Enum(_)
                | TypeDefKind::Option(_)
                | TypeDefKind::Result(_)
                | TypeDefKind::Future(_)
                | TypeDefKind::Stream(_) => false,
                TypeDefKind::Type(t) | TypeDefKind::FixedSizeList(t, ..) => self.all_bits_valid(t),

                TypeDefKind::Handle(h) => match h {
                    crate::Handle::Own(_) => true,
                    crate::Handle::Borrow(_) => true,
                },

                TypeDefKind::Resource => false,
                TypeDefKind::Record(r) => r.fields.iter().all(|f| self.all_bits_valid(&f.ty)),
                TypeDefKind::Tuple(t) => t.types.iter().all(|t| self.all_bits_valid(t)),

                // FIXME: this could perhaps be `true` for multiples-of-32 but
                // seems better to probably leave this as unconditionally
                // `false` for now, may want to reconsider later?
                TypeDefKind::Flags(_) => false,

                TypeDefKind::Unknown => unreachable!(),
            },
        }
    }

    /// Merges all the contents of a different `Resolve` into this one. The
    /// `Remap` structure returned provides a mapping from all old indices to
    /// new indices
    ///
    /// This operation can fail if `resolve` disagrees with `self` about the
    /// packages being inserted. Otherwise though this will additionally attempt
    /// to "union" packages found in `resolve` with those found in `self`.
    /// Unioning packages is keyed on the name/url of packages for those with
    /// URLs present. If found then it's assumed that both `Resolve` instances
    /// were originally created from the same contents and are two views
    /// of the same package.
    pub fn merge(&mut self, resolve: Resolve) -> Result<Remap> {
        log::trace!(
            "merging {} packages into {} packages",
            resolve.packages.len(),
            self.packages.len()
        );

        let mut map = MergeMap::new(&resolve, &self);
        map.build()?;
        let MergeMap {
            package_map,
            interface_map,
            type_map,
            world_map,
            interfaces_to_add,
            worlds_to_add,
            ..
        } = map;

        // With a set of maps from ids in `resolve` to ids in `self` the next
        // operation is to start moving over items and building a `Remap` to
        // update ids.
        //
        // Each component field of `resolve` is moved into `self` so long as
        // its ID is not within one of the maps above. If it's present in a map
        // above then that means the item is already present in `self` so a new
        // one need not be added. If it's not present in a map that means it's
        // not present in `self` so it must be added to an arena.
        //
        // When adding an item to an arena one of the `remap.update_*` methods
        // is additionally called to update all identifiers from pointers within
        // `resolve` to becoming pointers within `self`.
        //
        // Altogether this should weave all the missing items in `self` from
        // `resolve` into one structure while updating all identifiers to
        // be local within `self`.

        let mut remap = Remap::default();
        let Resolve {
            types,
            worlds,
            interfaces,
            packages,
            package_names,
            features: _,
            ..
        } = resolve;

        let mut moved_types = Vec::new();
        for (id, mut ty) in types {
            let new_id = match type_map.get(&id).copied() {
                Some(id) => {
                    update_stability(&ty.stability, &mut self.types[id].stability)?;
                    id
                }
                None => {
                    log::debug!("moving type {:?}", ty.name);
                    moved_types.push(id);
                    remap.update_typedef(self, &mut ty, None)?;
                    self.types.alloc(ty)
                }
            };
            assert_eq!(remap.types.len(), id.index());
            remap.types.push(Some(new_id));
        }

        let mut moved_interfaces = Vec::new();
        for (id, mut iface) in interfaces {
            let new_id = match interface_map.get(&id).copied() {
                Some(id) => {
                    update_stability(&iface.stability, &mut self.interfaces[id].stability)?;
                    id
                }
                None => {
                    log::debug!("moving interface {:?}", iface.name);
                    moved_interfaces.push(id);
                    remap.update_interface(self, &mut iface, None)?;
                    self.interfaces.alloc(iface)
                }
            };
            assert_eq!(remap.interfaces.len(), id.index());
            remap.interfaces.push(Some(new_id));
        }

        let mut moved_worlds = Vec::new();
        for (id, mut world) in worlds {
            let new_id = match world_map.get(&id).copied() {
                Some(world_id) => {
                    update_stability(&world.stability, &mut self.worlds[world_id].stability)?;
                    for from_import in world.imports.iter() {
                        Resolve::update_world_imports_stability(
                            from_import,
                            &mut self.worlds[world_id].imports,
                            &interface_map,
                        )?;
                    }
                    for from_export in world.exports.iter() {
                        Resolve::update_world_imports_stability(
                            from_export,
                            &mut self.worlds[world_id].exports,
                            &interface_map,
                        )?;
                    }
                    world_id
                }
                None => {
                    log::debug!("moving world {}", world.name);
                    moved_worlds.push(id);
                    let mut update = |map: &mut IndexMap<WorldKey, WorldItem>| -> Result<_> {
                        for (mut name, mut item) in mem::take(map) {
                            remap.update_world_key(&mut name, None)?;
                            match &mut item {
                                WorldItem::Function(f) => remap.update_function(self, f, None)?,
                                WorldItem::Interface { id, .. } => {
                                    *id = remap.map_interface(*id, None)?
                                }
                                WorldItem::Type(i) => *i = remap.map_type(*i, None)?,
                            }
                            map.insert(name, item);
                        }
                        Ok(())
                    };
                    update(&mut world.imports)?;
                    update(&mut world.exports)?;
                    self.worlds.alloc(world)
                }
            };
            assert_eq!(remap.worlds.len(), id.index());
            remap.worlds.push(Some(new_id));
        }

        for (id, mut pkg) in packages {
            let new_id = match package_map.get(&id).copied() {
                Some(id) => id,
                None => {
                    for (_, id) in pkg.interfaces.iter_mut() {
                        *id = remap.map_interface(*id, None)?;
                    }
                    for (_, id) in pkg.worlds.iter_mut() {
                        *id = remap.map_world(*id, None)?;
                    }
                    self.packages.alloc(pkg)
                }
            };
            assert_eq!(remap.packages.len(), id.index());
            remap.packages.push(new_id);
        }

        for (name, id) in package_names {
            let id = remap.packages[id.index()];
            if let Some(prev) = self.package_names.insert(name, id) {
                assert_eq!(prev, id);
            }
        }

        // Fixup all "parent" links now.
        //
        // Note that this is only done for items that are actually moved from
        // `resolve` into `self`, which is tracked by the various `moved_*`
        // lists built incrementally above. The ids in the `moved_*` lists
        // are ids within `resolve`, so they're translated through `remap` to
        // ids within `self`.
        for id in moved_worlds {
            let id = remap.map_world(id, None)?;
            if let Some(pkg) = self.worlds[id].package.as_mut() {
                *pkg = remap.packages[pkg.index()];
            }
        }
        for id in moved_interfaces {
            let id = remap.map_interface(id, None)?;
            if let Some(pkg) = self.interfaces[id].package.as_mut() {
                *pkg = remap.packages[pkg.index()];
            }
        }
        for id in moved_types {
            let id = remap.map_type(id, None)?;
            match &mut self.types[id].owner {
                TypeOwner::Interface(id) => *id = remap.map_interface(*id, None)?,
                TypeOwner::World(id) => *id = remap.map_world(*id, None)?,
                TypeOwner::None => {}
            }
        }

        // And finally process items that were present in `resolve` but were
        // not present in `self`. This is only done for merged packages as
        // documents may be added to `self.documents` but wouldn't otherwise be
        // present in the `documents` field of the corresponding package.
        for (name, pkg, iface) in interfaces_to_add {
            let prev = self.packages[pkg]
                .interfaces
                .insert(name, remap.map_interface(iface, None)?);
            assert!(prev.is_none());
        }
        for (name, pkg, world) in worlds_to_add {
            let prev = self.packages[pkg]
                .worlds
                .insert(name, remap.map_world(world, None)?);
            assert!(prev.is_none());
        }

        log::trace!("now have {} packages", self.packages.len());

        #[cfg(debug_assertions)]
        self.assert_valid();
        Ok(remap)
    }

    fn update_world_imports_stability(
        from_item: (&WorldKey, &WorldItem),
        into_items: &mut IndexMap<WorldKey, WorldItem>,
        interface_map: &HashMap<Id<Interface>, Id<Interface>>,
    ) -> Result<()> {
        match from_item.0 {
            WorldKey::Name(_) => {
                // No stability info to update here, only updating import/include stability
                Ok(())
            }
            key @ WorldKey::Interface(_) => {
                let new_key = MergeMap::map_name(key, interface_map);
                if let Some(into) = into_items.get_mut(&new_key) {
                    match (from_item.1, into) {
                        (
                            WorldItem::Interface {
                                id: aid,
                                stability: astability,
                            },
                            WorldItem::Interface {
                                id: bid,
                                stability: bstability,
                            },
                        ) => {
                            let aid = interface_map.get(aid).copied().unwrap_or(*aid);
                            assert_eq!(aid, *bid);
                            update_stability(astability, bstability)?;
                            Ok(())
                        }
                        _ => unreachable!(),
                    }
                } else {
                    // we've already matched all the imports/exports by the time we are calling this
                    // so this is unreachable since we should always find the item
                    unreachable!()
                }
            }
        }
    }

    /// Merges the world `from` into the world `into`.
    ///
    /// This will attempt to merge one world into another, unioning all of its
    /// imports and exports together. This is an operation performed by
    /// `wit-component`, for example where two different worlds from two
    /// different libraries were linked into the same core wasm file and are
    /// producing a singular world that will be the final component's
    /// interface.
    ///
    /// During the merge operation, some of the types and/or interfaces in
    /// `from` might need to be cloned so that backreferences point to `into`
    /// instead of `from`.  Any such clones will be added to `clone_maps`.
    ///
    /// This operation can fail if the imports/exports overlap.
    pub fn merge_worlds(
        &mut self,
        from: WorldId,
        into: WorldId,
        clone_maps: &mut CloneMaps,
    ) -> Result<()> {
        let mut new_imports = Vec::new();
        let mut new_exports = Vec::new();

        let from_world = &self.worlds[from];
        let into_world = &self.worlds[into];

        log::trace!("merging {} into {}", from_world.name, into_world.name);

        // First walk over all the imports of `from` world and figure out what
        // to do with them.
        //
        // If the same item exists in `from` and `into` then merge it together
        // below with `merge_world_item` which basically asserts they're the
        // same. Otherwise queue up a new import since if `from` has more
        // imports than `into` then it's fine to add new imports.
        for (name, from_import) in from_world.imports.iter() {
            let name_str = self.name_world_key(name);
            match into_world.imports.get(name) {
                Some(into_import) => {
                    log::trace!("info/from shared import on `{name_str}`");
                    self.merge_world_item(from_import, into_import)
                        .with_context(|| format!("failed to merge world import {name_str}"))?;
                }
                None => {
                    log::trace!("new import: `{name_str}`");
                    new_imports.push((name.clone(), from_import.clone()));
                }
            }
        }

        // Build a set of interfaces which are required to be imported because
        // of `into`'s exports. This set is then used below during
        // `ensure_can_add_world_export`.
        //
        // This is the set of interfaces which exports depend on that are
        // themselves not exports.
        let mut must_be_imported = HashMap::new();
        for (key, export) in into_world.exports.iter() {
            for dep in self.world_item_direct_deps(export) {
                if into_world.exports.contains_key(&WorldKey::Interface(dep)) {
                    continue;
                }
                self.foreach_interface_dep(dep, &mut |id| {
                    must_be_imported.insert(id, key.clone());
                });
            }
        }

        // Next walk over exports of `from` and process these similarly to
        // imports.
        for (name, from_export) in from_world.exports.iter() {
            let name_str = self.name_world_key(name);
            match into_world.exports.get(name) {
                Some(into_export) => {
                    log::trace!("info/from shared export on `{name_str}`");
                    self.merge_world_item(from_export, into_export)
                        .with_context(|| format!("failed to merge world export {name_str}"))?;
                }
                None => {
                    log::trace!("new export `{name_str}`");
                    // See comments in `ensure_can_add_world_export` for why
                    // this is slightly different than imports.
                    self.ensure_can_add_world_export(
                        into_world,
                        name,
                        from_export,
                        &must_be_imported,
                    )
                    .with_context(|| {
                        format!("failed to add export `{}`", self.name_world_key(name))
                    })?;
                    new_exports.push((name.clone(), from_export.clone()));
                }
            }
        }

        // For all the new imports and exports they may need to be "cloned" to
        // be able to belong to the new world. For example:
        //
        // * Anonymous interfaces have a `package` field which points to the
        //   package of the containing world, but `from` and `into` may not be
        //   in the same package.
        //
        // * Type imports have an `owner` field that point to `from`, but they
        //   now need to point to `into` instead.
        //
        // Cloning is no trivial task, however, so cloning is delegated to a
        // submodule to perform a "deep" clone and copy items into new arena
        // entries as necessary.
        let mut cloner = clone::Cloner::new(self, TypeOwner::World(from), TypeOwner::World(into));
        cloner.register_world_type_overlap(from, into);
        for (name, item) in new_imports.iter_mut().chain(&mut new_exports) {
            cloner.world_item(name, item, clone_maps);
        }

        clone_maps.types.extend(cloner.types);

        // Insert any new imports and new exports found first.
        let into_world = &mut self.worlds[into];
        for (name, import) in new_imports {
            let prev = into_world.imports.insert(name, import);
            assert!(prev.is_none());
        }
        for (name, export) in new_exports {
            let prev = into_world.exports.insert(name, export);
            assert!(prev.is_none());
        }

        #[cfg(debug_assertions)]
        self.assert_valid();
        Ok(())
    }

    fn merge_world_item(&self, from: &WorldItem, into: &WorldItem) -> Result<()> {
        let mut map = MergeMap::new(self, self);
        match (from, into) {
            (WorldItem::Interface { id: from, .. }, WorldItem::Interface { id: into, .. }) => {
                // If these imports are the same that can happen, for
                // example, when both worlds to `import foo:bar/baz;`. That
                // foreign interface will point to the same interface within
                // `Resolve`.
                if from == into {
                    return Ok(());
                }

                // .. otherwise this MUST be a case of
                // `import foo: interface { ... }`. If `from != into` but
                // both `from` and `into` have the same name then the
                // `WorldKey::Interface` case is ruled out as otherwise
                // they'd have different names.
                //
                // In the case of an anonymous interface all we can do is
                // ensure that the interfaces both match, so use `MergeMap`
                // for that.
                map.build_interface(*from, *into)
                    .context("failed to merge interfaces")?;
            }

            // Like `WorldKey::Name` interfaces for functions and types the
            // structure is asserted to be the same.
            (WorldItem::Function(from), WorldItem::Function(into)) => {
                map.build_function(from, into)
                    .context("failed to merge functions")?;
            }
            (WorldItem::Type(from), WorldItem::Type(into)) => {
                map.build_type_id(*from, *into)
                    .context("failed to merge types")?;
            }

            // Kind-level mismatches are caught here.
            (WorldItem::Interface { .. }, _)
            | (WorldItem::Function { .. }, _)
            | (WorldItem::Type { .. }, _) => {
                bail!("different kinds of items");
            }
        }
        assert!(map.interfaces_to_add.is_empty());
        assert!(map.worlds_to_add.is_empty());
        Ok(())
    }

    /// This method ensures that the world export of `name` and `item` can be
    /// added to the world `into` without changing the meaning of `into`.
    ///
    /// All dependencies of world exports must either be:
    ///
    /// * An export themselves
    /// * An import with all transitive dependencies of the import also imported
    ///
    /// It's not possible to depend on an import which then also depends on an
    /// export at some point, for example. This method ensures that if `name`
    /// and `item` are added that this property is upheld.
    fn ensure_can_add_world_export(
        &self,
        into: &World,
        name: &WorldKey,
        item: &WorldItem,
        must_be_imported: &HashMap<InterfaceId, WorldKey>,
    ) -> Result<()> {
        assert!(!into.exports.contains_key(name));
        let name = self.name_world_key(name);

        // First make sure that all of this item's dependencies are either
        // exported or the entire chain of imports rooted at that dependency are
        // all imported.
        for dep in self.world_item_direct_deps(item) {
            if into.exports.contains_key(&WorldKey::Interface(dep)) {
                continue;
            }
            self.ensure_not_exported(into, dep)
                .with_context(|| format!("failed validating export of `{name}`"))?;
        }

        // Second make sure that this item, if it's an interface, will not alter
        // the meaning of the preexisting world by ensuring that it's not in the
        // set of "must be imported" items.
        if let WorldItem::Interface { id, .. } = item {
            if let Some(export) = must_be_imported.get(&id) {
                let export_name = self.name_world_key(export);
                bail!(
                    "export `{export_name}` depends on `{name}` \
                     previously as an import which will change meaning \
                     if `{name}` is added as an export"
                );
            }
        }

        Ok(())
    }

    fn ensure_not_exported(&self, world: &World, id: InterfaceId) -> Result<()> {
        let key = WorldKey::Interface(id);
        let name = self.name_world_key(&key);
        if world.exports.contains_key(&key) {
            bail!(
                "world exports `{name}` but it's also transitively used by an \
                     import \
                   which means that this is not valid"
            )
        }
        for dep in self.interface_direct_deps(id) {
            self.ensure_not_exported(world, dep)
                .with_context(|| format!("failed validating transitive import dep `{name}`"))?;
        }
        Ok(())
    }

    /// Returns an iterator of all the direct interface dependencies of this
    /// `item`.
    ///
    /// Note that this doesn't include transitive dependencies, that must be
    /// followed manually.
    fn world_item_direct_deps(&self, item: &WorldItem) -> impl Iterator<Item = InterfaceId> + '_ {
        let mut interface = None;
        let mut ty = None;
        match item {
            WorldItem::Function(_) => {}
            WorldItem::Type(id) => ty = Some(*id),
            WorldItem::Interface { id, .. } => interface = Some(*id),
        }

        interface
            .into_iter()
            .flat_map(move |id| self.interface_direct_deps(id))
            .chain(ty.and_then(|t| self.type_interface_dep(t)))
    }

    /// Invokes `f` with `id` and all transitive interface dependencies of `id`.
    ///
    /// Note that `f` may be called with the same id multiple times.
    fn foreach_interface_dep(&self, id: InterfaceId, f: &mut dyn FnMut(InterfaceId)) {
        self._foreach_interface_dep(id, f, &mut HashSet::new())
    }

    // Internal detail of `foreach_interface_dep` which uses a hash map to prune
    // the visit tree to ensure that this doesn't visit an exponential number of
    // interfaces.
    fn _foreach_interface_dep(
        &self,
        id: InterfaceId,
        f: &mut dyn FnMut(InterfaceId),
        visited: &mut HashSet<InterfaceId>,
    ) {
        if !visited.insert(id) {
            return;
        }
        f(id);
        for dep in self.interface_direct_deps(id) {
            self._foreach_interface_dep(dep, f, visited);
        }
    }

    /// Returns the ID of the specified `interface`.
    ///
    /// Returns `None` for unnamed interfaces.
    pub fn id_of(&self, interface: InterfaceId) -> Option<String> {
        let interface = &self.interfaces[interface];
        Some(self.id_of_name(interface.package.unwrap(), interface.name.as_ref()?))
    }

    /// Returns the "canonicalized interface name" of `interface`.
    ///
    /// Returns `None` for unnamed interfaces. See `BuildTargets.md` in the
    /// upstream component model repository for more information about this.
    pub fn canonicalized_id_of(&self, interface: InterfaceId) -> Option<String> {
        let interface = &self.interfaces[interface];
        Some(self.canonicalized_id_of_name(interface.package.unwrap(), interface.name.as_ref()?))
    }

    /// Convert a world to an "importized" version where the world is updated
    /// in-place to reflect what it would look like to be imported.
    ///
    /// This is a transformation which is used as part of the process of
    /// importing a component today. For example when a component depends on
    /// another component this is useful for generating WIT which can be use to
    /// represent the component being imported. The general idea is that this
    /// function will update the `world_id` specified such it imports the
    /// functionality that it previously exported. The world will be left with
    /// no exports.
    ///
    /// This world is then suitable for merging into other worlds or generating
    /// bindings in a context that is importing the original world. This
    /// is intended to be used as part of language tooling when depending on
    /// other components.
    pub fn importize(&mut self, world_id: WorldId, out_world_name: Option<String>) -> Result<()> {
        // Rename the world to avoid having it get confused with the original
        // name of the world. Add `-importized` to it for now. Precisely how
        // this new world is created may want to be updated over time if this
        // becomes problematic.
        let world = &mut self.worlds[world_id];
        let pkg = &mut self.packages[world.package.unwrap()];
        pkg.worlds.shift_remove(&world.name);
        if let Some(name) = out_world_name {
            world.name = name.clone();
            pkg.worlds.insert(name, world_id);
        } else {
            world.name.push_str("-importized");
            pkg.worlds.insert(world.name.clone(), world_id);
        }

        // Trim all non-type definitions from imports. Types can be used by
        // exported functions, for example, so they're preserved.
        world.imports.retain(|_, item| match item {
            WorldItem::Type(_) => true,
            _ => false,
        });

        for (name, export) in mem::take(&mut world.exports) {
            match (name.clone(), world.imports.insert(name, export)) {
                // no previous item? this insertion was ok
                (_, None) => {}

                // cannot overwrite an import with an export
                (WorldKey::Name(name), Some(_)) => {
                    bail!("world export `{name}` conflicts with import of same name");
                }

                // Exports already don't overlap each other and the only imports
                // preserved above were types so this shouldn't be reachable.
                (WorldKey::Interface(_), _) => unreachable!(),
            }
        }

        // Fill out any missing transitive interface imports by elaborating this
        // world which does that for us.
        self.elaborate_world(world_id)?;

        #[cfg(debug_assertions)]
        self.assert_valid();
        Ok(())
    }

    /// Returns the ID of the specified `name` within the `pkg`.
    pub fn id_of_name(&self, pkg: PackageId, name: &str) -> String {
        let package = &self.packages[pkg];
        let mut base = String::new();
        base.push_str(&package.name.namespace);
        base.push_str(":");
        base.push_str(&package.name.name);
        base.push_str("/");
        base.push_str(name);
        if let Some(version) = &package.name.version {
            base.push_str(&format!("@{version}"));
        }
        base
    }

    /// Returns the "canonicalized interface name" of the specified `name`
    /// within the `pkg`.
    ///
    /// See `BuildTargets.md` in the upstream component model repository for
    /// more information about this.
    pub fn canonicalized_id_of_name(&self, pkg: PackageId, name: &str) -> String {
        let package = &self.packages[pkg];
        let mut base = String::new();
        base.push_str(&package.name.namespace);
        base.push_str(":");
        base.push_str(&package.name.name);
        base.push_str("/");
        base.push_str(name);
        if let Some(version) = &package.name.version {
            base.push_str("@");
            let string = PackageName::version_compat_track_string(version);
            base.push_str(&string);
        }
        base
    }

    /// Selects a world from among the packages in a `Resolve`.
    ///
    /// A `Resolve` may have many packages, each with many worlds. Many WIT
    /// tools need a specific world to operate on. This function choses a
    /// world, failing if the choice is ambiguous.
    ///
    /// `main_packages` provides the package IDs returned by
    /// [`push_path`](Resolve::push_path), [`push_dir`](Resolve::push_dir),
    /// [`push_file`](Resolve::push_file), [`push_group`](Resolve::push_group),
    /// and [`push_str`](Resolve::push_str), which are the "main packages",
    /// as distinguished from any packages nested inside them.
    ///
    /// `world` is a world name such as from a `--world` command-line option or
    /// a `world:` macro parameter. `world` can be:
    ///
    /// * A kebab-name of a world, for example `"the-world"`. It is resolved
    ///   within the "main package", if there is exactly one.
    ///
    /// * An ID-based form of a world, for example `"wasi:http/proxy"`. Note
    ///   that a version does not need to be specified in this string if
    ///   there's only one package of the same name and it has a version. In
    ///   this situation the version can be omitted.
    ///
    /// * `None`. If there's exactly one "main package" and it contains exactly
    ///   one world, that world is chosen.
    ///
    /// If successful, the chosen `WorldId` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use wit_parser::Resolve;
    ///
    /// fn main() -> Result<()> {
    ///     let mut resolve = Resolve::default();
    ///
    ///     // If there's a single package and only one world, that world is
    ///     // the obvious choice.
    ///     let wit1 = resolve.push_str(
    ///         "./my-test.wit",
    ///         r#"
    ///             package example:wit1;
    ///
    ///             world foo {
    ///                 // ...
    ///             }
    ///         "#,
    ///     )?;
    ///     assert!(resolve.select_world(&[wit1], None).is_ok());
    ///
    ///     // If there are multiple packages, we need to be told which package
    ///     // to use, either by a "main package" or by a fully-qualified name.
    ///     let wit2 = resolve.push_str(
    ///         "./my-test.wit",
    ///         r#"
    ///             package example:wit2;
    ///
    ///             world foo { /* ... */ }
    ///         "#,
    ///     )?;
    ///     assert!(resolve.select_world(&[wit1, wit2], None).is_err());
    ///     assert!(resolve.select_world(&[wit1, wit2], Some("foo")).is_err());
    ///     // Fix: use fully-qualified names.
    ///     assert!(resolve.select_world(&[wit1, wit2], Some("example:wit1/foo")).is_ok());
    ///     assert!(resolve.select_world(&[wit1, wit2], Some("example:wit2/foo")).is_ok());
    ///
    ///     // If a package has multiple worlds, then we can't guess the world
    ///     // even if we know the package.
    ///     let wit3 = resolve.push_str(
    ///         "./my-test.wit",
    ///         r#"
    ///             package example:wit3;
    ///
    ///             world foo { /* ... */ }
    ///
    ///             world bar { /* ... */ }
    ///         "#,
    ///     )?;
    ///     assert!(resolve.select_world(&[wit3], None).is_err());
    ///     // Fix: pick between "foo" and "bar" here.
    ///     assert!(resolve.select_world(&[wit3], Some("foo")).is_ok());
    ///
    ///     // When selecting with a version it's ok to drop the version when
    ///     // there's only a single copy of that package in `Resolve`.
    ///     let wit5_1 = resolve.push_str(
    ///         "./my-test.wit",
    ///         r#"
    ///             package example:wit5@1.0.0;
    ///
    ///             world foo { /* ... */ }
    ///         "#,
    ///     )?;
    ///     assert!(resolve.select_world(&[wit5_1], Some("foo")).is_ok());
    ///     assert!(resolve.select_world(&[wit5_1], Some("example:wit5/foo")).is_ok());
    ///
    ///     // However when a single package has multiple versions in a resolve
    ///     // it's required to specify the version to select which one.
    ///     let wit5_2 = resolve.push_str(
    ///         "./my-test.wit",
    ///         r#"
    ///             package example:wit5@2.0.0;
    ///
    ///             world foo { /* ... */ }
    ///         "#,
    ///     )?;
    ///     assert!(resolve.select_world(&[wit5_1, wit5_2], Some("example:wit5/foo")).is_err());
    ///     // Fix: Pass explicit versions.
    ///     assert!(resolve.select_world(&[wit5_1, wit5_2], Some("example:wit5/foo@1.0.0")).is_ok());
    ///     assert!(resolve.select_world(&[wit5_1, wit5_2], Some("example:wit5/foo@2.0.0")).is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn select_world(
        &self,
        main_packages: &[PackageId],
        world: Option<&str>,
    ) -> Result<WorldId> {
        // Determine if `world` is a kebab-name or an ID.
        let world_path = match world {
            Some(world) => Some(
                parse_use_path(world)
                    .with_context(|| format!("failed to parse world specifier `{world}`"))?,
            ),
            None => None,
        };

        match world_path {
            // We have a world path. If needed, pick a package to resolve it in.
            Some(world_path) => {
                let (pkg, world_name) = match (main_packages, world_path) {
                    // We have no main packages; fail.
                    ([], _) => bail!("No main packages defined"),

                    // We have exactly one main package.
                    ([main_package], ParsedUsePath::Name(name)) => (*main_package, name),

                    // We have more than one main package; fail.
                    (_, ParsedUsePath::Name(_name)) => {
                        bail!(
                            "There are multiple main packages; a world must be explicitly chosen:{}",
                            self.worlds
                                .iter()
                                .map(|world| format!(
                                    "\n  {}",
                                    self.id_of_name(world.1.package.unwrap(), &world.1.name)
                                ))
                                .collect::<String>()
                        )
                    }

                    // The world name is fully-qualified.
                    (_, ParsedUsePath::Package(pkg, world_name)) => {
                        let pkg = match self.package_names.get(&pkg) {
                            Some(pkg) => *pkg,
                            None => {
                                let mut candidates =
                                    self.package_names.iter().filter(|(name, _)| {
                                        pkg.version.is_none()
                                            && pkg.name == name.name
                                            && pkg.namespace == name.namespace
                                            && name.version.is_some()
                                    });
                                let candidate = candidates.next();
                                if let Some((c2, _)) = candidates.next() {
                                    let (c1, _) = candidate.unwrap();
                                    bail!(
                                        "package name `{pkg}` is available at both \
                                    versions {} and {} but which is not specified",
                                        c1.version.as_ref().unwrap(),
                                        c2.version.as_ref().unwrap(),
                                    );
                                }
                                match candidate {
                                    Some((_, id)) => *id,
                                    None => bail!("unknown package `{pkg}`"),
                                }
                            }
                        };
                        (pkg, world_name.to_string())
                    }
                };

                // Now that we've picked the package, resolve the world name.
                let pkg = &self.packages[pkg];
                pkg.worlds.get(&world_name).copied().ok_or_else(|| {
                    anyhow!("World `{world_name}` not found in package `{}`", pkg.name)
                })
            }

            // With no specified `world`, try to find a single obvious world.
            None => match main_packages {
                [] => bail!("No main packages defined"),

                // Check for exactly one main package with exactly one world.
                [main_package] => {
                    let pkg = &self.packages[*main_package];
                    match pkg.worlds.len() {
                        0 => bail!("The main package `{}` contains no worlds", pkg.name),
                        1 => Ok(pkg.worlds[0]),
                        _ => bail!(
                            "There are multiple worlds in `{}`; one must be explicitly chosen:{}",
                            pkg.name,
                            pkg.worlds
                                .values()
                                .map(|world| format!(
                                    "\n  {}",
                                    self.id_of_name(*main_package, &self.worlds[*world].name)
                                ))
                                .collect::<String>()
                        ),
                    }
                }

                // Multiple main packages and no world name; fail.
                _ => {
                    bail!(
                        "There are multiple main packages; a world must be explicitly chosen:{}",
                        self.worlds
                            .iter()
                            .map(|world| format!(
                                "\n  {}",
                                self.id_of_name(world.1.package.unwrap(), &world.1.name)
                            ))
                            .collect::<String>()
                    )
                }
            },
        }
    }

    /// Assigns a human readable name to the `WorldKey` specified.
    pub fn name_world_key(&self, key: &WorldKey) -> String {
        match key {
            WorldKey::Name(s) => s.to_string(),
            WorldKey::Interface(i) => self.id_of(*i).expect("unexpected anonymous interface"),
        }
    }

    /// Same as [`Resolve::name_world_key`] except that `WorldKey::Interfaces`
    /// uses [`Resolve::canonicalized_id_of`].
    pub fn name_canonicalized_world_key(&self, key: &WorldKey) -> String {
        match key {
            WorldKey::Name(s) => s.to_string(),
            WorldKey::Interface(i) => self
                .canonicalized_id_of(*i)
                .expect("unexpected anonymous interface"),
        }
    }

    /// Returns the interface that `id` uses a type from, if it uses a type from
    /// a different interface than `id` is defined within.
    ///
    /// If `id` is not a use-of-a-type or it's using a type in the same
    /// interface then `None` is returned.
    pub fn type_interface_dep(&self, id: TypeId) -> Option<InterfaceId> {
        let ty = &self.types[id];
        let dep = match ty.kind {
            TypeDefKind::Type(Type::Id(id)) => id,
            _ => return None,
        };
        let other = &self.types[dep];
        if ty.owner == other.owner {
            None
        } else {
            match other.owner {
                TypeOwner::Interface(id) => Some(id),
                _ => unreachable!(),
            }
        }
    }

    /// Returns an iterator of all interfaces that the interface `id` depends
    /// on.
    ///
    /// Interfaces may depend on others for type information to resolve type
    /// imports.
    ///
    /// Note that the returned iterator may yield the same interface as a
    /// dependency multiple times. Additionally only direct dependencies of `id`
    /// are yielded, not transitive dependencies.
    pub fn interface_direct_deps(&self, id: InterfaceId) -> impl Iterator<Item = InterfaceId> + '_ {
        self.interfaces[id]
            .types
            .iter()
            .filter_map(move |(_name, ty)| self.type_interface_dep(*ty))
    }

    /// Returns an iterator of all packages that the package `id` depends
    /// on.
    ///
    /// Packages may depend on others for type information to resolve type
    /// imports or interfaces to resolve worlds.
    ///
    /// Note that the returned iterator may yield the same package as a
    /// dependency multiple times. Additionally only direct dependencies of `id`
    /// are yielded, not transitive dependencies.
    pub fn package_direct_deps(&self, id: PackageId) -> impl Iterator<Item = PackageId> + '_ {
        let pkg = &self.packages[id];

        pkg.interfaces
            .iter()
            .flat_map(move |(_name, id)| self.interface_direct_deps(*id))
            .chain(pkg.worlds.iter().flat_map(move |(_name, id)| {
                let world = &self.worlds[*id];
                world
                    .imports
                    .iter()
                    .chain(world.exports.iter())
                    .filter_map(move |(_name, item)| match item {
                        WorldItem::Interface { id, .. } => Some(*id),
                        WorldItem::Function(_) => None,
                        WorldItem::Type(t) => self.type_interface_dep(*t),
                    })
            }))
            .filter_map(move |iface_id| {
                let pkg = self.interfaces[iface_id].package?;
                if pkg == id { None } else { Some(pkg) }
            })
    }

    /// Returns a topological ordering of packages contained in this `Resolve`.
    ///
    /// This returns a list of `PackageId` such that when visited in order it's
    /// guaranteed that all dependencies will have been defined by prior items
    /// in the list.
    pub fn topological_packages(&self) -> Vec<PackageId> {
        let mut pushed = vec![false; self.packages.len()];
        let mut order = Vec::new();
        for (id, _) in self.packages.iter() {
            self.build_topological_package_ordering(id, &mut pushed, &mut order);
        }
        order
    }

    fn build_topological_package_ordering(
        &self,
        id: PackageId,
        pushed: &mut Vec<bool>,
        order: &mut Vec<PackageId>,
    ) {
        if pushed[id.index()] {
            return;
        }
        for dep in self.package_direct_deps(id) {
            self.build_topological_package_ordering(dep, pushed, order);
        }
        order.push(id);
        pushed[id.index()] = true;
    }

    #[doc(hidden)]
    pub fn assert_valid(&self) {
        let mut package_interfaces = Vec::new();
        let mut package_worlds = Vec::new();
        for (id, pkg) in self.packages.iter() {
            let mut interfaces = HashSet::new();
            for (name, iface) in pkg.interfaces.iter() {
                assert!(interfaces.insert(*iface));
                let iface = &self.interfaces[*iface];
                assert_eq!(name, iface.name.as_ref().unwrap());
                assert_eq!(iface.package.unwrap(), id);
            }
            package_interfaces.push(pkg.interfaces.values().copied().collect::<HashSet<_>>());
            let mut worlds = HashSet::new();
            for (name, world) in pkg.worlds.iter() {
                assert!(worlds.insert(*world));
                assert_eq!(
                    pkg.worlds.get_key_value(name),
                    Some((name, world)),
                    "`MutableKeys` impl may have been used to change a key's hash or equality"
                );
                let world = &self.worlds[*world];
                assert_eq!(*name, world.name);
                assert_eq!(world.package.unwrap(), id);
            }
            package_worlds.push(pkg.worlds.values().copied().collect::<HashSet<_>>());
        }

        let mut interface_types = Vec::new();
        for (id, iface) in self.interfaces.iter() {
            assert!(self.packages.get(iface.package.unwrap()).is_some());
            if iface.name.is_some() {
                assert!(package_interfaces[iface.package.unwrap().index()].contains(&id));
            }

            for (name, ty) in iface.types.iter() {
                let ty = &self.types[*ty];
                assert_eq!(ty.name.as_ref(), Some(name));
                assert_eq!(ty.owner, TypeOwner::Interface(id));
            }
            interface_types.push(iface.types.values().copied().collect::<HashSet<_>>());
            for (name, f) in iface.functions.iter() {
                assert_eq!(*name, f.name);
            }
        }

        let mut world_types = Vec::new();
        for (id, world) in self.worlds.iter() {
            log::debug!("validating world {}", &world.name);
            if let Some(package) = world.package {
                assert!(self.packages.get(package).is_some());
                assert!(package_worlds[package.index()].contains(&id));
            }
            assert!(world.includes.is_empty());

            let mut types = HashSet::new();
            for (name, item) in world.imports.iter().chain(world.exports.iter()) {
                log::debug!("validating world item: {}", self.name_world_key(name));
                match item {
                    WorldItem::Interface { id, .. } => {
                        // anonymous interfaces must belong to the same package
                        // as the world's package.
                        if matches!(name, WorldKey::Name(_)) {
                            assert_eq!(self.interfaces[*id].package, world.package);
                        }
                    }
                    WorldItem::Function(f) => {
                        assert!(!matches!(name, WorldKey::Interface(_)));
                        assert_eq!(f.name, name.clone().unwrap_name());
                    }
                    WorldItem::Type(ty) => {
                        assert!(!matches!(name, WorldKey::Interface(_)));
                        assert!(types.insert(*ty));
                        let ty = &self.types[*ty];
                        assert_eq!(ty.name, Some(name.clone().unwrap_name()));
                        assert_eq!(ty.owner, TypeOwner::World(id));
                    }
                }
            }
            self.assert_world_elaborated(world);
            world_types.push(types);
        }

        for (ty_id, ty) in self.types.iter() {
            match ty.owner {
                TypeOwner::Interface(id) => {
                    assert!(self.interfaces.get(id).is_some());
                    assert!(interface_types[id.index()].contains(&ty_id));
                }
                TypeOwner::World(id) => {
                    assert!(self.worlds.get(id).is_some());
                    assert!(world_types[id.index()].contains(&ty_id));
                }
                TypeOwner::None => {}
            }
        }

        self.assert_topologically_sorted();
    }

    fn assert_topologically_sorted(&self) {
        let mut positions = IndexMap::new();
        for id in self.topological_packages() {
            let pkg = &self.packages[id];
            log::debug!("pkg {}", pkg.name);
            let prev = positions.insert(Some(id), IndexSet::new());
            assert!(prev.is_none());
        }
        positions.insert(None, IndexSet::new());

        for (id, iface) in self.interfaces.iter() {
            log::debug!("iface {:?}", iface.name);
            let ok = positions.get_mut(&iface.package).unwrap().insert(id);
            assert!(ok);
        }

        for (_, world) in self.worlds.iter() {
            log::debug!("world {:?}", world.name);

            let my_package = world.package;
            let my_package_pos = positions.get_index_of(&my_package).unwrap();

            for (_, item) in world.imports.iter().chain(&world.exports) {
                let id = match item {
                    WorldItem::Interface { id, .. } => *id,
                    _ => continue,
                };
                let other_package = self.interfaces[id].package;
                let other_package_pos = positions.get_index_of(&other_package).unwrap();

                assert!(other_package_pos <= my_package_pos);
            }
        }

        for (_id, ty) in self.types.iter() {
            log::debug!("type {:?} {:?}", ty.name, ty.owner);
            let other_id = match ty.kind {
                TypeDefKind::Type(Type::Id(ty)) => ty,
                _ => continue,
            };
            let other = &self.types[other_id];
            if ty.kind == other.kind {
                continue;
            }
            let my_interface = match ty.owner {
                TypeOwner::Interface(id) => id,
                _ => continue,
            };
            let other_interface = match other.owner {
                TypeOwner::Interface(id) => id,
                _ => continue,
            };

            let my_package = self.interfaces[my_interface].package;
            let other_package = self.interfaces[other_interface].package;
            let my_package_pos = positions.get_index_of(&my_package).unwrap();
            let other_package_pos = positions.get_index_of(&other_package).unwrap();

            if my_package_pos == other_package_pos {
                let interfaces = &positions[&my_package];
                let my_interface_pos = interfaces.get_index_of(&my_interface).unwrap();
                let other_interface_pos = interfaces.get_index_of(&other_interface).unwrap();
                assert!(other_interface_pos <= my_interface_pos);
            } else {
                assert!(other_package_pos < my_package_pos);
            }
        }
    }

    fn assert_world_elaborated(&self, world: &World) {
        for (key, item) in world.imports.iter() {
            log::debug!(
                "asserting elaborated world import {}",
                self.name_world_key(key)
            );
            match item {
                WorldItem::Type(t) => self.assert_world_imports_type_deps(world, key, *t),

                // All types referred to must be imported.
                WorldItem::Function(f) => self.assert_world_function_imports_types(world, key, f),

                // All direct dependencies of this interface must be imported.
                WorldItem::Interface { id, .. } => {
                    for dep in self.interface_direct_deps(*id) {
                        assert!(
                            world.imports.contains_key(&WorldKey::Interface(dep)),
                            "world import of {} is missing transitive dep of {}",
                            self.name_world_key(key),
                            self.id_of(dep).unwrap(),
                        );
                    }
                }
            }
        }
        for (key, item) in world.exports.iter() {
            log::debug!(
                "asserting elaborated world export {}",
                self.name_world_key(key)
            );
            match item {
                // Types referred to by this function must be imported.
                WorldItem::Function(f) => self.assert_world_function_imports_types(world, key, f),

                // Dependencies of exported interfaces must also be exported, or
                // if imported then that entire chain of imports must be
                // imported and not exported.
                WorldItem::Interface { id, .. } => {
                    for dep in self.interface_direct_deps(*id) {
                        let dep_key = WorldKey::Interface(dep);
                        if world.exports.contains_key(&dep_key) {
                            continue;
                        }
                        self.foreach_interface_dep(dep, &mut |dep| {
                            let dep_key = WorldKey::Interface(dep);
                            assert!(
                                world.imports.contains_key(&dep_key),
                                "world should import {} (required by {})",
                                self.name_world_key(&dep_key),
                                self.name_world_key(key),
                            );
                            assert!(
                                !world.exports.contains_key(&dep_key),
                                "world should not export {} (required by {})",
                                self.name_world_key(&dep_key),
                                self.name_world_key(key),
                            );
                        });
                    }
                }

                // exported types not allowed at this time
                WorldItem::Type(_) => unreachable!(),
            }
        }
    }

    fn assert_world_imports_type_deps(&self, world: &World, key: &WorldKey, ty: TypeId) {
        // If this is a `use` statement then the referred-to interface must be
        // imported into this world.
        let ty = &self.types[ty];
        if let TypeDefKind::Type(Type::Id(other)) = ty.kind {
            if let TypeOwner::Interface(id) = self.types[other].owner {
                let key = WorldKey::Interface(id);
                assert!(world.imports.contains_key(&key));
                return;
            }
        }

        // ... otherwise any named type that this type refers to, one level
        // deep, must be imported into this world under that name.

        let mut visitor = MyVisit(self, Vec::new());
        visitor.visit_type_def(self, ty);
        for ty in visitor.1 {
            let ty = &self.types[ty];
            let Some(name) = ty.name.clone() else {
                continue;
            };
            let dep_key = WorldKey::Name(name);
            assert!(
                world.imports.contains_key(&dep_key),
                "world import `{}` should also force an import of `{}`",
                self.name_world_key(key),
                self.name_world_key(&dep_key),
            );
        }

        struct MyVisit<'a>(&'a Resolve, Vec<TypeId>);

        impl TypeIdVisitor for MyVisit<'_> {
            fn before_visit_type_id(&mut self, id: TypeId) -> bool {
                self.1.push(id);
                // recurse into unnamed types to look at all named types
                self.0.types[id].name.is_none()
            }
        }
    }

    /// This asserts that all types referred to by `func` are imported into
    /// `world` under `WorldKey::Name`. Note that this is only applicable to
    /// named type
    fn assert_world_function_imports_types(&self, world: &World, key: &WorldKey, func: &Function) {
        for ty in func
            .parameter_and_result_types()
            .chain(func.kind.resource().map(Type::Id))
        {
            let Type::Id(id) = ty else {
                continue;
            };
            self.assert_world_imports_type_deps(world, key, id);
        }
    }

    /// Returns whether the `stability` annotation contained within `pkg_id`
    /// should be included or not.
    ///
    /// The `span` provided here is an optional span pointing to the item that
    /// is annotated with `stability`.
    ///
    /// Returns `Ok(true)` if the item is included, or `Ok(false)` if the item
    /// is not.
    ///
    /// # Errors
    ///
    /// Returns an error if the `pkg_id` isn't annotated with sufficient version
    /// information to have a `stability` annotation. For example if `pkg_id`
    /// has no version listed then an error will be returned if `stability`
    /// mentions a version.
    fn include_stability(
        &self,
        stability: &Stability,
        pkg_id: &PackageId,
        span: Option<Span>,
    ) -> Result<bool> {
        let err = |msg: String| match span {
            Some(span) => Error::new(span, msg).into(),
            None => anyhow::Error::msg(msg),
        };
        Ok(match stability {
            Stability::Unknown => true,
            // NOTE: deprecations are intentionally omitted -- an existing
            // `@since` takes precedence over `@deprecated`
            Stability::Stable { since, .. } => {
                let Some(p) = self.packages.get(*pkg_id) else {
                    // We can't check much without a package (possibly dealing
                    // with an item in an `UnresolvedPackage`), @since version &
                    // deprecations can't be checked because there's no package
                    // version to compare to.
                    //
                    // Feature requirements on stabilized features are ignored
                    // in resolved packages, so we do the same here.
                    return Ok(true);
                };

                // Use of feature gating with version specifiers inside a
                // package that is not versioned is not allowed
                let package_version = p.name.version.as_ref().ok_or_else(|| {
                    err(format!(
                        "package [{}] contains a feature gate with a version \
                         specifier, so it must have a version",
                        p.name
                    ))
                })?;

                // If the version on the feature gate is:
                // - released, then we can include it
                // - unreleased, then we must check the feature (if present)
                if since > package_version {
                    return Err(err(format!(
                        "feature gate cannot reference unreleased version \
                        {since} of package [{}] (current version {package_version})",
                        p.name
                    )));
                }

                true
            }
            Stability::Unstable { feature, .. } => {
                self.features.contains(feature) || self.all_features
            }
        })
    }

    /// Convenience wrapper around `include_stability` specialized for types
    /// with a more targeted error message.
    fn include_type(&self, ty: &TypeDef, pkgid: PackageId, span: Span) -> Result<bool> {
        self.include_stability(&ty.stability, &pkgid, Some(span))
            .with_context(|| {
                format!(
                    "failed to process feature gate for type [{}] in package [{}]",
                    ty.name.as_ref().map(String::as_str).unwrap_or("<unknown>"),
                    self.packages[pkgid].name,
                )
            })
    }

    /// Performs the "elaboration process" necessary for the `world_id`
    /// specified to ensure that all of its transitive imports are listed.
    ///
    /// This function will take the unordered lists of the specified world's
    /// imports and exports and "elaborate" them to ensure that they're
    /// topographically sorted where all transitively required interfaces by
    /// imports, or exports, are listed. This will additionally validate that
    /// the exports are all valid and present, specifically with the restriction
    /// noted on `elaborate_world_exports`.
    ///
    /// The world is mutated in-place in this `Resolve`.
    fn elaborate_world(&mut self, world_id: WorldId) -> Result<()> {
        // First process all imports. This is easier than exports since the only
        // requirement here is that all interfaces need to be added with a
        // topological order between them.
        let mut new_imports = IndexMap::new();
        let world = &self.worlds[world_id];

        // Sort the imports by "class" to ensure that this matches the order
        // that items are printed and that items are in topological order.
        //
        // When printing worlds in WIT:
        //
        // * interfaces come first
        // * types are next
        //   * type imports are first
        //   * type definitions are next
        //   * resource definitions have methods printed inline
        // * freestanding functions are last
        //
        // This reflects the topological order between items where types
        // can refer to imports and functions can refer to these types. Ordering
        // within a single class (e.g. imports depending on each other, types
        // referring to each other) is already preserved by other passes in this
        // file and general AST resolution. That means that a stable sort here
        // can be used to ensure that each class is in the right location
        // relative to the others.
        //
        // Overall this ensures that round-trips of WIT through wasm should
        // always produce the same result.
        let sort_key = |resolve: &Resolve, item: &WorldItem| match item {
            WorldItem::Interface { .. } => 0,
            WorldItem::Type(ty) => {
                let ty = &resolve.types[*ty];
                match ty.kind {
                    TypeDefKind::Type(Type::Id(t)) if resolve.types[t].owner != ty.owner => 1,
                    _ => 2,
                }
            }
            WorldItem::Function(f) => {
                if f.kind.resource().is_none() {
                    3
                } else {
                    4
                }
            }
        };

        // Sort world items when we start to elaborate the world to start with a
        // topological view of items.
        let mut world_imports = world.imports.iter().collect::<Vec<_>>();
        world_imports.sort_by_key(|(_name, import)| sort_key(self, import));
        for (name, item) in world_imports {
            match item {
                // Interfaces get their dependencies added first followed by the
                // interface itself.
                WorldItem::Interface { id, stability } => {
                    self.elaborate_world_import(&mut new_imports, name.clone(), *id, &stability);
                }

                // Functions are added as-is since their dependence on types in
                // the world should already be satisfied.
                WorldItem::Function(_) => {
                    let prev = new_imports.insert(name.clone(), item.clone());
                    assert!(prev.is_none());
                }

                // Types may depend on an interface, in which case a (possibly)
                // recursive addition of that interface happens here. Afterwards
                // the type itself can be added safely.
                WorldItem::Type(id) => {
                    if let Some(dep) = self.type_interface_dep(*id) {
                        self.elaborate_world_import(
                            &mut new_imports,
                            WorldKey::Interface(dep),
                            dep,
                            &self.types[*id].stability,
                        );
                    }
                    let prev = new_imports.insert(name.clone(), item.clone());
                    assert!(prev.is_none());
                }
            }
        }

        // Exports are trickier than imports, notably to uphold the invariant
        // required by `elaborate_world_exports`. To do this the exports are
        // partitioned into interfaces/functions. All functions are added to
        // the new exports list during this loop but interfaces are all deferred
        // to be handled in the `elaborate_world_exports` function.
        let mut new_exports = IndexMap::new();
        let mut export_interfaces = IndexMap::new();
        for (name, item) in world.exports.iter() {
            match item {
                WorldItem::Interface { id, stability } => {
                    let prev = export_interfaces.insert(*id, (name.clone(), stability));
                    assert!(prev.is_none());
                }
                WorldItem::Function(_) => {
                    let prev = new_exports.insert(name.clone(), item.clone());
                    assert!(prev.is_none());
                }
                WorldItem::Type(_) => unreachable!(),
            }
        }

        self.elaborate_world_exports(&export_interfaces, &mut new_imports, &mut new_exports)?;

        // In addition to sorting at the start of elaboration also sort here at
        // the end of elaboration to handle types being interspersed with
        // interfaces as they're found.
        new_imports.sort_by_cached_key(|_name, import| sort_key(self, import));

        // And with all that done the world is updated in-place with
        // imports/exports.
        log::trace!("imports = {new_imports:?}");
        log::trace!("exports = {new_exports:?}");
        let world = &mut self.worlds[world_id];
        world.imports = new_imports;
        world.exports = new_exports;

        Ok(())
    }

    fn elaborate_world_import(
        &self,
        imports: &mut IndexMap<WorldKey, WorldItem>,
        key: WorldKey,
        id: InterfaceId,
        stability: &Stability,
    ) {
        if imports.contains_key(&key) {
            return;
        }
        for dep in self.interface_direct_deps(id) {
            self.elaborate_world_import(imports, WorldKey::Interface(dep), dep, stability);
        }
        let prev = imports.insert(
            key,
            WorldItem::Interface {
                id,
                stability: stability.clone(),
            },
        );
        assert!(prev.is_none());
    }

    /// This function adds all of the interfaces in `export_interfaces` to the
    /// list of exports of the `world` specified.
    ///
    /// This method is more involved than adding imports because it is fallible.
    /// Chiefly what can happen is that the dependencies of all exports must be
    /// satisfied by other exports or imports, but not both. For example given a
    /// situation such as:
    ///
    /// ```wit
    /// interface a {
    ///     type t = u32
    /// }
    /// interface b {
    ///     use a.{t}
    /// }
    /// interface c {
    ///     use a.{t}
    ///     use b.{t as t2}
    /// }
    /// ```
    ///
    /// where `c` depends on `b` and `a` where `b` depends on `a`, then the
    /// purpose of this method is to reject this world:
    ///
    /// ```wit
    /// world foo {
    ///     export a
    ///     export c
    /// }
    /// ```
    ///
    /// The reasoning here is unfortunately subtle and is additionally the
    /// subject of WebAssembly/component-model#208. Effectively the `c`
    /// interface depends on `b`, but it's not listed explicitly as an import,
    /// so it's then implicitly added as an import. This then transitively
    /// depends on `a` so it's also added as an import. At this point though `c`
    /// also depends on `a`, and it's also exported, so naively it should depend
    /// on the export and not implicitly add an import. This means though that
    /// `c` has access to two copies of `a`, one imported and one exported. This
    /// is not valid, especially in the face of resource types.
    ///
    /// Overall this method is tasked with rejecting the above world by walking
    /// over all the exports and adding their dependencies. Each dependency is
    /// recorded with whether it's required to be imported, and then if an
    /// export is added for something that's required to be an error then the
    /// operation fails.
    fn elaborate_world_exports(
        &self,
        export_interfaces: &IndexMap<InterfaceId, (WorldKey, &Stability)>,
        imports: &mut IndexMap<WorldKey, WorldItem>,
        exports: &mut IndexMap<WorldKey, WorldItem>,
    ) -> Result<()> {
        let mut required_imports = HashSet::new();
        for (id, (key, stability)) in export_interfaces.iter() {
            let name = self.name_world_key(&key);
            let ok = add_world_export(
                self,
                imports,
                exports,
                export_interfaces,
                &mut required_imports,
                *id,
                key,
                true,
                stability,
            );
            if !ok {
                bail!(
                    // FIXME: this is not a great error message and basically no
                    // one will know what to do when it gets printed. Improving
                    // this error message, however, is a chunk of work that may
                    // not be best spent doing this at this time, so I'm writing
                    // this comment instead.
                    //
                    // More-or-less what should happen here is that a "path"
                    // from this interface to the conflicting interface should
                    // be printed. It should be explained why an import is being
                    // injected, why that's conflicting with an export, and
                    // ideally with a suggestion of "add this interface to the
                    // export list to fix this error".
                    //
                    // That's a lot of info that's not easy to get at without
                    // more refactoring, so it's left to a future date in the
                    // hopes that most folks won't actually run into this for
                    // the time being.
                    InvalidTransitiveDependency(name),
                );
            }
        }
        return Ok(());

        fn add_world_export(
            resolve: &Resolve,
            imports: &mut IndexMap<WorldKey, WorldItem>,
            exports: &mut IndexMap<WorldKey, WorldItem>,
            export_interfaces: &IndexMap<InterfaceId, (WorldKey, &Stability)>,
            required_imports: &mut HashSet<InterfaceId>,
            id: InterfaceId,
            key: &WorldKey,
            add_export: bool,
            stability: &Stability,
        ) -> bool {
            if exports.contains_key(key) {
                if add_export {
                    return true;
                } else {
                    return false;
                }
            }
            // If this is an import and it's already in the `required_imports`
            // set then we can skip it as we've already visited this interface.
            if !add_export && required_imports.contains(&id) {
                return true;
            }
            let ok = resolve.interface_direct_deps(id).all(|dep| {
                let key = WorldKey::Interface(dep);
                let add_export = add_export && export_interfaces.contains_key(&dep);
                add_world_export(
                    resolve,
                    imports,
                    exports,
                    export_interfaces,
                    required_imports,
                    dep,
                    &key,
                    add_export,
                    stability,
                )
            });
            if !ok {
                return false;
            }
            let item = WorldItem::Interface {
                id,
                stability: stability.clone(),
            };
            if add_export {
                if required_imports.contains(&id) {
                    return false;
                }
                exports.insert(key.clone(), item);
            } else {
                required_imports.insert(id);
                imports.insert(key.clone(), item);
            }
            true
        }
    }

    /// Remove duplicate imports from a world if they import from the same
    /// interface with semver-compatible versions.
    ///
    /// This will merge duplicate interfaces present at multiple versions in
    /// both a world by selecting the larger version of the two interfaces. This
    /// requires that the interfaces are indeed semver-compatible and it means
    /// that some imports might be removed and replaced. Note that this is only
    /// done within a single semver track, for example the world imports 0.2.0
    /// and 0.2.1 then the result afterwards will be that it imports
    /// 0.2.1. If, however, 0.3.0 where imported then the final result would
    /// import both 0.2.0 and 0.3.0.
    pub fn merge_world_imports_based_on_semver(&mut self, world_id: WorldId) -> Result<()> {
        let world = &self.worlds[world_id];

        // The first pass here is to build a map of "semver tracks" where they
        // key is per-interface and the value is the maximal version found in
        // that semver-compatible-track plus the interface which is the maximal
        // version.
        //
        // At the same time a `to_remove` set is maintained to remember what
        // interfaces are being removed from `from` and `into`. All of
        // `to_remove` are placed with a known other version.
        let mut semver_tracks = HashMap::new();
        let mut to_remove = HashSet::new();
        for (key, _) in world.imports.iter() {
            let iface_id = match key {
                WorldKey::Interface(id) => *id,
                WorldKey::Name(_) => continue,
            };
            let (track, version) = match self.semver_track(iface_id) {
                Some(track) => track,
                None => continue,
            };
            log::debug!(
                "{} is on track {}/{}",
                self.id_of(iface_id).unwrap(),
                track.0,
                track.1,
            );
            match semver_tracks.entry(track.clone()) {
                hash_map::Entry::Vacant(e) => {
                    e.insert((version, iface_id));
                }
                hash_map::Entry::Occupied(mut e) => match version.cmp(&e.get().0) {
                    Ordering::Greater => {
                        to_remove.insert(e.get().1);
                        e.insert((version, iface_id));
                    }
                    Ordering::Equal => {}
                    Ordering::Less => {
                        to_remove.insert(iface_id);
                    }
                },
            }
        }

        // Build a map of "this interface is replaced with this interface" using
        // the results of the loop above.
        let mut replacements = HashMap::new();
        for id in to_remove {
            let (track, _) = self.semver_track(id).unwrap();
            let (_, latest) = semver_tracks[&track];
            let prev = replacements.insert(id, latest);
            assert!(prev.is_none());
        }

        // Validate that `merge_world_item` succeeds for merging all removed
        // interfaces with their replacement. This is a double-check that the
        // semver version is actually correct and all items present in the old
        // interface are in the new.
        for (to_replace, replace_with) in replacements.iter() {
            self.merge_world_item(
                &WorldItem::Interface {
                    id: *to_replace,
                    stability: Default::default(),
                },
                &WorldItem::Interface {
                    id: *replace_with,
                    stability: Default::default(),
                },
            )
            .with_context(|| {
                let old_name = self.id_of(*to_replace).unwrap();
                let new_name = self.id_of(*replace_with).unwrap();
                format!(
                    "failed to upgrade `{old_name}` to `{new_name}`, was \
                     this semver-compatible update not semver compatible?"
                )
            })?;
        }

        for (to_replace, replace_with) in replacements.iter() {
            log::debug!(
                "REPLACE {} => {}",
                self.id_of(*to_replace).unwrap(),
                self.id_of(*replace_with).unwrap(),
            );
        }

        // Finally perform the actual transformation of the imports/exports.
        // Here all imports are removed if they're replaced and otherwise all
        // imports have their dependencies updated, possibly transitively, to
        // point to the new interfaces in `replacements`.
        //
        // Afterwards exports are additionally updated, but only their
        // dependencies on imports which were remapped. Exports themselves are
        // not deduplicated and/or removed.
        for (key, item) in mem::take(&mut self.worlds[world_id].imports) {
            if let WorldItem::Interface { id, .. } = item {
                if replacements.contains_key(&id) {
                    continue;
                }
            }

            self.update_interface_deps_of_world_item(&item, &replacements);

            let prev = self.worlds[world_id].imports.insert(key, item);
            assert!(prev.is_none());
        }
        for (key, item) in mem::take(&mut self.worlds[world_id].exports) {
            self.update_interface_deps_of_world_item(&item, &replacements);
            let prev = self.worlds[world_id].exports.insert(key, item);
            assert!(prev.is_none());
        }

        // Run through `elaborate_world` to reorder imports as appropriate and
        // fill anything back in if it's actually required by exports. For now
        // this doesn't tamper with exports at all. Also note that this is
        // applied to all worlds in this `Resolve` because interfaces were
        // modified directly.
        let ids = self.worlds.iter().map(|(id, _)| id).collect::<Vec<_>>();
        for world_id in ids {
            self.elaborate_world(world_id).with_context(|| {
                let name = &self.worlds[world_id].name;
                format!(
                    "failed to elaborate world `{name}` after deduplicating imports \
                     based on semver"
                )
            })?;
        }

        #[cfg(debug_assertions)]
        self.assert_valid();

        Ok(())
    }

    fn update_interface_deps_of_world_item(
        &mut self,
        item: &WorldItem,
        replacements: &HashMap<InterfaceId, InterfaceId>,
    ) {
        match *item {
            WorldItem::Type(t) => self.update_interface_dep_of_type(t, &replacements),
            WorldItem::Interface { id, .. } => {
                let types = self.interfaces[id]
                    .types
                    .values()
                    .copied()
                    .collect::<Vec<_>>();
                for ty in types {
                    self.update_interface_dep_of_type(ty, &replacements);
                }
            }
            WorldItem::Function(_) => {}
        }
    }

    /// Returns the "semver track" of an interface plus the interface's version.
    ///
    /// This function returns `None` if the interface `id` has a package without
    /// a version. If the version is present, however, the first element of the
    /// tuple returned is a "semver track" for the specific interface. The
    /// version listed in `PackageName` will be modified so all
    /// semver-compatible versions are listed the same way.
    ///
    /// The second element in the returned tuple is this interface's package's
    /// version.
    fn semver_track(&self, id: InterfaceId) -> Option<((PackageName, String), &Version)> {
        let iface = &self.interfaces[id];
        let pkg = &self.packages[iface.package?];
        let version = pkg.name.version.as_ref()?;
        let mut name = pkg.name.clone();
        name.version = Some(PackageName::version_compat_track(version));
        Some(((name, iface.name.clone()?), version))
    }

    /// If `ty` is a definition where it's a `use` from another interface, then
    /// change what interface it's using from according to the pairs in the
    /// `replacements` map.
    fn update_interface_dep_of_type(
        &mut self,
        ty: TypeId,
        replacements: &HashMap<InterfaceId, InterfaceId>,
    ) {
        let to_replace = match self.type_interface_dep(ty) {
            Some(id) => id,
            None => return,
        };
        let replace_with = match replacements.get(&to_replace) {
            Some(id) => id,
            None => return,
        };
        let dep = match self.types[ty].kind {
            TypeDefKind::Type(Type::Id(id)) => id,
            _ => return,
        };
        let name = self.types[dep].name.as_ref().unwrap();
        // Note the infallible name indexing happening here. This should be
        // previously validated with `merge_world_item` to succeed.
        let replacement_id = self.interfaces[*replace_with].types[name];
        self.types[ty].kind = TypeDefKind::Type(Type::Id(replacement_id));
    }

    /// Returns the core wasm module/field names for the specified `import`.
    ///
    /// This function will return the core wasm module/field that can be used to
    /// use `import` with the name `mangling` scheme specified as well. This can
    /// be useful for bindings generators, for example, and these names are
    /// recognized by `wit-component` and `wasm-tools component new`.
    pub fn wasm_import_name(
        &self,
        mangling: ManglingAndAbi,
        import: WasmImport<'_>,
    ) -> (String, String) {
        match mangling {
            ManglingAndAbi::Standard32 => match import {
                WasmImport::Func { interface, func } => {
                    let module = match interface {
                        Some(key) => format!("cm32p2|{}", self.name_canonicalized_world_key(key)),
                        None => format!("cm32p2"),
                    };
                    (module, func.name.clone())
                }
                WasmImport::ResourceIntrinsic {
                    interface,
                    resource,
                    intrinsic,
                } => {
                    let name = self.types[resource].name.as_ref().unwrap();
                    let (prefix, name) = match intrinsic {
                        ResourceIntrinsic::ImportedDrop => ("", format!("{name}_drop")),
                        ResourceIntrinsic::ExportedDrop => ("_ex_", format!("{name}_drop")),
                        ResourceIntrinsic::ExportedNew => ("_ex_", format!("{name}_new")),
                        ResourceIntrinsic::ExportedRep => ("_ex_", format!("{name}_rep")),
                    };
                    let module = match interface {
                        Some(key) => {
                            format!("cm32p2|{prefix}{}", self.name_canonicalized_world_key(key))
                        }
                        None => {
                            assert_eq!(prefix, "");
                            format!("cm32p2")
                        }
                    };
                    (module, name)
                }
            },
            ManglingAndAbi::Legacy(abi) => match import {
                WasmImport::Func { interface, func } => {
                    let module = match interface {
                        Some(key) => self.name_world_key(key),
                        None => format!("$root"),
                    };
                    (module, format!("{}{}", abi.import_prefix(), func.name))
                }
                WasmImport::ResourceIntrinsic {
                    interface,
                    resource,
                    intrinsic,
                } => {
                    let name = self.types[resource].name.as_ref().unwrap();
                    let (prefix, name) = match intrinsic {
                        ResourceIntrinsic::ImportedDrop => ("", format!("[resource-drop]{name}")),
                        ResourceIntrinsic::ExportedDrop => {
                            ("[export]", format!("[resource-drop]{name}"))
                        }
                        ResourceIntrinsic::ExportedNew => {
                            ("[export]", format!("[resource-new]{name}"))
                        }
                        ResourceIntrinsic::ExportedRep => {
                            ("[export]", format!("[resource-rep]{name}"))
                        }
                    };
                    let module = match interface {
                        Some(key) => format!("{prefix}{}", self.name_world_key(key)),
                        None => {
                            assert_eq!(prefix, "");
                            format!("$root")
                        }
                    };
                    (module, format!("{}{name}", abi.import_prefix()))
                }
            },
        }
    }

    /// Returns the core wasm export name for the specified `export`.
    ///
    /// This is the same as [`Resolve::wasm_import_name`], except for exports.
    pub fn wasm_export_name(&self, mangling: ManglingAndAbi, export: WasmExport<'_>) -> String {
        match mangling {
            ManglingAndAbi::Standard32 => match export {
                WasmExport::Func {
                    interface,
                    func,
                    kind,
                } => {
                    let mut name = String::from("cm32p2|");
                    if let Some(interface) = interface {
                        let s = self.name_canonicalized_world_key(interface);
                        name.push_str(&s);
                    }
                    name.push_str("|");
                    name.push_str(&func.name);
                    match kind {
                        WasmExportKind::Normal => {}
                        WasmExportKind::PostReturn => name.push_str("_post"),
                        WasmExportKind::Callback => todo!(
                            "not yet supported: \
                             async callback functions using standard name mangling"
                        ),
                    }
                    name
                }
                WasmExport::ResourceDtor {
                    interface,
                    resource,
                } => {
                    let name = self.types[resource].name.as_ref().unwrap();
                    let interface = self.name_canonicalized_world_key(interface);
                    format!("cm32p2|{interface}|{name}_dtor")
                }
                WasmExport::Memory => "cm32p2_memory".to_string(),
                WasmExport::Initialize => "cm32p2_initialize".to_string(),
                WasmExport::Realloc => "cm32p2_realloc".to_string(),
            },
            ManglingAndAbi::Legacy(abi) => match export {
                WasmExport::Func {
                    interface,
                    func,
                    kind,
                } => {
                    let mut name = abi.export_prefix().to_string();
                    match kind {
                        WasmExportKind::Normal => {}
                        WasmExportKind::PostReturn => name.push_str("cabi_post_"),
                        WasmExportKind::Callback => {
                            assert!(matches!(abi, LiftLowerAbi::AsyncCallback));
                            name = format!("[callback]{name}")
                        }
                    }
                    if let Some(interface) = interface {
                        let s = self.name_world_key(interface);
                        name.push_str(&s);
                        name.push_str("#");
                    }
                    name.push_str(&func.name);
                    name
                }
                WasmExport::ResourceDtor {
                    interface,
                    resource,
                } => {
                    let name = self.types[resource].name.as_ref().unwrap();
                    let interface = self.name_world_key(interface);
                    format!("{}{interface}#[dtor]{name}", abi.export_prefix())
                }
                WasmExport::Memory => "memory".to_string(),
                WasmExport::Initialize => "_initialize".to_string(),
                WasmExport::Realloc => "cabi_realloc".to_string(),
            },
        }
    }
}

/// Possible imports that can be passed to [`Resolve::wasm_import_name`].
#[derive(Debug)]
pub enum WasmImport<'a> {
    /// A WIT function is being imported. Optionally from an interface.
    Func {
        /// The name of the interface that the function is being imported from.
        ///
        /// If the function is imported directly from the world then this is
        /// `Noen`.
        interface: Option<&'a WorldKey>,

        /// The function being imported.
        func: &'a Function,
    },

    /// A resource-related intrinsic is being imported.
    ResourceIntrinsic {
        /// The optional interface to import from, same as `WasmImport::Func`.
        interface: Option<&'a WorldKey>,

        /// The resource that's being operated on.
        resource: TypeId,

        /// The intrinsic that's being imported.
        intrinsic: ResourceIntrinsic,
    },
}

/// Intrinsic definitions to go with [`WasmImport::ResourceIntrinsic`] which
/// also goes with [`Resolve::wasm_import_name`].
#[derive(Debug)]
pub enum ResourceIntrinsic {
    ImportedDrop,
    ExportedDrop,
    ExportedNew,
    ExportedRep,
}

/// Indicates whether a function export is a normal export, a post-return
/// function, or a callback function.
#[derive(Debug)]
pub enum WasmExportKind {
    /// Normal function export.
    Normal,

    /// Post-return function.
    PostReturn,

    /// Async callback function.
    Callback,
}

/// Different kinds of exports that can be passed to
/// [`Resolve::wasm_export_name`] to export from core wasm modules.
#[derive(Debug)]
pub enum WasmExport<'a> {
    /// A WIT function is being exported, optionally from an interface.
    Func {
        /// An optional interface which owns `func`. Use `None` for top-level
        /// world function.
        interface: Option<&'a WorldKey>,

        /// The function being exported.
        func: &'a Function,

        /// Kind of function (normal, post-return, or callback) being exported.
        kind: WasmExportKind,
    },

    /// A destructor for a resource exported from this module.
    ResourceDtor {
        /// The interface that owns the resource.
        interface: &'a WorldKey,
        /// The resource itself that the destructor is for.
        resource: TypeId,
    },

    /// Linear memory, the one that the canonical ABI uses.
    Memory,

    /// An initialization function (not the core wasm `start`).
    Initialize,

    /// The general-purpose realloc hook.
    Realloc,
}

/// Structure returned by [`Resolve::merge`] which contains mappings from
/// old-ids to new-ids after the merge.
#[derive(Default)]
pub struct Remap {
    pub types: Vec<Option<TypeId>>,
    pub interfaces: Vec<Option<InterfaceId>>,
    pub worlds: Vec<Option<WorldId>>,
    pub packages: Vec<PackageId>,

    /// A cache of anonymous `own<T>` handles for resource types.
    ///
    /// The appending operation of `Remap` is the one responsible for
    /// translating references to `T` where `T` is a resource into `own<T>`
    /// instead. This map is used to deduplicate the `own<T>` types generated
    /// to generate as few as possible.
    ///
    /// The key of this map is the resource id `T` in the new resolve, and
    /// the value is the `own<T>` type pointing to `T`.
    own_handles: HashMap<TypeId, TypeId>,

    type_has_borrow: Vec<Option<bool>>,
}

fn apply_map<T>(map: &[Option<Id<T>>], id: Id<T>, desc: &str, span: Option<Span>) -> Result<Id<T>> {
    match map.get(id.index()) {
        Some(Some(id)) => Ok(*id),
        Some(None) => {
            let msg = format!(
                "found a reference to a {desc} which is excluded \
                 due to its feature not being activated"
            );
            match span {
                Some(span) => Err(Error::new(span, msg).into()),
                None => bail!("{msg}"),
            }
        }
        None => panic!("request to remap a {desc} that has not yet been registered"),
    }
}

fn rename(original_name: &str, include_name: &IncludeName) -> Option<String> {
    if original_name == include_name.name {
        return Some(include_name.as_.to_string());
    }
    let (kind, rest) = original_name.split_once(']')?;
    match rest.split_once('.') {
        Some((name, rest)) if name == include_name.name => {
            Some(format!("{kind}]{}.{rest}", include_name.as_))
        }
        _ if rest == include_name.name => Some(format!("{kind}]{}", include_name.as_)),
        _ => None,
    }
}

impl Remap {
    pub fn map_type(&self, id: TypeId, span: Option<Span>) -> Result<TypeId> {
        apply_map(&self.types, id, "type", span)
    }

    pub fn map_interface(&self, id: InterfaceId, span: Option<Span>) -> Result<InterfaceId> {
        apply_map(&self.interfaces, id, "interface", span)
    }

    pub fn map_world(&self, id: WorldId, span: Option<Span>) -> Result<WorldId> {
        apply_map(&self.worlds, id, "world", span)
    }

    fn append(
        &mut self,
        resolve: &mut Resolve,
        unresolved: UnresolvedPackage,
    ) -> Result<PackageId> {
        let pkgid = resolve.packages.alloc(Package {
            name: unresolved.name.clone(),
            docs: unresolved.docs.clone(),
            interfaces: Default::default(),
            worlds: Default::default(),
        });
        let prev = resolve.package_names.insert(unresolved.name.clone(), pkgid);
        if let Some(prev) = prev {
            resolve.package_names.insert(unresolved.name.clone(), prev);
            bail!(
                "attempting to re-add package `{}` when it's already present in this `Resolve`",
                unresolved.name,
            );
        }

        self.process_foreign_deps(resolve, pkgid, &unresolved)?;

        let foreign_types = self.types.len();
        let foreign_interfaces = self.interfaces.len();
        let foreign_worlds = self.worlds.len();

        // Copy over all types first, updating any intra-type references. Note
        // that types are sorted topologically which means this iteration
        // order should be sufficient. Also note though that the interface
        // owner of a type isn't updated here due to interfaces not being known
        // yet.
        assert_eq!(unresolved.types.len(), unresolved.type_spans.len());
        for ((id, mut ty), span) in unresolved
            .types
            .into_iter()
            .zip(&unresolved.type_spans)
            .skip(foreign_types)
        {
            if !resolve.include_type(&ty, pkgid, *span)? {
                self.types.push(None);
                continue;
            }

            self.update_typedef(resolve, &mut ty, Some(*span))?;
            let new_id = resolve.types.alloc(ty);
            assert_eq!(self.types.len(), id.index());

            let new_id = match resolve.types[new_id] {
                // If this is an `own<T>` handle then either replace it with a
                // preexisting `own<T>` handle which may have been generated in
                // `update_ty`. If that doesn't exist though then insert it into
                // the `own_handles` cache.
                TypeDef {
                    name: None,
                    owner: TypeOwner::None,
                    kind: TypeDefKind::Handle(Handle::Own(id)),
                    docs: _,
                    stability: _,
                } => *self.own_handles.entry(id).or_insert(new_id),

                // Everything not-related to `own<T>` doesn't get its ID
                // modified.
                _ => new_id,
            };
            self.types.push(Some(new_id));
        }

        // Next transfer all interfaces into `Resolve`, updating type ids
        // referenced along the way.
        assert_eq!(
            unresolved.interfaces.len(),
            unresolved.interface_spans.len()
        );
        for ((id, mut iface), span) in unresolved
            .interfaces
            .into_iter()
            .zip(&unresolved.interface_spans)
            .skip(foreign_interfaces)
        {
            if !resolve
                .include_stability(&iface.stability, &pkgid, Some(span.span))
                .with_context(|| {
                    format!(
                        "failed to process feature gate for interface [{}] in package [{}]",
                        iface
                            .name
                            .as_ref()
                            .map(String::as_str)
                            .unwrap_or("<unknown>"),
                        resolve.packages[pkgid].name,
                    )
                })?
            {
                self.interfaces.push(None);
                continue;
            }
            assert!(iface.package.is_none());
            iface.package = Some(pkgid);
            self.update_interface(resolve, &mut iface, Some(span))?;
            let new_id = resolve.interfaces.alloc(iface);
            assert_eq!(self.interfaces.len(), id.index());
            self.interfaces.push(Some(new_id));
        }

        // Now that interfaces are identified go back through the types and
        // update their interface owners.
        for (i, id) in self.types.iter().enumerate().skip(foreign_types) {
            let id = match id {
                Some(id) => *id,
                None => continue,
            };
            match &mut resolve.types[id].owner {
                TypeOwner::Interface(id) => {
                    let span = unresolved.type_spans[i];
                    *id = self.map_interface(*id, Some(span))
                        .with_context(|| {
                            "this type is not gated by a feature but its interface is gated by a feature"
                        })?;
                }
                TypeOwner::World(_) | TypeOwner::None => {}
            }
        }

        // Expand worlds. Note that this does NOT process `include` statements,
        // that's handled below. Instead this just handles world item updates
        // and resolves references to types/items within `Resolve`.
        //
        // This is done after types/interfaces are fully settled so the
        // transitive relation between interfaces, through types, is understood
        // here.
        assert_eq!(unresolved.worlds.len(), unresolved.world_spans.len());
        for ((id, mut world), span) in unresolved
            .worlds
            .into_iter()
            .zip(&unresolved.world_spans)
            .skip(foreign_worlds)
        {
            if !resolve
                .include_stability(&world.stability, &pkgid, Some(span.span))
                .with_context(|| {
                    format!(
                        "failed to process feature gate for world [{}] in package [{}]",
                        world.name, resolve.packages[pkgid].name,
                    )
                })?
            {
                self.worlds.push(None);
                continue;
            }
            self.update_world(&mut world, resolve, &pkgid, &span)?;

            let new_id = resolve.worlds.alloc(world);
            assert_eq!(self.worlds.len(), id.index());
            self.worlds.push(Some(new_id));
        }

        // As with interfaces, now update the ids of world-owned types.
        for (i, id) in self.types.iter().enumerate().skip(foreign_types) {
            let id = match id {
                Some(id) => *id,
                None => continue,
            };
            match &mut resolve.types[id].owner {
                TypeOwner::World(id) => {
                    let span = unresolved.type_spans[i];
                    *id = self.map_world(*id, Some(span))
                        .with_context(|| {
                            "this type is not gated by a feature but its interface is gated by a feature"
                        })?;
                }
                TypeOwner::Interface(_) | TypeOwner::None => {}
            }
        }

        // After the above, process `include` statements for worlds and
        // additionally fully elaborate them. Processing of `include` is
        // deferred until after the steps above so the fully resolved state of
        // local types in this package are all available. This is required
        // because `include` may copy types between worlds when the type is
        // defined in the world itself.
        //
        // This step, after processing `include`, will also use
        // `elaborate_world` to fully expand the world in terms of
        // imports/exports and ensure that all necessary imports/exports are all
        // listed.
        //
        // Note that `self.worlds` is already sorted in topological order so if
        // one world refers to another via `include` then it's guaranteed that
        // the one we're referring to is already expanded and ready to be
        // included.
        assert_eq!(self.worlds.len(), unresolved.world_spans.len());
        for (id, span) in self
            .worlds
            .iter()
            .zip(unresolved.world_spans.iter())
            .skip(foreign_worlds)
        {
            let Some(id) = *id else {
                continue;
            };
            self.process_world_includes(id, resolve, &pkgid, &span)?;

            resolve.elaborate_world(id).with_context(|| {
                Error::new(
                    span.span,
                    format!(
                        "failed to elaborate world imports/exports of `{}`",
                        resolve.worlds[id].name
                    ),
                )
            })?;
        }

        // Fixup "parent" ids now that everything has been identified
        for id in self.interfaces.iter().skip(foreign_interfaces) {
            let id = match id {
                Some(id) => *id,
                None => continue,
            };
            let iface = &mut resolve.interfaces[id];
            iface.package = Some(pkgid);
            if let Some(name) = &iface.name {
                let prev = resolve.packages[pkgid].interfaces.insert(name.clone(), id);
                assert!(prev.is_none());
            }
        }
        for id in self.worlds.iter().skip(foreign_worlds) {
            let id = match id {
                Some(id) => *id,
                None => continue,
            };
            let world = &mut resolve.worlds[id];
            world.package = Some(pkgid);
            let prev = resolve.packages[pkgid]
                .worlds
                .insert(world.name.clone(), id);
            assert!(prev.is_none());
        }
        Ok(pkgid)
    }

    fn process_foreign_deps(
        &mut self,
        resolve: &mut Resolve,
        pkgid: PackageId,
        unresolved: &UnresolvedPackage,
    ) -> Result<()> {
        // Invert the `foreign_deps` map to be keyed by world id to get
        // used in the loops below.
        let mut world_to_package = HashMap::new();
        let mut interface_to_package = HashMap::new();
        for (i, (pkg_name, worlds_or_ifaces)) in unresolved.foreign_deps.iter().enumerate() {
            for (name, (item, stabilities)) in worlds_or_ifaces {
                match item {
                    AstItem::Interface(unresolved_interface_id) => {
                        let prev = interface_to_package.insert(
                            *unresolved_interface_id,
                            (pkg_name, name, unresolved.foreign_dep_spans[i], stabilities),
                        );
                        assert!(prev.is_none());
                    }
                    AstItem::World(unresolved_world_id) => {
                        let prev = world_to_package.insert(
                            *unresolved_world_id,
                            (pkg_name, name, unresolved.foreign_dep_spans[i], stabilities),
                        );
                        assert!(prev.is_none());
                    }
                }
            }
        }

        // Connect all interfaces referred to in `interface_to_package`, which
        // are at the front of `unresolved.interfaces`, to interfaces already
        // contained within `resolve`.
        self.process_foreign_interfaces(unresolved, &interface_to_package, resolve, &pkgid)?;

        // Connect all worlds referred to in `world_to_package`, which
        // are at the front of `unresolved.worlds`, to worlds already
        // contained within `resolve`.
        self.process_foreign_worlds(unresolved, &world_to_package, resolve, &pkgid)?;

        // Finally, iterate over all foreign-defined types and determine
        // what they map to.
        self.process_foreign_types(unresolved, pkgid, resolve)?;

        for (id, span) in unresolved.required_resource_types.iter() {
            // Note that errors are ignored here because an error represents a
            // type that has been configured away. If a type is configured away
            // then any future use of it will generate an error so there's no
            // need to validate that it's a resource here.
            let Ok(mut id) = self.map_type(*id, Some(*span)) else {
                continue;
            };
            loop {
                match resolve.types[id].kind {
                    TypeDefKind::Type(Type::Id(i)) => id = i,
                    TypeDefKind::Resource => break,
                    _ => bail!(Error::new(
                        *span,
                        format!("type used in a handle must be a resource"),
                    )),
                }
            }
        }

        #[cfg(debug_assertions)]
        resolve.assert_valid();

        Ok(())
    }

    fn process_foreign_interfaces(
        &mut self,
        unresolved: &UnresolvedPackage,
        interface_to_package: &HashMap<InterfaceId, (&PackageName, &String, Span, &Vec<Stability>)>,
        resolve: &mut Resolve,
        parent_pkg_id: &PackageId,
    ) -> Result<(), anyhow::Error> {
        for (unresolved_iface_id, unresolved_iface) in unresolved.interfaces.iter() {
            let (pkg_name, interface, span, stabilities) =
                match interface_to_package.get(&unresolved_iface_id) {
                    Some(items) => *items,
                    // All foreign interfaces are defined first, so the first one
                    // which is defined in a non-foreign document means that all
                    // further interfaces will be non-foreign as well.
                    None => break,
                };

            let pkgid = resolve
                .package_names
                .get(pkg_name)
                .copied()
                .ok_or_else(|| {
                    PackageNotFoundError::new(
                        span,
                        pkg_name.clone(),
                        resolve.package_names.keys().cloned().collect(),
                    )
                })?;

            // Functions can't be imported so this should be empty.
            assert!(unresolved_iface.functions.is_empty());

            let pkg = &resolve.packages[pkgid];
            let span = &unresolved.interface_spans[unresolved_iface_id.index()];

            let mut enabled = false;
            for stability in stabilities {
                if resolve.include_stability(stability, parent_pkg_id, Some(span.span))? {
                    enabled = true;
                    break;
                }
            }

            if !enabled {
                self.interfaces.push(None);
                continue;
            }

            let iface_id = pkg
                .interfaces
                .get(interface)
                .copied()
                .ok_or_else(|| Error::new(span.span, "interface not found in package"))?;
            assert_eq!(self.interfaces.len(), unresolved_iface_id.index());
            self.interfaces.push(Some(iface_id));
        }
        for (id, _) in unresolved.interfaces.iter().skip(self.interfaces.len()) {
            assert!(
                interface_to_package.get(&id).is_none(),
                "found foreign interface after local interface"
            );
        }
        Ok(())
    }

    fn process_foreign_worlds(
        &mut self,
        unresolved: &UnresolvedPackage,
        world_to_package: &HashMap<WorldId, (&PackageName, &String, Span, &Vec<Stability>)>,
        resolve: &mut Resolve,
        parent_pkg_id: &PackageId,
    ) -> Result<(), anyhow::Error> {
        for (unresolved_world_id, _) in unresolved.worlds.iter() {
            let (pkg_name, world, span, stabilities) =
                match world_to_package.get(&unresolved_world_id) {
                    Some(items) => *items,
                    // Same as above, all worlds are foreign until we find a
                    // non-foreign one.
                    None => break,
                };

            let pkgid = resolve
                .package_names
                .get(pkg_name)
                .copied()
                .ok_or_else(|| Error::new(span, "package not found"))?;
            let pkg = &resolve.packages[pkgid];
            let span = &unresolved.world_spans[unresolved_world_id.index()];

            let mut enabled = false;
            for stability in stabilities {
                if resolve.include_stability(stability, parent_pkg_id, Some(span.span))? {
                    enabled = true;
                    break;
                }
            }

            if !enabled {
                self.worlds.push(None);
                continue;
            }

            let world_id = pkg
                .worlds
                .get(world)
                .copied()
                .ok_or_else(|| Error::new(span.span, "world not found in package"))?;
            assert_eq!(self.worlds.len(), unresolved_world_id.index());
            self.worlds.push(Some(world_id));
        }
        for (id, _) in unresolved.worlds.iter().skip(self.worlds.len()) {
            assert!(
                world_to_package.get(&id).is_none(),
                "found foreign world after local world"
            );
        }
        Ok(())
    }

    fn process_foreign_types(
        &mut self,
        unresolved: &UnresolvedPackage,
        pkgid: PackageId,
        resolve: &mut Resolve,
    ) -> Result<(), anyhow::Error> {
        for ((unresolved_type_id, unresolved_ty), span) in
            unresolved.types.iter().zip(&unresolved.type_spans)
        {
            // All "Unknown" types should appear first so once we're no longer
            // in unknown territory it's package-defined types so break out of
            // this loop.
            match unresolved_ty.kind {
                TypeDefKind::Unknown => {}
                _ => break,
            }

            if !resolve.include_type(unresolved_ty, pkgid, *span)? {
                self.types.push(None);
                continue;
            }

            let unresolved_iface_id = match unresolved_ty.owner {
                TypeOwner::Interface(id) => id,
                _ => unreachable!(),
            };
            let iface_id = self.map_interface(unresolved_iface_id, None)?;
            let name = unresolved_ty.name.as_ref().unwrap();
            let span = unresolved.unknown_type_spans[unresolved_type_id.index()];
            let type_id = *resolve.interfaces[iface_id]
                .types
                .get(name)
                .ok_or_else(|| {
                    Error::new(span, format!("type `{name}` not defined in interface"))
                })?;
            assert_eq!(self.types.len(), unresolved_type_id.index());
            self.types.push(Some(type_id));
        }
        for (_, ty) in unresolved.types.iter().skip(self.types.len()) {
            if let TypeDefKind::Unknown = ty.kind {
                panic!("unknown type after defined type");
            }
        }
        Ok(())
    }

    fn update_typedef(
        &mut self,
        resolve: &mut Resolve,
        ty: &mut TypeDef,
        span: Option<Span>,
    ) -> Result<()> {
        // NB: note that `ty.owner` is not updated here since interfaces
        // haven't been mapped yet and that's done in a separate step.
        use crate::TypeDefKind::*;
        match &mut ty.kind {
            Handle(handle) => match handle {
                crate::Handle::Own(ty) | crate::Handle::Borrow(ty) => {
                    self.update_type_id(ty, span)?
                }
            },
            Resource => {}
            Record(r) => {
                for field in r.fields.iter_mut() {
                    self.update_ty(resolve, &mut field.ty, span)
                        .with_context(|| format!("failed to update field `{}`", field.name))?;
                }
            }
            Tuple(t) => {
                for ty in t.types.iter_mut() {
                    self.update_ty(resolve, ty, span)?;
                }
            }
            Variant(v) => {
                for case in v.cases.iter_mut() {
                    if let Some(t) = &mut case.ty {
                        self.update_ty(resolve, t, span)?;
                    }
                }
            }
            Option(t) | List(t, ..) | FixedSizeList(t, ..) | Future(Some(t)) | Stream(Some(t)) => {
                self.update_ty(resolve, t, span)?
            }
            Map(k, v) => {
                self.update_ty(resolve, k, span)?;
                self.update_ty(resolve, v, span)?;
            }
            Result(r) => {
                if let Some(ty) = &mut r.ok {
                    self.update_ty(resolve, ty, span)?;
                }
                if let Some(ty) = &mut r.err {
                    self.update_ty(resolve, ty, span)?;
                }
            }

            // Note that `update_ty` is specifically not used here as typedefs
            // because for the `type a = b` form that doesn't force `a` to be a
            // handle type if `b` is a resource type, instead `a` is
            // simultaneously usable as a resource and a handle type
            Type(crate::Type::Id(id)) => self.update_type_id(id, span)?,
            Type(_) => {}

            // nothing to do for these as they're just names or empty
            Flags(_) | Enum(_) | Future(None) | Stream(None) => {}

            Unknown => unreachable!(),
        }

        Ok(())
    }

    fn update_ty(
        &mut self,
        resolve: &mut Resolve,
        ty: &mut Type,
        span: Option<Span>,
    ) -> Result<()> {
        let id = match ty {
            Type::Id(id) => id,
            _ => return Ok(()),
        };
        self.update_type_id(id, span)?;

        // If `id` points to a `Resource` type then this means that what was
        // just discovered was a reference to what will implicitly become an
        // `own<T>` handle. This `own` handle is implicitly allocated here
        // and handled during the merging process.
        let mut cur = *id;
        let points_to_resource = loop {
            match resolve.types[cur].kind {
                TypeDefKind::Type(Type::Id(id)) => cur = id,
                TypeDefKind::Resource => break true,
                _ => break false,
            }
        };

        if points_to_resource {
            *id = *self.own_handles.entry(*id).or_insert_with(|| {
                resolve.types.alloc(TypeDef {
                    name: None,
                    owner: TypeOwner::None,
                    kind: TypeDefKind::Handle(Handle::Own(*id)),
                    docs: Default::default(),
                    stability: Default::default(),
                })
            });
        }
        Ok(())
    }

    fn update_type_id(&self, id: &mut TypeId, span: Option<Span>) -> Result<()> {
        *id = self.map_type(*id, span)?;
        Ok(())
    }

    fn update_interface(
        &mut self,
        resolve: &mut Resolve,
        iface: &mut Interface,
        spans: Option<&InterfaceSpan>,
    ) -> Result<()> {
        iface.types.retain(|_, ty| self.types[ty.index()].is_some());
        let iface_pkg_id = iface.package.as_ref().unwrap_or_else(|| {
            panic!(
                "unexpectedly missing package on interface [{}]",
                iface
                    .name
                    .as_ref()
                    .map(String::as_str)
                    .unwrap_or("<unknown>"),
            )
        });

        // NB: note that `iface.doc` is not updated here since interfaces
        // haven't been mapped yet and that's done in a separate step.
        for (_name, ty) in iface.types.iter_mut() {
            self.update_type_id(ty, spans.map(|s| s.span))?;
        }
        if let Some(spans) = spans {
            assert_eq!(iface.functions.len(), spans.funcs.len());
        }
        for (i, (func_name, func)) in iface.functions.iter_mut().enumerate() {
            let span = spans.map(|s| s.funcs[i]);
            if !resolve
                .include_stability(&func.stability, iface_pkg_id, span)
                .with_context(|| {
                    format!(
                        "failed to process feature gate for function [{func_name}] in package [{}]",
                        resolve.packages[*iface_pkg_id].name,
                    )
                })?
            {
                continue;
            }
            self.update_function(resolve, func, span)
                .with_context(|| format!("failed to update function `{}`", func.name))?;
        }

        // Filter out all of the existing functions in interface which fail the
        // `include_stability()` check, as they shouldn't be available.
        for (name, func) in mem::take(&mut iface.functions) {
            if resolve.include_stability(&func.stability, iface_pkg_id, None)? {
                iface.functions.insert(name, func);
            }
        }

        Ok(())
    }

    fn update_function(
        &mut self,
        resolve: &mut Resolve,
        func: &mut Function,
        span: Option<Span>,
    ) -> Result<()> {
        if let Some(id) = func.kind.resource_mut() {
            self.update_type_id(id, span)?;
        }
        for (_, ty) in func.params.iter_mut() {
            self.update_ty(resolve, ty, span)?;
        }
        if let Some(ty) = &mut func.result {
            self.update_ty(resolve, ty, span)?;
        }

        if let Some(ty) = &func.result {
            if self.type_has_borrow(resolve, ty) {
                match span {
                    Some(span) => {
                        bail!(Error::new(
                            span,
                            format!(
                                "function returns a type which contains \
                                 a `borrow<T>` which is not supported"
                            )
                        ))
                    }
                    None => unreachable!(),
                }
            }
        }

        Ok(())
    }

    fn update_world(
        &mut self,
        world: &mut World,
        resolve: &mut Resolve,
        pkg_id: &PackageId,
        spans: &WorldSpan,
    ) -> Result<()> {
        assert_eq!(world.imports.len(), spans.imports.len());
        assert_eq!(world.exports.len(), spans.exports.len());

        // Rewrite imports/exports with their updated versions. Note that this
        // may involve updating the key of the imports/exports maps so this
        // starts by emptying them out and then everything is re-inserted.
        let imports = mem::take(&mut world.imports).into_iter();
        let imports = imports.zip(&spans.imports).map(|p| (p, true));
        let exports = mem::take(&mut world.exports).into_iter();
        let exports = exports.zip(&spans.exports).map(|p| (p, false));
        for (((mut name, mut item), span), import) in imports.chain(exports) {
            // Update the `id` eagerly here so `item.stability(..)` below
            // works.
            if let WorldItem::Type(id) = &mut item {
                *id = self.map_type(*id, Some(*span))?;
            }
            let stability = item.stability(resolve);
            if !resolve
                .include_stability(stability, pkg_id, Some(*span))
                .with_context(|| format!("failed to process world item in `{}`", world.name))?
            {
                continue;
            }
            self.update_world_key(&mut name, Some(*span))?;
            match &mut item {
                WorldItem::Interface { id, .. } => {
                    *id = self.map_interface(*id, Some(*span))?;
                }
                WorldItem::Function(f) => {
                    self.update_function(resolve, f, Some(*span))?;
                }
                WorldItem::Type(_) => {
                    // already mapped above
                }
            }

            let dst = if import {
                &mut world.imports
            } else {
                &mut world.exports
            };
            let prev = dst.insert(name, item);
            assert!(prev.is_none());
        }

        Ok(())
    }

    fn process_world_includes(
        &self,
        id: WorldId,
        resolve: &mut Resolve,
        pkg_id: &PackageId,
        spans: &WorldSpan,
    ) -> Result<()> {
        let world = &mut resolve.worlds[id];
        // Resolve all `include` statements of the world which will add more
        // entries to the imports/exports list for this world.
        assert_eq!(world.includes.len(), spans.includes.len());
        let includes = mem::take(&mut world.includes);
        let include_names = mem::take(&mut world.include_names);
        for (((stability, include_world), span), names) in includes
            .into_iter()
            .zip(&spans.includes)
            .zip(&include_names)
        {
            if !resolve
                .include_stability(&stability, pkg_id, Some(*span))
                .with_context(|| {
                    format!(
                        "failed to process feature gate for included world [{}] in package [{}]",
                        resolve.worlds[include_world].name.as_str(),
                        resolve.packages[*pkg_id].name
                    )
                })?
            {
                continue;
            }
            self.resolve_include(id, include_world, names, *span, pkg_id, resolve)?;
        }

        // Validate that there are no case-insensitive duplicate names in imports/exports
        Self::validate_world_case_insensitive_names(resolve, id)?;

        Ok(())
    }

    /// Validates that a world's imports and exports don't have case-insensitive
    /// duplicate names. Per the WIT specification, kebab-case identifiers are
    /// case-insensitive within the same scope.
    fn validate_world_case_insensitive_names(resolve: &Resolve, world_id: WorldId) -> Result<()> {
        let world = &resolve.worlds[world_id];

        // Helper closure to check for case-insensitive duplicates in a map
        let validate_names = |items: &IndexMap<WorldKey, WorldItem>,
                              item_type: &str|
         -> Result<()> {
            let mut seen_lowercase: HashMap<String, String> = HashMap::new();

            for key in items.keys() {
                // Only WorldKey::Name variants can have case-insensitive conflicts
                if let WorldKey::Name(name) = key {
                    let lowercase_name = name.to_lowercase();

                    if let Some(existing_name) = seen_lowercase.get(&lowercase_name) {
                        // Only error on case-insensitive duplicates (e.g., "foo" vs "FOO").
                        // Exact duplicates would have been caught earlier.
                        if existing_name != name {
                            bail!(
                                "{item_type} `{name}` conflicts with {item_type} `{existing_name}` \
                                (kebab-case identifiers are case-insensitive)"
                            );
                        }
                    }

                    seen_lowercase.insert(lowercase_name, name.clone());
                }
            }

            Ok(())
        };

        validate_names(&world.imports, "import")
            .with_context(|| format!("failed to validate imports in world `{}`", world.name))?;
        validate_names(&world.exports, "export")
            .with_context(|| format!("failed to validate exports in world `{}`", world.name))?;

        Ok(())
    }

    fn update_world_key(&self, key: &mut WorldKey, span: Option<Span>) -> Result<()> {
        match key {
            WorldKey::Name(_) => {}
            WorldKey::Interface(id) => {
                *id = self.map_interface(*id, span)?;
            }
        }
        Ok(())
    }

    fn resolve_include(
        &self,
        id: WorldId,
        include_world_id_orig: WorldId,
        names: &[IncludeName],
        span: Span,
        pkg_id: &PackageId,
        resolve: &mut Resolve,
    ) -> Result<()> {
        let world = &resolve.worlds[id];
        let include_world_id = self.map_world(include_world_id_orig, Some(span))?;
        let include_world = resolve.worlds[include_world_id].clone();
        let mut names_ = names.to_owned();
        let is_external_include = world.package != include_world.package;

        // remove all imports and exports that match the names we're including
        for import in include_world.imports.iter() {
            self.remove_matching_name(import, &mut names_);
        }
        for export in include_world.exports.iter() {
            self.remove_matching_name(export, &mut names_);
        }
        if !names_.is_empty() {
            bail!(Error::new(
                span,
                format!(
                    "no import or export kebab-name `{}`. Note that an ID does not support renaming",
                    names_[0].name
                ),
            ));
        }

        let mut cloner = clone::Cloner::new(
            resolve,
            TypeOwner::World(if is_external_include {
                include_world_id
            } else {
                include_world_id
                // include_world_id_orig
            }),
            TypeOwner::World(id),
        );
        cloner.new_package = Some(*pkg_id);

        // copy the imports and exports from the included world into the current world
        for import in include_world.imports.iter() {
            self.resolve_include_item(
                &mut cloner,
                names,
                |resolve| &mut resolve.worlds[id].imports,
                import,
                span,
                "import",
                is_external_include,
            )?;
        }

        for export in include_world.exports.iter() {
            self.resolve_include_item(
                &mut cloner,
                names,
                |resolve| &mut resolve.worlds[id].exports,
                export,
                span,
                "export",
                is_external_include,
            )?;
        }
        Ok(())
    }

    fn resolve_include_item(
        &self,
        cloner: &mut clone::Cloner<'_>,
        names: &[IncludeName],
        get_items: impl Fn(&mut Resolve) -> &mut IndexMap<WorldKey, WorldItem>,
        item: (&WorldKey, &WorldItem),
        span: Span,
        item_type: &str,
        is_external_include: bool,
    ) -> Result<()> {
        match item.0 {
            WorldKey::Name(n) => {
                let n = names
                    .into_iter()
                    .find_map(|include_name| rename(n, include_name))
                    .unwrap_or(n.clone());

                // When the `with` option to the `include` directive is
                // specified and is used to rename a function that means that
                // the function's own original name needs to be updated, so
                // reflect the change not only in the world key but additionally
                // in the function itself.
                let mut new_item = item.1.clone();
                let key = WorldKey::Name(n.clone());
                cloner.world_item(&key, &mut new_item, &mut CloneMaps::default());
                match &mut new_item {
                    WorldItem::Function(f) => f.name = n.clone(),
                    WorldItem::Type(id) => cloner.resolve.types[*id].name = Some(n.clone()),
                    WorldItem::Interface { .. } => {}
                }

                let prev = get_items(cloner.resolve).insert(key, new_item);
                if prev.is_some() {
                    bail!(Error::new(
                        span,
                        format!("{item_type} of `{n}` shadows previously {item_type}ed items"),
                    ))
                }
            }
            key @ WorldKey::Interface(_) => {
                let prev = get_items(cloner.resolve)
                    .entry(key.clone())
                    .or_insert(item.1.clone());
                match (&item.1, prev) {
                    (
                        WorldItem::Interface {
                            id: aid,
                            stability: astability,
                        },
                        WorldItem::Interface {
                            id: bid,
                            stability: bstability,
                        },
                    ) => {
                        assert_eq!(*aid, *bid);
                        merge_include_stability(astability, bstability, is_external_include)?;
                    }
                    (WorldItem::Interface { .. }, _) => unreachable!(),
                    (WorldItem::Function(_), _) => unreachable!(),
                    (WorldItem::Type(_), _) => unreachable!(),
                }
            }
        };

        Ok(())
    }

    fn remove_matching_name(&self, item: (&WorldKey, &WorldItem), names: &mut Vec<IncludeName>) {
        match item.0 {
            WorldKey::Name(n) => {
                names.retain(|name| rename(n, name).is_none());
            }
            _ => {}
        }
    }

    fn type_has_borrow(&mut self, resolve: &Resolve, ty: &Type) -> bool {
        let id = match ty {
            Type::Id(id) => *id,
            _ => return false,
        };

        if let Some(Some(has_borrow)) = self.type_has_borrow.get(id.index()) {
            return *has_borrow;
        }

        let result = self.typedef_has_borrow(resolve, &resolve.types[id]);
        if self.type_has_borrow.len() <= id.index() {
            self.type_has_borrow.resize(id.index() + 1, None);
        }
        self.type_has_borrow[id.index()] = Some(result);
        result
    }

    fn typedef_has_borrow(&mut self, resolve: &Resolve, ty: &TypeDef) -> bool {
        match &ty.kind {
            TypeDefKind::Type(t) => self.type_has_borrow(resolve, t),
            TypeDefKind::Variant(v) => v
                .cases
                .iter()
                .filter_map(|case| case.ty.as_ref())
                .any(|ty| self.type_has_borrow(resolve, ty)),
            TypeDefKind::Handle(Handle::Borrow(_)) => true,
            TypeDefKind::Handle(Handle::Own(_)) => false,
            TypeDefKind::Resource => false,
            TypeDefKind::Record(r) => r
                .fields
                .iter()
                .any(|case| self.type_has_borrow(resolve, &case.ty)),
            TypeDefKind::Flags(_) => false,
            TypeDefKind::Tuple(t) => t.types.iter().any(|t| self.type_has_borrow(resolve, t)),
            TypeDefKind::Enum(_) => false,
            TypeDefKind::List(ty)
            | TypeDefKind::FixedSizeList(ty, ..)
            | TypeDefKind::Future(Some(ty))
            | TypeDefKind::Stream(Some(ty))
            | TypeDefKind::Option(ty) => self.type_has_borrow(resolve, ty),
            TypeDefKind::Map(k, v) => {
                self.type_has_borrow(resolve, k) || self.type_has_borrow(resolve, v)
            }
            TypeDefKind::Result(r) => [&r.ok, &r.err]
                .iter()
                .filter_map(|t| t.as_ref())
                .any(|t| self.type_has_borrow(resolve, t)),
            TypeDefKind::Future(None) | TypeDefKind::Stream(None) => false,
            TypeDefKind::Unknown => unreachable!(),
        }
    }
}

struct MergeMap<'a> {
    /// A map of package ids in `from` to those in `into` for those that are
    /// found to be equivalent.
    package_map: HashMap<PackageId, PackageId>,

    /// A map of interface ids in `from` to those in `into` for those that are
    /// found to be equivalent.
    interface_map: HashMap<InterfaceId, InterfaceId>,

    /// A map of type ids in `from` to those in `into` for those that are
    /// found to be equivalent.
    type_map: HashMap<TypeId, TypeId>,

    /// A map of world ids in `from` to those in `into` for those that are
    /// found to be equivalent.
    world_map: HashMap<WorldId, WorldId>,

    /// A list of documents that need to be added to packages in `into`.
    ///
    /// The elements here are:
    ///
    /// * The name of the interface/world
    /// * The ID within `into` of the package being added to
    /// * The ID within `from` of the item being added.
    interfaces_to_add: Vec<(String, PackageId, InterfaceId)>,
    worlds_to_add: Vec<(String, PackageId, WorldId)>,

    /// Which `Resolve` is being merged from.
    from: &'a Resolve,

    /// Which `Resolve` is being merged into.
    into: &'a Resolve,
}

impl<'a> MergeMap<'a> {
    fn new(from: &'a Resolve, into: &'a Resolve) -> MergeMap<'a> {
        MergeMap {
            package_map: Default::default(),
            interface_map: Default::default(),
            type_map: Default::default(),
            world_map: Default::default(),
            interfaces_to_add: Default::default(),
            worlds_to_add: Default::default(),
            from,
            into,
        }
    }

    fn build(&mut self) -> Result<()> {
        for from_id in self.from.topological_packages() {
            let from = &self.from.packages[from_id];
            let into_id = match self.into.package_names.get(&from.name) {
                Some(id) => *id,

                // This package, according to its name and url, is not present
                // in `self` so it needs to get added below.
                None => {
                    log::trace!("adding unique package {}", from.name);
                    continue;
                }
            };
            log::trace!("merging duplicate package {}", from.name);

            self.build_package(from_id, into_id).with_context(|| {
                format!("failed to merge package `{}` into existing copy", from.name)
            })?;
        }

        Ok(())
    }

    fn build_package(&mut self, from_id: PackageId, into_id: PackageId) -> Result<()> {
        let prev = self.package_map.insert(from_id, into_id);
        assert!(prev.is_none());

        let from = &self.from.packages[from_id];
        let into = &self.into.packages[into_id];

        // If an interface is present in `from_id` but not present in `into_id`
        // then it can be copied over wholesale. That copy is scheduled to
        // happen within the `self.interfaces_to_add` list.
        for (name, from_interface_id) in from.interfaces.iter() {
            let into_interface_id = match into.interfaces.get(name) {
                Some(id) => *id,
                None => {
                    log::trace!("adding unique interface {name}");
                    self.interfaces_to_add
                        .push((name.clone(), into_id, *from_interface_id));
                    continue;
                }
            };

            log::trace!("merging duplicate interfaces {name}");
            self.build_interface(*from_interface_id, into_interface_id)
                .with_context(|| format!("failed to merge interface `{name}`"))?;
        }

        for (name, from_world_id) in from.worlds.iter() {
            let into_world_id = match into.worlds.get(name) {
                Some(id) => *id,
                None => {
                    log::trace!("adding unique world {name}");
                    self.worlds_to_add
                        .push((name.clone(), into_id, *from_world_id));
                    continue;
                }
            };

            log::trace!("merging duplicate worlds {name}");
            self.build_world(*from_world_id, into_world_id)
                .with_context(|| format!("failed to merge world `{name}`"))?;
        }

        Ok(())
    }

    fn build_interface(&mut self, from_id: InterfaceId, into_id: InterfaceId) -> Result<()> {
        let prev = self.interface_map.insert(from_id, into_id);
        assert!(prev.is_none());

        let from_interface = &self.from.interfaces[from_id];
        let into_interface = &self.into.interfaces[into_id];

        // Unlike documents/interfaces above if an interface in `from`
        // differs from the interface in `into` then that's considered an
        // error. Changing interfaces can reflect changes in imports/exports
        // which may not be expected so it's currently required that all
        // interfaces, when merged, exactly match.
        //
        // One case to consider here, for example, is that if a world in
        // `into` exports the interface `into_id` then if `from_id` were to
        // add more items into `into` then it would unexpectedly require more
        // items to be exported which may not work. In an import context this
        // might work since it's "just more items available for import", but
        // for now a conservative route of "interfaces must match" is taken.

        for (name, from_type_id) in from_interface.types.iter() {
            let into_type_id = *into_interface
                .types
                .get(name)
                .ok_or_else(|| anyhow!("expected type `{name}` to be present"))?;
            let prev = self.type_map.insert(*from_type_id, into_type_id);
            assert!(prev.is_none());

            self.build_type_id(*from_type_id, into_type_id)
                .with_context(|| format!("mismatch in type `{name}`"))?;
        }

        for (name, from_func) in from_interface.functions.iter() {
            let into_func = match into_interface.functions.get(name) {
                Some(func) => func,
                None => bail!("expected function `{name}` to be present"),
            };
            self.build_function(from_func, into_func)
                .with_context(|| format!("mismatch in function `{name}`"))?;
        }

        Ok(())
    }

    fn build_type_id(&mut self, from_id: TypeId, into_id: TypeId) -> Result<()> {
        // FIXME: ideally the types should be "structurally
        // equal" but that's not trivial to do in the face of
        // resources.
        let _ = from_id;
        let _ = into_id;
        Ok(())
    }

    fn build_type(&mut self, from_ty: &Type, into_ty: &Type) -> Result<()> {
        match (from_ty, into_ty) {
            (Type::Id(from), Type::Id(into)) => {
                self.build_type_id(*from, *into)?;
            }
            (from, into) if from != into => bail!("different kinds of types"),
            _ => {}
        }
        Ok(())
    }

    fn build_function(&mut self, from_func: &Function, into_func: &Function) -> Result<()> {
        if from_func.name != into_func.name {
            bail!(
                "different function names `{}` and `{}`",
                from_func.name,
                into_func.name
            );
        }
        match (&from_func.kind, &into_func.kind) {
            (FunctionKind::Freestanding, FunctionKind::Freestanding) => {}
            (FunctionKind::AsyncFreestanding, FunctionKind::AsyncFreestanding) => {}

            (FunctionKind::Method(from), FunctionKind::Method(into))
            | (FunctionKind::Static(from), FunctionKind::Static(into))
            | (FunctionKind::AsyncMethod(from), FunctionKind::AsyncMethod(into))
            | (FunctionKind::AsyncStatic(from), FunctionKind::AsyncStatic(into))
            | (FunctionKind::Constructor(from), FunctionKind::Constructor(into)) => {
                self.build_type_id(*from, *into)
                    .context("different function kind types")?;
            }

            (FunctionKind::Method(_), _)
            | (FunctionKind::Constructor(_), _)
            | (FunctionKind::Static(_), _)
            | (FunctionKind::Freestanding, _)
            | (FunctionKind::AsyncFreestanding, _)
            | (FunctionKind::AsyncMethod(_), _)
            | (FunctionKind::AsyncStatic(_), _) => {
                bail!("different function kind types")
            }
        }

        if from_func.params.len() != into_func.params.len() {
            bail!("different number of function parameters");
        }
        for ((from_name, from_ty), (into_name, into_ty)) in
            from_func.params.iter().zip(&into_func.params)
        {
            if from_name != into_name {
                bail!("different function parameter names: {from_name} != {into_name}");
            }
            self.build_type(from_ty, into_ty)
                .with_context(|| format!("different function parameter types for `{from_name}`"))?;
        }
        match (&from_func.result, &into_func.result) {
            (Some(from_ty), Some(into_ty)) => {
                self.build_type(from_ty, into_ty)
                    .context("different function result types")?;
            }
            (None, None) => {}
            (Some(_), None) | (None, Some(_)) => bail!("different number of function results"),
        }
        Ok(())
    }

    fn build_world(&mut self, from_id: WorldId, into_id: WorldId) -> Result<()> {
        let prev = self.world_map.insert(from_id, into_id);
        assert!(prev.is_none());

        let from_world = &self.from.worlds[from_id];
        let into_world = &self.into.worlds[into_id];

        // Same as interfaces worlds are expected to exactly match to avoid
        // unexpectedly changing a particular component's view of imports and
        // exports.
        //
        // FIXME: this should probably share functionality with
        // `Resolve::merge_worlds` to support adding imports but not changing
        // exports.

        if from_world.imports.len() != into_world.imports.len() {
            bail!("world contains different number of imports than expected");
        }
        if from_world.exports.len() != into_world.exports.len() {
            bail!("world contains different number of exports than expected");
        }

        for (from_name, from) in from_world.imports.iter() {
            let into_name = MergeMap::map_name(from_name, &self.interface_map);
            let name_str = self.from.name_world_key(from_name);
            let into = into_world
                .imports
                .get(&into_name)
                .ok_or_else(|| anyhow!("import `{name_str}` not found in target world"))?;
            self.match_world_item(from, into)
                .with_context(|| format!("import `{name_str}` didn't match target world"))?;
        }

        for (from_name, from) in from_world.exports.iter() {
            let into_name = MergeMap::map_name(from_name, &self.interface_map);
            let name_str = self.from.name_world_key(from_name);
            let into = into_world
                .exports
                .get(&into_name)
                .ok_or_else(|| anyhow!("export `{name_str}` not found in target world"))?;
            self.match_world_item(from, into)
                .with_context(|| format!("export `{name_str}` didn't match target world"))?;
        }

        Ok(())
    }

    fn map_name(
        from_name: &WorldKey,
        interface_map: &HashMap<InterfaceId, InterfaceId>,
    ) -> WorldKey {
        match from_name {
            WorldKey::Name(s) => WorldKey::Name(s.clone()),
            WorldKey::Interface(id) => {
                WorldKey::Interface(interface_map.get(id).copied().unwrap_or(*id))
            }
        }
    }

    fn match_world_item(&mut self, from: &WorldItem, into: &WorldItem) -> Result<()> {
        match (from, into) {
            (WorldItem::Interface { id: from, .. }, WorldItem::Interface { id: into, .. }) => {
                match (
                    &self.from.interfaces[*from].name,
                    &self.into.interfaces[*into].name,
                ) {
                    // If one interface is unnamed then they must both be
                    // unnamed and they must both have the same structure for
                    // now.
                    (None, None) => self.build_interface(*from, *into)?,

                    // Otherwise both interfaces must be named and they must
                    // have been previously found to be equivalent. Note that
                    // if either is unnamed it won't be present in
                    // `interface_map` so this'll return an error.
                    _ => {
                        if self.interface_map.get(&from) != Some(&into) {
                            bail!("interfaces are not the same");
                        }
                    }
                }
            }
            (WorldItem::Function(from), WorldItem::Function(into)) => {
                let _ = (from, into);
                // FIXME: should assert an check that `from` structurally
                // matches `into`
            }
            (WorldItem::Type(from), WorldItem::Type(into)) => {
                // FIXME: should assert an check that `from` structurally
                // matches `into`
                let prev = self.type_map.insert(*from, *into);
                assert!(prev.is_none());
            }

            (WorldItem::Interface { .. }, _)
            | (WorldItem::Function(_), _)
            | (WorldItem::Type(_), _) => {
                bail!("world items do not have the same type")
            }
        }
        Ok(())
    }
}

/// Updates stability annotations when merging `from` into `into`.
///
/// This is done to keep up-to-date stability information if possible.
/// Components for example don't carry stability information but WIT does so
/// this tries to move from "unknown" to stable/unstable if possible.
fn update_stability(from: &Stability, into: &mut Stability) -> Result<()> {
    // If `from` is unknown or the two stability annotations are equal then
    // there's nothing to do here.
    if from == into || from.is_unknown() {
        return Ok(());
    }
    // Otherwise if `into` is unknown then inherit the stability listed in
    // `from`.
    if into.is_unknown() {
        *into = from.clone();
        return Ok(());
    }

    // Failing all that this means that the two attributes are different so
    // generate an error.
    bail!("mismatch in stability from '{:?}' to '{:?}'", from, into)
}

fn merge_include_stability(
    from: &Stability,
    into: &mut Stability,
    is_external_include: bool,
) -> Result<()> {
    if is_external_include && from.is_stable() {
        log::trace!("dropped stability from external package");
        *into = Stability::Unknown;
        return Ok(());
    }

    return update_stability(from, into);
}

/// An error that can be returned during "world elaboration" during various
/// [`Resolve`] operations.
///
/// Methods on [`Resolve`] which mutate its internals, such as
/// [`Resolve::push_dir`] or [`Resolve::importize`] can fail if `world` imports
/// in WIT packages are invalid. This error indicates one of these situations
/// where an invalid dependency graph between imports and exports are detected.
///
/// Note that at this time this error is subtle and not easy to understand, and
/// work needs to be done to explain this better and additionally provide a
/// better error message. For now though this type enables callers to test for
/// the exact kind of error emitted.
#[derive(Debug, Clone)]
pub struct InvalidTransitiveDependency(String);

impl fmt::Display for InvalidTransitiveDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "interface `{}` transitively depends on an interface in \
             incompatible ways",
            self.0
        )
    }
}

impl std::error::Error for InvalidTransitiveDependency {}

#[cfg(test)]
mod tests {
    use crate::Resolve;
    use anyhow::Result;

    #[test]
    fn select_world() -> Result<()> {
        let mut resolve = Resolve::default();
        resolve.push_str(
            "test.wit",
            r#"
                package foo:bar@0.1.0;

                world foo {}
            "#,
        )?;
        resolve.push_str(
            "test.wit",
            r#"
                package foo:baz@0.1.0;

                world foo {}
            "#,
        )?;
        resolve.push_str(
            "test.wit",
            r#"
                package foo:baz@0.2.0;

                world foo {}
            "#,
        )?;

        let dummy = resolve.push_str(
            "test.wit",
            r#"
                package foo:dummy;

                world foo {}
            "#,
        )?;

        assert!(resolve.select_world(&[dummy], None).is_ok());
        assert!(resolve.select_world(&[dummy], Some("xx")).is_err());
        assert!(resolve.select_world(&[dummy], Some("")).is_err());
        assert!(resolve.select_world(&[dummy], Some("foo:bar/foo")).is_ok());
        assert!(
            resolve
                .select_world(&[dummy], Some("foo:bar/foo@0.1.0"))
                .is_ok()
        );
        assert!(resolve.select_world(&[dummy], Some("foo:baz/foo")).is_err());
        assert!(
            resolve
                .select_world(&[dummy], Some("foo:baz/foo@0.1.0"))
                .is_ok()
        );
        assert!(
            resolve
                .select_world(&[dummy], Some("foo:baz/foo@0.2.0"))
                .is_ok()
        );
        Ok(())
    }

    /// When there are multiple packages and there's no main package, don't
    /// pick a world just based on it being the only one that matches.
    #[test]
    fn select_world_multiple_packages() -> Result<()> {
        use wit_parser::Resolve;

        let mut resolve = Resolve::default();

        // Just one world in one package; we always succeed.
        let stuff = resolve.push_str(
            "./my-test.wit",
            r#"
                    package test:stuff;

                    world foo {
                        // ...
                    }
                "#,
        )?;
        assert!(resolve.select_world(&[stuff], None).is_ok());
        assert!(resolve.select_world(&[stuff], Some("foo")).is_ok());

        // Multiple packages, but still just one total world. Lookups
        // without a main package now fail.
        let empty = resolve.push_str(
            "./my-test.wit",
            r#"
                    package test:empty;
                "#,
        )?;
        assert!(resolve.select_world(&[stuff, empty], None).is_err());
        assert!(resolve.select_world(&[stuff, empty], Some("foo")).is_err());
        assert!(resolve.select_world(&[empty], None).is_err());
        assert!(resolve.select_world(&[empty], Some("foo")).is_err());

        Ok(())
    }

    /// Test selecting a world with multiple versions of a package name.
    #[test]
    fn select_world_versions() -> Result<()> {
        use wit_parser::Resolve;

        let mut resolve = Resolve::default();

        let _id = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:distraction;
                "#,
        )?;

        // When selecting with a version it's ok to drop the version when
        // there's only a single copy of that package in `Resolve`.
        let versions_1 = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:versions@1.0.0;

                    world foo { /* ... */ }
                "#,
        )?;
        assert!(resolve.select_world(&[versions_1], Some("foo")).is_ok());
        assert!(
            resolve
                .select_world(&[versions_1], Some("foo@1.0.0"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[versions_1], Some("example:versions/foo"))
                .is_ok()
        );
        assert!(
            resolve
                .select_world(&[versions_1], Some("example:versions/foo@1.0.0"))
                .is_ok()
        );

        // However when a single package has multiple versions in a resolve
        // it's required to specify the version to select which one.
        let versions_2 = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:versions@2.0.0;

                    world foo { /* ... */ }
                "#,
        )?;
        assert!(
            resolve
                .select_world(&[versions_1, versions_2], Some("foo"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[versions_1, versions_2], Some("foo@1.0.0"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[versions_1, versions_2], Some("foo@2.0.0"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[versions_1, versions_2], Some("example:versions/foo"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(
                    &[versions_1, versions_2],
                    Some("example:versions/foo@1.0.0")
                )
                .is_ok()
        );
        assert!(
            resolve
                .select_world(
                    &[versions_1, versions_2],
                    Some("example:versions/foo@2.0.0")
                )
                .is_ok()
        );

        Ok(())
    }

    /// Test overriding a main package using name qualification
    #[test]
    fn select_world_override_qualification() -> Result<()> {
        use wit_parser::Resolve;

        let mut resolve = Resolve::default();

        let other = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:other;

                    world foo { }
                "#,
        )?;

        // A fully-qualified name overrides a main package.
        let fq = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:fq;

                    world bar { }
                "#,
        )?;
        assert!(resolve.select_world(&[other, fq], Some("foo")).is_err());
        assert!(resolve.select_world(&[other, fq], Some("bar")).is_err());
        assert!(
            resolve
                .select_world(&[other, fq], Some("example:other/foo"))
                .is_ok()
        );
        assert!(
            resolve
                .select_world(&[other, fq], Some("example:fq/bar"))
                .is_ok()
        );
        assert!(
            resolve
                .select_world(&[other, fq], Some("example:other/bar"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[other, fq], Some("example:fq/foo"))
                .is_err()
        );

        Ok(())
    }

    /// Test selecting with fully-qualified world names.
    #[test]
    fn select_world_fully_qualified() -> Result<()> {
        use wit_parser::Resolve;

        let mut resolve = Resolve::default();

        let distraction = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:distraction;
                "#,
        )?;

        // If a package has multiple worlds, then we can't guess the world
        // even if we know the package.
        let multiworld = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:multiworld;

                    world foo { /* ... */ }

                    world bar { /* ... */ }
                "#,
        )?;
        assert!(
            resolve
                .select_world(&[distraction, multiworld], None)
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[distraction, multiworld], Some("foo"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[distraction, multiworld], Some("example:multiworld/foo"))
                .is_ok()
        );
        assert!(
            resolve
                .select_world(&[distraction, multiworld], Some("bar"))
                .is_err()
        );
        assert!(
            resolve
                .select_world(&[distraction, multiworld], Some("example:multiworld/bar"))
                .is_ok()
        );

        Ok(())
    }

    /// Test `select_world` with single and multiple packages.
    #[test]
    fn select_world_packages() -> Result<()> {
        use wit_parser::Resolve;

        let mut resolve = Resolve::default();

        // If there's a single package and only one world, that world is
        // the obvious choice.
        let wit1 = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:wit1;

                    world foo {
                        // ...
                    }
                "#,
        )?;
        assert!(resolve.select_world(&[wit1], None).is_ok());
        assert!(resolve.select_world(&[wit1], Some("foo")).is_ok());
        assert!(
            resolve
                .select_world(&[wit1], Some("example:wit1/foo"))
                .is_ok()
        );
        assert!(resolve.select_world(&[wit1], Some("bar")).is_err());
        assert!(
            resolve
                .select_world(&[wit1], Some("example:wit2/foo"))
                .is_err()
        );

        // If there are multiple packages, we need to be told which package
        // to use.
        let wit2 = resolve.push_str(
            "./my-test.wit",
            r#"
                    package example:wit2;

                    world foo { /* ... */ }
                "#,
        )?;
        assert!(resolve.select_world(&[wit1, wit2], None).is_err());
        assert!(resolve.select_world(&[wit1, wit2], Some("foo")).is_err());
        assert!(
            resolve
                .select_world(&[wit1, wit2], Some("example:wit1/foo"))
                .is_ok()
        );
        assert!(resolve.select_world(&[wit2], None).is_ok());
        assert!(resolve.select_world(&[wit2], Some("foo")).is_ok());
        assert!(
            resolve
                .select_world(&[wit2], Some("example:wit1/foo"))
                .is_ok()
        );
        assert!(resolve.select_world(&[wit1, wit2], Some("bar")).is_err());
        assert!(
            resolve
                .select_world(&[wit1, wit2], Some("example:wit2/foo"))
                .is_ok()
        );
        assert!(resolve.select_world(&[wit2], Some("bar")).is_err());
        assert!(
            resolve
                .select_world(&[wit2], Some("example:wit2/foo"))
                .is_ok()
        );

        Ok(())
    }
}
