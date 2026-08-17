// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_DATA_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_DATA_RANGE_H_

#include "third_party/blink/renderer/platform/fonts/shaping/glyph_data.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

struct ShapeResultRun;

// Represents a range of HarfBuzzRunGlyphData. |begin| and |end| follow the
// iterator pattern; i.e., |begin| is lower or equal to |end| in the address
// space regardless of LTR/RTL. |begin| is inclusive, |end| is exclusive.
class PLATFORM_EXPORT GlyphDataRange {
  DISALLOW_NEW();

 public:
  GlyphDataRange() = default;
  explicit GlyphDataRange(const ShapeResultRun&);

  unsigned size() const { return size_; }
  bool IsEmpty() const { return !size_; }

  // The `span` of `HarfBuzzRunGlyphData`.
  base::span<const HarfBuzzRunGlyphData> Glyphs() const;

  using const_iterator = const HarfBuzzRunGlyphData*;
  const_iterator begin() const;
  const_iterator end() const;

  bool HasOffsets() const;

  // The `span` of `GlyphOffset` if `HasOffsets()`, or an empty span.
  base::span<const GlyphOffset> Offsets() const;

  void Trace(Visitor*) const;

  GlyphDataRange FindGlyphDataRange(bool is_rtl,
                                    unsigned start_character_index,
                                    unsigned end_character_index) const;

 private:
  GlyphDataRange(const GlyphDataRange&,
                 const_iterator begin,
                 const_iterator end);

  Member<const ShapeResultRun> run_;
  wtf_size_t index_ = 0;
  wtf_size_t size_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_DATA_RANGE_H_
