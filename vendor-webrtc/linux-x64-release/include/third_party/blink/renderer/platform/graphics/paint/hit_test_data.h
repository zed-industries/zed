// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_HIT_TEST_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_HIT_TEST_DATA_H_

#include "third_party/blink/renderer/platform/geometry/infinite_int_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/transform_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/touch_action_rect.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

struct PLATFORM_EXPORT HitTestData : public GarbageCollected<HitTestData> {
 public:
  Vector<TouchActionRect> touch_action_rects;
  Vector<gfx::Rect> wheel_event_rects;

  // If scroll_translation is nullptr, this marks a region in which composited
  // scroll is not allowed. When scroll_translation is not nullptr, this is the
  // bounds of the scroll container, and whether the region allows composited
  // scrolling depends whether the scroll_translation is composited.
  gfx::Rect scroll_hit_test_rect;
  Member<const TransformPaintPropertyNode> scroll_translation;
  gfx::Rect scrolling_contents_cull_rect = InfiniteIntRect();

  void Trace(Visitor* visitor) const { visitor->Trace(scroll_translation); }

  bool operator==(const HitTestData& rhs) const {
    return touch_action_rects == rhs.touch_action_rects &&
           wheel_event_rects == rhs.wheel_event_rects &&
           scroll_hit_test_rect == rhs.scroll_hit_test_rect &&
           scroll_translation == rhs.scroll_translation &&
           scrolling_contents_cull_rect == rhs.scrolling_contents_cull_rect;
  }

  String ToString() const;
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const HitTestData&);
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const HitTestData*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_HIT_TEST_DATA_H_
