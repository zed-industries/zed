// Copyright 2024 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_UTIL_UNIQUE_PTR_WITH_BITSET_H
#define GRPC_SRC_CORE_UTIL_UNIQUE_PTR_WITH_BITSET_H

#include <memory>
#include <utility>

#include "absl/log/check.h"
#include "absl/numeric/bits.h"

namespace grpc_core {

// Like std::unique_ptr, but also includes a small bitset stored in the lower
// bits of the underlying T*.
template <typename T, size_t kBits>
class UniquePtrWithBitset {
 public:
  UniquePtrWithBitset() : p_(0) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  UniquePtrWithBitset(std::nullptr_t) : p_(0) {}
  explicit UniquePtrWithBitset(T* p) : p_(reinterpret_cast<uintptr_t>(p)) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  UniquePtrWithBitset(std::unique_ptr<T>&& p)
      : UniquePtrWithBitset(p.release()) {}
  ~UniquePtrWithBitset() {
    DCHECK_LE(kBits, static_cast<size_t>(absl::countr_zero(alignof(T))));
    delete get();
  }
  UniquePtrWithBitset(const UniquePtrWithBitset&) = delete;
  UniquePtrWithBitset& operator=(const UniquePtrWithBitset&) = delete;
  UniquePtrWithBitset(UniquePtrWithBitset&& other) noexcept
      : p_(std::exchange(other.p_, 0)) {}
  UniquePtrWithBitset& operator=(UniquePtrWithBitset&& other) noexcept {
    p_ = std::exchange(other.p_, 0);
    return *this;
  }

  T* get() const { return reinterpret_cast<T*>(p_ & ~kBitMask); }
  T* operator->() const { return get(); }
  T& operator*() const { return *get(); }
  explicit operator bool() const { return get() != nullptr; }
  void reset(T* p = nullptr) {
    uintptr_t bits = p_ & kBitMask;
    delete get();
    p_ = reinterpret_cast<uintptr_t>(p) | bits;
  }

  void SetBit(size_t bit) {
    DCHECK_LT(bit, kBits);
    p_ |= 1 << bit;
  }
  void ClearBit(size_t bit) {
    DCHECK_LT(bit, kBits);
    p_ &= ~(1 << bit);
  }
  bool TestBit(size_t bit) const {
    DCHECK_LT(bit, kBits);
    return p_ & (1 << bit);
  }

  friend bool operator==(const UniquePtrWithBitset& a,
                         const UniquePtrWithBitset& b) {
    return a.p_ == b.p_;
  }

 private:
  static constexpr uintptr_t kBitMask = (1 << kBits) - 1;
  uintptr_t p_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_UNIQUE_PTR_WITH_BITSET_H
