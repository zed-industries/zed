//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_DATASIZEOF_H
#define _LIBCPP___TYPE_TRAITS_DATASIZEOF_H

#include <__config>
#include <__cstddef/size_t.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

// This trait provides the size of a type excluding any tail padding.
//
// It is useful in contexts where performing an operation using the full size of the class (including padding) may
// have unintended side effects, such as overwriting a derived class' member when writing the tail padding of a class
// through a pointer-to-base.

_LIBCPP_BEGIN_NAMESPACE_STD

// TODO: Enable this again once #94816 is fixed.
#if (__has_keyword(__datasizeof) || __has_extension(datasizeof)) && 0
template <class _Tp>
inline const size_t __datasizeof_v = __datasizeof(_Tp);
#else
template <class _Tp>
struct _FirstPaddingByte {
  _LIBCPP_NO_UNIQUE_ADDRESS _Tp __v_;
  char __first_padding_byte_;
};

// _FirstPaddingByte<> is sometimes non-standard layout.
// It is conditionally-supported to use __builtin_offsetof in that case, but GCC and Clang allow it.
_LIBCPP_DIAGNOSTIC_PUSH
_LIBCPP_CLANG_DIAGNOSTIC_IGNORED("-Winvalid-offsetof")
_LIBCPP_GCC_DIAGNOSTIC_IGNORED("-Winvalid-offsetof")
template <class _Tp>
inline const size_t __datasizeof_v = __builtin_offsetof(_FirstPaddingByte<_Tp>, __first_padding_byte_);
_LIBCPP_DIAGNOSTIC_POP
#endif // __has_extension(datasizeof)

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_DATASIZEOF_H
