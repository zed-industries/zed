//===---------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef _LIBCPP___FWD_VARIANT_H
#define _LIBCPP___FWD_VARIANT_H

#include <__config>
#include <__cstddef/size_t.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

#if _LIBCPP_STD_VER >= 17

template <class... _Types>
class variant;

template <class _Tp>
struct variant_size;

template <class _Tp>
inline constexpr size_t variant_size_v = variant_size<_Tp>::value;

template <size_t _Ip, class _Tp>
struct variant_alternative;

template <size_t _Ip, class _Tp>
using variant_alternative_t = typename variant_alternative<_Ip, _Tp>::type;

inline constexpr size_t variant_npos = static_cast<size_t>(-1);

template <size_t _Ip, class... _Types>
_LIBCPP_HIDE_FROM_ABI
_LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr variant_alternative_t<_Ip, variant<_Types...>>&
get(variant<_Types...>&);

template <size_t _Ip, class... _Types>
_LIBCPP_HIDE_FROM_ABI
_LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr variant_alternative_t<_Ip, variant<_Types...>>&&
get(variant<_Types...>&&);

template <size_t _Ip, class... _Types>
_LIBCPP_HIDE_FROM_ABI
_LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr const variant_alternative_t<_Ip, variant<_Types...>>&
get(const variant<_Types...>&);

template <size_t _Ip, class... _Types>
_LIBCPP_HIDE_FROM_ABI
_LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr const variant_alternative_t<_Ip, variant<_Types...>>&&
get(const variant<_Types...>&&);

template <class _Tp, class... _Types>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr _Tp& get(variant<_Types...>&);

template <class _Tp, class... _Types>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr _Tp&& get(variant<_Types...>&&);

template <class _Tp, class... _Types>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr const _Tp& get(const variant<_Types...>&);

template <class _Tp, class... _Types>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_AVAILABILITY_THROW_BAD_VARIANT_ACCESS constexpr const _Tp&&
get(const variant<_Types...>&&);

#endif // _LIBCPP_STD_VER >= 17

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___FWD_VARIANT_H
