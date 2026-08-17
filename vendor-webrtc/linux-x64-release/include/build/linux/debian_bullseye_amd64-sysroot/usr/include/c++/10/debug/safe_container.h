// Safe container implementation  -*- C++ -*-

// Copyright (C) 2014-2020 Free Software Foundation, Inc.
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

/** @file debug/safe_container.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_SAFE_CONTAINER_H
#define _GLIBCXX_DEBUG_SAFE_CONTAINER_H 1

#include <ext/alloc_traits.h>

namespace __gnu_debug
{
  /// Safe class dealing with some allocator dependent operations.
  template<typename _SafeContainer,
	   typename _Alloc,
	   template<typename> class _SafeBase,
	   bool _IsCxx11AllocatorAware = true>
    class _Safe_container
    : public _SafeBase<_SafeContainer>
    {
      typedef _SafeBase<_SafeContainer> _Base;

      _SafeContainer&
      _M_cont() _GLIBCXX_NOEXCEPT
      { return *static_cast<_SafeContainer*>(this); }

    protected:
      _Safe_container&
      _M_safe() _GLIBCXX_NOEXCEPT
      { return *this; }

#if __cplusplus >= 201103L
      _Safe_container() = default;
      _Safe_container(const _Safe_container&) = default;
      _Safe_container(_Safe_container&&) = default;

      _Safe_container(_Safe_container&& __x, const _Alloc& __a)
      : _Safe_container()
      {
	if (__x._M_cont().get_allocator() == __a)
	  _Base::_M_swap(__x);
	else
	  __x._M_invalidate_all();
      }
#endif

    public:
      // Copy assignment invalidate all iterators.
      _Safe_container&
      operator=(const _Safe_container&) _GLIBCXX_NOEXCEPT
      {
	this->_M_invalidate_all();
	return *this;
      }

#if __cplusplus >= 201103L
      _Safe_container&
      operator=(_Safe_container&& __x) noexcept
      {
	__glibcxx_check_self_move_assign(__x);

	if (_IsCxx11AllocatorAware)
	  {
	    typedef __gnu_cxx::__alloc_traits<_Alloc> _Alloc_traits;

	    bool __xfer_memory = _Alloc_traits::_S_propagate_on_move_assign()
	      || _M_cont().get_allocator() == __x._M_cont().get_allocator();
	    if (__xfer_memory)
	      _Base::_M_swap(__x);
	    else
	      this->_M_invalidate_all();
	  }
	else
	  _Base::_M_swap(__x);

	__x._M_invalidate_all();
	return *this;
      }

      void
      _M_swap(_Safe_container& __x) noexcept
      {
	if (_IsCxx11AllocatorAware)
	  {
	    typedef __gnu_cxx::__alloc_traits<_Alloc> _Alloc_traits;

	    if (!_Alloc_traits::_S_propagate_on_swap())
	      __glibcxx_check_equal_allocs(this->_M_cont()._M_base(),
					   __x._M_cont()._M_base());
	  }

	_Base::_M_swap(__x);
      }
#endif
    };

} // namespace __gnu_debug

#endif
