// <bits/enable_special_members.h> -*- C++ -*-

// Copyright (C) 2013-2020 Free Software Foundation, Inc.
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

/** @file bits/enable_special_members.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly.
 */

#ifndef _ENABLE_SPECIAL_MEMBERS_H
#define _ENABLE_SPECIAL_MEMBERS_H 1

#pragma GCC system_header

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  struct _Enable_default_constructor_tag
  {
    explicit constexpr _Enable_default_constructor_tag() = default;
  };

/**
  * @brief A mixin helper to conditionally enable or disable the default
  * constructor.
  * @sa _Enable_special_members
  */
template<bool _Switch, typename _Tag = void>
  struct _Enable_default_constructor
  {
    constexpr _Enable_default_constructor() noexcept = default;
    constexpr _Enable_default_constructor(_Enable_default_constructor const&)
      noexcept  = default;
    constexpr _Enable_default_constructor(_Enable_default_constructor&&)
      noexcept = default;
    _Enable_default_constructor&
    operator=(_Enable_default_constructor const&) noexcept = default;
    _Enable_default_constructor&
    operator=(_Enable_default_constructor&&) noexcept = default;

    // Can be used in other ctors.
    constexpr explicit
    _Enable_default_constructor(_Enable_default_constructor_tag) { }
  };


/**
  * @brief A mixin helper to conditionally enable or disable the default
  * destructor.
  * @sa _Enable_special_members
  */
template<bool _Switch, typename _Tag = void>
  struct _Enable_destructor { };

/**
  * @brief A mixin helper to conditionally enable or disable the copy/move
  * special members.
  * @sa _Enable_special_members
  */
template<bool _Copy, bool _CopyAssignment,
         bool _Move, bool _MoveAssignment,
         typename _Tag = void>
  struct _Enable_copy_move { };

/**
  * @brief A mixin helper to conditionally enable or disable the special
  * members.
  *
  * The @c _Tag type parameter is to make mixin bases unique and thus avoid
  * ambiguities.
  */
template<bool _Default, bool _Destructor,
         bool _Copy, bool _CopyAssignment,
         bool _Move, bool _MoveAssignment,
         typename _Tag = void>
  struct _Enable_special_members
  : private _Enable_default_constructor<_Default, _Tag>,
    private _Enable_destructor<_Destructor, _Tag>,
    private _Enable_copy_move<_Copy, _CopyAssignment,
                              _Move, _MoveAssignment,
                              _Tag>
  { };

// Boilerplate follows.

template<typename _Tag>
  struct _Enable_default_constructor<false, _Tag>
  {
    constexpr _Enable_default_constructor() noexcept = delete;
    constexpr _Enable_default_constructor(_Enable_default_constructor const&)
      noexcept  = default;
    constexpr _Enable_default_constructor(_Enable_default_constructor&&)
      noexcept = default;
    _Enable_default_constructor&
    operator=(_Enable_default_constructor const&) noexcept = default;
    _Enable_default_constructor&
    operator=(_Enable_default_constructor&&) noexcept = default;

    // Can be used in other ctors.
    constexpr explicit
    _Enable_default_constructor(_Enable_default_constructor_tag) { }
  };

template<typename _Tag>
  struct _Enable_destructor<false, _Tag>
  { ~_Enable_destructor() noexcept = delete; };

template<typename _Tag>
  struct _Enable_copy_move<false, true, true, true, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = default;
  };

template<typename _Tag>
  struct _Enable_copy_move<true, false, true, true, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = default;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = default;
  };

template<typename _Tag>
  struct _Enable_copy_move<false, false, true, true, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = default;
  };

template<typename _Tag>
  struct _Enable_copy_move<true, true, false, true, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = default;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = default;
  };

template<typename _Tag>
  struct _Enable_copy_move<false, true, false, true, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = default;
  };

template<typename _Tag>
  struct _Enable_copy_move<true, false, false, true, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = default;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = default;
  };

template<typename _Tag>
  struct _Enable_copy_move<false, false, false, true, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = default;
  };

template<typename _Tag>
  struct _Enable_copy_move<true, true, true, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = default;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

template<typename _Tag>
  struct _Enable_copy_move<false, true, true, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

template<typename _Tag>
  struct _Enable_copy_move<true, false, true, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = default;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

template<typename _Tag>
  struct _Enable_copy_move<false, false, true, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

template<typename _Tag>
  struct _Enable_copy_move<true, true, false, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = default;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

template<typename _Tag>
  struct _Enable_copy_move<false, true, false, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = default;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

template<typename _Tag>
  struct _Enable_copy_move<true, false, false, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = default;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

template<typename _Tag>
  struct _Enable_copy_move<false, false, false, false, _Tag>
  {
    constexpr _Enable_copy_move() noexcept                          = default;
    constexpr _Enable_copy_move(_Enable_copy_move const&) noexcept  = delete;
    constexpr _Enable_copy_move(_Enable_copy_move&&) noexcept       = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move const&) noexcept                    = delete;
    _Enable_copy_move&
    operator=(_Enable_copy_move&&) noexcept                         = delete;
  };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // _ENABLE_SPECIAL_MEMBERS_H
