// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_TEXT_ELEMENT_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_TEXT_ELEMENT_TIMING_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/paint/timing/container_timing.h"
#include "third_party/blink/renderer/core/timing/window_performance.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace gfx {
class Rect;
class RectF;
}  // namespace gfx

namespace blink {

class LayoutObject;
class LocalFrameView;
class PropertyTreeStateOrAlias;
class TextRecord;

// TextElementTiming is responsible for tracking the paint timings for groups of
// text nodes associated with elements of a given window.
class CORE_EXPORT TextElementTiming final
    : public GarbageCollected<TextElementTiming>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit TextElementTiming(LocalDOMWindow&);
  TextElementTiming(const TextElementTiming&) = delete;
  TextElementTiming& operator=(const TextElementTiming&) = delete;

  static TextElementTiming& From(LocalDOMWindow&);

  static inline bool NeededForTiming(Node& node) {
    auto* element = DynamicTo<Element>(node);
    return !node.IsInShadowTree() && element &&
           (element->FastHasAttribute(html_names::kElementtimingAttr) ||
            element->SelfOrAncestorHasContainerTiming());
  }

  static gfx::RectF ComputeIntersectionRect(
      const LayoutObject&,
      const gfx::Rect& aggregated_visual_rect,
      const PropertyTreeStateOrAlias&,
      const LocalFrameView*);

  bool CanReportToElementTiming() const;
  bool CanReportToContainerTiming();
  bool CanReportElements();

  // Called when the swap promise queued by TextPaintTimingDetector has been
  // resolved. Dispatches PerformanceElementTiming entries to WindowPerformance.
  void OnTextObjectPainted(const TextRecord&, const DOMPaintTimingInfo&);

  void Trace(Visitor* visitor) const override;

 private:
  void EnsureContainerTiming();

  Member<WindowPerformance> performance_;
  Member<ContainerTiming> container_timing_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_TEXT_ELEMENT_TIMING_H_
