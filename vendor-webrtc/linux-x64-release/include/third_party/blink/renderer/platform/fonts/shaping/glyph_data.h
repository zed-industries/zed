// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_DATA_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/assertions.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

enum class SafeToBreak : uint8_t { kSafe = 0, kUnsafe = 1 };

inline bool IsSafeToBreak(SafeToBreak value) {
  return value == SafeToBreak::kSafe;
}

// Because glyph offsets are often zero, particularly for Latin runs, we hold it
// in |ShapeResultRun::GlyphDataCollection::offsets_| for reducing memory
// usage.
struct HarfBuzzRunGlyphData {
  DISALLOW_NEW();

  // The max number of characters in a |RunInfo| is limited by
  // |character_index|.
  static constexpr unsigned kCharacterIndexBits = 15;
  static constexpr unsigned kMaxCharacters = 1 << kCharacterIndexBits;
  static constexpr unsigned kMaxCharacterIndex = kMaxCharacters - 1;
  // The max number of glyphs in a |RunInfo|. This make the number
  // of glyphs predictable and minimizes the buffer reallocations.
  static constexpr unsigned kMaxGlyphs = kMaxCharacters;

  HarfBuzzRunGlyphData() = default;
  HarfBuzzRunGlyphData(unsigned glyph,
                       unsigned character_index,
                       SafeToBreak safe_to_break_before,
                       TextRunLayoutUnit advance)
      : glyph(glyph),
        character_index(character_index),
        unsafe_to_break_before(static_cast<bool>(safe_to_break_before)),
        advance(advance) {}

  SafeToBreak SafeToBreakBefore() const {
    return static_cast<SafeToBreak>(unsafe_to_break_before);
  }
  bool IsSafeToBreakBefore() const {
    return blink::IsSafeToBreak(SafeToBreakBefore());
  }
  void SetSafeToBreakBefore(SafeToBreak value) {
    unsafe_to_break_before = static_cast<bool>(value);
  }

  void SetAdvance(TextRunLayoutUnit value) { advance = value; }
  void AddAdvance(TextRunLayoutUnit value) { advance += value; }
  void SetAdvance(float value) {
    advance = TextRunLayoutUnit::FromFloatRound(value);
  }
  void AddAdvance(float value) {
    advance += TextRunLayoutUnit::FromFloatRound(value);
  }

  unsigned glyph : 16;
  // The index of the character this glyph is for. To use as an index of
  // |String|, it is the index of UTF16 code unit, and it is always at the
  // HarfBuzz cluster boundary.
  unsigned character_index : kCharacterIndexBits;
  unsigned unsafe_to_break_before : 1;

  TextRunLayoutUnit advance;
};
static_assert(std::is_trivially_copyable<HarfBuzzRunGlyphData>::value, "");

using GlyphOffset = gfx::Vector2dF;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_DATA_H_
