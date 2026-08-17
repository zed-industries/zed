// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_LINUX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_LINUX_H_

#include "third_party/blink/renderer/core/layout/layout_theme_default.h"

namespace blink {

class LayoutThemeLinux final : public LayoutThemeDefault {
 public:
  static scoped_refptr<LayoutTheme> Create();
  String ExtraDefaultStyleSheet() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_THEME_LINUX_H_
