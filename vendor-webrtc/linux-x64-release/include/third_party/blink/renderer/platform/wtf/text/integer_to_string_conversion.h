/*
 * Copyright (C) 2012 Apple Inc. All Rights Reserved.
 * Copyright (C) 2012 Patrick Gansterer <paroga@paroga.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_INTEGER_TO_STRING_CONVERSION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_INTEGER_TO_STRING_CONVERSION_H_

#include <array>
#include <limits>
#include <type_traits>

#include "base/check_op.h"
#include "base/containers/span.h"
#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace WTF {

// TODO(esprehn): See if we can generalize IntToStringT in
// base/strings/string_number_conversions.cc, and use unsigned type expansion
// optimization here instead of base::CheckedNumeric::UnsignedAbs().
template <typename IntegerType>
class IntegerToStringConverter {
  USING_FAST_MALLOC(IntegerToStringConverter);

 public:
  static_assert(std::is_integral<IntegerType>::value,
                "IntegerType must be a type of integer.");

  explicit IntegerToStringConverter(IntegerType input) {
    // We need to switch to the unsigned type when negating the value since
    // abs(INT_MIN) == INT_MAX + 1.
    bool is_negative = base::IsValueNegative(input);
    UnsignedIntegerType value = is_negative ? 0u - static_cast<UnsignedIntegerType>(input) : input;

    size_t pos = buffer_.size();
    do {
      --pos;
      buffer_[pos] = static_cast<LChar>((value % 10) + '0');
      value /= 10;
    } while (value);

    if (is_negative) {
      --pos;
      buffer_[pos] = static_cast<LChar>('-');
    }

    length_ = static_cast<unsigned>(buffer_.size() - pos);
  }

  base::span<const LChar> Span() const {
    return base::span(buffer_).last(length_);
  }

 private:
  using UnsignedIntegerType = typename std::make_unsigned<IntegerType>::type;
  static const size_t kBufferSize = 3 * sizeof(UnsignedIntegerType) +
                                    std::numeric_limits<IntegerType>::is_signed;

  std::array<LChar, kBufferSize> buffer_;
  unsigned length_;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_INTEGER_TO_STRING_CONVERSION_H_
