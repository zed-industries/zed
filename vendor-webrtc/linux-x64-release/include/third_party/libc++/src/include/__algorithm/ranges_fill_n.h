//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ALGORITHM_RANGES_FILL_N_H
#define _LIBCPP___ALGORITHM_RANGES_FILL_N_H

#include <__algorithm/fill_n.h>
#include <__config>
#include <__iterator/concepts.h>
#include <__iterator/incrementable_traits.h>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

#if _LIBCPP_STD_VER >= 20

_LIBCPP_BEGIN_NAMESPACE_STD

namespace ranges {
struct __fill_n {
  template <class _Type, output_iterator<const _Type&> _Iter>
  _LIBCPP_HIDE_FROM_ABI constexpr _Iter
  operator()(_Iter __first, iter_difference_t<_Iter> __n, const _Type& __value) const {
    return std::__fill_n(std::move(__first), __n, __value);
  }
};

inline namespace __cpo {
inline constexpr auto fill_n = __fill_n{};
} // namespace __cpo
} // namespace ranges

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 20

_LIBCPP_POP_MACROS

#endif // _LIBCPP___ALGORITHM_RANGES_FILL_N_H
