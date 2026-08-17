// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_MATH_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_MATH_TRANSFORM_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {
namespace unicode {

// Performs the character mapping needed to implement MathML's mathvariant
// attribute. It takes a unicode character and maps it to its appropriate
// mathvariant counterpart specified by mathvariant. The mapped character is
// typically located within Unicode's mathematical blocks (U+1D400–U+1D7FF,
// U+1EExx) but there are exceptions which this function accounts for.
// Characters without a valid mapping value are returned unaltered. Characters
// already in the mathematical blocks (or are one of the exceptions) are never
// transformed. Acceptable values for mathvariant are specified in
// MathMLElement.h. The transformable characters can be found at:
// http://lists.w3.org/Archives/Public/www-math/2013Sep/0012.html and
// https://unicode.org/cldr/utility/character.jsp.
// TODO(https://crbug.com/1076420): this needs to handle all mathvariants, not
// just italics.
WTF_EXPORT UChar32 ItalicMathVariant(UChar32 code_point);

}  // namespace unicode
}  // namespace WTF

using WTF::unicode::ItalicMathVariant;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_MATH_TRANSFORM_H_
