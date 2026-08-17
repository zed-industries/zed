/**
 * Copyright (C) 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2007 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2009 Google, Inc.  All rights reserved.
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
 * Copyright (C) 2012 Zoltan Herczeg <zherczeg@webkit.org>.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SCOPED_SVG_PAINT_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SCOPED_SVG_PAINT_STATE_H_

#include <optional>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/paint/object_paint_properties.h"
#include "third_party/blink/renderer/core/paint/paint_info.h"
#include "third_party/blink/renderer/platform/graphics/paint/scoped_paint_chunk_properties.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutObject;
class ObjectPaintProperties;

// Hooks up the correct paint property transform node.
class ScopedSVGTransformState {
  STACK_ALLOCATED();

 public:
  ScopedSVGTransformState(const PaintInfo& paint_info,
                          const LayoutObject& object)
      : content_paint_info_(paint_info) {
    DCHECK(object.IsSVGChild());

    const auto* fragment = &object.FirstFragment();
    const auto* properties = fragment->PaintProperties();
    if (!properties)
      return;

    // TODO(https://crbug.com/1278452): Also consider Translate, Rotate,
    // Scale, and Offset.
    if (const auto* transform_node = properties->Transform()) {
      transform_property_scope_.emplace(
          paint_info.context.GetPaintController(), *transform_node, object,
          DisplayItem::PaintPhaseToSVGTransformType(paint_info.phase));
      if (auto* context_paints = paint_info.GetSvgContextPaints()) {
        transformed_context_paints_.emplace(
            context_paints->fill, context_paints->stroke,
            context_paints->transform *
                AffineTransform::FromTransform(transform_node->Matrix()));
        content_paint_info_.SetSvgContextPaints(
            base::OptionalToPtr(transformed_context_paints_));
      }
    }
  }

  PaintInfo& ContentPaintInfo() { return content_paint_info_; }

 private:
  std::optional<SvgContextPaints> transformed_context_paints_;
  std::optional<ScopedPaintChunkProperties> transform_property_scope_;
  PaintInfo content_paint_info_;
};

class ScopedSVGPaintState {
  STACK_ALLOCATED();

 public:
  ScopedSVGPaintState(const LayoutObject& object, const PaintInfo& paint_info)
      : ScopedSVGPaintState(object, paint_info, object) {}
  ScopedSVGPaintState(const LayoutObject& object,
                      const PaintInfo& paint_info,
                      const DisplayItemClient& display_item_client)
      : object_(object),
        paint_info_(paint_info),
        display_item_client_(display_item_client) {
    if (paint_info.phase == PaintPhase::kForeground)
      ApplyEffects();
  }
  ~ScopedSVGPaintState();

 private:
  void ApplyEffects();
  void ApplyPaintPropertyState(const ObjectPaintProperties&);

  const LayoutObject& object_;
  const PaintInfo& paint_info_;
  const DisplayItemClient& display_item_client_;
  std::optional<ScopedPaintChunkProperties> scoped_paint_chunk_properties_;
  bool should_paint_mask_ = false;
  bool should_paint_clip_path_as_mask_image_ = false;
#if DCHECK_IS_ON()
  bool apply_effects_called_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SCOPED_SVG_PAINT_STATE_H_
