// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_BUFFER_H_

#include "third_party/blink/renderer/platform/fonts/shaping/shape_result.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

struct CharacterRange;
class FontDescription;
struct GlyphData;
class ShapeResultBloberizer;
class TextRun;

class PLATFORM_EXPORT ShapeResultBuffer {
  STACK_ALLOCATED();

 public:
  ShapeResultBuffer() : has_vertical_offsets_(false) {}
  ShapeResultBuffer(const ShapeResultBuffer&) = delete;
  ShapeResultBuffer& operator=(const ShapeResultBuffer&) = delete;

  void AppendResult(const ShapeResult* result) {
    has_vertical_offsets_ |= result->HasVerticalOffsets();
    results_.push_back(result);
  }

  bool HasVerticalOffsets() const { return has_vertical_offsets_; }

  int OffsetForPosition(const TextRun& run,
                        float target_x,
                        IncludePartialGlyphsOption,
                        BreakGlyphsOption) const;
  CharacterRange GetCharacterRange(const StringView& text,
                                   TextDirection,
                                   float total_width,
                                   unsigned from,
                                   unsigned to) const;

  HeapVector<ShapeResult::RunFontData> GetRunFontData() const;

  wtf_size_t ShapeResultSize() const { return results_.size(); }
  ShapeResultView* ViewAt(wtf_size_t index) const;

  GlyphData EmphasisMarkGlyphData(const FontDescription&) const;

  struct CharacterRangeContext {
    const StringView& text;
    const bool is_rtl;
    int from;
    int to;
    float current_x;
    unsigned total_num_characters = 0;
    std::optional<float> from_x;
    std::optional<float> to_x;
    float min_y = 0;
    float max_y = 0;
  };
  // A helper for GetCharacterRange().
  static void ComputeRangeIn(const ShapeResult& result,
                             const gfx::RectF& ink_bounds,
                             CharacterRangeContext& context);

 private:
  friend class ShapeResultBloberizer;

  // Empirically, cases where we get more than 50 ShapeResults are extremely
  // rare.
  HeapVector<Member<const ShapeResult>, 64> results_;
  bool has_vertical_offsets_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_BUFFER_H_
