// Allocator that wraps "C" malloc -*- C++ -*-

// Copyright (C) 2001-2020 Free Software Foundation, Inc.
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

/** @file ext/malloc_allocator.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _MALLOC_ALLOCATOR_H
#define _MALLOC_ALLOCATOR_H 1

#include <cstdlib>
#include <cstddef>
#include <new>
#include <bits/functexcept.h>
#include <bits/move.h>
#if __cplusplus >= 201103L
#include <type_traits>
#endif

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief  An allocator that uses malloc.
   *  @ingroup allocators
   *
   *  This is precisely the allocator defined in the C++ Standard. 
   *    - all allocation calls malloc
   *    - all deallocation calls free
   */
  template<typename _Tp>
    class malloc_allocator
    {
    public:
      typedef _Tp        value_type;
      typedef std::size_t     size_type;
      typedef std::ptrdiff_t  difference_type;
#if __cplusplus <= 201703L
      typedef _Tp*       pointer;
      typedef const _Tp* const_pointer;
      typedef _Tp&       reference;
      typedef const _Tp& const_reference;

      template<typename _Tp1>
        struct rebind
        { typedef malloc_allocator<_Tp1> other; };
#endif

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2103. propagate_on_container_move_assignment
      typedef std::true_type propagate_on_container_move_assignment;
#endif

      _GLIBCXX20_CONSTEXPR
      malloc_allocator() _GLIBCXX_USE_NOEXCEPT { }

      _GLIBCXX20_CONSTEXPR
      malloc_allocator(const malloc_allocator&) _GLIBCXX_USE_NOEXCEPT { }

      template<typename _Tp1>
	_GLIBCXX20_CONSTEXPR
        malloc_allocator(const malloc_allocator<_Tp1>&)
	_GLIBCXX_USE_NOEXCEPT { }

#if __cplusplus <= 201703L
      ~malloc_allocator() _GLIBCXX_USE_NOEXCEPT { }

      pointer
      address(reference __x) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__x); }

      const_pointer
      address(const_reference __x) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__x); }
#endif

      // NB: __n is permitted to be 0.  The C++ standard says nothing
      // about what the return value is when __n == 0.
      _Tp*
      allocate(size_type __n, const void* = 0)
      {
	if (__n > this->_M_max_size())
	  std::__throw_bad_alloc();

	_Tp* __ret = 0;
#if __cpp_aligned_new
#if __cplusplus > 201402L && _GLIBCXX_HAVE_ALIGNED_ALLOC
	if (alignof(_Tp) > alignof(std::max_align_t))
	  {
	    __ret = static_cast<_Tp*>(::aligned_alloc(alignof(_Tp),
						      __n * sizeof(_Tp)));
	  }
#else
# define _GLIBCXX_CHECK_MALLOC_RESULT
#endif
#endif
	if (!__ret)
	  __ret = static_cast<_Tp*>(std::malloc(__n * sizeof(_Tp)));
	if (!__ret)
	  std::__throw_bad_alloc();
#ifdef _GLIBCXX_CHECK_MALLOC_RESULT
#undef _GLIBCXX_CHECK_MALLOC_RESULT
	  if (reinterpret_cast<std::size_t>(__ret) % alignof(_Tp))
	    {
	      // Memory returned by malloc is not suitably aligned for _Tp.
	      deallocate(__ret, __n);
	      std::__throw_bad_alloc();
	    }
#endif
	return __ret;
      }

      // __p is not permitted to be a null pointer.
      void
      deallocate(_Tp* __p, size_type)
      { std::free(static_cast<void*>(__p)); }

#if __cplusplus <= 201703L
      size_type
      max_size() const _GLIBCXX_USE_NOEXCEPT 
      { return _M_max_size(); }

#if __cplusplus >= 201103L
      template<typename _Up, typename... _Args>
        void
        construct(_Up* __p, _Args&&... __args)
	noexcept(std::is_nothrow_constructible<_Up, _Args...>::value)
	{ ::new((void *)__p) _Up(std::forward<_Args>(__args)...); }

      template<typename _Up>
        void 
        destroy(_Up* __p)
	noexcept(std::is_nothrow_destructible<_Up>::value)
	{ __p->~_Up(); }
#else
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 402. wrong new expression in [some_] allocator::construct
      void
      construct(pointer __p, const _Tp& __val)
      { ::new((void *)__p) value_type(__val); }

      void
      destroy(pointer __p) { __p->~_Tp(); }
#endif
#endif // ! C++20

      template<typename _Up>
	friend _GLIBCXX20_CONSTEXPR bool
	operator==(const malloc_allocator&, const malloc_allocator<_Up>&)
	_GLIBCXX_NOTHROW
	{ return true; }

#if __cpp_impl_three_way_comparison < 201907L
      template<typename _Up>
	friend _GLIBCXX20_CONSTEXPR bool
	operator!=(const malloc_allocator&, const malloc_allocator<_Up>&)
	_GLIBCXX_NOTHROW
	{ return false; }
#endif

    private:
      _GLIBCXX_CONSTEXPR size_type
      _M_max_size() const _GLIBCXX_USE_NOEXCEPT
      {
#if __PTRDIFF_MAX__ < __SIZE_MAX__
	return std::size_t(__PTRDIFF_MAX__) / sizeof(_Tp);
#else
	return std::size_t(-1) / sizeof(_Tp);
#endif
      }
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
