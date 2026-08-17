// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FLAT_SET_UTILS_H
#define _LIBCPP___FLAT_SET_UTILS_H

#include <__config>
#include <__ranges/access.h>
#include <__ranges/concepts.h>
#include <__type_traits/container_traits.h>
#include <__type_traits/decay.h>
#include <__utility/exception_guard.h>
#include <__utility/forward.h>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

#if _LIBCPP_STD_VER >= 23

_LIBCPP_BEGIN_NAMESPACE_STD

// These utilities are defined in a class instead of a namespace so that this class can be befriended more easily.
struct __flat_set_utils {
  // Emplace a key into a flat_{multi}set, at the exact position that
  // __it point to, assuming that the key is not already present in the set.
  // When an exception is thrown during the emplacement, the function will clear the set if the container does not
  // have strong exception safety guarantee on emplacement.
  template <class _Set, class _Iter, class _KeyArg>
  _LIBCPP_HIDE_FROM_ABI static auto __emplace_exact_pos(_Set& __set, _Iter&& __iter, _KeyArg&& __key) {
    using _KeyContainer = typename decay_t<_Set>::container_type;
    auto __on_failure   = std::__make_exception_guard([&]() noexcept {
      if constexpr (!__container_traits<_KeyContainer>::__emplacement_has_strong_exception_safety_guarantee) {
        __set.clear() /* noexcept */;
      }
    });
    auto __key_it       = __set.__keys_.emplace(__iter.__base(), std::forward<_KeyArg>(__key));
    __on_failure.__complete();
    return typename decay_t<_Set>::iterator(std::move(__key_it));
  }

  template <class _Set, class _InputIterator>
  _LIBCPP_HIDE_FROM_ABI static void __append(_Set& __set, _InputIterator __first, _InputIterator __last) {
    __set.__keys_.insert(__set.__keys_.end(), std::move(__first), std::move(__last));
  }

  template <class _Set, class _Range>
  _LIBCPP_HIDE_FROM_ABI static void __append(_Set& __set, _Range&& __rng) {
    if constexpr (requires { __set.__keys_.insert_range(__set.__keys_.end(), std::forward<_Range>(__rng)); }) {
      // C++23 Sequence Container should have insert_range member function
      // Note that not all Sequence Containers provide append_range.
      __set.__keys_.insert_range(__set.__keys_.end(), std::forward<_Range>(__rng));
    } else if constexpr (ranges::common_range<_Range>) {
      __set.__keys_.insert(__set.__keys_.end(), ranges::begin(__rng), ranges::end(__rng));
    } else {
      for (auto&& __x : __rng) {
        __set.__keys_.insert(__set.__keys_.end(), std::forward<decltype(__x)>(__x));
      }
    }
  }
};
_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 23

_LIBCPP_POP_MACROS

#endif // #define _LIBCPP___FLAT_SET_UTILS_H
