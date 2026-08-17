/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_DESCRIPTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_DESCRIPTION_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

class FontDescription;

struct BLINK_PLATFORM_EXPORT WebFontDescription {
  enum GenericFamily {
    kGenericFamilyNone,
    kGenericFamilyStandard,
    kGenericFamilySerif,
    kGenericFamilySansSerif,
    kGenericFamilyMonospace,
    kGenericFamilyCursive,
    kGenericFamilyFantasy
  };

  enum Weight {
    kWeight100,
    kWeight200,
    kWeight300,
    kWeight400,
    kWeight500,
    kWeight600,
    kWeight700,
    kWeight800,
    kWeight900,
    kWeightNormal = kWeight400,
    kWeightBold = kWeight700
  };

  WebFontDescription() = default;

  WebString family;
  GenericFamily generic_family = kGenericFamilyNone;
  float size = 0;
  bool family_is_generic = false;
  bool italic = false;
  bool small_caps = false;
  Weight weight = kWeightNormal;

  int16_t letter_spacing = 0;
  int16_t word_spacing = 0;

#if INSIDE_BLINK
  WebFontDescription(const FontDescription&);
  operator FontDescription() const;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_DESCRIPTION_H_
