// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_BIGINT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_BIGINT_H_

#include <optional>

#include "base/compiler_specific.h"
#include "third_party/abseil-cpp/absl/numeric/int128.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8-context.h"
#include "v8/include/v8-forward.h"
#include "v8/include/v8-local-handle.h"
#include "v8/include/v8-primitive.h"
#include "v8/include/v8-value.h"

namespace blink {

class ExceptionState;

class PLATFORM_EXPORT BigInt final {
 public:
  BigInt() = default;
  explicit BigInt(v8::Local<v8::BigInt> bigint) {
    int word_count = bigint->WordCount();
    words_.resize(word_count);
    bigint->ToWordsArray(&sign_bit_, &word_count, words_.data());
  }

  bool IsNegative() const { return sign_bit_ != 0; }
  bool FitsIn128Bits() const { return words_.size() <= 2; }

  v8::Local<v8::BigInt> ToV8(v8::Local<v8::Context> context) const {
    return v8::BigInt::NewFromWords(context, sign_bit_, words_.size(),
                                    words_.data())
        .ToLocalChecked();
  }

  // Will return nullopt if this is negative or will not fit in 128 bits.
  std::optional<absl::uint128> ToUInt128() const {
    if (IsNegative() || !FitsIn128Bits()) {
      return std::nullopt;
    }
    if (words_.size() == 0) {
      return 0;
    }
    if (words_.size() == 1) {
      return words_[0];
    }
    return absl::MakeUint128(words_[1], words_[0]);
  }

 private:
  Vector<uint64_t> words_;  // least significant at the front
  int sign_bit_ = 0;        // 0 for positive/zero, 1 for negative
};

// Convert a value to a BigInt. This can fail depending on the input type
// and value.
// https://webidl.spec.whatwg.org/#es-bigint
PLATFORM_EXPORT BigInt ToBigIntSlow(v8::Isolate*,
                                    v8::Local<v8::Value>,
                                    ExceptionState&);
inline BigInt ToBigInt(v8::Isolate* isolate,
                       v8::Local<v8::Value> value,
                       ExceptionState& exception_state) {
  // Fast case. The value is already a BigInt.
  if (value->IsBigInt()) [[likely]] {
    return BigInt(value.As<v8::BigInt>());
  }

  return ToBigIntSlow(isolate, value, exception_state);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_BIGINT_H_
