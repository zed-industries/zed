// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_INDEX_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_INDEX_RESULT_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct GlyphIndexResult {
  STACK_ALLOCATED();

 public:
  // The total number of characters of runs_[0..run_index - 1].
  unsigned characters_on_left_runs = 0;

  // Those are the left and right character indexes of the group of glyphs
  // that were selected by OffsetForPosition.
  unsigned left_character_index = 0;
  unsigned right_character_index = 0;

  // The glyph origin of the glyph.
  float origin_x = 0;
  // The advance of the glyph.
  float advance = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_INDEX_RESULT_H_
