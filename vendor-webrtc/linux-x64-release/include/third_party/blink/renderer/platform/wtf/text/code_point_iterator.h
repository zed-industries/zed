// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CODE_POINT_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CODE_POINT_ITERATOR_H_

#include <unicode/utf16.h>

#include "base/check_op.h"
#include "base/memory/stack_allocated.h"
#include "base/types/to_address.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace WTF {

//
// A code point iterator for 8-bits or 16-bits strings.
//
// This iterates 32-bit code points from 8-bits or 16-bits strings. In 8-bits
// strings, a code unit is 8-bits, and it's always a code point. In UTF16
// 16-bits strings, a code unit is 16-bits, and a code point is either a code
// unit (16-bits) or two code units (32-bits.)
//
// An instance must not outlive the target.
//
class CodePointIterator {
  STACK_ALLOCATED();

 public:
  CodePointIterator(bool is_8bit, const void* data, wtf_size_t len)
      : data_(data), data_length_(len), is_8bit_(is_8bit) {}

  // Create a `begin()` iterator.
  template <class T>
  explicit CodePointIterator(const T& string)
      : CodePointIterator(string.Is8Bit(), string.Bytes(), string.length()) {}

  // Create an `end()` iterator.
  template <class T>
  static CodePointIterator End(const T& string) {
    return CodePointIterator(
        string.Is8Bit(),
        string.Is8Bit()
            ? static_cast<const void*>(base::to_address(string.Span8().end()))
            : static_cast<const void*>(base::to_address(string.Span16().end())),
        0);
  }

  UChar32 operator*() const;
  void operator++();

  bool operator==(const CodePointIterator& other) const {
    DCHECK_EQ(is_8bit_, other.is_8bit_);
    return data_ == other.data_;
  }

  bool operator!=(const CodePointIterator& other) const {
    return !(*this == other);
  }

 private:
  const void* data_;
  wtf_size_t data_length_;
  // Caches the length of the current code point, in the number of code units.
  mutable wtf_size_t code_point_length_ = 0;
  bool is_8bit_;
};

inline UChar32 CodePointIterator::operator*() const {
  CHECK_GT(data_length_, 0u);
  if (is_8bit_) {
    return *static_cast<const uint8_t*>(data_);
  }
  // Get a code point, and cache its length to `code_point_length_`.
  UChar32 ch;
  code_point_length_ = 0;
  U16_NEXT(static_cast<const uint16_t*>(data_), code_point_length_,
           data_length_, ch);
  return ch;
}

inline void CodePointIterator::operator++() {
  CHECK_GT(data_length_, 0u);
  if (is_8bit_) {
    data_ = static_cast<const uint8_t*>(data_) + 1;
    return;
  }
  if (!code_point_length_) {
    // `code_point_length_` is cached by `operator*()`. If not, compute it.
    U16_FWD_1(static_cast<const uint16_t*>(data_), code_point_length_,
              data_length_);
  }
  data_ = static_cast<const uint16_t*>(data_) + code_point_length_;
  data_length_ -= code_point_length_;
  code_point_length_ = 0;
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CODE_POINT_ITERATOR_H_
