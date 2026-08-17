use crate::hb::common::{HB_FEATURE_GLOBAL_END, HB_FEATURE_GLOBAL_START};
use crate::Feature;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;

use super::aat_layout::*;
use super::{hb_font_t, hb_mask_t, hb_tag_t};

#[derive(Default)]
pub struct hb_aat_map_t {
    pub chain_flags: Vec<Vec<range_flags_t>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct feature_info_t {
    pub kind: u16,
    pub setting: u16,
    pub is_exclusive: bool,
}

impl Ord for feature_info_t {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for feature_info_t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.kind != other.kind {
            Some(self.kind.cmp(&other.kind))
        } else if !self.is_exclusive && (self.setting & !1) != (other.setting & !1) {
            Some(self.setting.cmp(&other.setting))
        } else {
            Some(core::cmp::Ordering::Equal)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct feature_range_t {
    pub info: feature_info_t,
    pub start: u32,
    pub end: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct feature_event_t {
    pub index: usize,
    pub start: bool,
    pub feature: feature_info_t,
}

#[derive(Copy, Clone)]
pub struct range_flags_t {
    pub flags: hb_mask_t,
    pub cluster_first: u32,
    pub cluster_last: u32, // end - 1
}

impl Ord for feature_event_t {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for feature_event_t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.index != other.index {
            Some(self.index.cmp(&other.index))
        } else if self.start != other.start {
            Some(self.start.cmp(&other.start))
        } else {
            Some(Ordering::Equal)
        }
    }
}

pub struct hb_aat_map_builder_t {
    pub current_features: Vec<feature_info_t>,
    pub features: Vec<feature_range_t>,
    pub range_first: usize,
    pub range_last: usize,
}

impl Default for hb_aat_map_builder_t {
    fn default() -> Self {
        Self {
            range_first: HB_FEATURE_GLOBAL_START as usize,
            range_last: HB_FEATURE_GLOBAL_END as usize,
            current_features: Vec::default(),
            features: Vec::default(),
        }
    }
}

impl hb_aat_map_builder_t {
    pub fn add_feature(&mut self, face: &hb_font_t, feature: &Feature) -> Option<()> {
        let feat = face.tables().feat?;

        if feature.tag == hb_tag_t::from_bytes(b"aalt") {
            let exposes_feature = feat
                .names
                .find(HB_AAT_LAYOUT_FEATURE_TYPE_CHARACTER_ALTERNATIVES as u16)
                .map(|f| !f.setting_names.is_empty())
                .unwrap_or(false);

            if !exposes_feature {
                return Some(());
            }

            self.features.push(feature_range_t {
                start: feature.start,
                end: feature.end,
                info: feature_info_t {
                    kind: HB_AAT_LAYOUT_FEATURE_TYPE_CHARACTER_ALTERNATIVES as u16,
                    setting: u16::try_from(feature.value).unwrap(),
                    is_exclusive: true,
                },
            });
        }

        let idx = feature_mappings
            .binary_search_by(|map| map.ot_feature_tag.cmp(&feature.tag))
            .ok()?;
        let mapping = &feature_mappings[idx];

        let mut feature_name = feat.names.find(mapping.aat_feature_type as u16);

        match feature_name {
            Some(feature) if !feature.setting_names.is_empty() => {}
            _ => {
                // Special case: Chain::compile_flags will fall back to the deprecated version of
                // small-caps if necessary, so we need to check for that possibility.
                // https://github.com/harfbuzz/harfbuzz/issues/2307
                if mapping.aat_feature_type == HB_AAT_LAYOUT_FEATURE_TYPE_LOWER_CASE
                    && mapping.selector_to_enable
                        == HB_AAT_LAYOUT_FEATURE_SELECTOR_LOWER_CASE_SMALL_CAPS
                {
                    feature_name = feat
                        .names
                        .find(HB_AAT_LAYOUT_FEATURE_TYPE_LETTER_CASE as u16);
                }
            }
        }

        match feature_name {
            Some(feature_name) if !feature_name.setting_names.is_empty() => {
                let setting = if feature.value != 0 {
                    mapping.selector_to_enable
                } else {
                    mapping.selector_to_disable
                } as u16;

                self.features.push(feature_range_t {
                    start: feature.start,
                    end: feature.end,
                    info: feature_info_t {
                        kind: mapping.aat_feature_type as u16,
                        setting,
                        is_exclusive: feature_name.exclusive,
                    },
                });
            }
            _ => {}
        }

        Some(())
    }

    pub fn compile(&mut self, face: &hb_font_t, m: &mut hb_aat_map_t) {
        // Compute active features per range, and compile each.
        let mut feature_events = vec![];
        for feature in &self.features {
            if feature.start == feature.end {
                continue;
            }

            feature_events.push(feature_event_t {
                index: feature.start as usize,
                start: true,
                feature: feature.info,
            });

            feature_events.push(feature_event_t {
                index: feature.end as usize,
                start: false,
                feature: feature.info,
            })
        }

        feature_events.sort();

        // Add a strategic final event.
        feature_events.push(feature_event_t {
            index: u32::MAX as usize,
            start: false,
            feature: feature_info_t::default(),
        });

        // Scan events and save features for each range.
        let mut active_features = vec![];
        let mut last_index = 0;

        for event in &feature_events {
            if event.index != last_index {
                // Save a snapshot of active features and the range.
                // Sort features and merge duplicates.
                self.current_features = active_features.clone();
                self.range_first = last_index;
                self.range_last = event.index.wrapping_sub(1);

                if !self.current_features.is_empty() {
                    self.current_features.sort();
                    let mut j = 0;
                    for i in 1..self.current_features.len() {
                        // Nonexclusive feature selectors come in even/odd pairs to turn a setting on/off
                        // respectively, so we mask out the low-order bit when checking for "duplicates"
                        // (selectors referring to the same feature setting) here.
                        let non_exclusive = !self.current_features[i].is_exclusive
                            && (self.current_features[i].setting & !1)
                                != (self.current_features[j].setting & !1);

                        if self.current_features[i].kind != self.current_features[j].kind
                            || non_exclusive
                        {
                            j += 1;
                            self.current_features[j] = self.current_features[i];
                        }
                    }
                    self.current_features.truncate(j + 1);
                }

                super::aat_layout_morx_table::compile_flags(face, self, m);
                last_index = event.index;
            }

            if event.start {
                active_features.push(event.feature);
            } else {
                if let Some(index) = active_features.iter().position(|&f| f == event.feature) {
                    active_features.remove(index);
                }
            }
        }

        for chain_flags in m.chain_flags.iter_mut() {
            if let Some(last) = chain_flags.last_mut() {
                last.cluster_last = HB_FEATURE_GLOBAL_END;
            }
        }
    }
}
