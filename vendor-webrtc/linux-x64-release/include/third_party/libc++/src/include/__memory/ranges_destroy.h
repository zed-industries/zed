// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_RANGES_DESTROY_H
#define _LIBCPP___MEMORY_RANGES_DESTROY_H

#include <__concepts/destructible.h>
#include <__config>
#include <__iterator/incrementable_traits.h>
#include <__iterator/iterator_traits.h>
#include <__memory/concepts.h>
#include <__memory/destroy.h>
#include <__ranges/access.h>
#include <__ranges/concepts.h>
#include <__ranges/dangling.h>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

#if _LIBCPP_STD_VER >= 20
namespace ranges {

// destroy

struct __destroy {
  template <__nothrow_input_iterator _InputIterator, __nothrow_sentinel_for<_InputIterator> _Sentinel>
    requires destructible<iter_value_t<_InputIterator>>
  _LIBCPP_HIDE_FROM_ABI constexpr _InputIterator operator()(_InputIterator __first, _Sentinel __last) const noexcept {
    return std::__destroy(std::move(__first), std::move(__last));
  }

  template <__nothrow_input_range _InputRange>
    requires destructible<range_value_t<_InputRange>>
  _LIBCPP_HIDE_FROM_ABI constexpr borrowed_iterator_t<_InputRange> operator()(_InputRange&& __range) const noexcept {
    return (*this)(ranges::begin(__range), ranges::end(__range));
  }
};

inline namespace __cpo {
inline constexpr auto destroy = __destroy{};
} // namespace __cpo

// destroy_n

struct __destroy_n {
  template <__nothrow_input_iterator _InputIterator>
    requires destructible<iter_value_t<_InputIterator>>
  _LIBCPP_HIDE_FROM_ABI constexpr _InputIterator
  operator()(_InputIterator __first, iter_difference_t<_InputIterator> __n) const noexcept {
    return std::destroy_n(std::move(__first), __n);
  }
};

inline namespace __cpo {
inline constexpr auto destroy_n = __destroy_n{};
} // namespace __cpo

} // namespace ranges

#endif // _LIBCPP_STD_VER >= 20

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___MEMORY_RANGES_DESTROY_H
