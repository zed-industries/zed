//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_FLOATING_POINT_HELPERS_H
#define TEST_SUPPORT_FLOATING_POINT_HELPERS_H

#include <limits>

#include "test_macros.h"

template <class T>
TEST_CONSTEXPR_CXX20 bool is_close(T v, T comp) {
  return v <= comp + std::numeric_limits<T>::epsilon() && v >= comp - std::numeric_limits<T>::epsilon();
}

#endif // TEST_SUPPORT_FLOATING_POINT_HELPERS_H
