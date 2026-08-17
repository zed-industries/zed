// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_ANDROID_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_ANDROID_H_

#include "third_party/blink/public/platform/web_theme_engine.h"

namespace blink {

class WebThemeEngineAndroid : public blink::WebThemeEngine {
 public:
  // WebThemeEngine methods:
  ~WebThemeEngineAndroid() override;
  gfx::Size GetSize(blink::WebThemeEngine::Part) override;
  void GetOverlayScrollbarStyle(
      blink::WebThemeEngine::ScrollbarStyle*) override;
  void Paint(cc::PaintCanvas* canvas,
             blink::WebThemeEngine::Part part,
             blink::WebThemeEngine::State state,
             const gfx::Rect& rect,
             const blink::WebThemeEngine::ExtraParams* extra_params,
             blink::mojom::ColorScheme color_scheme,
             bool in_forced_colors,
             const ui::ColorProvider* color_provider,
             const std::optional<SkColor>& accent_color) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_ANDROID_H_
