// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_IS_SUFFICIENTLY_ALIGNED_H
#define _LIBCPP___MEMORY_IS_SUFFICIENTLY_ALIGNED_H

#include <__config>
#include <__cstddef/size_t.h>
#include <cstdint>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

#if _LIBCPP_STD_VER >= 26

template <size_t _Alignment, class _Tp>
_LIBCPP_HIDE_FROM_ABI bool is_sufficiently_aligned(_Tp* __ptr) {
  return reinterpret_cast<uintptr_t>(__ptr) % _Alignment == 0;
}

#endif // _LIBCPP_STD_VER >= 26

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___MEMORY_IS_SUFFICIENTLY_ALIGNED_H
