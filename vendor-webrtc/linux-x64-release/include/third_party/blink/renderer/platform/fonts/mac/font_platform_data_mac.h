/*
 * Copyright (c) 2019, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_FONT_PLATFORM_DATA_MAC_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_FONT_PLATFORM_DATA_MAC_H_

#include "third_party/blink/renderer/platform/fonts/font_optical_sizing.h"
#include "third_party/blink/renderer/platform/fonts/resolved_font_features.h"
#include "third_party/blink/renderer/platform/fonts/text_rendering_mode.h"
#include "third_party/blink/renderer/platform/platform_export.h"

@class NSFont;
class SkTypeface;
typedef uint32_t SkFourByteTag;

namespace blink {

enum class FontOrientation;
class FontPlatformData;
class FontVariationSettings;

// Given a typeface and a variable axis, returns whether a new value for that
// axis isn't clamped and therefore will effect a change to the typeface if
// applied.
bool PLATFORM_EXPORT VariableAxisChangeEffective(SkTypeface* typeface,
                                                 SkFourByteTag axis,
                                                 float new_value);

// Creates a FontPlatform object for specified original CTFont and font
// parameters. `size` is scaled size, `specified_size` is passed in to control
// optical sizing and tracking, needed in particular for the San Francisco
// system font.
const FontPlatformData* FontPlatformDataFromCTFont(
    CTFontRef,
    float size,
    float specified_size,
    bool synthetic_bold,
    bool synthetic_italic,
    TextRenderingMode text_rendering,
    ResolvedFontFeatures resolved_font_features,
    FontOrientation,
    OpticalSizing,
    const FontVariationSettings*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_FONT_PLATFORM_DATA_MAC_H_
