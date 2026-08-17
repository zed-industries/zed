/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_OPERATORS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_OPERATORS_H_

#include "base/numerics/checked_math.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_concatenate.h"

namespace WTF {

template <typename StringType1, typename StringType2>
class StringAppend final {
  STACK_ALLOCATED();

 public:
  StringAppend(StringType1 string1, StringType2 string2);

  operator String() const;
  operator AtomicString() const;

  size_t length() const;
  bool Is8Bit() const;

  void WriteTo(base::span<LChar> destination) const;
  void WriteTo(base::span<UChar> destination) const;

 private:
  const StringType1 string1_;
  const StringType2 string2_;
};

template <typename StringType1, typename StringType2>
StringAppend<StringType1, StringType2>::StringAppend(StringType1 string1,
                                                     StringType2 string2)
    : string1_(string1), string2_(string2) {}

template <typename StringType1, typename StringType2>
StringAppend<StringType1, StringType2>::operator String() const {
  const size_t computed_length = length();
  if (Is8Bit()) {
    base::span<LChar> buffer;
    scoped_refptr<StringImpl> result =
        StringImpl::CreateUninitialized(computed_length, buffer);
    WriteTo(buffer);
    return result;
  }
  base::span<UChar> buffer;
  scoped_refptr<StringImpl> result =
      StringImpl::CreateUninitialized(computed_length, buffer);
  WriteTo(buffer);
  return result;
}

template <typename StringType1, typename StringType2>
StringAppend<StringType1, StringType2>::operator AtomicString() const {
  return AtomicString(static_cast<String>(*this));
}

template <typename StringType1, typename StringType2>
bool StringAppend<StringType1, StringType2>::Is8Bit() const {
  StringTypeAdapter<StringType1> adapter1(string1_);
  StringTypeAdapter<StringType2> adapter2(string2_);
  return adapter1.Is8Bit() && adapter2.Is8Bit();
}

template <typename StringType1, typename StringType2>
void StringAppend<StringType1, StringType2>::WriteTo(
    base::span<LChar> destination) const {
  DCHECK(Is8Bit());
  StringTypeAdapter<StringType1> adapter1(string1_);
  StringTypeAdapter<StringType2> adapter2(string2_);
  // Assuming a left-heavy tree of StringAppend<>s, split using the length of
  // the right side of the expression.
  auto [part1, part2] =
      destination.split_at(destination.size() - adapter2.length());
  adapter1.WriteTo(part1);
  adapter2.WriteTo(part2);
}

template <typename StringType1, typename StringType2>
void StringAppend<StringType1, StringType2>::WriteTo(
    base::span<UChar> destination) const {
  StringTypeAdapter<StringType1> adapter1(string1_);
  StringTypeAdapter<StringType2> adapter2(string2_);
  // Assuming a left-heavy tree of StringAppend<>s, split using the length of
  // the right side of the expression.
  auto [part1, part2] =
      destination.split_at(destination.size() - adapter2.length());
  adapter1.WriteTo(part1);
  adapter2.WriteTo(part2);
}

template <typename StringType1, typename StringType2>
size_t StringAppend<StringType1, StringType2>::length() const {
  StringTypeAdapter<StringType1> adapter1(string1_);
  base::CheckedNumeric<size_t> total(adapter1.length());
  StringTypeAdapter<StringType2> adapter2(string2_);
  total += adapter2.length();
  return total.ValueOrDie();
}

template <typename StringType1, typename StringType2>
class StringTypeAdapter<StringAppend<StringType1, StringType2>> {
  STACK_ALLOCATED();

 public:
  explicit StringTypeAdapter(
      const StringAppend<StringType1, StringType2>& buffer)
      : buffer_(buffer) {}

  size_t length() const { return buffer_.length(); }
  bool Is8Bit() const { return buffer_.Is8Bit(); }

  void WriteTo(base::span<LChar> destination) const {
    buffer_.WriteTo(destination);
  }
  void WriteTo(base::span<UChar> destination) const {
    buffer_.WriteTo(destination);
  }

 private:
  const StringAppend<StringType1, StringType2>& buffer_;
};

inline StringAppend<const char*, String> operator+(const char* string1,
                                                   const String& string2) {
  return StringAppend<const char*, String>(string1, string2);
}

inline StringAppend<const char*, AtomicString> operator+(
    const char* string1,
    const AtomicString& string2) {
  return StringAppend<const char*, AtomicString>(string1, string2);
}

inline StringAppend<const char*, StringView> operator+(
    const char* string1,
    const StringView& string2) {
  return StringAppend<const char*, StringView>(string1, string2);
}

inline StringAppend<const UChar*, String> operator+(const UChar* string1,
                                                    const String& string2) {
  return StringAppend<const UChar*, String>(string1, string2);
}

inline StringAppend<const UChar*, AtomicString> operator+(
    const UChar* string1,
    const AtomicString& string2) {
  return StringAppend<const UChar*, AtomicString>(string1, string2);
}

inline StringAppend<const UChar*, StringView> operator+(
    const UChar* string1,
    const StringView& string2) {
  return StringAppend<const UChar*, StringView>(string1, string2);
}

template <typename T>
StringAppend<String, T> operator+(const String& string1, T string2) {
  return StringAppend<String, T>(string1, string2);
}

template <typename T>
StringAppend<AtomicString, T> operator+(const AtomicString& string1,
                                        T string2) {
  return StringAppend<AtomicString, T>(string1, string2);
}

template <typename T>
StringAppend<StringView, T> operator+(const StringView& string1, T string2) {
  return StringAppend<StringView, T>(string1, string2);
}

template <typename U, typename V, typename W>
StringAppend<StringAppend<U, V>, W> operator+(const StringAppend<U, V>& string1,
                                              W string2) {
  return StringAppend<StringAppend<U, V>, W>(string1, string2);
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_OPERATORS_H_
