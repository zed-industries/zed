//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ATOMIC_SUPPORT_H
#define _LIBCPP___ATOMIC_SUPPORT_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

//
// This file implements base support for atomics on the platform.
//
// The following operations and types must be implemented (where _Atmc
// is __cxx_atomic_base_impl for readability):
//
// clang-format off
//
// template <class _Tp>
// struct __cxx_atomic_base_impl;
//
// #define __cxx_atomic_is_lock_free(__size)
//
// void __cxx_atomic_thread_fence(memory_order __order) noexcept;
// void __cxx_atomic_signal_fence(memory_order __order) noexcept;
//
// template <class _Tp>
// void __cxx_atomic_init(_Atmc<_Tp> volatile* __a, _Tp __val) noexcept;
// template <class _Tp>
// void __cxx_atomic_init(_Atmc<_Tp>* __a, _Tp __val) noexcept;
//
// template <class _Tp>
// void __cxx_atomic_store(_Atmc<_Tp> volatile* __a, _Tp __val, memory_order __order) noexcept;
// template <class _Tp>
// void __cxx_atomic_store(_Atmc<_Tp>* __a, _Tp __val, memory_order __order) noexcept;
//
// template <class _Tp>
// _Tp __cxx_atomic_load(_Atmc<_Tp> const volatile* __a, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_load(_Atmc<_Tp> const* __a, memory_order __order) noexcept;
//
// template <class _Tp>
// void __cxx_atomic_load_inplace(_Atmc<_Tp> const volatile* __a, _Tp* __dst, memory_order __order) noexcept;
// template <class _Tp>
// void __cxx_atomic_load_inplace(_Atmc<_Tp> const* __a, _Tp* __dst, memory_order __order) noexcept;
//
// template <class _Tp>
// _Tp __cxx_atomic_exchange(_Atmc<_Tp> volatile* __a, _Tp __value, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_exchange(_Atmc<_Tp>* __a, _Tp __value, memory_order __order) noexcept;
//
// template <class _Tp>
// bool __cxx_atomic_compare_exchange_strong(_Atmc<_Tp> volatile* __a, _Tp* __expected, _Tp __value, memory_order __success, memory_order __failure) noexcept;
// template <class _Tp>
// bool __cxx_atomic_compare_exchange_strong(_Atmc<_Tp>* __a, _Tp* __expected, _Tp __value, memory_order __success, memory_order __failure) noexcept;
//
// template <class _Tp>
// bool __cxx_atomic_compare_exchange_weak(_Atmc<_Tp> volatile* __a, _Tp* __expected, _Tp __value, memory_order __success, memory_order __failure) noexcept;
// template <class _Tp>
// bool __cxx_atomic_compare_exchange_weak(_Atmc<_Tp>* __a, _Tp* __expected, _Tp __value, memory_order __success, memory_order __failure) noexcept;
//
// template <class _Tp>
// _Tp __cxx_atomic_fetch_add(_Atmc<_Tp> volatile* __a, _Tp __delta, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_fetch_add(_Atmc<_Tp>* __a, _Tp __delta, memory_order __order) noexcept;
//
// template <class _Tp>
// _Tp* __cxx_atomic_fetch_add(_Atmc<_Tp*> volatile* __a, ptrdiff_t __delta, memory_order __order) noexcept;
// template <class _Tp>
// _Tp* __cxx_atomic_fetch_add(_Atmc<_Tp*>* __a, ptrdiff_t __delta, memory_order __order) noexcept;
//
// template <class _Tp>
// _Tp __cxx_atomic_fetch_sub(_Atmc<_Tp> volatile* __a, _Tp __delta, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_fetch_sub(_Atmc<_Tp>* __a, _Tp __delta, memory_order __order) noexcept;
// template <class _Tp>
// _Tp* __cxx_atomic_fetch_sub(_Atmc<_Tp*> volatile* __a, ptrdiff_t __delta, memory_order __order) noexcept;
// template <class _Tp>
// _Tp* __cxx_atomic_fetch_sub(_Atmc<_Tp*>* __a, ptrdiff_t __delta, memory_order __order) noexcept;
//
// template <class _Tp>
// _Tp __cxx_atomic_fetch_and(_Atmc<_Tp> volatile* __a, _Tp __pattern, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_fetch_and(_Atmc<_Tp>* __a, _Tp __pattern, memory_order __order) noexcept;
//
// template <class _Tp>
// _Tp __cxx_atomic_fetch_or(_Atmc<_Tp> volatile* __a, _Tp __pattern, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_fetch_or(_Atmc<_Tp>* __a, _Tp __pattern, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_fetch_xor(_Atmc<_Tp> volatile* __a, _Tp __pattern, memory_order __order) noexcept;
// template <class _Tp>
// _Tp __cxx_atomic_fetch_xor(_Atmc<_Tp>* __a, _Tp __pattern, memory_order __order) noexcept;
//
// clang-format on
//

#if _LIBCPP_HAS_GCC_ATOMIC_IMP
#  include <__atomic/support/gcc.h>
#elif _LIBCPP_HAS_C_ATOMIC_IMP
#  include <__atomic/support/c11.h>
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <typename _Tp, typename _Base = __cxx_atomic_base_impl<_Tp> >
struct __cxx_atomic_impl : public _Base {
  _LIBCPP_HIDE_FROM_ABI __cxx_atomic_impl() _NOEXCEPT = default;
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR explicit __cxx_atomic_impl(_Tp __value) _NOEXCEPT : _Base(__value) {}
};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___ATOMIC_SUPPORT_H
