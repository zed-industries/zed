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

use crate::validation::BARE_FUNC_MODULE_NAME;
use crate::{DecodedWasm, StringEncoding};
use anyhow::{bail, Context, Result};
use indexmap::IndexMap;
use std::borrow::Cow;
use wasm_encoder::{
    ComponentBuilder, ComponentExportKind, ComponentType, ComponentTypeRef, CustomSection,
};
use wasm_metadata::Producers;
use wasmparser::{BinaryReader, Parser, Payload};
use wit_parser::{Package, PackageName, Resolve, World, WorldId, WorldItem};

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
    pub import_encodings: IndexMap<(String, String), StringEncoding>,

    /// Per-function options exported from the core wasm module, currently only
    /// related to string encoding.
    pub export_encodings: IndexMap<String, StringEncoding>,
}

/// This function will parse the `wasm` binary given as input and return a
/// [`Bindgen`] which extracts the custom sections describing component-level
/// types from within the binary itself.
///
/// This is used to parse the output of `wit-bindgen`-generated modules and is
/// one of the earliest phases in transitioning such a module to a component.
/// The extraction here provides the metadata necessary to continue the process
/// later on.
///
/// Note that a "stripped" binary where `component-type` sections are removed
/// is returned as well to embed within a component.
pub fn decode(wasm: &[u8]) -> Result<(Vec<u8>, Bindgen)> {
    let mut ret = Bindgen::default();
    let mut new_module = wasm_encoder::Module::new();

    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        let payload = payload.context("decoding item in module")?;
        match payload {
            wasmparser::Payload::CustomSection(cs) if cs.name().starts_with("component-type") => {
                let data = Bindgen::decode_custom_section(cs.data())
                    .with_context(|| format!("decoding custom section {}", cs.name()))?;
                ret.merge(data)
                    .with_context(|| format!("updating metadata for section {}", cs.name()))?;
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

    Ok((new_module.finish(), ret))
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

        let mut reader = BinaryReader::new(data);
        match reader.read_u8()? {
            // Historical 0x03 format where the support here will be deleted in
            // the future
            0x03 => {
                encoding = decode_string_encoding(reader.read_u8()?)?;
                let world_name = reader.read_string()?;
                wasm = &data[reader.original_position()..];

                let (r, pkg) = match crate::decode(wasm)? {
                    DecodedWasm::WitPackage(resolve, pkg) => (resolve, pkg),
                    DecodedWasm::Component(..) => bail!("expected an encoded wit package"),
                };
                resolve = r;
                world = resolve.packages[pkg].worlds[world_name];
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
    pub fn merge(&mut self, other: Bindgen) -> Result<WorldId> {
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

        let world = self
            .resolve
            .merge(resolve)
            .context("failed to merge WIT package sets together")?
            .worlds[world.index()];
        self.resolve
            .merge_worlds(world, self.world)
            .context("failed to merge worlds from two documents")?;

        for (name, encoding) in export_encodings {
            let prev = self
                .metadata
                .export_encodings
                .insert(name.clone(), encoding);
            if let Some(prev) = prev {
                if prev != encoding {
                    bail!("conflicting string encodings specified for export `{name}`");
                }
            }
        }
        for ((module, name), encoding) in import_encodings {
            let prev = self
                .metadata
                .import_encodings
                .insert((module.clone(), name.clone()), encoding);
            if let Some(prev) = prev {
                if prev != encoding {
                    bail!("conflicting string encodings specified for import `{module}::{name}`");
                }
            }
        }
        if let Some(producers) = producers {
            if let Some(mine) = &mut self.producers {
                mine.merge(&producers);
            } else {
                self.producers = Some(producers);
            }
        }

        Ok(world)
    }
}

impl ModuleMetadata {
    /// Creates a new `ModuleMetadata` instance holding the given set of
    /// interfaces which are expected to all use the `encoding` specified.
    pub fn new(resolve: &Resolve, world: WorldId, encoding: StringEncoding) -> ModuleMetadata {
        let mut ret = ModuleMetadata::default();

        let world = &resolve.worlds[world];
        for (name, item) in world.imports.iter() {
            let name = resolve.name_world_key(name);
            match item {
                WorldItem::Function(_) => {
                    let prev = ret
                        .import_encodings
                        .insert((BARE_FUNC_MODULE_NAME.to_string(), name.clone()), encoding);
                    assert!(prev.is_none());
                }
                WorldItem::Interface(i) => {
                    for (func, _) in resolve.interfaces[*i].functions.iter() {
                        let prev = ret
                            .import_encodings
                            .insert((name.clone(), func.clone()), encoding);
                        assert!(prev.is_none());
                    }
                }
                WorldItem::Type(_) => {}
            }
        }

        for (name, item) in world.exports.iter() {
            let name = resolve.name_world_key(name);
            match item {
                WorldItem::Function(func) => {
                    let name = func.core_export_name(None).into_owned();
                    let prev = ret.export_encodings.insert(name.clone(), encoding);
                    assert!(prev.is_none());
                }
                WorldItem::Interface(i) => {
                    for (_, func) in resolve.interfaces[*i].functions.iter() {
                        let name = func.core_export_name(Some(&name)).into_owned();
                        let prev = ret.export_encodings.insert(name, encoding);
                        assert!(prev.is_none());
                    }
                }
                WorldItem::Type(_) => {}
            }
        }

        ret
    }
}
