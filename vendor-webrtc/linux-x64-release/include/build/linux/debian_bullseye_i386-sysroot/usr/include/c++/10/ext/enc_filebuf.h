// filebuf with encoding state type -*- C++ -*-

// Copyright (C) 2002-2020 Free Software Foundation, Inc.
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

/** @file ext/enc_filebuf.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _EXT_ENC_FILEBUF_H
#define _EXT_ENC_FILEBUF_H 1

#include <fstream>
#include <locale>
#include <ext/codecvt_specializations.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /// class enc_filebuf.
  template<typename _CharT>
    class enc_filebuf
    : public std::basic_filebuf<_CharT, encoding_char_traits<_CharT> >
    {
    public:
      typedef encoding_char_traits<_CharT>     	traits_type;
      typedef typename traits_type::state_type	state_type;
      typedef typename traits_type::pos_type	pos_type;

      enc_filebuf(state_type& __state)
      : std::basic_filebuf<_CharT, encoding_char_traits<_CharT> >()
      { this->_M_state_beg = __state; }

    private:
      // concept requirements:
      // Set state type to something useful.
      // Something more than copyconstructible is needed here, so
      // require default and copy constructible + assignment operator.
      __glibcxx_class_requires(state_type, _SGIAssignableConcept)
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
