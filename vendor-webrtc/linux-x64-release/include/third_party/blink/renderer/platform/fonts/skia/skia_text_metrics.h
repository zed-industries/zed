// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SKIA_SKIA_TEXT_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SKIA_SKIA_TEXT_METRICS_H_

#include "third_party/blink/renderer/platform/fonts/glyph.h"

#include <hb.h>
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkRect.h"

class SkFont;

namespace blink {

// TODO: Width functions are affected by issue
// https://bugs.chromium.org/p/skia/issues/detail?id=10123 in Skia, which
// currently does not return trak-free advances on Mac OS 10.15.

void SkFontGetGlyphWidthForHarfBuzz(const SkFont&,
                                    hb_codepoint_t,
                                    hb_position_t* width);
void SkFontGetGlyphWidthForHarfBuzz(const SkFont&,
                                    unsigned count,
                                    const hb_codepoint_t* first_glyph,
                                    unsigned glyph_stride,
                                    hb_position_t* first_advance,
                                    unsigned advance_stride);
void SkFontGetGlyphExtentsForHarfBuzz(const SkFont&,
                                      hb_codepoint_t,
                                      hb_glyph_extents_t*);

void SkFontGetBoundsForGlyph(const SkFont&, Glyph, SkRect* bounds);
void SkFontGetBoundsForGlyphs(const SkFont&,
                              const Vector<Glyph, 256>&,
                              SkRect*);
float SkFontGetWidthForGlyph(const SkFont&, Glyph);

hb_position_t SkiaScalarToHarfBuzzPosition(SkScalar value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SKIA_SKIA_TEXT_METRICS_H_
