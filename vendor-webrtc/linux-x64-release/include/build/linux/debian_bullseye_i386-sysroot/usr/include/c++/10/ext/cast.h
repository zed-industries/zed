// <cast.h> -*- C++ -*-

// Copyright (C) 2008-2020 Free Software Foundation, Inc.
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

/** @file ext/cast.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{ext/pointer.h}
 */

#ifndef _GLIBCXX_CAST_H
#define _GLIBCXX_CAST_H 1

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * These functions are here to allow containers to support non standard
   * pointer types.  For normal pointers, these resolve to the use of the
   * standard cast operation.  For other types the functions will perform
   * the appropriate cast to/from the custom pointer class so long as that
   * class meets the following conditions:
   * 1) has a typedef element_type which names tehe type it points to.
   * 2) has a get() const method which returns element_type*.
   * 3) has a constructor which can take one element_type* argument.
   */

  /**
   * This type supports the semantics of the pointer cast operators (below.)
   */
  template<typename _ToType>
    struct _Caster
    { typedef typename _ToType::element_type*  type; };

  template<typename _ToType>
    struct _Caster<_ToType*>
    { typedef _ToType*  type; };

  /**
   * Casting operations for cases where _FromType is not a standard pointer.
   * _ToType can be a standard or non-standard pointer.  Given that _FromType
   * is not a pointer, it must have a get() method that returns the standard
   * pointer equivalent of the address it points to, and must have an
   * element_type typedef which names the type it points to.
   */
  template<typename _ToType, typename _FromType>
    inline _ToType
    __static_pointer_cast(const _FromType& __arg)
    { return _ToType(static_cast<typename _Caster<_ToType>::
		     type>(__arg.get())); }

  template<typename _ToType, typename _FromType>
    inline _ToType
    __dynamic_pointer_cast(const _FromType& __arg)
    { return _ToType(dynamic_cast<typename _Caster<_ToType>::
		     type>(__arg.get())); }

  template<typename _ToType, typename _FromType>
    inline _ToType
    __const_pointer_cast(const _FromType& __arg)
    { return _ToType(const_cast<typename _Caster<_ToType>::
		     type>(__arg.get())); }

  template<typename _ToType, typename _FromType>
    inline _ToType
    __reinterpret_pointer_cast(const _FromType& __arg)
    { return _ToType(reinterpret_cast<typename _Caster<_ToType>::
		     type>(__arg.get())); }

  /**
   * Casting operations for cases where _FromType is a standard pointer.
   * _ToType can be a standard or non-standard pointer.
   */
  template<typename _ToType, typename _FromType>
    inline _ToType
    __static_pointer_cast(_FromType* __arg)
    { return _ToType(static_cast<typename _Caster<_ToType>::
		     type>(__arg)); }

  template<typename _ToType, typename _FromType>
    inline _ToType
    __dynamic_pointer_cast(_FromType* __arg)
    { return _ToType(dynamic_cast<typename _Caster<_ToType>::
		     type>(__arg)); }

  template<typename _ToType, typename _FromType>
    inline _ToType
    __const_pointer_cast(_FromType* __arg)
    { return _ToType(const_cast<typename _Caster<_ToType>::
		     type>(__arg)); }

  template<typename _ToType, typename _FromType>
    inline _ToType
    __reinterpret_pointer_cast(_FromType* __arg)
    { return _ToType(reinterpret_cast<typename _Caster<_ToType>::
		     type>(__arg)); }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif // _GLIBCXX_CAST_H
