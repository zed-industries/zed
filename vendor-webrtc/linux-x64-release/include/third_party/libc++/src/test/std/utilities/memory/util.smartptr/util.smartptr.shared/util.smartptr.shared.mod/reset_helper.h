//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
#ifndef TEST_STD_SHARED_PTR_RESET_H
#define TEST_STD_SHARED_PTR_RESET_H

#include <memory>
#include <type_traits>

template <class T, class... Args>
std::false_type test_has_reset(...);

template <class T, class... Args>
typename std::enable_if<std::is_same<decltype(std::declval<T>().reset(std::declval<Args>()...)), void>::value,
                        std::true_type>::type
test_has_reset(int);

template <class T, class... Args>
using HasReset = decltype(test_has_reset<T, Args...>(0));

#endif // TEST_STD_SHARED_PTR_RESET_H
