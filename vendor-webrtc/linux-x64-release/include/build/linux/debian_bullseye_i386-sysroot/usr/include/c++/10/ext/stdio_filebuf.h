// File descriptor layer for filebuf -*- C++ -*-

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

/** @file ext/stdio_filebuf.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _STDIO_FILEBUF_H
#define _STDIO_FILEBUF_H 1

#pragma GCC system_header

#include <fstream>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief Provides a layer of compatibility for C/POSIX.
   *  @ingroup io
   *
   *  This GNU extension provides extensions for working with standard C
   *  FILE*'s and POSIX file descriptors.  It must be instantiated by the
   *  user with the type of character used in the file stream, e.g.,
   *  stdio_filebuf<char>.
  */
  template<typename _CharT, typename _Traits = std::char_traits<_CharT> >
    class stdio_filebuf : public std::basic_filebuf<_CharT, _Traits>
    {
    public:
      // Types:
      typedef _CharT				        char_type;
      typedef _Traits				        traits_type;
      typedef typename traits_type::int_type		int_type;
      typedef typename traits_type::pos_type		pos_type;
      typedef typename traits_type::off_type		off_type;
      typedef std::size_t                               size_t;

    public:
      /**
       * deferred initialization
      */
      stdio_filebuf() : std::basic_filebuf<_CharT, _Traits>() {}

      /**
       *  @param  __fd  An open file descriptor.
       *  @param  __mode  Same meaning as in a standard filebuf.
       *  @param  __size Optimal or preferred size of internal buffer,
       *                 in chars.
       *
       *  This constructor associates a file stream buffer with an open
       *  POSIX file descriptor. The file descriptor will be automatically
       *  closed when the stdio_filebuf is closed/destroyed.
      */
      stdio_filebuf(int __fd, std::ios_base::openmode __mode,
		    size_t __size = static_cast<size_t>(BUFSIZ));

      /**
       *  @param  __f  An open @c FILE*.
       *  @param  __mode  Same meaning as in a standard filebuf.
       *  @param  __size Optimal or preferred size of internal buffer,
       *                 in chars.  Defaults to system's @c BUFSIZ.
       *
       *  This constructor associates a file stream buffer with an open
       *  C @c FILE*.  The @c FILE* will not be automatically closed when the
       *  stdio_filebuf is closed/destroyed.
      */
      stdio_filebuf(std::__c_file* __f, std::ios_base::openmode __mode,
		    size_t __size = static_cast<size_t>(BUFSIZ));

      /**
       *  Closes the external data stream if the file descriptor constructor
       *  was used.
      */
      virtual
      ~stdio_filebuf();

#if __cplusplus >= 201103L
      stdio_filebuf(stdio_filebuf&&) = default;
      stdio_filebuf& operator=(stdio_filebuf&&) = default;

      void
      swap(stdio_filebuf& __fb)
      { std::basic_filebuf<_CharT, _Traits>::swap(__fb); }
#endif

      /**
       *  @return  The underlying file descriptor.
       *
       *  Once associated with an external data stream, this function can be
       *  used to access the underlying POSIX file descriptor.  Note that
       *  there is no way for the library to track what you do with the
       *  descriptor, so be careful.
      */
      int
      fd() { return this->_M_file.fd(); }

      /**
       *  @return  The underlying FILE*.
       *
       *  This function can be used to access the underlying "C" file pointer.
       *  Note that there is no way for the library to track what you do
       *  with the file, so be careful.
       */
      std::__c_file*
      file() { return this->_M_file.file(); }
    };

  template<typename _CharT, typename _Traits>
    stdio_filebuf<_CharT, _Traits>::~stdio_filebuf()
    { }

  template<typename _CharT, typename _Traits>
    stdio_filebuf<_CharT, _Traits>::
    stdio_filebuf(int __fd, std::ios_base::openmode __mode, size_t __size)
    {
      this->_M_file.sys_open(__fd, __mode);
      if (this->is_open())
	{
	  this->_M_mode = __mode;
	  this->_M_buf_size = __size;
	  this->_M_allocate_internal_buffer();
	  this->_M_reading = false;
	  this->_M_writing = false;
	  this->_M_set_buffer(-1);
	}
    }

  template<typename _CharT, typename _Traits>
    stdio_filebuf<_CharT, _Traits>::
    stdio_filebuf(std::__c_file* __f, std::ios_base::openmode __mode,
		  size_t __size)
    {
      this->_M_file.sys_open(__f, __mode);
      if (this->is_open())
	{
	  this->_M_mode = __mode;
	  this->_M_buf_size = __size;
	  this->_M_allocate_internal_buffer();
	  this->_M_reading = false;
	  this->_M_writing = false;
	  this->_M_set_buffer(-1);
	}
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
