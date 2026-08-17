// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_TEMPORARY_BUFFER_H
#define _LIBCPP___MEMORY_TEMPORARY_BUFFER_H

#include <__config>
#include <__cstddef/ptrdiff_t.h>
#include <__memory/unique_temporary_buffer.h>
#include <__utility/pair.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

#if _LIBCPP_STD_VER <= 17 || defined(_LIBCPP_ENABLE_CXX20_REMOVED_TEMPORARY_BUFFER)

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp>
[[__nodiscard__]] _LIBCPP_HIDE_FROM_ABI _LIBCPP_NO_CFI _LIBCPP_DEPRECATED_IN_CXX17 pair<_Tp*, ptrdiff_t>
get_temporary_buffer(ptrdiff_t __n) _NOEXCEPT {
  __unique_temporary_buffer<_Tp> __unique_buf = std::__allocate_unique_temporary_buffer<_Tp>(__n);
  pair<_Tp*, ptrdiff_t> __result(__unique_buf.get(), __unique_buf.get_deleter().__count_);
  __unique_buf.release();
  return __result;
}

template <class _Tp>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_DEPRECATED_IN_CXX17 void return_temporary_buffer(_Tp* __p) _NOEXCEPT {
  __unique_temporary_buffer<_Tp> __unique_buf(__p);
  (void)__unique_buf;
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER <= 17 || defined(_LIBCPP_ENABLE_CXX20_REMOVED_TEMPORARY_BUFFER)

#endif // _LIBCPP___MEMORY_TEMPORARY_BUFFER_H
