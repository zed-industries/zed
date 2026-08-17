// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_ELEMENT_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_ELEMENT_TIMING_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/geometry/dom_rect_read_only.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"

namespace blink {

// The PerformanceElementTiming object exposes the time in which an element is
// first rendered on the screen and its intersection with the viewport at the
// time it is painted. Currently this is only done for <img> elements but other
// element types should be supported in the future.
class CORE_EXPORT PerformanceElementTiming final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PerformanceElementTiming* Create(const AtomicString& name,
                                          const String& url,
                                          const gfx::RectF& intersection_rect,
                                          DOMHighResTimeStamp render_time,
                                          DOMHighResTimeStamp load_time,
                                          const AtomicString& identifier,
                                          int naturalWidth,
                                          int naturalHeight,
                                          const AtomicString& id,
                                          Element*,
                                          DOMWindow* source);
  PerformanceElementTiming(const AtomicString& name,
                           DOMHighResTimeStamp start_time,
                           const String& url,
                           const gfx::RectF& intersection_rect,
                           DOMHighResTimeStamp render_time,
                           DOMHighResTimeStamp load_time,
                           const AtomicString& identifier,
                           int naturalWidth,
                           int naturalHeight,
                           const AtomicString& id,
                           Element*,
                           DOMWindow* source);

  ~PerformanceElementTiming() override;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  DOMRectReadOnly* intersectionRect() const { return intersection_rect_.Get(); }
  DOMHighResTimeStamp renderTime() const { return render_time_; }
  DOMHighResTimeStamp loadTime() const { return load_time_; }
  AtomicString identifier() const { return identifier_; }
  unsigned naturalWidth() const { return naturalWidth_; }
  unsigned naturalHeight() const { return naturalHeight_; }
  AtomicString id() const { return id_; }
  String url() const { return url_; }
  Element* element() const;

  std::unique_ptr<TracedValue> ToTracedValue() const;

  void Trace(Visitor*) const override;

 private:
  void BuildJSONValue(V8ObjectBuilder&) const override;

  WeakMember<Element> element_;
  Member<DOMRectReadOnly> intersection_rect_;
  DOMHighResTimeStamp render_time_;
  DOMHighResTimeStamp load_time_;
  AtomicString identifier_;
  unsigned naturalWidth_;
  unsigned naturalHeight_;
  AtomicString id_;
  String url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_ELEMENT_TIMING_H_
