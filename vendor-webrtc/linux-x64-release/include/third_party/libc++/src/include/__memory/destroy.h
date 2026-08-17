//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_DESTROY_H
#define _LIBCPP___MEMORY_DESTROY_H

#include <__config>
#include <__memory/addressof.h>
#include <__memory/allocator_traits.h>
#include <__memory/construct_at.h>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _ForwardIterator>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _ForwardIterator
__destroy(_ForwardIterator __first, _ForwardIterator __last) {
  for (; __first != __last; ++__first)
    std::__destroy_at(std::addressof(*__first));
  return __first;
}

template <class _BidirectionalIterator>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _BidirectionalIterator
__reverse_destroy(_BidirectionalIterator __first, _BidirectionalIterator __last) {
  while (__last != __first) {
    --__last;
    std::__destroy_at(std::addressof(*__last));
  }
  return __last;
}

// Destroy all elements in [__first, __last) from left to right using allocator destruction.
template <class _Alloc, class _Iter, class _Sent>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void
__allocator_destroy(_Alloc& __alloc, _Iter __first, _Sent __last) {
  for (; __first != __last; ++__first)
    allocator_traits<_Alloc>::destroy(__alloc, std::addressof(*__first));
}

#if _LIBCPP_STD_VER >= 17
template <class _ForwardIterator>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 void destroy(_ForwardIterator __first, _ForwardIterator __last) {
  (void)std::__destroy(std::move(__first), std::move(__last));
}

template <class _ForwardIterator, class _Size>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _ForwardIterator destroy_n(_ForwardIterator __first, _Size __n) {
  for (; __n > 0; (void)++__first, --__n)
    std::__destroy_at(std::addressof(*__first));
  return __first;
}
#endif

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___MEMORY_DESTROY_H
