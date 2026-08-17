// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_SCRIPT_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_SCRIPT_TIMING_H_

#include <cstdint>

#include "third_party/blink/renderer/bindings/core/v8/v8_script_window_attribution.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/timing/animation_frame_timing_info.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class V8ScriptInvokerType;

class PerformanceScriptTiming final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This constructor uses int for |duration| to coarsen it in advance.
  // LongAnimationFrameTiming is always at 1-ms granularity.
  PerformanceScriptTiming(ScriptTimingInfo* info,
                          base::TimeTicks time_origin,
                          bool cross_origin_isolated_capability,
                          DOMWindow* source);
  ~PerformanceScriptTiming() override;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  DOMHighResTimeStamp executionStart() const { return execution_start_; }
  DOMHighResTimeStamp forcedStyleAndLayoutDuration() const;
  DOMHighResTimeStamp pauseDuration() const;
  LocalDOMWindow* window() const;
  WTF::String sourceURL() const;
  WTF::String sourceFunctionName() const;
  int32_t sourceCharPosition() const;
  int32_t sourceLine() const;
  int32_t sourceColumn() const;
  V8ScriptWindowAttribution windowAttribution() const;
  V8ScriptInvokerType invokerType() const;
  AtomicString invoker() const;
  void Trace(Visitor*) const override;

 private:
  void BuildJSONValue(V8ObjectBuilder&) const override;
  DOMHighResTimeStamp ToMonotonicTime(base::TimeTicks) const;
  Member<ScriptTimingInfo> info_;
  V8ScriptWindowAttribution::Enum window_attribution_;
  DOMHighResTimeStamp execution_start_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_SCRIPT_TIMING_H_
