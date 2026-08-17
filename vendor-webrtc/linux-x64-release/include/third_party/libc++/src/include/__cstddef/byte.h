//===---------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef _LIBCPP___CSTDDEF_BYTE_H
#define _LIBCPP___CSTDDEF_BYTE_H

#include <__config>
#include <__fwd/byte.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/is_integral.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

#if _LIBCPP_STD_VER >= 17
_LIBCPP_BEGIN_UNVERSIONED_NAMESPACE_STD

enum class byte : unsigned char {};

_LIBCPP_HIDE_FROM_ABI inline constexpr byte operator|(byte __lhs, byte __rhs) noexcept {
  return static_cast<byte>(
      static_cast<unsigned char>(static_cast<unsigned int>(__lhs) | static_cast<unsigned int>(__rhs)));
}

_LIBCPP_HIDE_FROM_ABI inline constexpr byte& operator|=(byte& __lhs, byte __rhs) noexcept {
  return __lhs = __lhs | __rhs;
}

_LIBCPP_HIDE_FROM_ABI inline constexpr byte operator&(byte __lhs, byte __rhs) noexcept {
  return static_cast<byte>(
      static_cast<unsigned char>(static_cast<unsigned int>(__lhs) & static_cast<unsigned int>(__rhs)));
}

_LIBCPP_HIDE_FROM_ABI inline constexpr byte& operator&=(byte& __lhs, byte __rhs) noexcept {
  return __lhs = __lhs & __rhs;
}

_LIBCPP_HIDE_FROM_ABI inline constexpr byte operator^(byte __lhs, byte __rhs) noexcept {
  return static_cast<byte>(
      static_cast<unsigned char>(static_cast<unsigned int>(__lhs) ^ static_cast<unsigned int>(__rhs)));
}

_LIBCPP_HIDE_FROM_ABI inline constexpr byte& operator^=(byte& __lhs, byte __rhs) noexcept {
  return __lhs = __lhs ^ __rhs;
}

_LIBCPP_HIDE_FROM_ABI inline constexpr byte operator~(byte __b) noexcept {
  return static_cast<byte>(static_cast<unsigned char>(~static_cast<unsigned int>(__b)));
}

template <class _Integer, __enable_if_t<is_integral<_Integer>::value, int> = 0>
_LIBCPP_HIDE_FROM_ABI constexpr byte& operator<<=(byte& __lhs, _Integer __shift) noexcept {
  return __lhs = __lhs << __shift;
}

template <class _Integer, __enable_if_t<is_integral<_Integer>::value, int> = 0>
_LIBCPP_HIDE_FROM_ABI constexpr byte operator<<(byte __lhs, _Integer __shift) noexcept {
  return static_cast<byte>(static_cast<unsigned char>(static_cast<unsigned int>(__lhs) << __shift));
}

template <class _Integer, __enable_if_t<is_integral<_Integer>::value, int> = 0>
_LIBCPP_HIDE_FROM_ABI constexpr byte& operator>>=(byte& __lhs, _Integer __shift) noexcept {
  return __lhs = __lhs >> __shift;
}

template <class _Integer, __enable_if_t<is_integral<_Integer>::value, int> = 0>
_LIBCPP_HIDE_FROM_ABI constexpr byte operator>>(byte __lhs, _Integer __shift) noexcept {
  return static_cast<byte>(static_cast<unsigned char>(static_cast<unsigned int>(__lhs) >> __shift));
}

template <class _Integer, __enable_if_t<is_integral<_Integer>::value, int> = 0>
[[nodiscard]] _LIBCPP_HIDE_FROM_ABI constexpr _Integer to_integer(byte __b) noexcept {
  return static_cast<_Integer>(__b);
}

_LIBCPP_END_UNVERSIONED_NAMESPACE_STD
#endif // _LIBCPP_STD_VER >= 17

#endif // _LIBCPP___CSTDDEF_BYTE_H
