// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_BORDER_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_BORDER_PAINTER_H_

#include "third_party/blink/renderer/core/layout/background_bleed_avoidance.h"
#include "third_party/blink/renderer/core/layout/geometry/box_sides.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/style/border_edge.h"
#include "third_party/blink/renderer/platform/geometry/contoured_rect.h"
#include "third_party/blink/renderer/platform/geometry/float_rounded_rect.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context.h"

namespace blink {

class ComputedStyle;
class Path;

typedef unsigned BorderEdgeFlags;

class BoxBorderPainter {
  STACK_ALLOCATED();

 public:
  static void PaintBorder(GraphicsContext& context,
                          const PhysicalRect& border_rect,
                          const ComputedStyle& style,
                          BackgroundBleedAvoidance bleed_avoidance,
                          PhysicalBoxSides sides_to_include) {
    BoxBorderPainter(context, border_rect, style, bleed_avoidance,
                     sides_to_include)
        .Paint();
  }

  static void PaintSingleRectOutline(GraphicsContext& context,
                                     const ComputedStyle& style,
                                     const PhysicalRect& border_rect,
                                     int width,
                                     const PhysicalBoxStrut& inner_outsets) {
    BoxBorderPainter(context, style, border_rect, width, inner_outsets).Paint();
  }

  static void DrawBoxSide(GraphicsContext& context,
                          const gfx::Rect& snapped_edge_rect,
                          BoxSide side,
                          Color color,
                          EBorderStyle style,
                          const AutoDarkMode& auto_dark_mode);

 private:
  // For PaintBorder().
  BoxBorderPainter(GraphicsContext&,
                   const PhysicalRect& border_rect,
                   const ComputedStyle&,
                   BackgroundBleedAvoidance,
                   PhysicalBoxSides sides_to_include);
  // For PaintSingleRectOutline().
  BoxBorderPainter(GraphicsContext&,
                   const ComputedStyle&,
                   const PhysicalRect& border_rect,
                   int width,
                   const PhysicalBoxStrut& inner_outsets);

  void Paint() const;

  struct ComplexBorderInfo;
  enum MiterType {
    kNoMiter,
    kSoftMiter,  // Anti-aliased
    kHardMiter,  // Not anti-aliased
  };

  void ComputeBorderProperties();

  BorderEdgeFlags PaintOpacityGroup(const ComplexBorderInfo&,
                                    unsigned index,
                                    float accumulated_opacity) const;
  void PaintSide(const ComplexBorderInfo&,
                 BoxSide,
                 float alpha,
                 BorderEdgeFlags) const;
  void PaintOneBorderSide(const gfx::Rect& side_rect,
                          BoxSide,
                          BoxSide adjacent_side1,
                          BoxSide adjacent_side2,
                          const Path*,
                          Color,
                          BorderEdgeFlags) const;
  bool PaintBorderFastPath() const;
  void DrawDoubleBorder() const;

  void DrawBoxSideFromPath(const Path&,
                           int thickness,
                           int draw_thickness,
                           BoxSide,
                           Color,
                           EBorderStyle) const;
  void DrawDashedDottedBoxSideFromPath(int thickness,
                                       int draw_thickness,
                                       Color,
                                       EBorderStyle) const;
  void DrawWideDottedBoxSideFromPath(const Path&, int thickness) const;
  void DrawDoubleBoxSideFromPath(const Path&,
                                 int thickness,
                                 int draw_thickness,
                                 BoxSide,
                                 Color) const;
  void DrawRidgeGrooveBoxSideFromPath(const Path&,
                                      int thickness,
                                      int draw_thickness,
                                      BoxSide,
                                      Color,
                                      EBorderStyle) const;
  void ClipBorderSidePolygon(BoxSide, MiterType miter1, MiterType miter2) const;
  gfx::Rect CalculateSideRectIncludingInner(BoxSide) const;

  MiterType ComputeMiter(BoxSide, BoxSide adjacent_side, BorderEdgeFlags) const;
  static bool MitersRequireClipping(MiterType miter1,
                                    MiterType miter2,
                                    EBorderStyle);

  PhysicalBoxStrut DoubleStripeOutsets(
      BorderEdge::DoubleBorderStripe stripe) const;
  PhysicalBoxStrut CenterOutsets() const;

  bool ColorsMatchAtCorner(BoxSide side, BoxSide adjacent_side) const;

  const BorderEdge& FirstEdge() const {
    DCHECK(visible_edge_set_);
    return edges_[first_visible_edge_];
  }

  BorderEdge& Edge(BoxSide side) { return edges_[static_cast<unsigned>(side)]; }
  const BorderEdge& Edge(BoxSide side) const {
    return edges_[static_cast<unsigned>(side)];
  }

  GraphicsContext& context_;

  // const inputs
  const PhysicalRect border_rect_;
  const PhysicalBoxStrut outer_outsets_;
  const ComputedStyle& style_;
  const BackgroundBleedAvoidance bleed_avoidance_;
  const PhysicalBoxSides sides_to_include_;

  // computed attributes
  ContouredRect outer_;
  ContouredRect inner_;
  BorderEdgeArray edges_;

  unsigned visible_edge_count_;
  unsigned first_visible_edge_;
  BorderEdgeFlags visible_edge_set_;
  DarkModeFilter::ElementRole element_role_;

  bool is_uniform_style_;
  bool is_uniform_width_;
  bool is_uniform_color_;
  bool is_rounded_;
  bool has_transparency_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_BORDER_PAINTER_H_
