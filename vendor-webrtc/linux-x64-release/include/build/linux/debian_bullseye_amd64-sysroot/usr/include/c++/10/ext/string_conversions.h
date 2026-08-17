// String Conversions -*- C++ -*-

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

/** @file ext/string_conversions.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _STRING_CONVERSIONS_H
#define _STRING_CONVERSIONS_H 1

#pragma GCC system_header

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else

#include <bits/c++config.h>
#include <ext/numeric_traits.h>
#include <bits/functexcept.h>
#include <cstdlib>
#include <cwchar>
#include <cstdio>
#include <cerrno>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // Helper for all the sto* functions.
  template<typename _TRet, typename _Ret = _TRet, typename _CharT,
	   typename... _Base>
    _Ret
    __stoa(_TRet (*__convf) (const _CharT*, _CharT**, _Base...),
	   const char* __name, const _CharT* __str, std::size_t* __idx,
	   _Base... __base)
    {
      _Ret __ret;

      _CharT* __endptr;

      struct _Save_errno {
	_Save_errno() : _M_errno(errno) { errno = 0; }
	~_Save_errno() { if (errno == 0) errno = _M_errno; }
	int _M_errno;
      } const __save_errno;

      struct _Range_chk {
	  static bool
	  _S_chk(_TRet, std::false_type) { return false; }

	  static bool
	  _S_chk(_TRet __val, std::true_type) // only called when _Ret is int
	  {
	    return __val < _TRet(__numeric_traits<int>::__min)
	      || __val > _TRet(__numeric_traits<int>::__max);
	  }
      };

      const _TRet __tmp = __convf(__str, &__endptr, __base...);

      if (__endptr == __str)
	std::__throw_invalid_argument(__name);
      else if (errno == ERANGE
	  || _Range_chk::_S_chk(__tmp, std::is_same<_Ret, int>{}))
	std::__throw_out_of_range(__name);
      else
	__ret = __tmp;

      if (__idx)
	*__idx = __endptr - __str;

      return __ret;
    }

  // Helper for the to_string / to_wstring functions.
  template<typename _String, typename _CharT = typename _String::value_type>
    _String
    __to_xstring(int (*__convf) (_CharT*, std::size_t, const _CharT*,
				 __builtin_va_list), std::size_t __n,
		 const _CharT* __fmt, ...)
    {
      // XXX Eventually the result should be constructed in-place in
      // the __cxx11 string, likely with the help of internal hooks.
      _CharT* __s = static_cast<_CharT*>(__builtin_alloca(sizeof(_CharT)
							  * __n));

      __builtin_va_list __args;
      __builtin_va_start(__args, __fmt);

      const int __len = __convf(__s, __n, __fmt, __args);

      __builtin_va_end(__args);

      return _String(__s, __s + __len);
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif // C++11

#endif // _STRING_CONVERSIONS_H
