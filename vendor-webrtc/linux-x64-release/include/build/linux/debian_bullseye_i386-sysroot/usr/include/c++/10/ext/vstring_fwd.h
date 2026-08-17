// <vstring.h> Forward declarations -*- C++ -*-

// Copyright (C) 2005-2020 Free Software Foundation, Inc.
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

/** @file ext/vstring_fwd.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{ext/vstring.h}
 */

#ifndef _VSTRING_FWD_H
#define _VSTRING_FWD_H 1

#pragma GCC system_header

#include <bits/c++config.h>
#include <bits/char_traits.h>
#include <bits/allocator.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  template<typename _CharT, typename _Traits, typename _Alloc>
    class __sso_string_base;

  template<typename _CharT, typename _Traits, typename _Alloc>
    class __rc_string_base;

  template<typename _CharT, typename _Traits = std::char_traits<_CharT>,
           typename _Alloc = std::allocator<_CharT>,
	   template
	   <typename, typename, typename> class _Base = __sso_string_base>
    class __versa_string;

  typedef __versa_string<char>                              __vstring;
  typedef __vstring                                         __sso_string;
  typedef 
  __versa_string<char, std::char_traits<char>,
		 std::allocator<char>, __rc_string_base>    __rc_string;

#ifdef _GLIBCXX_USE_WCHAR_T
  typedef __versa_string<wchar_t>                           __wvstring;
  typedef __wvstring                                        __wsso_string;
  typedef
  __versa_string<wchar_t, std::char_traits<wchar_t>,
		 std::allocator<wchar_t>, __rc_string_base> __wrc_string;
#endif  

#if __cplusplus >= 201103L
  typedef __versa_string<char16_t>                          __u16vstring;
  typedef __u16vstring                                      __u16sso_string;
  typedef 
  __versa_string<char16_t, std::char_traits<char16_t>,
		 std::allocator<char16_t>, __rc_string_base> __u16rc_string;

  typedef __versa_string<char32_t>                          __u32vstring;
  typedef __u32vstring                                      __u32sso_string;
  typedef 
  __versa_string<char32_t, std::char_traits<char32_t>,
		 std::allocator<char32_t>, __rc_string_base> __u32rc_string;
#endif // C++11

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _VSTRING_FWD_H */
