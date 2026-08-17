// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_VIEW_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_VIEW_PAINTER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}

namespace blink {

struct PaintInfo;
struct PhysicalRect;
class DisplayItemClient;
class Document;
class LayoutView;
class PhysicalBoxFragment;
class PropertyTreeStateOrAlias;

class ViewPainter {
  STACK_ALLOCATED();

 public:
  // The box fragment specified may be:
  // * The one and only LayoutView fragment (this is always the case when not
  //   printing)
  // * A page container fragment (for a given page), to fill the entire page,
  //   including the margin area, with an @page background
  explicit ViewPainter(const PhysicalBoxFragment& box_fragment)
      : box_fragment_(box_fragment) {}

  void PaintBoxDecorationBackground(const PaintInfo&);

  static bool ShouldApplyRootBackgroundBehavior(const Document&);

 private:
  void PaintRootElementGroup(
      const PaintInfo&,
      const gfx::Rect& pixel_snapped_background_rect,
      const PropertyTreeStateOrAlias& background_paint_state,
      const DisplayItemClient& background_client,
      bool painted_separate_backdrop,
      bool painted_separate_effect);

  void PaintRootGroup(const PaintInfo& paint_info,
                      const gfx::Rect& pixel_snapped_background_rect,
                      const Document&,
                      const DisplayItemClient& background_client,
                      const PropertyTreeStateOrAlias& state);

  const LayoutView& GetLayoutView() const;
  PhysicalRect BackgroundRect() const;

  const PhysicalBoxFragment& box_fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_VIEW_PAINTER_H_
