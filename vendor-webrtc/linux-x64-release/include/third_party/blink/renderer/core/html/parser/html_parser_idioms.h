/*
 * Copyright (C) 2010 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PARSER_IDIOMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PARSER_IDIOMS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/html/parser/literal_buffer.h"
#include "third_party/blink/renderer/platform/wtf/decimal.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class TextEncoding;
}

namespace blink {

// Strip leading and trailing whitespace as defined by the HTML specification.
CORE_EXPORT String StripLeadingAndTrailingHTMLSpaces(const String&);

// https://infra.spec.whatwg.org/#split-on-ascii-whitespace
CORE_EXPORT Vector<String> SplitOnASCIIWhitespace(const String&);

// An implementation of the HTML specification's algorithm to convert a number
// to a string for number and range types.
String SerializeForNumberType(const Decimal&);
String SerializeForNumberType(double);

// Convert the specified string to a decimal/double. If the conversion fails,
// the return value is fallback value or NaN if not specified. Leading or
// trailing illegal characters cause failure, as does passing an empty string.
// The double* parameter may be 0 to check if the string can be parsed without
// getting the result.
Decimal ParseToDecimalForNumberType(
    const String&,
    const Decimal& fallback_value = Decimal::Nan());
CORE_EXPORT double ParseToDoubleForNumberType(
    const String&,
    double fallback_value = std::numeric_limits<double>::quiet_NaN());

// http://www.whatwg.org/specs/web-apps/current-work/#rules-for-parsing-integers
CORE_EXPORT bool ParseHTMLInteger(const String&, int&);

// http://www.whatwg.org/specs/web-apps/current-work/#rules-for-parsing-non-negative-integers
CORE_EXPORT bool ParseHTMLNonNegativeInteger(const String&, unsigned&);

// https://html.spec.whatwg.org/C/#clamped-to-the-range
// without default value processing.
bool ParseHTMLClampedNonNegativeInteger(const String&,
                                        unsigned min,
                                        unsigned max,
                                        unsigned&);

// https://html.spec.whatwg.org/C/#rules-for-parsing-a-list-of-floating-point-numbers
CORE_EXPORT Vector<double> ParseHTMLListOfFloatingPointNumbers(const String&);

typedef Vector<std::pair<String, String>> HTMLAttributeList;
// The returned encoding might not be valid.
WTF::TextEncoding EncodingFromMetaAttributes(const HTMLAttributeList&);

// Space characters as defined by the HTML specification.
template <typename CharType>
inline bool IsHTMLSpace(CharType character) {
  // Histogram from Apple's page load test combined with some ad hoc browsing
  // some other test suites.
  //
  //     82%: 216330 non-space characters, all > U+0020
  //     11%:  30017 plain space characters, U+0020
  //      5%:  12099 newline characters, U+000A
  //      2%:   5346 tab characters, U+0009
  //
  // No other characters seen. No U+000C or U+000D, and no other control
  // characters. Accordingly, we check for non-spaces first, then space, then
  // newline, then tab, then the other characters.

  return character <= ' ' &&
         (character == ' ' || character == '\n' || character == '\t' ||
          character == '\r' || character == '\f');
}

template <typename CharType>
ALWAYS_INLINE bool IsHTMLSpecialWhitespace(CharType character) {
  return character <= '\r' && (character == '\r' || character == '\n' ||
                               character == '\t' || character == '\f');
}

template <typename CharType>
inline bool IsComma(CharType character) {
  return character == ',';
}

template <typename CharType>
inline bool IsColon(CharType character) {
  return character == ':';
}

template <typename CharType>
inline bool IsHTMLSpaceOrComma(CharType character) {
  return IsComma(character) || IsHTMLSpace(character);
}

inline bool IsHTMLLineBreak(UChar character) {
  return character <= '\r' && (character == '\n' || character == '\r');
}

template <typename CharType>
inline bool IsNotHTMLSpace(CharType character) {
  return !IsHTMLSpace<CharType>(character);
}

template <typename CharType>
inline bool IsHTMLSpaceNotLineBreak(CharType character) {
  return IsHTMLSpace<CharType>(character) && !IsHTMLLineBreak(character);
}

bool ThreadSafeMatch(const QualifiedName&, const QualifiedName&);
bool ThreadSafeMatch(const String&, const QualifiedName&);

enum CharacterWidth { kLikely8Bit, kForce8Bit, kForce16Bit };

String AttemptStaticStringCreation(base::span<const LChar>);
String AttemptStaticStringCreation(base::span<const UChar>, CharacterWidth);

template <wtf_size_t inlineCapacity>
inline static String AttemptStaticStringCreation(
    const UCharLiteralBuffer<inlineCapacity>& vector) {
  return AttemptStaticStringCreation(
      vector, vector.Is8Bit() ? kForce8Bit : kForce16Bit);
}

inline static String AttemptStaticStringCreation(const String& str) {
  if (!str.Is8Bit())
    return AttemptStaticStringCreation(str.Span16(), kForce16Bit);
  return AttemptStaticStringCreation(str.Span8());
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PARSER_IDIOMS_H_
