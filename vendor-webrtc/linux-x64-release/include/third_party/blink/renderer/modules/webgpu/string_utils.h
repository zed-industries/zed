// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_STRING_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_STRING_UTILS_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Create WTF::String from a null-terminated char string. Treat the provided
// message as UTF-8 string, with Latin1 as fallback if the string is not valid
// UTF-8. Parts of Dawn's messages are user-defined strings like identifiers
// that could possibly be invalid UTF8, and WTF::String::FromUTF8 would result
// in a null string in this case. So use the FromUTF8WithLatin1Fallback methods
// that fallbacks to Latin1 for invalid UTF8 strings so that we are always able
// to display something.
WTF::String StringFromASCIIAndUTF8(std::string_view message);

// Create a utf8 std::string from a WTF:String and replace `\0` with `\0xFFFD`
// which is the unicode replacement codepoint. This used for some strings passed
// to Dawn which expects null terminated strings.
std::string UTF8StringFromUSVStringWithNullReplacedByReplacementCodePoint(
    const String& s);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_STRING_UTILS_H_
