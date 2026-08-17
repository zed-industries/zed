// shared_ptr and weak_ptr implementation -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
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

// GCC Note: Based on files from version 1.32.0 of the Boost library.

//  shared_count.hpp
//  Copyright (c) 2001, 2002, 2003 Peter Dimov and Multi Media Ltd.

//  shared_ptr.hpp
//  Copyright (C) 1998, 1999 Greg Colvin and Beman Dawes.
//  Copyright (C) 2001, 2002, 2003 Peter Dimov

//  weak_ptr.hpp
//  Copyright (C) 2001, 2002, 2003 Peter Dimov

//  enable_shared_from_this.hpp
//  Copyright (C) 2002 Peter Dimov

// Distributed under the Boost Software License, Version 1.0. (See
// accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

/** @file
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{memory}
 */

#ifndef _SHARED_PTR_H
#define _SHARED_PTR_H 1

#include <bits/shared_ptr_base.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @addtogroup pointer_abstractions
   * @{
   */

  // 20.7.2.2.11 shared_ptr I/O

  /// Write the stored pointer to an ostream.
  /// @relates shared_ptr
  template<typename _Ch, typename _Tr, typename _Tp, _Lock_policy _Lp>
    inline std::basic_ostream<_Ch, _Tr>&
    operator<<(std::basic_ostream<_Ch, _Tr>& __os,
	       const __shared_ptr<_Tp, _Lp>& __p)
    {
      __os << __p.get();
      return __os;
    }

  template<typename _Del, typename _Tp, _Lock_policy _Lp>
    inline _Del*
    get_deleter(const __shared_ptr<_Tp, _Lp>& __p) noexcept
    {
#if __cpp_rtti
      return static_cast<_Del*>(__p._M_get_deleter(typeid(_Del)));
#else
      return 0;
#endif
    }

  /// 20.7.2.2.10 shared_ptr get_deleter

  /// If `__p` has a deleter of type `_Del`, return a pointer to it.
  /// @relates shared_ptr
  template<typename _Del, typename _Tp>
    inline _Del*
    get_deleter(const shared_ptr<_Tp>& __p) noexcept
    {
#if __cpp_rtti
      return static_cast<_Del*>(__p._M_get_deleter(typeid(_Del)));
#else
      return 0;
#endif
    }

  /**
   *  @brief  A smart pointer with reference-counted copy semantics.
   *
   * A `shared_ptr` object is either empty or _owns_ a pointer passed
   * to the constructor. Copies of a `shared_ptr` share ownership of
   * the same pointer. When the last `shared_ptr` that owns the pointer
   * is destroyed or reset, the owned pointer is freed (either by `delete`
   * or by invoking a custom deleter that was passed to the constructor).
   *
   * A `shared_ptr` also stores another pointer, which is usually
   * (but not always) the same pointer as it owns. The stored pointer
   * can be retrieved by calling the `get()` member function.
   *
   * The equality and relational operators for `shared_ptr` only compare
   * the stored pointer returned by `get()`, not the owned pointer.
   * To test whether two `shared_ptr` objects share ownership of the same
   * pointer see `std::shared_ptr::owner_before` and `std::owner_less`.
  */
  template<typename _Tp>
    class shared_ptr : public __shared_ptr<_Tp>
    {
      template<typename... _Args>
	using _Constructible = typename enable_if<
	  is_constructible<__shared_ptr<_Tp>, _Args...>::value
	>::type;

      template<typename _Arg>
	using _Assignable = typename enable_if<
	  is_assignable<__shared_ptr<_Tp>&, _Arg>::value, shared_ptr&
	>::type;

    public:

      /// The type pointed to by the stored pointer, remove_extent_t<_Tp>
      using element_type = typename __shared_ptr<_Tp>::element_type;

#if __cplusplus >= 201703L
# define __cpp_lib_shared_ptr_weak_type 201606
      /// The corresponding weak_ptr type for this shared_ptr
      using weak_type = weak_ptr<_Tp>;
#endif
      /**
       *  @brief  Construct an empty %shared_ptr.
       *  @post   use_count()==0 && get()==0
       */
      constexpr shared_ptr() noexcept : __shared_ptr<_Tp>() { }

      shared_ptr(const shared_ptr&) noexcept = default; ///< Copy constructor

      /**
       *  @brief  Construct a %shared_ptr that owns the pointer @a __p.
       *  @param  __p  A pointer that is convertible to element_type*.
       *  @post   use_count() == 1 && get() == __p
       *  @throw  std::bad_alloc, in which case @c delete @a __p is called.
       */
      template<typename _Yp, typename = _Constructible<_Yp*>>
	explicit
	shared_ptr(_Yp* __p) : __shared_ptr<_Tp>(__p) { }

      /**
       *  @brief  Construct a %shared_ptr that owns the pointer @a __p
       *          and the deleter @a __d.
       *  @param  __p  A pointer.
       *  @param  __d  A deleter.
       *  @post   use_count() == 1 && get() == __p
       *  @throw  std::bad_alloc, in which case @a __d(__p) is called.
       *
       *  Requirements: _Deleter's copy constructor and destructor must
       *  not throw
       *
       *  __shared_ptr will release __p by calling __d(__p)
       */
      template<typename _Yp, typename _Deleter,
	       typename = _Constructible<_Yp*, _Deleter>>
	shared_ptr(_Yp* __p, _Deleter __d)
        : __shared_ptr<_Tp>(__p, std::move(__d)) { }

      /**
       *  @brief  Construct a %shared_ptr that owns a null pointer
       *          and the deleter @a __d.
       *  @param  __p  A null pointer constant.
       *  @param  __d  A deleter.
       *  @post   use_count() == 1 && get() == __p
       *  @throw  std::bad_alloc, in which case @a __d(__p) is called.
       *
       *  Requirements: _Deleter's copy constructor and destructor must
       *  not throw
       *
       *  The last owner will call __d(__p)
       */
      template<typename _Deleter>
	shared_ptr(nullptr_t __p, _Deleter __d)
        : __shared_ptr<_Tp>(__p, std::move(__d)) { }

      /**
       *  @brief  Construct a %shared_ptr that owns the pointer @a __p
       *          and the deleter @a __d.
       *  @param  __p  A pointer.
       *  @param  __d  A deleter.
       *  @param  __a  An allocator.
       *  @post   use_count() == 1 && get() == __p
       *  @throw  std::bad_alloc, in which case @a __d(__p) is called.
       *
       *  Requirements: _Deleter's copy constructor and destructor must
       *  not throw _Alloc's copy constructor and destructor must not
       *  throw.
       *
       *  __shared_ptr will release __p by calling __d(__p)
       */
      template<typename _Yp, typename _Deleter, typename _Alloc,
	       typename = _Constructible<_Yp*, _Deleter, _Alloc>>
	shared_ptr(_Yp* __p, _Deleter __d, _Alloc __a)
	: __shared_ptr<_Tp>(__p, std::move(__d), std::move(__a)) { }

      /**
       *  @brief  Construct a %shared_ptr that owns a null pointer
       *          and the deleter @a __d.
       *  @param  __p  A null pointer constant.
       *  @param  __d  A deleter.
       *  @param  __a  An allocator.
       *  @post   use_count() == 1 && get() == __p
       *  @throw  std::bad_alloc, in which case @a __d(__p) is called.
       *
       *  Requirements: _Deleter's copy constructor and destructor must
       *  not throw _Alloc's copy constructor and destructor must not
       *  throw.
       *
       *  The last owner will call __d(__p)
       */
      template<typename _Deleter, typename _Alloc>
	shared_ptr(nullptr_t __p, _Deleter __d, _Alloc __a)
	: __shared_ptr<_Tp>(__p, std::move(__d), std::move(__a)) { }

      // Aliasing constructor

      /**
       *  @brief  Constructs a `shared_ptr` instance that stores `__p`
       *          and shares ownership with `__r`.
       *  @param  __r  A `shared_ptr`.
       *  @param  __p  A pointer that will remain valid while `*__r` is valid.
       *  @post   `get() == __p && use_count() == __r.use_count()`
       *
       *  This can be used to construct a `shared_ptr` to a sub-object
       *  of an object managed by an existing `shared_ptr`. The complete
       *  object will remain valid while any `shared_ptr` owns it, even
       *  if they don't store a pointer to the complete object.
       *
       * @code
       * shared_ptr<pair<int,int>> pii(new pair<int,int>());
       * shared_ptr<int> pi(pii, &pii->first);
       * assert(pii.use_count() == 2);
       * @endcode
       */
      template<typename _Yp>
	shared_ptr(const shared_ptr<_Yp>& __r, element_type* __p) noexcept
	: __shared_ptr<_Tp>(__r, __p) { }

#if __cplusplus > 201703L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2996. Missing rvalue overloads for shared_ptr operations
      /**
       *  @brief  Constructs a `shared_ptr` instance that stores `__p`
       *          and shares ownership with `__r`.
       *  @param  __r  A `shared_ptr`.
       *  @param  __p  A pointer that will remain valid while `*__r` is valid.
       *  @post   `get() == __p && !__r.use_count() && !__r.get()`
       *
       *  This can be used to construct a `shared_ptr` to a sub-object
       *  of an object managed by an existing `shared_ptr`. The complete
       *  object will remain valid while any `shared_ptr` owns it, even
       *  if they don't store a pointer to the complete object.
       *
       * @code
       * shared_ptr<pair<int,int>> pii(new pair<int,int>());
       * shared_ptr<int> pi1(pii, &pii->first);
       * assert(pii.use_count() == 2);
       * shared_ptr<int> pi2(std::move(pii), &pii->second);
       * assert(pii.use_count() == 0);
       * @endcode
       */
      template<typename _Yp>
	shared_ptr(shared_ptr<_Yp>&& __r, element_type* __p) noexcept
	: __shared_ptr<_Tp>(std::move(__r), __p) { }
#endif
      /**
       *  @brief  If @a __r is empty, constructs an empty %shared_ptr;
       *          otherwise construct a %shared_ptr that shares ownership
       *          with @a __r.
       *  @param  __r  A %shared_ptr.
       *  @post   get() == __r.get() && use_count() == __r.use_count()
       */
      template<typename _Yp,
	       typename = _Constructible<const shared_ptr<_Yp>&>>
	shared_ptr(const shared_ptr<_Yp>& __r) noexcept
        : __shared_ptr<_Tp>(__r) { }

      /**
       *  @brief  Move-constructs a %shared_ptr instance from @a __r.
       *  @param  __r  A %shared_ptr rvalue.
       *  @post   *this contains the old value of @a __r, @a __r is empty.
       */
      shared_ptr(shared_ptr&& __r) noexcept
      : __shared_ptr<_Tp>(std::move(__r)) { }

      /**
       *  @brief  Move-constructs a %shared_ptr instance from @a __r.
       *  @param  __r  A %shared_ptr rvalue.
       *  @post   *this contains the old value of @a __r, @a __r is empty.
       */
      template<typename _Yp, typename = _Constructible<shared_ptr<_Yp>>>
	shared_ptr(shared_ptr<_Yp>&& __r) noexcept
	: __shared_ptr<_Tp>(std::move(__r)) { }

      /**
       *  @brief  Constructs a %shared_ptr that shares ownership with @a __r
       *          and stores a copy of the pointer stored in @a __r.
       *  @param  __r  A weak_ptr.
       *  @post   use_count() == __r.use_count()
       *  @throw  bad_weak_ptr when __r.expired(),
       *          in which case the constructor has no effect.
       */
      template<typename _Yp, typename = _Constructible<const weak_ptr<_Yp>&>>
	explicit shared_ptr(const weak_ptr<_Yp>& __r)
	: __shared_ptr<_Tp>(__r) { }

#if _GLIBCXX_USE_DEPRECATED
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
      template<typename _Yp, typename = _Constructible<auto_ptr<_Yp>>>
	shared_ptr(auto_ptr<_Yp>&& __r);
#pragma GCC diagnostic pop
#endif

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2399. shared_ptr's constructor from unique_ptr should be constrained
      template<typename _Yp, typename _Del,
	       typename = _Constructible<unique_ptr<_Yp, _Del>>>
	shared_ptr(unique_ptr<_Yp, _Del>&& __r)
	: __shared_ptr<_Tp>(std::move(__r)) { }

#if __cplusplus <= 201402L && _GLIBCXX_USE_DEPRECATED
      // This non-standard constructor exists to support conversions that
      // were possible in C++11 and C++14 but are ill-formed in C++17.
      // If an exception is thrown this constructor has no effect.
      template<typename _Yp, typename _Del,
		_Constructible<unique_ptr<_Yp, _Del>, __sp_array_delete>* = 0>
	shared_ptr(unique_ptr<_Yp, _Del>&& __r)
	: __shared_ptr<_Tp>(std::move(__r), __sp_array_delete()) { }
#endif

      /**
       *  @brief  Construct an empty %shared_ptr.
       *  @post   use_count() == 0 && get() == nullptr
       */
      constexpr shared_ptr(nullptr_t) noexcept : shared_ptr() { }

      shared_ptr& operator=(const shared_ptr&) noexcept = default;

      template<typename _Yp>
	_Assignable<const shared_ptr<_Yp>&>
	operator=(const shared_ptr<_Yp>& __r) noexcept
	{
	  this->__shared_ptr<_Tp>::operator=(__r);
	  return *this;
	}

#if _GLIBCXX_USE_DEPRECATED
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
      template<typename _Yp>
	_Assignable<auto_ptr<_Yp>>
	operator=(auto_ptr<_Yp>&& __r)
	{
	  this->__shared_ptr<_Tp>::operator=(std::move(__r));
	  return *this;
	}
#pragma GCC diagnostic pop
#endif

      shared_ptr&
      operator=(shared_ptr&& __r) noexcept
      {
	this->__shared_ptr<_Tp>::operator=(std::move(__r));
	return *this;
      }

      template<class _Yp>
	_Assignable<shared_ptr<_Yp>>
	operator=(shared_ptr<_Yp>&& __r) noexcept
	{
	  this->__shared_ptr<_Tp>::operator=(std::move(__r));
	  return *this;
	}

      template<typename _Yp, typename _Del>
	_Assignable<unique_ptr<_Yp, _Del>>
	operator=(unique_ptr<_Yp, _Del>&& __r)
	{
	  this->__shared_ptr<_Tp>::operator=(std::move(__r));
	  return *this;
	}

    private:
      // This constructor is non-standard, it is used by allocate_shared.
      template<typename _Alloc, typename... _Args>
	shared_ptr(_Sp_alloc_shared_tag<_Alloc> __tag, _Args&&... __args)
	: __shared_ptr<_Tp>(__tag, std::forward<_Args>(__args)...)
	{ }

      template<typename _Yp, typename _Alloc, typename... _Args>
	friend shared_ptr<_Yp>
	allocate_shared(const _Alloc& __a, _Args&&... __args);

      // This constructor is non-standard, it is used by weak_ptr::lock().
      shared_ptr(const weak_ptr<_Tp>& __r, std::nothrow_t)
      : __shared_ptr<_Tp>(__r, std::nothrow) { }

      friend class weak_ptr<_Tp>;
    };

#if __cpp_deduction_guides >= 201606
  template<typename _Tp>
    shared_ptr(weak_ptr<_Tp>) ->  shared_ptr<_Tp>;
  template<typename _Tp, typename _Del>
    shared_ptr(unique_ptr<_Tp, _Del>) ->  shared_ptr<_Tp>;
#endif

  // 20.7.2.2.7 shared_ptr comparisons

  /// @relates shared_ptr @{

  /// Equality operator for shared_ptr objects, compares the stored pointers
  template<typename _Tp, typename _Up>
    _GLIBCXX_NODISCARD inline bool
    operator==(const shared_ptr<_Tp>& __a, const shared_ptr<_Up>& __b) noexcept
    { return __a.get() == __b.get(); }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator==(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return !__a; }

#ifdef __cpp_lib_three_way_comparison
  template<typename _Tp, typename _Up>
    inline strong_ordering
    operator<=>(const shared_ptr<_Tp>& __a,
		const shared_ptr<_Up>& __b) noexcept
    { return compare_three_way()(__a.get(), __b.get()); }

  template<typename _Tp>
    inline strong_ordering
    operator<=>(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    {
      using pointer = typename shared_ptr<_Tp>::element_type*;
      return compare_three_way()(__a.get(), static_cast<pointer>(nullptr));
    }
#else
  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator==(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return !__a; }

  /// Inequality operator for shared_ptr objects, compares the stored pointers
  template<typename _Tp, typename _Up>
    _GLIBCXX_NODISCARD inline bool
    operator!=(const shared_ptr<_Tp>& __a, const shared_ptr<_Up>& __b) noexcept
    { return __a.get() != __b.get(); }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator!=(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return (bool)__a; }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator!=(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return (bool)__a; }

  /// Relational operator for shared_ptr objects, compares the stored pointers
  template<typename _Tp, typename _Up>
    _GLIBCXX_NODISCARD inline bool
    operator<(const shared_ptr<_Tp>& __a, const shared_ptr<_Up>& __b) noexcept
    {
      using _Tp_elt = typename shared_ptr<_Tp>::element_type;
      using _Up_elt = typename shared_ptr<_Up>::element_type;
      using _Vp = typename common_type<_Tp_elt*, _Up_elt*>::type;
      return less<_Vp>()(__a.get(), __b.get());
    }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator<(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    {
      using _Tp_elt = typename shared_ptr<_Tp>::element_type;
      return less<_Tp_elt*>()(__a.get(), nullptr);
    }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator<(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    {
      using _Tp_elt = typename shared_ptr<_Tp>::element_type;
      return less<_Tp_elt*>()(nullptr, __a.get());
    }

  /// Relational operator for shared_ptr objects, compares the stored pointers
  template<typename _Tp, typename _Up>
    _GLIBCXX_NODISCARD inline bool
    operator<=(const shared_ptr<_Tp>& __a, const shared_ptr<_Up>& __b) noexcept
    { return !(__b < __a); }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator<=(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return !(nullptr < __a); }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator<=(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return !(__a < nullptr); }

  /// Relational operator for shared_ptr objects, compares the stored pointers
  template<typename _Tp, typename _Up>
    _GLIBCXX_NODISCARD inline bool
    operator>(const shared_ptr<_Tp>& __a, const shared_ptr<_Up>& __b) noexcept
    { return (__b < __a); }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator>(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return nullptr < __a; }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator>(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return __a < nullptr; }

  /// Relational operator for shared_ptr objects, compares the stored pointers
  template<typename _Tp, typename _Up>
    _GLIBCXX_NODISCARD inline bool
    operator>=(const shared_ptr<_Tp>& __a, const shared_ptr<_Up>& __b) noexcept
    { return !(__a < __b); }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator>=(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return !(__a < nullptr); }

  /// shared_ptr comparison with nullptr
  template<typename _Tp>
    _GLIBCXX_NODISCARD inline bool
    operator>=(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return !(nullptr < __a); }
#endif

  // 20.7.2.2.8 shared_ptr specialized algorithms.

  /// Swap overload for shared_ptr
  template<typename _Tp>
    inline void
    swap(shared_ptr<_Tp>& __a, shared_ptr<_Tp>& __b) noexcept
    { __a.swap(__b); }

  // 20.7.2.2.9 shared_ptr casts.

  /// Convert type of `shared_ptr`, via `static_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    static_pointer_cast(const shared_ptr<_Up>& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      return _Sp(__r, static_cast<typename _Sp::element_type*>(__r.get()));
    }

  /// Convert type of `shared_ptr`, via `const_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    const_pointer_cast(const shared_ptr<_Up>& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      return _Sp(__r, const_cast<typename _Sp::element_type*>(__r.get()));
    }

  /// Convert type of `shared_ptr`, via `dynamic_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    dynamic_pointer_cast(const shared_ptr<_Up>& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      if (auto* __p = dynamic_cast<typename _Sp::element_type*>(__r.get()))
	return _Sp(__r, __p);
      return _Sp();
    }

#if __cplusplus >= 201703L
  /// Convert type of `shared_ptr`, via `reinterpret_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    reinterpret_pointer_cast(const shared_ptr<_Up>& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      return _Sp(__r, reinterpret_cast<typename _Sp::element_type*>(__r.get()));
    }

#if __cplusplus > 201703L
  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // 2996. Missing rvalue overloads for shared_ptr operations

  /// Convert type of `shared_ptr` rvalue, via `static_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    static_pointer_cast(shared_ptr<_Up>&& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      return _Sp(std::move(__r),
		 static_cast<typename _Sp::element_type*>(__r.get()));
    }

  /// Convert type of `shared_ptr` rvalue, via `const_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    const_pointer_cast(shared_ptr<_Up>&& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      return _Sp(std::move(__r),
		 const_cast<typename _Sp::element_type*>(__r.get()));
    }

  /// Convert type of `shared_ptr` rvalue, via `dynamic_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    dynamic_pointer_cast(shared_ptr<_Up>&& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      if (auto* __p = dynamic_cast<typename _Sp::element_type*>(__r.get()))
	return _Sp(std::move(__r), __p);
      return _Sp();
    }

  /// Convert type of `shared_ptr` rvalue, via `reinterpret_cast`
  template<typename _Tp, typename _Up>
    inline shared_ptr<_Tp>
    reinterpret_pointer_cast(shared_ptr<_Up>&& __r) noexcept
    {
      using _Sp = shared_ptr<_Tp>;
      return _Sp(std::move(__r),
		 reinterpret_cast<typename _Sp::element_type*>(__r.get()));
    }
#endif // C++20
#endif // C++17

  // @}

  /**
   * @brief  A non-owning observer for a pointer owned by a shared_ptr
   *
   * A weak_ptr provides a safe alternative to a raw pointer when you want
   * a non-owning reference to an object that is managed by a shared_ptr.
   *
   * Unlike a raw pointer, a weak_ptr can be converted to a new shared_ptr
   * that shares ownership with every other shared_ptr that already owns
   * the pointer. In other words you can upgrade from a non-owning "weak"
   * reference to an owning shared_ptr, without having access to any of
   * the existing shared_ptr objects.
   *
   * Also unlike a raw pointer, a weak_ptr does not become "dangling" after
   * the object it points to has been destroyed. Instead, a weak_ptr
   * becomes _expired_ and can no longer be converted to a shared_ptr that
   * owns the freed pointer, so you cannot accidentally access the pointed-to
   * object after it has been destroyed.
   */
  template<typename _Tp>
    class weak_ptr : public __weak_ptr<_Tp>
    {
      template<typename _Arg>
	using _Constructible = typename enable_if<
	  is_constructible<__weak_ptr<_Tp>, _Arg>::value
	>::type;

      template<typename _Arg>
	using _Assignable = typename enable_if<
	  is_assignable<__weak_ptr<_Tp>&, _Arg>::value, weak_ptr&
	>::type;

    public:
      constexpr weak_ptr() noexcept = default;

      template<typename _Yp,
	       typename = _Constructible<const shared_ptr<_Yp>&>>
	weak_ptr(const shared_ptr<_Yp>& __r) noexcept
	: __weak_ptr<_Tp>(__r) { }

      weak_ptr(const weak_ptr&) noexcept = default;

      template<typename _Yp, typename = _Constructible<const weak_ptr<_Yp>&>>
	weak_ptr(const weak_ptr<_Yp>& __r) noexcept
	: __weak_ptr<_Tp>(__r) { }

      weak_ptr(weak_ptr&&) noexcept = default;

      template<typename _Yp, typename = _Constructible<weak_ptr<_Yp>>>
	weak_ptr(weak_ptr<_Yp>&& __r) noexcept
	: __weak_ptr<_Tp>(std::move(__r)) { }

      weak_ptr&
      operator=(const weak_ptr& __r) noexcept = default;

      template<typename _Yp>
	_Assignable<const weak_ptr<_Yp>&>
	operator=(const weak_ptr<_Yp>& __r) noexcept
	{
	  this->__weak_ptr<_Tp>::operator=(__r);
	  return *this;
	}

      template<typename _Yp>
	_Assignable<const shared_ptr<_Yp>&>
	operator=(const shared_ptr<_Yp>& __r) noexcept
	{
	  this->__weak_ptr<_Tp>::operator=(__r);
	  return *this;
	}

      weak_ptr&
      operator=(weak_ptr&& __r) noexcept = default;

      template<typename _Yp>
	_Assignable<weak_ptr<_Yp>>
	operator=(weak_ptr<_Yp>&& __r) noexcept
	{
	  this->__weak_ptr<_Tp>::operator=(std::move(__r));
	  return *this;
	}

      shared_ptr<_Tp>
      lock() const noexcept
      { return shared_ptr<_Tp>(*this, std::nothrow); }
    };

#if __cpp_deduction_guides >= 201606
  template<typename _Tp>
    weak_ptr(shared_ptr<_Tp>) ->  weak_ptr<_Tp>;
#endif

  // 20.7.2.3.6 weak_ptr specialized algorithms.
  /// Swap overload for weak_ptr
  /// @relates weak_ptr
  template<typename _Tp>
    inline void
    swap(weak_ptr<_Tp>& __a, weak_ptr<_Tp>& __b) noexcept
    { __a.swap(__b); }


  /// Primary template owner_less
  template<typename _Tp = void>
    struct owner_less;

  /// Void specialization of owner_less compares either shared_ptr or weak_ptr
  template<>
    struct owner_less<void> : _Sp_owner_less<void, void>
    { };

  /// Partial specialization of owner_less for shared_ptr.
  template<typename _Tp>
    struct owner_less<shared_ptr<_Tp>>
    : public _Sp_owner_less<shared_ptr<_Tp>, weak_ptr<_Tp>>
    { };

  /// Partial specialization of owner_less for weak_ptr.
  template<typename _Tp>
    struct owner_less<weak_ptr<_Tp>>
    : public _Sp_owner_less<weak_ptr<_Tp>, shared_ptr<_Tp>>
    { };

  /**
   *  @brief Base class allowing use of member function shared_from_this.
   */
  template<typename _Tp>
    class enable_shared_from_this
    {
    protected:
      constexpr enable_shared_from_this() noexcept { }

      enable_shared_from_this(const enable_shared_from_this&) noexcept { }

      enable_shared_from_this&
      operator=(const enable_shared_from_this&) noexcept
      { return *this; }

      ~enable_shared_from_this() { }

    public:
      shared_ptr<_Tp>
      shared_from_this()
      { return shared_ptr<_Tp>(this->_M_weak_this); }

      shared_ptr<const _Tp>
      shared_from_this() const
      { return shared_ptr<const _Tp>(this->_M_weak_this); }

#if __cplusplus > 201402L || !defined(__STRICT_ANSI__) // c++1z or gnu++11
#define __cpp_lib_enable_shared_from_this 201603
      weak_ptr<_Tp>
      weak_from_this() noexcept
      { return this->_M_weak_this; }

      weak_ptr<const _Tp>
      weak_from_this() const noexcept
      { return this->_M_weak_this; }
#endif

    private:
      template<typename _Tp1>
	void
	_M_weak_assign(_Tp1* __p, const __shared_count<>& __n) const noexcept
	{ _M_weak_this._M_assign(__p, __n); }

      // Found by ADL when this is an associated class.
      friend const enable_shared_from_this*
      __enable_shared_from_this_base(const __shared_count<>&,
				     const enable_shared_from_this* __p)
      { return __p; }

      template<typename, _Lock_policy>
	friend class __shared_ptr;

      mutable weak_ptr<_Tp>  _M_weak_this;
    };

  /// @relates shared_ptr @{

  /**
   *  @brief  Create an object that is owned by a shared_ptr.
   *  @param  __a     An allocator.
   *  @param  __args  Arguments for the @a _Tp object's constructor.
   *  @return A shared_ptr that owns the newly created object.
   *  @throw  An exception thrown from @a _Alloc::allocate or from the
   *          constructor of @a _Tp.
   *
   *  A copy of @a __a will be used to allocate memory for the shared_ptr
   *  and the new object.
   */
  template<typename _Tp, typename _Alloc, typename... _Args>
    inline shared_ptr<_Tp>
    allocate_shared(const _Alloc& __a, _Args&&... __args)
    {
      return shared_ptr<_Tp>(_Sp_alloc_shared_tag<_Alloc>{__a},
			     std::forward<_Args>(__args)...);
    }

  /**
   *  @brief  Create an object that is owned by a shared_ptr.
   *  @param  __args  Arguments for the @a _Tp object's constructor.
   *  @return A shared_ptr that owns the newly created object.
   *  @throw  std::bad_alloc, or an exception thrown from the
   *          constructor of @a _Tp.
   */
  template<typename _Tp, typename... _Args>
    inline shared_ptr<_Tp>
    make_shared(_Args&&... __args)
    {
      typedef typename std::remove_cv<_Tp>::type _Tp_nc;
      return std::allocate_shared<_Tp>(std::allocator<_Tp_nc>(),
				       std::forward<_Args>(__args)...);
    }

  /// std::hash specialization for shared_ptr.
  template<typename _Tp>
    struct hash<shared_ptr<_Tp>>
    : public __hash_base<size_t, shared_ptr<_Tp>>
    {
      size_t
      operator()(const shared_ptr<_Tp>& __s) const noexcept
      {
	return std::hash<typename shared_ptr<_Tp>::element_type*>()(__s.get());
      }
    };

  // @} relates shared_ptr
  // @} group pointer_abstractions

#if __cplusplus >= 201703L
  namespace __detail::__variant
  {
    template<typename> struct _Never_valueless_alt; // see <variant>

    // Provide the strong exception-safety guarantee when emplacing a
    // shared_ptr into a variant.
    template<typename _Tp>
      struct _Never_valueless_alt<std::shared_ptr<_Tp>>
      : std::true_type
      { };

    // Provide the strong exception-safety guarantee when emplacing a
    // weak_ptr into a variant.
    template<typename _Tp>
      struct _Never_valueless_alt<std::weak_ptr<_Tp>>
      : std::true_type
      { };
  }  // namespace __detail::__variant
#endif // C++17

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif // _SHARED_PTR_H
