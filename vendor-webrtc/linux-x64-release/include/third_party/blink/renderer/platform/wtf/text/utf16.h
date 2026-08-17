// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_UTF16_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_UTF16_H_

#include <unicode/utf16.h>

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace WTF {

// U16_GET() for base::span.
//  - If text[offset] is a leading surrogate and text[offset + 1] is a
//    trailing surrogate, a code point computed from text[offset] and
//    text[offset + 1] is returned.
//  - If text[offset] is a trailing surrogate and text[offset - 1] is a
//    leading surrogate, a code point computed from text[offset - 1] and
//    text[offset] is returned.
//  - Otherwise, text[offset] is returned;
inline UChar32 CodePointAt(base::span<const UChar> text, size_t offset) {
  UChar32 code_point;
  U16_GET(text, 0, offset, text.size(), code_point);
  return code_point;
}

// U16_NEXT() for base::span.
// The return value is same as CodePointAt()'s. `offset` argument is updated
// to point the next of the read character.
template <typename T>
UChar32 CodePointAtAndNext(base::span<const UChar> text, T& offset) {
  UChar32 code_point;
  U16_NEXT(text, offset, text.size(), code_point);
  return code_point;
}
template <typename T>
UChar32 CodePointAtAndNext(base::span<const LChar> text, T& offset) {
  return text[offset++];
}

}  // namespace WTF

using WTF::CodePointAt;
using WTF::CodePointAtAndNext;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_UTF16_H_
