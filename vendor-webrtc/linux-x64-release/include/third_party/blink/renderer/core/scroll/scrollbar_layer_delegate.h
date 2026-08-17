// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_LAYER_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_LAYER_DELEGATE_H_

#include "cc/input/scrollbar.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/skia/include/core/SkColor.h"

namespace blink {

class Scrollbar;

// Implementation of cc::Scrollbar, providing a delegate to query about
// scrollbar state and to paint the image in the scrollbar.
class CORE_EXPORT ScrollbarLayerDelegate : public cc::Scrollbar {
 public:
  explicit ScrollbarLayerDelegate(blink::Scrollbar& scrollbar);
  ScrollbarLayerDelegate(const ScrollbarLayerDelegate&) = delete;
  ScrollbarLayerDelegate& operator=(const ScrollbarLayerDelegate&) = delete;

  // cc::Scrollbar implementation.
  bool IsSame(const cc::Scrollbar& other) const override;
  cc::ScrollbarOrientation Orientation() const override;
  bool IsLeftSideVerticalScrollbar() const override;
  bool HasThumb() const override;
  bool IsSolidColor() const override;
  bool IsOverlay() const override;
  bool IsRunningWebTest() const override;
  bool IsFluentOverlayScrollbarMinimalMode() const override;
  bool SupportsDragSnapBack() const override;
  bool JumpOnTrackClick() const override;
  bool IsOpaque() const override;

  // The following rects are all relative to the scrollbar's origin.
  gfx::Rect ThumbRect() const override;
  gfx::Rect TrackRect() const override;
  gfx::Rect BackButtonRect() const override;
  gfx::Rect ForwardButtonRect() const override;

  float Opacity() const override;
  bool ThumbNeedsRepaint() const override;
  void ClearThumbNeedsRepaint() override;
  bool TrackAndButtonsNeedRepaint() const override;
  bool NeedsUpdateDisplay() const override;
  void ClearNeedsUpdateDisplay() override;
  bool HasTickmarks() const override;
  void PaintThumb(cc::PaintCanvas& canvas, const gfx::Rect& rect) override;
  void PaintTrackAndButtons(cc::PaintCanvas& canvas,
                            const gfx::Rect& rect) override;
  SkColor4f ThumbColor() const override;

  bool UsesNinePatchThumbResource() const override;
  gfx::Size NinePatchThumbCanvasSize() const override;
  gfx::Rect NinePatchThumbAperture() const override;
  bool UsesSolidColorThumb() const override;
  gfx::Insets SolidColorThumbInsets() const override;
  bool UsesNinePatchTrackAndButtonsResource() const override;
  gfx::Size NinePatchTrackAndButtonsCanvasSize(float scale) const override;
  gfx::Rect NinePatchTrackAndButtonsAperture(float scale) const override;
  gfx::Rect ShrinkMainThreadedMinimalModeThumbRect(
      gfx::Rect& rect) const override;

 private:
  ~ScrollbarLayerDelegate() override;

  bool ShouldPaint() const;

  Persistent<blink::Scrollbar> scrollbar_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_LAYER_DELEGATE_H_
