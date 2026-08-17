// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FUZZED_DATA_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FUZZED_DATA_PROVIDER_H_

#include <fuzzer/FuzzedDataProvider.h>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// This class simply wraps FuzzedDataProvider and vends Blink friendly types.
class FuzzedDataProvider {
  DISALLOW_NEW();

 public:
  FuzzedDataProvider(const uint8_t* bytes, size_t num_bytes);
  FuzzedDataProvider(const FuzzedDataProvider&) = delete;
  FuzzedDataProvider& operator=(const FuzzedDataProvider&) = delete;

  // Returns a string with length between 0 and max_length.
  String ConsumeRandomLengthString(size_t max_length);

  // Returns a String containing all remaining bytes of the input data.
  std::string ConsumeRemainingBytes();

  // Returns a bool, or false when no data remains.
  bool ConsumeBool() { return provider_.ConsumeBool(); }

  // Returns an enum value. The enum must start at 0 and be contiguous. It must
  // also contain |kMaxValue| aliased to its largest (inclusive) value. Such as:
  // enum class Foo { SomeValue, OtherValue, kMaxValue = OtherValue };
  template <typename T>
  T ConsumeEnum() {
    return provider_.ConsumeEnum<T>();
  }

  // Returns a number in the range [min, max] by consuming bytes from the input
  // data. The value might not be uniformly distributed in the given range. If
  // there's no input data left, always returns |min|. |min| must be less than
  // or equal to |max|.
  template <typename T>
  T ConsumeIntegralInRange(T min, T max) {
    return provider_.ConsumeIntegralInRange<T>(min, max);
  }

  // Returns a number in the range [Type's min, Type's max]. The value might
  // not be uniformly distributed in the given range. If there's no input data
  // left, always returns |min|.
  template <typename T>
  T ConsumeIntegral() {
    return provider_.ConsumeIntegral<T>();
  }

  // Returns a value from |array|, consuming as many bytes as needed to do so.
  // |array| must be a fixed-size array.
  template <typename T, size_t size>
  T PickValueInArray(T (&array)[size]) {
    return array[provider_.ConsumeIntegralInRange<size_t>(0, size - 1)];
  }

  // Reports the remaining bytes available for fuzzed input.
  size_t RemainingBytes() { return provider_.remaining_bytes(); }

 private:
  ::FuzzedDataProvider provider_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FUZZED_DATA_PROVIDER_H_
