// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_APPLIED_TEXT_DECORATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_APPLIED_TEXT_DECORATION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/text_decoration_thickness.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT AppliedTextDecoration {
  DISALLOW_NEW();

 public:
  AppliedTextDecoration(TextDecorationLine,
                        ETextDecorationStyle,
                        Color,
                        TextDecorationThickness,
                        Length);

  TextDecorationLine Lines() const {
    return static_cast<TextDecorationLine>(lines_);
  }
  ETextDecorationStyle Style() const {
    return static_cast<ETextDecorationStyle>(style_);
  }
  Color GetColor() const { return color_; }
  void SetColor(Color color) { color_ = color; }

  TextDecorationThickness Thickness() const { return thickness_; }
  Length UnderlineOffset() const { return underline_offset_; }

  bool operator==(const AppliedTextDecoration&) const;
  bool operator!=(const AppliedTextDecoration& o) const {
    return !(*this == o);
  }

 private:
  unsigned lines_ : kTextDecorationLineBits;
  unsigned style_ : 3;  // ETextDecorationStyle
  Color color_;
  TextDecorationThickness thickness_;
  Length underline_offset_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_APPLIED_TEXT_DECORATION_H_
