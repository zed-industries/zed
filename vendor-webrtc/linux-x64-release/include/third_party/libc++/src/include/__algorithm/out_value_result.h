// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ALGORITHM_OUT_VALUE_RESULT_H
#define _LIBCPP___ALGORITHM_OUT_VALUE_RESULT_H

#include <__concepts/convertible_to.h>
#include <__config>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

#if _LIBCPP_STD_VER >= 23

namespace ranges {

template <class _OutIter1, class _ValType1>
struct out_value_result {
  _LIBCPP_NO_UNIQUE_ADDRESS _OutIter1 out;
  _LIBCPP_NO_UNIQUE_ADDRESS _ValType1 value;

  template <class _OutIter2, class _ValType2>
    requires convertible_to<const _OutIter1&, _OutIter2> && convertible_to<const _ValType1&, _ValType2>
  _LIBCPP_HIDE_FROM_ABI constexpr operator out_value_result<_OutIter2, _ValType2>() const& {
    return {out, value};
  }

  template <class _OutIter2, class _ValType2>
    requires convertible_to<_OutIter1, _OutIter2> && convertible_to<_ValType1, _ValType2>
  _LIBCPP_HIDE_FROM_ABI constexpr operator out_value_result<_OutIter2, _ValType2>() && {
    return {std::move(out), std::move(value)};
  }
};

} // namespace ranges

#endif // _LIBCPP_STD_VER >= 23

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___ALGORITHM_OUT_VALUE_RESULT_H
