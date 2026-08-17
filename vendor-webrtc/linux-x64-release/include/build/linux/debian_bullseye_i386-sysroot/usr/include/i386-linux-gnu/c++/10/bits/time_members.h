// std::time_get, std::time_put implementation, GNU version -*- C++ -*-

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

/** @file bits/time_members.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{locale}
 */

//
// ISO C++ 14882: 22.2.5.1.2 - time_get functions
// ISO C++ 14882: 22.2.5.3.2 - time_put functions
//

// Written by Benjamin Kosnik <bkoz@redhat.com>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  template<typename _CharT>
    __timepunct<_CharT>::__timepunct(size_t __refs)
    : facet(__refs), _M_data(0), _M_c_locale_timepunct(0),
      _M_name_timepunct(_S_get_c_name())
    { _M_initialize_timepunct(); }

  template<typename _CharT>
    __timepunct<_CharT>::__timepunct(__cache_type* __cache, size_t __refs)
    : facet(__refs), _M_data(__cache), _M_c_locale_timepunct(0),
      _M_name_timepunct(_S_get_c_name())
    { _M_initialize_timepunct(); }

  template<typename _CharT>
    __timepunct<_CharT>::__timepunct(__c_locale __cloc, const char* __s,
				     size_t __refs)
    : facet(__refs), _M_data(0), _M_c_locale_timepunct(0),
      _M_name_timepunct(0)
    {
      if (__builtin_strcmp(__s, _S_get_c_name()) != 0)
	{
	  const size_t __len = __builtin_strlen(__s) + 1;
	  char* __tmp = new char[__len];
	  __builtin_memcpy(__tmp, __s, __len);
	  _M_name_timepunct = __tmp;
	}
      else
	_M_name_timepunct = _S_get_c_name();

      __try
	{ _M_initialize_timepunct(__cloc); }
      __catch(...)
	{
	  if (_M_name_timepunct != _S_get_c_name())
	    delete [] _M_name_timepunct;
	  __throw_exception_again;
	}
    }

  template<typename _CharT>
    __timepunct<_CharT>::~__timepunct()
    {
      if (_M_name_timepunct != _S_get_c_name())
	delete [] _M_name_timepunct;
      delete _M_data;
      _S_destroy_c_locale(_M_c_locale_timepunct);
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace
