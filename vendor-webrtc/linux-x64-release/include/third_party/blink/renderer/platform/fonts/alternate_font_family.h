/*
 * Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2007 Nicholas Shanks <webkit@nickshanks.com>
 * Copyright (C) 2013 Google, Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_ALTERNATE_FONT_FAMILY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_ALTERNATE_FONT_FAMILY_H_

#include "build/build_config.h"
#include "third_party/blink/renderer/platform/font_family_names.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// We currently do not support bitmap fonts on windows.
// Instead of trying to construct a bitmap font and then going down the fallback
// path map certain common bitmap fonts to their truetype equivalent up front.
inline const AtomicString& AdjustFamilyNameToAvoidUnsupportedFonts(
    const AtomicString& family_name) {
#if BUILDFLAG(IS_WIN)
  // On Windows, 'Courier New' (truetype font) is always present and
  // 'Courier' is a bitmap font. On Mac on the other hand 'Courier' is
  // a truetype font. Thus pages asking for Courier are better of
  // using 'Courier New' on windows.
  if (EqualIgnoringASCIICase(family_name, font_family_names::kCourier))
    return font_family_names::kCourierNew;

  // Alias 'MS Sans Serif' (bitmap font) -> 'Microsoft Sans Serif'
  // (truetype font).
  if (EqualIgnoringASCIICase(family_name, font_family_names::kMSSansSerif))
    return font_family_names::kMicrosoftSansSerif;

  // Alias 'MS Serif' (bitmap) -> 'Times New Roman' (truetype font).
  // Alias 'Times' -> 'Times New Roman' (truetype font).
  // There's no 'Microsoft Sans Serif-equivalent' for Serif.
  if (EqualIgnoringASCIICase(family_name, font_family_names::kMSSerif) ||
      EqualIgnoringASCIICase(family_name, font_family_names::kTimes))
    return font_family_names::kTimesNewRoman;
#endif

  return family_name;
}

inline const AtomicString& AlternateFamilyName(
    const AtomicString& family_name) {
  // Alias Courier <-> Courier New
  if (EqualIgnoringASCIICase(family_name, font_family_names::kCourier))
    return font_family_names::kCourierNew;
#if !BUILDFLAG(IS_WIN)
  // On Windows, Courier New (truetype font) is always present and
  // Courier is a bitmap font. So, we don't want to map Courier New to
  // Courier.
  if (EqualIgnoringASCIICase(family_name, font_family_names::kCourierNew))
    return font_family_names::kCourier;
#endif

  // Alias Times and Times New Roman.
  if (EqualIgnoringASCIICase(family_name, font_family_names::kTimes))
    return font_family_names::kTimesNewRoman;
  if (EqualIgnoringASCIICase(family_name, font_family_names::kTimesNewRoman))
    return font_family_names::kTimes;

  // Alias Arial and Helvetica
  if (EqualIgnoringASCIICase(family_name, font_family_names::kArial))
    return font_family_names::kHelvetica;
  if (EqualIgnoringASCIICase(family_name, font_family_names::kHelvetica))
    return font_family_names::kArial;

  return g_empty_atom;
}

inline const AtomicString& GetFallbackFontFamily(
    const FontDescription& description) {
  switch (description.GenericFamily()) {
    case FontDescription::kSansSerifFamily:
      return font_family_names::kSansSerif;
    case FontDescription::kSerifFamily:
      return font_family_names::kSerif;
    case FontDescription::kMonospaceFamily:
      return font_family_names::kMonospace;
    case FontDescription::kCursiveFamily:
      return font_family_names::kCursive;
    case FontDescription::kFantasyFamily:
      return font_family_names::kFantasy;
    default:
      // Let the caller use the system default font.
      return g_empty_atom;
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_ALTERNATE_FONT_FAMILY_H_
