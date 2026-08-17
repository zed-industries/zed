//! Helper utils for tracking and patching intra unit or section references.

use gimli::write;
use gimli::{DebugInfoOffset, Reader, UnitHeader, UnitOffset};
use std::collections::HashMap;

/// Stores compiled unit references: UnitEntryId+DwAt denotes a patch location
/// and UnitOffset is a location in original DWARF.
pub struct PendingUnitRefs {
    refs: Vec<(write::UnitEntryId, gimli::DwAt, UnitOffset)>,
}

impl PendingUnitRefs {
    pub fn new() -> Self {
        Self { refs: Vec::new() }
    }
    pub fn insert(&mut self, entry_id: write::UnitEntryId, attr: gimli::DwAt, offset: UnitOffset) {
        self.refs.push((entry_id, attr, offset));
    }
}

/// Stores .debug_info references: UnitEntryId+DwAt denotes a patch location
/// and DebugInfoOffset is a location in original DWARF.
pub struct PendingDebugInfoRefs {
    refs: Vec<(write::UnitEntryId, gimli::DwAt, DebugInfoOffset)>,
}

impl PendingDebugInfoRefs {
    pub fn new() -> Self {
        Self { refs: Vec::new() }
    }
    pub fn insert(
        &mut self,
        entry_id: write::UnitEntryId,
        attr: gimli::DwAt,
        offset: DebugInfoOffset,
    ) {
        self.refs.push((entry_id, attr, offset));
    }
}

/// Stores map between read and written references of DWARF entries of
/// a compiled unit.
pub struct UnitRefsMap {
    map: HashMap<UnitOffset, write::UnitEntryId>,
}

impl UnitRefsMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, offset: UnitOffset, entry_id: write::UnitEntryId) {
        self.map.insert(offset, entry_id);
    }
    pub fn patch(&self, refs: PendingUnitRefs, comp_unit: &mut write::Unit) {
        for (die_id, attr_name, offset) in refs.refs {
            let die = comp_unit.get_mut(die_id);
            if let Some(unit_id) = self.map.get(&offset) {
                die.set(attr_name, write::AttributeValue::UnitRef(*unit_id));
            }
        }
    }
}

/// Stores map between read and written references of DWARF entries of
/// the entire .debug_info.
pub struct DebugInfoRefsMap {
    map: HashMap<DebugInfoOffset, (write::UnitId, write::UnitEntryId)>,
}

impl DebugInfoRefsMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn insert<R>(&mut self, unit: &UnitHeader<R>, unit_id: write::UnitId, unit_map: UnitRefsMap)
    where
        R: Reader<Offset = usize>,
    {
        self.map
            .extend(unit_map.map.into_iter().map(|(off, entry_id)| {
                let off = off
                    .to_debug_info_offset(unit)
                    .expect("should be in debug_info section");
                (off, (unit_id, entry_id))
            }));
    }
    pub fn patch(
        &self,
        refs: impl Iterator<Item = (write::UnitId, PendingDebugInfoRefs)>,
        units: &mut write::UnitTable,
    ) {
        for (id, refs) in refs {
            let unit = units.get_mut(id);
            for (die_id, attr_name, offset) in refs.refs {
                let die = unit.get_mut(die_id);
                if let Some((id, entry_id)) = self.map.get(&offset) {
                    die.set(
                        attr_name,
                        write::AttributeValue::DebugInfoRef(write::Reference::Entry(
                            *id, *entry_id,
                        )),
                    );
                }
            }
        }
    }
}
