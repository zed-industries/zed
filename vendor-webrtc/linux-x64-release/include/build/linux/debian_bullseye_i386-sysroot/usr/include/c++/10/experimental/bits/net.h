// Networking implementation details -*- C++ -*-

// Copyright (C) 2015-2020 Free Software Foundation, Inc.
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

/** @file experimental/bits/net.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{experimental/net}
 */

#ifndef _GLIBCXX_EXPERIMENTAL_NET_H
#define _GLIBCXX_EXPERIMENTAL_NET_H 1

#pragma GCC system_header

#if __cplusplus >= 201402L

#include <type_traits>
#include <system_error>
#include <experimental/netfwd>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
namespace experimental
{
namespace net
{
inline namespace v1
{

  /** @addtogroup networking-ts
   *  @{
   */

  template<typename _CompletionToken, typename _Signature, typename>
    class async_result;

  /// @cond undocumented

  // A type denoted by DEDUCED in the TS.
  template<typename _CompletionToken, typename _Signature>
    using __deduced_t = typename
      async_result<decay_t<_CompletionToken>, _Signature, void>::return_type;

  // Trait to check for construction from const/non-const lvalue/rvalue.
  template<typename _Tp>
    using __is_value_constructible = typename __and_<
      is_copy_constructible<_Tp>, is_move_constructible<_Tp>,
      is_constructible<_Tp, _Tp&>, is_constructible<_Tp, const _Tp&&>
      >::type;

  struct __throw_on_error
  {
    explicit
    __throw_on_error(const char* __msg) : _M_msg(__msg) { }

    ~__throw_on_error() noexcept(false)
    {
      if (_M_ec)
	_GLIBCXX_THROW_OR_ABORT(system_error(_M_ec, _M_msg));
    }

    __throw_on_error(const __throw_on_error&) = delete;
    __throw_on_error& operator=(const __throw_on_error&) = delete;

    operator error_code&() noexcept { return _M_ec; }

    const char* _M_msg;
    error_code _M_ec;
  };

  /// @endcond

  // Base class for types meeting IntegerSocketOption requirements.
  template<typename _Tp>
    struct __sockopt_base
    {
      __sockopt_base() = default;

      explicit __sockopt_base(int __val) : _M_value(__val) { }

      int value() const noexcept { return _M_value; }

      template<typename _Protocol>
	void*
	data(const _Protocol&) noexcept
	{ return std::addressof(_M_value); }

      template<typename _Protocol>
	const void*
	data(const _Protocol&) const noexcept
	{ return std::addressof(_M_value); }

      template<typename _Protocol>
	size_t
	size(const _Protocol&) const noexcept
	{ return sizeof(_M_value); }

      template<typename _Protocol>
	void
	resize(const _Protocol&, size_t __s)
	{
	  if (__s != sizeof(_M_value))
	    __throw_length_error("invalid value for socket option resize");
	}

    protected:
      _Tp _M_value { };
    };

  // Base class for types meeting BooleanSocketOption requirements.
  template<>
    struct __sockopt_base<bool> : __sockopt_base<int>
    {
      __sockopt_base() = default;

      explicit __sockopt_base(bool __val) : __sockopt_base<int>(__val) { }

      bool value() const noexcept { return __sockopt_base<int>::_M_value; }
      explicit operator bool() const noexcept { return value(); }
      bool operator!() const noexcept { return !value(); }
    };

  template<typename _Derived, typename _Tp = int>
    struct __sockopt_crtp : __sockopt_base<_Tp>
    {
      using __sockopt_base<_Tp>::__sockopt_base;

      _Derived&
      operator=(_Tp __value)
      {
	__sockopt_base<_Tp>::_M_value = __value;
	return static_cast<_Derived&>(*this);
      }

      template<typename _Protocol>
	int
	level(const _Protocol&) const noexcept
	{ return _Derived::_S_level; }

      template<typename _Protocol>
	int
	name(const _Protocol&) const noexcept
	{ return _Derived::_S_name; }
    };

  /// @}

} // namespace v1
} // namespace net
} // namespace experimental
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++14

#endif // _GLIBCXX_EXPERIMENTAL_NET_H
