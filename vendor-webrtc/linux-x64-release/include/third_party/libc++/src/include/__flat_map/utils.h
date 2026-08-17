// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FLAT_MAP_UTILS_H
#define _LIBCPP___FLAT_MAP_UTILS_H

#include <__config>
#include <__type_traits/container_traits.h>
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
struct __flat_map_utils {
  // Emplace a {key: value} into a flat_{multi}map, at the exact position that
  // __it_key and __it_mapped point to, assuming that the key is not already present in the map.
  // When an exception is thrown during the emplacement, the function will try its best to
  // roll back the changes it made to the map. If it cannot roll back the changes, it will
  // clear the map.
  template <class _Map, class _IterK, class _IterM, class _KeyArg, class... _MArgs>
  _LIBCPP_HIDE_FROM_ABI static typename _Map::iterator __emplace_exact_pos(
      _Map& __map, _IterK&& __it_key, _IterM&& __it_mapped, _KeyArg&& __key, _MArgs&&... __mapped_args) {
    auto __on_key_failed = std::__make_exception_guard([&]() noexcept {
      using _KeyContainer = typename _Map::key_container_type;
      if constexpr (__container_traits<_KeyContainer>::__emplacement_has_strong_exception_safety_guarantee) {
        // Nothing to roll back!
      } else {
        // we need to clear both because we don't know the state of our keys anymore
        __map.clear() /* noexcept */;
      }
    });
    auto __key_it        = __map.__containers_.keys.emplace(__it_key, std::forward<_KeyArg>(__key));
    __on_key_failed.__complete();

    auto __on_value_failed = std::__make_exception_guard([&]() noexcept {
      using _MappedContainer = typename _Map::mapped_container_type;
      if constexpr (!__container_traits<_MappedContainer>::__emplacement_has_strong_exception_safety_guarantee) {
        // we need to clear both because we don't know the state of our values anymore
        __map.clear() /* noexcept */;
      } else {
        // In this case, we know the values are just like before we attempted emplacement,
        // and we also know that the keys have been emplaced successfully. Just roll back the keys.
#  if _LIBCPP_HAS_EXCEPTIONS
        try {
#  endif // _LIBCPP_HAS_EXCEPTIONS
          __map.__containers_.keys.erase(__key_it);
#  if _LIBCPP_HAS_EXCEPTIONS
        } catch (...) {
          // Now things are funky for real. We're failing to rollback the keys.
          // Just give up and clear the whole thing.
          //
          // Also, swallow the exception that happened during the rollback and let the
          // original value-emplacement exception propagate normally.
          __map.clear() /* noexcept */;
        }
#  endif // _LIBCPP_HAS_EXCEPTIONS
      }
    });
    auto __mapped_it = __map.__containers_.values.emplace(__it_mapped, std::forward<_MArgs>(__mapped_args)...);
    __on_value_failed.__complete();

    return typename _Map::iterator(std::move(__key_it), std::move(__mapped_it));
  }

  // TODO: We could optimize this, see
  // https://github.com/llvm/llvm-project/issues/108624
  template <class _Map, class _InputIterator, class _Sentinel>
  _LIBCPP_HIDE_FROM_ABI static typename _Map::size_type
  __append(_Map& __map, _InputIterator __first, _Sentinel __last) {
    typename _Map::size_type __num_appended = 0;
    for (; __first != __last; ++__first) {
      typename _Map::value_type __kv = *__first;
      __map.__containers_.keys.insert(__map.__containers_.keys.end(), std::move(__kv.first));
      __map.__containers_.values.insert(__map.__containers_.values.end(), std::move(__kv.second));
      ++__num_appended;
    }
    return __num_appended;
  }
};
_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 23

_LIBCPP_POP_MACROS

#endif // #define _LIBCPP___FLAT_MAP_UTILS_H
