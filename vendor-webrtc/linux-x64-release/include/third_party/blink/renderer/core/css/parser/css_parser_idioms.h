/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_IDIOMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_IDIOMS_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

class CSSTokenizerInputStream;

// Space characters as defined by the CSS specification.
// http://www.w3.org/TR/css3-syntax/#whitespace
inline bool IsCSSSpace(UChar c) {
  return c == ' ' || c == '\t' || c == '\n';
}

inline bool IsCSSNewLine(UChar cc) {
  // We check \r and \f here, since we have no preprocessing stage
  return (cc == '\r' || cc == '\n' || cc == '\f');
}

// https://drafts.csswg.org/css-syntax/#name-start-code-point
template <typename CharacterType>
bool IsNameStartCodePoint(CharacterType c) {
  return IsASCIIAlpha(c) || c == '_' || !IsASCII(c);
}

// https://drafts.csswg.org/css-syntax/#name-code-point
template <typename CharacterType>
bool IsNameCodePoint(CharacterType c) {
  return IsNameStartCodePoint(c) || IsASCIIDigit(c) || c == '-';
}

// https://drafts.csswg.org/css-syntax/#check-if-two-code-points-are-a-valid-escape
inline bool TwoCharsAreValidEscape(UChar first, UChar second) {
  return first == '\\' && !IsCSSNewLine(second);
}

// Consumes a single whitespace, if the stream is currently looking at a
// whitespace. Note that \r\n counts as a single whitespace, as we don't do
// input preprocessing as a separate step.
//
// See https://drafts.csswg.org/css-syntax-3/#input-preprocessing
void ConsumeSingleWhitespaceIfNext(CSSTokenizerInputStream&);

// https://drafts.csswg.org/css-syntax/#consume-an-escaped-code-point
UChar32 ConsumeEscape(CSSTokenizerInputStream&);

// http://www.w3.org/TR/css3-syntax/#consume-a-name
String ConsumeName(CSSTokenizerInputStream&);

// https://drafts.csswg.org/css-syntax/#would-start-an-identifier
bool NextCharsAreIdentifier(UChar, const CSSTokenizerInputStream&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_IDIOMS_H_
