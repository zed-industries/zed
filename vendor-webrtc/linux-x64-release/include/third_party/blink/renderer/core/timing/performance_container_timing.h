// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_CONTAINER_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_CONTAINER_TIMING_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/geometry/dom_rect_read_only.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"

namespace blink {

class CORE_EXPORT PerformanceContainerTiming final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PerformanceContainerTiming* Create(
      const AtomicString& name,
      DOMHighResTimeStamp start_time,
      const gfx::Rect& intersection_rect,
      double size,
      const AtomicString& identifier,
      Element* last_painted_element,
      DOMHighResTimeStamp first_render_time,
      DOMWindow* source);
  PerformanceContainerTiming(const AtomicString& name,
                             DOMHighResTimeStamp start_time,
                             DOMHighResTimeStamp end_time,
                             const gfx::Rect& intersection_rect,
                             double size,
                             const AtomicString& identifier,
                             Element* last_painted_element,
                             DOMHighResTimeStamp first_render_time,
                             DOMWindow* source);

  ~PerformanceContainerTiming() override;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;

  DOMRectReadOnly* intersectionRect() const { return intersection_rect_.Get(); }
  double size() const { return size_; }
  AtomicString identifier() const { return identifier_; }
  Element* lastPaintedElement() const;
  DOMHighResTimeStamp firstRenderTime() const { return first_render_time_; }

  std::unique_ptr<TracedValue> ToTracedValue() const;

  void Trace(Visitor*) const override;

 private:
  void BuildJSONValue(V8ObjectBuilder&) const override;

  Member<DOMRectReadOnly> intersection_rect_;
  double size_;
  AtomicString identifier_;
  WeakMember<Element> last_painted_element_;
  DOMHighResTimeStamp first_render_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_CONTAINER_TIMING_H_
