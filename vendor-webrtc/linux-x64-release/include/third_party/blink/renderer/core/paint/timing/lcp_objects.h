// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_LCP_OBJECTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_LCP_OBJECTS_H_

#include "base/time/time.h"
#include "third_party/blink/public/common/performance/largest_contentful_paint_type.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

struct ResourceLoadTimings {
  base::TimeTicks load_start;
  base::TimeTicks load_end;
  base::TimeTicks discovery_time;
};

struct LargestContentfulPaintDetails {
  base::TimeTicks largest_image_paint_time;
  uint64_t largest_image_paint_size = 0;
  ResourceLoadTimings resource_load_timings = {};
  blink::LargestContentfulPaintType largest_contentful_paint_type =
      blink::LargestContentfulPaintType::kNone;
  double largest_contentful_paint_image_bpp = 0.0;
  base::TimeTicks largest_text_paint_time;
  uint64_t largest_text_paint_size = 0;
  base::TimeTicks largest_contentful_paint_time;
  std::optional<WebURLRequest::Priority>
      largest_contentful_paint_image_request_priority = std::nullopt;
};

// This class is used for tracing only.
class LCPRectInfo {
  USING_FAST_MALLOC(LCPRectInfo);

 public:
  LCPRectInfo(const gfx::Rect& frame_rect_info, const gfx::Rect& root_rect_info)
      : frame_rect_info_(frame_rect_info), root_rect_info_(root_rect_info) {}

  void OutputToTraceValue(TracedValue&) const;

 private:
  gfx::Rect frame_rect_info_;
  gfx::Rect root_rect_info_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_LCP_OBJECTS_H_
