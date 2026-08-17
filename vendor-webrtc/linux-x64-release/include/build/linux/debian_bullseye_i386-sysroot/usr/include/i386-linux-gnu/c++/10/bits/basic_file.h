// Wrapper of C-language FILE struct -*- C++ -*-

// Copyright (C) 2000-2020 Free Software Foundation, Inc.
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

//
// ISO C++ 14882: 27.8  File-based streams
//

/** @file bits/basic_file.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{ios}
 */

#ifndef _GLIBCXX_BASIC_FILE_STDIO_H
#define _GLIBCXX_BASIC_FILE_STDIO_H 1

#pragma GCC system_header

#include <bits/c++config.h>
#include <bits/c++io.h>  // for __c_lock and __c_file
#include <bits/move.h>   // for swap
#include <ios>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // Generic declaration.
  template<typename _CharT>
    class __basic_file;

  // Specialization.
  template<>
    class __basic_file<char>
    {
      // Underlying data source/sink.
      __c_file* 	_M_cfile;

      // True iff we opened _M_cfile, and thus must close it ourselves.
      bool 		_M_cfile_created;

    public:
      __basic_file(__c_lock* __lock = 0) throw ();

#if __cplusplus >= 201103L
      __basic_file(__basic_file&& __rv, __c_lock* = 0) noexcept
      : _M_cfile(__rv._M_cfile), _M_cfile_created(__rv._M_cfile_created)
      {
	__rv._M_cfile = nullptr;
	__rv._M_cfile_created = false;
      }

      __basic_file& operator=(const __basic_file&) = delete;
      __basic_file& operator=(__basic_file&&) = delete;

      void
      swap(__basic_file& __f) noexcept
      {
	std::swap(_M_cfile, __f._M_cfile);
	std::swap(_M_cfile_created, __f._M_cfile_created);
      }
#endif

      __basic_file*
      open(const char* __name, ios_base::openmode __mode, int __prot = 0664);

#if _GLIBCXX_HAVE__WFOPEN && _GLIBCXX_USE_WCHAR_T
      __basic_file*
      open(const wchar_t* __name, ios_base::openmode __mode);
#endif

      __basic_file*
      sys_open(__c_file* __file, ios_base::openmode);

      __basic_file*
      sys_open(int __fd, ios_base::openmode __mode) throw ();

      __basic_file*
      close();

      _GLIBCXX_PURE bool
      is_open() const throw ();

      _GLIBCXX_PURE int
      fd() throw ();

      _GLIBCXX_PURE __c_file*
      file() throw ();

      ~__basic_file();

      streamsize
      xsputn(const char* __s, streamsize __n);

      streamsize
      xsputn_2(const char* __s1, streamsize __n1,
	       const char* __s2, streamsize __n2);

      streamsize
      xsgetn(char* __s, streamsize __n);

      streamoff
      seekoff(streamoff __off, ios_base::seekdir __way) throw ();

      int
      sync();

      streamsize
      showmanyc();
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
