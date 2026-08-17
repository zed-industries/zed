/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_OVERLAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_OVERLAY_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scrollbar_theme.h"

namespace blink {

// This scrollbar theme is used to get overlay scrollbar for platforms other
// than Mac. Mac's overlay scrollbars are in ScrollbarThemeMac*.
class CORE_EXPORT ScrollbarThemeOverlay : public ScrollbarTheme {
 public:
  static ScrollbarThemeOverlay& GetInstance();

  ~ScrollbarThemeOverlay() override = default;

  bool ShouldRepaintAllPartsOnInvalidation() const override;

  ScrollbarPart PartsToInvalidateOnThumbPositionChange(
      const Scrollbar&,
      float old_position,
      float new_position) const override;

  int ScrollbarThickness(float scale_from_dip,
                         EScrollbarWidth scrollbar_width) const override;
  int ScrollbarMargin(float scale_from_dip,
                      EScrollbarWidth scrollbar_width) const override;
  bool UsesOverlayScrollbars() const override;
  base::TimeDelta OverlayScrollbarFadeOutDelay() const override;
  base::TimeDelta OverlayScrollbarFadeOutDuration() const override;

  int ThumbLength(const Scrollbar&) const override;
  int ThumbThickness(float scale_from_dip,
                     EScrollbarWidth scrollbar_width) const;

  bool NativeThemeHasButtons() const override { return false; }
  bool HasThumb(const Scrollbar&) const override;

  gfx::Rect BackButtonRect(const Scrollbar&) const override;
  gfx::Rect ForwardButtonRect(const Scrollbar&) const override;
  gfx::Rect TrackRect(const Scrollbar&) const override;
  gfx::Rect ThumbRect(const Scrollbar&) const override;

  void PaintThumb(GraphicsContext&,
                  const Scrollbar&,
                  const gfx::Rect&) override;

  bool UsesNinePatchThumbResource() const override;
  gfx::Size NinePatchThumbCanvasSize(const Scrollbar&) const override;
  gfx::Rect NinePatchThumbAperture(const Scrollbar&) const override;

  int MinimumThumbLength(const Scrollbar&) const override;

 protected:
  FRIEND_TEST_ALL_PREFIXES(ScrollbarThemeOverlayTest, PaintInvalidation);

  ScrollbarThemeOverlay(int thumb_thickness_default_dip,
                        int scrollbar_margin_default_dip,
                        int thumb_thickness_thin_dip,
                        int scrollbar_margin_thin_dip);

  ScrollbarPart HitTest(const Scrollbar&, const gfx::Point&) const override;

 private:
  int thumb_thickness_default_dip_;
  int scrollbar_margin_default_dip_;
  int thumb_thickness_thin_dip_;
  int scrollbar_margin_thin_dip_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_OVERLAY_H_
