// Safe container implementation  -*- C++ -*-

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

/** @file debug/safe_unordered_container.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_SAFE_UNORDERED_CONTAINER_H
#define _GLIBCXX_DEBUG_SAFE_UNORDERED_CONTAINER_H 1

#include <debug/assertions.h>
#include <debug/macros.h>
#include <debug/functions.h>
#include <debug/safe_unordered_base.h>

namespace __gnu_debug
{
  /**
   * @brief Base class for constructing a @a safe unordered container type
   * that tracks iterators that reference it.
   *
   * The class template %_Safe_unordered_container simplifies the
   * construction of @a safe unordered containers that track the iterators
   * that reference the container, so that the iterators are notified of
   * changes in the container that may affect their operation, e.g., if
   * the container invalidates its iterators or is destructed. This class
   * template may only be used by deriving from it and passing the name
   * of the derived class as its template parameter via the curiously
   * recurring template pattern. The derived class must have @c
   * iterator and @c const_iterator types that are instantiations of
   * class template _Safe_iterator for this container and @c local_iterator
   * and @c const_local_iterator types that are instantiations of class
   * template _Safe_local_iterator for this container. Iterators will
   * then be tracked automatically.
   */
  template<typename _Container>
    class _Safe_unordered_container : public _Safe_unordered_container_base
    {
    private:
      _Container&
      _M_cont() noexcept
      { return *static_cast<_Container*>(this); }

    protected:
      void
      _M_invalidate_locals()
      {
	auto __local_end = _M_cont()._M_base().cend(0);
	this->_M_invalidate_local_if(
		[__local_end](__decltype(__local_end) __it)
		{ return __it != __local_end; });
      }

      void
      _M_invalidate_all()
      {
	auto __end = _M_cont()._M_base().cend();
	this->_M_invalidate_if([__end](__decltype(__end) __it)
			       { return __it != __end; });
	_M_invalidate_locals();
      }

      /** Invalidates all iterators @c x that reference this container,
	  are not singular, and for which @c __pred(x) returns @c
	  true. @c __pred will be invoked with the normal iterators nested
	  in the safe ones. */
      template<typename _Predicate>
	void
	_M_invalidate_if(_Predicate __pred);

      /** Invalidates all local iterators @c x that reference this container,
	  are not singular, and for which @c __pred(x) returns @c
	  true. @c __pred will be invoked with the normal local iterators
	  nested in the safe ones. */
      template<typename _Predicate>
	void
	_M_invalidate_local_if(_Predicate __pred);
    };
} // namespace __gnu_debug

#include <debug/safe_unordered_container.tcc>

#endif
