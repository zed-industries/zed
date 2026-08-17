// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_UNIQUE_TEMPORARY_BUFFER_H
#define _LIBCPP___MEMORY_UNIQUE_TEMPORARY_BUFFER_H

#include <__assert>
#include <__config>

#include <__cstddef/ptrdiff_t.h>
#include <__memory/allocator.h>
#include <__memory/unique_ptr.h>
#include <__new/allocate.h>
#include <__new/global_new_delete.h>
#include <__type_traits/is_constant_evaluated.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp>
struct __temporary_buffer_deleter {
  ptrdiff_t __count_; // ignored in non-constant evaluation

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR __temporary_buffer_deleter() _NOEXCEPT : __count_(0) {}
  _LIBCPP_HIDE_FROM_ABI
  _LIBCPP_CONSTEXPR explicit __temporary_buffer_deleter(ptrdiff_t __count) _NOEXCEPT : __count_(__count) {}

  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX23 void operator()(_Tp* __ptr) _NOEXCEPT {
    if (__libcpp_is_constant_evaluated()) {
      allocator<_Tp>().deallocate(__ptr, __count_);
      return;
    }

    std::__libcpp_deallocate_unsized<_Tp>(__ptr);
  }
};

template <class _Tp>
using __unique_temporary_buffer _LIBCPP_NODEBUG = unique_ptr<_Tp, __temporary_buffer_deleter<_Tp> >;

template <class _Tp>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_NO_CFI _LIBCPP_CONSTEXPR_SINCE_CXX23 __unique_temporary_buffer<_Tp>
__allocate_unique_temporary_buffer(ptrdiff_t __count) {
  using __deleter_type       = __temporary_buffer_deleter<_Tp>;
  using __unique_buffer_type = __unique_temporary_buffer<_Tp>;

  if (__libcpp_is_constant_evaluated()) {
    return __unique_buffer_type(allocator<_Tp>().allocate(__count), __deleter_type(__count));
  }

  _Tp* __ptr = nullptr;
  const ptrdiff_t __max_count =
      (~ptrdiff_t(0) ^ ptrdiff_t(ptrdiff_t(1) << (sizeof(ptrdiff_t) * __CHAR_BIT__ - 1))) / sizeof(_Tp);
  if (__count > __max_count)
    __count = __max_count;
  while (__count > 0) {
#if _LIBCPP_HAS_ALIGNED_ALLOCATION
    if (__is_overaligned_for_new(_LIBCPP_ALIGNOF(_Tp))) {
      align_val_t __al = align_val_t(_LIBCPP_ALIGNOF(_Tp));
      __ptr            = static_cast<_Tp*>(::operator new(__count * sizeof(_Tp), __al, nothrow));
    } else {
      __ptr = static_cast<_Tp*>(::operator new(__count * sizeof(_Tp), nothrow));
    }
#else
    if (__is_overaligned_for_new(_LIBCPP_ALIGNOF(_Tp))) {
      // Since aligned operator new is unavailable, constructs an empty buffer rather than one with invalid alignment.
      return __unique_buffer_type();
    }

    __ptr = static_cast<_Tp*>(::operator new(__count * sizeof(_Tp), nothrow));
#endif

    if (__ptr) {
      break;
    }
    __count /= 2;
  }

  return __unique_buffer_type(__ptr, __deleter_type(__count));
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___MEMORY_UNIQUE_TEMPORARY_BUFFER_H
