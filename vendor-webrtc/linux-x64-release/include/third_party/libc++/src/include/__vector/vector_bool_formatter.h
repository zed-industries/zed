//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___VECTOR_VECTOR_BOOL_FORMATTER_H
#define _LIBCPP___VECTOR_VECTOR_BOOL_FORMATTER_H

#include <__concepts/same_as.h>
#include <__config>
#include <__format/formatter.h>
#include <__format/formatter_bool.h>
#include <__fwd/vector.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

#if _LIBCPP_STD_VER >= 23

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp, class _CharT>
// Since is-vector-bool-reference is only used once it's inlined here.
  requires same_as<typename _Tp::__container, vector<bool, typename _Tp::__container::allocator_type>>
struct formatter<_Tp, _CharT> {
private:
  formatter<bool, _CharT> __underlying_;

public:
  template <class _ParseContext>
  _LIBCPP_HIDE_FROM_ABI constexpr typename _ParseContext::iterator parse(_ParseContext& __ctx) {
    return __underlying_.parse(__ctx);
  }

  template <class _FormatContext>
  _LIBCPP_HIDE_FROM_ABI typename _FormatContext::iterator format(const _Tp& __ref, _FormatContext& __ctx) const {
    return __underlying_.format(__ref, __ctx);
  }
};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 23

#endif // _LIBCPP___VECTOR_VECTOR_BOOL_FORMATTER_H
