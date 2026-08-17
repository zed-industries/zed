// Guarded Allocation -*- C++ -*-

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

/** @file bits/allocated_ptr.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{memory}
 */

#ifndef _ALLOCATED_PTR_H
#define _ALLOCATED_PTR_H 1

#if __cplusplus < 201103L
# include <bits/c++0xwarning.h>
#else
# include <type_traits>
# include <bits/ptr_traits.h>
# include <bits/alloc_traits.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /// Non-standard RAII type for managing pointers obtained from allocators.
  template<typename _Alloc>
    struct __allocated_ptr
    {
      using pointer = typename allocator_traits<_Alloc>::pointer;
      using value_type = typename allocator_traits<_Alloc>::value_type;

      /// Take ownership of __ptr
      __allocated_ptr(_Alloc& __a, pointer __ptr) noexcept
      : _M_alloc(std::__addressof(__a)), _M_ptr(__ptr)
      { }

      /// Convert __ptr to allocator's pointer type and take ownership of it
      template<typename _Ptr,
	       typename _Req = _Require<is_same<_Ptr, value_type*>>>
      __allocated_ptr(_Alloc& __a, _Ptr __ptr)
      : _M_alloc(std::__addressof(__a)),
	_M_ptr(pointer_traits<pointer>::pointer_to(*__ptr))
      { }

      /// Transfer ownership of the owned pointer
      __allocated_ptr(__allocated_ptr&& __gd) noexcept
      : _M_alloc(__gd._M_alloc), _M_ptr(__gd._M_ptr)
      { __gd._M_ptr = nullptr; }

      /// Deallocate the owned pointer
      ~__allocated_ptr()
      {
	if (_M_ptr != nullptr)
	  std::allocator_traits<_Alloc>::deallocate(*_M_alloc, _M_ptr, 1);
      }

      /// Release ownership of the owned pointer
      __allocated_ptr&
      operator=(std::nullptr_t) noexcept
      {
	_M_ptr = nullptr;
	return *this;
      }

      /// Get the address that the owned pointer refers to.
      value_type* get() { return std::__to_address(_M_ptr); }

    private:
      _Alloc* _M_alloc;
      pointer _M_ptr;
    };

  /// Allocate space for a single object using __a
  template<typename _Alloc>
    __allocated_ptr<_Alloc>
    __allocate_guarded(_Alloc& __a)
    {
      return { __a, std::allocator_traits<_Alloc>::allocate(__a, 1) };
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif
#endif
