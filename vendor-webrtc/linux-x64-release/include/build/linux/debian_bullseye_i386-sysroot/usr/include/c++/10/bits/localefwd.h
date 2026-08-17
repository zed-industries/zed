// <locale> Forward declarations -*- C++ -*-

// Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

/** @file bits/localefwd.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{locale}
 */

//
// ISO C++ 14882: 22.1  Locales
//

#ifndef _LOCALE_FWD_H
#define _LOCALE_FWD_H 1

#pragma GCC system_header

#include <bits/c++config.h>
#include <bits/c++locale.h>  // Defines __c_locale, config-specific include
#include <iosfwd>            // For ostreambuf_iterator, istreambuf_iterator
#include <cctype>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /** 
   *  @defgroup locales Locales
   *
   *  Classes and functions for internationalization and localization.
   */

  // 22.1.1 Locale
  class locale;

  template<typename _Facet>
    bool
    has_facet(const locale&) throw();

  template<typename _Facet>
    const _Facet&
    use_facet(const locale&);

  // 22.1.3 Convenience interfaces
  template<typename _CharT>
    bool
    isspace(_CharT, const locale&);

  template<typename _CharT>
    bool
    isprint(_CharT, const locale&);

  template<typename _CharT>
    bool
    iscntrl(_CharT, const locale&);

  template<typename _CharT>
    bool
    isupper(_CharT, const locale&);

  template<typename _CharT>
    bool
    islower(_CharT, const locale&);

  template<typename _CharT>
    bool
    isalpha(_CharT, const locale&);

  template<typename _CharT>
    bool
    isdigit(_CharT, const locale&);

  template<typename _CharT>
    bool
    ispunct(_CharT, const locale&);

  template<typename _CharT>
    bool
    isxdigit(_CharT, const locale&);

  template<typename _CharT>
    bool
    isalnum(_CharT, const locale&);

  template<typename _CharT>
    bool
    isgraph(_CharT, const locale&);

#if __cplusplus >= 201103L
  template<typename _CharT>
    bool
    isblank(_CharT, const locale&);
#endif

  template<typename _CharT>
    _CharT
    toupper(_CharT, const locale&);

  template<typename _CharT>
    _CharT
    tolower(_CharT, const locale&);

  // 22.2.1 and 22.2.1.3 ctype
  class ctype_base;
  template<typename _CharT>
    class ctype;
  template<> class ctype<char>;
#ifdef _GLIBCXX_USE_WCHAR_T
  template<> class ctype<wchar_t>;
#endif
  template<typename _CharT>
    class ctype_byname;
  // NB: Specialized for char and wchar_t in locale_facets.h.

  class codecvt_base;
  template<typename _InternT, typename _ExternT, typename _StateT>
    class codecvt;
  template<> class codecvt<char, char, mbstate_t>;
#ifdef _GLIBCXX_USE_WCHAR_T
  template<> class codecvt<wchar_t, char, mbstate_t>;
#endif
#if __cplusplus >= 201103L
  template<> class codecvt<char16_t, char, mbstate_t>;
  template<> class codecvt<char32_t, char, mbstate_t>;
#ifdef _GLIBCXX_USE_CHAR8_T
  template<> class codecvt<char16_t, char8_t, mbstate_t>;
  template<> class codecvt<char32_t, char8_t, mbstate_t>;
#endif
#endif
  template<typename _InternT, typename _ExternT, typename _StateT>
    class codecvt_byname;

  // 22.2.2 and 22.2.3 numeric
_GLIBCXX_BEGIN_NAMESPACE_LDBL
  template<typename _CharT, typename _InIter = istreambuf_iterator<_CharT> >
    class num_get;
  template<typename _CharT, typename _OutIter = ostreambuf_iterator<_CharT> >
    class num_put;
_GLIBCXX_END_NAMESPACE_LDBL
_GLIBCXX_BEGIN_NAMESPACE_CXX11
  template<typename _CharT> class numpunct;
  template<typename _CharT> class numpunct_byname;
_GLIBCXX_END_NAMESPACE_CXX11

_GLIBCXX_BEGIN_NAMESPACE_CXX11
  // 22.2.4 collation
  template<typename _CharT>
    class collate;
  template<typename _CharT>
    class collate_byname;
_GLIBCXX_END_NAMESPACE_CXX11

  // 22.2.5 date and time
  class time_base;
_GLIBCXX_BEGIN_NAMESPACE_CXX11
  template<typename _CharT, typename _InIter =  istreambuf_iterator<_CharT> >
    class time_get;
  template<typename _CharT, typename _InIter =  istreambuf_iterator<_CharT> >
    class time_get_byname;
_GLIBCXX_END_NAMESPACE_CXX11
  template<typename _CharT, typename _OutIter = ostreambuf_iterator<_CharT> >
    class time_put;
  template<typename _CharT, typename _OutIter = ostreambuf_iterator<_CharT> >
    class time_put_byname;

  // 22.2.6 money
  class money_base;
_GLIBCXX_BEGIN_NAMESPACE_LDBL_OR_CXX11
  template<typename _CharT, typename _InIter =  istreambuf_iterator<_CharT> >
    class money_get;
  template<typename _CharT, typename _OutIter = ostreambuf_iterator<_CharT> >
    class money_put;
_GLIBCXX_END_NAMESPACE_LDBL_OR_CXX11
_GLIBCXX_BEGIN_NAMESPACE_CXX11
  template<typename _CharT, bool _Intl = false>
    class moneypunct;
  template<typename _CharT, bool _Intl = false>
    class moneypunct_byname;
_GLIBCXX_END_NAMESPACE_CXX11

  // 22.2.7 message retrieval
  class messages_base;
_GLIBCXX_BEGIN_NAMESPACE_CXX11
  template<typename _CharT>
    class messages;
  template<typename _CharT>
    class messages_byname;
_GLIBCXX_END_NAMESPACE_CXX11

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif
