// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_INTO_VIEW_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_INTO_VIEW_UTIL_H_

#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/scroll/scroll_alignment.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"

namespace gfx {
class RectF;
}

namespace blink {

class LayoutObject;
class LayoutView;
struct PhysicalRect;
class ScrollableArea;
class ComputedStyle;
class ScrollIntoViewOptions;

namespace scroll_into_view_util {

// Takes the given rect, in absolute coordinates of the frame of the given
// LayoutObject, and scrolls the LayoutObject and all its containers such that
// the child content of the LayoutObject at that rect is visible in the
// viewport.
// TODO(bokan): `from_remote_frame` is temporary, to track cross-origin
// scroll-into-view prevalence. https://crbug.com/1339003.
void CORE_EXPORT ScrollRectToVisible(const LayoutObject& target,
                                     const PhysicalRect&,
                                     mojom::blink::ScrollIntoViewParamsPtr,
                                     const LayoutObject* container = nullptr,
                                     bool from_remote_frame = false);

// ScrollFocusedEditableIntoView uses the caret rect for ScrollIntoView but
// stores enough information in `params` so that the editable element's bounds
// can be reconstructed. Given `caret_rect`, this will return the editable
// element's rect (in whatever coordinate space `caret_rect` is in).
gfx::RectF FocusedEditableBoundsFromParams(
    const gfx::RectF& caret_rect,
    const mojom::blink::ScrollIntoViewParamsPtr& params);

// Whenever ScrollIntoView bubbles up across a frame boundary, the origin for
// the absolute coordinate space changes. This function will convert any
// parameters in `params` into the updated coordinate space.
void ConvertParamsToParentFrame(mojom::blink::ScrollIntoViewParamsPtr& params,
                                const gfx::RectF& caret_rect_in_src,
                                const LayoutObject& src_frame,
                                const LayoutView& dest_frame);

// Returns the scroll offset the scroller needs to scroll to in order to put
// |local_expose_rect| into |scrollable_area|'s visible scroll snapport rect
// aligned by |align_x| and |align_y|.
ScrollOffset GetScrollOffsetToExpose(
    const ScrollableArea& scrollable_area,
    const PhysicalRect& local_expose_rect,
    const PhysicalBoxStrut& expose_scroll_margin,
    const mojom::blink::ScrollAlignment& align_x,
    const mojom::blink::ScrollAlignment& align_y);

CORE_EXPORT mojom::blink::ScrollIntoViewParamsPtr CreateScrollIntoViewParams(
    const mojom::blink::ScrollAlignment& align_x =
        ScrollAlignment::CenterIfNeeded(),
    const mojom::blink::ScrollAlignment& align_y =
        ScrollAlignment::CenterIfNeeded(),
    mojom::blink::ScrollType scroll_type =
        mojom::blink::ScrollType::kProgrammatic,
    bool make_visible_in_visual_viewport = true,
    mojom::blink::ScrollBehavior scroll_behavior =
        mojom::blink::ScrollBehavior::kAuto,
    bool is_for_scroll_sequence = false,
    bool cross_origin_boundaries = true);

mojom::blink::ScrollIntoViewParamsPtr CreateScrollIntoViewParams(
    const ScrollIntoViewOptions& options,
    const ComputedStyle& computed_style);

mojom::blink::ScrollIntoViewParamsPtr CreateScrollIntoViewParams(
    const ComputedStyle& computed_style);

mojom::blink::ScrollAlignment PhysicalAlignmentFromSnapAlignStyle(
    const LayoutObject&,
    ScrollOrientation axis);

}  // namespace scroll_into_view_util

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_INTO_VIEW_UTIL_H_
