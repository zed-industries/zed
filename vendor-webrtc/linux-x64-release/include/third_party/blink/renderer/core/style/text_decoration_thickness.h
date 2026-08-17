// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_DECORATION_THICKNESS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_DECORATION_THICKNESS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/geometry/length.h"

namespace blink {

class TextDecorationThickness {
  DISALLOW_NEW();

 public:
  TextDecorationThickness();

  CORE_EXPORT explicit TextDecorationThickness(const Length& length);

  explicit TextDecorationThickness(CSSValueID from_font_keyword);

  bool IsFromFont() const { return thickness_from_font_; }
  const Length& Thickness() const {
    DCHECK(!thickness_from_font_);
    return thickness_;
  }
  bool IsAuto() const { return !thickness_from_font_ && thickness_.IsAuto(); }

  CORE_EXPORT bool operator==(const TextDecorationThickness&) const;
  bool operator!=(const TextDecorationThickness& other) const {
    return !(*this == other);
  }

 private:
  Length thickness_;
  bool thickness_from_font_{false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_DECORATION_THICKNESS_H_
