// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_MAC_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_MAC_H_

#include "third_party/blink/renderer/platform/theme/web_theme_engine_default.h"

namespace blink {

class WebThemeEngineMac : public WebThemeEngineDefault {
 public:
  ~WebThemeEngineMac() override {}

  void Paint(cc::PaintCanvas* canvas,
             WebThemeEngine::Part part,
             WebThemeEngine::State state,
             const gfx::Rect& rect,
             const WebThemeEngine::ExtraParams* extra_params,
             mojom::ColorScheme color_scheme,
             bool in_forced_colors,
             const ui::ColorProvider* color_provider,
             const std::optional<SkColor>& accent_color) override;

  static bool IsScrollbarPart(WebThemeEngine::Part part);
  static void PaintMacScrollBarParts(
      cc::PaintCanvas* canvas,
      const ui::ColorProvider* color_provider,
      WebThemeEngine::Part part,
      WebThemeEngine::State state,
      const gfx::Rect& rect,
      const WebThemeEngine::ExtraParams* extra_params,
      mojom::ColorScheme color_scheme);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_MAC_H_
