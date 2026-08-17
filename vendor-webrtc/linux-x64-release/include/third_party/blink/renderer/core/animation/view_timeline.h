// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_VIEW_TIMELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_VIEW_TIMELINE_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/animation/scroll_timeline.h"
#include "third_party/blink/renderer/core/animation/timeline_inset.h"
#include "third_party/blink/renderer/core/animation/timeline_range.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ViewTimelineOptions;

// Implements the ViewTimeline from the Scroll-linked Animations spec.
//
// A ViewTimeline is a special form of ScrollTimeline in which the start and
// end scroll offsets are derived based on relative offset of the subject view
// within the source scroll container.
//
// TODO(crbug.com/1329159): Update link once rewrite replaces the draft version.
// https://drafts.csswg.org/scroll-animations-1/rewrite#viewtimeline-interface
class CORE_EXPORT ViewTimeline : public ScrollTimeline {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ViewTimeline* Create(Document&, ViewTimelineOptions*, ExceptionState&);

  ViewTimeline(Document*, Element* subject, ScrollAxis axis, TimelineInset);

  bool IsViewTimeline() const override { return true; }

  CSSNumericValue* getCurrentTime(const String& rangeName) override;

  // IDL API implementation.
  Element* subject() const;

  bool Matches(Element* subject, ScrollAxis, const TimelineInset&) const;

  const TimelineInset& GetInset() const;

  CSSNumericValue* startOffset() const;
  CSSNumericValue* endOffset() const;

  void Trace(Visitor*) const override;

  double ToFractionalOffset(const TimelineOffset& timeline_offset) const;

 protected:
  void CalculateOffsets(PaintLayerScrollableArea* scrollable_area,
                        ScrollOrientation physical_orientation,
                        TimelineState* state) const override;

 private:

  std::optional<gfx::SizeF> SubjectSize() const;
  std::optional<gfx::PointF> SubjectPosition(LayoutBox* scroll_container) const;

  void ApplyStickyAdjustments(ScrollOffsets& scroll_offsets,
                              ViewOffsets& view_offsets,
                              double viewport_size,
                              double target_size,
                              double target_offset,
                              ScrollOrientation orientation,
                              LayoutBox* scroll_container) const;

  TimelineInset inset_;
  // If either of the following elements are non-null, we need to update
  // |inset_| on a style change.
  Member<const CSSValue> style_dependant_start_inset_;
  Member<const CSSValue> style_dependant_end_inset_;
};

template <>
struct DowncastTraits<ViewTimeline> {
  static bool AllowFrom(const AnimationTimeline& value) {
    return value.IsViewTimeline();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_VIEW_TIMELINE_H_
