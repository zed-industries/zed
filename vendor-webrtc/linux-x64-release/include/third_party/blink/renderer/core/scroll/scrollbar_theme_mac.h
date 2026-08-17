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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_MAC_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_MAC_H_

#include "third_party/blink/renderer/core/scroll/scrollbar_theme.h"

namespace blink {

class CORE_EXPORT ScrollbarThemeMac : public ScrollbarTheme {
 public:
  ScrollbarThemeMac();
  ~ScrollbarThemeMac() override;

  void RegisterScrollbar(Scrollbar&) override;
  bool IsScrollbarRegistered(Scrollbar&) const;

  // On Mac, the painting code itself animates the opacity so there's no need
  // to disable in order to make the scrollbars invisible. In fact,
  // disabling/enabling causes invalidations which can cause endless loops as
  // Mac queues up scrollbar paint timers.
  bool ShouldDisableInvisibleScrollbars() const override { return false; }

  // On Mac, if Blink updates the visibility itself, it cannot tell the Mac
  // painting code about the change. Allowing it to change means the two can
  // get out of sync and can cause issues like Blink believing a scrollbar is
  // visible while the user cannot see it; this can lead to odd hit testing
  // behavior.
  bool BlinkControlsOverlayVisibility() const override { return false; }

  base::TimeDelta InitialAutoscrollTimerDelay() const override;
  base::TimeDelta AutoscrollTimerDelay() const override;

  void PaintTickmarks(GraphicsContext&,
                      const Scrollbar&,
                      const gfx::Rect&) override;

  bool ShouldCenterOnThumb(const Scrollbar&,
                           const WebMouseEvent&) const override;
  bool JumpOnTrackClick() const override;

  bool ShouldRepaintAllPartsOnInvalidation() const override { return false; }
  ScrollbarPart PartsToInvalidateOnThumbPositionChange(
      const Scrollbar&,
      float old_position,
      float new_position) const override;
  void UpdateEnabledState(const Scrollbar&) override;
  int ScrollbarThickness(float scale_from_dip,
                         EScrollbarWidth scrollbar_width) const override;
  bool UsesOverlayScrollbars() const override;

  void SetNewPainterForScrollbar(Scrollbar&);

  void PaintThumb(GraphicsContext& context,
                  const Scrollbar& scrollbar,
                  const gfx::Rect& rect) override;

  float Opacity(const Scrollbar&) const override;

  static bool PreferOverlayScrollerStyle();

  // See WebScrollbarTheme for parameters description.
  static void UpdateScrollbarsWithNSDefaults(
      std::optional<float> initial_button_delay,
      std::optional<float> autoscroll_button_delay,
      bool prefer_overlay_scroller_style,
      bool redraw,
      bool jump_on_track_click);

 protected:
  bool ShouldDragDocumentInsteadOfThumb(const Scrollbar&,
                                        const WebMouseEvent&) const override;

  gfx::Rect TrackRect(const Scrollbar&) const override;
  gfx::Rect BackButtonRect(const Scrollbar&) const override;
  gfx::Rect ForwardButtonRect(const Scrollbar&) const override;

  bool NativeThemeHasButtons() const override { return false; }
  bool HasThumb(const Scrollbar&) const override;

  int MinimumThumbLength(const Scrollbar&) const override;

  int TickmarkBorderWidth() const override { return 1; }

  void PaintTrackBackground(GraphicsContext&,
                            const Scrollbar&,
                            const gfx::Rect&) override;
  void PaintScrollCorner(GraphicsContext&,
                         const ScrollableArea&,
                         const DisplayItemClient&,
                         const gfx::Rect& corner_rect) override;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_MAC_H_
