/*
 * Copyright (C) 2011 Daniel Bates (dbates@intudata.com). All Rights Reserved.
 * Copyright (c) 2012 Google, inc.  All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of Google Inc. nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_DECODE_ESCAPE_SEQUENCES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_DECODE_ESCAPE_SEQUENCES_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/ascii_ctype.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

// See
// <http://en.wikipedia.org/wiki/Percent-encoding#Non-standard_implementations>.
struct Unicode16BitEscapeSequence {
  STATIC_ONLY(Unicode16BitEscapeSequence);
  enum { kSequenceSize = 6 };  // e.g. %u26C4
  static wtf_size_t FindInString(const String& string,
                                 wtf_size_t start_position) {
    return string.Find("%u", start_position);
  }
  static wtf_size_t FindEndOfRun(const String& string,
                                 wtf_size_t start_position,
                                 wtf_size_t end_position) {
    wtf_size_t run_end = start_position;
    while (end_position - run_end >= kSequenceSize && string[run_end] == '%' &&
           string[run_end + 1] == 'u' && IsASCIIHexDigit(string[run_end + 2]) &&
           IsASCIIHexDigit(string[run_end + 3]) &&
           IsASCIIHexDigit(string[run_end + 4]) &&
           IsASCIIHexDigit(string[run_end + 5])) {
      run_end += kSequenceSize;
    }
    return run_end;
  }

  template <typename CharType>
  static String DecodeRun(const CharType* run,
                          wtf_size_t run_length,
                          const WTF::TextEncoding&) {
    // Each %u-escape sequence represents a UTF-16 code unit.  See
    // <http://www.w3.org/International/iri-edit/draft-duerst-iri.html#anchor29>.
    // For 16-bit escape sequences, we know that findEndOfRun() has given us a
    // contiguous run of sequences without any intervening characters, so decode
    // the run without additional checks.
    wtf_size_t number_of_sequences = run_length / kSequenceSize;
    StringBuilder builder;
    builder.reserve(number_of_sequences);
    while (number_of_sequences--) {
      UChar code_unit =
          (ToASCIIHexValue(run[2]) << 12) | (ToASCIIHexValue(run[3]) << 8) |
          (ToASCIIHexValue(run[4]) << 4) | ToASCIIHexValue(run[5]);
      builder.Append(code_unit);
      run += kSequenceSize;
    }
    return builder.ToString();
  }
};

struct URLEscapeSequence {
  enum { kSequenceSize = 3 };  // e.g. %41
  static wtf_size_t FindInString(const String& string,
                                 wtf_size_t start_position) {
    return string.find('%', start_position);
  }
  static wtf_size_t FindEndOfRun(const String& string,
                                 wtf_size_t start_position,
                                 wtf_size_t end_position) {
    // Make the simplifying assumption that supported encodings may have up to
    // two unescaped characters in the range 0x40 - 0x7F as the trailing bytes
    // of their sequences which need to be passed into the decoder as part of
    // the run. In other words, we end the run at the first value outside of the
    // 0x40 - 0x7F range, after two values in this range, or at a %-sign that
    // does not introduce a valid escape sequence.
    wtf_size_t run_end = start_position;
    wtf_size_t number_of_trailing_characters = 0;
    while (run_end < end_position) {
      if (string[run_end] == '%') {
        if (end_position - run_end >= kSequenceSize &&
            IsASCIIHexDigit(string[run_end + 1]) &&
            IsASCIIHexDigit(string[run_end + 2])) {
          run_end += kSequenceSize;
          number_of_trailing_characters = 0;
        } else
          break;
      } else if (string[run_end] >= 0x40 && string[run_end] <= 0x7F &&
                 number_of_trailing_characters < 2) {
        run_end += 1;
        number_of_trailing_characters += 1;
      } else
        break;
    }
    return run_end;
  }

  template <typename CharType>
  static String DecodeRun(const CharType* run,
                          wtf_size_t run_length,
                          const WTF::TextEncoding& encoding) {
    // For URL escape sequences, we know that findEndOfRun() has given us a run
    // where every %-sign introduces a valid escape sequence, but there may be
    // characters between the sequences.
    Vector<char, 512> buffer;
    buffer.resize(
        run_length);  // Unescaping hex sequences only makes the length smaller.
    char* p = buffer.data();
    const CharType* run_end = run + run_length;
    while (run < run_end) {
      if (run[0] == '%') {
        *p++ = (ToASCIIHexValue(run[1]) << 4) | ToASCIIHexValue(run[2]);
        run += kSequenceSize;
      } else {
        *p++ = run[0];
        run += 1;
      }
    }
    DCHECK_GE(
        buffer.size(),
        static_cast<size_t>(p - buffer.data()));  // Prove buffer not overrun.
    return (encoding.IsValid() ? encoding : UTF8Encoding())
        .Decode(buffer.data(), static_cast<wtf_size_t>(p - buffer.data()));
  }
};

template <typename EscapeSequence>
String DecodeEscapeSequences(const String& string,
                             const WTF::TextEncoding& encoding) {
  StringBuilder result;
  wtf_size_t length = string.length();
  wtf_size_t decoded_position = 0;
  wtf_size_t search_position = 0;
  wtf_size_t encoded_run_position;
  while ((encoded_run_position = EscapeSequence::FindInString(
              string, search_position)) != kNotFound) {
    wtf_size_t encoded_run_end =
        EscapeSequence::FindEndOfRun(string, encoded_run_position, length);
    search_position = encoded_run_end;
    if (encoded_run_end == encoded_run_position) {
      ++search_position;
      continue;
    }

    String decoded =
        string.Is8Bit() ? EscapeSequence::DecodeRun(
                              string.Characters8() + encoded_run_position,
                              encoded_run_end - encoded_run_position, encoding)
                        : EscapeSequence::DecodeRun(
                              string.Characters16() + encoded_run_position,
                              encoded_run_end - encoded_run_position, encoding);

    if (decoded.IsEmpty())
      continue;

    result.Append(string, decoded_position,
                  encoded_run_position - decoded_position);
    result.Append(decoded);
    decoded_position = encoded_run_end;
  }
  result.Append(string, decoded_position, length - decoded_position);
  return result.ToString();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_DECODE_ESCAPE_SEQUENCES_H_
