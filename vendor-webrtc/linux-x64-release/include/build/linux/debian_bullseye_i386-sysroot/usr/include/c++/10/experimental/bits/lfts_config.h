// Namespace declarations for Library Fundamentals TS -*- C++ -*-

// Copyright (C) 2016-2020 Free Software Foundation, Inc.
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

/** @file experimental/bits/lfts_config.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly.
 */

#if __cplusplus >= 201402L
#include <bits/c++config.h>

  /** @defgroup libfund-ts Library Fundamentals TS
   *  @ingroup experimental
   *
   * Components defined by the _C++ Extensions for Library Fundamentals_
   * Technical Specification, versions 1 and 2.
   *
   * - ISO/IEC TS 19568:2015 C++ Extensions for Library Fundamentals
   * - ISO/IEC TS 19568:2017 C++ Extensions for Library Fundamentals, Version 2
   */

#if _GLIBCXX_INLINE_VERSION
namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace chrono
{
namespace experimental
{
inline namespace fundamentals_v1 { }
inline namespace fundamentals_v2 { }
} // namespace experimental
} // namespace chrono

namespace experimental
{
inline namespace fundamentals_v1 { }
inline namespace fundamentals_v2 { }
inline namespace literals { inline namespace string_view_literals { } }
} // namespace experimental

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
#endif
#endif
