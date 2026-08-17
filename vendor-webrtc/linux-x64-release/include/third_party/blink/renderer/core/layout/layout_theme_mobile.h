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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_MOBILE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_MOBILE_H_

#include "third_party/blink/renderer/core/layout/layout_theme_default.h"

namespace blink {

class LayoutThemeMobile : public LayoutThemeDefault {
 public:
  static scoped_refptr<LayoutTheme> Create();
  String ExtraDefaultStyleSheet() override;

  void AdjustInnerSpinButtonStyle(ComputedStyleBuilder&) const override;

  String ExtraFullscreenStyleSheet() override;

  Color PlatformTapHighlightColor() const override {
    return LayoutThemeMobile::kDefaultTapHighlightColor;
  }

  Color PlatformActiveSelectionForegroundColor(
      mojom::blink::ColorScheme color_scheme) const override;

  Color PlatformActiveSelectionBackgroundColor(
      mojom::blink::ColorScheme color_scheme) const override;

  Color PlatformInactiveSelectionForegroundColor(
      mojom::blink::ColorScheme color_scheme) const override;

  Color PlatformInactiveSelectionBackgroundColor(
      mojom::blink::ColorScheme color_scheme) const override;

  void SetSelectionColors(Color active_background_color,
                          Color active_foreground_color,
                          Color inactive_background_color,
                          Color inactive_foreground_color) override;

 protected:
  ~LayoutThemeMobile() override;

 private:
  static Color active_selection_background_color_;
  static Color active_selection_foreground_color_;
  static Color inactive_selection_background_color_;
  static Color inactive_selection_foreground_color_;
  static constexpr Color kDefaultTapHighlightColor =
      Color::FromRGBA32(0x6633b5e5);
  static constexpr Color kDefaultActiveSelectionBackgroundColor =
      Color::FromRGBA32(0x6633b5e5);
  static constexpr Color kDefaultActiveSelectionForegroundColor =
      Color::FromRGBA32(0xFF000000);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_MOBILE_H_
