// cxxabi.h subset for cancellation -*- C++ -*-
  
// Copyright (C) 2007-2020 Free Software Foundation, Inc.
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

/** @file bits/cxxabi_forced.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{cxxabi.h}
 */

#ifndef _CXXABI_FORCED_H
#define _CXXABI_FORCED_H 1

#pragma GCC system_header

#pragma GCC visibility push(default)

#ifdef __cplusplus
namespace __cxxabiv1
{  
  /** 
   *  @brief Thrown as part of forced unwinding.
   *  @ingroup exceptions
   *
   *  A magic placeholder class that can be caught by reference to
   *  recognize forced unwinding.
   */
  class __forced_unwind
  {
    virtual ~__forced_unwind() throw();

    // Prevent catch by value.
    virtual void __pure_dummy() = 0; 
  };
}
#endif // __cplusplus

#pragma GCC visibility pop

#endif // __CXXABI_FORCED_H 
