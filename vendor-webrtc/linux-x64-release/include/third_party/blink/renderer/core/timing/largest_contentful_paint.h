// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_LARGEST_CONTENTFUL_PAINT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_LARGEST_CONTENTFUL_PAINT_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// Exposes the Largest Contenful Paint, computed as described in
// https://github.com/WICG/LargestContentfulPaint.
class CORE_EXPORT LargestContentfulPaint final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  LargestContentfulPaint(double start_time,
                         DOMHighResTimeStamp render_time,
                         uint64_t size,
                         DOMHighResTimeStamp load_time,
                         const AtomicString& id,
                         const String& url,
                         Element* element,
                         DOMWindow* source,
                         bool is_triggered_by_soft_navigation);
  ~LargestContentfulPaint() override;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  uint64_t size() const { return size_; }
  DOMHighResTimeStamp renderTime() const { return render_time_; }
  DOMHighResTimeStamp loadTime() const { return load_time_; }
  const AtomicString& id() const { return id_; }
  const String& url() const { return url_; }
  Element* element() const;

  void Trace(Visitor*) const override;

 private:
  void BuildJSONValue(V8ObjectBuilder&) const override;

  uint64_t size_;
  DOMHighResTimeStamp render_time_;
  DOMHighResTimeStamp load_time_;
  AtomicString id_;
  String url_;
  WeakMember<Element> element_;
};

template <>
struct DowncastTraits<LargestContentfulPaint> {
  static bool AllowFrom(const PerformanceEntry& entry) {
    return entry.EntryTypeEnum() ==
           PerformanceEntry::EntryType::kLargestContentfulPaint;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_LARGEST_CONTENTFUL_PAINT_H_
