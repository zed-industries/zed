// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_RAW_PTR_H__
#define GOOGLE_PROTOBUF_RAW_PTR_H__

#include <algorithm>

#include "absl/base/optimization.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

PROTOBUF_EXPORT ABSL_CACHELINE_ALIGNED extern const char
    kZeroBuffer[std::max(ABSL_CACHELINE_SIZE, 64)];

// This class is trivially copyable/trivially destructible and constexpr
// constructible. The class allows for storing a raw pointer to a non-trivial
// object in a constexpr context.
template <typename T>
class RawPtr {
 public:
  constexpr RawPtr() : RawPtr(kZeroBuffer) {
    static_assert(sizeof(T) <= sizeof(kZeroBuffer), "");
    static_assert(alignof(T) <= ABSL_CACHELINE_SIZE, "");
  }
  explicit constexpr RawPtr(const void* p) : p_(const_cast<void*>(p)) {}

  bool IsDefault() const { return p_ == kZeroBuffer; }
  void DeleteIfNotDefault() {
    if (!IsDefault()) delete Get();
  }
  void ClearIfNotDefault() {
    if (!IsDefault()) Get()->Clear();
  }

  void Set(const void* p) { p_ = const_cast<void*>(p); }
  T* Get() const { return reinterpret_cast<T*>(p_); }
  T* operator->() const { return Get(); }
  T& operator*() const { return *Get(); }

 private:
  void* p_;
};

constexpr void* DefaultRawPtr() {
  return const_cast<void*>(static_cast<const void*>(kZeroBuffer));
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_RAW_PTR_H__
