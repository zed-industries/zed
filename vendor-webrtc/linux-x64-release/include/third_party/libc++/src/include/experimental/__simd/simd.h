// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP_EXPERIMENTAL___SIMD_SIMD_H
#define _LIBCPP_EXPERIMENTAL___SIMD_SIMD_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/is_integral.h>
#include <__type_traits/is_same.h>
#include <__type_traits/remove_cvref.h>
#include <__utility/forward.h>
#include <experimental/__simd/declaration.h>
#include <experimental/__simd/reference.h>
#include <experimental/__simd/traits.h>
#include <experimental/__simd/utility.h>

#if _LIBCPP_STD_VER >= 17 && defined(_LIBCPP_ENABLE_EXPERIMENTAL)

_LIBCPP_BEGIN_NAMESPACE_EXPERIMENTAL
inline namespace parallelism_v2 {

template <class _Simd, class _Impl, bool>
class __simd_int_operators {};

template <class _Simd, class _Impl>
class __simd_int_operators<_Simd, _Impl, true> {
public:
  // unary operators for integral _Tp
  _LIBCPP_HIDE_FROM_ABI _Simd operator~() const noexcept {
    return _Simd(_Impl::__bitwise_not((*static_cast<const _Simd*>(this)).__s_), _Simd::__storage_tag);
  }
};

// class template simd [simd.class]
// TODO: implement simd class
template <class _Tp, class _Abi>
class simd : public __simd_int_operators<simd<_Tp, _Abi>, __simd_operations<_Tp, _Abi>, is_integral_v<_Tp>> {
  using _Impl _LIBCPP_NODEBUG    = __simd_operations<_Tp, _Abi>;
  using _Storage _LIBCPP_NODEBUG = typename _Impl::_SimdStorage;

  _Storage __s_;

  friend class __simd_int_operators<simd, _Impl, true>;

public:
  using value_type = _Tp;
  using reference  = __simd_reference<_Tp, _Storage, value_type>;
  using mask_type  = simd_mask<_Tp, _Abi>;
  using abi_type   = _Abi;

  static _LIBCPP_HIDE_FROM_ABI constexpr size_t size() noexcept { return simd_size_v<value_type, abi_type>; }

  _LIBCPP_HIDE_FROM_ABI simd() noexcept = default;

  // explicit conversion from and to implementation-defined types
  struct __storage_tag_t {};
  static constexpr __storage_tag_t __storage_tag{};
  explicit _LIBCPP_HIDE_FROM_ABI operator _Storage() const { return __s_; }
  explicit _LIBCPP_HIDE_FROM_ABI simd(const _Storage& __s, __storage_tag_t) : __s_(__s) {}

  // broadcast constructor
  template <class _Up, enable_if_t<__can_broadcast_v<value_type, __remove_cvref_t<_Up>>, int> = 0>
  _LIBCPP_HIDE_FROM_ABI simd(_Up&& __v) noexcept : __s_(_Impl::__broadcast(static_cast<value_type>(__v))) {}

  // implicit type conversion constructor
  template <class _Up,
            enable_if_t<!is_same_v<_Up, _Tp> && is_same_v<abi_type, simd_abi::fixed_size<size()>> &&
                            __is_non_narrowing_convertible_v<_Up, value_type>,
                        int> = 0>
  _LIBCPP_HIDE_FROM_ABI simd(const simd<_Up, simd_abi::fixed_size<size()>>& __v) noexcept {
    for (size_t __i = 0; __i < size(); __i++) {
      (*this)[__i] = static_cast<value_type>(__v[__i]);
    }
  }

  // generator constructor
  template <class _Generator, enable_if_t<__can_generate_v<value_type, _Generator, size()>, int> = 0>
  explicit _LIBCPP_HIDE_FROM_ABI simd(_Generator&& __g) noexcept
      : __s_(_Impl::__generate(std::forward<_Generator>(__g))) {}

  // load constructor
  template <class _Up, class _Flags, enable_if_t<__is_vectorizable_v<_Up> && is_simd_flag_type_v<_Flags>, int> = 0>
  _LIBCPP_HIDE_FROM_ABI simd(const _Up* __mem, _Flags) {
    _Impl::__load(__s_, _Flags::template __apply<simd>(__mem));
  }

  // copy functions
  template <class _Up, class _Flags, enable_if_t<__is_vectorizable_v<_Up> && is_simd_flag_type_v<_Flags>, int> = 0>
  _LIBCPP_HIDE_FROM_ABI void copy_from(const _Up* __mem, _Flags) {
    _Impl::__load(__s_, _Flags::template __apply<simd>(__mem));
  }

  template <class _Up, class _Flags, enable_if_t<__is_vectorizable_v<_Up> && is_simd_flag_type_v<_Flags>, int> = 0>
  _LIBCPP_HIDE_FROM_ABI void copy_to(_Up* __mem, _Flags) const {
    _Impl::__store(__s_, _Flags::template __apply<simd>(__mem));
  }

  // scalar access [simd.subscr]
  _LIBCPP_HIDE_FROM_ABI reference operator[](size_t __i) noexcept { return reference(__s_, __i); }
  _LIBCPP_HIDE_FROM_ABI value_type operator[](size_t __i) const noexcept { return __s_.__get(__i); }

  // simd unary operators
  _LIBCPP_HIDE_FROM_ABI simd& operator++() noexcept {
    _Impl::__increment(__s_);
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI simd operator++(int) noexcept {
    simd __r = *this;
    _Impl::__increment(__s_);
    return __r;
  }

  _LIBCPP_HIDE_FROM_ABI simd& operator--() noexcept {
    _Impl::__decrement(__s_);
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI simd operator--(int) noexcept {
    simd __r = *this;
    _Impl::__decrement(__s_);
    return __r;
  }

  _LIBCPP_HIDE_FROM_ABI mask_type operator!() const noexcept {
    return mask_type(_Impl::__negate(__s_), mask_type::__storage_tag);
  }

  _LIBCPP_HIDE_FROM_ABI simd operator+() const noexcept { return *this; }

  _LIBCPP_HIDE_FROM_ABI simd operator-() const noexcept { return simd(_Impl::__unary_minus(__s_), __storage_tag); }
};

template <class _Tp, class _Abi>
inline constexpr bool is_simd_v<simd<_Tp, _Abi>> = true;

template <class _Tp>
using native_simd = simd<_Tp, simd_abi::native<_Tp>>;

template <class _Tp, int _Np>
using fixed_size_simd = simd<_Tp, simd_abi::fixed_size<_Np>>;

} // namespace parallelism_v2
_LIBCPP_END_NAMESPACE_EXPERIMENTAL

#endif // _LIBCPP_STD_VER >= 17 && defined(_LIBCPP_ENABLE_EXPERIMENTAL)
#endif // _LIBCPP_EXPERIMENTAL___SIMD_SIMD_H
