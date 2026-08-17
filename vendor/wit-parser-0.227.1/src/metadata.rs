//! Implementation of encoding/decoding package metadata (docs/stability) in a
//! custom section.
//!
//! This module contains the particulars for how this custom section is encoded
//! and decoded at this time. As of the time of this writing the component model
//! binary format does not have any means of storing documentation and/or item
//! stability inline with items themselves. These are important to preserve when
//! round-tripping WIT through the WebAssembly binary format, however, so this
//! module implements this with a custom section.
//!
//! The custom section, named `SECTION_NAME`, is stored within the component
//! that encodes a WIT package. This section is itself JSON-encoded with a small
//! version header to help forwards/backwards compatibility. The hope is that
//! one day this custom section will be obsoleted by extensions to the binary
//! format to store this information inline.

use crate::{
    Docs, Function, InterfaceId, PackageId, Resolve, Stability, TypeDefKind, TypeId, WorldId,
    WorldItem, WorldKey,
};
use anyhow::{bail, Result};
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

type StringMap<V> = IndexMap<String, V>;

/// Current supported format of the custom section.
///
/// This byte is a prefix byte intended to be a general version marker for the
/// entire custom section. This is bumped when backwards-incompatible changes
/// are made to prevent older implementations from loading newer versions.
///
/// The history of this is:
///
/// * [????/??/??] 0 - the original format added
/// * [2024/04/19] 1 - extensions were added for item stability and
///   additionally having world imports/exports have the same name.
#[cfg(feature = "serde")]
const PACKAGE_DOCS_SECTION_VERSION: u8 = 1;

/// At this time the v1 format was just written. For compatibility with older
/// tools we'll still try to emit the v0 format by default, if the input is
/// compatible. This will be turned off in the future once enough published
/// versions support the v1 format.
const TRY_TO_EMIT_V0_BY_DEFAULT: bool = true;

/// Represents serializable doc comments parsed from a WIT package.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct PackageMetadata {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    worlds: StringMap<WorldMetadata>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    interfaces: StringMap<InterfaceMetadata>,
}

impl PackageMetadata {
    pub const SECTION_NAME: &'static str = "package-docs";

    /// Extract package docs for the given package.
    pub fn extract(resolve: &Resolve, package: PackageId) -> Self {
        let package = &resolve.packages[package];

        let worlds = package
            .worlds
            .iter()
            .map(|(name, id)| (name.to_string(), WorldMetadata::extract(resolve, *id)))
            .filter(|(_, item)| !item.is_empty())
            .collect();
        let interfaces = package
            .interfaces
            .iter()
            .map(|(name, id)| (name.to_string(), InterfaceMetadata::extract(resolve, *id)))
            .filter(|(_, item)| !item.is_empty())
            .collect();

        Self {
            docs: package.docs.contents.as_deref().map(Into::into),
            worlds,
            interfaces,
        }
    }

    /// Inject package docs for the given package.
    ///
    /// This will override any existing docs in the [`Resolve`].
    pub fn inject(&self, resolve: &mut Resolve, package: PackageId) -> Result<()> {
        for (name, docs) in &self.worlds {
            let Some(&id) = resolve.packages[package].worlds.get(name) else {
                bail!("missing world {name:?}");
            };
            docs.inject(resolve, id)?;
        }
        for (name, docs) in &self.interfaces {
            let Some(&id) = resolve.packages[package].interfaces.get(name) else {
                bail!("missing interface {name:?}");
            };
            docs.inject(resolve, id)?;
        }
        if let Some(docs) = &self.docs {
            resolve.packages[package].docs.contents = Some(docs.to_string());
        }
        Ok(())
    }

    /// Encode package docs as a package-docs custom section.
    #[cfg(feature = "serde")]
    pub fn encode(&self) -> Result<Vec<u8>> {
        // Version byte, followed by JSON encoding of docs.
        //
        // Note that if this document is compatible with the v0 format then
        // that's preferred to keep older tools working at this time.
        // Eventually this branch will be removed and v1 will unconditionally
        // be used.
        let mut data = vec![
            if TRY_TO_EMIT_V0_BY_DEFAULT && self.is_compatible_with_v0() {
                0
            } else {
                PACKAGE_DOCS_SECTION_VERSION
            },
        ];
        serde_json::to_writer(&mut data, self)?;
        Ok(data)
    }

    /// Decode package docs from package-docs custom section content.
    #[cfg(feature = "serde")]
    pub fn decode(data: &[u8]) -> Result<Self> {
        match data.first().copied() {
            // Our serde structures transparently support v0 and the current
            // version, so allow either here.
            Some(0) | Some(PACKAGE_DOCS_SECTION_VERSION) => {}
            version => {
                bail!(
                    "expected package-docs version {PACKAGE_DOCS_SECTION_VERSION}, got {version:?}"
                );
            }
        }
        Ok(serde_json::from_slice(&data[1..])?)
    }

    #[cfg(feature = "serde")]
    fn is_compatible_with_v0(&self) -> bool {
        self.worlds.iter().all(|(_, w)| w.is_compatible_with_v0())
            && self
                .interfaces
                .iter()
                .all(|(_, w)| w.is_compatible_with_v0())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct WorldMetadata {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Stability::is_unknown")
    )]
    stability: Stability,

    /// Metadata for named interface, e.g.:
    ///
    /// ```wit
    /// world foo {
    ///     import x: interface {}
    /// }
    /// ```
    ///
    /// In the v0 format this was called "interfaces", hence the
    /// `serde(rename)`. When support was originally added here imports/exports
    /// could not overlap in their name, but now they can. This map has thus
    /// been repurposed as:
    ///
    /// * If an interface is imported, it goes here.
    /// * If an interface is exported, and no interface was imported with the
    ///   same name, it goes here.
    ///
    /// Otherwise exports go inside the `interface_exports` map.
    ///
    /// In the future when v0 support is dropped this should become only
    /// imports, not either imports-or-exports.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            rename = "interfaces",
            skip_serializing_if = "StringMap::is_empty"
        )
    )]
    interface_imports_or_exports: StringMap<InterfaceMetadata>,

    /// All types in this interface.
    ///
    /// Note that at this time types are only imported, never exported.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    types: StringMap<TypeMetadata>,

    /// Same as `interface_imports_or_exports`, but for functions.
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "funcs", skip_serializing_if = "StringMap::is_empty")
    )]
    func_imports_or_exports: StringMap<FunctionMetadata>,

    /// The "export half" of `interface_imports_or_exports`.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    interface_exports: StringMap<InterfaceMetadata>,

    /// The "export half" of `func_imports_or_exports`.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    func_exports: StringMap<FunctionMetadata>,

    /// Stability annotations for interface imports that aren't inline, for
    /// example:
    ///
    /// ```wit
    /// world foo {
    ///     @since(version = 1.0.0)
    ///     import an-interface;
    /// }
    /// ```
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    interface_import_stability: StringMap<Stability>,

    /// Same as `interface_import_stability`, but for exports.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    interface_export_stability: StringMap<Stability>,
}

impl WorldMetadata {
    fn extract(resolve: &Resolve, id: WorldId) -> Self {
        let world = &resolve.worlds[id];

        let mut interface_imports_or_exports = StringMap::default();
        let mut types = StringMap::default();
        let mut func_imports_or_exports = StringMap::default();
        let mut interface_exports = StringMap::default();
        let mut func_exports = StringMap::default();
        let mut interface_import_stability = StringMap::default();
        let mut interface_export_stability = StringMap::default();

        for ((key, item), import) in world
            .imports
            .iter()
            .map(|p| (p, true))
            .chain(world.exports.iter().map(|p| (p, false)))
        {
            match key {
                // For all named imports with kebab-names extract their
                // docs/stability and insert it into one of our maps.
                WorldKey::Name(name) => match item {
                    WorldItem::Interface { id, .. } => {
                        let data = InterfaceMetadata::extract(resolve, *id);
                        if data.is_empty() {
                            continue;
                        }
                        let map = if import {
                            &mut interface_imports_or_exports
                        } else if !TRY_TO_EMIT_V0_BY_DEFAULT
                            || interface_imports_or_exports.contains_key(name)
                        {
                            &mut interface_exports
                        } else {
                            &mut interface_imports_or_exports
                        };
                        let prev = map.insert(name.to_string(), data);
                        assert!(prev.is_none());
                    }
                    WorldItem::Type(id) => {
                        let data = TypeMetadata::extract(resolve, *id);
                        if !data.is_empty() {
                            types.insert(name.to_string(), data);
                        }
                    }
                    WorldItem::Function(f) => {
                        let data = FunctionMetadata::extract(f);
                        if data.is_empty() {
                            continue;
                        }
                        let map = if import {
                            &mut func_imports_or_exports
                        } else if !TRY_TO_EMIT_V0_BY_DEFAULT
                            || func_imports_or_exports.contains_key(name)
                        {
                            &mut func_exports
                        } else {
                            &mut func_imports_or_exports
                        };
                        let prev = map.insert(name.to_string(), data);
                        assert!(prev.is_none());
                    }
                },

                // For interface imports/exports extract the stability and
                // record it if necessary.
                WorldKey::Interface(_) => {
                    let stability = match item {
                        WorldItem::Interface { stability, .. } => stability,
                        _ => continue,
                    };
                    if stability.is_unknown() {
                        continue;
                    }

                    let map = if import {
                        &mut interface_import_stability
                    } else {
                        &mut interface_export_stability
                    };
                    let name = resolve.name_world_key(key);
                    map.insert(name, stability.clone());
                }
            }
        }

        Self {
            docs: world.docs.contents.clone(),
            stability: world.stability.clone(),
            interface_imports_or_exports,
            types,
            func_imports_or_exports,
            interface_exports,
            func_exports,
            interface_import_stability,
            interface_export_stability,
        }
    }

    fn inject(&self, resolve: &mut Resolve, id: WorldId) -> Result<()> {
        // Inject docs/stability for all kebab-named interfaces, both imports
        // and exports.
        for ((name, data), only_export) in self
            .interface_imports_or_exports
            .iter()
            .map(|p| (p, false))
            .chain(self.interface_exports.iter().map(|p| (p, true)))
        {
            let key = WorldKey::Name(name.to_string());
            let world = &mut resolve.worlds[id];

            let item = if only_export {
                world.exports.get_mut(&key)
            } else {
                match world.imports.get_mut(&key) {
                    Some(item) => Some(item),
                    None => world.exports.get_mut(&key),
                }
            };
            let Some(WorldItem::Interface { id, stability }) = item else {
                bail!("missing interface {name:?}");
            };
            *stability = data.stability.clone();
            let id = *id;
            data.inject(resolve, id)?;
        }

        // Process all types, which are always imported, for this world.
        for (name, data) in &self.types {
            let key = WorldKey::Name(name.to_string());
            let Some(WorldItem::Type(id)) = resolve.worlds[id].imports.get(&key) else {
                bail!("missing type {name:?}");
            };
            data.inject(resolve, *id)?;
        }

        // Build a map of `name_world_key` for interface imports/exports to the
        // actual key. This map is then consluted in the next loop.
        let world = &resolve.worlds[id];
        let stabilities = world
            .imports
            .iter()
            .map(|i| (i, true))
            .chain(world.exports.iter().map(|i| (i, false)))
            .filter_map(|((key, item), import)| match item {
                WorldItem::Interface { .. } => {
                    Some(((resolve.name_world_key(key), import), key.clone()))
                }
                _ => None,
            })
            .collect::<IndexMap<_, _>>();

        let world = &mut resolve.worlds[id];

        // Update the stability of an interface imports/exports that aren't
        // kebab-named.
        for ((name, stability), import) in self
            .interface_import_stability
            .iter()
            .map(|p| (p, true))
            .chain(self.interface_export_stability.iter().map(|p| (p, false)))
        {
            let key = match stabilities.get(&(name.clone(), import)) {
                Some(key) => key.clone(),
                None => bail!("missing interface `{name}`"),
            };
            let item = if import {
                world.imports.get_mut(&key)
            } else {
                world.exports.get_mut(&key)
            };
            match item {
                Some(WorldItem::Interface { stability: s, .. }) => *s = stability.clone(),
                _ => bail!("item `{name}` wasn't an interface"),
            }
        }

        // Update the docs/stability of all functions imported/exported from
        // this world.
        for ((name, data), only_export) in self
            .func_imports_or_exports
            .iter()
            .map(|p| (p, false))
            .chain(self.func_exports.iter().map(|p| (p, true)))
        {
            let key = WorldKey::Name(name.to_string());
            let item = if only_export {
                world.exports.get_mut(&key)
            } else {
                match world.imports.get_mut(&key) {
                    Some(item) => Some(item),
                    None => world.exports.get_mut(&key),
                }
            };
            match item {
                Some(WorldItem::Function(f)) => data.inject(f)?,
                _ => bail!("missing func {name:?}"),
            }
        }
        if let Some(docs) = &self.docs {
            world.docs.contents = Some(docs.to_string());
        }
        world.stability = self.stability.clone();
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.docs.is_none()
            && self.interface_imports_or_exports.is_empty()
            && self.types.is_empty()
            && self.func_imports_or_exports.is_empty()
            && self.stability.is_unknown()
            && self.interface_exports.is_empty()
            && self.func_exports.is_empty()
            && self.interface_import_stability.is_empty()
            && self.interface_export_stability.is_empty()
    }

    #[cfg(feature = "serde")]
    fn is_compatible_with_v0(&self) -> bool {
        self.stability.is_unknown()
            && self
                .interface_imports_or_exports
                .iter()
                .all(|(_, w)| w.is_compatible_with_v0())
            && self
                .func_imports_or_exports
                .iter()
                .all(|(_, w)| w.is_compatible_with_v0())
            && self.types.iter().all(|(_, w)| w.is_compatible_with_v0())
            // These maps weren't present in v0, so we're only compatible if
            // they're empty.
            && self.interface_exports.is_empty()
            && self.func_exports.is_empty()
            && self.interface_import_stability.is_empty()
            && self.interface_export_stability.is_empty()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct InterfaceMetadata {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Stability::is_unknown")
    )]
    stability: Stability,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    funcs: StringMap<FunctionMetadata>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    types: StringMap<TypeMetadata>,
}

impl InterfaceMetadata {
    fn extract(resolve: &Resolve, id: InterfaceId) -> Self {
        let interface = &resolve.interfaces[id];

        let funcs = interface
            .functions
            .iter()
            .map(|(name, func)| (name.to_string(), FunctionMetadata::extract(func)))
            .filter(|(_, item)| !item.is_empty())
            .collect();
        let types = interface
            .types
            .iter()
            .map(|(name, id)| (name.to_string(), TypeMetadata::extract(resolve, *id)))
            .filter(|(_, item)| !item.is_empty())
            .collect();

        Self {
            docs: interface.docs.contents.clone(),
            stability: interface.stability.clone(),
            funcs,
            types,
        }
    }

    fn inject(&self, resolve: &mut Resolve, id: InterfaceId) -> Result<()> {
        for (name, data) in &self.types {
            let Some(&id) = resolve.interfaces[id].types.get(name) else {
                bail!("missing type {name:?}");
            };
            data.inject(resolve, id)?;
        }
        let interface = &mut resolve.interfaces[id];
        for (name, data) in &self.funcs {
            let Some(f) = interface.functions.get_mut(name) else {
                bail!("missing func {name:?}");
            };
            data.inject(f)?;
        }
        if let Some(docs) = &self.docs {
            interface.docs.contents = Some(docs.to_string());
        }
        interface.stability = self.stability.clone();
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.docs.is_none()
            && self.funcs.is_empty()
            && self.types.is_empty()
            && self.stability.is_unknown()
    }

    #[cfg(feature = "serde")]
    fn is_compatible_with_v0(&self) -> bool {
        self.stability.is_unknown()
            && self.funcs.iter().all(|(_, w)| w.is_compatible_with_v0())
            && self.types.iter().all(|(_, w)| w.is_compatible_with_v0())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, deny_unknown_fields))]
enum FunctionMetadata {
    /// In the v0 format function metadata was only a string so this variant
    /// is preserved for the v0 format. In the future this can be removed
    /// entirely in favor of just the below struct variant.
    ///
    /// Note that this is an untagged enum so the name `JustDocs` is just for
    /// rust.
    JustDocs(Option<String>),

    /// In the v1+ format we're tracking at least docs but also the stability
    /// of functions.
    DocsAndStabilty {
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        docs: Option<String>,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Stability::is_unknown")
        )]
        stability: Stability,
    },
}

impl FunctionMetadata {
    fn extract(func: &Function) -> Self {
        if TRY_TO_EMIT_V0_BY_DEFAULT && func.stability.is_unknown() {
            FunctionMetadata::JustDocs(func.docs.contents.clone())
        } else {
            FunctionMetadata::DocsAndStabilty {
                docs: func.docs.contents.clone(),
                stability: func.stability.clone(),
            }
        }
    }

    fn inject(&self, func: &mut Function) -> Result<()> {
        match self {
            FunctionMetadata::JustDocs(docs) => {
                func.docs.contents = docs.clone();
            }
            FunctionMetadata::DocsAndStabilty { docs, stability } => {
                func.docs.contents = docs.clone();
                func.stability = stability.clone();
            }
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        match self {
            FunctionMetadata::JustDocs(docs) => docs.is_none(),
            FunctionMetadata::DocsAndStabilty { docs, stability } => {
                docs.is_none() && stability.is_unknown()
            }
        }
    }

    #[cfg(feature = "serde")]
    fn is_compatible_with_v0(&self) -> bool {
        match self {
            FunctionMetadata::JustDocs(_) => true,
            FunctionMetadata::DocsAndStabilty { .. } => false,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct TypeMetadata {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Stability::is_unknown")
    )]
    stability: Stability,
    // record fields, variant cases, etc.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    items: StringMap<String>,
}

impl TypeMetadata {
    fn extract(resolve: &Resolve, id: TypeId) -> Self {
        fn extract_items<T>(items: &[T], f: impl Fn(&T) -> (&String, &Docs)) -> StringMap<String> {
            items
                .iter()
                .flat_map(|item| {
                    let (name, docs) = f(item);
                    Some((name.to_string(), docs.contents.clone()?))
                })
                .collect()
        }
        let ty = &resolve.types[id];
        let items = match &ty.kind {
            TypeDefKind::Record(record) => {
                extract_items(&record.fields, |item| (&item.name, &item.docs))
            }
            TypeDefKind::Flags(flags) => {
                extract_items(&flags.flags, |item| (&item.name, &item.docs))
            }
            TypeDefKind::Variant(variant) => {
                extract_items(&variant.cases, |item| (&item.name, &item.docs))
            }
            TypeDefKind::Enum(enum_) => {
                extract_items(&enum_.cases, |item| (&item.name, &item.docs))
            }
            // other types don't have inner items
            _ => IndexMap::default(),
        };

        Self {
            docs: ty.docs.contents.clone(),
            stability: ty.stability.clone(),
            items,
        }
    }

    fn inject(&self, resolve: &mut Resolve, id: TypeId) -> Result<()> {
        let ty = &mut resolve.types[id];
        if !self.items.is_empty() {
            match &mut ty.kind {
                TypeDefKind::Record(record) => {
                    self.inject_items(&mut record.fields, |item| (&item.name, &mut item.docs))?
                }
                TypeDefKind::Flags(flags) => {
                    self.inject_items(&mut flags.flags, |item| (&item.name, &mut item.docs))?
                }
                TypeDefKind::Variant(variant) => {
                    self.inject_items(&mut variant.cases, |item| (&item.name, &mut item.docs))?
                }
                TypeDefKind::Enum(enum_) => {
                    self.inject_items(&mut enum_.cases, |item| (&item.name, &mut item.docs))?
                }
                _ => {
                    bail!("got 'items' for unexpected type {ty:?}");
                }
            }
        }
        if let Some(docs) = &self.docs {
            ty.docs.contents = Some(docs.to_string());
        }
        ty.stability = self.stability.clone();
        Ok(())
    }

    fn inject_items<T: std::fmt::Debug>(
        &self,
        items: &mut [T],
        f: impl Fn(&mut T) -> (&String, &mut Docs),
    ) -> Result<()> {
        let mut unused_docs = self.items.len();
        for item in items.iter_mut() {
            let (name, item_docs) = f(item);
            if let Some(docs) = self.items.get(name.as_str()) {
                item_docs.contents = Some(docs.to_string());
                unused_docs -= 1;
            }
        }
        if unused_docs > 0 {
            bail!(
                "not all 'items' match type items; {item_docs:?} vs {items:?}",
                item_docs = self.items
            );
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.docs.is_none() && self.items.is_empty() && self.stability.is_unknown()
    }

    #[cfg(feature = "serde")]
    fn is_compatible_with_v0(&self) -> bool {
        self.stability.is_unknown()
    }
}
