// Allocator traits -*- C++ -*-

// Copyright (C) 2011-2020 Free Software Foundation, Inc.
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

/** @file bits/alloc_traits.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{memory}
 */

#ifndef _ALLOC_TRAITS_H
#define _ALLOC_TRAITS_H 1

#include <bits/stl_construct.h>
#include <bits/memoryfwd.h>
#if __cplusplus >= 201103L
# include <bits/allocator.h>
# include <bits/ptr_traits.h>
# include <ext/numeric_traits.h>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

#if __cplusplus >= 201103L
#define __cpp_lib_allocator_traits_is_always_equal 201411

  struct __allocator_traits_base
  {
    template<typename _Tp, typename _Up, typename = void>
      struct __rebind : __replace_first_arg<_Tp, _Up> { };

    template<typename _Tp, typename _Up>
      struct __rebind<_Tp, _Up,
		      __void_t<typename _Tp::template rebind<_Up>::other>>
      { using type = typename _Tp::template rebind<_Up>::other; };

  protected:
    template<typename _Tp>
      using __pointer = typename _Tp::pointer;
    template<typename _Tp>
      using __c_pointer = typename _Tp::const_pointer;
    template<typename _Tp>
      using __v_pointer = typename _Tp::void_pointer;
    template<typename _Tp>
      using __cv_pointer = typename _Tp::const_void_pointer;
    template<typename _Tp>
      using __pocca = typename _Tp::propagate_on_container_copy_assignment;
    template<typename _Tp>
      using __pocma = typename _Tp::propagate_on_container_move_assignment;
    template<typename _Tp>
      using __pocs = typename _Tp::propagate_on_container_swap;
    template<typename _Tp>
      using __equal = typename _Tp::is_always_equal;
  };

  template<typename _Alloc, typename _Up>
    using __alloc_rebind
      = typename __allocator_traits_base::template __rebind<_Alloc, _Up>::type;

  /**
   * @brief  Uniform interface to all allocator types.
   * @ingroup allocators
  */
  template<typename _Alloc>
    struct allocator_traits : __allocator_traits_base
    {
      /// The allocator type
      typedef _Alloc allocator_type;
      /// The allocated type
      typedef typename _Alloc::value_type value_type;

      /**
       * @brief   The allocator's pointer type.
       *
       * @c Alloc::pointer if that type exists, otherwise @c value_type*
      */
      using pointer = __detected_or_t<value_type*, __pointer, _Alloc>;

    private:
      // Select _Func<_Alloc> or pointer_traits<pointer>::rebind<_Tp>
      template<template<typename> class _Func, typename _Tp, typename = void>
	struct _Ptr
	{
	  using type = typename pointer_traits<pointer>::template rebind<_Tp>;
	};

      template<template<typename> class _Func, typename _Tp>
	struct _Ptr<_Func, _Tp, __void_t<_Func<_Alloc>>>
	{
	  using type = _Func<_Alloc>;
	};

      // Select _A2::difference_type or pointer_traits<_Ptr>::difference_type
      template<typename _A2, typename _PtrT, typename = void>
	struct _Diff
	{ using type = typename pointer_traits<_PtrT>::difference_type; };

      template<typename _A2, typename _PtrT>
	struct _Diff<_A2, _PtrT, __void_t<typename _A2::difference_type>>
	{ using type = typename _A2::difference_type; };

      // Select _A2::size_type or make_unsigned<_DiffT>::type
      template<typename _A2, typename _DiffT, typename = void>
	struct _Size : make_unsigned<_DiffT> { };

      template<typename _A2, typename _DiffT>
	struct _Size<_A2, _DiffT, __void_t<typename _A2::size_type>>
	{ using type = typename _A2::size_type; };

    public:
      /**
       * @brief   The allocator's const pointer type.
       *
       * @c Alloc::const_pointer if that type exists, otherwise
       * <tt> pointer_traits<pointer>::rebind<const value_type> </tt>
      */
      using const_pointer = typename _Ptr<__c_pointer, const value_type>::type;

      /**
       * @brief   The allocator's void pointer type.
       *
       * @c Alloc::void_pointer if that type exists, otherwise
       * <tt> pointer_traits<pointer>::rebind<void> </tt>
      */
      using void_pointer = typename _Ptr<__v_pointer, void>::type;

      /**
       * @brief   The allocator's const void pointer type.
       *
       * @c Alloc::const_void_pointer if that type exists, otherwise
       * <tt> pointer_traits<pointer>::rebind<const void> </tt>
      */
      using const_void_pointer = typename _Ptr<__cv_pointer, const void>::type;

      /**
       * @brief   The allocator's difference type
       *
       * @c Alloc::difference_type if that type exists, otherwise
       * <tt> pointer_traits<pointer>::difference_type </tt>
      */
      using difference_type = typename _Diff<_Alloc, pointer>::type;

      /**
       * @brief   The allocator's size type
       *
       * @c Alloc::size_type if that type exists, otherwise
       * <tt> make_unsigned<difference_type>::type </tt>
      */
      using size_type = typename _Size<_Alloc, difference_type>::type;

      /**
       * @brief   How the allocator is propagated on copy assignment
       *
       * @c Alloc::propagate_on_container_copy_assignment if that type exists,
       * otherwise @c false_type
      */
      using propagate_on_container_copy_assignment
	= __detected_or_t<false_type, __pocca, _Alloc>;

      /**
       * @brief   How the allocator is propagated on move assignment
       *
       * @c Alloc::propagate_on_container_move_assignment if that type exists,
       * otherwise @c false_type
      */
      using propagate_on_container_move_assignment
	= __detected_or_t<false_type, __pocma, _Alloc>;

      /**
       * @brief   How the allocator is propagated on swap
       *
       * @c Alloc::propagate_on_container_swap if that type exists,
       * otherwise @c false_type
      */
      using propagate_on_container_swap
	= __detected_or_t<false_type, __pocs, _Alloc>;

      /**
       * @brief   Whether all instances of the allocator type compare equal.
       *
       * @c Alloc::is_always_equal if that type exists,
       * otherwise @c is_empty<Alloc>::type
      */
      using is_always_equal
	= __detected_or_t<typename is_empty<_Alloc>::type, __equal, _Alloc>;

      template<typename _Tp>
	using rebind_alloc = __alloc_rebind<_Alloc, _Tp>;
      template<typename _Tp>
	using rebind_traits = allocator_traits<rebind_alloc<_Tp>>;

    private:
      template<typename _Alloc2>
	static constexpr auto
	_S_allocate(_Alloc2& __a, size_type __n, const_void_pointer __hint, int)
	-> decltype(__a.allocate(__n, __hint))
	{ return __a.allocate(__n, __hint); }

      template<typename _Alloc2>
	static constexpr pointer
	_S_allocate(_Alloc2& __a, size_type __n, const_void_pointer, ...)
	{ return __a.allocate(__n); }

      template<typename _Tp, typename... _Args>
	struct __construct_helper
	{
	  template<typename _Alloc2,
	    typename = decltype(std::declval<_Alloc2*>()->construct(
		  std::declval<_Tp*>(), std::declval<_Args>()...))>
	    static true_type __test(int);

	  template<typename>
	    static false_type __test(...);

	  using type = decltype(__test<_Alloc>(0));
	};

      template<typename _Tp, typename... _Args>
	using __has_construct
	  = typename __construct_helper<_Tp, _Args...>::type;

      template<typename _Tp, typename... _Args>
	static _GLIBCXX14_CONSTEXPR _Require<__has_construct<_Tp, _Args...>>
	_S_construct(_Alloc& __a, _Tp* __p, _Args&&... __args)
	noexcept(noexcept(__a.construct(__p, std::forward<_Args>(__args)...)))
	{ __a.construct(__p, std::forward<_Args>(__args)...); }

      template<typename _Tp, typename... _Args>
	static _GLIBCXX14_CONSTEXPR
	_Require<__and_<__not_<__has_construct<_Tp, _Args...>>,
			       is_constructible<_Tp, _Args...>>>
	_S_construct(_Alloc&, _Tp* __p, _Args&&... __args)
	noexcept(std::is_nothrow_constructible<_Tp, _Args...>::value)
	{
#if __cplusplus <= 201703L
	  ::new((void*)__p) _Tp(std::forward<_Args>(__args)...);
#else
	  std::construct_at(__p, std::forward<_Args>(__args)...);
#endif
	}

      template<typename _Alloc2, typename _Tp>
	static _GLIBCXX14_CONSTEXPR auto
	_S_destroy(_Alloc2& __a, _Tp* __p, int)
	noexcept(noexcept(__a.destroy(__p)))
	-> decltype(__a.destroy(__p))
	{ __a.destroy(__p); }

      template<typename _Alloc2, typename _Tp>
	static _GLIBCXX14_CONSTEXPR void
	_S_destroy(_Alloc2&, _Tp* __p, ...)
	noexcept(std::is_nothrow_destructible<_Tp>::value)
	{ std::_Destroy(__p); }

      template<typename _Alloc2>
	static constexpr auto
	_S_max_size(_Alloc2& __a, int)
	-> decltype(__a.max_size())
	{ return __a.max_size(); }

      template<typename _Alloc2>
	static constexpr size_type
	_S_max_size(_Alloc2&, ...)
	{
	  // _GLIBCXX_RESOLVE_LIB_DEFECTS
	  // 2466. allocator_traits::max_size() default behavior is incorrect
	  return __gnu_cxx::__numeric_traits<size_type>::__max
	    / sizeof(value_type);
	}

      template<typename _Alloc2>
	static constexpr auto
	_S_select(_Alloc2& __a, int)
	-> decltype(__a.select_on_container_copy_construction())
	{ return __a.select_on_container_copy_construction(); }

      template<typename _Alloc2>
	static constexpr _Alloc2
	_S_select(_Alloc2& __a, ...)
	{ return __a; }

    public:

      /**
       *  @brief  Allocate memory.
       *  @param  __a  An allocator.
       *  @param  __n  The number of objects to allocate space for.
       *
       *  Calls @c a.allocate(n)
      */
      _GLIBCXX_NODISCARD static _GLIBCXX20_CONSTEXPR pointer
      allocate(_Alloc& __a, size_type __n)
      { return __a.allocate(__n); }

      /**
       *  @brief  Allocate memory.
       *  @param  __a  An allocator.
       *  @param  __n  The number of objects to allocate space for.
       *  @param  __hint Aid to locality.
       *  @return Memory of suitable size and alignment for @a n objects
       *          of type @c value_type
       *
       *  Returns <tt> a.allocate(n, hint) </tt> if that expression is
       *  well-formed, otherwise returns @c a.allocate(n)
      */
      _GLIBCXX_NODISCARD static _GLIBCXX20_CONSTEXPR pointer
      allocate(_Alloc& __a, size_type __n, const_void_pointer __hint)
      { return _S_allocate(__a, __n, __hint, 0); }

      /**
       *  @brief  Deallocate memory.
       *  @param  __a  An allocator.
       *  @param  __p  Pointer to the memory to deallocate.
       *  @param  __n  The number of objects space was allocated for.
       *
       *  Calls <tt> a.deallocate(p, n) </tt>
      */
      static _GLIBCXX20_CONSTEXPR void
      deallocate(_Alloc& __a, pointer __p, size_type __n)
      { __a.deallocate(__p, __n); }

      /**
       *  @brief  Construct an object of type @a _Tp
       *  @param  __a  An allocator.
       *  @param  __p  Pointer to memory of suitable size and alignment for Tp
       *  @param  __args Constructor arguments.
       *
       *  Calls <tt> __a.construct(__p, std::forward<Args>(__args)...) </tt>
       *  if that expression is well-formed, otherwise uses placement-new
       *  to construct an object of type @a _Tp at location @a __p from the
       *  arguments @a __args...
      */
      template<typename _Tp, typename... _Args>
	static _GLIBCXX20_CONSTEXPR auto
	construct(_Alloc& __a, _Tp* __p, _Args&&... __args)
	noexcept(noexcept(_S_construct(__a, __p,
				       std::forward<_Args>(__args)...)))
	-> decltype(_S_construct(__a, __p, std::forward<_Args>(__args)...))
	{ _S_construct(__a, __p, std::forward<_Args>(__args)...); }

      /**
       *  @brief  Destroy an object of type @a _Tp
       *  @param  __a  An allocator.
       *  @param  __p  Pointer to the object to destroy
       *
       *  Calls @c __a.destroy(__p) if that expression is well-formed,
       *  otherwise calls @c __p->~_Tp()
      */
      template<typename _Tp>
	static _GLIBCXX20_CONSTEXPR void
	destroy(_Alloc& __a, _Tp* __p)
	noexcept(noexcept(_S_destroy(__a, __p, 0)))
	{ _S_destroy(__a, __p, 0); }

      /**
       *  @brief  The maximum supported allocation size
       *  @param  __a  An allocator.
       *  @return @c __a.max_size() or @c numeric_limits<size_type>::max()
       *
       *  Returns @c __a.max_size() if that expression is well-formed,
       *  otherwise returns @c numeric_limits<size_type>::max()
      */
      static _GLIBCXX20_CONSTEXPR size_type
      max_size(const _Alloc& __a) noexcept
      { return _S_max_size(__a, 0); }

      /**
       *  @brief  Obtain an allocator to use when copying a container.
       *  @param  __rhs  An allocator.
       *  @return @c __rhs.select_on_container_copy_construction() or @a __rhs
       *
       *  Returns @c __rhs.select_on_container_copy_construction() if that
       *  expression is well-formed, otherwise returns @a __rhs
      */
      static _GLIBCXX20_CONSTEXPR _Alloc
      select_on_container_copy_construction(const _Alloc& __rhs)
      { return _S_select(__rhs, 0); }
    };

#if __cplusplus > 201703L
# define __cpp_lib_constexpr_dynamic_alloc 201907L
#endif

  /// Partial specialization for std::allocator.
  template<typename _Tp>
    struct allocator_traits<allocator<_Tp>>
    {
      /// The allocator type
      using allocator_type = allocator<_Tp>;

      /// The allocated type
      using value_type = _Tp;

      /// The allocator's pointer type.
      using pointer = _Tp*;

      /// The allocator's const pointer type.
      using const_pointer = const _Tp*;

      /// The allocator's void pointer type.
      using void_pointer = void*;

      /// The allocator's const void pointer type.
      using const_void_pointer = const void*;

      /// The allocator's difference type
      using difference_type = std::ptrdiff_t;

      /// The allocator's size type
      using size_type = std::size_t;

      /// How the allocator is propagated on copy assignment
      using propagate_on_container_copy_assignment = false_type;

      /// How the allocator is propagated on move assignment
      using propagate_on_container_move_assignment = true_type;

      /// How the allocator is propagated on swap
      using propagate_on_container_swap = false_type;

      /// Whether all instances of the allocator type compare equal.
      using is_always_equal = true_type;

      template<typename _Up>
	using rebind_alloc = allocator<_Up>;

      template<typename _Up>
	using rebind_traits = allocator_traits<allocator<_Up>>;

      /**
       *  @brief  Allocate memory.
       *  @param  __a  An allocator.
       *  @param  __n  The number of objects to allocate space for.
       *
       *  Calls @c a.allocate(n)
      */
      _GLIBCXX_NODISCARD static _GLIBCXX20_CONSTEXPR pointer
      allocate(allocator_type& __a, size_type __n)
      { return __a.allocate(__n); }

      /**
       *  @brief  Allocate memory.
       *  @param  __a  An allocator.
       *  @param  __n  The number of objects to allocate space for.
       *  @param  __hint Aid to locality.
       *  @return Memory of suitable size and alignment for @a n objects
       *          of type @c value_type
       *
       *  Returns <tt> a.allocate(n, hint) </tt>
      */
      _GLIBCXX_NODISCARD static _GLIBCXX20_CONSTEXPR pointer
      allocate(allocator_type& __a, size_type __n, const_void_pointer __hint)
      {
#if __cplusplus <= 201703L
	return __a.allocate(__n, __hint);
#else
	return __a.allocate(__n);
#endif
      }

      /**
       *  @brief  Deallocate memory.
       *  @param  __a  An allocator.
       *  @param  __p  Pointer to the memory to deallocate.
       *  @param  __n  The number of objects space was allocated for.
       *
       *  Calls <tt> a.deallocate(p, n) </tt>
      */
      static _GLIBCXX20_CONSTEXPR void
      deallocate(allocator_type& __a, pointer __p, size_type __n)
      { __a.deallocate(__p, __n); }

      /**
       *  @brief  Construct an object of type `_Up`
       *  @param  __a  An allocator.
       *  @param  __p  Pointer to memory of suitable size and alignment for
       *	       an object of type `_Up`.
       *  @param  __args Constructor arguments.
       *
       *  Calls `__a.construct(__p, std::forward<_Args>(__args)...)`
       *  in C++11, C++14 and C++17. Changed in C++20 to call
       *  `std::construct_at(__p, std::forward<_Args>(__args)...)` instead.
      */
      template<typename _Up, typename... _Args>
	static _GLIBCXX20_CONSTEXPR void
	construct(allocator_type& __a __attribute__((__unused__)), _Up* __p,
		  _Args&&... __args)
	noexcept(std::is_nothrow_constructible<_Up, _Args...>::value)
	{
#if __cplusplus <= 201703L
	  __a.construct(__p, std::forward<_Args>(__args)...);
#else
	  std::construct_at(__p, std::forward<_Args>(__args)...);
#endif
	}

      /**
       *  @brief  Destroy an object of type @a _Up
       *  @param  __a  An allocator.
       *  @param  __p  Pointer to the object to destroy
       *
       *  Calls @c __a.destroy(__p).
      */
      template<typename _Up>
	static _GLIBCXX20_CONSTEXPR void
	destroy(allocator_type& __a __attribute__((__unused__)), _Up* __p)
	noexcept(is_nothrow_destructible<_Up>::value)
	{
#if __cplusplus <= 201703L
	  __a.destroy(__p);
#else
	  std::destroy_at(__p);
#endif
	}

      /**
       *  @brief  The maximum supported allocation size
       *  @param  __a  An allocator.
       *  @return @c __a.max_size()
      */
      static _GLIBCXX20_CONSTEXPR size_type
      max_size(const allocator_type& __a __attribute__((__unused__))) noexcept
      {
#if __cplusplus <= 201703L
	return __a.max_size();
#else
	return size_t(-1) / sizeof(value_type);
#endif
      }

      /**
       *  @brief  Obtain an allocator to use when copying a container.
       *  @param  __rhs  An allocator.
       *  @return @c __rhs
      */
      static _GLIBCXX20_CONSTEXPR allocator_type
      select_on_container_copy_construction(const allocator_type& __rhs)
      { return __rhs; }
    };

#if __cplusplus < 201703L
  template<typename _Alloc>
    inline void
    __do_alloc_on_copy(_Alloc& __one, const _Alloc& __two, true_type)
    { __one = __two; }

  template<typename _Alloc>
    inline void
    __do_alloc_on_copy(_Alloc&, const _Alloc&, false_type)
    { }
#endif

  template<typename _Alloc>
    _GLIBCXX14_CONSTEXPR inline void
    __alloc_on_copy(_Alloc& __one, const _Alloc& __two)
    {
      typedef allocator_traits<_Alloc> __traits;
      typedef typename __traits::propagate_on_container_copy_assignment __pocca;
#if __cplusplus >= 201703L
      if constexpr (__pocca::value)
	__one = __two;
#else
      __do_alloc_on_copy(__one, __two, __pocca());
#endif
    }

  template<typename _Alloc>
    constexpr _Alloc
    __alloc_on_copy(const _Alloc& __a)
    {
      typedef allocator_traits<_Alloc> __traits;
      return __traits::select_on_container_copy_construction(__a);
    }

#if __cplusplus < 201703L
  template<typename _Alloc>
    inline void __do_alloc_on_move(_Alloc& __one, _Alloc& __two, true_type)
    { __one = std::move(__two); }

  template<typename _Alloc>
    inline void __do_alloc_on_move(_Alloc&, _Alloc&, false_type)
    { }
#endif

  template<typename _Alloc>
    _GLIBCXX14_CONSTEXPR inline void
    __alloc_on_move(_Alloc& __one, _Alloc& __two)
    {
      typedef allocator_traits<_Alloc> __traits;
      typedef typename __traits::propagate_on_container_move_assignment __pocma;
#if __cplusplus >= 201703L
      if constexpr (__pocma::value)
	__one = std::move(__two);
#else
      __do_alloc_on_move(__one, __two, __pocma());
#endif
    }

#if __cplusplus < 201703L
  template<typename _Alloc>
    inline void __do_alloc_on_swap(_Alloc& __one, _Alloc& __two, true_type)
    {
      using std::swap;
      swap(__one, __two);
    }

  template<typename _Alloc>
    inline void __do_alloc_on_swap(_Alloc&, _Alloc&, false_type)
    { }
#endif

  template<typename _Alloc>
    _GLIBCXX14_CONSTEXPR inline void
    __alloc_on_swap(_Alloc& __one, _Alloc& __two)
    {
      typedef allocator_traits<_Alloc> __traits;
      typedef typename __traits::propagate_on_container_swap __pocs;
#if __cplusplus >= 201703L
      if constexpr (__pocs::value)
	{
	  using std::swap;
	  swap(__one, __two);
	}
#else
      __do_alloc_on_swap(__one, __two, __pocs());
#endif
    }

  template<typename _Alloc, typename _Tp,
	   typename _ValueT = __remove_cvref_t<typename _Alloc::value_type>,
	   typename = void>
    struct __is_alloc_insertable_impl
    : false_type
    { };

  template<typename _Alloc, typename _Tp, typename _ValueT>
    struct __is_alloc_insertable_impl<_Alloc, _Tp, _ValueT,
      __void_t<decltype(allocator_traits<_Alloc>::construct(
		   std::declval<_Alloc&>(), std::declval<_ValueT*>(),
		   std::declval<_Tp>()))>>
    : true_type
    { };

  // true if _Alloc::value_type is CopyInsertable into containers using _Alloc
  // (might be wrong if _Alloc::construct exists but is not constrained,
  // i.e. actually trying to use it would still be invalid. Use with caution.)
  template<typename _Alloc>
    struct __is_copy_insertable
    : __is_alloc_insertable_impl<_Alloc,
				 typename _Alloc::value_type const&>::type
    { };

  // std::allocator<_Tp> just requires CopyConstructible
  template<typename _Tp>
    struct __is_copy_insertable<allocator<_Tp>>
    : is_copy_constructible<_Tp>
    { };

  // true if _Alloc::value_type is MoveInsertable into containers using _Alloc
  // (might be wrong if _Alloc::construct exists but is not constrained,
  // i.e. actually trying to use it would still be invalid. Use with caution.)
  template<typename _Alloc>
    struct __is_move_insertable
    : __is_alloc_insertable_impl<_Alloc, typename _Alloc::value_type>::type
    { };

  // std::allocator<_Tp> just requires MoveConstructible
  template<typename _Tp>
    struct __is_move_insertable<allocator<_Tp>>
    : is_move_constructible<_Tp>
    { };

  // Trait to detect Allocator-like types.
  template<typename _Alloc, typename = void>
    struct __is_allocator : false_type { };

  template<typename _Alloc>
    struct __is_allocator<_Alloc,
      __void_t<typename _Alloc::value_type,
	       decltype(std::declval<_Alloc&>().allocate(size_t{}))>>
    : true_type { };

  template<typename _Alloc>
    using _RequireAllocator
      = typename enable_if<__is_allocator<_Alloc>::value, _Alloc>::type;

  template<typename _Alloc>
    using _RequireNotAllocator
      = typename enable_if<!__is_allocator<_Alloc>::value, _Alloc>::type;
#endif // C++11

  /**
   * Destroy a range of objects using the supplied allocator.  For
   * non-default allocators we do not optimize away invocation of
   * destroy() even if _Tp has a trivial destructor.
   */

  template<typename _ForwardIterator, typename _Allocator>
    void
    _Destroy(_ForwardIterator __first, _ForwardIterator __last,
	     _Allocator& __alloc)
    {
      for (; __first != __last; ++__first)
#if __cplusplus < 201103L
	__alloc.destroy(std::__addressof(*__first));
#else
	allocator_traits<_Allocator>::destroy(__alloc,
					      std::__addressof(*__first));
#endif
    }

  template<typename _ForwardIterator, typename _Tp>
    inline void
    _Destroy(_ForwardIterator __first, _ForwardIterator __last,
	     allocator<_Tp>&)
    {
      _Destroy(__first, __last);
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
#endif // _ALLOC_TRAITS_H
