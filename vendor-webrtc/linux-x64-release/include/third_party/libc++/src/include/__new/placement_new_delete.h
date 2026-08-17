//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___NEW_PLACEMENT_NEW_DELETE_H
#define _LIBCPP___NEW_PLACEMENT_NEW_DELETE_H

#include <__config>
#include <__cstddef/size_t.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

#if defined(_LIBCPP_ABI_VCRUNTIME)
#  include <new.h>
#else
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX26 void*
operator new(std::size_t, void* __p) _NOEXCEPT {
  return __p;
}
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX26 void*
operator new[](std::size_t, void* __p) _NOEXCEPT {
  return __p;
}
inline _LIBCPP_HIDE_FROM_ABI void operator delete(void*, void*) _NOEXCEPT {}
inline _LIBCPP_HIDE_FROM_ABI void operator delete[](void*, void*) _NOEXCEPT {}
#endif

#endif // _LIBCPP___NEW_PLACEMENT_NEW_DELETE_H
