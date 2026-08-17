// Filesystem declarations -*- C++ -*-

// Copyright (C) 2014-2020 Free Software Foundation, Inc.
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

/** @file experimental/bits/fs_fwd.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{experimental/filesystem}
 */

#ifndef _GLIBCXX_EXPERIMENTAL_FS_FWD_H
#define _GLIBCXX_EXPERIMENTAL_FS_FWD_H 1

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else

#include <system_error>
#include <cstdint>
#include <chrono>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace experimental
{
namespace filesystem
{
inline namespace v1
{
#if _GLIBCXX_USE_CXX11_ABI
inline namespace __cxx11 __attribute__((__abi_tag__ ("cxx11"))) { }
#endif

  /**
   * @defgroup filesystem-ts Filesystem TS
   * @ingroup experimental
   *
   * Utilities for performing operations on file systems and their components,
   * such as paths, regular files, and directories.
   *
   * ISO/IEC TS 18822:2015	C++ File System Technical Specification
   * @{
   */

  class file_status;
_GLIBCXX_BEGIN_NAMESPACE_CXX11
  class path;
  class filesystem_error;
  class directory_entry;
  class directory_iterator;
  class recursive_directory_iterator;
_GLIBCXX_END_NAMESPACE_CXX11

  struct space_info
  {
    uintmax_t capacity;
    uintmax_t free;
    uintmax_t available;
  };

  enum class file_type : signed char {
      none = 0, not_found = -1, regular = 1, directory = 2, symlink = 3,
      block = 4, character = 5, fifo = 6, socket = 7, unknown = 8
  };

  /// Bitmask type
  enum class copy_options : unsigned short {
      none = 0,
      skip_existing = 1, overwrite_existing = 2, update_existing = 4,
      recursive = 8,
      copy_symlinks = 16, skip_symlinks = 32,
      directories_only = 64, create_symlinks = 128, create_hard_links = 256
  };

  constexpr copy_options
  operator&(copy_options __x, copy_options __y) noexcept
  {
    using __utype = typename std::underlying_type<copy_options>::type;
    return static_cast<copy_options>(
	static_cast<__utype>(__x) & static_cast<__utype>(__y));
  }

  constexpr copy_options
  operator|(copy_options __x, copy_options __y) noexcept
  {
    using __utype = typename std::underlying_type<copy_options>::type;
    return static_cast<copy_options>(
	static_cast<__utype>(__x) | static_cast<__utype>(__y));
  }

  constexpr copy_options
  operator^(copy_options __x, copy_options __y) noexcept
  {
    using __utype = typename std::underlying_type<copy_options>::type;
    return static_cast<copy_options>(
	static_cast<__utype>(__x) ^ static_cast<__utype>(__y));
  }

  constexpr copy_options
  operator~(copy_options __x) noexcept
  {
    using __utype = typename std::underlying_type<copy_options>::type;
    return static_cast<copy_options>(~static_cast<__utype>(__x));
  }

  inline copy_options&
  operator&=(copy_options& __x, copy_options __y) noexcept
  { return __x = __x & __y; }

  inline copy_options&
  operator|=(copy_options& __x, copy_options __y) noexcept
  { return __x = __x | __y; }

  inline copy_options&
  operator^=(copy_options& __x, copy_options __y) noexcept
  { return __x = __x ^ __y; }


  /// Bitmask type
  enum class perms : unsigned {
      none		=  0,
      owner_read	=  0400,
      owner_write	=  0200,
      owner_exec	=  0100,
      owner_all		=  0700,
      group_read	=   040,
      group_write	=   020,
      group_exec	=   010,
      group_all		=   070,
      others_read	=    04,
      others_write	=    02,
      others_exec	=    01,
      others_all	=    07,
      all		=  0777,
      set_uid		= 04000,
      set_gid		= 02000,
      sticky_bit	= 01000,
      mask		= 07777,
      unknown		=  0xFFFF,
      add_perms		= 0x10000,
      remove_perms	= 0x20000,
      symlink_nofollow	= 0x40000
  };

  constexpr perms
  operator&(perms __x, perms __y) noexcept
  {
    using __utype = typename std::underlying_type<perms>::type;
    return static_cast<perms>(
	static_cast<__utype>(__x) & static_cast<__utype>(__y));
  }

  constexpr perms
  operator|(perms __x, perms __y) noexcept
  {
    using __utype = typename std::underlying_type<perms>::type;
    return static_cast<perms>(
	static_cast<__utype>(__x) | static_cast<__utype>(__y));
  }

  constexpr perms
  operator^(perms __x, perms __y) noexcept
  {
    using __utype = typename std::underlying_type<perms>::type;
    return static_cast<perms>(
	static_cast<__utype>(__x) ^ static_cast<__utype>(__y));
  }

  constexpr perms
  operator~(perms __x) noexcept
  {
    using __utype = typename std::underlying_type<perms>::type;
    return static_cast<perms>(~static_cast<__utype>(__x));
  }

  inline perms&
  operator&=(perms& __x, perms __y) noexcept
  { return __x = __x & __y; }

  inline perms&
  operator|=(perms& __x, perms __y) noexcept
  { return __x = __x | __y; }

  inline perms&
  operator^=(perms& __x, perms __y) noexcept
  { return __x = __x ^ __y; }

  // Bitmask type
  enum class directory_options : unsigned char {
      none = 0, follow_directory_symlink = 1, skip_permission_denied = 2
  };

  constexpr directory_options
  operator&(directory_options __x, directory_options __y) noexcept
  {
    using __utype = typename std::underlying_type<directory_options>::type;
    return static_cast<directory_options>(
	static_cast<__utype>(__x) & static_cast<__utype>(__y));
  }

  constexpr directory_options
  operator|(directory_options __x, directory_options __y) noexcept
  {
    using __utype = typename std::underlying_type<directory_options>::type;
    return static_cast<directory_options>(
	static_cast<__utype>(__x) | static_cast<__utype>(__y));
  }

  constexpr directory_options
  operator^(directory_options __x, directory_options __y) noexcept
  {
    using __utype = typename std::underlying_type<directory_options>::type;
    return static_cast<directory_options>(
	static_cast<__utype>(__x) ^ static_cast<__utype>(__y));
  }

  constexpr directory_options
  operator~(directory_options __x) noexcept
  {
    using __utype = typename std::underlying_type<directory_options>::type;
    return static_cast<directory_options>(~static_cast<__utype>(__x));
  }

  inline directory_options&
  operator&=(directory_options& __x, directory_options __y) noexcept
  { return __x = __x & __y; }

  inline directory_options&
  operator|=(directory_options& __x, directory_options __y) noexcept
  { return __x = __x | __y; }

  inline directory_options&
  operator^=(directory_options& __x, directory_options __y) noexcept
  { return __x = __x ^ __y; }

  using file_time_type = std::chrono::system_clock::time_point;

  // operational functions

  void copy(const path& __from, const path& __to, copy_options __options);
  void copy(const path& __from, const path& __to, copy_options __options,
	    error_code&) noexcept;

  bool copy_file(const path& __from, const path& __to, copy_options __option);
  bool copy_file(const path& __from, const path& __to, copy_options __option,
		 error_code&) noexcept;

  path current_path();

  file_status status(const path&);
  file_status status(const path&, error_code&) noexcept;

  bool status_known(file_status) noexcept;

  file_status symlink_status(const path&);
  file_status symlink_status(const path&, error_code&) noexcept;

  bool is_regular_file(file_status) noexcept;
  bool is_symlink(file_status) noexcept;

  // @} group filesystem-ts
} // namespace v1
} // namespace filesystem
} // namespace experimental

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++11

#endif // _GLIBCXX_EXPERIMENTAL_FS_FWD_H
