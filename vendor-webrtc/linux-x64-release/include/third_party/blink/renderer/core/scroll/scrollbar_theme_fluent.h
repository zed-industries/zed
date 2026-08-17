// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_FLUENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_FLUENT_H_

#include "third_party/blink/public/platform/web_theme_engine.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scrollbar_theme.h"
#include "third_party/blink/renderer/core/scroll/scrollbar_theme_aura.h"

namespace blink {

// This scrollbar theme is only used for Fluent scrollbars.
// Please see the visual spec and the design document for more details:
// https://docs.google.com/document/d/1EpJnWAcPCxBQo6zPGR1Tg1NACiIJ-6dk7cYyK1DhBWw
class CORE_EXPORT ScrollbarThemeFluent : public ScrollbarThemeAura {
 public:
  ScrollbarThemeFluent(const ScrollbarThemeFluent&) = delete;
  ScrollbarThemeFluent& operator=(const ScrollbarThemeFluent&) = delete;

  static ScrollbarThemeFluent& GetInstance();
  ~ScrollbarThemeFluent() override = default;

  int ScrollbarThickness(float scale_from_dip,
                         EScrollbarWidth scrollbar_width) const override;
  bool UsesOverlayScrollbars() const override;
  bool UsesFluentScrollbars() const override;
  bool UsesFluentOverlayScrollbars() const override;
  // When scrollbars are main threaded the thumb size returned by ThumbRect()
  // is the expanded thumb size. This function shrinks the thumb and displaces
  // it to be near the correct Edge of the scrollable area.
  gfx::Rect ShrinkMainThreadedMinimalModeThumbRect(
      const Scrollbar&,
      const gfx::Rect& rect) const override;

  bool UsesSolidColorThumb() const override { return true; }
  bool UsesNinePatchTrackAndButtonsResource() const override;

 protected:
  ScrollbarThemeFluent();

  gfx::Rect ThumbRect(const Scrollbar&) const override;
  gfx::Size ButtonSize(const Scrollbar&) const override;

  void PaintTrackBackground(GraphicsContext&,
                            const Scrollbar&,
                            const gfx::Rect&) override;
  void PaintButton(GraphicsContext& context,
                   const Scrollbar& scrollbar,
                   const gfx::Rect& rect,
                   ScrollbarPart part) override;
  WebThemeEngine::ScrollbarThumbExtraParams BuildScrollbarThumbExtraParams(
      const Scrollbar&) const override;
  base::TimeDelta OverlayScrollbarFadeOutDelay() const override;
  base::TimeDelta OverlayScrollbarFadeOutDuration() const override;

  ScrollbarPart PartsToInvalidateOnThumbPositionChange(
      const Scrollbar&,
      float old_position,
      float new_position) const override;

 private:
  friend class ScrollbarThemeFluentMock;

  int ThumbThickness(float scale_from_dip,
                     EScrollbarWidth scrollbar_width) const;
  // Overlay scrollbar tracks have a invisible length-wise inset to give them a
  // floating appearance.
  gfx::Rect InsetButtonRect(const Scrollbar& scrollbar,
                            gfx::Rect rect,
                            ScrollbarPart part) const;
  gfx::Rect InsetTrackRect(const Scrollbar& scrollbar, gfx::Rect rect) const;
  int ScrollbarTrackInsetPx(float scale) const;

  // Button's height for vertical and width for the horizontal scrollbar.
  int scrollbar_button_length_;
  int scrollbar_thumb_thickness_;
  int scrollbar_track_thickness_;

  // Overlay scrollbar-related variables.
  int scrollbar_track_inset_ = 0;
  bool is_fluent_overlay_scrollbar_enabled_ = false;
  WebThemeEngine::ScrollbarStyle style_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_THEME_FLUENT_H_
