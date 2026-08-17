// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_TIMELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_TIMELINE_H_

#include <optional>
#include <utility>
#include <variant>

#include "base/check_op.h"
#include "base/memory/values_equivalent.h"
#include "third_party/blink/renderer/core/animation/timeline_inset.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/scoped_css_name.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-timeline
class CORE_EXPORT StyleTimeline {
 public:
  // https://drafts.csswg.org/scroll-animations-1/#scroll-notation
  class ScrollData {
   public:
    // https://drafts.csswg.org/scroll-animations-1/#valdef-scroll-block
    static TimelineAxis DefaultAxis() { return TimelineAxis::kBlock; }
    // https://drafts.csswg.org/scroll-animations-1/#valdef-scroll-nearest
    static TimelineScroller DefaultScroller() {
      return TimelineScroller::kNearest;
    }

    ScrollData(TimelineAxis axis, TimelineScroller scroller)
        : axis_(axis), scroller_(scroller) {}
    bool operator==(const ScrollData& other) const {
      return axis_ == other.axis_ && scroller_ == other.scroller_;
    }
    bool operator!=(const ScrollData& other) const { return !(*this == other); }

    TimelineAxis GetAxis() const { return axis_; }
    const TimelineScroller& GetScroller() const { return scroller_; }

    bool HasDefaultAxis() const { return axis_ == DefaultAxis(); }
    bool HasDefaultScroller() const { return scroller_ == DefaultScroller(); }

   private:
    TimelineAxis axis_;
    TimelineScroller scroller_;
  };

  // https://drafts.csswg.org/scroll-animations-1/#view-notation
  class ViewData {
   public:
    static TimelineAxis DefaultAxis() { return TimelineAxis::kBlock; }

    ViewData(TimelineAxis axis, TimelineInset inset)
        : axis_(axis), inset_(inset) {}

    bool operator==(const ViewData& other) const {
      return axis_ == other.axis_ && inset_ == other.inset_;
    }
    bool operator!=(const ViewData& other) const { return !(*this == other); }

    TimelineAxis GetAxis() const { return axis_; }
    TimelineInset GetInset() const { return inset_; }
    bool HasDefaultAxis() const { return axis_ == DefaultAxis(); }
    bool HasDefaultInset() const {
      return inset_.GetStart().IsAuto() && inset_.GetEnd().IsAuto();
    }

   private:
    TimelineAxis axis_;
    TimelineInset inset_;
  };

  explicit StyleTimeline(CSSValueID keyword) : data_(keyword) {}
  explicit StyleTimeline(const ScopedCSSName* name)
      : data_(std::in_place_type<Persistent<const ScopedCSSName>>, name) {}
  explicit StyleTimeline(const ScrollData& scroll_data) : data_(scroll_data) {}
  explicit StyleTimeline(const ViewData& view_data) : data_(view_data) {}

  bool operator==(const StyleTimeline& other) const {
    if (IsName() && other.IsName()) {
      return base::ValuesEquivalent(&GetName(), &other.GetName());
    }
    return data_ == other.data_;
  }
  bool operator!=(const StyleTimeline& other) const {
    return !(*this == other);
  }

  bool IsKeyword() const { return std::holds_alternative<CSSValueID>(data_); }
  bool IsName() const {
    return std::holds_alternative<Persistent<const ScopedCSSName>>(data_);
  }
  bool IsScroll() const { return std::holds_alternative<ScrollData>(data_); }
  bool IsView() const { return std::holds_alternative<ViewData>(data_); }

  const CSSValueID& GetKeyword() const { return std::get<CSSValueID>(data_); }
  const ScopedCSSName& GetName() const {
    return *std::get<Persistent<const ScopedCSSName>>(data_);
  }
  const ScrollData& GetScroll() const { return std::get<ScrollData>(data_); }
  const ViewData& GetView() const { return std::get<ViewData>(data_); }

 private:
  std::
      variant<CSSValueID, Persistent<const ScopedCSSName>, ScrollData, ViewData>
          data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_TIMELINE_H_
