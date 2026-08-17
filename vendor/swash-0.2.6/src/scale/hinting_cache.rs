use alloc::vec::Vec;
use skrifa::{
    instance::{NormalizedCoord, Size},
    outline::{
        HintingInstance, HintingMode, LcdLayout, OutlineGlyphCollection, OutlineGlyphFormat,
    },
};

/// We keep this small to enable a simple LRU cache with a linear
/// search. Regenerating hinting data is low to medium cost so it's fine
/// to redo it occasionally.
const MAX_CACHED_HINT_INSTANCES: usize = 8;

pub(crate) struct HintingKey<'a> {
    pub id: [u64; 2],
    pub outlines: &'a OutlineGlyphCollection<'a>,
    pub size: Size,
    pub coords: &'a [NormalizedCoord],
}

impl<'a> HintingKey<'a> {
    fn new_instance(&self) -> Option<HintingInstance> {
        HintingInstance::new(self.outlines, self.size, self.coords, HINTING_MODE).ok()
    }
}

const HINTING_MODE: HintingMode = HintingMode::Smooth {
    lcd_subpixel: Some(LcdLayout::Horizontal),
    preserve_linear_metrics: true,
};

#[derive(Default)]
pub(super) struct HintingCache {
    // Split caches for glyf/cff because the instance type can reuse
    // internal memory when reconfigured for the same format.
    glyf_entries: Vec<HintingEntry>,
    cff_entries: Vec<HintingEntry>,
    serial: u64,
}

impl HintingCache {
    pub(super) fn get(&mut self, key: &HintingKey) -> Option<&HintingInstance> {
        let entries = match key.outlines.format()? {
            OutlineGlyphFormat::Glyf => &mut self.glyf_entries,
            OutlineGlyphFormat::Cff | OutlineGlyphFormat::Cff2 => &mut self.cff_entries,
        };
        let (entry_ix, is_current) = find_hinting_entry(entries, key)?;
        let entry = entries.get_mut(entry_ix)?;
        self.serial += 1;
        entry.serial = self.serial;
        if !is_current {
            entry.id = key.id;
            entry
                .instance
                .reconfigure(key.outlines, key.size, key.coords, HINTING_MODE)
                .ok()?;
        }
        Some(&entry.instance)
    }
}

struct HintingEntry {
    id: [u64; 2],
    instance: HintingInstance,
    serial: u64,
}

fn find_hinting_entry(entries: &mut Vec<HintingEntry>, key: &HintingKey) -> Option<(usize, bool)> {
    let mut found_serial = u64::MAX;
    let mut found_index = 0;
    for (ix, entry) in entries.iter().enumerate() {
        if entry.id == key.id
            && entry.instance.size() == key.size
            && entry.instance.location().coords() == key.coords
        {
            return Some((ix, true));
        }
        if entry.serial < found_serial {
            found_serial = entry.serial;
            found_index = ix;
        }
    }
    if entries.len() < MAX_CACHED_HINT_INSTANCES {
        let instance = key.new_instance()?;
        let ix = entries.len();
        entries.push(HintingEntry {
            id: key.id,
            instance,
            serial: 0,
        });
        Some((ix, true))
    } else {
        Some((found_index, false))
    }
}
