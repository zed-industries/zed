// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_LONG_TASK_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_LONG_TASK_TIMING_H_

#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class TaskAttributionTiming;
using TaskAttributionVector = HeapVector<Member<TaskAttributionTiming>>;

class PerformanceLongTaskTiming final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This constructor uses int for |duration| on purpose: to avoid exposing a
  // high resolution value in its entry. Having it be int ensures 1 ms
  // granularity, even though it is ultimately stored as a double in
  // PerformanceEntry.
  PerformanceLongTaskTiming(double start_time,
                            int duration,
                            const AtomicString& name,
                            const AtomicString& culprit_type,
                            const AtomicString& culprit_src,
                            const AtomicString& culprit_id,
                            const AtomicString& culprit_name,
                            DOMWindow* source);
  ~PerformanceLongTaskTiming() override;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  TaskAttributionVector attribution() const;

  void Trace(Visitor*) const override;

 private:
  void BuildJSONValue(V8ObjectBuilder&) const override;

  TaskAttributionVector attribution_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_LONG_TASK_TIMING_H_
