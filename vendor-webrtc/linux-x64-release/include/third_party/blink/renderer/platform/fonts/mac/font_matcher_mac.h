/*
 * Copyright (C) 2006, 2008 Apple Inc.  All rights reserved.
 * Copyright (C) 2007 Nicholas Shanks <webkit@nickshanks.com>
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_FONT_MATCHER_MAC_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_FONT_MATCHER_MAC_H_

#import <AppKit/AppKit.h>
#import <CoreText/CoreText.h>

#include "third_party/blink/renderer/platform/fonts/font_selection_types.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

PLATFORM_EXPORT NSFont* MatchNSFontFamily(const AtomicString& desired_family,
                                          NSFontTraitMask desired_traits,
                                          FontSelectionValue desired_weight,
                                          float size);

PLATFORM_EXPORT
base::apple::ScopedCFTypeRef<CTFontRef> MatchFontFamily(
    const AtomicString& desired_family,
    FontSelectionValue desired_weight,
    FontSelectionValue desired_slant,
    FontSelectionValue desired_width,
    float size);

PLATFORM_EXPORT
base::apple::ScopedCFTypeRef<CTFontRef> MatchUniqueFont(
    const AtomicString& unique_font_name,
    float size);

PLATFORM_EXPORT
base::apple::ScopedCFTypeRef<CTFontRef> MatchSystemUIFont(
    FontSelectionValue desired_weight,
    FontSelectionValue desired_slant,
    FontSelectionValue desired_width,
    float size);

// Converts a blink::FontSelectionValue to the nearest AppKit font weight if
// possible, otherwise returns the default font weight.
int ToAppKitFontWeight(FontSelectionValue);

PLATFORM_EXPORT
int ToCSSFontWeight(float ct_font_weight);

float ToCTFontWeight(int css_weight);

int AppKitToCSSFontWeight(int appkit_font_weight);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_MAC_FONT_MATCHER_MAC_H_
