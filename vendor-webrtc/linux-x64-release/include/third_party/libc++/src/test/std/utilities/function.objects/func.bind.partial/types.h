//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_UTILITIES_FUNCTION_OBJECTS_FUNC_BIND_PARTIAL_TYPES_H
#define TEST_STD_UTILITIES_FUNCTION_OBJECTS_FUNC_BIND_PARTIAL_TYPES_H

#include <tuple>
#include <utility>

struct MakeTuple {
  template <class... Args>
  constexpr auto operator()(Args&&... args) const {
    return std::make_tuple(std::forward<Args>(args)...);
  }
};

template <int X>
struct Elem {
  template <int Y>
  constexpr bool operator==(const Elem<Y>&) const {
    return X == Y;
  }
};

struct CopyMoveInfo {
  enum { none, copy, move } copy_kind;

  constexpr CopyMoveInfo() : copy_kind(none) {}
  constexpr CopyMoveInfo(const CopyMoveInfo&) : copy_kind(copy) {}
  constexpr CopyMoveInfo(CopyMoveInfo&&) : copy_kind(move) {}
};

template <class T>
T do_nothing(T t) {
  return t;
}

#endif // TEST_STD_UTILITIES_FUNCTION_OBJECTS_FUNC_BIND_PARTIAL_TYPES_H
