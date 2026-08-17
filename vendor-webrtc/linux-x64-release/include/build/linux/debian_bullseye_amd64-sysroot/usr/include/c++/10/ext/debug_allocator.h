// Allocators -*- C++ -*-

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

/*
 * Copyright (c) 1996-1997
 * Silicon Graphics Computer Systems, Inc.
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Silicon Graphics makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 */

/** @file ext/debug_allocator.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _DEBUG_ALLOCATOR_H
#define _DEBUG_ALLOCATOR_H 1

#include <stdexcept>
#include <bits/functexcept.h>
#include <ext/alloc_traits.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief  A meta-allocator with debugging bits.
   *  @ingroup allocators
   *
   *  This is precisely the allocator defined in the C++03 Standard.
   */
  template<typename _Alloc>
    class debug_allocator
    {
      template<typename> friend class debug_allocator;

      typedef __alloc_traits<_Alloc> _Traits;

    public:
      typedef typename _Traits::size_type       	size_type;
      typedef typename _Traits::difference_type	difference_type;
      typedef typename _Traits::pointer       	pointer;
      typedef typename _Traits::const_pointer    const_pointer;
      typedef typename _Traits::reference       	reference;
      typedef typename _Traits::const_reference  const_reference;
      typedef typename _Traits::value_type       value_type;

      template<typename _Up>
	class rebind
	{
	  typedef typename _Traits::template rebind<_Up>::other __other;

	public:
	  typedef debug_allocator<__other> other;
	};

    private:
      // _M_extra is the number of objects that correspond to the
      // extra space where debug information is stored.
      size_type 		_M_extra;
      
      _Alloc			_M_allocator;

      template<typename _Alloc2,
	       typename = typename __alloc_traits<_Alloc2>::template
			   rebind<value_type>::other>
	struct __convertible
	{ };

      template<typename _Alloc2>
	struct __convertible<_Alloc2, _Alloc>
	{
	  typedef void* __type;
	};

      size_type _S_extra()
      {
	const std::size_t __obj_size = sizeof(value_type);
	return (sizeof(size_type) + __obj_size - 1) / __obj_size; 
      }

    public:
      debug_allocator() : _M_extra(_S_extra()) { }

      template<typename _Alloc2>
	debug_allocator(const debug_allocator<_Alloc2>& __a2,
			typename __convertible<_Alloc2>::__type = 0)
	: _M_allocator(__a2._M_allocator), _M_extra(_S_extra()) { }

      debug_allocator(const _Alloc& __a)
      : _M_allocator(__a), _M_extra(_S_extra()) { }

      _GLIBCXX_NODISCARD pointer
      allocate(size_type __n)
      {
        pointer __res = _M_allocator.allocate(__n + _M_extra);      
	size_type* __ps = reinterpret_cast<size_type*>(__res);
	*__ps = __n;
        return __res + _M_extra;
      }

      _GLIBCXX_NODISCARD pointer
      allocate(size_type __n, const void* __hint)
      {
        pointer __res = _M_allocator.allocate(__n + _M_extra, __hint);
	size_type* __ps = reinterpret_cast<size_type*>(__res);
	*__ps = __n;
        return __res + _M_extra;
      }

      void
      deallocate(pointer __p, size_type __n)
      {
	using std::__throw_runtime_error;
	if (__p)
	  {
	    pointer __real_p = __p - _M_extra;
	    if (*reinterpret_cast<size_type*>(__real_p) != __n)
	      __throw_runtime_error("debug_allocator::deallocate wrong size");
	    _M_allocator.deallocate(__real_p, __n + _M_extra);
	  }
	else
	  __throw_runtime_error("debug_allocator::deallocate null pointer");
      }

      void
      construct(pointer __p, const value_type& __val)
      { _Traits::construct(_M_allocator, __p, __val); }

#if __cplusplus >= 201103L
      template<typename _Tp, typename... _Args>
	void
	construct(_Tp* __p, _Args&&... __args)
	{
	  _Traits::construct(_M_allocator, __p,
			     std::forward<_Args>(__args)...);
	}
#endif

      template<typename _Tp>
	void
	destroy(_Tp* __p)
	{ _Traits::destroy(_M_allocator, __p); }

      size_type
      max_size() const throw()
      { return _Traits::max_size(_M_allocator) - _M_extra; }

      template<typename _Alloc2>
	friend bool
	operator==(const debug_allocator& __lhs,
		   const debug_allocator<_Alloc2>& __rhs) _GLIBCXX_NOTHROW
	{ return __lhs._M_allocator == debug_allocator(__rhs)._M_allocator; }

#if __cpp_impl_three_way_comparison < 201907L
      template<typename _Alloc2>
	friend bool
	operator!=(const debug_allocator& __lhs,
		   const debug_allocator<_Alloc2>& __rhs) _GLIBCXX_NOTHROW
	{ return !(__lhs == __rhs); }
#endif
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
