/*
 * Copyright (C) 2007, 2008, 2009, 2011 Apple Inc. All rights reserved.
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

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ASCII_CTYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ASCII_CTYPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

// The behavior of many of the functions in the <ctype.h> header is dependent
// on the current locale. But in the WebKit project, all uses of those functions
// are in code processing something that's not locale-specific. These
// equivalents for some of the <ctype.h> functions are named more explicitly,
// not dependent on the C library locale, and we should also optimize them as
// needed.

// All functions return false or leave the character unchanged if passed a
// character that is outside the range 0-7F. So they can be used on Unicode
// strings or characters if the intent is to do processing only if the
// character is ASCII.

namespace WTF {

template <typename CharType>
constexpr inline bool IsASCII(CharType c) {
  return !(c & ~0x7F);
}

template <typename CharType>
inline bool IsASCIIAlpha(CharType c) {
  return (c | 0x20) >= 'a' && (c | 0x20) <= 'z';
}

template <typename CharType>
inline bool IsASCIIDigit(CharType c) {
  return c >= '0' && c <= '9';
}

template <typename CharType>
inline bool IsASCIIAlphanumeric(CharType c) {
  return IsASCIIDigit(c) || IsASCIIAlpha(c);
}

template <typename CharType>
inline bool IsASCIIHexDigit(CharType c) {
  return IsASCIIDigit(c) || ((c | 0x20) >= 'a' && (c | 0x20) <= 'f');
}

template <typename CharType>
inline bool IsASCIILower(CharType c) {
  return c >= 'a' && c <= 'z';
}

template <typename CharType>
inline bool IsASCIIPrintable(CharType c) {
  return c >= ' ' && c <= '~';
}

/*
 Statistics from a run of Apple's page load test for callers of IsASCIISpace:

 character          count
 ---------          -----
 non-spaces         689383
 20  space          294720
 0A  \n             89059
 09  \t             28320
 0D  \r             0
 0C  \f             0
 0B  \v             0
 */
template <typename CharType>
inline bool IsASCIISpace(CharType c) {
  return c <= ' ' && (c == ' ' || (c <= 0xD && c >= 0x9));
}

template <typename CharType>
inline bool IsASCIIUpper(CharType c) {
  return c >= 'A' && c <= 'Z';
}

WTF_EXPORT extern const LChar kASCIICaseFoldTable[256];

template <typename CharType>
inline CharType ToASCIILower(CharType c) {
  return c | ((c >= 'A' && c <= 'Z') << 5);
}

inline LChar ToASCIILower(LChar c) {
  return kASCIICaseFoldTable[c];
}

inline char ToASCIILower(char c) {
  return static_cast<char>(kASCIICaseFoldTable[static_cast<LChar>(c)]);
}

template <typename CharType>
constexpr inline CharType ToASCIIUpper(CharType c) {
  return c & ~((c >= 'a' && c <= 'z') << 5);
}

template <typename CharType>
inline int ToASCIIHexValue(CharType c) {
  DCHECK(IsASCIIHexDigit(c));
  return c < 'A' ? c - '0' : (c - 'A' + 10) & 0xF;
}

template <typename CharType>
inline int ToASCIIHexValue(CharType upper_value, CharType lower_value) {
  DCHECK(IsASCIIHexDigit(upper_value));
  DCHECK(IsASCIIHexDigit(lower_value));
  return ((ToASCIIHexValue(upper_value) << 4) & 0xF0) |
         ToASCIIHexValue(lower_value);
}

inline char LowerNibbleToASCIIHexDigit(char c) {
  char nibble = c & 0xF;
  return nibble < 10 ? '0' + nibble : 'A' + nibble - 10;
}

inline char UpperNibbleToASCIIHexDigit(char c) {
  char nibble = (c >> 4) & 0xF;
  return nibble < 10 ? '0' + nibble : 'A' + nibble - 10;
}

template <typename CharType>
inline bool IsASCIIAlphaCaselessEqual(CharType css_character, char character) {
  // This function compares a (preferably) constant ASCII
  // lowercase letter to any input character.
  DCHECK_GE(character, 'a');
  DCHECK_LE(character, 'z');
  if ((css_character | 0x20) == character) [[likely]] {
    return true;
  }
  return false;
}

}  // namespace WTF

using WTF::IsASCII;
using WTF::IsASCIIAlpha;
using WTF::IsASCIIAlphaCaselessEqual;
using WTF::IsASCIIAlphanumeric;
using WTF::IsASCIIDigit;
using WTF::IsASCIIHexDigit;
using WTF::IsASCIILower;
using WTF::IsASCIIPrintable;
using WTF::IsASCIISpace;
using WTF::IsASCIIUpper;
using WTF::LowerNibbleToASCIIHexDigit;
using WTF::ToASCIIHexValue;
using WTF::ToASCIILower;
using WTF::ToASCIIUpper;
using WTF::UpperNibbleToASCIIHexDigit;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ASCII_CTYPE_H_
