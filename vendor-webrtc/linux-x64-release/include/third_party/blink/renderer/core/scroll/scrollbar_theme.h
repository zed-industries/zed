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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_H_

#include "base/check_op.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/core/scroll/scrollbar.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/scrollbar_theme_settings.h"
#include "third_party/skia/include/core/SkColor.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class GraphicsContext;
class ScrollableArea;
class WebMouseEvent;
class WebViewImpl;

class CORE_EXPORT ScrollbarTheme {
  USING_FAST_MALLOC(ScrollbarTheme);

 public:
  ScrollbarTheme() = default;
  ScrollbarTheme(const ScrollbarTheme&) = delete;
  ScrollbarTheme& operator=(const ScrollbarTheme&) = delete;
  virtual ~ScrollbarTheme() = default;

  // If true, then scrollbars with this theme will be painted every time
  // Scrollbar::SetNeedsPaintInvalidation is called. If false, then only parts
  // which are explicitly invalidated will be repainted.
  virtual bool ShouldRepaintAllPartsOnInvalidation() const { return true; }

  virtual void UpdateEnabledState(const Scrollbar&) {}

  ScrollbarPart HitTestRootFramePosition(const Scrollbar&,
                                         const gfx::Point&) const;

  virtual int ScrollbarThickness(float scale_from_dip,
                                 EScrollbarWidth scrollbar_width) const {
    return 0;
  }
  virtual int ScrollbarMargin(float scale_from_dip,
                              EScrollbarWidth scrollbar_width) const {
    return 0;
  }

  virtual bool IsSolidColor() const { return false; }
  virtual bool UsesOverlayScrollbars() const { return false; }
  virtual bool UsesFluentScrollbars() const { return false; }
  virtual bool UsesFluentOverlayScrollbars() const { return false; }
  virtual gfx::Rect ShrinkMainThreadedMinimalModeThumbRect(
      const Scrollbar&,
      const gfx::Rect& rect) const {
    return rect;
  }

  // If true, scrollbars that become invisible (i.e. overlay scrollbars that
  // fade out) should be marked as disabled. This option exists since Mac and
  // Aura overlays implement the fade out differently, with Mac painting code
  // fading out the scrollbars. Aura scrollbars require disabling the scrollbar
  // to prevent painting it.
  virtual bool ShouldDisableInvisibleScrollbars() const { return true; }

  // If true, Blink is in charge of hiding/showing of overlay scrollbars.  As
  // above, this option exists because on Mac the visibility is controlled by
  // Mac painting code which Blink doesn't have an input into. In order to
  // prevent the two from getting out of sync we disable setting the Blink-side
  // parameter on Mac.
  virtual bool BlinkControlsOverlayVisibility() const { return true; }

  virtual bool InvalidateOnMouseEnterExit() const { return false; }

  // Returns parts of the scrollbar which must be repainted following a change
  // in the thumb position, given scroll positions before and after.
  virtual ScrollbarPart PartsToInvalidateOnThumbPositionChange(
      const Scrollbar&,
      float old_position,
      float new_position) const {
    return kAllParts;
  }

  virtual void PaintScrollCorner(GraphicsContext&,
                                 const ScrollableArea&,
                                 const DisplayItemClient&,
                                 const gfx::Rect& corner_rect);
  virtual void PaintTickmarks(GraphicsContext&,
                              const Scrollbar&,
                              const gfx::Rect&);
  virtual SkColor4f ThumbColor(const Scrollbar&) const { NOTREACHED(); }

  virtual bool ShouldCenterOnThumb(const Scrollbar&,
                                   const WebMouseEvent&) const {
    return false;
  }
  virtual bool ShouldSnapBackToDragOrigin(const Scrollbar&,
                                          const WebMouseEvent&) const {
    return false;
  }
  virtual bool ShouldDragDocumentInsteadOfThumb(const Scrollbar&,
                                                const WebMouseEvent&) const {
    return false;
  }

  virtual bool SupportsDragSnapBack() const { return false; }
  virtual bool JumpOnTrackClick() const { return false; }

  // The position of the thumb relative to the track.
  int ThumbPosition(const Scrollbar& scrollbar) const {
    return ThumbPosition(scrollbar, scrollbar.CurrentPos());
  }
  virtual base::TimeDelta OverlayScrollbarFadeOutDelay() const;
  virtual base::TimeDelta OverlayScrollbarFadeOutDuration() const;
  // The position the thumb would have, relative to the track, at the specified
  // scroll position.
  virtual int ThumbPosition(const Scrollbar&, float scroll_position) const;
  // The length of the thumb along the axis of the scrollbar.
  virtual int ThumbLength(const Scrollbar&) const;
  // The position of the track relative to the scrollbar.
  virtual int TrackPosition(const Scrollbar&) const;
  // The length of the track along the axis of the scrollbar.
  virtual int TrackLength(const Scrollbar&) const;
  // The opacity to be applied to the scrollbar.
  virtual float Opacity(const Scrollbar&) const { return 1.0f; }

  // Whether the native theme of the OS has scrollbar buttons.
  virtual bool NativeThemeHasButtons() const = 0;
  // Whether the scrollbar has buttons. It's the same as NativeThemeHasButtons()
  // except for custom scrollbars which can override the OS settings.
  virtual bool HasButtons(const Scrollbar&) const {
    return NativeThemeHasButtons();
  }

  virtual bool HasThumb(const Scrollbar&) const = 0;

  // All these rects are in the same coordinate space as the scrollbar's
  // FrameRect.
  virtual gfx::Rect BackButtonRect(const Scrollbar&) const = 0;
  virtual gfx::Rect ForwardButtonRect(const Scrollbar&) const = 0;
  virtual gfx::Rect TrackRect(const Scrollbar&) const = 0;
  virtual gfx::Rect ThumbRect(const Scrollbar&) const;

  virtual int MinimumThumbLength(const Scrollbar&) const = 0;

  virtual void SplitTrack(const Scrollbar&,
                          const gfx::Rect& track,
                          gfx::Rect& start_track,
                          gfx::Rect& thumb,
                          gfx::Rect& end_track) const;

  virtual void PaintThumb(GraphicsContext&,
                          const Scrollbar&,
                          const gfx::Rect&) {}

  // Paints the track (including tickmarks if present) and the buttons.
  void PaintTrackAndButtons(GraphicsContext&,
                            const Scrollbar&,
                            const gfx::Rect&);

  virtual base::TimeDelta InitialAutoscrollTimerDelay() const;
  virtual base::TimeDelta AutoscrollTimerDelay() const;

  virtual gfx::Rect ConstrainTrackRectToTrackPieces(
      const Scrollbar&,
      const gfx::Rect& rect) const {
    return rect;
  }

  virtual void RegisterScrollbar(Scrollbar&) {}

  virtual bool IsMockTheme() const { return false; }

  virtual bool UsesNinePatchThumbResource() const { return false; }
  virtual bool UsesSolidColorThumb() const { return false; }
  virtual gfx::Insets SolidColorThumbInsets(const Scrollbar& scrollbar) const {
    NOTREACHED();
  }
  virtual bool UsesNinePatchTrackAndButtonsResource() const { return false; }

  // For a nine-patch scrollbar, this defines the painting canvas size which the
  // painting code will use to paint the scrollbar into. The actual scrollbar
  // dimensions will be ignored for purposes of painting since the resource can
  // be then resized without a repaint.
  virtual gfx::Size NinePatchThumbCanvasSize(const Scrollbar&) const {
    NOTREACHED();
  }

  // For a nine-patch resource, the aperture defines the center patch that will
  // be stretched out.
  virtual gfx::Rect NinePatchThumbAperture(const Scrollbar&) const {
    NOTREACHED();
  }

  // For a nine-patch scrollbar, this defines the painting canvas size which the
  // painting code will use to paint the scrollbar into. The actual scrollbar
  // dimensions will be ignored for purposes of painting since the resource can
  // be then resized without a repaint.
  virtual gfx::Size NinePatchTrackAndButtonsCanvasSize(const Scrollbar&) const {
    NOTREACHED();
  }
  virtual gfx::Size NinePatchTrackAndButtonsCanvasSize(const Scrollbar&,
                                                       float scale) const {
    NOTREACHED();
  }

  // For a nine-patch resource, the aperture defines the center patch that will
  // be stretched out.
  virtual gfx::Rect NinePatchTrackAndButtonsAperture(const Scrollbar&) const {
    NOTREACHED();
  }
  virtual gfx::Rect NinePatchTrackAndButtonsAperture(const Scrollbar&,
                                                     float scale) const {
    NOTREACHED();
  }

  virtual bool AllowsHitTest() const { return true; }

 protected:
  // The point is in the same coordinate space as the scrollbar's FrameRect.
  virtual ScrollbarPart HitTest(const Scrollbar&, const gfx::Point&) const;

  virtual int TickmarkBorderWidth() const { return 0; }
  // Paints the background of the track, not including tickmarks.
  virtual void PaintTrackBackground(GraphicsContext&,
                                    const Scrollbar&,
                                    const gfx::Rect&) {}
  virtual void PaintButton(GraphicsContext&,
                           const Scrollbar&,
                           const gfx::Rect&,
                           ScrollbarPart) {}

  virtual void PaintTrackBackgroundAndButtons(GraphicsContext& context,
                                              const Scrollbar&,
                                              const gfx::Rect&);

 protected:
  // For GetTheme().
  friend class MockScrollableArea;
  friend class MockScrollableAreaForAnimatorTest;
  friend class Page;
  // For MockScrollbarsEnabled().
  friend class WebViewImpl;

  // Get the theme based on global scrollbar settings. We should always use
  // Page::GetScrollbarTheme() to get scrollbar theme because we support
  // different native scrollbar theme base on page settings.
  // See http://crrev.com/c/646727.
  static ScrollbarTheme& GetTheme();

  static bool OverlayScrollbarsEnabled() {
    return ScrollbarThemeSettings::OverlayScrollbarsEnabled();
  }
  static bool MockScrollbarsEnabled() {
    return ScrollbarThemeSettings::MockScrollbarsEnabled();
  }
  static bool FluentScrollbarsEnabled() {
    return ScrollbarThemeSettings::FluentScrollbarsEnabled();
  }

 private:
  // Must be implemented to return the correct theme subclass.
  static ScrollbarTheme& NativeTheme();
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_H_
