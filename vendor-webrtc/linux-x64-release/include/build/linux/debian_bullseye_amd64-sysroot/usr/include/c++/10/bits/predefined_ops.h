// Default predicates for internal use -*- C++ -*-

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

/** @file predefined_ops.h
 *  This is an internal header file, included by other library headers.
 *  You should not attempt to use it directly. @headername{algorithm}
 */

#ifndef _GLIBCXX_PREDEFINED_OPS_H
#define _GLIBCXX_PREDEFINED_OPS_H	1

namespace __gnu_cxx
{
namespace __ops
{
  struct _Iter_less_iter
  {
    template<typename _Iterator1, typename _Iterator2>
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(_Iterator1 __it1, _Iterator2 __it2) const
      { return *__it1 < *__it2; }
  };

  _GLIBCXX14_CONSTEXPR
  inline _Iter_less_iter
  __iter_less_iter()
  { return _Iter_less_iter(); }

  struct _Iter_less_val
  {
#if __cplusplus >= 201103L
    constexpr _Iter_less_val() = default;
#else
    _Iter_less_val() { }
#endif

    _GLIBCXX20_CONSTEXPR
    explicit
    _Iter_less_val(_Iter_less_iter) { }

    template<typename _Iterator, typename _Value>
      _GLIBCXX20_CONSTEXPR
      bool
      operator()(_Iterator __it, _Value& __val) const
      { return *__it < __val; }
  };

  _GLIBCXX20_CONSTEXPR
  inline _Iter_less_val
  __iter_less_val()
  { return _Iter_less_val(); }

  _GLIBCXX20_CONSTEXPR
  inline _Iter_less_val
  __iter_comp_val(_Iter_less_iter)
  { return _Iter_less_val(); }

  struct _Val_less_iter
  {
#if __cplusplus >= 201103L
    constexpr _Val_less_iter() = default;
#else
    _Val_less_iter() { }
#endif

    _GLIBCXX20_CONSTEXPR
    explicit
    _Val_less_iter(_Iter_less_iter) { }

    template<typename _Value, typename _Iterator>
      _GLIBCXX20_CONSTEXPR
      bool
      operator()(_Value& __val, _Iterator __it) const
      { return __val < *__it; }
  };

  _GLIBCXX20_CONSTEXPR
  inline _Val_less_iter
  __val_less_iter()
  { return _Val_less_iter(); }

  _GLIBCXX20_CONSTEXPR
  inline _Val_less_iter
  __val_comp_iter(_Iter_less_iter)
  { return _Val_less_iter(); }

  struct _Iter_equal_to_iter
  {
    template<typename _Iterator1, typename _Iterator2>
      _GLIBCXX20_CONSTEXPR
      bool
      operator()(_Iterator1 __it1, _Iterator2 __it2) const
      { return *__it1 == *__it2; }
  };

  _GLIBCXX20_CONSTEXPR
  inline _Iter_equal_to_iter
  __iter_equal_to_iter()
  { return _Iter_equal_to_iter(); }

  struct _Iter_equal_to_val
  {
    template<typename _Iterator, typename _Value>
      _GLIBCXX20_CONSTEXPR
      bool
      operator()(_Iterator __it, _Value& __val) const
      { return *__it == __val; }
  };

  _GLIBCXX20_CONSTEXPR
  inline _Iter_equal_to_val
  __iter_equal_to_val()
  { return _Iter_equal_to_val(); }

  _GLIBCXX20_CONSTEXPR
  inline _Iter_equal_to_val
  __iter_comp_val(_Iter_equal_to_iter)
  { return _Iter_equal_to_val(); }

  template<typename _Compare>
    struct _Iter_comp_iter
    {
      _Compare _M_comp;

      explicit _GLIBCXX14_CONSTEXPR
      _Iter_comp_iter(_Compare __comp)
	: _M_comp(_GLIBCXX_MOVE(__comp))
      { }

      template<typename _Iterator1, typename _Iterator2>
        _GLIBCXX14_CONSTEXPR
        bool
        operator()(_Iterator1 __it1, _Iterator2 __it2)
        { return bool(_M_comp(*__it1, *__it2)); }
    };

  template<typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline _Iter_comp_iter<_Compare>
    __iter_comp_iter(_Compare __comp)
    { return _Iter_comp_iter<_Compare>(_GLIBCXX_MOVE(__comp)); }

  template<typename _Compare>
    struct _Iter_comp_val
    {
      _Compare _M_comp;

      _GLIBCXX20_CONSTEXPR
      explicit
      _Iter_comp_val(_Compare __comp)
	: _M_comp(_GLIBCXX_MOVE(__comp))
      { }

      _GLIBCXX20_CONSTEXPR
      explicit
      _Iter_comp_val(const _Iter_comp_iter<_Compare>& __comp)
	: _M_comp(__comp._M_comp)
      { }

#if __cplusplus >= 201103L
      _GLIBCXX20_CONSTEXPR
      explicit
      _Iter_comp_val(_Iter_comp_iter<_Compare>&& __comp)
	: _M_comp(std::move(__comp._M_comp))
      { }
#endif

      template<typename _Iterator, typename _Value>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Iterator __it, _Value& __val)
	{ return bool(_M_comp(*__it, __val)); }
    };

  template<typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _Iter_comp_val<_Compare>
    __iter_comp_val(_Compare __comp)
    { return _Iter_comp_val<_Compare>(_GLIBCXX_MOVE(__comp)); }

  template<typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _Iter_comp_val<_Compare>
    __iter_comp_val(_Iter_comp_iter<_Compare> __comp)
    { return _Iter_comp_val<_Compare>(_GLIBCXX_MOVE(__comp)); }

  template<typename _Compare>
    struct _Val_comp_iter
    {
      _Compare _M_comp;

      _GLIBCXX20_CONSTEXPR
      explicit
      _Val_comp_iter(_Compare __comp)
	: _M_comp(_GLIBCXX_MOVE(__comp))
      { }

      _GLIBCXX20_CONSTEXPR
      explicit
      _Val_comp_iter(const _Iter_comp_iter<_Compare>& __comp)
	: _M_comp(__comp._M_comp)
      { }

#if __cplusplus >= 201103L
      _GLIBCXX20_CONSTEXPR
      explicit
      _Val_comp_iter(_Iter_comp_iter<_Compare>&& __comp)
	: _M_comp(std::move(__comp._M_comp))
      { }
#endif

      template<typename _Value, typename _Iterator>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Value& __val, _Iterator __it)
	{ return bool(_M_comp(__val, *__it)); }
    };

  template<typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _Val_comp_iter<_Compare>
    __val_comp_iter(_Compare __comp)
    { return _Val_comp_iter<_Compare>(_GLIBCXX_MOVE(__comp)); }

  template<typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _Val_comp_iter<_Compare>
    __val_comp_iter(_Iter_comp_iter<_Compare> __comp)
    { return _Val_comp_iter<_Compare>(_GLIBCXX_MOVE(__comp)); }

  template<typename _Value>
    struct _Iter_equals_val
    {
      _Value& _M_value;

      _GLIBCXX20_CONSTEXPR
      explicit
      _Iter_equals_val(_Value& __value)
	: _M_value(__value)
      { }

      template<typename _Iterator>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Iterator __it)
	{ return *__it == _M_value; }
    };

  template<typename _Value>
    _GLIBCXX20_CONSTEXPR
    inline _Iter_equals_val<_Value>
    __iter_equals_val(_Value& __val)
    { return _Iter_equals_val<_Value>(__val); }

  template<typename _Iterator1>
    struct _Iter_equals_iter
    {
      _Iterator1 _M_it1;

      _GLIBCXX20_CONSTEXPR
      explicit
      _Iter_equals_iter(_Iterator1 __it1)
	: _M_it1(__it1)
      { }

      template<typename _Iterator2>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Iterator2 __it2)
	{ return *__it2 == *_M_it1; }
    };

  template<typename _Iterator>
    _GLIBCXX20_CONSTEXPR
    inline _Iter_equals_iter<_Iterator>
    __iter_comp_iter(_Iter_equal_to_iter, _Iterator __it)
    { return _Iter_equals_iter<_Iterator>(__it); }

  template<typename _Predicate>
    struct _Iter_pred
    {
      _Predicate _M_pred;

      _GLIBCXX20_CONSTEXPR
      explicit
      _Iter_pred(_Predicate __pred)
	: _M_pred(_GLIBCXX_MOVE(__pred))
      { }

      template<typename _Iterator>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Iterator __it)
	{ return bool(_M_pred(*__it)); }
    };

  template<typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _Iter_pred<_Predicate>
    __pred_iter(_Predicate __pred)
    { return _Iter_pred<_Predicate>(_GLIBCXX_MOVE(__pred)); }

  template<typename _Compare, typename _Value>
    struct _Iter_comp_to_val
    {
      _Compare _M_comp;
      _Value& _M_value;

      _GLIBCXX20_CONSTEXPR
      _Iter_comp_to_val(_Compare __comp, _Value& __value)
	: _M_comp(_GLIBCXX_MOVE(__comp)), _M_value(__value)
      { }

      template<typename _Iterator>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Iterator __it)
	{ return bool(_M_comp(*__it, _M_value)); }
    };

  template<typename _Compare, typename _Value>
    _Iter_comp_to_val<_Compare, _Value>
    _GLIBCXX20_CONSTEXPR
    __iter_comp_val(_Compare __comp, _Value &__val)
    {
      return _Iter_comp_to_val<_Compare, _Value>(_GLIBCXX_MOVE(__comp), __val);
    }

  template<typename _Compare, typename _Iterator1>
    struct _Iter_comp_to_iter
    {
      _Compare _M_comp;
      _Iterator1 _M_it1;

      _GLIBCXX20_CONSTEXPR
      _Iter_comp_to_iter(_Compare __comp, _Iterator1 __it1)
	: _M_comp(_GLIBCXX_MOVE(__comp)), _M_it1(__it1)
      { }

      template<typename _Iterator2>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Iterator2 __it2)
	{ return bool(_M_comp(*__it2, *_M_it1)); }
    };

  template<typename _Compare, typename _Iterator>
    _GLIBCXX20_CONSTEXPR
    inline _Iter_comp_to_iter<_Compare, _Iterator>
    __iter_comp_iter(_Iter_comp_iter<_Compare> __comp, _Iterator __it)
    {
      return _Iter_comp_to_iter<_Compare, _Iterator>(
	  _GLIBCXX_MOVE(__comp._M_comp), __it);
    }

  template<typename _Predicate>
    struct _Iter_negate
    {
      _Predicate _M_pred;

      _GLIBCXX20_CONSTEXPR
      explicit
      _Iter_negate(_Predicate __pred)
	: _M_pred(_GLIBCXX_MOVE(__pred))
      { }

      template<typename _Iterator>
	_GLIBCXX20_CONSTEXPR
	bool
	operator()(_Iterator __it)
	{ return !bool(_M_pred(*__it)); }
    };

  template<typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _Iter_negate<_Predicate>
    __negate(_Iter_pred<_Predicate> __pred)
    { return _Iter_negate<_Predicate>(_GLIBCXX_MOVE(__pred._M_pred)); }

} // namespace __ops
} // namespace __gnu_cxx

#endif
