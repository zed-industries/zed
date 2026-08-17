//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ATOMIC_SUPPORT_C11_H
#define _LIBCPP___ATOMIC_SUPPORT_C11_H

#include <__atomic/memory_order.h>
#include <__config>
#include <__cstddef/ptrdiff_t.h>
#include <__memory/addressof.h>
#include <__type_traits/remove_const.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

//
// This file implements support for C11-style atomics
//

_LIBCPP_BEGIN_NAMESPACE_STD

template <typename _Tp>
struct __cxx_atomic_base_impl {
  _LIBCPP_HIDE_FROM_ABI
#ifndef _LIBCPP_CXX03_LANG
  __cxx_atomic_base_impl() _NOEXCEPT = default;
#else
  __cxx_atomic_base_impl() _NOEXCEPT : __a_value() {
  }
#endif // _LIBCPP_CXX03_LANG
  _LIBCPP_CONSTEXPR explicit __cxx_atomic_base_impl(_Tp __value) _NOEXCEPT : __a_value(__value) {}
  _Atomic(_Tp) __a_value;
};

#define __cxx_atomic_is_lock_free(__s) __c11_atomic_is_lock_free(__s)

_LIBCPP_HIDE_FROM_ABI inline void __cxx_atomic_thread_fence(memory_order __order) _NOEXCEPT {
  __c11_atomic_thread_fence(static_cast<__memory_order_underlying_t>(__order));
}

_LIBCPP_HIDE_FROM_ABI inline void __cxx_atomic_signal_fence(memory_order __order) _NOEXCEPT {
  __c11_atomic_signal_fence(static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI void __cxx_atomic_init(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __val) _NOEXCEPT {
  __c11_atomic_init(std::addressof(__a->__a_value), __val);
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI void __cxx_atomic_init(__cxx_atomic_base_impl<_Tp>* __a, _Tp __val) _NOEXCEPT {
  __c11_atomic_init(std::addressof(__a->__a_value), __val);
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI void
__cxx_atomic_store(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __val, memory_order __order) _NOEXCEPT {
  __c11_atomic_store(std::addressof(__a->__a_value), __val, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI void
__cxx_atomic_store(__cxx_atomic_base_impl<_Tp>* __a, _Tp __val, memory_order __order) _NOEXCEPT {
  __c11_atomic_store(std::addressof(__a->__a_value), __val, static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_load(__cxx_atomic_base_impl<_Tp> const volatile* __a, memory_order __order) _NOEXCEPT {
  using __ptr_type = __remove_const_t<decltype(__a->__a_value)>*;
  return __c11_atomic_load(
      const_cast<__ptr_type>(std::addressof(__a->__a_value)), static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp __cxx_atomic_load(__cxx_atomic_base_impl<_Tp> const* __a, memory_order __order) _NOEXCEPT {
  using __ptr_type = __remove_const_t<decltype(__a->__a_value)>*;
  return __c11_atomic_load(
      const_cast<__ptr_type>(std::addressof(__a->__a_value)), static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI void
__cxx_atomic_load_inplace(__cxx_atomic_base_impl<_Tp> const volatile* __a, _Tp* __dst, memory_order __order) _NOEXCEPT {
  using __ptr_type = __remove_const_t<decltype(__a->__a_value)>*;
  *__dst           = __c11_atomic_load(
      const_cast<__ptr_type>(std::addressof(__a->__a_value)), static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI void
__cxx_atomic_load_inplace(__cxx_atomic_base_impl<_Tp> const* __a, _Tp* __dst, memory_order __order) _NOEXCEPT {
  using __ptr_type = __remove_const_t<decltype(__a->__a_value)>*;
  *__dst           = __c11_atomic_load(
      const_cast<__ptr_type>(std::addressof(__a->__a_value)), static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_exchange(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __value, memory_order __order) _NOEXCEPT {
  return __c11_atomic_exchange(
      std::addressof(__a->__a_value), __value, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_exchange(__cxx_atomic_base_impl<_Tp>* __a, _Tp __value, memory_order __order) _NOEXCEPT {
  return __c11_atomic_exchange(
      std::addressof(__a->__a_value), __value, static_cast<__memory_order_underlying_t>(__order));
}

_LIBCPP_HIDE_FROM_ABI inline _LIBCPP_CONSTEXPR memory_order __to_failure_order(memory_order __order) {
  // Avoid switch statement to make this a constexpr.
  return __order == memory_order_release
           ? memory_order_relaxed
           : (__order == memory_order_acq_rel ? memory_order_acquire : __order);
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI bool __cxx_atomic_compare_exchange_strong(
    __cxx_atomic_base_impl<_Tp> volatile* __a,
    _Tp* __expected,
    _Tp __value,
    memory_order __success,
    memory_order __failure) _NOEXCEPT {
  return __c11_atomic_compare_exchange_strong(
      std::addressof(__a->__a_value),
      __expected,
      __value,
      static_cast<__memory_order_underlying_t>(__success),
      static_cast<__memory_order_underlying_t>(__to_failure_order(__failure)));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI bool __cxx_atomic_compare_exchange_strong(
    __cxx_atomic_base_impl<_Tp>* __a, _Tp* __expected, _Tp __value, memory_order __success, memory_order __failure)
    _NOEXCEPT {
  return __c11_atomic_compare_exchange_strong(
      std::addressof(__a->__a_value),
      __expected,
      __value,
      static_cast<__memory_order_underlying_t>(__success),
      static_cast<__memory_order_underlying_t>(__to_failure_order(__failure)));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI bool __cxx_atomic_compare_exchange_weak(
    __cxx_atomic_base_impl<_Tp> volatile* __a,
    _Tp* __expected,
    _Tp __value,
    memory_order __success,
    memory_order __failure) _NOEXCEPT {
  return __c11_atomic_compare_exchange_weak(
      std::addressof(__a->__a_value),
      __expected,
      __value,
      static_cast<__memory_order_underlying_t>(__success),
      static_cast<__memory_order_underlying_t>(__to_failure_order(__failure)));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI bool __cxx_atomic_compare_exchange_weak(
    __cxx_atomic_base_impl<_Tp>* __a, _Tp* __expected, _Tp __value, memory_order __success, memory_order __failure)
    _NOEXCEPT {
  return __c11_atomic_compare_exchange_weak(
      std::addressof(__a->__a_value),
      __expected,
      __value,
      static_cast<__memory_order_underlying_t>(__success),
      static_cast<__memory_order_underlying_t>(__to_failure_order(__failure)));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_add(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_add(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_add(__cxx_atomic_base_impl<_Tp>* __a, _Tp __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_add(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp*
__cxx_atomic_fetch_add(__cxx_atomic_base_impl<_Tp*> volatile* __a, ptrdiff_t __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_add(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp*
__cxx_atomic_fetch_add(__cxx_atomic_base_impl<_Tp*>* __a, ptrdiff_t __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_add(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_sub(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_sub(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_sub(__cxx_atomic_base_impl<_Tp>* __a, _Tp __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_sub(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp*
__cxx_atomic_fetch_sub(__cxx_atomic_base_impl<_Tp*> volatile* __a, ptrdiff_t __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_sub(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp*
__cxx_atomic_fetch_sub(__cxx_atomic_base_impl<_Tp*>* __a, ptrdiff_t __delta, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_sub(
      std::addressof(__a->__a_value), __delta, static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_and(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __pattern, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_and(
      std::addressof(__a->__a_value), __pattern, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_and(__cxx_atomic_base_impl<_Tp>* __a, _Tp __pattern, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_and(
      std::addressof(__a->__a_value), __pattern, static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_or(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __pattern, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_or(
      std::addressof(__a->__a_value), __pattern, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_or(__cxx_atomic_base_impl<_Tp>* __a, _Tp __pattern, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_or(
      std::addressof(__a->__a_value), __pattern, static_cast<__memory_order_underlying_t>(__order));
}

template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_xor(__cxx_atomic_base_impl<_Tp> volatile* __a, _Tp __pattern, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_xor(
      std::addressof(__a->__a_value), __pattern, static_cast<__memory_order_underlying_t>(__order));
}
template <class _Tp>
_LIBCPP_HIDE_FROM_ABI _Tp
__cxx_atomic_fetch_xor(__cxx_atomic_base_impl<_Tp>* __a, _Tp __pattern, memory_order __order) _NOEXCEPT {
  return __c11_atomic_fetch_xor(
      std::addressof(__a->__a_value), __pattern, static_cast<__memory_order_underlying_t>(__order));
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___ATOMIC_SUPPORT_C11_H
