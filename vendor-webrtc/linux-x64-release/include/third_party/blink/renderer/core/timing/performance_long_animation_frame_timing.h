// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_LONG_ANIMATION_FRAME_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_LONG_ANIMATION_FRAME_TIMING_H_

#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/frame/dom_window.h"
#include "third_party/blink/renderer/core/timing/animation_frame_timing_info.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/core/timing/performance_script_timing.h"

namespace blink {

using PerformanceScriptVector = HeapVector<Member<PerformanceScriptTiming>>;

class PerformanceLongAnimationFrameTiming final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This constructor uses int for |duration| to coarsen it in advance.
  // LongAnimationFrameTiming is always at 1-ms granularity.

  static PerformanceLongAnimationFrameTiming* Create(
      AnimationFrameTimingInfo* info,
      base::TimeTicks time_origin,
      bool cross_origin_isolated_capability,
      DOMWindow*,
      const std::optional<DOMPaintTimingInfo>&);
  ~PerformanceLongAnimationFrameTiming() override;

  PerformanceLongAnimationFrameTiming(double duration,
                                      DOMHighResTimeStamp startTime,
                                      AnimationFrameTimingInfo* info,
                                      base::TimeTicks time_origin,
                                      bool cross_origin_isolated_capability,
                                      DOMWindow*);

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  DOMHighResTimeStamp renderStart() const { return render_start_; }
  DOMHighResTimeStamp styleAndLayoutStart() const {
    return style_and_layout_start_;
  }
  DOMHighResTimeStamp firstUIEventTimestamp() const {
    return first_ui_event_timestamp_;
  }
  DOMHighResTimeStamp blockingDuration() const { return blocking_duration_; }

  const PerformanceScriptVector& scripts() const { return scripts_; }

  void Trace(Visitor*) const override;
 private:
  void BuildJSONValue(V8ObjectBuilder&) const override;
  DOMHighResTimeStamp render_start_;
  DOMHighResTimeStamp style_and_layout_start_;
  DOMHighResTimeStamp first_ui_event_timestamp_;
  double blocking_duration_;
  PerformanceScriptVector scripts_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_LONG_ANIMATION_FRAME_TIMING_H_
