// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_CONTAINER_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_CONTAINER_TIMING_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/timing/window_performance.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

// ContainerTiming is responsible for aggregating the text and image element
// timing events for a given window.
class CORE_EXPORT ContainerTiming final
    : public GarbageCollected<ContainerTiming>,
      public Supplement<LocalDOMWindow> {
 public:
  static constexpr const char kSupplementName[] = "ContainerTiming";

  explicit ContainerTiming(LocalDOMWindow&);
  ContainerTiming(const ContainerTiming&) = delete;
  ContainerTiming& operator=(const ContainerTiming&) = delete;

  static ContainerTiming& From(LocalDOMWindow&);

  static inline bool ContributesToContainerTiming(const Element* element) {
    return element && !element->IsInShadowTree() &&
           element->SelfOrAncestorHasContainerTiming();
  }

  bool CanReportToContainerTiming() const;

  void EmitPerformanceEntries();

  void OnElementPainted(const DOMPaintTimingInfo& paint_timing_info,
                        Element* element,
                        const gfx::RectF& intersection_rect);

  void Trace(Visitor* visitor) const override;

 private:
  static Element* GetContainerRoot(Element*);
  static Element* GetParentContainerRoot(Element*);
  class Record final : public GarbageCollected<Record> {
   public:
    Record(const DOMPaintTimingInfo& paint_timing_info,
           const AtomicString& identifier);
    Record(const Record&) = delete;
    Record& operator=(const Record&) = delete;

    void MaybeUpdateLastNewPaintedArea(
        ContainerTiming* container_timing,
        const DOMPaintTimingInfo& paint_timing_info,
        Element* container_root,
        Element* element,
        const gfx::RectF& intersection_rect);

    void MaybeEmitPerformanceEntry(WindowPerformance*);

    void Trace(Visitor*) const;

   private:
    const DOMPaintTimingInfo first_paint_timing_info_;
    const AtomicString identifier_;
    DOMPaintTimingInfo last_new_painted_area_paint_timing_info_;
    WeakMember<Element> last_new_painted_area_element_;
    cc::Region painted_region_;
    bool has_pending_changes_ = false;
  };
  Record* GetOrCreateRecord(const DOMPaintTimingInfo& paint_timing_info,
                            Element* container_root);

  Member<WindowPerformance> performance_;
  HeapHashMap<WeakMember<Element>, Member<Record>> container_root_records_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_CONTAINER_TIMING_H_
