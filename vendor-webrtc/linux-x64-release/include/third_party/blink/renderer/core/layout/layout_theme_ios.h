// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_IOS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_IOS_H_

#include "third_party/blink/renderer/core/layout/layout_theme_mobile.h"

namespace blink {

class LayoutThemeIOS final : public LayoutThemeMobile {
 public:
  static scoped_refptr<LayoutTheme> Create();
  bool DelegatesMenuListRendering() const override { return true; }
  Color PlatformActiveSelectionBackgroundColor(
      mojom::blink::ColorScheme color_scheme) const override;
  Color PlatformActiveSelectionForegroundColor(
      mojom::blink::ColorScheme color_scheme) const override;
  Color PlatformSpellingMarkerUnderlineColor() const override;
  Color PlatformGrammarMarkerUnderlineColor() const override;

 private:
  ~LayoutThemeIOS() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_IOS_H_
