// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_DECORATION_OFFSET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_DECORATION_OFFSET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/fonts/font_baseline.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/length.h"

namespace blink {

class ComputedStyle;
class SimpleFontData;
enum class FontVerticalPositionType;
enum class ResolvedUnderlinePosition;

// Class for computing the decoration offset for text fragments in LayoutNG.
class CORE_EXPORT TextDecorationOffset {
  STACK_ALLOCATED();

 public:
  explicit TextDecorationOffset(const ComputedStyle& text_style)
      : text_style_(text_style) {}
  ~TextDecorationOffset() = default;

  int ComputeUnderlineOffsetForUnder(const Length& style_underline_offset,
                                     float computed_font_size,
                                     const SimpleFontData* font_data,
                                     float text_decoration_thickness,
                                     FontVerticalPositionType) const;

  int ComputeUnderlineOffset(ResolvedUnderlinePosition,
                             float computed_font_size,
                             const SimpleFontData* font_data,
                             const Length& style_underline_offset,
                             float text_decoration_thickness) const;

 private:
  static float StyleUnderlineOffsetToPixels(
      const Length& style_underline_offset,
      float font_size);

  const ComputedStyle& text_style_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_DECORATION_OFFSET_H_
