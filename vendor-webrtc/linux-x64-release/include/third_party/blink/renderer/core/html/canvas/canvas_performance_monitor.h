// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_PERFORMANCE_MONITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_PERFORMANCE_MONITOR_H_

#include "base/task/sequence_manager/task_time_observer.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class CanvasRenderingContext;

// Measures tasks that draw to canvas rendering contexts.
class CORE_EXPORT CanvasPerformanceMonitor
    : public base::sequence_manager::TaskTimeObserver {
 public:
  // Flag bits that can be passed to DidDraw.
  enum class DrawType {
    kOther = 0,

    // It is allowed to re-number the elements of this enum

    // 2D context draw types
    kPath = 1 << 0,
    kImage = 1 << 1,
    kText = 1 << 2,
    kRectangle = 1 << 3,
    kImageData = 1 << 4,

    // WebGL draw types
    kDrawArrays = 1 << 5,
    kDrawElements = 1 << 6,
  };

  CanvasPerformanceMonitor() = default;

  void CurrentTaskDrawsToContext(CanvasRenderingContext*);
  ALWAYS_INLINE void DidDraw(DrawType draw_type) {
    draw_types_ |= static_cast<uint32_t>(draw_type);
  }

  // Call this at the end of unit tests that use CanvasRenderingContext, to
  // reset state that may otherwise be leaked between tests.
  void ResetForTesting();

  // TaskTimeObserver:
  void WillProcessTask(base::TimeTicks start_time) final;
  void DidProcessTask(base::TimeTicks start_time,
                      base::TimeTicks end_time) final;

 private:
  void RecordMetrics(base::TimeTicks start_time, base::TimeTicks end_time);

  enum class CallType {
    kAnimation,
    kUserInput,
    kOther,
  };

  HashSet<uint32_t> rendering_context_descriptions_;
  uint32_t draw_types_ = 0;
  CallType call_type_ = CallType::kOther;
  uint32_t task_counter_ = 0;
  bool is_render_task_ = false;
  bool measure_current_task_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_PERFORMANCE_MONITOR_H_
