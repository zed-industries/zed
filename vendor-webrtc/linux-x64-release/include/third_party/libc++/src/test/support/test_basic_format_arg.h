//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include <concepts>
#include <format>
#include <utility>

#include "test_macros.h"

/// Returns whether the basic_format_arg contains a type T with the expected value.
template <class Context, class T>
bool test_basic_format_arg(std::basic_format_arg<Context> arg, T expected) {
  auto visitor = [expected](auto a) {
    if constexpr (std::same_as<decltype(a), T>)
      return a == expected;
    else
      return false;
  };
#if TEST_STD_VER >= 26 && defined(TEST_HAS_EXPLICIT_THIS_PARAMETER)
  return arg.visit(std::move(visitor));
#else
  return std::visit_format_arg(std::move(visitor), arg);
#endif
}
