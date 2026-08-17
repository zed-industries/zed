// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_BACK_FORWARD_CACHE_RESTORATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_BACK_FORWARD_CACHE_RESTORATION_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {
class CORE_EXPORT BackForwardCacheRestoration : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BackForwardCacheRestoration(DOMHighResTimeStamp start_time,
                              DOMHighResTimeStamp pageshow_event_start,
                              DOMHighResTimeStamp pageshow_event_end,
                              DOMWindow* source);
  ~BackForwardCacheRestoration() override;
  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  DOMHighResTimeStamp pageshowEventStart() const {
    return pageshow_event_start_;
  }
  DOMHighResTimeStamp pageshowEventEnd() const { return pageshow_event_end_; }

  void Trace(Visitor*) const override;

 private:
  void BuildJSONValue(V8ObjectBuilder&) const override;

  // Time when persisted pageshow events are dispatched.
  DOMHighResTimeStamp pageshow_event_start_;
  // Time when persisted pageshow events end.
  DOMHighResTimeStamp pageshow_event_end_;
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_BACK_FORWARD_CACHE_RESTORATION_H_
