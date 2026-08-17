/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 * Copyright (C) 2012 Intel Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_ENTRY_H_

#include "third_party/blink/public/mojom/timing/performance_mark_or_measure.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/frame/dom_window.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ScriptObject;
class ScriptState;
class V8ObjectBuilder;

using PerformanceEntryType = unsigned;
using PerformanceEntryTypeMask = unsigned;

struct DOMPaintTimingInfo {
  // https://w3c.github.io/paint-timing/#paint-timing-info-rendering-update-end-time
  DOMHighResTimeStamp paint_time;

  // https://w3c.github.io/paint-timing/#paint-timing-info-implementation-defined-presentation-time
  DOMHighResTimeStamp presentation_time;
};

class CORE_EXPORT PerformanceEntry : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~PerformanceEntry() override;

  enum EntryType : PerformanceEntryType {
    kInvalid = 0,
    kNavigation = 1 << 0,
    kMark = 1 << 1,
    kMeasure = 1 << 2,
    kResource = 1 << 3,
    kLongTask = 1 << 4,
    kTaskAttribution = 1 << 5,
    kPaint = 1 << 6,
    kEvent = 1 << 7,
    kFirstInput = 1 << 8,
    kElement = 1 << 9,
    kLayoutShift = 1 << 10,
    kLargestContentfulPaint = 1 << 11,
    kVisibilityState = 1 << 12,
    kBackForwardCacheRestoration = 1 << 13,
    kSoftNavigation = 1 << 14,
    kLongAnimationFrame = 1 << 15,
    kScript = 1 << 16,
    kContainer = 1 << 17,
  };

  const AtomicString& name() const { return name_; }
  DOMHighResTimeStamp startTime() const;
  String navigationId() const;
  // source() will return null if the PerformanceEntry did not originate from a
  // Window context.
  DOMWindow* source() const;
  virtual const AtomicString& entryType() const = 0;
  virtual PerformanceEntryType EntryTypeEnum() const = 0;
  // PerformanceNavigationTiming will override this due to
  // the nature of reporting it early, which means not having a
  // finish time available at construction time.
  // Other classes must NOT override this.
  virtual DOMHighResTimeStamp duration() const;

  ScriptObject toJSONForBinding(ScriptState*) const;

  bool IsResource() const { return EntryTypeEnum() == kResource; }
  bool IsMark() const { return EntryTypeEnum() == kMark; }
  bool IsMeasure() const { return EntryTypeEnum() == kMeasure; }

  static bool StartTimeCompareLessThan(PerformanceEntry* a,
                                       PerformanceEntry* b) {
    if (a->startTime() == b->startTime()) {
      // The navigation entry is created lazily, and we want it to always be
      // first for compatibility.
      // TODO: create NT entry eagerly so that we don't have to do this
      // https://bugs.chromium.org/p/chromium/issues/detail?id=1432565
      if (a->EntryTypeEnum() == kNavigation &&
          b->EntryTypeEnum() != kNavigation) {
        return true;
      } else if (b->EntryTypeEnum() == kNavigation) {
        return false;
      } else if (a->EntryTypeEnum() == kPaint && b->EntryTypeEnum() != kPaint) {
        return true;
      } else if (b->EntryTypeEnum() == kPaint) {
        return false;
      } else {
        return a->index_ < b->index_;
      }
    }

    return a->startTime() < b->startTime();
  }

  static PerformanceEntry::EntryType ToEntryTypeEnum(
      const AtomicString& entry_type);

  // Entries of the types listed here will be accessible from the
  // PerformanceTimeline or the PerformanceObserver.  Those not listed will
  // only be available via the PerformanceObserver.
  // Note: Currently buffered flags don't support long task entries
  // so leaving it out of this list for now.
  static bool IsValidTimelineEntryType(const PerformanceEntryType& entry_type) {
    if (entry_type == kInvalid) {
      return true;
    }
    constexpr PerformanceEntryTypeMask kTimelineEntryMask =
        kNavigation | kMark | kMeasure | kResource | kTaskAttribution | kPaint |
        kFirstInput | kBackForwardCacheRestoration | kSoftNavigation |
        kLongAnimationFrame | kVisibilityState;
    return (entry_type & kTimelineEntryMask) != 0;
  }

  static String GetNavigationId(ScriptState* script_state);

  // PerformanceMark/Measure override this and it returns Mojo structure pointer
  // which has all members of PerformanceMark/Measure. Common data members are
  // set by PerformanceMark/Measure calling
  // PerformanceEntry::ToMojoPerformanceMarkOrMeasure().
  virtual mojom::blink::PerformanceMarkOrMeasurePtr
  ToMojoPerformanceMarkOrMeasure();

  bool IsTriggeredBySoftNavigation() const {
    return is_triggered_by_soft_navigation_;
  }

  // PaintTimingMixin. It's implemented here for simplicity.
  // If an interface doesn't have PaintTimingMixin, these functions
  // would not be exposed by WebIDL.
  DOMHighResTimeStamp paintTime() const;
  std::optional<DOMHighResTimeStamp> presentationTime() const;

  void Trace(Visitor*) const override;

  void SetPaintTimingInfo(const DOMPaintTimingInfo& paint_timing_info) {
    paint_timing_info_ = paint_timing_info;
  }

 protected:
  PerformanceEntry(const AtomicString& name,
                   double start_time,
                   double finish_time,
                   DOMWindow* source,
                   bool is_triggered_by_soft_navigation = false);
  PerformanceEntry(double duration,
                   const AtomicString& name,
                   double start_time,
                   DOMWindow* source,
                   bool is_triggered_by_soft_navigation = false);

  virtual void BuildJSONValue(V8ObjectBuilder&) const;

  // Protected and not const because PerformanceEventTiming needs to modify it.
  double duration_;

 private:
  const AtomicString name_;
  const double start_time_;
  const int index_;
  const String navigation_id_;
  // source_ will be null if the PerformanceEntry did not originate from a
  // Window context.
  const WeakMember<DOMWindow> source_;
  // For entries implementing PaintTimingMixin.
  std::optional<DOMPaintTimingInfo> paint_timing_info_;
  const bool is_triggered_by_soft_navigation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_ENTRY_H_
