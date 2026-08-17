// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_MATH_TEST_FONTS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_MATH_TEST_FONTS_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Font;

// operators.woff contains stretchy operators from the MathML operator
// dictionary (including left and over braces) represented by squares. It also
// contains glyphs h0, h1, h2, h3 and v0, v1, v2, v3 that are respectively
// horizontal and vertical rectangles of increasing size. The MathVariants table
// contains the following data for horizontal (respectively vertical) operators:
// - Glyph variants: h0, h1, h2, h3 (respectively v0, v1, v2, v3).
// - Glyph parts: non-extender h2 and extender h1 (respectively v2 and v1).
// stretchy.woff and stretchy-centered-on-baseline.woff contain similar stretchy
// constructions for horizontal and vertical arrows only. For the latter, the
// glyphs are centered on the baseline.
// For details, see createSizeVariants() and createStretchy() from
// third_party/blink/web_tests/external/wpt/mathml/tools/operator-dictionary.py
const UChar32 kLeftBraceCodePoint = '{';
const UChar32 kOverBraceCodePoint = 0x23DE;
const UChar32 kVerticalArrow = 0x295C;
const UChar32 kHorizontalArrow = 0x295A;
PLATFORM_EXPORT void retrieveGlyphForStretchyOperators(
    const blink::Font* operatorsWoff,
    Vector<UChar32>& verticalGlyphs,
    Vector<UChar32>& horizontalGlyphs);

// The largeop-displayoperatorminheight*-2AFF-italiccorrection*.woff fonts
// contain the largeop 0x2AFF character from the MathML operator dictionary,
// represented by a square. They also contain glyphs and constructions to
// provide large variants for this character. For details, see
// third_party/blink/web_tests/external/wpt/mathml/tools/largeop.py
const UChar32 kNAryWhiteVerticalBarCodePoint = 0x2AFF;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_MATH_TEST_FONTS_H_
