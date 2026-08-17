// Filesystem operational functions -*- C++ -*-

// Copyright (C) 2014-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your __option)
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

#ifndef _GLIBCXX_EXPERIMENTAL_FS_OPS_H
#define _GLIBCXX_EXPERIMENTAL_FS_OPS_H 1

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else

#include <cstdint>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace experimental
{
namespace filesystem
{
inline namespace v1
{
  /**
   * @addtogroup filesystem-ts
   * @{
   */

  path absolute(const path& __p, const path& __base = current_path());

  path canonical(const path& __p, const path& __base = current_path());
  path canonical(const path& __p, error_code& __ec);
  path canonical(const path& __p, const path& __base, error_code& __ec);

  inline void
  copy(const path& __from, const path& __to)
  { copy(__from, __to, copy_options::none); }

  inline void
  copy(const path& __from, const path& __to, error_code& __ec) noexcept
  { copy(__from, __to, copy_options::none, __ec); }

  void copy(const path& __from, const path& __to, copy_options __options);
  void copy(const path& __from, const path& __to, copy_options __options,
	    error_code& __ec) noexcept;

  inline bool
  copy_file(const path& __from, const path& __to)
  { return copy_file(__from, __to, copy_options::none); }

  inline bool
  copy_file(const path& __from, const path& __to, error_code& __ec) noexcept
  { return copy_file(__from, __to, copy_options::none, __ec); }

  bool copy_file(const path& __from, const path& __to, copy_options __option);
  bool copy_file(const path& __from, const path& __to, copy_options __option,
		 error_code& __ec) noexcept;

  void copy_symlink(const path& __existing_symlink, const path& __new_symlink);
  void copy_symlink(const path& __existing_symlink, const path& __new_symlink,
		    error_code& __ec) noexcept;

  bool create_directories(const path& __p);
  bool create_directories(const path& __p, error_code& __ec) noexcept;

  bool create_directory(const path& __p);
  bool create_directory(const path& __p, error_code& __ec) noexcept;

  bool create_directory(const path& __p, const path& attributes);
  bool create_directory(const path& __p, const path& attributes,
			error_code& __ec) noexcept;

  void create_directory_symlink(const path& __to, const path& __new_symlink);
  void create_directory_symlink(const path& __to, const path& __new_symlink,
				error_code& __ec) noexcept;

  void create_hard_link(const path& __to, const path& __new_hard_link);
  void create_hard_link(const path& __to, const path& __new_hard_link,
			error_code& __ec) noexcept;

  void create_symlink(const path& __to, const path& __new_symlink);
  void create_symlink(const path& __to, const path& __new_symlink,
		      error_code& __ec) noexcept;

  path current_path();
  path current_path(error_code& __ec);
  void current_path(const path& __p);
  void current_path(const path& __p, error_code& __ec) noexcept;

  bool
  equivalent(const path& __p1, const path& __p2);

  bool
  equivalent(const path& __p1, const path& __p2, error_code& __ec) noexcept;

  inline bool
  exists(file_status __s) noexcept
  { return status_known(__s) && __s.type() != file_type::not_found; }

  inline bool
  exists(const path& __p)
  { return exists(status(__p)); }

  inline bool
  exists(const path& __p, error_code& __ec) noexcept
  {
    auto __s = status(__p, __ec);
    if (status_known(__s))
      {
	__ec.clear();
	return __s.type() != file_type::not_found;
      }
    return false;
  }

  uintmax_t file_size(const path& __p);
  uintmax_t file_size(const path& __p, error_code& __ec) noexcept;

  uintmax_t hard_link_count(const path& __p);
  uintmax_t hard_link_count(const path& __p, error_code& __ec) noexcept;

  inline bool
  is_block_file(file_status __s) noexcept
  { return __s.type() == file_type::block; }

  inline bool
  is_block_file(const path& __p)
  { return is_block_file(status(__p)); }

  inline bool
  is_block_file(const path& __p, error_code& __ec) noexcept
  { return is_block_file(status(__p, __ec)); }

  inline bool
  is_character_file(file_status __s) noexcept
  { return __s.type() == file_type::character; }

  inline bool
  is_character_file(const path& __p)
  { return is_character_file(status(__p)); }

  inline bool
  is_character_file(const path& __p, error_code& __ec) noexcept
  { return is_character_file(status(__p, __ec)); }

  inline bool
  is_directory(file_status __s) noexcept
  { return __s.type() == file_type::directory; }

  inline bool
  is_directory(const path& __p)
  { return is_directory(status(__p)); }

  inline bool
  is_directory(const path& __p, error_code& __ec) noexcept
  { return is_directory(status(__p, __ec)); }

  bool is_empty(const path& __p);
  bool is_empty(const path& __p, error_code& __ec) noexcept;

  inline bool
  is_fifo(file_status __s) noexcept
  { return __s.type() == file_type::fifo; }

  inline bool
  is_fifo(const path& __p)
  { return is_fifo(status(__p)); }

  inline bool
  is_fifo(const path& __p, error_code& __ec) noexcept
  { return is_fifo(status(__p, __ec)); }

  inline bool
  is_other(file_status __s) noexcept
  {
    return exists(__s) && !is_regular_file(__s) && !is_directory(__s)
      && !is_symlink(__s);
  }

  inline bool
  is_other(const path& __p)
  { return is_other(status(__p)); }

  inline bool
  is_other(const path& __p, error_code& __ec) noexcept
  { return is_other(status(__p, __ec)); }

  inline bool
  is_regular_file(file_status __s) noexcept
  { return __s.type() == file_type::regular; }

  inline bool
  is_regular_file(const path& __p)
  { return is_regular_file(status(__p)); }

  inline bool
  is_regular_file(const path& __p, error_code& __ec) noexcept
  { return is_regular_file(status(__p, __ec)); }

  inline bool
  is_socket(file_status __s) noexcept
  { return __s.type() == file_type::socket; }

  inline bool
  is_socket(const path& __p)
  { return is_socket(status(__p)); }

  inline bool
  is_socket(const path& __p, error_code& __ec) noexcept
  { return is_socket(status(__p, __ec)); }

  inline bool
  is_symlink(file_status __s) noexcept
  { return __s.type() == file_type::symlink; }

  inline bool
  is_symlink(const path& __p)
  { return is_symlink(symlink_status(__p)); }

  inline bool
  is_symlink(const path& __p, error_code& __ec) noexcept
  { return is_symlink(symlink_status(__p, __ec)); }

  file_time_type  last_write_time(const path& __p);
  file_time_type  last_write_time(const path& __p, error_code& __ec) noexcept;
  void last_write_time(const path& __p, file_time_type __new_time);
  void last_write_time(const path& __p, file_time_type __new_time,
		       error_code& __ec) noexcept;

  void permissions(const path& __p, perms __prms);
  void permissions(const path& __p, perms __prms, error_code& __ec) noexcept;

  path read_symlink(const path& __p);
  path read_symlink(const path& __p, error_code& __ec);

  bool remove(const path& __p);
  bool remove(const path& __p, error_code& __ec) noexcept;

  uintmax_t remove_all(const path& __p);
  uintmax_t remove_all(const path& __p, error_code& __ec) noexcept;

  void rename(const path& __from, const path& __to);
  void rename(const path& __from, const path& __to, error_code& __ec) noexcept;

  void resize_file(const path& __p, uintmax_t __size);
  void resize_file(const path& __p, uintmax_t __size, error_code& __ec) noexcept;

  space_info space(const path& __p);
  space_info space(const path& __p, error_code& __ec) noexcept;

  file_status status(const path& __p);
  file_status status(const path& __p, error_code& __ec) noexcept;

  inline bool status_known(file_status __s) noexcept
  { return __s.type() != file_type::none; }

  file_status symlink_status(const path& __p);
  file_status symlink_status(const path& __p, error_code& __ec) noexcept;

  path system_complete(const path& __p);
  path system_complete(const path& __p, error_code& __ec);

  path temp_directory_path();
  path temp_directory_path(error_code& __ec);

  // @} group filesystem-ts
} // namespace v1
} // namespace filesystem
} // namespace experimental

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++11

#endif // _GLIBCXX_EXPERIMENTAL_FS_OPS_H
