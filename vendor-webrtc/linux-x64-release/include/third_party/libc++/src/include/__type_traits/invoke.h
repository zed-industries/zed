// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_INVOKE_H
#define _LIBCPP___TYPE_TRAITS_INVOKE_H

#include <__config>
#include <__type_traits/conditional.h>
#include <__type_traits/decay.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/integral_constant.h>
#include <__type_traits/is_base_of.h>
#include <__type_traits/is_core_convertible.h>
#include <__type_traits/is_member_pointer.h>
#include <__type_traits/is_reference_wrapper.h>
#include <__type_traits/is_same.h>
#include <__type_traits/is_void.h>
#include <__type_traits/nat.h>
#include <__utility/declval.h>
#include <__utility/forward.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

// This file defines the following libc++-internal API (back-ported to C++03):
//
// template <class... Args>
// decltype(auto) __invoke(Args&&... args) noexcept(noexcept(std::invoke(std::forward<Args>(args...)))) {
//   return std::invoke(std::forward<Args>(args)...);
// }
//
// template <class Ret, class... Args>
// Ret __invoke_r(Args&&... args) {
//   return std::invoke_r(std::forward<Args>(args)...);
// }
//
// template <class Ret, class Func, class... Args>
// inline const bool __is_invocable_r_v = is_invocable_r_v<Ret, Func, Args...>;
//
// template <class Func, class... Args>
// struct __is_invocable : is_invocable<Func, Args...> {};
//
// template <class Func, class... Args>
// inline const bool __is_invocable_v = is_invocable_v<Func, Args...>;
//
// template <class Func, class... Args>
// inline const bool __is_nothrow_invocable_v = is_nothrow_invocable_v<Func, Args...>;
//
// template <class Func, class... Args>
// struct __invoke_result : invoke_result {};
//
// template <class Func, class... Args>
// using __invoke_result_t = invoke_result_t<Func, Args...>;

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _DecayedFp>
struct __member_pointer_class_type {};

template <class _Ret, class _ClassType>
struct __member_pointer_class_type<_Ret _ClassType::*> {
  typedef _ClassType type;
};

template <class _Fp,
          class _A0,
          class _DecayFp = __decay_t<_Fp>,
          class _DecayA0 = __decay_t<_A0>,
          class _ClassT  = typename __member_pointer_class_type<_DecayFp>::type>
using __enable_if_bullet1 _LIBCPP_NODEBUG =
    __enable_if_t<is_member_function_pointer<_DecayFp>::value &&
                  (is_same<_ClassT, _DecayA0>::value || is_base_of<_ClassT, _DecayA0>::value)>;

template <class _Fp, class _A0, class _DecayFp = __decay_t<_Fp>, class _DecayA0 = __decay_t<_A0> >
using __enable_if_bullet2 _LIBCPP_NODEBUG =
    __enable_if_t<is_member_function_pointer<_DecayFp>::value && __is_reference_wrapper<_DecayA0>::value>;

template <class _Fp,
          class _A0,
          class _DecayFp = __decay_t<_Fp>,
          class _DecayA0 = __decay_t<_A0>,
          class _ClassT  = typename __member_pointer_class_type<_DecayFp>::type>
using __enable_if_bullet3 _LIBCPP_NODEBUG =
    __enable_if_t<is_member_function_pointer<_DecayFp>::value &&
                  !(is_same<_ClassT, _DecayA0>::value || is_base_of<_ClassT, _DecayA0>::value) &&
                  !__is_reference_wrapper<_DecayA0>::value>;

template <class _Fp,
          class _A0,
          class _DecayFp = __decay_t<_Fp>,
          class _DecayA0 = __decay_t<_A0>,
          class _ClassT  = typename __member_pointer_class_type<_DecayFp>::type>
using __enable_if_bullet4 _LIBCPP_NODEBUG =
    __enable_if_t<is_member_object_pointer<_DecayFp>::value &&
                  (is_same<_ClassT, _DecayA0>::value || is_base_of<_ClassT, _DecayA0>::value)>;

template <class _Fp, class _A0, class _DecayFp = __decay_t<_Fp>, class _DecayA0 = __decay_t<_A0> >
using __enable_if_bullet5 _LIBCPP_NODEBUG =
    __enable_if_t<is_member_object_pointer<_DecayFp>::value && __is_reference_wrapper<_DecayA0>::value>;

template <class _Fp,
          class _A0,
          class _DecayFp = __decay_t<_Fp>,
          class _DecayA0 = __decay_t<_A0>,
          class _ClassT  = typename __member_pointer_class_type<_DecayFp>::type>
using __enable_if_bullet6 _LIBCPP_NODEBUG =
    __enable_if_t<is_member_object_pointer<_DecayFp>::value &&
                  !(is_same<_ClassT, _DecayA0>::value || is_base_of<_ClassT, _DecayA0>::value) &&
                  !__is_reference_wrapper<_DecayA0>::value>;

// __invoke forward declarations

// fall back - none of the bullets

template <class... _Args>
__nat __invoke(_Args&&... __args);

// bullets 1, 2 and 3

// clang-format off
template <class _Fp, class _A0, class... _Args, class = __enable_if_bullet1<_Fp, _A0> >
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
decltype((std::declval<_A0>().*std::declval<_Fp>())(std::declval<_Args>()...))
__invoke(_Fp&& __f, _A0&& __a0, _Args&&... __args)
    _NOEXCEPT_(noexcept((static_cast<_A0&&>(__a0).*__f)(static_cast<_Args&&>(__args)...)))
               { return (static_cast<_A0&&>(__a0).*__f)(static_cast<_Args&&>(__args)...); }

template <class _Fp, class _A0, class... _Args, class = __enable_if_bullet2<_Fp, _A0> >
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
decltype((std::declval<_A0>().get().*std::declval<_Fp>())(std::declval<_Args>()...))
__invoke(_Fp&& __f, _A0&& __a0, _Args&&... __args)
    _NOEXCEPT_(noexcept((__a0.get().*__f)(static_cast<_Args&&>(__args)...)))
               { return (__a0.get().*__f)(static_cast<_Args&&>(__args)...); }

template <class _Fp, class _A0, class... _Args, class = __enable_if_bullet3<_Fp, _A0> >
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
decltype(((*std::declval<_A0>()).*std::declval<_Fp>())(std::declval<_Args>()...))
__invoke(_Fp&& __f, _A0&& __a0, _Args&&... __args)
    _NOEXCEPT_(noexcept(((*static_cast<_A0&&>(__a0)).*__f)(static_cast<_Args&&>(__args)...)))
               { return ((*static_cast<_A0&&>(__a0)).*__f)(static_cast<_Args&&>(__args)...); }

// bullets 4, 5 and 6

template <class _Fp, class _A0, class = __enable_if_bullet4<_Fp, _A0> >
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
decltype(std::declval<_A0>().*std::declval<_Fp>())
__invoke(_Fp&& __f, _A0&& __a0)
    _NOEXCEPT_(noexcept(static_cast<_A0&&>(__a0).*__f))
               { return static_cast<_A0&&>(__a0).*__f; }

template <class _Fp, class _A0, class = __enable_if_bullet5<_Fp, _A0> >
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
decltype(std::declval<_A0>().get().*std::declval<_Fp>())
__invoke(_Fp&& __f, _A0&& __a0)
    _NOEXCEPT_(noexcept(__a0.get().*__f))
               { return __a0.get().*__f; }

template <class _Fp, class _A0, class = __enable_if_bullet6<_Fp, _A0> >
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
decltype((*std::declval<_A0>()).*std::declval<_Fp>())
__invoke(_Fp&& __f, _A0&& __a0)
    _NOEXCEPT_(noexcept((*static_cast<_A0&&>(__a0)).*__f))
               { return (*static_cast<_A0&&>(__a0)).*__f; }

// bullet 7

template <class _Fp, class... _Args>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR
decltype(std::declval<_Fp>()(std::declval<_Args>()...))
__invoke(_Fp&& __f, _Args&&... __args)
    _NOEXCEPT_(noexcept(static_cast<_Fp&&>(__f)(static_cast<_Args&&>(__args)...)))
               { return static_cast<_Fp&&>(__f)(static_cast<_Args&&>(__args)...); }
// clang-format on

// __invokable
template <class _Ret, class _Fp, class... _Args>
struct __invokable_r {
  template <class _XFp, class... _XArgs>
  static decltype(std::__invoke(std::declval<_XFp>(), std::declval<_XArgs>()...)) __try_call(int);
  template <class _XFp, class... _XArgs>
  static __nat __try_call(...);

  // FIXME: Check that _Ret, _Fp, and _Args... are all complete types, cv void,
  // or incomplete array types as required by the standard.
  using _Result _LIBCPP_NODEBUG = decltype(__try_call<_Fp, _Args...>(0));

  using type              = __conditional_t<_IsNotSame<_Result, __nat>::value,
                                            __conditional_t<is_void<_Ret>::value, true_type, __is_core_convertible<_Result, _Ret> >,
                                            false_type>;
  static const bool value = type::value;
};
template <class _Fp, class... _Args>
using __is_invocable _LIBCPP_NODEBUG = __invokable_r<void, _Fp, _Args...>;

template <bool _IsInvokable, bool _IsCVVoid, class _Ret, class _Fp, class... _Args>
struct __nothrow_invokable_r_imp {
  static const bool value = false;
};

template <class _Ret, class _Fp, class... _Args>
struct __nothrow_invokable_r_imp<true, false, _Ret, _Fp, _Args...> {
  typedef __nothrow_invokable_r_imp _ThisT;

  template <class _Tp>
  static void __test_noexcept(_Tp) _NOEXCEPT;

#ifdef _LIBCPP_CXX03_LANG
  static const bool value = false;
#else
  static const bool value =
      noexcept(_ThisT::__test_noexcept<_Ret>(std::__invoke(std::declval<_Fp>(), std::declval<_Args>()...)));
#endif
};

template <class _Ret, class _Fp, class... _Args>
struct __nothrow_invokable_r_imp<true, true, _Ret, _Fp, _Args...> {
#ifdef _LIBCPP_CXX03_LANG
  static const bool value = false;
#else
  static const bool value = noexcept(std::__invoke(std::declval<_Fp>(), std::declval<_Args>()...));
#endif
};

template <class _Ret, class _Fp, class... _Args>
using __nothrow_invokable_r _LIBCPP_NODEBUG =
    __nothrow_invokable_r_imp<__invokable_r<_Ret, _Fp, _Args...>::value, is_void<_Ret>::value, _Ret, _Fp, _Args...>;

template <class _Fp, class... _Args>
using __nothrow_invokable _LIBCPP_NODEBUG =
    __nothrow_invokable_r_imp<__is_invocable<_Fp, _Args...>::value, true, void, _Fp, _Args...>;

template <class _Ret, bool = is_void<_Ret>::value>
struct __invoke_void_return_wrapper {
  template <class... _Args>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 static _Ret __call(_Args&&... __args) {
    return std::__invoke(std::forward<_Args>(__args)...);
  }
};

template <class _Ret>
struct __invoke_void_return_wrapper<_Ret, true> {
  template <class... _Args>
  _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 static void __call(_Args&&... __args) {
    std::__invoke(std::forward<_Args>(__args)...);
  }
};

template <class _Func, class... _Args>
inline const bool __is_invocable_v = __is_invocable<_Func, _Args...>::value;

template <class _Ret, class _Func, class... _Args>
inline const bool __is_invocable_r_v = __invokable_r<_Ret, _Func, _Args...>::value;

template <class _Func, class... _Args>
inline const bool __is_nothrow_invocable_v = __nothrow_invokable<_Func, _Args...>::value;

template <class _Func, class... _Args>
struct __invoke_result
    : enable_if<__is_invocable_v<_Func, _Args...>, typename __invokable_r<void, _Func, _Args...>::_Result> {};

template <class _Func, class... _Args>
using __invoke_result_t _LIBCPP_NODEBUG = typename __invoke_result<_Func, _Args...>::type;

template <class _Ret, class... _Args>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _Ret __invoke_r(_Args&&... __args) {
  return __invoke_void_return_wrapper<_Ret>::__call(std::forward<_Args>(__args)...);
}

#if _LIBCPP_STD_VER >= 17

// is_invocable

template <class _Fn, class... _Args>
struct _LIBCPP_NO_SPECIALIZATIONS is_invocable : bool_constant<__is_invocable_v<_Fn, _Args...>> {};

template <class _Ret, class _Fn, class... _Args>
struct _LIBCPP_NO_SPECIALIZATIONS is_invocable_r : bool_constant<__is_invocable_r_v<_Ret, _Fn, _Args...>> {};

template <class _Fn, class... _Args>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr bool is_invocable_v = __is_invocable_v<_Fn, _Args...>;

template <class _Ret, class _Fn, class... _Args>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr bool is_invocable_r_v = __is_invocable_r_v<_Ret, _Fn, _Args...>;

// is_nothrow_invocable

template <class _Fn, class... _Args>
struct _LIBCPP_NO_SPECIALIZATIONS is_nothrow_invocable : bool_constant<__nothrow_invokable<_Fn, _Args...>::value> {};

template <class _Ret, class _Fn, class... _Args>
struct _LIBCPP_NO_SPECIALIZATIONS is_nothrow_invocable_r
    : bool_constant<__nothrow_invokable_r<_Ret, _Fn, _Args...>::value> {};

template <class _Fn, class... _Args>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr bool is_nothrow_invocable_v = is_nothrow_invocable<_Fn, _Args...>::value;

template <class _Ret, class _Fn, class... _Args>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr bool is_nothrow_invocable_r_v =
    is_nothrow_invocable_r<_Ret, _Fn, _Args...>::value;

template <class _Fn, class... _Args>
struct _LIBCPP_NO_SPECIALIZATIONS invoke_result : __invoke_result<_Fn, _Args...> {};

template <class _Fn, class... _Args>
using invoke_result_t = typename invoke_result<_Fn, _Args...>::type;

#endif // _LIBCPP_STD_VER >= 17

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_INVOKE_H
