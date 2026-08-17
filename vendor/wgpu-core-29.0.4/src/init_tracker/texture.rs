use super::{InitTracker, MemoryInitKind};
use crate::resource::Texture;
use alloc::{sync::Arc, vec::Vec};
use arrayvec::ArrayVec;
use core::ops::Range;
use wgt::TextureSelector;

#[derive(Debug, Clone)]
pub(crate) struct TextureInitRange {
    pub(crate) mip_range: Range<u32>,
    // Strictly array layers. We do *not* track volume slices separately.
    pub(crate) layer_range: Range<u32>,
}

// Returns true if a copy operation doesn't fully cover the texture init
// tracking granularity. I.e. if this function returns true for a pending copy
// operation, the target texture needs to be ensured to be initialized first!
pub(crate) fn has_copy_partial_init_tracker_coverage(
    copy_size: &wgt::Extent3d,
    mip_level: u32,
    desc: &wgt::TextureDescriptor<(), Vec<wgt::TextureFormat>>,
) -> bool {
    let target_size = desc.mip_level_size(mip_level).unwrap();
    copy_size.width != target_size.width
        || copy_size.height != target_size.height
        || (desc.dimension == wgt::TextureDimension::D3
            && copy_size.depth_or_array_layers != target_size.depth_or_array_layers)
}

impl From<TextureSelector> for TextureInitRange {
    fn from(selector: TextureSelector) -> Self {
        TextureInitRange {
            mip_range: selector.mips,
            layer_range: selector.layers,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TextureInitTrackerAction {
    pub(crate) texture: Arc<Texture>,
    pub(crate) range: TextureInitRange,
    pub(crate) kind: MemoryInitKind,
}

pub(crate) type TextureLayerInitTracker = InitTracker<u32>;

#[derive(Debug)]
pub(crate) struct TextureInitTracker {
    pub mips: ArrayVec<TextureLayerInitTracker, { hal::MAX_MIP_LEVELS as usize }>,
}

impl TextureInitTracker {
    pub(crate) fn new(mip_level_count: u32, depth_or_array_layers: u32) -> Self {
        TextureInitTracker {
            mips: core::iter::repeat_n(
                TextureLayerInitTracker::new(depth_or_array_layers),
                mip_level_count as usize,
            )
            .collect(),
        }
    }

    pub(crate) fn check_action(
        &self,
        action: &TextureInitTrackerAction,
    ) -> Option<TextureInitTrackerAction> {
        let mut mip_range_start = usize::MAX;
        let mut mip_range_end = usize::MIN;
        let mut layer_range_start = u32::MAX;
        let mut layer_range_end = u32::MIN;

        for (i, mip_tracker) in self
            .mips
            .iter()
            .enumerate()
            .take(action.range.mip_range.end as usize)
            .skip(action.range.mip_range.start as usize)
        {
            if let Some(uninitialized_layer_range) =
                mip_tracker.check(action.range.layer_range.clone())
            {
                mip_range_start = mip_range_start.min(i);
                mip_range_end = i + 1;
                layer_range_start = layer_range_start.min(uninitialized_layer_range.start);
                layer_range_end = layer_range_end.max(uninitialized_layer_range.end);
            };
        }

        if mip_range_start < mip_range_end && layer_range_start < layer_range_end {
            Some(TextureInitTrackerAction {
                texture: action.texture.clone(),
                range: TextureInitRange {
                    mip_range: mip_range_start as u32..mip_range_end as u32,
                    layer_range: layer_range_start..layer_range_end,
                },
                kind: action.kind,
            })
        } else {
            None
        }
    }

    pub(crate) fn discard(&mut self, mip_level: u32, layer: u32) {
        self.mips[mip_level as usize].discard(layer);
    }
}
