use crate::*;
use anyhow::{anyhow, bail};
use indexmap::IndexSet;
use std::mem;
use std::{collections::HashMap, io::Read};
use wasmparser::Chunk;
use wasmparser::{
    ComponentExternalKind, Parser, Payload, PrimitiveValType, ValidPayload, Validator,
    WasmFeatures,
    component_types::{
        ComponentAnyTypeId, ComponentDefinedType, ComponentEntityType, ComponentFuncType,
        ComponentInstanceType, ComponentType, ComponentValType,
    },
    names::{ComponentName, ComponentNameKind},
    types,
    types::Types,
};

/// Represents information about a decoded WebAssembly component.
struct ComponentInfo {
    /// Wasmparser-defined type information learned after a component is fully
    /// validated.
    types: types::Types,
    /// List of all imports and exports from this component.
    externs: Vec<(String, Extern)>,
    /// Decoded package metadata
    package_metadata: Option<PackageMetadata>,
}

struct DecodingExport {
    name: String,
    kind: ComponentExternalKind,
    index: u32,
}

enum Extern {
    Import(String),
    Export(DecodingExport),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WitEncodingVersion {
    V1,
    V2,
}

impl ComponentInfo {
    /// Creates a new component info by parsing the given WebAssembly component bytes.

    fn from_reader(mut reader: impl Read) -> Result<Self> {
        let mut validator = Validator::new_with_features(WasmFeatures::all());
        let mut externs = Vec::new();
        let mut depth = 1;
        let mut types = None;
        let mut _package_metadata = None;
        let mut cur = Parser::new(0);
        let mut eof = false;
        let mut stack = Vec::new();
        let mut buffer = Vec::new();

        loop {
            let chunk = cur.parse(&buffer, eof)?;
            let (payload, consumed) = match chunk {
                Chunk::NeedMoreData(hint) => {
                    assert!(!eof); // otherwise an error would be returned

                    // Use the hint to preallocate more space, then read
                    // some more data into our buffer.
                    //
                    // Note that the buffer management here is not ideal,
                    // but it's compact enough to fit in an example!
                    let len = buffer.len();
                    buffer.extend((0..hint).map(|_| 0u8));
                    let n = reader.read(&mut buffer[len..])?;
                    buffer.truncate(len + n);
                    eof = n == 0;
                    continue;
                }

                Chunk::Parsed { consumed, payload } => (payload, consumed),
            };
            match validator.payload(&payload)? {
                ValidPayload::Ok => {}
                ValidPayload::Parser(_) => depth += 1,
                ValidPayload::End(t) => {
                    depth -= 1;
                    if depth == 0 {
                        types = Some(t);
                    }
                }
                ValidPayload::Func(..) => {}
            }

            match payload {
                Payload::ComponentImportSection(s) if depth == 1 => {
                    for import in s {
                        let import = import?;
                        externs.push((
                            import.name.0.to_string(),
                            Extern::Import(import.name.0.to_string()),
                        ));
                    }
                }
                Payload::ComponentExportSection(s) if depth == 1 => {
                    for export in s {
                        let export = export?;
                        externs.push((
                            export.name.0.to_string(),
                            Extern::Export(DecodingExport {
                                name: export.name.0.to_string(),
                                kind: export.kind,
                                index: export.index,
                            }),
                        ));
                    }
                }
                #[cfg(feature = "serde")]
                Payload::CustomSection(s) if s.name() == PackageMetadata::SECTION_NAME => {
                    if _package_metadata.is_some() {
                        bail!("multiple {:?} sections", PackageMetadata::SECTION_NAME);
                    }
                    _package_metadata = Some(PackageMetadata::decode(s.data())?);
                }
                Payload::ModuleSection { parser, .. }
                | Payload::ComponentSection { parser, .. } => {
                    stack.push(cur.clone());
                    cur = parser.clone();
                }
                Payload::End(_) => {
                    if let Some(parent_parser) = stack.pop() {
                        cur = parent_parser.clone();
                    } else {
                        break;
                    }
                }
                _ => {}
            }

            // once we're done processing the payload we can forget the
            // original.
            buffer.drain(..consumed);
        }

        Ok(Self {
            types: types.unwrap(),
            externs,
            package_metadata: _package_metadata,
        })
    }

    fn is_wit_package(&self) -> Option<WitEncodingVersion> {
        // all wit package exports must be component types, and there must be at
        // least one
        if self.externs.is_empty() {
            return None;
        }

        if !self.externs.iter().all(|(_, item)| {
            let export = match item {
                Extern::Export(e) => e,
                _ => return false,
            };
            match export.kind {
                ComponentExternalKind::Type => matches!(
                    self.types.as_ref().component_any_type_at(export.index),
                    ComponentAnyTypeId::Component(_)
                ),
                _ => false,
            }
        }) {
            return None;
        }

        // The distinction between v1 and v2 encoding formats is the structure of the export
        // strings for each component. The v1 format uses "<namespace>:<package>/wit" as the name
        // for the top-level exports, while the v2 format uses the unqualified name of the encoded
        // entity.
        match ComponentName::new(&self.externs[0].0, 0).ok()?.kind() {
            ComponentNameKind::Interface(name) if name.interface().as_str() == "wit" => {
                Some(WitEncodingVersion::V1)
            }
            ComponentNameKind::Label(_) => Some(WitEncodingVersion::V2),
            _ => None,
        }
    }

    fn decode_wit_v1_package(&self) -> Result<(Resolve, PackageId)> {
        let mut decoder = WitPackageDecoder::new(&self.types);

        let mut pkg = None;
        for (name, item) in self.externs.iter() {
            let export = match item {
                Extern::Export(e) => e,
                _ => unreachable!(),
            };
            let id = self.types.as_ref().component_type_at(export.index);
            let ty = &self.types[id];
            if pkg.is_some() {
                bail!("more than one top-level exported component type found");
            }
            let name = ComponentName::new(name, 0).unwrap();
            pkg = Some(
                decoder
                    .decode_v1_package(&name, ty)
                    .with_context(|| format!("failed to decode document `{name}`"))?,
            );
        }

        let pkg = pkg.ok_or_else(|| anyhow!("no exported component type found"))?;
        let (mut resolve, package) = decoder.finish(pkg);
        if let Some(package_metadata) = &self.package_metadata {
            package_metadata.inject(&mut resolve, package)?;
        }
        Ok((resolve, package))
    }

    fn decode_wit_v2_package(&self) -> Result<(Resolve, PackageId)> {
        let mut decoder = WitPackageDecoder::new(&self.types);

        let mut pkg_name = None;

        let mut interfaces = IndexMap::new();
        let mut worlds = IndexMap::new();
        let mut fields = PackageFields {
            interfaces: &mut interfaces,
            worlds: &mut worlds,
        };

        for (_, item) in self.externs.iter() {
            let export = match item {
                Extern::Export(e) => e,
                _ => unreachable!(),
            };

            let index = export.index;
            let id = self.types.as_ref().component_type_at(index);
            let component = &self.types[id];

            // The single export of this component will determine if it's a world or an interface:
            // worlds export a component, while interfaces export an instance.
            if component.exports.len() != 1 {
                bail!(
                    "Expected a single export, but found {} instead",
                    component.exports.len()
                );
            }

            let name = component.exports.keys().nth(0).unwrap();

            let name = match component.exports[name] {
                ComponentEntityType::Component(ty) => {
                    let package_name =
                        decoder.decode_world(name.as_str(), &self.types[ty], &mut fields)?;
                    package_name
                }
                ComponentEntityType::Instance(ty) => {
                    let package_name = decoder.decode_interface(
                        name.as_str(),
                        &component.imports,
                        &self.types[ty],
                        &mut fields,
                    )?;
                    package_name
                }
                _ => unreachable!(),
            };

            if let Some(pkg_name) = pkg_name.as_ref() {
                // TODO: when we have fully switched to the v2 format, we should switch to parsing
                // multiple wit documents instead of bailing.
                if pkg_name != &name {
                    bail!("item defined with mismatched package name")
                }
            } else {
                pkg_name.replace(name);
            }
        }

        let pkg = if let Some(name) = pkg_name {
            Package {
                name,
                docs: Docs::default(),
                interfaces,
                worlds,
            }
        } else {
            bail!("no exported component type found");
        };

        let (mut resolve, package) = decoder.finish(pkg);
        if let Some(package_metadata) = &self.package_metadata {
            package_metadata.inject(&mut resolve, package)?;
        }
        Ok((resolve, package))
    }

    fn decode_component(&self) -> Result<(Resolve, WorldId)> {
        assert!(self.is_wit_package().is_none());
        let mut decoder = WitPackageDecoder::new(&self.types);
        // Note that this name is arbitrarily chosen. We may one day perhaps
        // want to encode this in the component binary format itself, but for
        // now it shouldn't be an issue to have a defaulted name here.
        let world_name = "root";
        let world = decoder.resolve.worlds.alloc(World {
            name: world_name.to_string(),
            docs: Default::default(),
            imports: Default::default(),
            exports: Default::default(),
            package: None,
            includes: Default::default(),
            include_names: Default::default(),
            stability: Default::default(),
        });
        let mut package = Package {
            // Similar to `world_name` above this is arbitrarily chosen as it's
            // not otherwise encoded in a binary component. This theoretically
            // shouldn't cause issues, however.
            name: PackageName {
                namespace: "root".to_string(),
                version: None,
                name: "component".to_string(),
            },
            docs: Default::default(),
            worlds: [(world_name.to_string(), world)].into_iter().collect(),
            interfaces: Default::default(),
        };

        let mut fields = PackageFields {
            worlds: &mut package.worlds,
            interfaces: &mut package.interfaces,
        };

        for (_name, item) in self.externs.iter() {
            match item {
                Extern::Import(import) => {
                    decoder.decode_component_import(import, world, &mut fields)?
                }
                Extern::Export(export) => {
                    decoder.decode_component_export(export, world, &mut fields)?
                }
            }
        }

        let (resolve, _) = decoder.finish(package);
        Ok((resolve, world))
    }
}

/// Result of the [`decode`] function.
pub enum DecodedWasm {
    /// The input to [`decode`] was one or more binary-encoded WIT package(s).
    ///
    /// The full resolve graph is here plus the identifier of the packages that
    /// were encoded. Note that other packages may be within the resolve if any
    /// of the main packages refer to other, foreign packages.
    WitPackage(Resolve, PackageId),

    /// The input to [`decode`] was a component and its interface is specified
    /// by the world here.
    Component(Resolve, WorldId),
}

impl DecodedWasm {
    /// Returns the [`Resolve`] for WIT types contained.
    pub fn resolve(&self) -> &Resolve {
        match self {
            DecodedWasm::WitPackage(resolve, _) => resolve,
            DecodedWasm::Component(resolve, _) => resolve,
        }
    }

    /// Returns the main packages of what was decoded.
    pub fn package(&self) -> PackageId {
        match self {
            DecodedWasm::WitPackage(_, id) => *id,
            DecodedWasm::Component(resolve, world) => resolve.worlds[*world].package.unwrap(),
        }
    }
}

/// Decode for incremental reading
pub fn decode_reader(reader: impl Read) -> Result<DecodedWasm> {
    let info = ComponentInfo::from_reader(reader)?;

    if let Some(version) = info.is_wit_package() {
        match version {
            WitEncodingVersion::V1 => {
                log::debug!("decoding a v1 WIT package encoded as wasm");
                let (resolve, pkg) = info.decode_wit_v1_package()?;
                Ok(DecodedWasm::WitPackage(resolve, pkg))
            }
            WitEncodingVersion::V2 => {
                log::debug!("decoding a v2 WIT package encoded as wasm");
                let (resolve, pkg) = info.decode_wit_v2_package()?;
                Ok(DecodedWasm::WitPackage(resolve, pkg))
            }
        }
    } else {
        log::debug!("inferring the WIT of a concrete component");
        let (resolve, world) = info.decode_component()?;
        Ok(DecodedWasm::Component(resolve, world))
    }
}

/// Decodes an in-memory WebAssembly binary into a WIT [`Resolve`] and
/// associated metadata.
///
/// The WebAssembly binary provided here can either be a
/// WIT-package-encoded-as-binary or an actual component itself. A [`Resolve`]
/// is always created and the return value indicates which was detected.
pub fn decode(bytes: &[u8]) -> Result<DecodedWasm> {
    decode_reader(bytes)
}

/// Decodes the single component type `world` specified as a WIT world.
///
/// The `world` should be an exported component type. The `world` must have been
/// previously created via `encode_world` meaning that it is a component that
/// itself imports nothing and exports a single component, and the single
/// component export represents the world. The name of the export is also the
/// name of the package/world/etc.
pub fn decode_world(wasm: &[u8]) -> Result<(Resolve, WorldId)> {
    let mut validator = Validator::new_with_features(WasmFeatures::all());
    let mut exports = Vec::new();
    let mut depth = 1;
    let mut types = None;

    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload?;

        match validator.payload(&payload)? {
            ValidPayload::Ok => {}
            ValidPayload::Parser(_) => depth += 1,
            ValidPayload::End(t) => {
                depth -= 1;
                if depth == 0 {
                    types = Some(t);
                }
            }
            ValidPayload::Func(..) => {}
        }

        match payload {
            Payload::ComponentExportSection(s) if depth == 1 => {
                for export in s {
                    exports.push(export?);
                }
            }
            _ => {}
        }
    }

    if exports.len() != 1 {
        bail!("expected one export in component");
    }
    if exports[0].kind != ComponentExternalKind::Type {
        bail!("expected an export of a type");
    }
    if exports[0].ty.is_some() {
        bail!("expected an un-ascribed exported type");
    }
    let types = types.as_ref().unwrap();
    let world = match types.as_ref().component_any_type_at(exports[0].index) {
        ComponentAnyTypeId::Component(c) => c,
        _ => bail!("expected an exported component type"),
    };

    let mut decoder = WitPackageDecoder::new(types);
    let mut interfaces = IndexMap::new();
    let mut worlds = IndexMap::new();
    let ty = &types[world];
    assert_eq!(ty.imports.len(), 0);
    assert_eq!(ty.exports.len(), 1);
    let name = ty.exports.keys().nth(0).unwrap();
    let ty = match ty.exports[0] {
        ComponentEntityType::Component(ty) => ty,
        _ => unreachable!(),
    };
    let name = decoder.decode_world(
        name,
        &types[ty],
        &mut PackageFields {
            interfaces: &mut interfaces,
            worlds: &mut worlds,
        },
    )?;
    let (resolve, pkg) = decoder.finish(Package {
        name,
        interfaces,
        worlds,
        docs: Default::default(),
    });
    // The package decoded here should only have a single world so extract that
    // here to return.
    let world = *resolve.packages[pkg].worlds.iter().next().unwrap().1;
    Ok((resolve, world))
}

struct PackageFields<'a> {
    interfaces: &'a mut IndexMap<String, InterfaceId>,
    worlds: &'a mut IndexMap<String, WorldId>,
}

struct WitPackageDecoder<'a> {
    resolve: Resolve,
    types: &'a Types,
    foreign_packages: IndexMap<String, Package>,
    iface_to_package_index: HashMap<InterfaceId, usize>,
    named_interfaces: HashMap<String, InterfaceId>,

    /// A map which tracks named resources to what their corresponding `TypeId`
    /// is. This first layer of key in this map is the owner scope of a
    /// resource, more-or-less the `world` or `interface` that it's defined
    /// within. The second layer of this map is keyed by name of the resource
    /// and points to the actual ID of the resource.
    ///
    /// This map is populated in `register_type_export`.
    resources: HashMap<TypeOwner, HashMap<String, TypeId>>,

    /// A map from a type id to what it's been translated to.
    type_map: HashMap<ComponentAnyTypeId, TypeId>,
}

impl WitPackageDecoder<'_> {
    fn new<'a>(types: &'a Types) -> WitPackageDecoder<'a> {
        WitPackageDecoder {
            resolve: Resolve::default(),
            types,
            type_map: HashMap::new(),
            foreign_packages: Default::default(),
            iface_to_package_index: Default::default(),
            named_interfaces: Default::default(),
            resources: Default::default(),
        }
    }

    fn decode_v1_package(&mut self, name: &ComponentName, ty: &ComponentType) -> Result<Package> {
        // Process all imports for this package first, where imports are
        // importing from remote packages.
        for (name, ty) in ty.imports.iter() {
            let ty = match ty {
                ComponentEntityType::Instance(idx) => &self.types[*idx],
                _ => bail!("import `{name}` is not an instance"),
            };
            self.register_import(name, ty)
                .with_context(|| format!("failed to process import `{name}`"))?;
        }

        let mut package = Package {
            // The name encoded for packages must be of the form `foo:bar/wit`
            // where "wit" is just a placeholder for now. The package name in
            // this case would be `foo:bar`.
            name: match name.kind() {
                ComponentNameKind::Interface(name) if name.interface().as_str() == "wit" => {
                    name.to_package_name()
                }
                _ => bail!("package name is not a valid id: {name}"),
            },
            docs: Default::default(),
            interfaces: Default::default(),
            worlds: Default::default(),
        };

        let mut fields = PackageFields {
            interfaces: &mut package.interfaces,
            worlds: &mut package.worlds,
        };

        for (name, ty) in ty.exports.iter() {
            match ty {
                ComponentEntityType::Instance(idx) => {
                    let ty = &self.types[*idx];
                    self.register_interface(name.as_str(), ty, &mut fields)
                        .with_context(|| format!("failed to process export `{name}`"))?;
                }
                ComponentEntityType::Component(idx) => {
                    let ty = &self.types[*idx];
                    self.register_world(name.as_str(), ty, &mut fields)
                        .with_context(|| format!("failed to process export `{name}`"))?;
                }
                _ => bail!("component export `{name}` is not an instance or component"),
            }
        }
        Ok(package)
    }

    fn decode_interface<'a>(
        &mut self,
        name: &str,
        imports: &wasmparser::collections::IndexMap<String, ComponentEntityType>,
        ty: &ComponentInstanceType,
        fields: &mut PackageFields<'a>,
    ) -> Result<PackageName> {
        let component_name = self
            .parse_component_name(name)
            .context("expected world name to have an ID form")?;

        let package = match component_name.kind() {
            ComponentNameKind::Interface(name) => name.to_package_name(),
            _ => bail!("expected world name to be fully qualified"),
        };

        for (name, ty) in imports.iter() {
            let ty = match ty {
                ComponentEntityType::Instance(idx) => &self.types[*idx],
                _ => bail!("import `{name}` is not an instance"),
            };
            self.register_import(name, ty)
                .with_context(|| format!("failed to process import `{name}`"))?;
        }

        let _ = self.register_interface(name, ty, fields)?;

        Ok(package)
    }

    fn decode_world<'a>(
        &mut self,
        name: &str,
        ty: &ComponentType,
        fields: &mut PackageFields<'a>,
    ) -> Result<PackageName> {
        let kebab_name = self
            .parse_component_name(name)
            .context("expected world name to have an ID form")?;

        let package = match kebab_name.kind() {
            ComponentNameKind::Interface(name) => name.to_package_name(),
            _ => bail!("expected world name to be fully qualified"),
        };

        let _ = self.register_world(name, ty, fields)?;

        Ok(package)
    }

    fn decode_component_import<'a>(
        &mut self,
        name: &str,
        world: WorldId,
        package: &mut PackageFields<'a>,
    ) -> Result<()> {
        log::debug!("decoding component import `{name}`");
        let ty = self
            .types
            .as_ref()
            .component_entity_type_of_import(name)
            .unwrap();
        let owner = TypeOwner::World(world);
        let (name, item) = match ty {
            ComponentEntityType::Instance(i) => {
                let ty = &self.types[i];
                let (name, id) = if name.contains('/') {
                    let id = self.register_import(name, ty)?;
                    (WorldKey::Interface(id), id)
                } else {
                    self.register_interface(name, ty, package)
                        .with_context(|| format!("failed to decode WIT from import `{name}`"))?
                };
                (
                    name,
                    WorldItem::Interface {
                        id,
                        stability: Default::default(),
                    },
                )
            }
            ComponentEntityType::Func(i) => {
                let ty = &self.types[i];
                let func = self
                    .convert_function(name, ty, owner)
                    .with_context(|| format!("failed to decode function from import `{name}`"))?;
                (WorldKey::Name(name.to_string()), WorldItem::Function(func))
            }
            ComponentEntityType::Type {
                referenced,
                created,
            } => {
                let id = self
                    .register_type_export(name, owner, referenced, created)
                    .with_context(|| format!("failed to decode type from export `{name}`"))?;
                (WorldKey::Name(name.to_string()), WorldItem::Type(id))
            }
            // All other imports do not form part of the component's world
            _ => return Ok(()),
        };
        self.resolve.worlds[world].imports.insert(name, item);
        Ok(())
    }

    fn decode_component_export<'a>(
        &mut self,
        export: &DecodingExport,
        world: WorldId,
        package: &mut PackageFields<'a>,
    ) -> Result<()> {
        let name = &export.name;
        log::debug!("decoding component export `{name}`");
        let types = self.types.as_ref();
        let ty = types.component_entity_type_of_export(name).unwrap();
        let (name, item) = match ty {
            ComponentEntityType::Func(i) => {
                let ty = &types[i];
                let func = self
                    .convert_function(name, ty, TypeOwner::World(world))
                    .with_context(|| format!("failed to decode function from export `{name}`"))?;

                (WorldKey::Name(name.to_string()), WorldItem::Function(func))
            }
            ComponentEntityType::Instance(i) => {
                let ty = &types[i];
                let (name, id) = if name.contains('/') {
                    let id = self.register_import(name, ty)?;
                    (WorldKey::Interface(id), id)
                } else {
                    self.register_interface(name, ty, package)
                        .with_context(|| format!("failed to decode WIT from export `{name}`"))?
                };
                (
                    name,
                    WorldItem::Interface {
                        id,
                        stability: Default::default(),
                    },
                )
            }
            _ => {
                bail!("component export `{name}` was not a function or instance")
            }
        };
        self.resolve.worlds[world].exports.insert(name, item);
        Ok(())
    }

    /// Registers that the `name` provided is either imported interface from a
    /// foreign package or  referencing a previously defined interface in this
    /// package.
    ///
    /// This function will internally ensure that `name` is well-structured and
    /// will fill in any information as necessary. For example with a foreign
    /// dependency the foreign package structure, types, etc, all need to be
    /// created. For a local dependency it's instead ensured that all the types
    /// line up with the previous definitions.
    fn register_import(&mut self, name: &str, ty: &ComponentInstanceType) -> Result<InterfaceId> {
        let (is_local, interface) = match self.named_interfaces.get(name) {
            Some(id) => (true, *id),
            None => (false, self.extract_dep_interface(name)?),
        };
        let owner = TypeOwner::Interface(interface);
        for (name, ty) in ty.exports.iter() {
            log::debug!("decoding import instance export `{name}`");
            match *ty {
                ComponentEntityType::Type {
                    referenced,
                    created,
                } => {
                    match self.resolve.interfaces[interface]
                        .types
                        .get(name.as_str())
                        .copied()
                    {
                        // If this name is already defined as a type in the
                        // specified interface then that's ok. For package-local
                        // interfaces that's expected since the interface was
                        // fully defined. For remote interfaces it means we're
                        // using something that was already used elsewhere. In
                        // both cases continue along.
                        //
                        // Notably for the remotely defined case this will also
                        // walk over the structure of the type and register
                        // internal wasmparser ids with wit-parser ids. This is
                        // necessary to ensure that anonymous types like
                        // `list<u8>` defined in original definitions are
                        // unified with anonymous types when duplicated inside
                        // of worlds. Overall this prevents, for example, extra
                        // `list<u8>` types from popping up when decoding. This
                        // is not strictly necessary but assists with
                        // roundtripping assertions during fuzzing.
                        Some(id) => {
                            log::debug!("type already exist");
                            match referenced {
                                ComponentAnyTypeId::Defined(ty) => {
                                    self.register_defined(id, &self.types[ty])?;
                                }
                                ComponentAnyTypeId::Resource(_) => {}
                                _ => unreachable!(),
                            }
                            let prev = self.type_map.insert(created, id);
                            assert!(prev.is_none());
                        }

                        // If the name is not defined, however, then there's two
                        // possibilities:
                        //
                        // * For package-local interfaces this is an error
                        //   because the package-local interface defined
                        //   everything already and this is referencing
                        //   something that isn't defined.
                        //
                        // * For remote interfaces they're never fully declared
                        //   so it's lazily filled in here. This means that the
                        //   view of remote interfaces ends up being the minimal
                        //   slice needed for this resolve, which is what's
                        //   intended.
                        None => {
                            if is_local {
                                bail!("instance type export `{name}` not defined in interface");
                            }
                            let id = self.register_type_export(
                                name.as_str(),
                                owner,
                                referenced,
                                created,
                            )?;
                            let prev = self.resolve.interfaces[interface]
                                .types
                                .insert(name.to_string(), id);
                            assert!(prev.is_none());
                        }
                    }
                }

                // This has similar logic to types above where we lazily fill in
                // functions for remote dependencies and otherwise assert
                // they're already defined for local dependencies.
                ComponentEntityType::Func(ty) => {
                    let def = &self.types[ty];
                    if self.resolve.interfaces[interface]
                        .functions
                        .contains_key(name.as_str())
                    {
                        // TODO: should ideally verify that function signatures
                        // match.
                        continue;
                    }
                    if is_local {
                        bail!("instance function export `{name}` not defined in interface");
                    }
                    let func = self.convert_function(name.as_str(), def, owner)?;
                    let prev = self.resolve.interfaces[interface]
                        .functions
                        .insert(name.to_string(), func);
                    assert!(prev.is_none());
                }

                _ => bail!("instance type export `{name}` is not a type"),
            }
        }

        Ok(interface)
    }

    fn find_alias(&self, id: ComponentAnyTypeId) -> Option<TypeId> {
        // Consult `type_map` for `referenced` or anything in its
        // chain of aliases to determine what it maps to. This may
        // bottom out in `None` in the case that this type is
        // just now being defined, but this should otherwise follow
        // chains of aliases to determine what exactly this was a
        // `use` of if it exists.
        let mut prev = None;
        let mut cur = id;
        while prev.is_none() {
            prev = self.type_map.get(&cur).copied();
            cur = match self.types.as_ref().peel_alias(cur) {
                Some(next) => next,
                None => break,
            };
        }
        prev
    }

    /// This will parse the `name_string` as a component model ID string and
    /// ensure that there's an `InterfaceId` corresponding to its components.
    fn extract_dep_interface(&mut self, name_string: &str) -> Result<InterfaceId> {
        let name = ComponentName::new(name_string, 0).unwrap();
        let name = match name.kind() {
            ComponentNameKind::Interface(name) => name,
            _ => bail!("package name is not a valid id: {name_string}"),
        };
        let package_name = name.to_package_name();
        // Lazily create a `Package` as necessary, along with the interface.
        let package = self
            .foreign_packages
            .entry(package_name.to_string())
            .or_insert_with(|| Package {
                name: package_name.clone(),
                docs: Default::default(),
                interfaces: Default::default(),
                worlds: Default::default(),
            });
        let interface = *package
            .interfaces
            .entry(name.interface().to_string())
            .or_insert_with(|| {
                self.resolve.interfaces.alloc(Interface {
                    name: Some(name.interface().to_string()),
                    docs: Default::default(),
                    types: IndexMap::default(),
                    functions: IndexMap::new(),
                    package: None,
                    stability: Default::default(),
                })
            });

        // Record a mapping of which foreign package this interface belongs to
        self.iface_to_package_index.insert(
            interface,
            self.foreign_packages
                .get_full(&package_name.to_string())
                .unwrap()
                .0,
        );
        Ok(interface)
    }

    /// A general-purpose helper function to translate a component instance
    /// into a WIT interface.
    ///
    /// This is one of the main workhorses of this module. This handles
    /// interfaces both at the type level, for concrete components, and
    /// internally within worlds as well.
    ///
    /// The `name` provided is the contextual ID or name of the interface. This
    /// could be a kebab-name in the case of a world import or export or it can
    /// also be an ID. This is used to guide insertion into various maps.
    ///
    /// The `ty` provided is the actual component type being decoded.
    ///
    /// The `package` is where to insert the final interface if `name` is an ID
    /// meaning it's registered as a named standalone item within the package.
    fn register_interface<'a>(
        &mut self,
        name: &str,
        ty: &ComponentInstanceType,
        package: &mut PackageFields<'a>,
    ) -> Result<(WorldKey, InterfaceId)> {
        // If this interface's name is already known then that means this is an
        // interface that's both imported and exported.  Use `register_import`
        // to draw connections between types and this interface's types.
        if self.named_interfaces.contains_key(name) {
            let id = self.register_import(name, ty)?;
            return Ok((WorldKey::Interface(id), id));
        }

        // If this is a bare kebab-name for an interface then the interface's
        // listed name is `None` and the name goes out through the key.
        // Otherwise this name is extracted from `name` interpreted as an ID.
        let interface_name = self.extract_interface_name_from_component_name(name)?;

        let mut interface = Interface {
            name: interface_name.clone(),
            docs: Default::default(),
            types: IndexMap::default(),
            functions: IndexMap::new(),
            package: None,
            stability: Default::default(),
        };

        let owner = TypeOwner::Interface(self.resolve.interfaces.next_id());
        for (name, ty) in ty.exports.iter() {
            match *ty {
                ComponentEntityType::Type {
                    referenced,
                    created,
                } => {
                    let ty = self
                        .register_type_export(name.as_str(), owner, referenced, created)
                        .with_context(|| format!("failed to register type export '{name}'"))?;
                    let prev = interface.types.insert(name.to_string(), ty);
                    assert!(prev.is_none());
                }

                ComponentEntityType::Func(ty) => {
                    let ty = &self.types[ty];
                    let func = self
                        .convert_function(name.as_str(), ty, owner)
                        .with_context(|| format!("failed to convert function '{name}'"))?;
                    let prev = interface.functions.insert(name.to_string(), func);
                    assert!(prev.is_none());
                }
                _ => bail!("instance type export `{name}` is not a type or function"),
            };
        }
        let id = self.resolve.interfaces.alloc(interface);
        let key = match interface_name {
            // If this interface is named then it's part of the package, so
            // insert it. Additionally register it in `named_interfaces` so
            // further use comes back to this original definition.
            Some(interface_name) => {
                let prev = package.interfaces.insert(interface_name, id);
                assert!(prev.is_none(), "duplicate interface added for {name:?}");
                let prev = self.named_interfaces.insert(name.to_string(), id);
                assert!(prev.is_none());
                WorldKey::Interface(id)
            }

            // If this interface isn't named then its key is always a
            // kebab-name.
            None => WorldKey::Name(name.to_string()),
        };
        Ok((key, id))
    }

    fn parse_component_name(&self, name: &str) -> Result<ComponentName> {
        ComponentName::new(name, 0)
            .with_context(|| format!("cannot extract item name from: {name}"))
    }

    fn extract_interface_name_from_component_name(&self, name: &str) -> Result<Option<String>> {
        let component_name = self.parse_component_name(name)?;
        match component_name.kind() {
            ComponentNameKind::Interface(name) => Ok(Some(name.interface().to_string())),
            ComponentNameKind::Label(_name) => Ok(None),
            _ => bail!("cannot extract item name from: {name}"),
        }
    }

    fn register_type_export(
        &mut self,
        name: &str,
        owner: TypeOwner,
        referenced: ComponentAnyTypeId,
        created: ComponentAnyTypeId,
    ) -> Result<TypeId> {
        let kind = match self.find_alias(referenced) {
            // If this `TypeId` points to a type which has
            // previously been defined, meaning we're aliasing a
            // prior definition.
            Some(prev) => {
                log::debug!("type export for `{name}` is an alias");
                TypeDefKind::Type(Type::Id(prev))
            }

            // ... or this `TypeId`'s source definition has never
            // been seen before, so declare the full type.
            None => {
                log::debug!("type export for `{name}` is a new type");
                match referenced {
                    ComponentAnyTypeId::Defined(ty) => self
                        .convert_defined(&self.types[ty])
                        .context("failed to convert unaliased type")?,
                    ComponentAnyTypeId::Resource(_) => TypeDefKind::Resource,
                    _ => unreachable!(),
                }
            }
        };
        let ty = self.resolve.types.alloc(TypeDef {
            name: Some(name.to_string()),
            kind,
            docs: Default::default(),
            stability: Default::default(),
            owner,
        });

        // If this is a resource then doubly-register it in `self.resources` so
        // the ID allocated here can be looked up via name later on during
        // `convert_function`.
        if let TypeDefKind::Resource = self.resolve.types[ty].kind {
            let prev = self
                .resources
                .entry(owner)
                .or_insert(HashMap::new())
                .insert(name.to_string(), ty);
            assert!(prev.is_none());
        }

        let prev = self.type_map.insert(created, ty);
        assert!(prev.is_none());
        Ok(ty)
    }

    fn register_world<'a>(
        &mut self,
        name: &str,
        ty: &ComponentType,
        package: &mut PackageFields<'a>,
    ) -> Result<WorldId> {
        let name = self
            .extract_interface_name_from_component_name(name)?
            .context("expected world name to have an ID form")?;
        let mut world = World {
            name: name.clone(),
            docs: Default::default(),
            imports: Default::default(),
            exports: Default::default(),
            includes: Default::default(),
            include_names: Default::default(),
            package: None,
            stability: Default::default(),
        };

        let owner = TypeOwner::World(self.resolve.worlds.next_id());
        for (name, ty) in ty.imports.iter() {
            let (name, item) = match ty {
                ComponentEntityType::Instance(idx) => {
                    let ty = &self.types[*idx];
                    let (name, id) = if name.contains('/') {
                        // If a name is an interface import then it is either to
                        // a package-local or foreign interface, and both
                        // situations are handled in `register_import`.
                        let id = self.register_import(name, ty)?;
                        (WorldKey::Interface(id), id)
                    } else {
                        // A plain kebab-name indicates an inline interface that
                        // wasn't declared explicitly elsewhere with a name, and
                        // `register_interface` will create a new `Interface`
                        // with no name.
                        self.register_interface(name, ty, package)?
                    };
                    (
                        name,
                        WorldItem::Interface {
                            id,
                            stability: Default::default(),
                        },
                    )
                }
                ComponentEntityType::Type {
                    created,
                    referenced,
                } => {
                    let ty =
                        self.register_type_export(name.as_str(), owner, *referenced, *created)?;
                    (WorldKey::Name(name.to_string()), WorldItem::Type(ty))
                }
                ComponentEntityType::Func(idx) => {
                    let ty = &self.types[*idx];
                    let func = self.convert_function(name.as_str(), ty, owner)?;
                    (WorldKey::Name(name.to_string()), WorldItem::Function(func))
                }
                _ => bail!("component import `{name}` is not an instance, func, or type"),
            };
            world.imports.insert(name, item);
        }

        for (name, ty) in ty.exports.iter() {
            let (name, item) = match ty {
                ComponentEntityType::Instance(idx) => {
                    let ty = &self.types[*idx];
                    let (name, id) = if name.contains('/') {
                        // Note that despite this being an export this is
                        // calling `register_import`. With a URL this interface
                        // must have been previously defined so this will
                        // trigger the logic of either filling in a remotely
                        // defined interface or connecting items to local
                        // definitions of our own interface.
                        let id = self.register_import(name, ty)?;
                        (WorldKey::Interface(id), id)
                    } else {
                        self.register_interface(name, ty, package)?
                    };
                    (
                        name,
                        WorldItem::Interface {
                            id,
                            stability: Default::default(),
                        },
                    )
                }

                ComponentEntityType::Func(idx) => {
                    let ty = &self.types[*idx];
                    let func = self.convert_function(name.as_str(), ty, owner)?;
                    (WorldKey::Name(name.to_string()), WorldItem::Function(func))
                }

                _ => bail!("component export `{name}` is not an instance or function"),
            };
            world.exports.insert(name, item);
        }
        let id = self.resolve.worlds.alloc(world);
        let prev = package.worlds.insert(name, id);
        assert!(prev.is_none());
        Ok(id)
    }

    fn convert_function(
        &mut self,
        name: &str,
        ty: &ComponentFuncType,
        owner: TypeOwner,
    ) -> Result<Function> {
        let name = ComponentName::new(name, 0).unwrap();
        let params = ty
            .params
            .iter()
            .map(|(name, ty)| Ok((name.to_string(), self.convert_valtype(ty)?)))
            .collect::<Result<Vec<_>>>()
            .context("failed to convert params")?;
        let result = match &ty.result {
            Some(ty) => Some(
                self.convert_valtype(ty)
                    .context("failed to convert anonymous result type")?,
            ),
            None => None,
        };
        Ok(Function {
            docs: Default::default(),
            stability: Default::default(),
            kind: match name.kind() {
                ComponentNameKind::Label(_) => {
                    if ty.async_ {
                        FunctionKind::AsyncFreestanding
                    } else {
                        FunctionKind::Freestanding
                    }
                }
                ComponentNameKind::Constructor(resource) => {
                    FunctionKind::Constructor(self.resources[&owner][resource.as_str()])
                }
                ComponentNameKind::Method(name) => {
                    if ty.async_ {
                        FunctionKind::AsyncMethod(self.resources[&owner][name.resource().as_str()])
                    } else {
                        FunctionKind::Method(self.resources[&owner][name.resource().as_str()])
                    }
                }
                ComponentNameKind::Static(name) => {
                    if ty.async_ {
                        FunctionKind::AsyncStatic(self.resources[&owner][name.resource().as_str()])
                    } else {
                        FunctionKind::Static(self.resources[&owner][name.resource().as_str()])
                    }
                }

                // Functions shouldn't have ID-based names at this time.
                ComponentNameKind::Interface(_)
                | ComponentNameKind::Url(_)
                | ComponentNameKind::Hash(_)
                | ComponentNameKind::Dependency(_) => unreachable!(),
            },

            // Note that this name includes "name mangling" such as
            // `[method]foo.bar` which is intentional. The `FunctionKind`
            // discriminant calculated above indicates how to interpret this
            // name.
            name: name.to_string(),
            params,
            result,
        })
    }

    fn convert_valtype(&mut self, ty: &ComponentValType) -> Result<Type> {
        let id = match ty {
            ComponentValType::Primitive(ty) => return Ok(self.convert_primitive(*ty)),
            ComponentValType::Type(id) => *id,
        };

        // Don't create duplicate types for anything previously created.
        if let Some(ret) = self.type_map.get(&id.into()) {
            return Ok(Type::Id(*ret));
        }

        // Otherwise create a new `TypeDef` without a name since this is an
        // anonymous valtype. Note that this is invalid for some types so return
        // errors on those types, but eventually the `bail!` here  is
        // more-or-less unreachable due to expected validation to be added to
        // the component model binary format itself.
        let def = &self.types[id];
        let kind = self.convert_defined(def)?;
        match &kind {
            TypeDefKind::Type(_)
            | TypeDefKind::List(_)
            | TypeDefKind::Map(_, _)
            | TypeDefKind::FixedSizeList(..)
            | TypeDefKind::Tuple(_)
            | TypeDefKind::Option(_)
            | TypeDefKind::Result(_)
            | TypeDefKind::Handle(_)
            | TypeDefKind::Future(_)
            | TypeDefKind::Stream(_) => {}

            TypeDefKind::Resource
            | TypeDefKind::Record(_)
            | TypeDefKind::Enum(_)
            | TypeDefKind::Variant(_)
            | TypeDefKind::Flags(_) => {
                bail!("unexpected unnamed type of kind '{}'", kind.as_str());
            }
            TypeDefKind::Unknown => unreachable!(),
        }
        let ty = self.resolve.types.alloc(TypeDef {
            name: None,
            docs: Default::default(),
            stability: Default::default(),
            owner: TypeOwner::None,
            kind,
        });
        let prev = self.type_map.insert(id.into(), ty);
        assert!(prev.is_none());
        Ok(Type::Id(ty))
    }

    /// Converts a wasmparser `ComponentDefinedType`, the definition of a type
    /// in the component model, to a WIT `TypeDefKind` to get inserted into the
    /// types arena by the caller.
    fn convert_defined(&mut self, ty: &ComponentDefinedType) -> Result<TypeDefKind> {
        match ty {
            ComponentDefinedType::Primitive(t) => Ok(TypeDefKind::Type(self.convert_primitive(*t))),

            ComponentDefinedType::List(t) => {
                let t = self.convert_valtype(t)?;
                Ok(TypeDefKind::List(t))
            }

            ComponentDefinedType::Map(k, v) => {
                let k = self.convert_valtype(k)?;
                let v = self.convert_valtype(v)?;
                Ok(TypeDefKind::Map(k, v))
            }

            ComponentDefinedType::FixedSizeList(t, size) => {
                let t = self.convert_valtype(t)?;
                Ok(TypeDefKind::FixedSizeList(t, *size))
            }

            ComponentDefinedType::Tuple(t) => {
                let types = t
                    .types
                    .iter()
                    .map(|t| self.convert_valtype(t))
                    .collect::<Result<_>>()?;
                Ok(TypeDefKind::Tuple(Tuple { types }))
            }

            ComponentDefinedType::Option(t) => {
                let t = self.convert_valtype(t)?;
                Ok(TypeDefKind::Option(t))
            }

            ComponentDefinedType::Result { ok, err } => {
                let ok = match ok {
                    Some(t) => Some(self.convert_valtype(t)?),
                    None => None,
                };
                let err = match err {
                    Some(t) => Some(self.convert_valtype(t)?),
                    None => None,
                };
                Ok(TypeDefKind::Result(Result_ { ok, err }))
            }

            ComponentDefinedType::Record(r) => {
                let fields = r
                    .fields
                    .iter()
                    .map(|(name, ty)| {
                        Ok(Field {
                            name: name.to_string(),
                            ty: self.convert_valtype(ty).with_context(|| {
                                format!("failed to convert record field '{name}'")
                            })?,
                            docs: Default::default(),
                        })
                    })
                    .collect::<Result<_>>()?;
                Ok(TypeDefKind::Record(Record { fields }))
            }

            ComponentDefinedType::Variant(v) => {
                let cases = v
                    .cases
                    .iter()
                    .map(|(name, case)| {
                        if case.refines.is_some() {
                            bail!("unimplemented support for `refines`");
                        }
                        Ok(Case {
                            name: name.to_string(),
                            ty: match &case.ty {
                                Some(ty) => Some(self.convert_valtype(ty)?),
                                None => None,
                            },
                            docs: Default::default(),
                        })
                    })
                    .collect::<Result<_>>()?;
                Ok(TypeDefKind::Variant(Variant { cases }))
            }

            ComponentDefinedType::Flags(f) => {
                let flags = f
                    .iter()
                    .map(|name| Flag {
                        name: name.to_string(),
                        docs: Default::default(),
                    })
                    .collect();
                Ok(TypeDefKind::Flags(Flags { flags }))
            }

            ComponentDefinedType::Enum(e) => {
                let cases = e
                    .iter()
                    .cloned()
                    .map(|name| EnumCase {
                        name: name.into(),
                        docs: Default::default(),
                    })
                    .collect();
                Ok(TypeDefKind::Enum(Enum { cases }))
            }

            ComponentDefinedType::Own(id) => {
                let id = self.type_map[&(*id).into()];
                Ok(TypeDefKind::Handle(Handle::Own(id)))
            }

            ComponentDefinedType::Borrow(id) => {
                let id = self.type_map[&(*id).into()];
                Ok(TypeDefKind::Handle(Handle::Borrow(id)))
            }

            ComponentDefinedType::Future(ty) => Ok(TypeDefKind::Future(
                ty.as_ref().map(|ty| self.convert_valtype(ty)).transpose()?,
            )),

            ComponentDefinedType::Stream(ty) => Ok(TypeDefKind::Stream(
                ty.as_ref().map(|ty| self.convert_valtype(ty)).transpose()?,
            )),
        }
    }

    fn convert_primitive(&self, ty: PrimitiveValType) -> Type {
        match ty {
            PrimitiveValType::U8 => Type::U8,
            PrimitiveValType::S8 => Type::S8,
            PrimitiveValType::U16 => Type::U16,
            PrimitiveValType::S16 => Type::S16,
            PrimitiveValType::U32 => Type::U32,
            PrimitiveValType::S32 => Type::S32,
            PrimitiveValType::U64 => Type::U64,
            PrimitiveValType::S64 => Type::S64,
            PrimitiveValType::Bool => Type::Bool,
            PrimitiveValType::Char => Type::Char,
            PrimitiveValType::String => Type::String,
            PrimitiveValType::F32 => Type::F32,
            PrimitiveValType::F64 => Type::F64,
            PrimitiveValType::ErrorContext => Type::ErrorContext,
        }
    }

    fn register_defined(&mut self, id: TypeId, def: &ComponentDefinedType) -> Result<()> {
        Registrar {
            types: &self.types,
            type_map: &mut self.type_map,
            resolve: &self.resolve,
        }
        .defined(id, def)
    }

    /// Completes the decoding of this resolve by finalizing all packages into
    /// their topological ordering within the returned `Resolve`.
    ///
    /// Takes the root package as an argument to insert.
    fn finish(mut self, package: Package) -> (Resolve, PackageId) {
        // Build a topological ordering is then calculated by visiting all the
        // transitive dependencies of packages.
        let mut order = IndexSet::new();
        for i in 0..self.foreign_packages.len() {
            self.visit_package(i, &mut order);
        }

        // Using the topological ordering create a temporary map from
        // index-in-`foreign_packages` to index-in-`order`
        let mut idx_to_pos = vec![0; self.foreign_packages.len()];
        for (pos, idx) in order.iter().enumerate() {
            idx_to_pos[*idx] = pos;
        }
        // .. and then using `idx_to_pos` sort the `foreign_packages` array based
        // on the position it's at in the topological ordering
        let mut deps = mem::take(&mut self.foreign_packages)
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        deps.sort_by_key(|(idx, _)| idx_to_pos[*idx]);

        // .. and finally insert the packages, in their final topological
        // ordering, into the returned array.
        for (_idx, (_url, pkg)) in deps {
            self.insert_package(pkg);
        }

        let id = self.insert_package(package);
        assert!(self.resolve.worlds.iter().all(|(_, w)| w.package.is_some()));
        assert!(
            self.resolve
                .interfaces
                .iter()
                .all(|(_, i)| i.package.is_some())
        );
        (self.resolve, id)
    }

    fn insert_package(&mut self, package: Package) -> PackageId {
        let Package {
            name,
            interfaces,
            worlds,
            docs,
        } = package;

        // Most of the time the `package` being inserted is not already present
        // in `self.resolve`, but in the case of the top-level `decode_world`
        // function this isn't the case. This shouldn't in general be a problem
        // so union-up the packages here while asserting that nothing gets
        // replaced by accident which would indicate a bug.
        let pkg = self
            .resolve
            .package_names
            .get(&name)
            .copied()
            .unwrap_or_else(|| {
                let id = self.resolve.packages.alloc(Package {
                    name: name.clone(),
                    interfaces: Default::default(),
                    worlds: Default::default(),
                    docs,
                });
                let prev = self.resolve.package_names.insert(name, id);
                assert!(prev.is_none());
                id
            });

        for (name, id) in interfaces {
            let prev = self.resolve.packages[pkg].interfaces.insert(name, id);
            assert!(prev.is_none());
            self.resolve.interfaces[id].package = Some(pkg);
        }

        for (name, id) in worlds {
            let prev = self.resolve.packages[pkg].worlds.insert(name, id);
            assert!(prev.is_none());
            let world = &mut self.resolve.worlds[id];
            world.package = Some(pkg);
            for (name, item) in world.imports.iter().chain(world.exports.iter()) {
                if let WorldKey::Name(_) = name {
                    if let WorldItem::Interface { id, .. } = item {
                        self.resolve.interfaces[*id].package = Some(pkg);
                    }
                }
            }
        }

        pkg
    }

    fn visit_package(&self, idx: usize, order: &mut IndexSet<usize>) {
        if order.contains(&idx) {
            return;
        }

        let (_name, pkg) = self.foreign_packages.get_index(idx).unwrap();
        let interfaces = pkg.interfaces.values().copied().chain(
            pkg.worlds
                .values()
                .flat_map(|w| {
                    let world = &self.resolve.worlds[*w];
                    world.imports.values().chain(world.exports.values())
                })
                .filter_map(|item| match item {
                    WorldItem::Interface { id, .. } => Some(*id),
                    WorldItem::Function(_) | WorldItem::Type(_) => None,
                }),
        );
        for iface in interfaces {
            for dep in self.resolve.interface_direct_deps(iface) {
                let dep_idx = self.iface_to_package_index[&dep];
                if dep_idx != idx {
                    self.visit_package(dep_idx, order);
                }
            }
        }

        assert!(order.insert(idx));
    }
}

/// Helper type to register the structure of a wasm-defined type against a
/// wit-defined type.
struct Registrar<'a> {
    types: &'a Types,
    type_map: &'a mut HashMap<ComponentAnyTypeId, TypeId>,
    resolve: &'a Resolve,
}

impl Registrar<'_> {
    /// Verifies that the wasm structure of `def` matches the wit structure of
    /// `id` and recursively registers types.
    fn defined(&mut self, id: TypeId, def: &ComponentDefinedType) -> Result<()> {
        match def {
            ComponentDefinedType::Primitive(_) => Ok(()),

            ComponentDefinedType::List(t) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::List(r) => r,
                    // Note that all cases below have this match and the general
                    // idea is that once a type is named or otherwise identified
                    // here there's no need to recurse. The purpose of this
                    // registrar is to build connections for anonymous types
                    // that don't otherwise have a name to ensure that they're
                    // decoded to reuse the same constructs consistently. For
                    // that reason once something is named we can bail out.
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a list"),
                };
                self.valtype(t, ty)
            }

            ComponentDefinedType::Map(k, v) => {
                let (key_ty, value_ty) = match &self.resolve.types[id].kind {
                    TypeDefKind::Map(k, v) => (k, v),
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a map"),
                };
                self.valtype(k, key_ty)?;
                self.valtype(v, value_ty)
            }

            ComponentDefinedType::FixedSizeList(t, elements) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::FixedSizeList(r, elements2) if elements2 == elements => r,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a fixed size {elements} list"),
                };
                self.valtype(t, ty)
            }

            ComponentDefinedType::Tuple(t) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::Tuple(r) => r,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a tuple"),
                };
                if ty.types.len() != t.types.len() {
                    bail!("mismatched number of tuple fields");
                }
                for (a, b) in t.types.iter().zip(ty.types.iter()) {
                    self.valtype(a, b)?;
                }
                Ok(())
            }

            ComponentDefinedType::Option(t) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::Option(r) => r,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected an option"),
                };
                self.valtype(t, ty)
            }

            ComponentDefinedType::Result { ok, err } => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::Result(r) => r,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a result"),
                };
                match (ok, &ty.ok) {
                    (Some(a), Some(b)) => self.valtype(a, b)?,
                    (None, None) => {}
                    _ => bail!("disagreement on result structure"),
                }
                match (err, &ty.err) {
                    (Some(a), Some(b)) => self.valtype(a, b)?,
                    (None, None) => {}
                    _ => bail!("disagreement on result structure"),
                }
                Ok(())
            }

            ComponentDefinedType::Record(def) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::Record(r) => r,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a record"),
                };
                if def.fields.len() != ty.fields.len() {
                    bail!("mismatched number of record fields");
                }
                for ((name, ty), field) in def.fields.iter().zip(&ty.fields) {
                    if name.as_str() != field.name {
                        bail!("mismatched field order");
                    }
                    self.valtype(ty, &field.ty)?;
                }
                Ok(())
            }

            ComponentDefinedType::Variant(def) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::Variant(r) => r,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a variant"),
                };
                if def.cases.len() != ty.cases.len() {
                    bail!("mismatched number of variant cases");
                }
                for ((name, ty), case) in def.cases.iter().zip(&ty.cases) {
                    if name.as_str() != case.name {
                        bail!("mismatched case order");
                    }
                    match (&ty.ty, &case.ty) {
                        (Some(a), Some(b)) => self.valtype(a, b)?,
                        (None, None) => {}
                        _ => bail!("disagreement on case type"),
                    }
                }
                Ok(())
            }

            ComponentDefinedType::Future(payload) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::Future(p) => p,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a future"),
                };
                match (payload, ty) {
                    (Some(a), Some(b)) => self.valtype(a, b),
                    (None, None) => Ok(()),
                    _ => bail!("disagreement on future payload"),
                }
            }

            ComponentDefinedType::Stream(payload) => {
                let ty = match &self.resolve.types[id].kind {
                    TypeDefKind::Stream(p) => p,
                    TypeDefKind::Type(Type::Id(_)) => return Ok(()),
                    _ => bail!("expected a stream"),
                };
                match (payload, ty) {
                    (Some(a), Some(b)) => self.valtype(a, b),
                    (None, None) => Ok(()),
                    _ => bail!("disagreement on stream payload"),
                }
            }

            // These have no recursive structure so they can bail out.
            ComponentDefinedType::Flags(_)
            | ComponentDefinedType::Enum(_)
            | ComponentDefinedType::Own(_)
            | ComponentDefinedType::Borrow(_) => Ok(()),
        }
    }

    fn valtype(&mut self, wasm: &ComponentValType, wit: &Type) -> Result<()> {
        let wasm = match wasm {
            ComponentValType::Type(wasm) => *wasm,
            ComponentValType::Primitive(_wasm) => {
                assert!(!matches!(wit, Type::Id(_)));
                return Ok(());
            }
        };
        let wit = match wit {
            Type::Id(id) => *id,
            _ => bail!("expected id-based type"),
        };
        let prev = match self.type_map.insert(wasm.into(), wit) {
            Some(prev) => prev,
            None => {
                let wasm = &self.types[wasm];
                return self.defined(wit, wasm);
            }
        };
        // If `wit` matches `prev` then we've just rediscovered what we already
        // knew which is that the `wasm` id maps to the `wit` id.
        //
        // If, however, `wit` is not equal to `prev` then that's more
        // interesting. Consider a component such as:
        //
        // ```wasm
        // (component
        //   (import (interface "a:b/name") (instance
        //      (type $l (list string))
        //      (type $foo (variant (case "l" $l)))
        //      (export "foo" (type (eq $foo)))
        //   ))
        //   (component $c
        //     (type $l (list string))
        //     (type $bar (variant (case "n" u16) (case "l" $l)))
        //     (export "bar" (type $bar))
        //     (type $foo (variant (case "l" $l)))
        //     (export "foo" (type $foo))
        //   )
        //   (instance $i (instantiate $c))
        //   (export (interface "a:b/name") (instance $i))
        // )
        // ```
        //
        // This roughly corresponds to:
        //
        // ```wit
        // package a:b
        //
        // interface name {
        //   variant bar {
        //     n(u16),
        //     l(list<string>),
        //   }
        //
        //   variant foo {
        //     l(list<string>),
        //   }
        // }
        //
        // world module {
        //   import name
        //   export name
        // }
        // ```
        //
        // In this situation first we'll see the `import` which records type
        // information for the `foo` type in `interface name`. Later on the full
        // picture of `interface name` becomes apparent with the export of a
        // component which has full type information. When walking over this
        // first `bar` is seen and its recursive structure.
        //
        // The problem arises when walking over the `foo` type. In this
        // situation the code path we're currently on will be hit because
        // there's a preexisting definition of `foo` from the import and it's
        // now going to be unified with what we've seen in the export. When
        // visiting the `list<string>` case of the `foo` variant this ends up
        // being different than the `list<string>` used by the `bar` variant. The
        // reason for this is that when visiting `bar` the wasm-defined `(list
        // string)` hasn't been seen before so a new type is allocated. Later
        // though this same wasm type is unified with the first `(list string)`
        // type in the `import`.
        //
        // All-in-all this ends up meaning that it's possible for `prev` to not
        // match `wit`. In this situation it means the decoded WIT interface
        // will have duplicate definitions of `list<string>`. This is,
        // theoretically, not that big of a problem because the same underlying
        // definition is still there and the meaning of the type is the same.
        // This can, however, perhaps be a problem for consumers where it's
        // assumed that all `list<string>` are equal and there's only one. For
        // example a bindings generator for C may assume that `list<string>`
        // will only appear once and generate a single name for it, but with two
        // different types in play here it may generate two types of the same
        // name (or something like that).
        //
        // For now though this is left for a future refactoring. Fixing this
        // issue would require tracking anonymous types during type translation
        // so the decoding process for the `bar` export would reuse the
        // `list<string>` type created from decoding the `foo` import. That's
        // somewhat nontrivial at this time, so it's left for a future
        // refactoring.
        let _ = prev;
        Ok(())
    }
}

pub(crate) trait InterfaceNameExt {
    fn to_package_name(&self) -> PackageName;
}

impl InterfaceNameExt for wasmparser::names::InterfaceName<'_> {
    fn to_package_name(&self) -> PackageName {
        PackageName {
            namespace: self.namespace().to_string(),
            name: self.package().to_string(),
            version: self.version(),
        }
    }
}
