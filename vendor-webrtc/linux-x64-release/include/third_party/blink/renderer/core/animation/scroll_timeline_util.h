// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_TIMELINE_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_TIMELINE_UTIL_H_

#include <optional>

#include "cc/animation/scroll_timeline.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_scroll_axis.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation.h"

namespace blink {

using CompositorScrollTimeline = cc::ScrollTimeline;
using ScrollOffsets = cc::ScrollTimeline::ScrollOffsets;
using ScrollAxis = V8ScrollAxis::Enum;

class AnimationTimeline;
class ComputedStyle;
class Node;

namespace scroll_timeline_util {

// Converts the input timeline to the compositor representation of a
// ScrollTimeline. Returns nullptr if the input is not a ScrollTimeline.
scoped_refptr<CompositorScrollTimeline> CORE_EXPORT
ToCompositorScrollTimeline(AnimationTimeline*);

// Retrieves the 'scroll' compositor element id for the input node, or
// std::nullopt if it does not exist.
std::optional<CompositorElementId> CORE_EXPORT
GetCompositorScrollElementId(const Node*);

// Convert the blink concept of a ScrollTimeline axis into the cc one.
//
// This implements a subset of the conversions documented in
// https://drafts.csswg.org/css-writing-modes-3/#logical-to-physical
CompositorScrollTimeline::ScrollDirection CORE_EXPORT
ConvertOrientation(ScrollAxis, const ComputedStyle*);

}  // namespace scroll_timeline_util

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_TIMELINE_UTIL_H_
