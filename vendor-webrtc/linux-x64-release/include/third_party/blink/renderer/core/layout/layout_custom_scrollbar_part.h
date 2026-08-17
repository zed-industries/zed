/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_CUSTOM_SCROLLBAR_PART_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_CUSTOM_SCROLLBAR_PART_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_replaced.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"

namespace blink {

class CustomScrollbar;
class ScrollableArea;

class CORE_EXPORT LayoutCustomScrollbarPart final : public LayoutReplaced {
 public:
  static LayoutCustomScrollbarPart* CreateAnonymous(
      Document*,
      ScrollableArea*,
      CustomScrollbar* = nullptr,
      ScrollbarPart = kNoPart,
      bool suppress_use_counters = false);

  void Trace(Visitor*) const override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutCustomScrollbarPart";
  }

  PaintLayerType LayerTypeRequired() const override {
    NOT_DESTROYED();
    return kNoPaintLayer;
  }

  // Computes thickness of the scrollbar (which defines thickness of all parts).
  // For kScrollbarBGPart only. This can be called during style update.
  // Percentage size will be ignored.
  int ComputeThickness() const;

  // Computes size of the part in the direction of the scrollbar orientation.
  // This doesn't apply to kScrollbarBGPart because its length is not determined
  // by the style of the part of itself. For kThumbPart this returns the
  // minimum length of the thumb. The length may depend on the size of the
  // containing box, so this function can only be called after the size is
  // available.
  int ComputeLength() const;

  // Update the overridden size.
  void SetOverriddenSize(const PhysicalSize& size);
  // This should not be called.
  DeprecatedLayoutPoint LocationInternal() const override;
  // Rerturn the overridden size set by SetOverriddenSize();
  PhysicalSize Size() const override;

  LayoutUnit MarginTop() const override;
  LayoutUnit MarginBottom() const override;
  LayoutUnit MarginLeft() const override;
  LayoutUnit MarginRight() const override;

  bool IsLayoutCustomScrollbarPart() const final {
    NOT_DESTROYED();
    return true;
  }
  ScrollableArea* GetScrollableArea() const {
    NOT_DESTROYED();
    return scrollable_area_.Get();
  }

  LayoutCustomScrollbarPart(ScrollableArea*,
                            CustomScrollbar*,
                            ScrollbarPart,
                            bool suppress_use_counters);

 private:
  void UpdateFromStyle() override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override;

  // A scrollbar part's PhysicalLocation() is relative to the scrollbar
  // (instead of relative to any LayoutBox ancestor), so it doesn't have a
  // meaningful location container as a LayoutBox.
  LayoutBox* LocationContainer() const override {
    NOT_DESTROYED();
    return nullptr;
  }

  // Have all padding getters return 0. The important point here is to avoid
  // resolving percents against the containing block, since scroll bar corners
  // don't always have one (so it would crash). Scroll bar corners are not
  // actually laid out, and they don't have child content, so what we return
  // here doesn't really matter.
  LayoutUnit PaddingTop() const override {
    NOT_DESTROYED();
    return LayoutUnit();
  }
  LayoutUnit PaddingBottom() const override {
    NOT_DESTROYED();
    return LayoutUnit();
  }
  LayoutUnit PaddingLeft() const override {
    NOT_DESTROYED();
    return LayoutUnit();
  }
  LayoutUnit PaddingRight() const override {
    NOT_DESTROYED();
    return LayoutUnit();
  }

  void SetNeedsPaintInvalidation();

  void RecordPercentLengthStats() const;

  PhysicalNaturalSizingInfo GetNaturalDimensions() const override;

  int ComputeSize(const Length& length, int container_size) const;
  int ComputeWidth(int container_width) const;
  int ComputeHeight(int container_height) const;

  Member<ScrollableArea> scrollable_area_;
  Member<CustomScrollbar> scrollbar_;
  PhysicalSize overridden_size_;
  ScrollbarPart part_;
  bool suppress_use_counters_ = false;
};

template <>
struct DowncastTraits<LayoutCustomScrollbarPart> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutCustomScrollbarPart();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_CUSTOM_SCROLLBAR_PART_H_
