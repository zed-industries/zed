use self::debug_transform_logging::dbi_log;
use self::refs::DebugInfoRefsMap;
use self::simulate::generate_simulated_dwarf;
use self::unit::clone_unit;
use crate::debug::Compilation;
use crate::debug::gc::build_dependencies;
use anyhow::Error;
use cranelift_codegen::isa::TargetIsa;
use gimli::{Dwarf, DwarfPackage, LittleEndian, Section, Unit, UnitSectionOffset, write};
use std::{collections::HashSet, fmt::Debug};
use synthetic::ModuleSyntheticUnit;
use thiserror::Error;
use wasmtime_environ::{
    DefinedFuncIndex, ModuleTranslation, PrimaryMap, StaticModuleIndex, Tunables,
};

pub use address_transform::AddressTransform;

mod address_transform;
mod attr;
mod debug_transform_logging;
mod expression;
mod line_program;
mod range_info_builder;
mod refs;
mod simulate;
mod synthetic;
mod unit;
mod utils;

impl<'a> Compilation<'a> {
    fn function_frame_info(
        &mut self,
        module: StaticModuleIndex,
        func: DefinedFuncIndex,
    ) -> expression::FunctionFrameInfo<'a> {
        let (_, func) = self.function(module, func);

        expression::FunctionFrameInfo {
            value_ranges: &func.value_labels_ranges,
            memory_offset: self.module_memory_offsets[module].clone(),
        }
    }
}

pub(crate) trait Reader: gimli::Reader<Offset = usize> + Send + Sync {}

impl<'input, Endian> Reader for gimli::EndianSlice<'input, Endian> where
    Endian: gimli::Endianity + Send + Sync
{
}

#[derive(Error, Debug)]
#[error("Debug info transform error: {0}")]
pub struct TransformError(&'static str);

pub(crate) struct DebugInputContext<'a> {
    reachable: &'a HashSet<UnitSectionOffset>,
}

fn load_dwp<'data>(
    translation: ModuleTranslation<'data>,
    buffer: &'data [u8],
) -> anyhow::Result<DwarfPackage<gimli::EndianSlice<'data, gimli::LittleEndian>>> {
    let endian_slice = gimli::EndianSlice::new(buffer, LittleEndian);

    let dwarf_package = DwarfPackage::load(
        |id| -> anyhow::Result<_> {
            let slice = match id {
                gimli::SectionId::DebugAbbrev => {
                    translation.debuginfo.dwarf.debug_abbrev.reader().slice()
                }
                gimli::SectionId::DebugInfo => {
                    translation.debuginfo.dwarf.debug_info.reader().slice()
                }
                gimli::SectionId::DebugLine => {
                    translation.debuginfo.dwarf.debug_line.reader().slice()
                }
                gimli::SectionId::DebugStr => {
                    translation.debuginfo.dwarf.debug_str.reader().slice()
                }
                gimli::SectionId::DebugStrOffsets => translation
                    .debuginfo
                    .dwarf
                    .debug_str_offsets
                    .reader()
                    .slice(),
                gimli::SectionId::DebugLoc => translation.debuginfo.debug_loc.reader().slice(),
                gimli::SectionId::DebugLocLists => {
                    translation.debuginfo.debug_loclists.reader().slice()
                }
                gimli::SectionId::DebugRngLists => {
                    translation.debuginfo.debug_rnglists.reader().slice()
                }
                gimli::SectionId::DebugTypes => {
                    translation.debuginfo.dwarf.debug_types.reader().slice()
                }
                gimli::SectionId::DebugCuIndex => {
                    translation.debuginfo.debug_cu_index.reader().slice()
                }
                gimli::SectionId::DebugTuIndex => {
                    translation.debuginfo.debug_tu_index.reader().slice()
                }
                _ => &buffer,
            };

            Ok(gimli::EndianSlice::new(slice, gimli::LittleEndian))
        },
        endian_slice,
    )?;

    Ok(dwarf_package)
}

/// Attempts to load a DWARF package using the passed bytes.
fn read_dwarf_package_from_bytes<'data>(
    dwp_bytes: &'data [u8],
    buffer: &'data [u8],
    tunables: &Tunables,
) -> Option<DwarfPackage<gimli::EndianSlice<'data, gimli::LittleEndian>>> {
    let mut validator = wasmparser::Validator::new();
    let parser = wasmparser::Parser::new(0);
    let mut types = wasmtime_environ::ModuleTypesBuilder::new(&validator);
    let translation =
        match wasmtime_environ::ModuleEnvironment::new(tunables, &mut validator, &mut types)
            .translate(parser, dwp_bytes)
        {
            Ok(translation) => translation,
            Err(e) => {
                log::warn!("failed to parse wasm dwarf package: {e:?}");
                return None;
            }
        };

    match load_dwp(translation, buffer) {
        Ok(package) => Some(package),
        Err(err) => {
            log::warn!("Failed to load Dwarf package {err}");
            None
        }
    }
}

pub fn transform_dwarf(
    isa: &dyn TargetIsa,
    compilation: &mut Compilation<'_>,
) -> Result<write::Dwarf, Error> {
    dbi_log!("Commencing DWARF transform for {:?}", compilation);

    let mut transforms = PrimaryMap::new();
    for (i, _) in compilation.translations.iter() {
        transforms.push(AddressTransform::new(compilation, i));
    }

    let buffer = Vec::new();

    let dwarf_package = compilation
        .dwarf_package_bytes
        .map(
            |bytes| -> Option<DwarfPackage<gimli::EndianSlice<'_, gimli::LittleEndian>>> {
                read_dwarf_package_from_bytes(bytes, &buffer, compilation.tunables)
            },
        )
        .flatten();

    let out_encoding = gimli::Encoding {
        format: gimli::Format::Dwarf32,
        version: 4, // TODO: this should be configurable
        address_size: isa.pointer_bytes(),
    };
    let mut out_strings = write::StringTable::default();
    let mut out_units = write::UnitTable::default();

    let out_line_strings = write::LineStringTable::default();
    let mut pending_di_refs = Vec::new();
    let mut di_ref_map = DebugInfoRefsMap::new();
    let mut vmctx_ptr_die_refs = PrimaryMap::new();

    let mut translated = HashSet::new();

    for (module, translation) in compilation.translations.iter() {
        dbi_log!("[== Transforming CUs for module #{} ==]", module.as_u32());

        let addr_tr = &transforms[module];
        let di = &translation.debuginfo;
        let reachable = build_dependencies(&di.dwarf, addr_tr)?.get_reachable();

        let out_module_synthetic_unit = ModuleSyntheticUnit::new(
            module,
            compilation,
            out_encoding,
            &mut out_units,
            &mut out_strings,
        );
        // TODO-DebugInfo-Cleanup: move the simulation code to be per-module and delete this map.
        vmctx_ptr_die_refs.push(out_module_synthetic_unit.vmctx_ptr_die_ref());

        let mut iter = di.dwarf.debug_info.units();
        while let Some(header) = iter.next().unwrap_or(None) {
            let unit = di.dwarf.unit(header)?;

            let mut split_unit = None;
            let mut split_dwarf = None;
            let mut split_reachable = None;

            if unit.dwo_id.is_some() {
                if let Some(dwarf_package) = &dwarf_package {
                    if let Some((fused, fused_dwarf)) =
                        replace_unit_from_split_dwarf(&unit, dwarf_package, &di.dwarf)
                    {
                        split_reachable =
                            Some(build_dependencies(&fused_dwarf, addr_tr)?.get_reachable());
                        split_unit = Some(fused);
                        split_dwarf = Some(fused_dwarf);
                    }
                }
            }
            let context = DebugInputContext {
                reachable: split_reachable.as_ref().unwrap_or(&reachable),
            };

            if let Some((id, ref_map, pending_refs)) = clone_unit(
                compilation,
                module,
                &unit,
                split_unit.as_ref(),
                split_dwarf.as_ref(),
                &context,
                &addr_tr,
                out_encoding,
                &out_module_synthetic_unit,
                &mut out_units,
                &mut out_strings,
                &mut translated,
                isa,
            )? {
                di_ref_map.insert(&header, id, ref_map);
                pending_di_refs.push((id, pending_refs));
            }
        }
    }
    di_ref_map.patch(pending_di_refs.into_iter(), &mut out_units);

    generate_simulated_dwarf(
        compilation,
        &transforms,
        &translated,
        out_encoding,
        &vmctx_ptr_die_refs,
        &mut out_units,
        &mut out_strings,
        isa,
    )?;

    Ok(write::Dwarf {
        units: out_units,
        line_programs: vec![],
        line_strings: out_line_strings,
        strings: out_strings,
    })
}

fn replace_unit_from_split_dwarf<'a>(
    unit: &'a Unit<gimli::EndianSlice<'a, gimli::LittleEndian>, usize>,
    dwp: &DwarfPackage<gimli::EndianSlice<'a, gimli::LittleEndian>>,
    parent: &Dwarf<gimli::EndianSlice<'a, gimli::LittleEndian>>,
) -> Option<(
    Unit<gimli::EndianSlice<'a, gimli::LittleEndian>, usize>,
    Dwarf<gimli::EndianSlice<'a, gimli::LittleEndian>>,
)> {
    let dwo_id = unit.dwo_id?;
    let split_unit_dwarf = dwp.find_cu(dwo_id, parent).ok()??;
    let unit_header = split_unit_dwarf.debug_info.units().next().ok()??;
    let mut split_unit = split_unit_dwarf.unit(unit_header).ok()?;
    split_unit.copy_relocated_attributes(unit);
    Some((split_unit, split_unit_dwarf))
}
