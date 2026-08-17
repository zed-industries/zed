/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 * Copyright (C) 2009 Torch Mobile, Inc. http://www.torchmobile.com/
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_MARKUP_TOKENIZER_INLINES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_MARKUP_TOKENIZER_INLINES_H_

#include "base/check_op.h"
#include "base/notreached.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/text/segmented_string.h"

namespace blink {

inline bool IsTokenizerWhitespace(UChar cc) {
  return cc == ' ' || cc == '\x0A' || cc == '\x09' || cc == '\x0C';
}

inline void AdvanceStringAndASSERTIgnoringCase(
    SegmentedString& source,
    const char* expected_characters) {
  while (*expected_characters)
    source.AdvanceAndASSERTIgnoringCase(UNSAFE_TODO(*expected_characters++));
}

inline void AdvanceStringAndASSERT(SegmentedString& source,
                                   const char* expected_characters) {
  while (*expected_characters)
    source.AdvanceAndASSERT(UNSAFE_TODO(*expected_characters++));
}

#if defined(COMPILER_MSVC)
// We need to disable the "unreachable code" warning because we want to assert
// that some code points aren't reached in the state machine.
#pragma warning(disable : 4702)
#endif

#define BEGIN_STATE(prefix, stateName) \
  case prefix::stateName:              \
  stateName:
#define BEGIN_STATE_NOLABEL(prefix, stateName) case prefix::stateName:
#define END_STATE() NOTREACHED();

// We use this macro when the HTML5 spec says "reconsume the current input
// character in the <mumble> state."
#define RECONSUME_IN(prefix, stateName)   \
  do {                                    \
    DCHECK_NE(state_, prefix::stateName); \
    state_ = prefix::stateName;           \
    goto stateName;                       \
  } while (false)

// We use this macro when the HTML5 spec says "consume the next input
// character ... and switch to the <mumble> state."
#define ADVANCE_TO(prefix, stateName)                    \
  do {                                                   \
    DCHECK_NE(state_, prefix::stateName);                \
    state_ = prefix::stateName;                          \
    if (!input_stream_preprocessor_.Advance(source, cc)) \
      return HaveBufferedCharacterToken();               \
    goto stateName;                                      \
  } while (false)

// Similar to ADVANCE_TO, but we use this macro when the next input character is
// known not to be a newline character. |AdvancePastNonNewline| will DCHECK this
// property.
#define ADVANCE_PAST_NON_NEWLINE_TO(prefix, stateName)                 \
  do {                                                                 \
    DCHECK_NE(state_, prefix::stateName);                              \
    state_ = prefix::stateName;                                        \
    if (!input_stream_preprocessor_.AdvancePastNonNewline(source, cc)) \
      return HaveBufferedCharacterToken();                             \
    goto stateName;                                                    \
  } while (false)

// We use this macro when the HTML5 spec says "consume the next input
// character" and it doesn't say "switch to ... state".
#define CONSUME(prefix, stateName)                       \
  do {                                                   \
    DCHECK_EQ(state_, prefix::stateName);                \
    if (!input_stream_preprocessor_.Advance(source, cc)) \
      return HaveBufferedCharacterToken();               \
    goto stateName;                                      \
  } while (false)

// Similar to CONSUME, but we use this macro when the next input character is
// known not to be a newline character. |AdvancePastNonNewline| will DCHECK this
// property.
#define CONSUME_NON_NEWLINE(prefix, stateName)                         \
  do {                                                                 \
    DCHECK_EQ(state_, prefix::stateName);                              \
    if (!input_stream_preprocessor_.AdvancePastNonNewline(source, cc)) \
      return HaveBufferedCharacterToken();                             \
    goto stateName;                                                    \
  } while (false)

// Sometimes there's more complicated logic in the spec that separates when
// we consume the next input character and when we switch to a particular
// state. We handle those cases by advancing the source directly and using
// this macro to switch to the indicated state.
#define SWITCH_TO(prefix, stateName)                                      \
  do {                                                                    \
    DCHECK_NE(state_, prefix::stateName);                                 \
    state_ = prefix::stateName;                                           \
    if (source.IsEmpty() || !input_stream_preprocessor_.Peek(source, cc)) \
      return HaveBufferedCharacterToken();                                \
    goto stateName;                                                       \
  } while (false)

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_MARKUP_TOKENIZER_INLINES_H_
