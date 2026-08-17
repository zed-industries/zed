/*
 * Copyright (C) 2006 Apple Computer, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_TYPES_H_

#include "base/notreached.h"
#include "third_party/blink/public/mojom/input/scroll_direction.mojom-blink.h"
#include "third_party/blink/public/mojom/scroll/scroll_enums.mojom-blink.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "ui/gfx/geometry/vector2d_conversions.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

// A ScrollOffset represents an offset from the scroll origin of a
// ScrollableArea.  Note that "scroll origin" is not the same as the layout
// concept of "location", nor is it necessarily coincident with the top/left of
// the ScrollableArea's overflow rect.  See core/layout/README.md for more
// information.
using ScrollOffset = gfx::Vector2dF;

enum ScrollDirectionPhysical {
  kScrollUp,
  kScrollDown,
  kScrollLeft,
  kScrollRight
};

// An explicit scroll is one that was requested by the user or the webpage.
// An implicit scroll is a side effect of a layout change.
inline bool IsExplicitScrollType(mojom::blink::ScrollType scroll_type) {
  return scroll_type == mojom::blink::ScrollType::kUser ||
         scroll_type == mojom::blink::ScrollType::kProgrammatic ||
         scroll_type == mojom::blink::ScrollType::kCompositor;
}

// Convert logical scroll direction to physical. Physical scroll directions are
// unaffected.
inline ScrollDirectionPhysical ToPhysicalDirection(
    mojom::blink::ScrollDirection direction,
    bool is_vertical,
    bool is_flipped) {
  switch (direction) {
    case mojom::blink::ScrollDirection::kScrollBlockDirectionBackward: {
      if (is_vertical) {
        if (!is_flipped)
          return kScrollUp;
        return kScrollDown;
      }
      if (!is_flipped)
        return kScrollLeft;
      return kScrollRight;
    }
    case mojom::blink::ScrollDirection::kScrollBlockDirectionForward: {
      if (is_vertical) {
        if (!is_flipped)
          return kScrollDown;
        return kScrollUp;
      }
      if (!is_flipped)
        return kScrollRight;
      return kScrollLeft;
    }
    case mojom::blink::ScrollDirection::kScrollInlineDirectionBackward: {
      if (is_vertical) {
        if (!is_flipped)
          return kScrollLeft;
        return kScrollRight;
      }
      if (!is_flipped)
        return kScrollUp;
      return kScrollDown;
    }
    case mojom::blink::ScrollDirection::kScrollInlineDirectionForward: {
      if (is_vertical) {
        if (!is_flipped)
          return kScrollRight;
        return kScrollLeft;
      }
      if (!is_flipped)
        return kScrollDown;
      return kScrollUp;
    }
    // Direction is already physical
    case mojom::blink::ScrollDirection::kScrollUpIgnoringWritingMode:
      return kScrollUp;
    case mojom::blink::ScrollDirection::kScrollDownIgnoringWritingMode:
      return kScrollDown;
    case mojom::blink::ScrollDirection::kScrollLeftIgnoringWritingMode:
      return kScrollLeft;
    case mojom::blink::ScrollDirection::kScrollRightIgnoringWritingMode:
      return kScrollRight;
    default:
      NOTREACHED();
  }
  return kScrollUp;
}

inline mojom::blink::ScrollDirection ToScrollDirection(
    ScrollDirectionPhysical direction) {
  switch (direction) {
    case kScrollUp:
      return mojom::blink::ScrollDirection::kScrollUpIgnoringWritingMode;
    case kScrollDown:
      return mojom::blink::ScrollDirection::kScrollDownIgnoringWritingMode;
    case kScrollLeft:
      return mojom::blink::ScrollDirection::kScrollLeftIgnoringWritingMode;
    case kScrollRight:
      return mojom::blink::ScrollDirection::kScrollRightIgnoringWritingMode;
    default:
      NOTREACHED();
  }
  return mojom::blink::ScrollDirection::kScrollUpIgnoringWritingMode;
}

enum ScrollInertialPhase {
  kScrollInertialPhaseUnknown,
  kScrollInertialPhaseNonMomentum,
  kScrollInertialPhaseMomentum
};

enum ScrollbarOrientation { kHorizontalScrollbar, kVerticalScrollbar };

enum ScrollOrientation { kHorizontalScroll, kVerticalScroll };

typedef unsigned ScrollbarControlState;

enum ScrollbarControlStateMask {
  kActiveScrollbarState = 1,
  kEnabledScrollbarState = 1 << 1,
  kPressedScrollbarState = 1 << 2
};

enum ScrollbarPart {
  kNoPart = 0,
  kBackButtonStartPart = 1,
  kForwardButtonStartPart = 1 << 1,  // For custom scrollbars only.
  kBackTrackPart = 1 << 2,
  kThumbPart = 1 << 3,
  kForwardTrackPart = 1 << 4,
  kBackButtonEndPart = 1 << 5,  // For custom scrollbars only.
  kForwardButtonEndPart = 1 << 6,
  kScrollbarBGPart = 1 << 7,  // For custom scrollbars only.
  kTrackBGPart = 1 << 8,
  kAllParts = 0xffffffff
};

// The result of an attempt to scroll. If didScroll is true, then
// unusedScrollDelta gives the amount of the scroll delta that was not consumed
// by scrolling.
struct ScrollResult {
  STACK_ALLOCATED();

 public:
  explicit ScrollResult()
      : did_scroll_x(false),
        did_scroll_y(false),
        unused_scroll_delta_x(0),
        unused_scroll_delta_y(0) {}
  ScrollResult(bool did_scroll_x,
               bool did_scroll_y,
               float unused_scroll_delta_x,
               float unused_scroll_delta_y)
      : did_scroll_x(did_scroll_x),
        did_scroll_y(did_scroll_y),
        unused_scroll_delta_x(unused_scroll_delta_x),
        unused_scroll_delta_y(unused_scroll_delta_y) {}

  bool DidScroll() { return did_scroll_x || did_scroll_y; }

  bool did_scroll_x;
  bool did_scroll_y;

  // In pixels.
  float unused_scroll_delta_x;
  float unused_scroll_delta_y;
};

inline ScrollOffset ToScrollDelta(ScrollbarOrientation orientation,
                                  float delta) {
  return orientation == kHorizontalScrollbar ? ScrollOffset(delta, 0.0f)
                                             : ScrollOffset(0.0f, delta);
}

inline ScrollOffset ToScrollDelta(ScrollDirectionPhysical dir, float delta) {
  if (dir == kScrollUp || dir == kScrollLeft)
    delta = -delta;

  return (dir == kScrollLeft || dir == kScrollRight) ? ScrollOffset(delta, 0)
                                                     : ScrollOffset(0, delta);
}

// ScrollableArea supports storing scroll offsets with subpixel precision;
// however, the web has historically only allowed scroll offsets matching
// physical pixels.
inline gfx::Vector2d SnapScrollOffsetToPhysicalPixels(
    const gfx::Vector2dF& offset) {
  if (RuntimeEnabledFeatures::RoundScrollOffsetsEnabled()) {
    return gfx::ToRoundedVector2d(offset);
  }

  return gfx::ToFlooredVector2d(offset);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_TYPES_H_
