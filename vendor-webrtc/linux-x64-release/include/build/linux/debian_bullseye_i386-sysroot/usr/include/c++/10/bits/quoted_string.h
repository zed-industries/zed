// Helpers for quoted stream manipulators -*- C++ -*-

// Copyright (C) 2013-2020 Free Software Foundation, Inc.
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

/** @file bits/quoted_string.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{iomanip}
 */

#ifndef _GLIBCXX_QUOTED_STRING_H
#define _GLIBCXX_QUOTED_STRING_H 1

#pragma GCC system_header

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else
#include <sstream>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  namespace __detail {
    /**
     * @brief Struct for delimited strings.
     */
    template<typename _String, typename _CharT>
      struct _Quoted_string
      {
	static_assert(is_reference<_String>::value
		   || is_pointer<_String>::value,
		      "String type must be pointer or reference");

	_Quoted_string(_String __str, _CharT __del, _CharT __esc)
	: _M_string(__str), _M_delim{__del}, _M_escape{__esc}
	{ }

	_Quoted_string&
	operator=(_Quoted_string&) = delete;

	_String _M_string;
	_CharT _M_delim;
	_CharT _M_escape;
      };

#if __cplusplus >= 201703L
    template<typename _CharT, typename _Traits>
      struct _Quoted_string<basic_string_view<_CharT, _Traits>, _CharT>
      {
	_Quoted_string(basic_string_view<_CharT, _Traits> __str,
		       _CharT __del, _CharT __esc)
	: _M_string(__str), _M_delim{__del}, _M_escape{__esc}
	{ }

	_Quoted_string&
	operator=(_Quoted_string&) = delete;

	basic_string_view<_CharT, _Traits> _M_string;
	_CharT _M_delim;
	_CharT _M_escape;
      };
#endif // C++17

    /**
     * @brief Inserter for quoted strings.
     *
     *  _GLIBCXX_RESOLVE_LIB_DEFECTS
     *  DR 2344 quoted()'s interaction with padding is unclear
     */
    template<typename _CharT, typename _Traits>
      std::basic_ostream<_CharT, _Traits>&
      operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		 const _Quoted_string<const _CharT*, _CharT>& __str)
      {
	std::basic_ostringstream<_CharT, _Traits> __ostr;
	__ostr << __str._M_delim;
	for (const _CharT* __c = __str._M_string; *__c; ++__c)
	  {
	    if (*__c == __str._M_delim || *__c == __str._M_escape)
	      __ostr << __str._M_escape;
	    __ostr << *__c;
	  }
	__ostr << __str._M_delim;

	return __os << __ostr.str();
      }

    /**
     * @brief Inserter for quoted strings.
     *
     *  _GLIBCXX_RESOLVE_LIB_DEFECTS
     *  DR 2344 quoted()'s interaction with padding is unclear
     */
    template<typename _CharT, typename _Traits, typename _String>
      std::basic_ostream<_CharT, _Traits>&
      operator<<(std::basic_ostream<_CharT, _Traits>& __os,
		 const _Quoted_string<_String, _CharT>& __str)
      {
	std::basic_ostringstream<_CharT, _Traits> __ostr;
	__ostr << __str._M_delim;
	for (auto __c : __str._M_string)
	  {
	    if (__c == __str._M_delim || __c == __str._M_escape)
	      __ostr << __str._M_escape;
	    __ostr << __c;
	  }
	__ostr << __str._M_delim;

	return __os << __ostr.str();
      }

    /**
     * @brief Extractor for delimited strings.
     *        The left and right delimiters can be different.
     */
    template<typename _CharT, typename _Traits, typename _Alloc>
      std::basic_istream<_CharT, _Traits>&
      operator>>(std::basic_istream<_CharT, _Traits>& __is,
		 const _Quoted_string<basic_string<_CharT, _Traits, _Alloc>&,
				      _CharT>& __str)
      {
	_CharT __c;
	__is >> __c;
	if (!__is.good())
	  return __is;
	if (__c != __str._M_delim)
	  {
	    __is.unget();
	    __is >> __str._M_string;
	    return __is;
	  }
	__str._M_string.clear();
	std::ios_base::fmtflags __flags
	  = __is.flags(__is.flags() & ~std::ios_base::skipws);
	do
	  {
	    __is >> __c;
	    if (!__is.good())
	      break;
	    if (__c == __str._M_escape)
	      {
		__is >> __c;
		if (!__is.good())
		  break;
	      }
	    else if (__c == __str._M_delim)
	      break;
	    __str._M_string += __c;
	  }
	while (true);
	__is.setf(__flags);

	return __is;
      }
  } // namespace __detail

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++11
#endif /* _GLIBCXX_QUOTED_STRING_H */
