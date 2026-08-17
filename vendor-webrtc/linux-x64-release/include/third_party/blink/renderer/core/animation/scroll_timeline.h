// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_TIMELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_TIMELINE_H_

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "cc/animation/scroll_timeline.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_scroll_axis.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/animation/scroll_snapshot_timeline.h"
#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"

namespace blink {

class Element;
class PaintLayerScrollableArea;
class ScrollTimelineOptions;

// Implements the ScrollTimeline concept from the Scroll-linked Animations spec.
//
// A ScrollTimeline is a special form of AnimationTimeline whose time values are
// not determined by wall-clock time but instead the progress of scrolling in a
// scroll container. The user is able to specify which scroll container to
// track, the direction of scroll they care about, and various attributes to
// control the conversion of scroll amount to time output.
//
// Spec: https://wicg.github.io/scroll-animations/#scroll-timelines
class CORE_EXPORT ScrollTimeline : public ScrollSnapshotTimeline {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Indicates the relation between the reference element and source of the
  // scroll timeline.
  enum class ReferenceType {
    kSource,          // The reference element matches the source.
    kNearestAncestor  // The source is the nearest scrollable ancestor to the
                      // reference element.
  };

  static constexpr double kScrollTimelineMicrosecondsPerPixel =
      cc::ScrollTimeline::kScrollTimelineMicrosecondsPerPixel;

  static ScrollTimeline* Create(Document&,
                                ScrollTimelineOptions*,
                                ExceptionState&);

  static ScrollTimeline* Create(Document* document,
                                Element* source,
                                ScrollAxis axis);

  // Construct ScrollTimeline objects through one of the Create methods, which
  // perform initial snapshots, as it can't be done during the constructor due
  // to possibly depending on overloaded functions.
  ScrollTimeline(Document*,
                 ReferenceType reference_type,
                 Element* reference,
                 ScrollAxis axis);

  bool IsScrollTimeline() const override { return true; }

  // IDL API implementation.
  Element* source() const;
  const V8ScrollAxis axis() const { return V8ScrollAxis(GetAxis()); }

  bool Matches(ReferenceType, Element* reference_element, ScrollAxis) const;

  ScrollAxis GetAxis() const override;

  std::optional<double> GetMaximumScrollPosition() const;

  void AnimationAttached(Animation*) override;
  void AnimationDetached(Animation*) override;

  std::optional<double> GetCurrentScrollPosition() const;

  Node* ComputeResolvedSource() const;

  void Trace(Visitor*) const override;

  TimelineState ComputeTimelineState() const override;

  static ScrollOrientation ToPhysicalScrollOrientation(
      ScrollAxis axis,
      const LayoutBox& source_box);

 protected:

  // Scroll offsets corresponding to 0% and 100% progress. By default, these
  // correspond to the scroll range of the container.
  virtual void CalculateOffsets(PaintLayerScrollableArea* scrollable_area,
                                ScrollOrientation physical_orientation,
                                TimelineState* state) const;

  // Determines the source for the scroll timeline. It may be the reference
  // element or its nearest scrollable ancestor, depending on |reference_type_|.
  Element* ComputeSource() const;
  // This version does not force a style update and is therefore safe to call
  // during lifecycle update.
  Element* ComputeSourceNoLayout() const;

  Element* GetReferenceElement() const { return reference_element_.Get(); }

 private:
  FRIEND_TEST_ALL_PREFIXES(ScrollTimelineTest, MultipleScrollOffsetsClamping);
  FRIEND_TEST_ALL_PREFIXES(ScrollTimelineTest, ResolveScrollOffsets);

  // The retaining element is the element responsible for keeping
  // the timeline alive while animations are attached.
  //
  // See Node::[Un]RegisterScrollTimeline.
  Element* RetainingElement() const;

  ReferenceType reference_type_;
  Member<Element> reference_element_;
  ScrollAxis axis_;
};

template <>
struct DowncastTraits<ScrollTimeline> {
  static bool AllowFrom(const AnimationTimeline& value) {
    return value.IsScrollTimeline();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_TIMELINE_H_
