use crate::ast::lex::Span;
use crate::ast::{parse_use_path, AstUsePath};
#[cfg(feature = "serde")]
use crate::serde_::{serialize_arena, serialize_id_map};
use crate::{
    AstItem, Docs, Error, Function, FunctionKind, Handle, IncludeName, Interface, InterfaceId,
    PackageName, Results, Type, TypeDef, TypeDefKind, TypeId, TypeOwner, UnresolvedPackage, World,
    WorldId, WorldItem, WorldKey,
};
use anyhow::{anyhow, bail, Context, Result};
use id_arena::{Arena, Id};
use indexmap::{IndexMap, IndexSet};
#[cfg(feature = "serde")]
use serde_derive::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::mem;
use std::path::{Path, PathBuf};

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
    /// All knowns worlds within this `Resolve`.
    ///
    /// Each world points at a `PackageId` which is stored below. No ordering is
    /// guaranteed between this list of worlds.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub worlds: Arena<World>,

    /// All knowns interfaces within this `Resolve`.
    ///
    /// Each interface points at a `PackageId` which is stored below. No
    /// ordering is guaranteed between this list of interfaces.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub interfaces: Arena<Interface>,

    /// All knowns types within this `Resolve`.
    ///
    /// Types are topologically sorted such that any type referenced from one
    /// type is guaranteed to be defined previously. Otherwise though these are
    /// not sorted by interface for example.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub types: Arena<TypeDef>,

    /// All knowns packages within this `Resolve`.
    ///
    /// This list of packages is not sorted. Sorted packages can be queried
    /// through [`Resolve::topological_packages`].
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arena"))]
    pub packages: Arena<Package>,

    /// A map of package names to the ID of the package with that name.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub package_names: IndexMap<PackageName, PackageId>,
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

enum ParsedFile {
    #[cfg(feature = "decoding")]
    Package(PackageId),
    Unresolved(UnresolvedPackage),
}

impl Resolve {
    /// Creates a new [`Resolve`] with no packages/items inside of it.
    pub fn new() -> Resolve {
        Resolve::default()
    }

    /// Parse a WIT package from the input `path`.
    ///
    /// The input `path` can be one of:
    ///
    /// * A directory containing a WIT package with an optional `deps` directory
    ///   for any dependent WIT packages it references.
    /// * A single standalone WIT file depending on what's already in `Resolve`.
    /// * A wasm-encoded WIT package as a single file in the wasm binary format.
    /// * A wasm-encoded WIT package as a single file in the wasm text format.
    ///
    /// The `PackageId` of the parsed package is returned. For more information
    /// see [`Resolve::push_dir`] and [`Resolve::push_file`]. This method will
    /// automatically call the appropriate method based on what kind of
    /// filesystem entry `path` is.
    ///
    /// Returns the top-level [`PackageId`] as well as a list of all files read
    /// during this parse.
    pub fn push_path(&mut self, path: impl AsRef<Path>) -> Result<(PackageId, Vec<PathBuf>)> {
        self._push_path(path.as_ref())
    }

    fn _push_path(&mut self, path: &Path) -> Result<(PackageId, Vec<PathBuf>)> {
        if path.is_dir() {
            self.push_dir(path).with_context(|| {
                format!(
                    "failed to resolve directory while parsing WIT for path [{}]",
                    path.display()
                )
            })
        } else {
            let id = self.push_file(path)?;
            Ok((id, vec![path.to_path_buf()]))
        }
    }

    /// Parses the filesystem directory at `path` as a WIT package and returns
    /// the fully resolved [`PackageId`] as a result.
    ///
    /// The directory itself is parsed with [`UnresolvedPackage::parse_dir`]
    /// which has more information on the layout of the directory. This method,
    /// however, additionally supports an optional `deps` dir where dependencies
    /// can be located.
    ///
    /// All entries in the `deps` directory are inspected and parsed as follows:
    ///
    /// * Any directories inside of `deps` are assumed to be another WIT package
    ///   and are parsed with [`UnresolvedPackage::parse_dir`].
    /// * WIT files (`*.wit`) are parsed with [`UnresolvedPackage::parse_file`].
    /// * WebAssembly files (`*.wasm` or `*.wat`) are assumed to be WIT packages
    ///   encoded to wasm and are parsed and inserted into `self`.
    ///
    /// This function returns the [`PackageId`] of the root parsed package at
    /// `path`, along with a list of all paths that were consumed during parsing
    /// for the root package and all dependency packages.
    pub fn push_dir(&mut self, path: &Path) -> Result<(PackageId, Vec<PathBuf>)> {
        let pkg = UnresolvedPackage::parse_dir(path)
            .with_context(|| format!("failed to parse package: {}", path.display()))?;

        let deps = path.join("deps");
        let mut deps = self
            .parse_deps_dir(&deps)
            .with_context(|| format!("failed to parse dependency directory: {}", deps.display()))?;

        // Perform a simple topological sort which will bail out on cycles
        // and otherwise determine the order that packages must be added to
        // this `Resolve`.
        let mut order = IndexSet::new();
        let mut visiting = HashSet::new();
        for pkg in deps.values().chain([&pkg]) {
            visit(&pkg, &deps, &mut order, &mut visiting)?;
        }

        // Using the topological ordering insert each package incrementally.
        // Additionally note that the last item visited here is the root
        // package, which is the one returned here.
        let mut last = None;
        let mut files = Vec::new();
        let mut pkg = Some(pkg);
        for name in order {
            let pkg = deps.remove(&name).unwrap_or_else(|| pkg.take().unwrap());
            files.extend(pkg.source_files().map(|p| p.to_path_buf()));
            let pkgid = self.push(pkg)?;
            last = Some(pkgid);
        }

        return Ok((last.unwrap(), files));

        fn visit<'a>(
            pkg: &'a UnresolvedPackage,
            deps: &'a BTreeMap<PackageName, UnresolvedPackage>,
            order: &mut IndexSet<PackageName>,
            visiting: &mut HashSet<&'a PackageName>,
        ) -> Result<()> {
            if order.contains(&pkg.name) {
                return Ok(());
            }
            pkg.source_map.rewrite_error(|| {
                for (i, (dep, _)) in pkg.foreign_deps.iter().enumerate() {
                    let span = pkg.foreign_dep_spans[i];
                    if !visiting.insert(dep) {
                        bail!(Error {
                            span,
                            msg: format!("package depends on itself"),
                        });
                    }
                    if let Some(dep) = deps.get(dep) {
                        visit(dep, deps, order, visiting)?;
                    }
                    assert!(visiting.remove(dep));
                }
                assert!(order.insert(pkg.name.clone()));
                Ok(())
            })
        }
    }

    fn parse_deps_dir(&mut self, path: &Path) -> Result<BTreeMap<PackageName, UnresolvedPackage>> {
        let mut ret = BTreeMap::new();
        // If there's no `deps` dir, then there's no deps, so return the
        // empty set.
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

            let pkg = if dep.file_type()?.is_dir() {
                // If this entry is a directory then always parse it as an
                // `UnresolvedPackage` since it's intentional to not support
                // recursive `deps` directories.
                UnresolvedPackage::parse_dir(&path)
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
            let prev = ret.insert(pkg.name.clone(), pkg);
            if let Some(prev) = prev {
                bail!("duplicate definitions of package `{}` found", prev.name);
            }
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
    /// In both situations the `PackageId` of the resulting resolved package is
    /// returned from this method.
    pub fn push_file(&mut self, path: impl AsRef<Path>) -> Result<PackageId> {
        match self._push_file(path.as_ref())? {
            #[cfg(feature = "decoding")]
            ParsedFile::Package(id) => Ok(id),
            ParsedFile::Unresolved(pkg) => self.push(pkg),
        }
    }

    fn _push_file(&mut self, path: &Path) -> Result<ParsedFile> {
        let contents = std::fs::read(path)
            .with_context(|| format!("failed to read path for WIT [{}]", path.display()))?;

        // If decoding is enabled at compile time then try to see if this is a
        // wasm file.
        #[cfg(feature = "decoding")]
        {
            use crate::decoding::{decode, DecodedWasm};

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
        let pkg = UnresolvedPackage::parse(path, text)?;
        Ok(ParsedFile::Unresolved(pkg))
    }

    /// Appends a new [`UnresolvedPackage`] to this [`Resolve`], creating a
    /// fully resolved package with no dangling references.
    ///
    /// The `deps` argument indicates that the named dependencies in
    /// `unresolved` to packages are resolved by the mapping specified.
    ///
    /// Any dependency resolution error or otherwise world-elaboration error
    /// will be returned here. If successful a package identifier is returned.
    pub fn push(&mut self, mut unresolved: UnresolvedPackage) -> Result<PackageId> {
        let source_map = mem::take(&mut unresolved.source_map);
        source_map.rewrite_error(|| Remap::default().append(self, unresolved))
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
            | Type::Float32
            | Type::Float64 => true,

            Type::Bool | Type::Char | Type::String => false,

            Type::Id(id) => match &self.types[*id].kind {
                TypeDefKind::List(_)
                | TypeDefKind::Variant(_)
                | TypeDefKind::Enum(_)
                | TypeDefKind::Option(_)
                | TypeDefKind::Result(_)
                | TypeDefKind::Future(_)
                | TypeDefKind::Stream(_) => false,
                TypeDefKind::Type(t) => self.all_bits_valid(t),

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

        let mut map = MergeMap::new(&resolve, &self)?;
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
        } = resolve;

        let mut moved_types = Vec::new();
        for (id, mut ty) in types {
            let new_id = type_map.get(&id).copied().unwrap_or_else(|| {
                log::debug!("moving type {:?}", ty.name);
                moved_types.push(id);
                remap.update_typedef(self, &mut ty);
                self.types.alloc(ty)
            });
            assert_eq!(remap.types.len(), id.index());
            remap.types.push(new_id);
        }

        let mut moved_interfaces = Vec::new();
        for (id, mut iface) in interfaces {
            let new_id = interface_map.get(&id).copied().unwrap_or_else(|| {
                log::debug!("moving interface {:?}", iface.name);
                moved_interfaces.push(id);
                remap.update_interface(self, &mut iface);
                self.interfaces.alloc(iface)
            });
            assert_eq!(remap.interfaces.len(), id.index());
            remap.interfaces.push(new_id);
        }

        let mut moved_worlds = Vec::new();
        for (id, mut world) in worlds {
            let new_id = world_map.get(&id).copied().unwrap_or_else(|| {
                log::debug!("moving world {}", world.name);
                moved_worlds.push(id);
                let mut update = |map: &mut IndexMap<WorldKey, WorldItem>| {
                    for (mut name, mut item) in mem::take(map) {
                        remap.update_world_key(&mut name);
                        match &mut item {
                            WorldItem::Function(f) => remap.update_function(self, f),
                            WorldItem::Interface(i) => *i = remap.interfaces[i.index()],
                            WorldItem::Type(i) => *i = remap.types[i.index()],
                        }
                        map.insert(name, item);
                    }
                };
                update(&mut world.imports);
                update(&mut world.exports);
                self.worlds.alloc(world)
            });
            assert_eq!(remap.worlds.len(), id.index());
            remap.worlds.push(new_id);
        }

        for (id, mut pkg) in packages {
            let new_id = package_map.get(&id).copied().unwrap_or_else(|| {
                for (_, id) in pkg.interfaces.iter_mut() {
                    *id = remap.interfaces[id.index()];
                }
                for (_, id) in pkg.worlds.iter_mut() {
                    *id = remap.worlds[id.index()];
                }
                self.packages.alloc(pkg)
            });
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
            let id = remap.worlds[id.index()];
            let pkg = self.worlds[id].package.as_mut().unwrap();
            *pkg = remap.packages[pkg.index()];
        }
        for id in moved_interfaces {
            let id = remap.interfaces[id.index()];
            let pkg = self.interfaces[id].package.as_mut().unwrap();
            *pkg = remap.packages[pkg.index()];
        }
        for id in moved_types {
            let id = remap.types[id.index()];
            match &mut self.types[id].owner {
                TypeOwner::Interface(id) => *id = remap.interfaces[id.index()],
                TypeOwner::World(id) => *id = remap.worlds[id.index()],
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
                .insert(name, remap.interfaces[iface.index()]);
            assert!(prev.is_none());
        }
        for (name, pkg, world) in worlds_to_add {
            let prev = self.packages[pkg]
                .worlds
                .insert(name, remap.worlds[world.index()]);
            assert!(prev.is_none());
        }

        log::trace!("now have {} packages", self.packages.len());
        Ok(remap)
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
    /// This operation can fail if the imports/exports overlap.
    pub fn merge_worlds(&mut self, from: WorldId, into: WorldId) -> Result<()> {
        let mut new_imports = Vec::new();
        let mut new_exports = Vec::new();

        let from_world = &self.worlds[from];
        let into_world = &self.worlds[into];

        // Build a map of the imports/exports in `into` going the reverse
        // direction from what's listed. This is then consulted below to ensure
        // that the same item isn't exported or imported under two different
        // names which isn't allowed in the component model.
        let mut into_imports_by_id = HashMap::new();
        let mut into_exports_by_id = HashMap::new();
        for (name, import) in into_world.imports.iter() {
            if let WorldItem::Interface(id) = *import {
                let prev = into_imports_by_id.insert(id, name);
                assert!(prev.is_none());
            }
        }
        for (name, export) in into_world.exports.iter() {
            if let WorldItem::Interface(id) = *export {
                let prev = into_exports_by_id.insert(id, name);
                assert!(prev.is_none());
            }
        }
        for (name, import) in from_world.imports.iter() {
            // If the "from" world imports an interface which is already
            // imported by the "into" world then this is allowed if the names
            // are the same. Importing the same interface under different names
            // isn't allowed, but otherwise merging imports of
            // same-named-interfaces is allowed to merge them together.
            if let WorldItem::Interface(id) = import {
                if let Some(prev) = into_imports_by_id.get(id) {
                    if *prev != name {
                        let name = self.name_world_key(name);
                        let prev = self.name_world_key(prev);
                        bail!("import `{name}` conflicts with previous name of `{prev}`");
                    }
                }
            }
        }
        for (name, export) in from_world.exports.iter() {
            // Note that unlike imports same-named exports are not handled here
            // since if something is exported twice there's no way to "unify" it
            // so it's left as an error.
            if let WorldItem::Interface(id) = export {
                if let Some(prev) = into_exports_by_id.get(id) {
                    let name = self.name_world_key(name);
                    let prev = self.name_world_key(prev);
                    bail!("export `{name}` conflicts with previous name of `{prev}`");
                }
            }
        }

        // Next walk over the interfaces imported into `from_world` and queue up
        // imports to get inserted into `into_world`.
        for (name, from_import) in from_world.imports.iter() {
            match into_world.imports.get(name) {
                Some(into_import) => match (from_import, into_import) {
                    // If these imports, which have the same name, are of the
                    // same interface then union them together at this point.
                    (WorldItem::Interface(from), WorldItem::Interface(into)) if from == into => {
                        continue
                    }
                    _ => {
                        let name = self.name_world_key(name);
                        bail!("duplicate import found for interface `{name}`");
                    }
                },
                None => new_imports.push((name.clone(), from_import.clone())),
            }
        }

        // All exports at this time must be unique. For example the same
        // interface exported from two locations can't really be resolved to one
        // canonical definition, so make sure that merging worlds only succeeds
        // if the worlds have disjoint sets of exports.
        for (name, export) in from_world.exports.iter() {
            match into_world.exports.get(name) {
                Some(_) => {
                    let name = self.name_world_key(name);
                    bail!("duplicate export found for interface `{name}`");
                }
                None => new_exports.push((name.clone(), export.clone())),
            }
        }

        // Insert any new imports and new exports found first.
        let into = &mut self.worlds[into];
        for (name, import) in new_imports {
            let prev = into.imports.insert(name, import);
            assert!(prev.is_none());
        }
        for (name, export) in new_exports {
            let prev = into.exports.insert(name, export);
            assert!(prev.is_none());
        }

        Ok(())
    }

    /// Returns the ID of the specified `interface`.
    ///
    /// Returns `None` for unnamed interfaces.
    pub fn id_of(&self, interface: InterfaceId) -> Option<String> {
        let interface = &self.interfaces[interface];
        Some(self.id_of_name(interface.package.unwrap(), interface.name.as_ref()?))
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

    /// Attempts to locate a world given the "default" package `pkg` and the
    /// optional string specifier `world`.
    ///
    /// This method is intended to be used by bindings generation tools to
    /// select a world from either `pkg` or a package in this `Resolve`.
    ///
    /// If `world` is `None` then `pkg` must have precisely one world which will
    /// be returned.
    ///
    /// If `world` is `Some` then it can either be:
    ///
    /// * A kebab-name of a world contained within `pkg` which is being
    ///   selected, such as `"the-world"`.
    ///
    /// * An ID-based form of a world which is selected within this `Resolve`,
    ///   ignoring `pkg`. For example `"wasi:http/proxy"`.
    ///
    /// If successful the corresponding `WorldId` is returned, otherwise an
    /// error is returned.
    pub fn select_world(&self, pkg: PackageId, world: Option<&str>) -> Result<WorldId> {
        let world = match world {
            Some(world) => world,
            None => {
                let pkg = &self.packages[pkg];
                match pkg.worlds.len() {
                    0 => bail!("no worlds found in package `{}`", pkg.name),
                    1 => return Ok(*pkg.worlds.values().next().unwrap()),
                    _ => bail!(
                        "multiple worlds found in package `{}`: one must be explicitly chosen",
                        pkg.name
                    ),
                }
            }
        };

        let path = parse_use_path(world)
            .with_context(|| format!("failed to parse world specifier `{world}`"))?;
        let (pkg, world) = match path {
            AstUsePath::Name(name) => (pkg, name),
            AstUsePath::Package(pkg, interface) => {
                let pkg = match self.package_names.get(&pkg) {
                    Some(pkg) => *pkg,
                    None => {
                        let mut candidates = self.package_names.iter().filter(|(name, _)| {
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
                (pkg, interface)
            }
        };
        let pkg = &self.packages[pkg];
        pkg.worlds
            .get(&world)
            .copied()
            .ok_or_else(|| anyhow!("no world named `{world}` in package"))
    }

    /// Assigns a human readable name to the `WorldKey` specified.
    pub fn name_world_key(&self, key: &WorldKey) -> String {
        match key {
            WorldKey::Name(s) => s.to_string(),
            WorldKey::Interface(i) => self.id_of(*i).expect("unexpected anonymous interface"),
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
                        WorldItem::Interface(id) => Some(*id),
                        WorldItem::Function(_) => None,
                        WorldItem::Type(t) => self.type_interface_dep(*t),
                    })
            }))
            .filter_map(move |iface_id| {
                let pkg = self.interfaces[iface_id].package?;
                if pkg == id {
                    None
                } else {
                    Some(pkg)
                }
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
            assert!(self.packages.get(world.package.unwrap()).is_some());
            assert!(package_worlds[world.package.unwrap().index()].contains(&id));

            let mut types = HashSet::new();
            for (name, item) in world.imports.iter().chain(world.exports.iter()) {
                log::debug!("validating world item: {}", self.name_world_key(name));
                match item {
                    WorldItem::Interface(_) => {}
                    WorldItem::Function(f) => {
                        assert_eq!(f.name, name.clone().unwrap_name());
                    }
                    WorldItem::Type(ty) => {
                        assert!(types.insert(*ty));
                        let ty = &self.types[*ty];
                        assert_eq!(ty.name, Some(name.clone().unwrap_name()));

                        // TODO: `Resolve::merge_worlds` doesn't uphold this
                        // invariant, and that should be fixed.
                        if false {
                            assert_eq!(ty.owner, TypeOwner::World(id));
                        }
                    }
                }
            }
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
                    WorldItem::Interface(id) => *id,
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
}

/// Structure returned by [`Resolve::merge`] which contains mappings from
/// old-ids to new-ids after the merge.
#[derive(Default)]
pub struct Remap {
    pub types: Vec<TypeId>,
    pub interfaces: Vec<InterfaceId>,
    pub worlds: Vec<WorldId>,
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
}

impl Remap {
    fn append(
        &mut self,
        resolve: &mut Resolve,
        unresolved: UnresolvedPackage,
    ) -> Result<PackageId> {
        self.process_foreign_deps(resolve, &unresolved)?;

        let foreign_types = self.types.len();
        let foreign_interfaces = self.interfaces.len();
        let foreign_worlds = self.worlds.len();

        // Copy over all types first, updating any intra-type references. Note
        // that types are sorted topologically which means this iteration
        // order should be sufficient. Also note though that the interface
        // owner of a type isn't updated here due to interfaces not being known
        // yet.
        for (id, mut ty) in unresolved.types.into_iter().skip(foreign_types) {
            self.update_typedef(resolve, &mut ty);
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
                } => *self.own_handles.entry(id).or_insert(new_id),

                // Everything not-related to `own<T>` doesn't get its ID
                // modified.
                _ => new_id,
            };
            self.types.push(new_id);
        }

        // Next transfer all interfaces into `Resolve`, updating type ids
        // referenced along the way.
        for (id, mut iface) in unresolved.interfaces.into_iter().skip(foreign_interfaces) {
            self.update_interface(resolve, &mut iface);
            let new_id = resolve.interfaces.alloc(iface);
            assert_eq!(self.interfaces.len(), id.index());
            self.interfaces.push(new_id);
        }

        // Now that interfaces are identified go back through the types and
        // update their interface owners.
        for id in self.types.iter().skip(foreign_types) {
            match &mut resolve.types[*id].owner {
                TypeOwner::Interface(id) => *id = self.interfaces[id.index()],
                TypeOwner::World(_) | TypeOwner::None => {}
            }
        }

        // Perform a weighty step of full resolution of worlds. This will fully
        // expand imports/exports for a world and create the topological
        // ordering necessary for this.
        //
        // This is done after types/interfaces are fully settled so the
        // transitive relation between interfaces, through types, is understood
        // here.
        assert_eq!(unresolved.worlds.len(), unresolved.world_item_spans.len());
        let include_world_spans = unresolved.include_world_spans;
        for ((id, mut world), (import_spans, export_spans)) in unresolved
            .worlds
            .into_iter()
            .zip(unresolved.world_item_spans)
            .skip(foreign_worlds)
        {
            self.update_world(
                &mut world,
                resolve,
                &import_spans,
                &export_spans,
                &include_world_spans,
            )?;

            let new_id = resolve.worlds.alloc(world);
            assert_eq!(self.worlds.len(), id.index());
            self.worlds.push(new_id);
        }

        // As with interfaces, now update the ids of world-owned types.
        for id in self.types.iter().skip(foreign_types) {
            match &mut resolve.types[*id].owner {
                TypeOwner::World(id) => *id = self.worlds[id.index()],
                TypeOwner::Interface(_) | TypeOwner::None => {}
            }
        }

        // Fixup "parent" ids now that everything has been identified
        let pkgid = resolve.packages.alloc(Package {
            name: unresolved.name.clone(),
            docs: unresolved.docs.clone(),
            interfaces: Default::default(),
            worlds: Default::default(),
        });
        let prev = resolve.package_names.insert(unresolved.name.clone(), pkgid);
        assert!(prev.is_none());
        for id in self.interfaces.iter().skip(foreign_interfaces) {
            let iface = &mut resolve.interfaces[*id];
            iface.package = Some(pkgid);
            if let Some(name) = &iface.name {
                let prev = resolve.packages[pkgid].interfaces.insert(name.clone(), *id);
                assert!(prev.is_none());
            }
        }
        for id in self.worlds.iter().skip(foreign_worlds) {
            let world = &mut resolve.worlds[*id];
            world.package = Some(pkgid);
            let prev = resolve.packages[pkgid]
                .worlds
                .insert(world.name.clone(), *id);
            assert!(prev.is_none());
        }
        Ok(pkgid)
    }

    fn process_foreign_deps(
        &mut self,
        resolve: &mut Resolve,
        unresolved: &UnresolvedPackage,
    ) -> Result<()> {
        // Invert the `foreign_deps` map to be keyed by world id to get
        // used in the loops below.
        let mut world_to_package = HashMap::new();
        let mut interface_to_package = HashMap::new();
        for (i, (pkg_name, worlds_or_ifaces)) in unresolved.foreign_deps.iter().enumerate() {
            for (name, item) in worlds_or_ifaces {
                match item {
                    AstItem::Interface(unresolved_interface_id) => {
                        let prev = interface_to_package.insert(
                            *unresolved_interface_id,
                            (pkg_name, name, unresolved.foreign_dep_spans[i]),
                        );
                        assert!(prev.is_none());
                    }
                    AstItem::World(unresolved_world_id) => {
                        let prev = world_to_package.insert(
                            *unresolved_world_id,
                            (pkg_name, name, unresolved.foreign_dep_spans[i]),
                        );
                        assert!(prev.is_none());
                    }
                }
            }
        }

        // Connect all interfaces referred to in `interface_to_package`, which
        // are at the front of `unresolved.interfaces`, to interfaces already
        // contained within `resolve`.
        self.process_foreign_interfaces(unresolved, &interface_to_package, resolve)?;

        // Connect all worlds referred to in `world_to_package`, which
        // are at the front of `unresolved.worlds`, to worlds already
        // contained within `resolve`.
        self.process_foreign_worlds(unresolved, &world_to_package, resolve)?;

        // Finally, iterate over all foreign-defined types and determine
        // what they map to.
        self.process_foreign_types(unresolved, resolve)?;

        for (id, span) in unresolved.required_resource_types.iter() {
            let mut id = self.types[id.index()];
            loop {
                match resolve.types[id].kind {
                    TypeDefKind::Type(Type::Id(i)) => id = i,
                    TypeDefKind::Resource => break,
                    _ => bail!(Error {
                        span: *span,
                        msg: format!("type used in a handle must be a resource"),
                    }),
                }
            }
        }

        Ok(())
    }

    fn process_foreign_interfaces(
        &mut self,
        unresolved: &UnresolvedPackage,
        interface_to_package: &HashMap<InterfaceId, (&PackageName, &String, Span)>,
        resolve: &mut Resolve,
    ) -> Result<(), anyhow::Error> {
        for (unresolved_iface_id, unresolved_iface) in unresolved.interfaces.iter() {
            let (pkg_name, interface, span) = match interface_to_package.get(&unresolved_iface_id) {
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
                .ok_or_else(|| Error {
                    span,
                    msg: format!("package not found"),
                })?;

            // Functions can't be imported so this should be empty.
            assert!(unresolved_iface.functions.is_empty());

            let pkg = &resolve.packages[pkgid];
            let span = unresolved.interface_spans[unresolved_iface_id.index()];
            let iface_id = pkg
                .interfaces
                .get(interface)
                .copied()
                .ok_or_else(|| Error {
                    span,
                    msg: format!("interface not found in package"),
                })?;
            assert_eq!(self.interfaces.len(), unresolved_iface_id.index());
            self.interfaces.push(iface_id);
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
        world_to_package: &HashMap<WorldId, (&PackageName, &String, Span)>,
        resolve: &mut Resolve,
    ) -> Result<(), anyhow::Error> {
        for (unresolved_world_id, _) in unresolved.worlds.iter() {
            let (pkg_name, world, span) = match world_to_package.get(&unresolved_world_id) {
                Some(items) => *items,
                // Same as above, all worlds are foreign until we find a
                // non-foreign one.
                None => break,
            };

            let pkgid = resolve
                .package_names
                .get(pkg_name)
                .copied()
                .ok_or_else(|| Error {
                    span,
                    msg: format!("package not found"),
                })?;
            let pkg = &resolve.packages[pkgid];
            let span = unresolved.world_spans[unresolved_world_id.index()];
            let world_id = pkg.worlds.get(world).copied().ok_or_else(|| Error {
                span,
                msg: format!("world not found in package"),
            })?;
            assert_eq!(self.worlds.len(), unresolved_world_id.index());
            self.worlds.push(world_id);
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
        resolve: &mut Resolve,
    ) -> Result<(), anyhow::Error> {
        for (unresolved_type_id, unresolved_ty) in unresolved.types.iter() {
            // All "Unknown" types should appear first so once we're no longer
            // in unknown territory it's package-defined types so break out of
            // this loop.
            match unresolved_ty.kind {
                TypeDefKind::Unknown => {}
                _ => break,
            }
            let unresolved_iface_id = match unresolved_ty.owner {
                TypeOwner::Interface(id) => id,
                _ => unreachable!(),
            };
            let iface_id = self.interfaces[unresolved_iface_id.index()];
            let name = unresolved_ty.name.as_ref().unwrap();
            let span = unresolved.unknown_type_spans[unresolved_type_id.index()];
            let type_id = *resolve.interfaces[iface_id]
                .types
                .get(name)
                .ok_or_else(|| Error {
                    span,
                    msg: format!("type `{name}` not defined in interface"),
                })?;
            assert_eq!(self.types.len(), unresolved_type_id.index());
            self.types.push(type_id);
        }
        for (_, ty) in unresolved.types.iter().skip(self.types.len()) {
            if let TypeDefKind::Unknown = ty.kind {
                panic!("unknown type after defined type");
            }
        }
        Ok(())
    }

    fn update_typedef(&mut self, resolve: &mut Resolve, ty: &mut TypeDef) {
        // NB: note that `ty.owner` is not updated here since interfaces
        // haven't been mapped yet and that's done in a separate step.
        use crate::TypeDefKind::*;
        match &mut ty.kind {
            Handle(handle) => match handle {
                crate::Handle::Own(ty) | crate::Handle::Borrow(ty) => self.update_type_id(ty),
            },
            Resource => {}
            Record(r) => {
                for field in r.fields.iter_mut() {
                    self.update_ty(resolve, &mut field.ty);
                }
            }
            Tuple(t) => {
                for ty in t.types.iter_mut() {
                    self.update_ty(resolve, ty);
                }
            }
            Variant(v) => {
                for case in v.cases.iter_mut() {
                    if let Some(t) = &mut case.ty {
                        self.update_ty(resolve, t);
                    }
                }
            }
            Option(t) => self.update_ty(resolve, t),
            Result(r) => {
                if let Some(ty) = &mut r.ok {
                    self.update_ty(resolve, ty);
                }
                if let Some(ty) = &mut r.err {
                    self.update_ty(resolve, ty);
                }
            }
            List(t) => self.update_ty(resolve, t),
            Future(Some(t)) => self.update_ty(resolve, t),
            Stream(t) => {
                if let Some(ty) = &mut t.element {
                    self.update_ty(resolve, ty);
                }
                if let Some(ty) = &mut t.end {
                    self.update_ty(resolve, ty);
                }
            }

            // Note that `update_ty` is specifically not used here as typedefs
            // because for the `type a = b` form that doesn't force `a` to be a
            // handle type if `b` is a resource type, instead `a` is
            // simultaneously usable as a resource and a handle type
            Type(crate::Type::Id(id)) => self.update_type_id(id),
            Type(_) => {}

            // nothing to do for these as they're just names or empty
            Flags(_) | Enum(_) | Future(None) => {}

            Unknown => unreachable!(),
        }
    }

    fn update_ty(&mut self, resolve: &mut Resolve, ty: &mut Type) {
        let id = match ty {
            Type::Id(id) => id,
            _ => return,
        };
        self.update_type_id(id);

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
                })
            });
        }
    }

    fn update_type_id(&self, id: &mut TypeId) {
        *id = self.types[id.index()];
    }

    fn update_interface(&mut self, resolve: &mut Resolve, iface: &mut Interface) {
        // NB: note that `iface.doc` is not updated here since interfaces
        // haven't been mapped yet and that's done in a separate step.
        for (_name, ty) in iface.types.iter_mut() {
            self.update_type_id(ty);
        }
        for (_, func) in iface.functions.iter_mut() {
            self.update_function(resolve, func);
        }
    }

    fn update_function(&mut self, resolve: &mut Resolve, func: &mut Function) {
        match &mut func.kind {
            FunctionKind::Freestanding => {}
            FunctionKind::Method(id) | FunctionKind::Constructor(id) | FunctionKind::Static(id) => {
                self.update_type_id(id);
            }
        }
        for (_, ty) in func.params.iter_mut() {
            self.update_ty(resolve, ty);
        }
        match &mut func.results {
            Results::Named(named) => {
                for (_, ty) in named.iter_mut() {
                    self.update_ty(resolve, ty);
                }
            }
            Results::Anon(ty) => self.update_ty(resolve, ty),
        }
    }

    fn update_world(
        &mut self,
        world: &mut World,
        resolve: &mut Resolve,
        import_spans: &[Span],
        export_spans: &[Span],
        include_world_spans: &[Span],
    ) -> Result<()> {
        // NB: this function is more more complicated than the prior versions
        // of merging an item because this is the location that elaboration of
        // imports/exports of a world are fully resolved. With full transitive
        // knowledge of all interfaces a worlds imports, for example, are
        // expanded fully to ensure that all transitive items are necessarily
        // imported.
        assert_eq!(world.imports.len(), import_spans.len());
        assert_eq!(world.exports.len(), export_spans.len());

        // First up, process all the `imports` of the world. Note that this
        // starts by gutting the list of imports stored in `world` to get
        // rebuilt iteratively below.
        //
        // Here each import of an interface is recorded and then additionally
        // explicitly named imports of interfaces are recorded as well for
        // determining names later on.
        let mut import_funcs = Vec::new();
        let mut import_types = Vec::new();
        for ((mut name, item), span) in mem::take(&mut world.imports).into_iter().zip(import_spans)
        {
            self.update_world_key(&mut name);
            match item {
                WorldItem::Interface(id) => {
                    let id = self.interfaces[id.index()];
                    self.add_world_import(resolve, world, name, id);
                }
                WorldItem::Function(mut f) => {
                    self.update_function(resolve, &mut f);
                    import_funcs.push((name.unwrap_name(), f, *span));
                }
                WorldItem::Type(id) => {
                    let id = self.types[id.index()];
                    import_types.push((name.unwrap_name(), id, *span));
                }
            }
        }

        for (_name, id, _span) in import_types.iter() {
            if let TypeDefKind::Type(Type::Id(other)) = resolve.types[*id].kind {
                if let TypeOwner::Interface(owner) = resolve.types[other].owner {
                    let name = WorldKey::Interface(owner);
                    self.add_world_import(resolve, world, name, owner);
                }
            }
        }

        let mut export_funcs = Vec::new();
        let mut export_interfaces = IndexMap::new();
        for ((mut name, item), span) in mem::take(&mut world.exports).into_iter().zip(export_spans)
        {
            self.update_world_key(&mut name);
            match item {
                WorldItem::Interface(id) => {
                    let id = self.interfaces[id.index()];
                    let prev = export_interfaces.insert(id, (name, *span));
                    assert!(prev.is_none());
                }
                WorldItem::Function(mut f) => {
                    self.update_function(resolve, &mut f);
                    let name = match name {
                        WorldKey::Name(name) => name,
                        WorldKey::Interface(_) => unreachable!(),
                    };
                    export_funcs.push((name, f, *span));
                }
                WorldItem::Type(_) => unreachable!(),
            }
        }

        self.add_world_exports(resolve, world, &export_interfaces)?;

        // Resolve all includes of the world
        let includes = mem::take(&mut world.includes);
        let include_names = mem::take(&mut world.include_names);
        for (index, include_world) in includes.into_iter().enumerate() {
            let span = include_world_spans[index];
            let names = &include_names[index];
            self.resolve_include(world, include_world, names, span, resolve)?;
        }

        for (name, id, span) in import_types {
            let prev = world
                .imports
                .insert(WorldKey::Name(name.clone()), WorldItem::Type(id));
            if prev.is_some() {
                bail!(Error {
                    msg: format!("export of type `{name}` shadows previously imported interface"),
                    span,
                })
            }
        }

        for (name, func, span) in import_funcs {
            let prev = world
                .imports
                .insert(WorldKey::Name(name.clone()), WorldItem::Function(func));
            if prev.is_some() {
                bail!(Error {
                    msg: format!(
                        "import of function `{name}` shadows previously imported interface"
                    ),
                    span,
                })
            }
        }

        for (name, func, span) in export_funcs {
            let prev = world
                .exports
                .insert(WorldKey::Name(name.clone()), WorldItem::Function(func));
            if prev.is_some() {
                bail!(Error {
                    msg: format!(
                        "export of function `{name}` shadows previously exported interface"
                    ),
                    span,
                })
            }
        }

        // After all that sort functions in exports to come before interfaces in
        // exports. This is not strictly required for correctness but make
        // iterating over a world much easier for consumers. Exported functions
        // are guaranteed to use types from either imported interfaces or
        // imported types into the world itself. Currently there is no means by
        // which an export function, at the root, can use types from any other
        // exported interfaces (can't be modeled syntactically in WIT). This
        // means that by placing all functions first it guarantees that visitors
        // which visit imports first then exports will walk over types and
        // references in the order of what they're actually using.
        //
        // For example if an interface is both imported and exported and an
        // exported function uses a type from that interface, then a visitor
        // should visit the imported interface, then the exported function, then
        // the exported interface. That way tables about "where was this type
        // defined" will be correct as the last-inserted item will be used and
        // correctly account for this.
        world.exports.sort_by(|_, a, _, b| {
            let rank = |item: &WorldItem| match item {
                WorldItem::Type(_) => unreachable!(),
                WorldItem::Function(_) => 0,
                WorldItem::Interface(_) => 1,
            };
            rank(a).cmp(&rank(b))
        });

        log::trace!("imports = {:?}", world.imports);
        log::trace!("exports = {:?}", world.exports);

        Ok(())
    }

    fn update_world_key(&self, key: &mut WorldKey) {
        match key {
            WorldKey::Name(_) => {}
            WorldKey::Interface(id) => {
                *id = self.interfaces[id.index()];
            }
        }
    }

    fn add_world_import(
        &self,
        resolve: &Resolve,
        world: &mut World,
        key: WorldKey,
        id: InterfaceId,
    ) {
        if world.imports.contains_key(&key) {
            return;
        }
        for dep in resolve.interface_direct_deps(id) {
            self.add_world_import(resolve, world, WorldKey::Interface(dep), dep);
        }
        let prev = world.imports.insert(key, WorldItem::Interface(id));
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
    fn add_world_exports(
        &self,
        resolve: &Resolve,
        world: &mut World,
        export_interfaces: &IndexMap<InterfaceId, (WorldKey, Span)>,
    ) -> Result<()> {
        let mut required_imports = HashSet::new();
        for (id, (key, span)) in export_interfaces.iter() {
            let ok = add_world_export(
                resolve,
                world,
                export_interfaces,
                &mut required_imports,
                *id,
                key,
                true,
            );
            if !ok {
                bail!(Error {
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
                    msg: format!(
                        "interface transitively depends on an interface in \
                         incompatible ways",
                    ),
                    span: *span,
                });
            }
        }
        return Ok(());

        fn add_world_export(
            resolve: &Resolve,
            world: &mut World,
            export_interfaces: &IndexMap<InterfaceId, (WorldKey, Span)>,
            required_imports: &mut HashSet<InterfaceId>,
            id: InterfaceId,
            key: &WorldKey,
            add_export: bool,
        ) -> bool {
            if world.exports.contains_key(key) {
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
                    world,
                    export_interfaces,
                    required_imports,
                    dep,
                    &key,
                    add_export,
                )
            });
            if !ok {
                return false;
            }
            if add_export {
                if required_imports.contains(&id) {
                    return false;
                }
                world.exports.insert(key.clone(), WorldItem::Interface(id));
            } else {
                required_imports.insert(id);
                world.imports.insert(key.clone(), WorldItem::Interface(id));
            }
            true
        }
    }

    fn resolve_include(
        &self,
        world: &mut World,
        include_world: WorldId,
        names: &[IncludeName],
        span: Span,
        resolve: &Resolve,
    ) -> Result<()> {
        let include_world_id = self.worlds[include_world.index()];
        let include_world = &resolve.worlds[include_world_id];
        let mut names_ = names.to_owned();

        // remove all imports and exports that match the names we're including
        for import in include_world.imports.iter() {
            self.remove_matching_name(import, &mut names_);
        }
        for export in include_world.exports.iter() {
            self.remove_matching_name(export, &mut names_);
        }
        if !names_.is_empty() {
            bail!(Error {
                msg: format!("no import or export kebab-name `{}`. Note that an ID does not support renaming", names_[0].name),
                span: span,
            });
        }

        // copy the imports and exports from the included world into the current world
        for import in include_world.imports.iter() {
            self.resolve_include_item(names, &mut world.imports, import, span, "import")?;
        }

        for export in include_world.exports.iter() {
            self.resolve_include_item(names, &mut world.exports, export, span, "export")?;
        }
        Ok(())
    }

    fn resolve_include_item(
        &self,
        names: &[IncludeName],
        items: &mut IndexMap<WorldKey, WorldItem>,
        item: (&WorldKey, &WorldItem),
        span: Span,
        item_type: &str,
    ) -> Result<()> {
        match item.0 {
            WorldKey::Name(n) => {
                let n = if let Some(found) = names
                    .into_iter()
                    .find(|include_name| include_name.name == n.clone())
                {
                    found.as_.clone()
                } else {
                    n.clone()
                };

                let prev = items.insert(WorldKey::Name(n.clone()), item.1.clone());
                if prev.is_some() {
                    bail!(Error {
                        msg: format!("{item_type} of `{n}` shadows previously {item_type}ed items"),
                        span,
                    })
                }
            }
            key => {
                let prev = items.insert(key.clone(), item.1.clone());
                if let Some(prev) = prev {
                    assert_eq!(prev, item.1.clone());
                }
            }
        };
        Ok(())
    }

    fn remove_matching_name(&self, item: (&WorldKey, &WorldItem), names: &mut Vec<IncludeName>) {
        match item.0 {
            WorldKey::Name(n) => {
                names.retain(|name| name.name != n.clone());
            }
            _ => {}
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
    fn new(from: &'a Resolve, into: &'a Resolve) -> Result<MergeMap<'a>> {
        Ok(MergeMap {
            package_map: Default::default(),
            interface_map: Default::default(),
            type_map: Default::default(),
            world_map: Default::default(),
            interfaces_to_add: Default::default(),
            worlds_to_add: Default::default(),
            from,
            into,
        })
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
                    log::trace!("adding unique interface {}", name);
                    self.interfaces_to_add
                        .push((name.clone(), into_id, *from_interface_id));
                    continue;
                }
            };

            log::trace!("merging duplicate interfaces {}", name);
            self.build_interface(*from_interface_id, into_interface_id)
                .with_context(|| format!("failed to merge interface `{name}`"))?;
        }

        for (name, from_world_id) in from.worlds.iter() {
            let into_world_id = match into.worlds.get(name) {
                Some(id) => *id,
                None => {
                    log::trace!("adding unique world {}", name);
                    self.worlds_to_add
                        .push((name.clone(), into_id, *from_world_id));
                    continue;
                }
            };

            log::trace!("merging duplicate worlds {}", name);
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

            // FIXME: ideally the types should be "structurally
            // equal" but that's not trivial to do in the face of
            // resources.
        }

        for (name, _) in from_interface.functions.iter() {
            if !into_interface.functions.contains_key(name) {
                bail!("expected function `{name}` to be present");
            }

            // FIXME: ideally the functions should be "structurally
            // equal" but that's not trivial to do in the face of
            // resources.
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
            let into_name = self.map_name(from_name);
            let name_str = self.from.name_world_key(from_name);
            let into = into_world
                .imports
                .get(&into_name)
                .ok_or_else(|| anyhow!("import `{name_str}` not found in target world"))?;
            self.match_world_item(from, into)
                .with_context(|| format!("import `{name_str}` didn't match target world"))?;
        }

        for (from_name, from) in from_world.exports.iter() {
            let into_name = self.map_name(from_name);
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

    fn map_name(&self, from_name: &WorldKey) -> WorldKey {
        match from_name {
            WorldKey::Name(s) => WorldKey::Name(s.clone()),
            WorldKey::Interface(id) => {
                WorldKey::Interface(self.interface_map.get(id).copied().unwrap_or(*id))
            }
        }
    }

    fn match_world_item(&mut self, from: &WorldItem, into: &WorldItem) -> Result<()> {
        match (from, into) {
            (WorldItem::Interface(from), WorldItem::Interface(into)) => {
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

            (WorldItem::Interface(_), _)
            | (WorldItem::Function(_), _)
            | (WorldItem::Type(_), _) => {
                bail!("world items do not have the same type")
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{PackageId, Resolve};

    #[test]
    fn select_world() {
        let mut resolve = Resolve::default();
        parse_into(
            &mut resolve,
            r#"
                package foo:bar@0.1.0;

                world foo {}
            "#,
        );
        parse_into(
            &mut resolve,
            r#"
                package foo:baz@0.1.0;

                world foo {}
            "#,
        );
        parse_into(
            &mut resolve,
            r#"
                package foo:baz@0.2.0;

                world foo {}
            "#,
        );

        let dummy = parse_into(
            &mut resolve,
            r#"
                package foo:dummy;

                world foo {}
            "#,
        );

        assert!(resolve.select_world(dummy, None).is_ok());
        assert!(resolve.select_world(dummy, Some("xx")).is_err());
        assert!(resolve.select_world(dummy, Some("")).is_err());
        assert!(resolve.select_world(dummy, Some("foo:bar/foo")).is_ok());
        assert!(resolve
            .select_world(dummy, Some("foo:bar/foo@0.1.0"))
            .is_ok());
        assert!(resolve.select_world(dummy, Some("foo:baz/foo")).is_err());
        assert!(resolve
            .select_world(dummy, Some("foo:baz/foo@0.1.0"))
            .is_ok());
        assert!(resolve
            .select_world(dummy, Some("foo:baz/foo@0.2.0"))
            .is_ok());
    }

    fn parse_into(resolve: &mut Resolve, wit: &str) -> PackageId {
        let pkg = crate::UnresolvedPackage::parse("input.wit".as_ref(), wit).unwrap();
        resolve.push(pkg).unwrap()
    }
}
