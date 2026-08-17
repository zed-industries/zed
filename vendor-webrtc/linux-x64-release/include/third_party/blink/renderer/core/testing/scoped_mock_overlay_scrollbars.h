// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SCOPED_MOCK_OVERLAY_SCROLLBARS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SCOPED_MOCK_OVERLAY_SCROLLBARS_H_

#include "base/check_op.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/core/scroll/scrollbar_theme.h"

namespace blink {

// Without the default parameter, the instance forces to use mocked overlay
// scrollbars instead of through WebThemeEngine to avoid crash in unit tests
// that don't initialize chrome WebThemeEngine implementation, and to ensure
// consistent layout regardless of differences between scrollbar themes.
//
// WebViewHelper and PageTestBase include this, so this is only needed if a
// test tests non-overlay scrollbars, or needs a bigger scope of mock overlay
// scrollbar settings than the scope of them.
//
// Note that ScopedMockOverlayScrollbars(false) doesn't always force
// non-overlay scrollbars because the platform (e.g. Android) may not support
// non-overlay scrollbars. Use USE_NON_OVERLAY_SCROLLBAR() instead. It will
// skip the test if non-overlay scrollbar is not supported by the platform.
//
// Note that unit tests should use ScopedMockOverlayScrollbars instead of
// ScrollbarThemeSettings::SetOverlayScrollbarsEnabled() because overlay
// scrollbars in testing must be mocked for the following reasons:
// 1. ScrollbarThemeSettings::SetOverlayScrollbarsEnabled() controls blink
//    scrollbar theme only, not the chrome WebThemeEngine implementation;
// 2. The chrome WebThemeEngine implementation may not support overlay
//    scrollbars;
// 3. The chrome WebThemeEngine implementation may not support dynamic
//    switching between overlay and non-overlay scrollbars.
//
class ScopedMockOverlayScrollbars {
 public:
  explicit ScopedMockOverlayScrollbars(bool use_mock_overlay_scrollbars = true)
      : use_mock_overlay_scrollbars_(use_mock_overlay_scrollbars),
        original_mock_scrollbars_enabled_(
            ScrollbarThemeSettings::MockScrollbarsEnabled()),
        original_overlay_scrollbars_enabled_(
            ScrollbarThemeSettings::OverlayScrollbarsEnabled()) {
    ScrollbarThemeSettings::SetOverlayScrollbarsEnabled(
        use_mock_overlay_scrollbars);
    ScrollbarThemeSettings::SetMockScrollbarsEnabled(
        use_mock_overlay_scrollbars);
  }

  ~ScopedMockOverlayScrollbars() {
    // Failure of any of these DCHECKs means that the settings are changed not
    // by this class, or incorrect nesting of instances of this class.
    DCHECK_EQ(use_mock_overlay_scrollbars_,
              ScrollbarThemeSettings::OverlayScrollbarsEnabled());
    DCHECK_EQ(use_mock_overlay_scrollbars_,
              ScrollbarThemeSettings::MockScrollbarsEnabled());
    ScrollbarThemeSettings::SetMockScrollbarsEnabled(
        original_mock_scrollbars_enabled_);
    ScrollbarThemeSettings::SetOverlayScrollbarsEnabled(
        original_overlay_scrollbars_enabled_);
  }

  bool IsSuccessful() const {
    // Our mock overlay scrollbar theme shortcuts WebThemeEngine, so it's
    // platform independent.
    if (use_mock_overlay_scrollbars_)
      return true;
#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_IOS) || \
    BUILDFLAG(IS_FUCHSIA)
    // Non-overlay scrollbar is not supported on these platforms.
    return false;
#else
    return true;
#endif
  }

 private:
  bool use_mock_overlay_scrollbars_;
  bool original_mock_scrollbars_enabled_;
  bool original_overlay_scrollbars_enabled_;
};

// This is used in tests that needs non-overlay scrollbars. To make sure the
// 'return' works, this macro must be used in a test directly, not in a function
// called by a test or a compound statement.
#define USE_NON_OVERLAY_SCROLLBARS_OR_QUIT()                 \
  ScopedMockOverlayScrollbars non_overlay_scrollbars(false); \
  if (!non_overlay_scrollbars.IsSuccessful())                \
  return

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SCOPED_MOCK_OVERLAY_SCROLLBARS_H_
