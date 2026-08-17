// Locale support -*- C++ -*-

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

/** @file bits/ctype_base.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{locale}
 */

//
// ISO C++ 14882: 22.1  Locales
//

// Information as gleaned from /usr/include/ctype.h

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /// @brief  Base class for ctype.
  struct ctype_base
  {
    // Non-standard typedefs.
    typedef const int* 		__to_type;

    // NB: Offsets into ctype<char>::_M_table force a particular size
    // on the mask type. Because of this, we don't use an enum.
    typedef unsigned short 	mask;
    static const mask upper    	= _ISupper;
    static const mask lower 	= _ISlower;
    static const mask alpha 	= _ISalpha;
    static const mask digit 	= _ISdigit;
    static const mask xdigit 	= _ISxdigit;
    static const mask space 	= _ISspace;
    static const mask print 	= _ISprint;
    static const mask graph 	= _ISalpha | _ISdigit | _ISpunct;
    static const mask cntrl 	= _IScntrl;
    static const mask punct 	= _ISpunct;
    static const mask alnum 	= _ISalpha | _ISdigit;
#if __cplusplus >= 201103L
    static const mask blank	= _ISblank;
#endif
  };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace
