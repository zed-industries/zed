use super::Reader;
use super::address_transform::AddressTransform;
use anyhow::Error;
use gimli::{AttributeValue, DebuggingInformationEntry, RangeListsOffset, Unit, write};
use wasmtime_environ::DefinedFuncIndex;

pub(crate) enum RangeInfoBuilder {
    Undefined,
    Position(u64),
    Ranges(Vec<(u64, u64)>),
    Function(DefinedFuncIndex),
}

impl RangeInfoBuilder {
    pub(crate) fn from<R>(
        dwarf: &gimli::Dwarf<R>,
        unit: &Unit<R, R::Offset>,
        entry: &DebuggingInformationEntry<R>,
    ) -> Result<Self, Error>
    where
        R: Reader,
    {
        if let Some(AttributeValue::RangeListsRef(r)) = entry.attr_value(gimli::DW_AT_ranges)? {
            let r = dwarf.ranges_offset_from_raw(unit, r);
            return RangeInfoBuilder::from_ranges_ref(dwarf, unit, r);
        };

        let low_pc =
            if let Some(AttributeValue::Addr(addr)) = entry.attr_value(gimli::DW_AT_low_pc)? {
                addr
            } else if let Some(AttributeValue::DebugAddrIndex(i)) =
                entry.attr_value(gimli::DW_AT_low_pc)?
            {
                dwarf.address(unit, i)?
            } else {
                return Ok(RangeInfoBuilder::Undefined);
            };

        Ok(
            if let Some(AttributeValue::Udata(u)) = entry.attr_value(gimli::DW_AT_high_pc)? {
                RangeInfoBuilder::Ranges(vec![(low_pc, low_pc + u)])
            } else {
                RangeInfoBuilder::Position(low_pc)
            },
        )
    }

    pub(crate) fn from_ranges_ref<R>(
        dwarf: &gimli::Dwarf<R>,
        unit: &Unit<R, R::Offset>,
        ranges: RangeListsOffset,
    ) -> Result<Self, Error>
    where
        R: Reader,
    {
        let mut ranges = dwarf.ranges(unit, ranges)?;
        let mut result = Vec::new();
        while let Some(range) = ranges.next()? {
            if range.begin >= range.end {
                // ignore empty ranges
            }
            result.push((range.begin, range.end));
        }

        Ok(if result.is_empty() {
            RangeInfoBuilder::Undefined
        } else {
            RangeInfoBuilder::Ranges(result)
        })
    }

    pub(crate) fn from_subprogram_die<R>(
        dwarf: &gimli::Dwarf<R>,
        unit: &Unit<R, R::Offset>,
        entry: &DebuggingInformationEntry<R>,
        addr_tr: &AddressTransform,
    ) -> Result<Self, Error>
    where
        R: Reader,
    {
        let addr =
            if let Some(AttributeValue::Addr(addr)) = entry.attr_value(gimli::DW_AT_low_pc)? {
                addr
            } else if let Some(AttributeValue::DebugAddrIndex(i)) =
                entry.attr_value(gimli::DW_AT_low_pc)?
            {
                dwarf.address(unit, i)?
            } else if let Some(AttributeValue::RangeListsRef(r)) =
                entry.attr_value(gimli::DW_AT_ranges)?
            {
                let r = dwarf.ranges_offset_from_raw(unit, r);
                let mut ranges = dwarf.ranges(unit, r)?;
                if let Some(range) = ranges.next()? {
                    range.begin
                } else {
                    return Ok(RangeInfoBuilder::Undefined);
                }
            } else {
                return Ok(RangeInfoBuilder::Undefined);
            };

        let index = addr_tr.find_func_index(addr);
        if index.is_none() {
            return Ok(RangeInfoBuilder::Undefined);
        }
        Ok(RangeInfoBuilder::Function(index.unwrap()))
    }

    pub(crate) fn build(
        &self,
        addr_tr: &AddressTransform,
        out_unit: &mut write::Unit,
        current_scope_id: write::UnitEntryId,
    ) {
        match self {
            RangeInfoBuilder::Undefined => (),
            RangeInfoBuilder::Position(pc) => {
                let addr = addr_tr
                    .translate(*pc)
                    .unwrap_or(write::Address::Constant(0));
                let current_scope = out_unit.get_mut(current_scope_id);
                current_scope.set(gimli::DW_AT_low_pc, write::AttributeValue::Address(addr));
            }
            RangeInfoBuilder::Ranges(ranges) => {
                let mut result = Vec::new();
                for (begin, end) in ranges {
                    result.extend(addr_tr.translate_ranges(*begin, *end));
                }

                // If we're seeing the ranges for a `DW_TAG_compile_unit` DIE
                // then don't use `DW_AT_low_pc` and `DW_AT_high_pc`. These
                // attributes, if set, will configure the base address of all
                // location lists that this unit refers to. Currently this
                // debug transform does not take this base address into account
                // when generate the `.debug_loc` section. Consequently when a
                // compile unit is configured here the `DW_AT_ranges` attribute
                // is unconditionally used instead of
                // `DW_AT_low_pc`/`DW_AT_high_pc`.
                let is_attr_for_compile_unit =
                    out_unit.get(current_scope_id).tag() == gimli::DW_TAG_compile_unit;

                if result.len() != 1 || is_attr_for_compile_unit {
                    let range_list = result
                        .iter()
                        .map(|tr| write::Range::StartLength {
                            begin: tr.0,
                            length: tr.1,
                        })
                        .collect::<Vec<_>>();
                    let range_list_id = out_unit.ranges.add(write::RangeList(range_list));
                    let current_scope = out_unit.get_mut(current_scope_id);
                    current_scope.set(
                        gimli::DW_AT_ranges,
                        write::AttributeValue::RangeListRef(range_list_id),
                    );
                } else {
                    let current_scope = out_unit.get_mut(current_scope_id);
                    current_scope.set(
                        gimli::DW_AT_low_pc,
                        write::AttributeValue::Address(result[0].0),
                    );
                    current_scope.set(
                        gimli::DW_AT_high_pc,
                        write::AttributeValue::Udata(result[0].1),
                    );
                }
            }
            RangeInfoBuilder::Function(index) => {
                let symbol = addr_tr.map()[*index].symbol;
                let range = addr_tr.func_range(*index);
                let addr = write::Address::Symbol {
                    symbol,
                    addend: range.0 as i64,
                };
                let len = (range.1 - range.0) as u64;
                let current_scope = out_unit.get_mut(current_scope_id);
                current_scope.set(gimli::DW_AT_low_pc, write::AttributeValue::Address(addr));
                current_scope.set(gimli::DW_AT_high_pc, write::AttributeValue::Udata(len));
            }
        }
    }

    pub(crate) fn get_ranges(&self, addr_tr: &AddressTransform) -> Vec<(u64, u64)> {
        match self {
            RangeInfoBuilder::Undefined | RangeInfoBuilder::Position(_) => vec![],
            RangeInfoBuilder::Ranges(ranges) => ranges.clone(),
            RangeInfoBuilder::Function(index) => {
                let range = addr_tr.func_source_range(*index);
                vec![(range.0, range.1)]
            }
        }
    }

    pub(crate) fn build_ranges(
        &self,
        addr_tr: &AddressTransform,
        out_range_lists: &mut write::RangeListTable,
    ) -> write::RangeListId {
        if let RangeInfoBuilder::Ranges(ranges) = self {
            let mut range_list = Vec::new();
            for (begin, end) in ranges {
                assert!(begin < end);
                range_list.extend(addr_tr.translate_ranges(*begin, *end).map(|tr| {
                    write::Range::StartLength {
                        begin: tr.0,
                        length: tr.1,
                    }
                }));
            }
            out_range_lists.add(write::RangeList(range_list))
        } else {
            unreachable!();
        }
    }
}
