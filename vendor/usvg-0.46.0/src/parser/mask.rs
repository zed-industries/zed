// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::sync::Arc;

use svgtypes::{Length, LengthUnit as Unit};

use super::svgtree::{AId, EId, SvgNode};
use super::{OptionLog, converter};
use crate::{Group, Mask, MaskType, Node, NonEmptyString, NonZeroRect, Transform, Units};

pub(crate) fn convert(
    node: SvgNode,
    state: &converter::State,
    object_bbox: Option<NonZeroRect>,
    cache: &mut converter::Cache,
) -> Option<Arc<Mask>> {
    // A `mask` attribute must reference a `mask` element.
    if node.tag_name() != Some(EId::Mask) {
        return None;
    }

    let units = node
        .attribute(AId::MaskUnits)
        .unwrap_or(Units::ObjectBoundingBox);

    let content_units = node
        .attribute(AId::MaskContentUnits)
        .unwrap_or(Units::UserSpaceOnUse);

    // Check if this element was already converted.
    //
    // Only `userSpaceOnUse` masks can be shared,
    // because `objectBoundingBox` one will be converted into user one
    // and will become node-specific.
    let cacheable = units == Units::UserSpaceOnUse && content_units == Units::UserSpaceOnUse;
    if cacheable {
        if let Some(mask) = cache.masks.get(node.element_id()) {
            return Some(mask.clone());
        }
    }

    let rect = NonZeroRect::from_xywh(
        node.convert_length(AId::X, units, state, Length::new(-10.0, Unit::Percent)),
        node.convert_length(AId::Y, units, state, Length::new(-10.0, Unit::Percent)),
        node.convert_length(AId::Width, units, state, Length::new(120.0, Unit::Percent)),
        node.convert_length(AId::Height, units, state, Length::new(120.0, Unit::Percent)),
    );
    let mut rect =
        rect.log_none(|| log::warn!("Mask '{}' has an invalid size. Skipped.", node.element_id()))?;

    let mut mask_all = false;
    if units == Units::ObjectBoundingBox {
        if let Some(bbox) = object_bbox {
            rect = rect.bbox_transform(bbox);
        } else {
            // When mask units are `objectBoundingBox` and bbox is zero-sized - the whole
            // element should be masked.
            // Technically an UB, but this is what Chrome and Firefox do.
            mask_all = true;
        }
    }

    let mut id = NonEmptyString::new(node.element_id().to_string())?;
    // Generate ID only when we're parsing `objectBoundingBox` mask for the second time.
    if !cacheable && cache.masks.contains_key(id.get()) {
        id = cache.gen_mask_id();
    }
    let id_copy = id.get().to_string();

    if mask_all {
        let mask = Arc::new(Mask {
            id,
            rect,
            kind: MaskType::Luminance,
            mask: None,
            root: Group::empty(),
        });
        cache.masks.insert(id_copy, mask.clone());
        return Some(mask);
    }

    // Resolve linked mask.
    let mut mask = None;
    if let Some(link) = node.attribute::<SvgNode>(AId::Mask) {
        mask = convert(link, state, object_bbox, cache);

        // Linked `mask` must be valid.
        if mask.is_none() {
            return None;
        }
    }

    let kind = if node.attribute(AId::MaskType) == Some("alpha") {
        MaskType::Alpha
    } else {
        MaskType::Luminance
    };

    let mut mask = Mask {
        id,
        rect,
        kind,
        mask,
        root: Group::empty(),
    };

    // To emulate content `objectBoundingBox` units we have to put
    // mask children into a group with a transform.
    let mut subroot = None;
    if content_units == Units::ObjectBoundingBox {
        let object_bbox = match object_bbox {
            Some(v) => v,
            None => {
                log::warn!("Masking of zero-sized shapes is not allowed.");
                return None;
            }
        };

        let mut g = Group::empty();
        g.transform = Transform::from_bbox(object_bbox);
        // Make sure to set `abs_transform`, because it must propagate to all children.
        g.abs_transform = g.transform;

        subroot = Some(g);
    }

    {
        // Prefer `subroot` to `mask.root`.
        let real_root = subroot.as_mut().unwrap_or(&mut mask.root);
        converter::convert_children(node, state, cache, real_root);

        // A mask without children at this point is invalid.
        // Only masks with zero bbox and `objectBoundingBox` can be empty.
        if !real_root.has_children() {
            return None;
        }
    }

    if let Some(mut subroot) = subroot {
        subroot.calculate_bounding_boxes();
        mask.root.children.push(Node::Group(Box::new(subroot)));
    }

    mask.root.calculate_bounding_boxes();

    let mask = Arc::new(mask);
    cache.masks.insert(id_copy, mask.clone());
    Some(mask)
}
