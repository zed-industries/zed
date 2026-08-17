//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___UTILITY_ELEMENT_COUNT_H
#define _LIBCPP___UTILITY_ELEMENT_COUNT_H

#include <__config>
#include <__cstddef/size_t.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// Type used to encode that a function takes an integer that represents a number
// of elements as opposed to a number of bytes.
enum class __element_count : size_t {};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___UTILITY_ELEMENT_COUNT_H
