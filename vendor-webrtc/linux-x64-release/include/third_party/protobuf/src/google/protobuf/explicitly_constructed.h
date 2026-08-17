// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_EXPLICITLY_CONSTRUCTED_H__
#define GOOGLE_PROTOBUF_EXPLICITLY_CONSTRUCTED_H__

#include <stdint.h>

#include <string>
#include <utility>

// clang-format off
#include "google/protobuf/port_def.inc"
// clang-format on

namespace google {
namespace protobuf {
namespace internal {

// Wraps a variable whose constructor and destructor are explicitly
// called. It is particularly useful for a global variable, without its
// constructor and destructor run on start and end of the program lifetime.
// This circumvents the initial construction order fiasco, while keeping
// the address of the empty string a compile time constant.
//
// Pay special attention to the initialization state of the object.
// 1. The object is "uninitialized" to begin with.
// 2. Call Construct() or DefaultConstruct() only if the object is
//    uninitialized. After the call, the object becomes "initialized".
// 3. Call get() and get_mutable() only if the object is initialized.
// 4. Call Destruct() only if the object is initialized.
//    After the call, the object becomes uninitialized.
template <typename T, size_t min_align = 1>
class ExplicitlyConstructed {
 public:
  void DefaultConstruct() { new (&union_) T(); }

  template <typename... Args>
  void Construct(Args&&... args) {
    new (&union_) T(std::forward<Args>(args)...);
  }

  void Destruct() { get_mutable()->~T(); }

  constexpr const T& get() const { return reinterpret_cast<const T&>(union_); }
  T* get_mutable() { return reinterpret_cast<T*>(&union_); }

 private:
  union AlignedUnion {
    alignas(min_align > alignof(T) ? min_align
                                   : alignof(T)) char space[sizeof(T)];
    int64_t align_to_int64;
    void* align_to_ptr;
  } union_;
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_EXPLICITLY_CONSTRUCTED_H__
