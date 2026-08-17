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

/** @file backward/backward_warning.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{iosfwd}
 */

#ifndef _BACKWARD_BACKWARD_WARNING_H
#define _BACKWARD_BACKWARD_WARNING_H 1

#ifdef __DEPRECATED
#warning \
  This file includes at least one deprecated or antiquated header which \
  may be removed without further notice at a future date. Please use a \
  non-deprecated interface with equivalent functionality instead. For a \
  listing of replacement headers and interfaces, consult the file \
  backward_warning.h. To disable this warning use -Wno-deprecated.

/*
  A list of valid replacements is as follows:

  Use:					Instead of:
  <sstream>, basic_stringbuf	   	<strstream>, strstreambuf
  <sstream>, basic_istringstream	<strstream>, istrstream
  <sstream>, basic_ostringstream	<strstream>, ostrstream
  <sstream>, basic_stringstream		<strstream>, strstream
  <unordered_set>, unordered_set     	<ext/hash_set>, hash_set
  <unordered_set>, unordered_multiset	<ext/hash_set>, hash_multiset
  <unordered_map>, unordered_map	<ext/hash_map>, hash_map
  <unordered_map>, unordered_multimap	<ext/hash_map>, hash_multimap
  <functional>, bind			<functional>, binder1st
  <functional>, bind			<functional>, binder2nd
  <functional>, bind			<functional>, bind1st
  <functional>, bind			<functional>, bind2nd
  <memory>, unique_ptr       		<memory>, auto_ptr
*/

#endif

#endif
