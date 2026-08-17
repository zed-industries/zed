use crate::{
    Docs, InterfaceId, PackageId, Resolve, TypeDefKind, TypeId, WorldId, WorldItem, WorldKey,
};
use anyhow::{bail, Result};
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

type StringMap<V> = IndexMap<String, V>;

#[cfg(feature = "serde")]
const PACKAGE_DOCS_SECTION_VERSION: u8 = 0;

/// Represents serializable doc comments parsed from a WIT package.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct PackageDocs {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    worlds: StringMap<WorldDocs>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    interfaces: StringMap<InterfaceDocs>,
}

impl PackageDocs {
    pub const SECTION_NAME: &'static str = "package-docs";

    /// Extract package docs for the given package.
    pub fn extract(resolve: &Resolve, package: PackageId) -> Self {
        let package = &resolve.packages[package];

        let worlds = package
            .worlds
            .iter()
            .map(|(name, id)| (name.to_string(), WorldDocs::extract(resolve, *id)))
            .filter(|(_, item)| !item.is_empty())
            .collect();
        let interfaces = package
            .interfaces
            .iter()
            .map(|(name, id)| (name.to_string(), InterfaceDocs::extract(resolve, *id)))
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
        // Version byte (0), followed by JSON encoding of docs
        let mut data = vec![PACKAGE_DOCS_SECTION_VERSION];
        serde_json::to_writer(&mut data, self)?;
        Ok(data)
    }

    /// Decode package docs from package-docs custom section content.
    #[cfg(feature = "serde")]
    pub fn decode(data: &[u8]) -> Result<Self> {
        let version = data.first();
        if version != Some(&PACKAGE_DOCS_SECTION_VERSION) {
            bail!("expected package-docs version {PACKAGE_DOCS_SECTION_VERSION}, got {version:?}");
        }
        Ok(serde_json::from_slice(&data[1..])?)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct WorldDocs {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    interfaces: StringMap<InterfaceDocs>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    types: StringMap<TypeDocs>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    funcs: StringMap<String>,
}

impl WorldDocs {
    fn extract(resolve: &Resolve, id: WorldId) -> Self {
        let world = &resolve.worlds[id];

        let mut interfaces = StringMap::default();
        let mut types = StringMap::default();
        let mut funcs = StringMap::default();

        // Iterate over all imports and exports, extracting documented items
        for (key, item) in world.imports.iter().chain(&world.exports) {
            if let WorldKey::Name(name) = key {
                match item {
                    WorldItem::Interface(id) => {
                        let docs = InterfaceDocs::extract(resolve, *id);
                        if !docs.is_empty() {
                            interfaces.insert(name.to_string(), docs);
                        }
                    }
                    WorldItem::Type(id) => {
                        let docs = TypeDocs::extract(resolve, *id);
                        if !docs.is_empty() {
                            types.insert(name.to_string(), docs);
                        }
                    }
                    WorldItem::Function(f) => {
                        if let Some(docs) = f.docs.contents.as_deref() {
                            funcs.insert(name.to_string(), docs.to_string());
                        }
                    }
                }
            }
        }

        Self {
            docs: world.docs.contents.clone(),
            interfaces,
            types,
            funcs,
        }
    }

    fn inject(&self, resolve: &mut Resolve, id: WorldId) -> Result<()> {
        for (name, docs) in &self.interfaces {
            let key = WorldKey::Name(name.to_string());
            let Some(WorldItem::Interface(id)) = resolve.worlds[id]
                .imports
                .get(&key)
                .or_else(|| resolve.worlds[id].exports.get(&key))
            else {
                bail!("missing interface {name:?}");
            };
            docs.inject(resolve, *id)?;
        }
        for (name, docs) in &self.types {
            let key = WorldKey::Name(name.to_string());
            let Some(WorldItem::Type(id)) = resolve.worlds[id]
                .imports
                .get(&key)
                .or_else(|| resolve.worlds[id].exports.get(&key))
            else {
                bail!("missing type {name:?}");
            };
            docs.inject(resolve, *id)?;
        }
        let world = &mut resolve.worlds[id];
        for (name, docs) in &self.funcs {
            let key = WorldKey::Name(name.to_string());
            if let Some(WorldItem::Function(f)) = world.exports.get_mut(&key) {
                f.docs.contents = Some(docs.to_string())
            } else if let Some(WorldItem::Function(f)) = world.imports.get_mut(&key) {
                f.docs.contents = Some(docs.to_string())
            } else {
                bail!("missing func {name:?}");
            };
        }
        if let Some(docs) = &self.docs {
            world.docs.contents = Some(docs.to_string());
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.docs.is_none()
            && self.interfaces.is_empty()
            && self.types.is_empty()
            && self.funcs.is_empty()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct InterfaceDocs {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    funcs: StringMap<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    types: StringMap<TypeDocs>,
}

impl InterfaceDocs {
    fn extract(resolve: &Resolve, id: InterfaceId) -> Self {
        let interface = &resolve.interfaces[id];

        let funcs = interface
            .functions
            .iter()
            .flat_map(|(name, func)| Some((name.to_string(), func.docs.contents.clone()?)))
            .collect();
        let types = interface
            .types
            .iter()
            .map(|(name, id)| (name.to_string(), TypeDocs::extract(resolve, *id)))
            .filter(|(_, item)| !item.is_empty())
            .collect();

        Self {
            docs: interface.docs.contents.clone(),
            funcs,
            types,
        }
    }

    fn inject(&self, resolve: &mut Resolve, id: InterfaceId) -> Result<()> {
        for (name, docs) in &self.types {
            let Some(&id) = resolve.interfaces[id].types.get(name) else {
                bail!("missing type {name:?}");
            };
            docs.inject(resolve, id)?;
        }
        let interface = &mut resolve.interfaces[id];
        for (name, docs) in &self.funcs {
            let Some(f) = interface.functions.get_mut(name) else {
                bail!("missing func {name:?}");
            };
            f.docs.contents = Some(docs.to_string());
        }
        if let Some(docs) = &self.docs {
            interface.docs.contents = Some(docs.to_string());
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.docs.is_none() && self.funcs.is_empty() && self.types.is_empty()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct TypeDocs {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    docs: Option<String>,
    // record fields, variant cases, etc.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "StringMap::is_empty")
    )]
    items: StringMap<String>,
}

impl TypeDocs {
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
        self.docs.is_none() && self.items.is_empty()
    }
}
