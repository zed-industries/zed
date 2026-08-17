//! Definition for encoding of custom sections within core wasm modules of
//! component-model related data.
//!
//! When creating a component from a source language the high-level process for
//! doing this is that code will be generated into the source language by
//! `wit-bindgen` or a similar tool which will be compiled down to core wasm.
//! The core wasm file is then fed into `wit-component` and a component is
//! created. This means that the componentization process is decoupled from the
//! binding generation process and intentionally affords for linking together
//! libraries into the main core wasm module that import different interfaces.
//!
//! The purpose of this module is to define an intermediate format to reside in
//! a custom section in the core wasm output. This intermediate format is
//! carried through the wasm linker through a custom section whose name starts
//! with `component-type`. This custom section is created
//! per-language-binding-generation and consumed by slurping up all the
//! sections during the component creation process.
//!
//! Currently the encoding of this custom section is itself a component. The
//! component has a single export which is a component type which represents the
//! `world` that was bound during bindings generation. This single export is
//! used to decode back into a `Resolve` with a WIT representation.
//!
//! Currently the component additionally has a custom section named
//! `wit-component-encoding` (see `CUSTOM_SECTION_NAME`). This section is
//! currently defined as 2 bytes:
//!
//! * The first byte is `CURRENT_VERSION` to help protect against future and
//!   past changes.
//! * The second byte indicates the string encoding used for imports/exports as
//!   part of the bindings process. The mapping is defined by
//!   `encode_string_encoding`.
//!
//! This means that the top-level `encode` function takes a `Resolve`, a
//! `WorldId`, and a `StringEncoding`. Note that the top-level `decode` function
//! is slightly difference because it's taking all custom sections in a core
//! wasm binary, possibly from multiple invocations of bindgen, and unioning
//! them all together. This means that the output is a `Bindgen` which
//! represents the union of all previous bindings.
//!
//! The dual of `encode` is the `decode_custom_section` fucntion which decodes
//! the three arguments originally passed to `encode`.

use crate::{DecodedWasm, StringEncoding};
use anyhow::{bail, Context, Result};
use indexmap::{IndexMap, IndexSet};
use std::borrow::Cow;
use wasm_encoder::{
    ComponentBuilder, ComponentExportKind, ComponentType, ComponentTypeRef, CustomSection,
};
use wasm_metadata::Producers;
use wasmparser::{BinaryReader, Encoding, Parser, Payload};
use wit_parser::{Package, PackageName, Resolve, World, WorldId, WorldItem, WorldKey};

const CURRENT_VERSION: u8 = 0x04;
const CUSTOM_SECTION_NAME: &str = "wit-component-encoding";

/// The result of decoding binding information from a WebAssembly binary.
///
/// This structure is returned by [`decode`] and represents the interface of a
/// WebAssembly binary.
pub struct Bindgen {
    /// Interface and type information for this binary.
    pub resolve: Resolve,
    /// The world that was bound.
    pub world: WorldId,
    /// Metadata about this specific module that was bound.
    pub metadata: ModuleMetadata,
    /// Producer information about tools used to produce this specific module.
    pub producers: Option<Producers>,
}

impl Default for Bindgen {
    fn default() -> Bindgen {
        let mut resolve = Resolve::default();
        let package = resolve.packages.alloc(Package {
            name: PackageName {
                namespace: "root".to_string(),
                name: "root".to_string(),
                version: None,
            },
            docs: Default::default(),
            interfaces: Default::default(),
            worlds: Default::default(),
        });
        let world = resolve.worlds.alloc(World {
            name: "root".to_string(),
            docs: Default::default(),
            imports: Default::default(),
            exports: Default::default(),
            includes: Default::default(),
            include_names: Default::default(),
            package: Some(package),
            stability: Default::default(),
        });
        resolve.packages[package]
            .worlds
            .insert("root".to_string(), world);
        Bindgen {
            resolve,
            world,
            metadata: ModuleMetadata::default(),
            producers: None,
        }
    }
}

/// Module-level metadata that's specific to one core WebAssembly module. This
/// is extracted with a [`Bindgen`].
#[derive(Default)]
pub struct ModuleMetadata {
    /// Per-function options imported into the core wasm module, currently only
    /// related to string encoding.
    pub import_encodings: EncodingMap,

    /// Per-function options exported from the core wasm module, currently only
    /// related to string encoding.
    pub export_encodings: EncodingMap,
}

/// Internal map that keeps track of encodings for various world imports and
/// exports.
///
/// Stored in [`ModuleMetadata`].
#[derive(Default)]
pub struct EncodingMap {
    /// A map of an "identifying string" for world items to what string
    /// encoding the import or export is using.
    ///
    /// The keys of this map are created by `EncodingMap::key` and are
    /// specifically chosen to be able to be looked up during both insertion and
    /// fetching. Note that in particular this map does not use `*Id` types such
    /// as `InterfaceId` from `wit_parser`. This is due to the fact that during
    /// world merging new interfaces are created for named imports (e.g. `import
    /// x: interface { ... }`) as inline interfaces are copied from one world to
    /// another. Additionally during world merging different interfaces at the
    /// same version may be deduplicated.
    ///
    /// For these reasons a string-based key is chosen to avoid juggling IDs
    /// through the world merging process. Additionally versions are chopped off
    /// for now to help with a problem such as:
    ///
    /// * The main module imports a:b/c@0.1.0
    /// * An adapter imports a:b/c@0.1.1
    /// * The final world uses a:b/c@0.1.1, but the main module has no
    ///   encoding listed for that exact item.
    ///
    /// By chopping off versions this is able to get everything registered
    /// correctly even in the fact of merging interfaces and worlds.
    encodings: IndexMap<String, StringEncoding>,
}

impl EncodingMap {
    fn insert_all(
        &mut self,
        resolve: &Resolve,
        set: &IndexMap<WorldKey, WorldItem>,
        encoding: StringEncoding,
    ) {
        for (name, item) in set {
            match item {
                WorldItem::Function(func) => {
                    let key = self.key(resolve, name, &func.name);
                    self.encodings.insert(key, encoding);
                }
                WorldItem::Interface { id, .. } => {
                    for (func, _) in resolve.interfaces[*id].functions.iter() {
                        let key = self.key(resolve, name, func);
                        self.encodings.insert(key, encoding);
                    }
                }
                WorldItem::Type(_) => {}
            }
        }
    }

    /// Looks up the encoding of the function `func` which is scoped under `key`
    /// in the world in question.
    pub fn get(&self, resolve: &Resolve, key: &WorldKey, func: &str) -> Option<StringEncoding> {
        let key = self.key(resolve, key, func);
        self.encodings.get(&key).copied()
    }

    fn key(&self, resolve: &Resolve, key: &WorldKey, func: &str) -> String {
        format!(
            "{}/{func}",
            match key {
                WorldKey::Name(name) => name.to_string(),
                WorldKey::Interface(id) => {
                    let iface = &resolve.interfaces[*id];
                    let pkg = &resolve.packages[iface.package.unwrap()];
                    format!(
                        "{}:{}/{}",
                        pkg.name.namespace,
                        pkg.name.name,
                        iface.name.as_ref().unwrap()
                    )
                }
            }
        )
    }

    fn merge(&mut self, other: EncodingMap) -> Result<()> {
        for (key, encoding) in other.encodings {
            if let Some(prev) = self.encodings.insert(key.clone(), encoding) {
                if prev != encoding {
                    bail!("conflicting string encodings specified for `{key}`");
                }
            }
        }
        Ok(())
    }
}

/// This function will parse the core `wasm` binary given as input and return a
/// [`Bindgen`] which extracts the custom sections describing component-level
/// types from within the binary itself.
///
/// This is used to parse the output of `wit-bindgen`-generated modules and is
/// one of the earliest phases in transitioning such a module to a component.
/// The extraction here provides the metadata necessary to continue the process
/// later on.
///
/// This will return an error if `wasm` is not a valid WebAssembly module.
///
/// If a `component-type` custom section was found then a new binary is
/// optionally returned with the custom sections stripped out. If no
/// `component-type` custom sections are found then `None` is returned.
pub fn decode(wasm: &[u8]) -> Result<(Option<Vec<u8>>, Bindgen)> {
    let mut ret = Bindgen::default();
    let mut new_module = wasm_encoder::Module::new();

    let mut found_custom = false;
    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        let payload = payload.context("decoding item in module")?;
        match payload {
            wasmparser::Payload::CustomSection(cs) if cs.name().starts_with("component-type") => {
                let data = Bindgen::decode_custom_section(cs.data())
                    .with_context(|| format!("decoding custom section {}", cs.name()))?;
                ret.merge(data)
                    .with_context(|| format!("updating metadata for section {}", cs.name()))?;
                found_custom = true;
            }
            wasmparser::Payload::Version { encoding, .. } if encoding != Encoding::Module => {
                bail!("decoding a component is not supported")
            }
            _ => {
                if let Some((id, range)) = payload.as_section() {
                    new_module.section(&wasm_encoder::RawSection {
                        id,
                        data: &wasm[range],
                    });
                }
            }
        }
    }

    if found_custom {
        Ok((Some(new_module.finish()), ret))
    } else {
        Ok((None, ret))
    }
}

/// Creates a `component-type*` custom section to be decoded by `decode` above.
///
/// This is primarily created by wit-bindgen-based guest generators to embed
/// into the final core wasm binary. The core wasm binary is later fed
/// through `wit-component` to produce the actual component where this returned
/// section will be decoded.
pub fn encode(
    resolve: &Resolve,
    world: WorldId,
    string_encoding: StringEncoding,
    extra_producers: Option<&Producers>,
) -> Result<Vec<u8>> {
    let ty = crate::encoding::encode_world(resolve, world)?;

    let world = &resolve.worlds[world];
    let mut outer_ty = ComponentType::new();
    outer_ty.ty().component(&ty);
    outer_ty.export(
        &resolve.id_of_name(world.package.unwrap(), &world.name),
        ComponentTypeRef::Component(0),
    );

    let mut builder = ComponentBuilder::default();

    let string_encoding = encode_string_encoding(string_encoding);
    builder.custom_section(&CustomSection {
        name: CUSTOM_SECTION_NAME.into(),
        data: Cow::Borrowed(&[CURRENT_VERSION, string_encoding]),
    });

    let ty = builder.type_component(&outer_ty);
    builder.export(&world.name, ComponentExportKind::Type, ty, None);

    let mut producers = crate::base_producers();
    if let Some(p) = extra_producers {
        producers.merge(&p);
    }
    builder.raw_custom_section(&producers.raw_custom_section());
    Ok(builder.finish())
}

fn decode_custom_section(wasm: &[u8]) -> Result<(Resolve, WorldId, StringEncoding)> {
    let (resolve, world) = wit_parser::decoding::decode_world(wasm)?;
    let mut custom_section = None;

    for payload in Parser::new(0).parse_all(wasm) {
        match payload? {
            Payload::CustomSection(s) if s.name() == CUSTOM_SECTION_NAME => {
                custom_section = Some(s.data());
            }
            _ => {}
        }
    }
    let string_encoding = match custom_section {
        None => bail!("missing custom section of name `{CUSTOM_SECTION_NAME}`"),
        Some([CURRENT_VERSION, byte]) => decode_string_encoding(*byte)?,
        Some([]) => bail!("custom section `{CUSTOM_SECTION_NAME}` in unknown format"),
        Some([version, ..]) => bail!(
            "custom section `{CUSTOM_SECTION_NAME}` uses format {version} but only {CURRENT_VERSION} is supported"
        ),
    };
    Ok((resolve, world, string_encoding))
}

fn encode_string_encoding(e: StringEncoding) -> u8 {
    match e {
        StringEncoding::UTF8 => 0x00,
        StringEncoding::UTF16 => 0x01,
        StringEncoding::CompactUTF16 => 0x02,
    }
}

fn decode_string_encoding(byte: u8) -> Result<StringEncoding> {
    match byte {
        0x00 => Ok(StringEncoding::UTF8),
        0x01 => Ok(StringEncoding::UTF16),
        0x02 => Ok(StringEncoding::CompactUTF16),
        byte => bail!("invalid string encoding {byte:#x}"),
    }
}

impl Bindgen {
    fn decode_custom_section(data: &[u8]) -> Result<Bindgen> {
        let wasm;
        let world;
        let resolve;
        let encoding;

        let mut reader = BinaryReader::new(data, 0);
        match reader.read_u8()? {
            // Historical 0x03 format where the support here will be deleted in
            // the future
            0x03 => {
                encoding = decode_string_encoding(reader.read_u8()?)?;
                let world_name = reader.read_string()?;
                wasm = &data[reader.original_position()..];

                let (r, pkg) = match crate::decode(wasm)? {
                    DecodedWasm::WitPackage(resolve, pkgs) => (resolve, pkgs),
                    DecodedWasm::Component(..) => bail!("expected encoded wit package(s)"),
                };
                resolve = r;
                world = resolve.select_world(pkg, Some(world_name.into()))?;
            }

            // Current format where `data` is a wasm component itself.
            _ => {
                wasm = data;
                (resolve, world, encoding) = decode_custom_section(wasm)?;
            }
        }

        Ok(Bindgen {
            metadata: ModuleMetadata::new(&resolve, world, encoding),
            producers: wasm_metadata::Producers::from_wasm(wasm)?,
            resolve,
            world,
        })
    }

    /// Merges another `BindgenMetadata` into this one.
    ///
    /// This operation is intended to be akin to "merging worlds" when the
    /// abstraction level for that is what we're working at here. For now the
    /// merge operation only succeeds if the two metadata descriptions are
    /// entirely disjoint.
    ///
    /// Note that at this time there's no support for changing string encodings
    /// between metadata.
    ///
    /// This function returns the set of exports that the main world of
    /// `other` added to the world in `self`.
    pub fn merge(&mut self, other: Bindgen) -> Result<IndexSet<WorldKey>> {
        let Bindgen {
            resolve,
            world,
            metadata:
                ModuleMetadata {
                    import_encodings,
                    export_encodings,
                },
            producers,
        } = other;

        let remap = self
            .resolve
            .merge(resolve)
            .context("failed to merge WIT package sets together")?;
        let world = remap.map_world(world, None)?;
        let exports = self.resolve.worlds[world].exports.keys().cloned().collect();
        self.resolve
            .merge_worlds(world, self.world)
            .context("failed to merge worlds from two documents")?;

        self.metadata.import_encodings.merge(import_encodings)?;
        self.metadata.export_encodings.merge(export_encodings)?;
        if let Some(producers) = producers {
            if let Some(mine) = &mut self.producers {
                mine.merge(&producers);
            } else {
                self.producers = Some(producers);
            }
        }

        Ok(exports)
    }
}

impl ModuleMetadata {
    /// Creates a new `ModuleMetadata` instance holding the given set of
    /// interfaces which are expected to all use the `encoding` specified.
    pub fn new(resolve: &Resolve, world: WorldId, encoding: StringEncoding) -> ModuleMetadata {
        let mut ret = ModuleMetadata::default();

        let world = &resolve.worlds[world];
        ret.export_encodings
            .insert_all(resolve, &world.exports, encoding);
        ret.import_encodings
            .insert_all(resolve, &world.imports, encoding);

        ret
    }
}
