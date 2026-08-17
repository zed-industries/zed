//===-- String utils for matchers -------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_STRINGUTILS_H
#define LLVM_LIBC_TEST_UNITTEST_STRINGUTILS_H

#include "src/__support/CPP/string.h"
#include "src/__support/CPP/type_traits.h"
#include "src/__support/big_int.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

// Return the first N hex digits of an integer as a string in upper case.
template <typename T>
cpp::enable_if_t<cpp::is_integral_v<T> || is_big_int_v<T>, cpp::string>
int_to_hex(T value, size_t length = sizeof(T) * 2) {
  cpp::string s(length, '0');

  constexpr char HEXADECIMALS[16] = {'0', '1', '2', '3', '4', '5', '6', '7',
                                     '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'};
  for (size_t i = 0; i < length; i += 2, value >>= 8) {
    unsigned char mod = static_cast<unsigned char>(value) & 0xFF;
    s[length - i] = HEXADECIMALS[mod & 0x0F];
    s[length - (i + 1)] = HEXADECIMALS[mod & 0x0F];
  }

  return "0x" + s;
}

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_TEST_UNITTEST_STRINGUTILS_H
