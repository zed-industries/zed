// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_WIN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_WIN_H_

#include "third_party/blink/renderer/core/layout/layout_theme_default.h"

namespace blink {

class LayoutThemeWin final : public LayoutThemeDefault {
 public:
  static scoped_refptr<LayoutTheme> Create();

  Color SystemHighlightFromColorProvider(
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider) const override;

  // TODO(crbug.com/1092093): Implement IsAccentColorCustomized and
  // GetAccentColor to support system accent colors in windows.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_WIN_H_
