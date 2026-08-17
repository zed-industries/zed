/*
 * Copyright (C) 2015 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_CACHING_WORD_SHAPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_CACHING_WORD_SHAPER_H_

#include "third_party/blink/renderer/platform/fonts/shaping/shape_result_buffer.h"
#include "third_party/blink/renderer/platform/text/text_run.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

struct CharacterRange;
class Font;
class ShapeCache;
struct GlyphData;

class PLATFORM_EXPORT CachingWordShaper final {
  STACK_ALLOCATED();

 public:
  explicit CachingWordShaper(const Font& font) : font_(font) {}
  CachingWordShaper(const CachingWordShaper&) = delete;
  CachingWordShaper& operator=(const CachingWordShaper&) = delete;
  ~CachingWordShaper() = default;

  float Width(const TextRun&, gfx::RectF* glyph_bounds);
  int OffsetForPosition(const TextRun&,
                        float target_x,
                        IncludePartialGlyphsOption,
                        BreakGlyphsOption);

  void FillResultBuffer(const TextRun&, ShapeResultBuffer*);
  CharacterRange GetCharacterRange(const TextRun&, unsigned from, unsigned to);

  HeapVector<ShapeResult::RunFontData> GetRunFontData(const TextRun&) const;

  GlyphData EmphasisMarkGlyphData(const TextRun&) const;

 private:
  ShapeCache* GetShapeCache() const;

  const Font& font_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_CACHING_WORD_SHAPER_H_
