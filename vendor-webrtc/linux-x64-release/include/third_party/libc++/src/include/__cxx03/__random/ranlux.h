//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___CXX03___RANDOM_RANLUX_H
#define _LIBCPP___CXX03___RANDOM_RANLUX_H

#include <__cxx03/__config>
#include <__cxx03/__random/discard_block_engine.h>
#include <__cxx03/__random/subtract_with_carry_engine.h>
#include <__cxx03/cstdint>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

typedef subtract_with_carry_engine<uint_fast32_t, 24, 10, 24> ranlux24_base;
typedef subtract_with_carry_engine<uint_fast64_t, 48, 5, 12> ranlux48_base;

typedef discard_block_engine<ranlux24_base, 223, 23> ranlux24;
typedef discard_block_engine<ranlux48_base, 389, 11> ranlux48;

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___CXX03___RANDOM_RANLUX_H
