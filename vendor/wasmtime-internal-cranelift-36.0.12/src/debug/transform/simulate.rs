use super::AddressTransform;
use super::expression::{CompiledExpression, FunctionFrameInfo};
use super::utils::append_vmctx_info;
use crate::debug::Compilation;
use crate::translate::get_vmctx_value_label;
use anyhow::{Context, Error};
use cranelift_codegen::isa::TargetIsa;
use gimli::LineEncoding;
use gimli::write;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use wasmtime_environ::{
    DebugInfoData, EntityRef, FunctionMetadata, PrimaryMap, StaticModuleIndex, WasmFileInfo,
    WasmValType,
};

const PRODUCER_NAME: &str = "wasmtime";

macro_rules! assert_dwarf_str {
    ($s:expr) => {{
        let s = $s;
        if cfg!(debug_assertions) {
            // Perform check the same way as gimli does it.
            let bytes: Vec<u8> = s.clone().into();
            debug_assert!(!bytes.contains(&0), "DWARF string shall not have NULL byte");
        }
        s
    }};
}

fn generate_line_info(
    addr_tr: &PrimaryMap<StaticModuleIndex, AddressTransform>,
    translated: &HashSet<usize>,
    out_encoding: gimli::Encoding,
    w: &WasmFileInfo,
    comp_dir_id: write::StringId,
    name_id: write::StringId,
    name: &str,
) -> Result<(write::LineProgram, write::FileId), Error> {
    let out_comp_dir = write::LineString::StringRef(comp_dir_id);
    let out_comp_name = write::LineString::StringRef(name_id);

    let line_encoding = LineEncoding::default();

    let mut out_program = write::LineProgram::new(
        out_encoding,
        line_encoding,
        out_comp_dir,
        None,
        out_comp_name,
        None,
    );

    let file_index = out_program.add_file(
        write::LineString::String(name.as_bytes().to_vec()),
        out_program.default_directory(),
        None,
    );

    let maps = addr_tr.iter().flat_map(|(_, transform)| {
        transform.map().iter().filter_map(|(_, map)| {
            if translated.contains(&map.symbol) {
                None
            } else {
                Some((map.symbol, map))
            }
        })
    });

    for (symbol, map) in maps {
        let base_addr = map.offset;
        out_program.begin_sequence(Some(write::Address::Symbol {
            symbol,
            addend: base_addr as i64,
        }));

        // Always emit a row for offset zero - debuggers expect this.
        out_program.row().address_offset = 0;
        out_program.row().file = file_index;
        out_program.row().line = 0; // Special line number for non-user code.
        out_program.row().discriminator = 1;
        out_program.row().is_statement = true;
        out_program.generate_row();

        let mut is_prologue_end = true;
        for addr_map in map.addresses.iter() {
            let address_offset = (addr_map.generated - base_addr) as u64;
            out_program.row().address_offset = address_offset;
            let wasm_offset = w.code_section_offset + addr_map.wasm;
            out_program.row().line = wasm_offset;
            out_program.row().discriminator = 1;
            out_program.row().prologue_end = is_prologue_end;
            out_program.generate_row();

            is_prologue_end = false;
        }
        let end_addr = (base_addr + map.len - 1) as u64;
        out_program.end_sequence(end_addr);
    }

    Ok((out_program, file_index))
}

fn check_invalid_chars_in_name(s: &str) -> Option<&str> {
    if s.contains('\x00') { None } else { Some(s) }
}

fn autogenerate_dwarf_wasm_path(di: &DebugInfoData) -> PathBuf {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    let module_name = di
        .name_section
        .module_name
        .and_then(check_invalid_chars_in_name)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("<gen-{}>.wasm", NEXT_ID.fetch_add(1, SeqCst)));
    let path = format!("/<wasm-module>/{module_name}");
    PathBuf::from(path)
}

struct WasmTypesDieRefs {
    i32: write::UnitEntryId,
    i64: write::UnitEntryId,
    f32: write::UnitEntryId,
    f64: write::UnitEntryId,
}

fn add_wasm_types(
    unit: &mut write::Unit,
    root_id: write::UnitEntryId,
    out_strings: &mut write::StringTable,
) -> WasmTypesDieRefs {
    macro_rules! def_type {
        ($id:literal, $size:literal, $enc:path) => {{
            let die_id = unit.add(root_id, gimli::DW_TAG_base_type);
            let die = unit.get_mut(die_id);
            die.set(
                gimli::DW_AT_name,
                write::AttributeValue::StringRef(out_strings.add($id)),
            );
            die.set(gimli::DW_AT_byte_size, write::AttributeValue::Data1($size));
            die.set(gimli::DW_AT_encoding, write::AttributeValue::Encoding($enc));
            die_id
        }};
    }

    let i32_die_id = def_type!("i32", 4, gimli::DW_ATE_signed);
    let i64_die_id = def_type!("i64", 8, gimli::DW_ATE_signed);
    let f32_die_id = def_type!("f32", 4, gimli::DW_ATE_float);
    let f64_die_id = def_type!("f64", 8, gimli::DW_ATE_float);

    WasmTypesDieRefs {
        i32: i32_die_id,
        i64: i64_die_id,
        f32: f32_die_id,
        f64: f64_die_id,
    }
}

fn resolve_var_type(
    index: usize,
    wasm_types: &WasmTypesDieRefs,
    func_meta: &FunctionMetadata,
) -> Option<(write::UnitEntryId, bool)> {
    let (ty, is_param) = if index < func_meta.params.len() {
        (func_meta.params[index], true)
    } else {
        let mut i = (index - func_meta.params.len()) as u32;
        let mut j = 0;
        while j < func_meta.locals.len() && i >= func_meta.locals[j].0 {
            i -= func_meta.locals[j].0;
            j += 1;
        }
        if j >= func_meta.locals.len() {
            // Ignore the var index out of bound.
            return None;
        }
        (func_meta.locals[j].1, false)
    };
    let type_die_id = match ty {
        WasmValType::I32 => wasm_types.i32,
        WasmValType::I64 => wasm_types.i64,
        WasmValType::F32 => wasm_types.f32,
        WasmValType::F64 => wasm_types.f64,
        _ => {
            // Ignore unsupported types.
            return None;
        }
    };
    Some((type_die_id, is_param))
}

fn generate_vars(
    unit: &mut write::Unit,
    die_id: write::UnitEntryId,
    addr_tr: &AddressTransform,
    frame_info: &FunctionFrameInfo,
    scope_ranges: &[(u64, u64)],
    vmctx_ptr_die_ref: write::Reference,
    wasm_types: &WasmTypesDieRefs,
    func_meta: &FunctionMetadata,
    locals_names: Option<&HashMap<u32, &str>>,
    out_strings: &mut write::StringTable,
    isa: &dyn TargetIsa,
) -> Result<(), Error> {
    let vmctx_label = get_vmctx_value_label();

    // Normalize order of ValueLabelsRanges keys to have reproducible results.
    let mut vars = frame_info.value_ranges.keys().collect::<Vec<_>>();
    vars.sort_by(|a, b| a.index().cmp(&b.index()));

    for label in vars {
        if label.index() == vmctx_label.index() {
            append_vmctx_info(
                unit,
                die_id,
                vmctx_ptr_die_ref,
                addr_tr,
                Some(frame_info),
                scope_ranges,
                out_strings,
                isa,
            )?;
        } else {
            let var_index = label.index();
            let (type_die_id, is_param) =
                if let Some(result) = resolve_var_type(var_index, wasm_types, func_meta) {
                    result
                } else {
                    // Skipping if type of local cannot be detected.
                    continue;
                };

            let loc_list_id = {
                let locs = CompiledExpression::from_label(*label)
                    .build_with_locals(scope_ranges, addr_tr, Some(frame_info), isa)
                    .expressions
                    .map(|i| {
                        i.map(|(begin, length, data)| write::Location::StartLength {
                            begin,
                            length,
                            data,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                unit.locations.add(write::LocationList(locs))
            };

            let var_id = unit.add(
                die_id,
                if is_param {
                    gimli::DW_TAG_formal_parameter
                } else {
                    gimli::DW_TAG_variable
                },
            );
            let var = unit.get_mut(var_id);

            let name_id = match locals_names
                .and_then(|m| m.get(&(var_index as u32)))
                .and_then(|s| check_invalid_chars_in_name(s))
            {
                Some(n) => out_strings.add(assert_dwarf_str!(n)),
                None => out_strings.add(format!("var{var_index}")),
            };

            var.set(gimli::DW_AT_name, write::AttributeValue::StringRef(name_id));
            var.set(
                gimli::DW_AT_type,
                write::AttributeValue::UnitRef(type_die_id),
            );
            var.set(
                gimli::DW_AT_location,
                write::AttributeValue::LocationListRef(loc_list_id),
            );
        }
    }
    Ok(())
}

fn check_invalid_chars_in_path(path: PathBuf) -> Option<PathBuf> {
    path.clone()
        .to_str()
        .and_then(move |s| if s.contains('\x00') { None } else { Some(path) })
}

/// Generate "simulated" native DWARF for functions lacking WASM-level DWARF.
pub fn generate_simulated_dwarf(
    compilation: &mut Compilation<'_>,
    addr_tr: &PrimaryMap<StaticModuleIndex, AddressTransform>,
    translated: &HashSet<usize>,
    out_encoding: gimli::Encoding,
    vmctx_ptr_die_refs: &PrimaryMap<StaticModuleIndex, write::Reference>,
    out_units: &mut write::UnitTable,
    out_strings: &mut write::StringTable,
    isa: &dyn TargetIsa,
) -> Result<(), Error> {
    let (wasm_file, path) = {
        let di = &compilation.translations.iter().next().unwrap().1.debuginfo;
        let path = di
            .wasm_file
            .path
            .to_owned()
            .and_then(check_invalid_chars_in_path)
            .unwrap_or_else(|| autogenerate_dwarf_wasm_path(di));
        (&di.wasm_file, path)
    };

    let (unit, root_id, file_id) = {
        let comp_dir_id = out_strings.add(assert_dwarf_str!(
            path.parent()
                .context("path dir")?
                .to_str()
                .context("path dir encoding")?
        ));
        let name = path
            .file_name()
            .context("path name")?
            .to_str()
            .context("path name encoding")?;
        let name_id = out_strings.add(assert_dwarf_str!(name));

        let (out_program, file_id) = generate_line_info(
            addr_tr,
            translated,
            out_encoding,
            wasm_file,
            comp_dir_id,
            name_id,
            name,
        )?;

        let unit_id = out_units.add(write::Unit::new(out_encoding, out_program));
        let unit = out_units.get_mut(unit_id);

        let root_id = unit.root();
        let root = unit.get_mut(root_id);

        let id = out_strings.add(PRODUCER_NAME);
        root.set(gimli::DW_AT_producer, write::AttributeValue::StringRef(id));
        root.set(
            gimli::DW_AT_language,
            write::AttributeValue::Language(gimli::DW_LANG_C11),
        );
        root.set(gimli::DW_AT_name, write::AttributeValue::StringRef(name_id));
        root.set(
            gimli::DW_AT_stmt_list,
            write::AttributeValue::LineProgramRef,
        );
        root.set(
            gimli::DW_AT_comp_dir,
            write::AttributeValue::StringRef(comp_dir_id),
        );
        (unit, root_id, file_id)
    };

    let wasm_types = add_wasm_types(unit, root_id, out_strings);
    let mut unit_ranges = vec![];
    for (module, index) in compilation.indexes().collect::<Vec<_>>() {
        let (symbol, _) = compilation.function(module, index);
        if translated.contains(&symbol) {
            continue;
        }

        let addr_tr = &addr_tr[module];
        let map = &addr_tr.map()[index];
        let die_id = unit.add(root_id, gimli::DW_TAG_subprogram);
        let die = unit.get_mut(die_id);
        let low_pc = write::Address::Symbol {
            symbol,
            addend: map.offset as i64,
        };
        let code_length = map.len as u64;
        die.set(gimli::DW_AT_low_pc, write::AttributeValue::Address(low_pc));
        die.set(
            gimli::DW_AT_high_pc,
            write::AttributeValue::Udata(code_length),
        );
        unit_ranges.push(write::Range::StartLength {
            begin: low_pc,
            length: code_length,
        });

        let translation = &compilation.translations[module];
        let func_index = translation.module.func_index(index);
        let di = &translation.debuginfo;
        let id = match di
            .name_section
            .func_names
            .get(&func_index)
            .and_then(|s| check_invalid_chars_in_name(s))
        {
            Some(n) => out_strings.add(assert_dwarf_str!(n)),
            None => out_strings.add(format!("wasm-function[{}]", func_index.as_u32())),
        };

        die.set(gimli::DW_AT_name, write::AttributeValue::StringRef(id));

        die.set(
            gimli::DW_AT_decl_file,
            write::AttributeValue::FileIndex(Some(file_id)),
        );

        let f_start = map.addresses[0].wasm;
        let wasm_offset = di.wasm_file.code_section_offset + f_start;
        die.set(
            gimli::DW_AT_decl_line,
            write::AttributeValue::Udata(wasm_offset),
        );

        let frame_info = compilation.function_frame_info(module, index);
        let source_range = addr_tr.func_source_range(index);
        generate_vars(
            unit,
            die_id,
            addr_tr,
            &frame_info,
            &[(source_range.0, source_range.1)],
            vmctx_ptr_die_refs[module],
            &wasm_types,
            &di.wasm_file.funcs[index.as_u32() as usize],
            di.name_section.locals_names.get(&func_index),
            out_strings,
            isa,
        )?;
    }
    let unit_ranges_id = unit.ranges.add(write::RangeList(unit_ranges));
    unit.get_mut(root_id).set(
        gimli::DW_AT_ranges,
        write::AttributeValue::RangeListRef(unit_ranges_id),
    );

    Ok(())
}
