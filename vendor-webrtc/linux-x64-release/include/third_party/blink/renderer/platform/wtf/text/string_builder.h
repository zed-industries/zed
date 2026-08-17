/*
 * Copyright (C) 2009, 2010, 2012, 2013 Apple Inc. All rights reserved.
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_BUILDER_H_

#include <unicode/utf16.h>

#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/integer_to_string_conversion.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

class WTF_EXPORT StringBuilder {
  USING_FAST_MALLOC(StringBuilder);

 public:
  StringBuilder() : no_buffer_() {}
  StringBuilder(const StringBuilder&) = delete;
  StringBuilder& operator=(const StringBuilder&) = delete;
  ~StringBuilder() { ClearBuffer(); }

  bool DoesAppendCauseOverflow(unsigned length) const;

  void Append(base::span<const UChar> chars);
  void Append(base::span<const LChar> chars);

  void Append(const StringBuilder& other) {
    if (!other.length_)
      return;

    if (!length_ && !HasBuffer() && !other.string_.IsNull()) {
      string_ = other.string_;
      length_ = other.string_.length();
      is_8bit_ = other.string_.Is8Bit();
      return;
    }

    if (other.Is8Bit())
      Append(other.Span8());
    else
      Append(other.Span16());
  }

  // NOTE: The semantics of this are different than StringView(..., offset,
  // length) in that an invalid offset or invalid length is a no-op instead of
  // an error.
  // TODO(esprehn): We should probably unify the semantics instead.
  void Append(const StringView& string, unsigned offset, unsigned length) {
    unsigned extent = offset + length;
    if (extent < offset || extent > string.length())
      return;

    // We can't do this before the above check since StringView's constructor
    // doesn't accept invalid offsets or lengths.
    Append(StringView(string, offset, length));
  }

  void Append(const StringView& string) {
    if (string.empty())
      return;

    // If we're appending to an empty builder, and there is not a buffer
    // (reserveCapacity has not been called), then share the impl if
    // possible.
    //
    // This is important to avoid string copies inside dom operations like
    // Node::textContent when there's only a single Text node child, or
    // inside the parser in the common case when flushing buffered text to
    // a Text node.
    StringImpl* impl = string.SharedImpl();
    if (!length_ && !HasBuffer() && impl) {
      string_ = impl;
      length_ = impl->length();
      is_8bit_ = impl->Is8Bit();
      return;
    }

    if (string.Is8Bit())
      Append(string.Span8());
    else
      Append(string.Span16());
  }

  void Append(UChar c) {
    if (is_8bit_ && c <= 0xFF) {
      Append(static_cast<LChar>(c));
      return;
    }
    EnsureBuffer16(1);
    buffer16_.push_back(c);
    ++length_;
  }

  void Append(LChar c) {
    if (!is_8bit_) {
      Append(static_cast<UChar>(c));
      return;
    }
    EnsureBuffer8(1);
    buffer8_.push_back(c);
    ++length_;
  }

  void Append(char c) { Append(static_cast<LChar>(c)); }

  void Append(UChar32 c) {
    if (U_IS_BMP(c)) {
      Append(static_cast<UChar>(c));
      return;
    }
    Append(U16_LEAD(c));
    Append(U16_TRAIL(c));
  }

  template <typename IntegerType>
  void AppendNumber(IntegerType number) {
    IntegerToStringConverter<IntegerType> converter(number);
    Append(converter.Span());
  }

  void AppendNumber(bool);

  void AppendNumber(float);

  void AppendNumber(double, unsigned precision = 6);

  // Like WTF::String::Format, supports Latin-1 only.
  PRINTF_FORMAT(2, 3)
  void AppendFormat(const char* format, ...);

  void erase(unsigned);

  // ReleaseString is similar to ToString but releases the string_ object
  // to the caller, preventing refcount trashing. Prefer it over ToString()
  // if the StringBuilder is going to be destroyed or cleared afterwards.
  String ReleaseString();
  String ToString();
  AtomicString ToAtomicString();
  String Substring(unsigned start, unsigned length) const;
  StringView SubstringView(unsigned start, unsigned length) const;

  operator StringView() const {
    if (Is8Bit()) {
      return StringView(Span8());
    } else {
      return StringView(Span16());
    }
  }

  unsigned length() const { return length_; }
  bool empty() const { return !length_; }

  unsigned Capacity() const;
  // Increase the capacity of the backing buffer to at least |new_capacity|. The
  // behavior is the same as |Vector::ReserveCapacity|:
  // * Increase the capacity even when there are existing characters or a
  //   capacity.
  // * The characters in the backing buffer are not affected.
  // * This function does not shrink the size of the backing buffer, even if
  //   |new_capacity| is small.
  // * This function may cause a reallocation.
  void ReserveCapacity(unsigned new_capacity);
  // This is analogous to |Ensure16Bit| and |ReserveCapacity|, but can avoid
  // double reallocations when the current buffer is 8 bits and is smaller than
  // |new_capacity|.
  void Reserve16BitCapacity(unsigned new_capacity);

  // TODO(esprehn): Rename to shrink().
  void Resize(unsigned new_size);

  UChar operator[](unsigned i) const {
    if (is_8bit_)
      return Span8()[i];
    return Span16()[i];
  }

  base::span<const LChar> Span8() const {
    DCHECK(is_8bit_);
    if (!length()) {
      return {};
    }
    if (!string_.IsNull()) {
      return string_.Span8();
    }
    DCHECK(has_buffer_);
    return base::span(buffer8_).first(length());
  }

  base::span<const UChar> Span16() const {
    DCHECK(!is_8bit_);
    if (!length()) {
      return {};
    }
    if (!string_.IsNull()) {
      return string_.Span16();
    }
    DCHECK(has_buffer_);
    return base::span(buffer16_).first(length());
  }

  bool Is8Bit() const { return is_8bit_; }
  void Ensure16Bit();

  void Clear();
  void Swap(StringBuilder&);

 private:
  static const unsigned kInlineBufferSize = 256;
  static unsigned InitialBufferSize() { return kInlineBufferSize; }

  typedef Vector<LChar, kInlineBufferSize / sizeof(LChar)> Buffer8;
  typedef Vector<UChar, kInlineBufferSize / sizeof(UChar)> Buffer16;

  void EnsureBuffer8(unsigned added_size) {
    DCHECK(is_8bit_);
    if (!HasBuffer())
      CreateBuffer8(added_size);
  }

  void EnsureBuffer16(unsigned added_size) {
    if (is_8bit_ || !HasBuffer())
      CreateBuffer16(added_size);
  }

  void CreateBuffer8(unsigned added_size);
  void CreateBuffer16(unsigned added_size);
  void ClearBuffer();
  bool HasBuffer() const { return has_buffer_; }

  template <typename StringType>
  void BuildString() {
    if (is_8bit_)
      string_ = StringType(Span8());
    else
      string_ = StringType(Span16());
    ClearBuffer();
  }

  String string_;
  union {
    char no_buffer_;
    Buffer8 buffer8_;
    Buffer16 buffer16_;
  };
  unsigned length_ = 0;
  bool is_8bit_ = true;
  bool has_buffer_ = false;
};

template <typename StringType>
bool Equal(const StringBuilder& a, const StringType& b) {
  if (a.length() != b.length())
    return false;

  if (!a.length())
    return true;

  if (a.Is8Bit()) {
    if (b.Is8Bit())
      return a.Span8() == b.Span8();
    return a.Span8() == b.Span16();
  }

  if (b.Is8Bit())
    return a.Span16() == b.Span8();
  return a.Span16() == b.Span16();
}

inline bool operator==(const StringBuilder& a, const StringBuilder& b) {
  return Equal(a, b);
}
inline bool operator!=(const StringBuilder& a, const StringBuilder& b) {
  return !Equal(a, b);
}
inline bool operator==(const StringBuilder& a, const String& b) {
  return Equal(a, b);
}
inline bool operator!=(const StringBuilder& a, const String& b) {
  return !Equal(a, b);
}
inline bool operator==(const String& a, const StringBuilder& b) {
  return Equal(b, a);
}
inline bool operator!=(const String& a, const StringBuilder& b) {
  return !Equal(b, a);
}

}  // namespace WTF

using WTF::StringBuilder;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_BUILDER_H_
