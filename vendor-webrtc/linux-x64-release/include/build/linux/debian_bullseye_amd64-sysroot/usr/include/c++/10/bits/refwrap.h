// Implementation of std::reference_wrapper -*- C++ -*-

// Copyright (C) 2004-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file include/bits/refwrap.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{functional}
 */

#ifndef _GLIBCXX_REFWRAP_H
#define _GLIBCXX_REFWRAP_H 1

#pragma GCC system_header

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else

#include <bits/move.h>
#include <bits/invoke.h>
#include <bits/stl_function.h> // for unary_function and binary_function

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /// @cond undocumented

  /**
   * Derives from @c unary_function or @c binary_function, or perhaps
   * nothing, depending on the number of arguments provided. The
   * primary template is the basis case, which derives nothing.
   */
  template<typename _Res, typename... _ArgTypes>
    struct _Maybe_unary_or_binary_function { };

  /// Derives from @c unary_function, as appropriate.
  template<typename _Res, typename _T1>
    struct _Maybe_unary_or_binary_function<_Res, _T1>
    : std::unary_function<_T1, _Res> { };

  /// Derives from @c binary_function, as appropriate.
  template<typename _Res, typename _T1, typename _T2>
    struct _Maybe_unary_or_binary_function<_Res, _T1, _T2>
    : std::binary_function<_T1, _T2, _Res> { };

  template<typename _Signature>
    struct _Mem_fn_traits;

  template<typename _Res, typename _Class, typename... _ArgTypes>
    struct _Mem_fn_traits_base
    {
      using __result_type = _Res;
      using __maybe_type
	= _Maybe_unary_or_binary_function<_Res, _Class*, _ArgTypes...>;
      using __arity = integral_constant<size_t, sizeof...(_ArgTypes)>;
    };

#define _GLIBCXX_MEM_FN_TRAITS2(_CV, _REF, _LVAL, _RVAL)		\
  template<typename _Res, typename _Class, typename... _ArgTypes>	\
    struct _Mem_fn_traits<_Res (_Class::*)(_ArgTypes...) _CV _REF>	\
    : _Mem_fn_traits_base<_Res, _CV _Class, _ArgTypes...>		\
    {									\
      using __vararg = false_type;					\
    };									\
  template<typename _Res, typename _Class, typename... _ArgTypes>	\
    struct _Mem_fn_traits<_Res (_Class::*)(_ArgTypes... ...) _CV _REF>	\
    : _Mem_fn_traits_base<_Res, _CV _Class, _ArgTypes...>		\
    {									\
      using __vararg = true_type;					\
    };

#define _GLIBCXX_MEM_FN_TRAITS(_REF, _LVAL, _RVAL)		\
  _GLIBCXX_MEM_FN_TRAITS2(		, _REF, _LVAL, _RVAL)	\
  _GLIBCXX_MEM_FN_TRAITS2(const		, _REF, _LVAL, _RVAL)	\
  _GLIBCXX_MEM_FN_TRAITS2(volatile	, _REF, _LVAL, _RVAL)	\
  _GLIBCXX_MEM_FN_TRAITS2(const volatile, _REF, _LVAL, _RVAL)

_GLIBCXX_MEM_FN_TRAITS( , true_type, true_type)
_GLIBCXX_MEM_FN_TRAITS(&, true_type, false_type)
_GLIBCXX_MEM_FN_TRAITS(&&, false_type, true_type)

#if __cplusplus > 201402L
_GLIBCXX_MEM_FN_TRAITS(noexcept, true_type, true_type)
_GLIBCXX_MEM_FN_TRAITS(& noexcept, true_type, false_type)
_GLIBCXX_MEM_FN_TRAITS(&& noexcept, false_type, true_type)
#endif

#undef _GLIBCXX_MEM_FN_TRAITS
#undef _GLIBCXX_MEM_FN_TRAITS2

  /// If we have found a result_type, extract it.
  template<typename _Functor, typename = __void_t<>>
    struct _Maybe_get_result_type
    { };

  template<typename _Functor>
    struct _Maybe_get_result_type<_Functor,
				  __void_t<typename _Functor::result_type>>
    { typedef typename _Functor::result_type result_type; };

  /**
   *  Base class for any function object that has a weak result type, as
   *  defined in 20.8.2 [func.require] of C++11.
  */
  template<typename _Functor>
    struct _Weak_result_type_impl
    : _Maybe_get_result_type<_Functor>
    { };

  /// Retrieve the result type for a function type.
  template<typename _Res, typename... _ArgTypes _GLIBCXX_NOEXCEPT_PARM>
    struct _Weak_result_type_impl<_Res(_ArgTypes...) _GLIBCXX_NOEXCEPT_QUAL>
    { typedef _Res result_type; };

  /// Retrieve the result type for a varargs function type.
  template<typename _Res, typename... _ArgTypes _GLIBCXX_NOEXCEPT_PARM>
    struct _Weak_result_type_impl<_Res(_ArgTypes......) _GLIBCXX_NOEXCEPT_QUAL>
    { typedef _Res result_type; };

  /// Retrieve the result type for a function pointer.
  template<typename _Res, typename... _ArgTypes _GLIBCXX_NOEXCEPT_PARM>
    struct _Weak_result_type_impl<_Res(*)(_ArgTypes...) _GLIBCXX_NOEXCEPT_QUAL>
    { typedef _Res result_type; };

  /// Retrieve the result type for a varargs function pointer.
  template<typename _Res, typename... _ArgTypes _GLIBCXX_NOEXCEPT_PARM>
    struct
    _Weak_result_type_impl<_Res(*)(_ArgTypes......) _GLIBCXX_NOEXCEPT_QUAL>
    { typedef _Res result_type; };

  // Let _Weak_result_type_impl perform the real work.
  template<typename _Functor,
	   bool = is_member_function_pointer<_Functor>::value>
    struct _Weak_result_type_memfun
    : _Weak_result_type_impl<_Functor>
    { };

  // A pointer to member function has a weak result type.
  template<typename _MemFunPtr>
    struct _Weak_result_type_memfun<_MemFunPtr, true>
    {
      using result_type = typename _Mem_fn_traits<_MemFunPtr>::__result_type;
    };

  // A pointer to data member doesn't have a weak result type.
  template<typename _Func, typename _Class>
    struct _Weak_result_type_memfun<_Func _Class::*, false>
    { };

  /**
   *  Strip top-level cv-qualifiers from the function object and let
   *  _Weak_result_type_memfun perform the real work.
  */
  template<typename _Functor>
    struct _Weak_result_type
    : _Weak_result_type_memfun<typename remove_cv<_Functor>::type>
    { };

#if __cplusplus <= 201703L
  // Detect nested argument_type.
  template<typename _Tp, typename = __void_t<>>
    struct _Refwrap_base_arg1
    { };

  // Nested argument_type.
  template<typename _Tp>
    struct _Refwrap_base_arg1<_Tp,
			      __void_t<typename _Tp::argument_type>>
    {
      typedef typename _Tp::argument_type argument_type;
    };

  // Detect nested first_argument_type and second_argument_type.
  template<typename _Tp, typename = __void_t<>>
    struct _Refwrap_base_arg2
    { };

  // Nested first_argument_type and second_argument_type.
  template<typename _Tp>
    struct _Refwrap_base_arg2<_Tp,
			      __void_t<typename _Tp::first_argument_type,
				       typename _Tp::second_argument_type>>
    {
      typedef typename _Tp::first_argument_type first_argument_type;
      typedef typename _Tp::second_argument_type second_argument_type;
    };

  /**
   *  Derives from unary_function or binary_function when it
   *  can. Specializations handle all of the easy cases. The primary
   *  template determines what to do with a class type, which may
   *  derive from both unary_function and binary_function.
  */
  template<typename _Tp>
    struct _Reference_wrapper_base
    : _Weak_result_type<_Tp>, _Refwrap_base_arg1<_Tp>, _Refwrap_base_arg2<_Tp>
    { };

  // - a function type (unary)
  template<typename _Res, typename _T1 _GLIBCXX_NOEXCEPT_PARM>
    struct _Reference_wrapper_base<_Res(_T1) _GLIBCXX_NOEXCEPT_QUAL>
    : unary_function<_T1, _Res>
    { };

  template<typename _Res, typename _T1>
    struct _Reference_wrapper_base<_Res(_T1) const>
    : unary_function<_T1, _Res>
    { };

  template<typename _Res, typename _T1>
    struct _Reference_wrapper_base<_Res(_T1) volatile>
    : unary_function<_T1, _Res>
    { };

  template<typename _Res, typename _T1>
    struct _Reference_wrapper_base<_Res(_T1) const volatile>
    : unary_function<_T1, _Res>
    { };

  // - a function type (binary)
  template<typename _Res, typename _T1, typename _T2 _GLIBCXX_NOEXCEPT_PARM>
    struct _Reference_wrapper_base<_Res(_T1, _T2) _GLIBCXX_NOEXCEPT_QUAL>
    : binary_function<_T1, _T2, _Res>
    { };

  template<typename _Res, typename _T1, typename _T2>
    struct _Reference_wrapper_base<_Res(_T1, _T2) const>
    : binary_function<_T1, _T2, _Res>
    { };

  template<typename _Res, typename _T1, typename _T2>
    struct _Reference_wrapper_base<_Res(_T1, _T2) volatile>
    : binary_function<_T1, _T2, _Res>
    { };

  template<typename _Res, typename _T1, typename _T2>
    struct _Reference_wrapper_base<_Res(_T1, _T2) const volatile>
    : binary_function<_T1, _T2, _Res>
    { };

  // - a function pointer type (unary)
  template<typename _Res, typename _T1 _GLIBCXX_NOEXCEPT_PARM>
    struct _Reference_wrapper_base<_Res(*)(_T1) _GLIBCXX_NOEXCEPT_QUAL>
    : unary_function<_T1, _Res>
    { };

  // - a function pointer type (binary)
  template<typename _Res, typename _T1, typename _T2 _GLIBCXX_NOEXCEPT_PARM>
    struct _Reference_wrapper_base<_Res(*)(_T1, _T2) _GLIBCXX_NOEXCEPT_QUAL>
    : binary_function<_T1, _T2, _Res>
    { };

  template<typename _Tp, bool = is_member_function_pointer<_Tp>::value>
    struct _Reference_wrapper_base_memfun
    : _Reference_wrapper_base<_Tp>
    { };

  template<typename _MemFunPtr>
    struct _Reference_wrapper_base_memfun<_MemFunPtr, true>
    : _Mem_fn_traits<_MemFunPtr>::__maybe_type
    {
      using result_type = typename _Mem_fn_traits<_MemFunPtr>::__result_type;
    };
#endif // ! C++20

  /// @endcond

  /**
   *  @brief Primary class template for reference_wrapper.
   *  @ingroup functors
   */
  template<typename _Tp>
    class reference_wrapper
#if __cplusplus <= 201703L
    // In C++20 std::reference_wrapper<T> allows T to be incomplete,
    // so checking for nested types could result in ODR violations.
    : public _Reference_wrapper_base_memfun<typename remove_cv<_Tp>::type>
#endif
    {
      _Tp* _M_data;

      _GLIBCXX20_CONSTEXPR
      static _Tp* _S_fun(_Tp& __r) noexcept { return std::__addressof(__r); }

      static void _S_fun(_Tp&&) = delete;

      template<typename _Up, typename _Up2 = __remove_cvref_t<_Up>>
	using __not_same
	  = typename enable_if<!is_same<reference_wrapper, _Up2>::value>::type;

    public:
      typedef _Tp type;

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2993. reference_wrapper<T> conversion from T&&
      // 3041. Unnecessary decay in reference_wrapper
      template<typename _Up, typename = __not_same<_Up>, typename
		= decltype(reference_wrapper::_S_fun(std::declval<_Up>()))>
	_GLIBCXX20_CONSTEXPR
	reference_wrapper(_Up&& __uref)
	noexcept(noexcept(reference_wrapper::_S_fun(std::declval<_Up>())))
	: _M_data(reference_wrapper::_S_fun(std::forward<_Up>(__uref)))
	{ }

      reference_wrapper(const reference_wrapper&) = default;

      reference_wrapper&
      operator=(const reference_wrapper&) = default;

      _GLIBCXX20_CONSTEXPR
      operator _Tp&() const noexcept
      { return this->get(); }

      _GLIBCXX20_CONSTEXPR
      _Tp&
      get() const noexcept
      { return *_M_data; }

      template<typename... _Args>
	_GLIBCXX20_CONSTEXPR
	typename result_of<_Tp&(_Args&&...)>::type
	operator()(_Args&&... __args) const
	{
#if __cplusplus > 201703L
	  if constexpr (is_object_v<type>)
	    static_assert(sizeof(type), "type must be complete");
#endif
	  return std::__invoke(get(), std::forward<_Args>(__args)...);
	}
    };

#if __cpp_deduction_guides
  template<typename _Tp>
    reference_wrapper(_Tp&) -> reference_wrapper<_Tp>;
#endif

  /// @relates reference_wrapper @{

  /// Denotes a reference should be taken to a variable.
  template<typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline reference_wrapper<_Tp>
    ref(_Tp& __t) noexcept
    { return reference_wrapper<_Tp>(__t); }

  /// Denotes a const reference should be taken to a variable.
  template<typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline reference_wrapper<const _Tp>
    cref(const _Tp& __t) noexcept
    { return reference_wrapper<const _Tp>(__t); }

  template<typename _Tp>
    void ref(const _Tp&&) = delete;

  template<typename _Tp>
    void cref(const _Tp&&) = delete;

  /// std::ref overload to prevent wrapping a reference_wrapper
  template<typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline reference_wrapper<_Tp>
    ref(reference_wrapper<_Tp> __t) noexcept
    { return __t; }

  /// std::cref overload to prevent wrapping a reference_wrapper
  template<typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline reference_wrapper<const _Tp>
    cref(reference_wrapper<_Tp> __t) noexcept
    { return { __t.get() }; }

  // @}

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++11

#endif // _GLIBCXX_REFWRAP_H
