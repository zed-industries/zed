// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCROLLBAR_THEME_SETTINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCROLLBAR_THEME_SETTINGS_H_

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Scrollbar theme settings are only accessible from particular classes. Other
// code should use Page::GetScrollbarTheme(), and test code can also use
// ScopedMockOverlayScrollbars. These settings are not under
// RuntimeEnabledFeatures because OverlayScrollbars can only be set though
// WebRuntimeSettings for chrome, and blink tests code must set
// OverlayScrollbars and MockScrollbars at the same time with
// ScopedOverlayMockScrollbars (see the class for the reasons), unless the
// callers (the listed friend classes only) know that the reasons don't apply.
class PLATFORM_EXPORT ScrollbarThemeSettings {
 private:
  ScrollbarThemeSettings() = delete;

  friend class DevToolsEmulator;
  friend class Element;
  friend class Internals;
  friend class ScopedMockOverlayScrollbars;
  friend class ScrollbarsTest;
  friend class ScrollbarTheme;
  friend class OverlayScrollbarThemeFluentTest;
  friend class ScrollbarThemeFluentTest;
  friend class ScrollbarThemeMacTest;
  friend class WebRuntimeFeatures;
  friend class WebThemeEngineDefault;

  static void SetMockScrollbarsEnabled(bool);
  static bool MockScrollbarsEnabled();

  static void SetOverlayScrollbarsEnabled(bool);

  // This is the global overlay scrollbars setting. We also allow per-page
  // setting of Android overlay scrollbars, which overrides this setting, for
  // device emulation on desktop, so code should use Page::GetScrollbarTheme()
  // instead of this function.
  static bool OverlayScrollbarsEnabled();

  static void SetFluentScrollbarsEnabled(bool);
  static bool FluentScrollbarsEnabled();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCROLLBAR_THEME_SETTINGS_H_
