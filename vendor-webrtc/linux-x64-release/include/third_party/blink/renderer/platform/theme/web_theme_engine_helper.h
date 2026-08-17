// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_HELPER_H_

#include "third_party/blink/public/common/renderer_preferences/renderer_preferences.h"
#include "third_party/blink/public/platform/web_theme_engine.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT WebThemeEngineHelper {
 public:
  static WebThemeEngine* GetNativeThemeEngine();
  static void DidUpdateRendererPreferences(
      const blink::RendererPreferences& renderer_prefs);

  // Swaps the current theme engine out returning the old one.
  static std::unique_ptr<WebThemeEngine> SwapNativeThemeEngineForTesting(
      std::unique_ptr<WebThemeEngine> new_theme);

  // This is here instead of WebThemeEngineAndroid because we also need it for
  // DevTools device emulation.
  static const WebThemeEngine::ScrollbarStyle& AndroidScrollbarStyle();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_WEB_THEME_ENGINE_HELPER_H_
