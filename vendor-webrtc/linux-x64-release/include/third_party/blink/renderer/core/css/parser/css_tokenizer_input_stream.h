// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_TOKENIZER_INPUT_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_TOKENIZER_INPUT_STREAM_H_

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSTokenizerInputStream {
 public:
  explicit CSSTokenizerInputStream(StringView input)
      : string_length_(input.length()), string_(input) {}

  CSSTokenizerInputStream(const CSSTokenizerInputStream&) = delete;
  CSSTokenizerInputStream& operator=(const CSSTokenizerInputStream&) = delete;

  // Gets the char in the stream replacing NUL characters with a unicode
  // replacement character. Will return (NUL) kEndOfFileMarker when at the
  // end of the stream.
  UChar NextInputChar() const {
    if (offset_ >= string_length_) {
      return '\0';
    }
    UChar result = string_[offset_];
    return result ? result : 0xFFFD;
  }

  // Gets the char at lookaheadOffset from the current stream position. Will
  // return NUL (kEndOfFileMarker) if the stream position is at the end.
  // NOTE: This may *also* return NUL if there's one in the input! Never
  // compare the return value to '\0'.
  UChar PeekWithoutReplacement(unsigned lookahead_offset) const {
    if ((offset_ + lookahead_offset) >= string_length_) {
      return '\0';
    }
    return string_[offset_ + lookahead_offset];
  }
  StringView Peek() const {
    return StringView(string_, offset_, length() - offset_);
  }

  void Advance(unsigned offset = 1) { offset_ += offset; }
  void PushBack(UChar cc) {
    --offset_;
    DCHECK(NextInputChar() == cc);
  }

  double GetDouble(unsigned start, unsigned end) const;

  // Like GetDouble(), but only for the case where the number matches
  // [0-9]+ (no decimal point, no exponent, no sign), and is faster.
  double GetNaturalNumberAsDouble(unsigned start, unsigned end) const;

  template <bool characterPredicate(UChar)>
  unsigned SkipWhilePredicate(unsigned offset) {
    if (string_.Is8Bit()) {
      const LChar* characters8 = UNSAFE_TODO(string_.Characters8());
      while ((offset_ + offset) < string_length_ &&
             characterPredicate(UNSAFE_TODO(characters8[offset_ + offset]))) {
        ++offset;
      }
    } else {
      const UChar* characters16 = UNSAFE_TODO(string_.Characters16());
      while ((offset_ + offset) < string_length_ &&
             characterPredicate(UNSAFE_TODO(characters16[offset_ + offset]))) {
        ++offset;
      }
    }
    return offset;
  }

  void AdvanceUntilNonWhitespace();

  unsigned length() const { return string_length_; }
  unsigned Offset() const { return std::min(offset_, string_length_); }

  StringView RangeFrom(unsigned start) const {
    return StringView(string_, start, string_length_ - start);
  }

  StringView RangeAt(unsigned start, unsigned length) const {
    DCHECK(start + length <= string_length_);
    return StringView(string_, start, length);
  }

  void Restore(wtf_size_t offset) { offset_ = offset; }

 private:
  wtf_size_t offset_ = 0;
  const wtf_size_t string_length_;
  StringView string_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_TOKENIZER_INPUT_STREAM_H_
