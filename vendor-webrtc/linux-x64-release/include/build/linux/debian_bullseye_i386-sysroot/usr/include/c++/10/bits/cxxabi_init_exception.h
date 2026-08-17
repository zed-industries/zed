// ABI Support -*- C++ -*-

// Copyright (C) 2016-2020 Free Software Foundation, Inc.
//
// This file is part of GCC.
//
// GCC is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// GCC is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file bits/cxxabi_init_exception.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly.
 */

#ifndef _CXXABI_INIT_EXCEPTION_H
#define _CXXABI_INIT_EXCEPTION_H 1

#pragma GCC system_header

#pragma GCC visibility push(default)

#include <stddef.h>
#include <bits/c++config.h>

#ifndef _GLIBCXX_CDTOR_CALLABI
#define _GLIBCXX_CDTOR_CALLABI
#define _GLIBCXX_HAVE_CDTOR_CALLABI 0
#else
#define _GLIBCXX_HAVE_CDTOR_CALLABI 1
#endif

#ifdef __cplusplus

namespace std
{
  class type_info;
}

namespace __cxxabiv1
{
  struct __cxa_refcounted_exception;

  extern "C"
    {
      // Allocate memory for the primary exception plus the thrown object.
      void*
      __cxa_allocate_exception(size_t) _GLIBCXX_NOTHROW;

      void
      __cxa_free_exception(void*) _GLIBCXX_NOTHROW;

      // Initialize exception (this is a GNU extension)
      __cxa_refcounted_exception*
      __cxa_init_primary_exception(void *object, std::type_info *tinfo,
                void (_GLIBCXX_CDTOR_CALLABI *dest) (void *)) _GLIBCXX_NOTHROW;

    }
} // namespace __cxxabiv1

#endif

#pragma GCC visibility pop

#endif // _CXXABI_INIT_EXCEPTION_H
