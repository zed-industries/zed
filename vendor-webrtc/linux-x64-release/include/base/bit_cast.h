// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BIT_CAST_H_
#define BASE_BIT_CAST_H_

#include <type_traits>

namespace base {

// This is an equivalent to C++20's std::bit_cast<>(), but with additional
// warnings. It morally does what `*reinterpret_cast<Dest*>(&source)` does, but
// the cast/deref pair is undefined behavior, while bit_cast<>() isn't.
//
// This is not a magic "get out of UB free" card. This must only be used on
// values, not on references or pointers. For pointers, use
// reinterpret_cast<>(), and then look at https://eel.is/c++draft/basic.lval#11
// as that's probably UB also.

template <class Dest, class Source>
constexpr Dest bit_cast(const Source& source) {
  static_assert(!std::is_pointer_v<Source>,
                "bit_cast must not be used on pointer types");
  static_assert(!std::is_pointer_v<Dest>,
                "bit_cast must not be used on pointer types");
  static_assert(!std::is_reference_v<Source>,
                "bit_cast must not be used on reference types");
  static_assert(!std::is_reference_v<Dest>,
                "bit_cast must not be used on reference types");
  static_assert(
      sizeof(Dest) == sizeof(Source),
      "bit_cast requires source and destination types to be the same size");
  static_assert(std::is_trivially_copyable_v<Source>,
                "bit_cast requires the source type to be trivially copyable");
  static_assert(
      std::is_trivially_copyable_v<Dest>,
      "bit_cast requires the destination type to be trivially copyable");

  return __builtin_bit_cast(Dest, source);
}

}  // namespace base

#endif  // BASE_BIT_CAST_H_
