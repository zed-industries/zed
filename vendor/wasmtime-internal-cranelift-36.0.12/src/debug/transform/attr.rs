use crate::debug::Reader;
use crate::debug::transform::utils::resolve_die_ref;

use super::address_transform::AddressTransform;
use super::expression::{CompiledExpression, FunctionFrameInfo, compile_expression};
use super::range_info_builder::RangeInfoBuilder;
use super::refs::{PendingDebugInfoRefs, PendingUnitRefs};
use super::unit::InheritedAttr;
use super::{TransformError, dbi_log};
use anyhow::{Error, bail};
use cranelift_codegen::isa::TargetIsa;
use gimli::{
    AttributeValue, DebugLineOffset, DebuggingInformationEntry, Dwarf, Unit, UnitOffset, write,
};

#[derive(Debug)]
pub(crate) enum EntryAttributesContext<'a> {
    Root(Option<DebugLineOffset>),
    Children {
        depth: usize,
        subprograms: &'a mut InheritedAttr<SubprogramContext>,
        file_map: &'a [write::FileId],
        file_index_base: u64,
        frame_base: Option<&'a CompiledExpression>,
    },
}

#[derive(Debug)]
pub struct SubprogramContext {
    pub obj_ptr: UnitOffset,
    pub param_num: isize,
}

fn is_exprloc_to_loclist_allowed(attr_name: gimli::constants::DwAt) -> bool {
    match attr_name {
        gimli::DW_AT_location
        | gimli::DW_AT_string_length
        | gimli::DW_AT_return_addr
        | gimli::DW_AT_data_member_location
        | gimli::DW_AT_frame_base
        | gimli::DW_AT_segment
        | gimli::DW_AT_static_link
        | gimli::DW_AT_use_location
        | gimli::DW_AT_vtable_elem_location => true,
        _ => false,
    }
}

pub(crate) fn clone_die_attributes<'a>(
    dwarf: &gimli::Dwarf<Reader<'a>>,
    unit: &Unit<Reader<'a>>,
    entry: &DebuggingInformationEntry<Reader<'a>>,
    addr_tr: &'a AddressTransform,
    frame_info: Option<&FunctionFrameInfo>,
    out_unit: &mut write::Unit,
    out_entry_id: write::UnitEntryId,
    subprogram_range_builder: Option<RangeInfoBuilder>,
    scope_ranges: Option<&Vec<(u64, u64)>>,
    out_strings: &mut write::StringTable,
    pending_die_refs: &mut PendingUnitRefs,
    pending_di_refs: &mut PendingDebugInfoRefs,
    mut attr_context: EntryAttributesContext<'a>,
    isa: &dyn TargetIsa,
) -> Result<(), Error> {
    let unit_encoding = unit.encoding();

    let range_info = if let Some(subprogram_range_builder) = subprogram_range_builder {
        subprogram_range_builder
    } else {
        // FIXME for CU: currently address_transform operate on a single
        // function range, and when CU spans multiple ranges the
        // transformation may be incomplete.
        RangeInfoBuilder::from(dwarf, unit, entry)?
    };
    range_info.build(addr_tr, out_unit, out_entry_id);

    let mut is_obj_ptr = false;
    prepare_die_context(dwarf, unit, entry, &mut attr_context, &mut is_obj_ptr)?;

    let mut attrs = entry.attrs();
    while let Some(attr) = attrs.next()? {
        match attr.name() {
            gimli::DW_AT_low_pc | gimli::DW_AT_high_pc | gimli::DW_AT_ranges => {
                // Handled by RangeInfoBuilder.
                continue;
            }
            gimli::DW_AT_object_pointer => {
                // Our consumers cannot handle 'this' typed as a non-pointer (recall
                // we translate all pointers to wrapper types), making it unusable.
                // To remedy this, we 'strip' instance-ness off of methods by removing
                // DW_AT_object_pointer and renaming 'this' to '__this'.
                if let EntryAttributesContext::Children {
                    depth,
                    ref mut subprograms,
                    ..
                } = attr_context
                {
                    if let Some(ref mut subprogram) = subprograms.top_with_depth_mut(depth) {
                        // We expect this to reference a child entry in the same unit.
                        if let Some(unit_offs) = match attr.value() {
                            AttributeValue::DebugInfoRef(di_ref) => {
                                di_ref.to_unit_offset(&unit.header)
                            }
                            AttributeValue::UnitRef(unit_ref) => Some(unit_ref),
                            _ => None,
                        } {
                            subprogram.obj_ptr = unit_offs;
                            dbi_log!("Stripped DW_AT_object_pointer");
                            continue;
                        }
                    }
                }
            }
            gimli::DW_AT_str_offsets_base
            | gimli::DW_AT_addr_base
            | gimli::DW_AT_rnglists_base
            | gimli::DW_AT_loclists_base
            | gimli::DW_AT_dwo_name
            | gimli::DW_AT_GNU_addr_base
            | gimli::DW_AT_GNU_ranges_base
            | gimli::DW_AT_GNU_dwo_name
            | gimli::DW_AT_GNU_dwo_id => {
                // DWARF encoding details that we don't need to copy.
                continue;
            }
            _ => {}
        }

        if is_obj_ptr {
            match attr.name() {
                gimli::DW_AT_artificial => {
                    dbi_log!("Object pointer: stripped DW_AT_artificial");
                    continue;
                }
                gimli::DW_AT_name => {
                    let old_name: &str = &dwarf.attr_string(unit, attr.value())?.to_string_lossy();
                    let new_name = format!("__{old_name}");
                    dbi_log!(
                        "Object pointer: renamed '{}' -> '{}'",
                        old_name,
                        new_name.as_str()
                    );

                    let attr_value = write::AttributeValue::StringRef(out_strings.add(new_name));
                    out_unit
                        .get_mut(out_entry_id)
                        .set(gimli::DW_AT_name, attr_value);
                    continue;
                }
                _ => {}
            }
        }

        let attr_value = attr.value();
        let out_attr_value = match attr_value {
            AttributeValue::Addr(u) => {
                let addr = addr_tr.translate(u).unwrap_or(write::Address::Constant(0));
                write::AttributeValue::Address(addr)
            }
            AttributeValue::DebugAddrIndex(i) => {
                let u = dwarf.address(unit, i)?;
                let addr = addr_tr.translate(u).unwrap_or(write::Address::Constant(0));
                write::AttributeValue::Address(addr)
            }
            AttributeValue::Block(d) => write::AttributeValue::Block(d.to_vec()),
            AttributeValue::Udata(u) => write::AttributeValue::Udata(u),
            AttributeValue::Data1(d) => write::AttributeValue::Data1(d),
            AttributeValue::Data2(d) => write::AttributeValue::Data2(d),
            AttributeValue::Data4(d) => write::AttributeValue::Data4(d),
            AttributeValue::Data8(d) => write::AttributeValue::Data8(d),
            AttributeValue::Sdata(d) => write::AttributeValue::Sdata(d),
            AttributeValue::Flag(f) => write::AttributeValue::Flag(f),
            AttributeValue::DebugLineRef(line_program_offset) => {
                if let EntryAttributesContext::Root(o) = attr_context {
                    if o != Some(line_program_offset) {
                        return Err(TransformError("invalid debug_line offset").into());
                    }
                    write::AttributeValue::LineProgramRef
                } else {
                    return Err(TransformError("unexpected debug_line index attribute").into());
                }
            }
            AttributeValue::FileIndex(i) => {
                if let EntryAttributesContext::Children {
                    file_map,
                    file_index_base,
                    ..
                } = attr_context
                {
                    let index = usize::try_from(i - file_index_base)
                        .ok()
                        .and_then(|i| file_map.get(i).copied());
                    match index {
                        Some(index) => write::AttributeValue::FileIndex(Some(index)),
                        // This was seen to be invalid in #8884 and #8904 so
                        // ignore this seemingly invalid DWARF from LLVM
                        None => continue,
                    }
                } else {
                    return Err(TransformError("unexpected file index attribute").into());
                }
            }
            AttributeValue::String(d) => write::AttributeValue::String(d.to_vec()),
            AttributeValue::DebugStrRef(_) | AttributeValue::DebugStrOffsetsIndex(_) => {
                let s = dwarf
                    .attr_string(unit, attr_value)?
                    .to_string_lossy()
                    .into_owned();
                write::AttributeValue::StringRef(out_strings.add(s))
            }
            AttributeValue::RangeListsRef(_) | AttributeValue::DebugRngListsIndex(_) => {
                let r = dwarf.attr_ranges_offset(unit, attr_value)?.unwrap();
                let range_info = RangeInfoBuilder::from_ranges_ref(dwarf, unit, r)?;
                let range_list_id = range_info.build_ranges(addr_tr, &mut out_unit.ranges);
                write::AttributeValue::RangeListRef(range_list_id)
            }
            AttributeValue::LocationListsRef(_) | AttributeValue::DebugLocListsIndex(_) => {
                let r = dwarf.attr_locations_offset(unit, attr_value)?.unwrap();
                let low_pc = 0;
                let mut locs = dwarf.locations.locations(
                    r,
                    unit_encoding,
                    low_pc,
                    &dwarf.debug_addr,
                    unit.addr_base,
                )?;
                let frame_base =
                    if let EntryAttributesContext::Children { frame_base, .. } = attr_context {
                        frame_base
                    } else {
                        None
                    };

                let mut result: Option<Vec<_>> = None;
                while let Some(loc) = locs.next()? {
                    if let Some(expr) = compile_expression(&loc.data, unit_encoding, frame_base)? {
                        let chunk = expr
                            .build_with_locals(
                                &[(loc.range.begin, loc.range.end)],
                                addr_tr,
                                frame_info,
                                isa,
                            )
                            .expressions
                            .filter(|i| {
                                // Ignore empty range
                                if let Ok((_, 0, _)) = i { false } else { true }
                            })
                            .map(|i| {
                                i.map(|(start, len, expr)| write::Location::StartLength {
                                    begin: start,
                                    length: len,
                                    data: expr,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        match &mut result {
                            Some(r) => r.extend(chunk),
                            x @ None => *x = Some(chunk),
                        }
                    } else {
                        // FIXME _expr contains invalid expression
                        continue; // ignore entry
                    }
                }
                if result.is_none() {
                    continue; // no valid locations
                }
                let list_id = out_unit.locations.add(write::LocationList(result.unwrap()));
                write::AttributeValue::LocationListRef(list_id)
            }
            AttributeValue::Exprloc(_) if attr.name() == gimli::DW_AT_frame_base => {
                // We do not really "rewrite" the frame base so much as replace it outright.
                // References to it through the DW_OP_fbreg opcode will be expanded below.
                let mut cfa = write::Expression::new();
                cfa.op(gimli::DW_OP_call_frame_cfa);
                write::AttributeValue::Exprloc(cfa)
            }
            AttributeValue::Exprloc(ref expr) => {
                let frame_base =
                    if let EntryAttributesContext::Children { frame_base, .. } = attr_context {
                        frame_base
                    } else {
                        None
                    };
                if let Some(expr) = compile_expression(expr, unit_encoding, frame_base)? {
                    if expr.is_simple() {
                        if let Some(expr) = expr.build() {
                            write::AttributeValue::Exprloc(expr)
                        } else {
                            continue;
                        }
                    } else {
                        // Conversion to loclist is required.
                        if let Some(scope_ranges) = scope_ranges {
                            let built_expression =
                                expr.build_with_locals(scope_ranges, addr_tr, frame_info, isa);
                            let exprs = built_expression
                                .expressions
                                .collect::<Result<Vec<_>, _>>()?;
                            if exprs.is_empty() {
                                continue;
                            }
                            // Micro-optimization all expressions alike, use one exprloc.
                            let mut single_expr: Option<write::Expression> = None;
                            if built_expression.covers_entire_scope {
                                for (_, _, expr) in &exprs {
                                    if let Some(ref prev_expr) = single_expr {
                                        if expr == prev_expr {
                                            continue; // the same expression
                                        }
                                        single_expr = None;
                                        break;
                                    }
                                    single_expr = Some(expr.clone())
                                }
                            }
                            if let Some(expr) = single_expr {
                                write::AttributeValue::Exprloc(expr)
                            } else if is_exprloc_to_loclist_allowed(attr.name()) {
                                // Converting exprloc to loclist.
                                let mut locs = Vec::new();
                                for (begin, length, data) in exprs {
                                    if length == 0 {
                                        // Ignore empty range
                                        continue;
                                    }
                                    locs.push(write::Location::StartLength {
                                        begin,
                                        length,
                                        data,
                                    });
                                }
                                let list_id = out_unit.locations.add(write::LocationList(locs));
                                write::AttributeValue::LocationListRef(list_id)
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }
                } else {
                    // FIXME _expr contains invalid expression
                    continue; // ignore attribute
                }
            }
            AttributeValue::Encoding(e) => write::AttributeValue::Encoding(e),
            AttributeValue::DecimalSign(e) => write::AttributeValue::DecimalSign(e),
            AttributeValue::Endianity(e) => write::AttributeValue::Endianity(e),
            AttributeValue::Accessibility(e) => write::AttributeValue::Accessibility(e),
            AttributeValue::Visibility(e) => write::AttributeValue::Visibility(e),
            AttributeValue::Virtuality(e) => write::AttributeValue::Virtuality(e),
            AttributeValue::Language(e) => write::AttributeValue::Language(e),
            AttributeValue::AddressClass(e) => write::AttributeValue::AddressClass(e),
            AttributeValue::IdentifierCase(e) => write::AttributeValue::IdentifierCase(e),
            AttributeValue::CallingConvention(e) => write::AttributeValue::CallingConvention(e),
            AttributeValue::Inline(e) => write::AttributeValue::Inline(e),
            AttributeValue::Ordering(e) => write::AttributeValue::Ordering(e),
            AttributeValue::UnitRef(offset) => {
                pending_die_refs.insert(out_entry_id, attr.name(), offset);
                continue;
            }
            AttributeValue::DebugInfoRef(offset) => {
                pending_di_refs.insert(out_entry_id, attr.name(), offset);
                continue;
            }
            a => bail!("Unexpected attribute: {:?}", a),
        };
        let out_entry: &mut write::DebuggingInformationEntry = out_unit.get_mut(out_entry_id);
        out_entry.set(attr.name(), out_attr_value);
    }
    Ok(())
}

fn prepare_die_context(
    dwarf: &Dwarf<Reader<'_>>,
    unit: &Unit<Reader<'_>>,
    entry: &DebuggingInformationEntry<Reader<'_>>,
    attr_context: &mut EntryAttributesContext<'_>,
    is_obj_ptr: &mut bool,
) -> Result<(), Error> {
    let EntryAttributesContext::Children {
        depth, subprograms, ..
    } = attr_context
    else {
        return Ok(());
    };

    // Update the current context based on what kind of entry this is.
    match entry.tag() {
        gimli::DW_TAG_subprogram | gimli::DW_TAG_inlined_subroutine | gimli::DW_TAG_entry_point => {
            // Push the 'context' of there being no parameters (yet).
            subprograms.push(
                *depth,
                SubprogramContext {
                    obj_ptr: UnitOffset { 0: 0 },
                    param_num: -1,
                },
            );
        }
        gimli::DW_TAG_formal_parameter => {
            // Formal parameter tags can be parented by catch blocks
            // and such - not just subprogram DIEs. So we need to check
            // that this DIE is indeed a direct child of a subprogram.
            if let Some(subprogram) = subprograms.top_with_depth_mut(*depth - 1) {
                subprogram.param_num += 1;

                if subprogram.obj_ptr == entry.offset()
                    || is_obj_ptr_param(dwarf, unit, entry, subprogram.param_num)?
                {
                    *is_obj_ptr = true;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn is_obj_ptr_param(
    dwarf: &Dwarf<Reader<'_>>,
    unit: &Unit<Reader<'_>>,
    entry: &DebuggingInformationEntry<Reader<'_>>,
    param_num: isize,
) -> Result<bool, Error> {
    debug_assert!(entry.tag() == gimli::DW_TAG_formal_parameter);

    // This logic was taken loosely from LLDB. It is known
    // that it is not fully correct (doesn't handle 'deduced
    // this', for example).
    // Q: DWARF includes DW_AT_object_pointer as we use it,
    // why do we need this heuristic as well?
    // A: Declarations do not include DW_AT_object_pointer.
    if param_num == 0
        && entry.attr_value(gimli::DW_AT_artificial)? == Some(AttributeValue::Flag(true))
    {
        // Either this has no name (declarations omit them), or its explicitly "this".
        let name = entry.attr_value(gimli::DW_AT_name)?;
        if name.is_none() || dwarf.attr_string(unit, name.unwrap())?.slice().eq(b"this") {
            // Finally, a type check. We expect a pointer.
            if let Some(type_attr) = entry.attr_value(gimli::DW_AT_type)? {
                if let Some(type_die) = resolve_die_ref(unit, &type_attr)? {
                    return Ok(type_die.tag() == gimli::DW_TAG_pointer_type);
                }
            }
        }
    };

    return Ok(false);
}
