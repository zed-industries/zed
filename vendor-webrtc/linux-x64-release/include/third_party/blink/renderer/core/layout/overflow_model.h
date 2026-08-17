/*
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc. All rights
 * reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OVERFLOW_MODEL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OVERFLOW_MODEL_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// BoxOverflowModel class tracks content that spills out of an object.
// It is used by LayoutBox.
//
// All overflows are in the physical coordinate space of the object. See
// documentation of LayoutBoxModelObject and LayoutBox::NoOverflowRect() for
// more details.
//
// The class models the overflows as rectangles that unite all the sources of
// overflow. This is the natural choice for scrollable overflow (scrollbars are
// linear in nature, thus are modeled by rectangles in 2D). For visual overflow
// and content visual overflow, this is a first order simplification though as
// they can be thought of as a collection of (potentially overlapping)
// rectangles.
//
// Scrollable overflow is the overflow that is reachable via scrollbars. It is
// used to size the scrollbar thumb and determine its position, which is
// determined by the maximum scrollable overflow size.
// Scrollable overflow cannot occur without an overflow clip as this is the only
// way to get scrollbars. As its name implies, it is a direct consequence of
// layout.
// Example of scrollable overflow:
// * in the inline case, a tall image could spill out of a line box.
// * 'height' / 'width' set to a value smaller than the one needed by the
//   descendants.
// Due to how scrollbars work, no overflow in the logical top and logical left
// direction is allowed(see LayoutBox::AddScrollableOverflow).
//
// Visual overflow covers all the effects that visually bleed out of the box.
// Its primary use is to determine the area to invalidate.
// Visual overflow includes ('text-shadow' / 'box-shadow'), text stroke,
// 'outline', 'border-image', etc.
//
// BoxModelOverflow separates visual overflow into self visual overflow and
// contents visual overflow.
//
// Self visual overflow covers all the effects of the object itself that
// visually bleed out of the box.
//
// Content visual overflow includes anything that would bleed out of the box and
// would be clipped by the overflow clip ('overflow' != visible). This
// corresponds to children that overflows their parent.
// It's important to note that this overflow ignores descendants with
// self-painting layers (see the SELF-PAINTING LAYER section in PaintLayer).
// This is required by the simplification made by this model (single united
// rectangle) to avoid gigantic invalidation. A good example for this is
// positioned objects that can be anywhere on the page and could artificially
// inflate the visual overflow.
// The main use of content visual overflow is to prevent unneeded clipping in
// BoxPainter (see https://crbug.com/238732). Note that the code path for
// self-painting layer is handled by PaintLayerPainter, which relies on
// PaintLayerClipper and thus ignores this optimization.
//
// Visual overflow covers self visual overflow, and if the box doesn't clip
// overflow, also content visual overflow. OverflowModel doesn't keep visual
// overflow, but keeps self visual overflow and contents visual overflow
// separately. The box should use self visual overflow as visual overflow if
// it clips overflow, otherwise union of self visual overflow and contents
// visual overflow.
//
// An overflow model object is allocated only when some of these fields have
// non-default values in the owning object. Care should be taken to use adder
// functions (AddScrollableOverflow, AddVisualOverflow, etc.) to keep this
// invariant.
class BoxScrollableOverflowModel {
 public:
  explicit BoxScrollableOverflowModel(const PhysicalRect& overflow_rect)
      : scrollable_overflow_(overflow_rect) {}
  BoxScrollableOverflowModel(const BoxScrollableOverflowModel&) = delete;
  BoxScrollableOverflowModel& operator=(const BoxScrollableOverflowModel&) =
      delete;

  const PhysicalRect& ScrollableOverflowRect() const {
    return scrollable_overflow_;
  }

 private:
  PhysicalRect scrollable_overflow_;
};

class BoxVisualOverflowModel {
 public:
  explicit BoxVisualOverflowModel(const PhysicalRect& self_visual_overflow_rect)
      : self_visual_overflow_(self_visual_overflow_rect) {}
  BoxVisualOverflowModel(const BoxVisualOverflowModel&) = delete;
  BoxVisualOverflowModel& operator=(const BoxVisualOverflowModel&) = delete;

  void SetSelfVisualOverflow(const PhysicalRect& rect) {
    self_visual_overflow_ = rect;
  }

  const PhysicalRect& SelfVisualOverflowRect() const {
    return self_visual_overflow_;
  }
  void AddSelfVisualOverflow(const PhysicalRect& rect) {
    self_visual_overflow_.Unite(rect);
  }

  const PhysicalRect& ContentsVisualOverflowRect() const {
    return contents_visual_overflow_;
  }
  void AddContentsVisualOverflow(const PhysicalRect& rect) {
    contents_visual_overflow_.Unite(rect);
  }

  void Move(LayoutUnit dx, LayoutUnit dy) {
    PhysicalOffset offset(dx, dy);
    self_visual_overflow_.Move(offset);
    contents_visual_overflow_.Move(offset);
  }

  void SetHasSubpixelVisualEffectOutsets(bool b) {
    has_subpixel_visual_effect_outsets_ = b;
  }
  bool HasSubpixelVisualEffectOutsets() const {
    return has_subpixel_visual_effect_outsets_;
  }

 private:
  PhysicalRect self_visual_overflow_;
  PhysicalRect contents_visual_overflow_;
  bool has_subpixel_visual_effect_outsets_ = false;
};

struct BoxOverflowModel : public GarbageCollected<BoxOverflowModel> {
  std::optional<BoxScrollableOverflowModel> scrollable_overflow;
  std::optional<BoxVisualOverflowModel> visual_overflow;

  // Used by BoxPaintInvalidator. Stores the previous overflow data after the
  // last paint invalidation.
  struct PreviousOverflowData {
    PhysicalRect previous_scrollable_overflow_rect;
    PhysicalRect previous_visual_overflow_rect;
    PhysicalRect previous_self_visual_overflow_rect;
  };
  std::optional<PreviousOverflowData> previous_overflow_data;

  void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OVERFLOW_MODEL_H_
