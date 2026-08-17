/*
    Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_SEGMENTED_STRING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_SEGMENTED_STRING_H_

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/unicode.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class SegmentedString;

class PLATFORM_EXPORT SegmentedSubstring {
  DISALLOW_NEW();

 public:
  SegmentedSubstring() { Clear(); }

  explicit SegmentedSubstring(const String& str) : string_(str) {
    unsigned len = str.length();
    if (len) {
      if (string_.Is8Bit()) {
        is_8bit_ = true;
        data_.string8_ptr = UNSAFE_TODO(string_.Characters8());
        // SAFETY: len is length of string and checked to be non-zero.
        data_last_char_ = UNSAFE_BUFFERS(data_.string8_ptr + len - 1);
      } else {
        is_8bit_ = false;
        data_.string16_ptr = UNSAFE_TODO(string_.Characters16());
        // SAFETY: len is length of string and checked to be non-zero.
        data_last_char_ = UNSAFE_BUFFERS(
            reinterpret_cast<const LChar*>(data_.string16_ptr + len - 1));
      }
    } else {
      is_8bit_ = true;
      data_.string8_ptr = nullptr;
      data_last_char_ = nullptr;
    }
    data_start_ = data_.string8_ptr;
  }

  void Clear() {
    is_8bit_ = true;
    data_.string8_ptr = nullptr;
    data_start_ = data_last_char_ = nullptr;
  }

  bool ExcludeLineNumbers() const { return !do_not_exclude_line_numbers_; }
  bool DoNotExcludeLineNumbers() const { return do_not_exclude_line_numbers_; }

  void SetExcludeLineNumbers() { do_not_exclude_line_numbers_ = false; }

  int NumberOfCharactersConsumed() const { return offset(); }

  void AppendTo(StringBuilder& builder) const {
    int off = offset();
    int len = length();

    if (!off) {
      if (len)
        builder.Append(string_);
    } else {
      builder.Append(string_.Substring(off, len));
    }
  }

  bool PushIfPossible(UChar c) {
    // This checks if either 8 or 16 bit strings are in the first character
    // or they are both nullptr (where we can't rewind).
    if (data_.string8_ptr == data_start_)
      return false;

    // SAFETY: if the string pointer is not at the start, then it points
    // into the string data and is safe to index one before it.

    if (is_8bit_) {
      if (*(UNSAFE_BUFFERS(data_.string8_ptr - 1)) != c) {
        return false;
      }

      UNSAFE_BUFFERS(--data_.string8_ptr);
    } else {
      if (*(UNSAFE_BUFFERS(data_.string16_ptr - 1)) != c) {
        return false;
      }

      UNSAFE_BUFFERS(--data_.string16_ptr);
    }

    return true;
  }

  ALWAYS_INLINE UChar GetCurrentChar() const {
    if (is_8bit_)
      return *data_.string8_ptr;
    return *data_.string16_ptr;
  }

  ALWAYS_INLINE bool CanAdvance() {
    return data_.string8_ptr < data_last_char_;
  }

  // Advances up to `delta` characters, returning how many characters were
  // advanced. This will not advance past the last character.
  unsigned Advance(unsigned delta) {
    DCHECK_NE(0, length());
    delta = std::min(static_cast<unsigned>(length()) - 1, delta);
    // Unsafe since a stronger check for non-zero length is required.
    if (is_8bit_)
      UNSAFE_TODO(data_.string8_ptr += delta);
    else
      UNSAFE_TODO(data_.string16_ptr += delta);
    return delta;
  }

  ALWAYS_INLINE UChar Advance() {
    return UNSAFE_TODO(is_8bit_ ? *++data_.string8_ptr : *++data_.string16_ptr);
  }

  StringView CurrentSubString(unsigned len) const {
    return StringView(string_, offset(), len);
  }

  ALWAYS_INLINE int offset() const {
    DCHECK_LE(data_start_, data_.string8_ptr);
    return static_cast<int>(data_.string8_ptr - data_start_) >> !is_8bit_;
  }

  ALWAYS_INLINE int length() const {
    DCHECK_LE(data_.string8_ptr, data_last_char_);
    return static_cast<int>(data_end() - data_.string8_ptr) >> !is_8bit_;
  }

 private:
  ALWAYS_INLINE const LChar* data_end() const {
    if (!data_last_char_)
      return nullptr;
    return UNSAFE_TODO(data_last_char_ + 1 + !is_8bit_);
  }

  union {
    // RAW_PTR_EXCLUSION: #union
    RAW_PTR_EXCLUSION const LChar* string8_ptr;
    RAW_PTR_EXCLUSION const UChar* string16_ptr;
  } data_;
  raw_ptr<const LChar, AllowPtrArithmetic | DanglingUntriaged> data_start_;
  // |data_last_char_| points to the last character (or nullptr). This is to
  // avoid extra computation in |CanAdvance|, which is in the critical path of
  // HTMLTokenizer.
  // RAW_PTR_EXCLUSION: End of the buffer already protected by raw_ptr.
  RAW_PTR_EXCLUSION const LChar* data_last_char_;
  bool do_not_exclude_line_numbers_ = true;
  bool is_8bit_ = true;
  String string_;
};

class PLATFORM_EXPORT SegmentedString {
  DISALLOW_NEW();

 public:
  SegmentedString()
      : number_of_characters_consumed_prior_to_current_string_(0),
        number_of_characters_consumed_prior_to_current_line_(0),
        current_line_(0),
        closed_(false),
        empty_(true),
        current_char_('\0') {}

  SegmentedString(const String& str)
      : current_string_(str),
        number_of_characters_consumed_prior_to_current_string_(0),
        number_of_characters_consumed_prior_to_current_line_(0),
        current_line_(0),
        closed_(false),
        empty_(!str.length()),
        current_char_(empty_ ? '\0' : current_string_.GetCurrentChar()) {}

  void Clear();
  void Close();

  void Append(const SegmentedString&);
  enum class PrependType {
    kNewInput = 0,
    kUnconsume = 1,
  };
  void Prepend(const SegmentedString&, PrependType);

  const SegmentedString* NextSegmentedString() const {
    return next_segmented_string_;
  }
  void SetNextSegmentedString(const SegmentedString* next) {
    next_segmented_string_ = next;
  }

  bool ExcludeLineNumbers() const {
    return current_string_.ExcludeLineNumbers();
  }
  void SetExcludeLineNumbers();

  void Push(UChar);

  bool IsEmpty() const { return empty_; }
  unsigned length() const;

  bool IsClosed() const { return closed_; }

  enum LookAheadResult {
    kDidNotMatch,
    kDidMatch,
    kNotEnoughCharacters,
  };

  LookAheadResult LookAhead(const String& string) {
    return LookAheadInline(string, kTextCaseSensitive);
  }
  LookAheadResult LookAheadIgnoringCase(const String& string) {
    return LookAheadInline(string, kTextCaseASCIIInsensitive);
  }

  // Used to advance by multiple characters. Specifically this advances by
  // `num_chars` and `num_lines`. This function advances without analyzing the
  // input string in anyway. As a result, the caller must know `num_lines` and
  // `current_column`.
  void Advance(unsigned num_chars, unsigned num_lines, int current_column);

  ALWAYS_INLINE UChar Advance() {
    if (current_string_.CanAdvance()) [[likely]] {
      current_char_ = current_string_.Advance();
      return current_char_;
    }
    return AdvanceSubstring();
  }

  ALWAYS_INLINE void UpdateLineNumber() {
    if (current_string_.DoNotExcludeLineNumbers()) [[likely]] {
      ++current_line_;
      // Plus 1 because numberOfCharactersConsumed value hasn't incremented yet;
      // it does with length() decrement below.
      number_of_characters_consumed_prior_to_current_line_ =
          NumberOfCharactersConsumed() + 1;
    }
  }

  ALWAYS_INLINE UChar AdvanceAndUpdateLineNumber() {
    DCHECK_GE(current_string_.length(), 1);
    if (current_char_ == '\n')
      UpdateLineNumber();
    return Advance();
  }

  ALWAYS_INLINE UChar AdvanceAndASSERT(UChar expected_character) {
    DCHECK_EQ(expected_character, CurrentChar());
    return Advance();
  }

  ALWAYS_INLINE UChar AdvanceAndASSERTIgnoringCase(UChar expected_character) {
    DCHECK_EQ(WTF::unicode::FoldCase(CurrentChar()),
              WTF::unicode::FoldCase(expected_character));
    return Advance();
  }

  ALWAYS_INLINE UChar AdvancePastNonNewline() {
    DCHECK_NE(CurrentChar(), '\n');
    return Advance();
  }

  ALWAYS_INLINE UChar AdvancePastNewlineAndUpdateLineNumber() {
    DCHECK_EQ(CurrentChar(), '\n');
    DCHECK_GE(current_string_.length(), 1);
    UpdateLineNumber();
    return Advance();
  }

  ALWAYS_INLINE int NumberOfCharactersConsumed() const {
    int number_of_pushed_characters = 0;
    return number_of_characters_consumed_prior_to_current_string_ +
           current_string_.NumberOfCharactersConsumed() -
           number_of_pushed_characters;
  }

  String ToString() const;

  ALWAYS_INLINE UChar CurrentChar() const { return current_char_; }

  // The method is moderately slow, comparing to currentLine method.
  OrdinalNumber CurrentColumn() const;
  OrdinalNumber CurrentLine() const;
  // Sets value of line/column variables. Column is specified indirectly by a
  // parameter columnAftreProlog which is a value of column that we should get
  // after a prolog (first prologLength characters) has been consumed.
  void SetCurrentPosition(OrdinalNumber line,
                          OrdinalNumber column_aftre_prolog,
                          int prolog_length);

 private:
  void Append(const SegmentedSubstring&);
  void Prepend(const SegmentedSubstring&, PrependType);

  UChar AdvanceSubstring();

  // Consume characters into `characters`, which should not be bigger than
  // `length()`.
  void AdvanceAndCollect(base::span<UChar> characters);

  inline LookAheadResult LookAheadInline(const String& string,
                                         TextCaseSensitivity case_sensitivity) {
    if (string.length() <= static_cast<unsigned>(current_string_.length())) {
      StringView current_substring =
          current_string_.CurrentSubString(string.length());
      if (string.StartsWith(current_substring, case_sensitivity))
        return kDidMatch;
      return kDidNotMatch;
    }
    return LookAheadSlowCase(string, case_sensitivity);
  }

  LookAheadResult LookAheadSlowCase(const String& string,
                                    TextCaseSensitivity case_sensitivity) {
    unsigned count = string.length();
    if (count > length())
      return kNotEnoughCharacters;
    base::span<UChar> consumed_characters;
    String consumed_string =
        String::CreateUninitialized(count, consumed_characters);
    AdvanceAndCollect(consumed_characters);
    LookAheadResult result = kDidNotMatch;
    if (consumed_string.StartsWith(string, case_sensitivity))
      result = kDidMatch;
    Prepend(SegmentedString(consumed_string), PrependType::kUnconsume);
    return result;
  }

  bool IsComposite() const { return !substrings_.empty(); }

  SegmentedSubstring current_string_;
  int number_of_characters_consumed_prior_to_current_string_;
  int number_of_characters_consumed_prior_to_current_line_;
  int current_line_;
  Deque<SegmentedSubstring> substrings_;
  bool closed_;
  bool empty_;
  UChar current_char_;
  raw_ptr<const SegmentedString> next_segmented_string_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_SEGMENTED_STRING_H_
