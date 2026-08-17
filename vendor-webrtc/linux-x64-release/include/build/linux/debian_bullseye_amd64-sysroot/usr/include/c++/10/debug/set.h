// Debugging set implementation -*- C++ -*-

// Copyright (C) 2003-2020 Free Software Foundation, Inc.
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

/** @file debug/set.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_SET_H
#define _GLIBCXX_DEBUG_SET_H 1

#include <debug/safe_sequence.h>
#include <debug/safe_container.h>
#include <debug/safe_iterator.h>
#include <utility>

namespace std _GLIBCXX_VISIBILITY(default)
{
namespace __debug
{
  /// Class std::set with safety/checking/debug instrumentation.
  template<typename _Key, typename _Compare = std::less<_Key>,
	   typename _Allocator = std::allocator<_Key> >
    class set
    : public __gnu_debug::_Safe_container<
	set<_Key, _Compare, _Allocator>, _Allocator,
	__gnu_debug::_Safe_node_sequence>,
      public _GLIBCXX_STD_C::set<_Key,_Compare,_Allocator>
    {
      typedef _GLIBCXX_STD_C::set<_Key, _Compare, _Allocator>	_Base;
      typedef __gnu_debug::_Safe_container<
	set, _Allocator, __gnu_debug::_Safe_node_sequence>	_Safe;

      typedef typename _Base::const_iterator	_Base_const_iterator;
      typedef typename _Base::iterator		_Base_iterator;
      typedef __gnu_debug::_Equal_to<_Base_const_iterator> _Equal;

      template<typename _ItT, typename _SeqT, typename _CatT>
	friend class ::__gnu_debug::_Safe_iterator;

    public:
      // types:
      typedef _Key					key_type;
      typedef _Key					value_type;
      typedef _Compare					key_compare;
      typedef _Compare					value_compare;
      typedef _Allocator				allocator_type;
      typedef typename _Base::reference			reference;
      typedef typename _Base::const_reference		const_reference;

      typedef __gnu_debug::_Safe_iterator<_Base_iterator, set>
							iterator;
      typedef __gnu_debug::_Safe_iterator<_Base_const_iterator, set>
							const_iterator;

      typedef typename _Base::size_type			size_type;
      typedef typename _Base::difference_type		difference_type;
      typedef typename _Base::pointer			pointer;
      typedef typename _Base::const_pointer		const_pointer;
      typedef std::reverse_iterator<iterator>		reverse_iterator;
      typedef std::reverse_iterator<const_iterator>	const_reverse_iterator;

      // 23.3.3.1 construct/copy/destroy:

#if __cplusplus < 201103L
      set() : _Base() { }

      set(const set& __x)
      : _Base(__x) { }

      ~set() { }
#else
      set() = default;
      set(const set&) = default;
      set(set&&) = default;

      set(initializer_list<value_type> __l,
	  const _Compare& __comp = _Compare(),
	  const allocator_type& __a = allocator_type())
      : _Base(__l, __comp, __a) { }

      explicit
      set(const allocator_type& __a)
      : _Base(__a) { }

      set(const set& __x, const allocator_type& __a)
      : _Base(__x, __a) { }

      set(set&& __x, const allocator_type& __a)
      noexcept( noexcept(_Base(std::move(__x._M_base()), __a)) )
      : _Safe(std::move(__x._M_safe()), __a),
	_Base(std::move(__x._M_base()), __a) { }

      set(initializer_list<value_type> __l, const allocator_type& __a)
      : _Base(__l, __a) { }

      template<typename _InputIterator>
	set(_InputIterator __first, _InputIterator __last,
	    const allocator_type& __a)
	: _Base(__gnu_debug::__base(
		  __glibcxx_check_valid_constructor_range(__first, __last)),
		__gnu_debug::__base(__last), __a) { }

      ~set() = default;
#endif

      explicit set(const _Compare& __comp,
		   const _Allocator& __a = _Allocator())
      : _Base(__comp, __a) { }

      template<typename _InputIterator>
	set(_InputIterator __first, _InputIterator __last,
	    const _Compare& __comp = _Compare(),
	    const _Allocator& __a = _Allocator())
	: _Base(__gnu_debug::__base(
		  __glibcxx_check_valid_constructor_range(__first, __last)),
		__gnu_debug::__base(__last),
		__comp, __a) { }

      set(const _Base& __x)
      : _Base(__x) { }

#if __cplusplus < 201103L
      set&
      operator=(const set& __x)
      {
	this->_M_safe() = __x;
	_M_base() = __x;
	return *this;
      }
#else
      set&
      operator=(const set&) = default;

      set&
      operator=(set&&) = default;

      set&
      operator=(initializer_list<value_type> __l)
      {
	_M_base() = __l;
	this->_M_invalidate_all();
	return *this;
      }
#endif

      using _Base::get_allocator;

      // iterators:
      iterator
      begin() _GLIBCXX_NOEXCEPT
      { return iterator(_Base::begin(), this); }

      const_iterator
      begin() const _GLIBCXX_NOEXCEPT
      { return const_iterator(_Base::begin(), this); }

      iterator
      end() _GLIBCXX_NOEXCEPT
      { return iterator(_Base::end(), this); }

      const_iterator
      end() const _GLIBCXX_NOEXCEPT
      { return const_iterator(_Base::end(), this); }

      reverse_iterator
      rbegin() _GLIBCXX_NOEXCEPT
      { return reverse_iterator(end()); }

      const_reverse_iterator
      rbegin() const _GLIBCXX_NOEXCEPT
      { return const_reverse_iterator(end()); }

      reverse_iterator
      rend() _GLIBCXX_NOEXCEPT
      { return reverse_iterator(begin()); }

      const_reverse_iterator
      rend() const _GLIBCXX_NOEXCEPT
      { return const_reverse_iterator(begin()); }

#if __cplusplus >= 201103L
      const_iterator
      cbegin() const noexcept
      { return const_iterator(_Base::begin(), this); }

      const_iterator
      cend() const noexcept
      { return const_iterator(_Base::end(), this); }

      const_reverse_iterator
      crbegin() const noexcept
      { return const_reverse_iterator(end()); }

      const_reverse_iterator
      crend() const noexcept
      { return const_reverse_iterator(begin()); }
#endif

      // capacity:
      using _Base::empty;
      using _Base::size;
      using _Base::max_size;

      // modifiers:
#if __cplusplus >= 201103L
      template<typename... _Args>
	std::pair<iterator, bool>
	emplace(_Args&&... __args)
	{
	  auto __res = _Base::emplace(std::forward<_Args>(__args)...);
	  return { { __res.first, this }, __res.second };
	}

      template<typename... _Args>
	iterator
	emplace_hint(const_iterator __pos, _Args&&... __args)
	{
	  __glibcxx_check_insert(__pos);
	  return
	    {
	      _Base::emplace_hint(__pos.base(), std::forward<_Args>(__args)...),
	      this
	    };
	}
#endif

      std::pair<iterator, bool>
      insert(const value_type& __x)
      {
	std::pair<_Base_iterator, bool> __res = _Base::insert(__x);
	return std::pair<iterator, bool>(iterator(__res.first, this),
					 __res.second);
      }

#if __cplusplus >= 201103L
      std::pair<iterator, bool>
      insert(value_type&& __x)
      {
	auto __res = _Base::insert(std::move(__x));
	return { { __res.first, this }, __res.second };
      }
#endif

      iterator
      insert(const_iterator __position, const value_type& __x)
      {
	__glibcxx_check_insert(__position);
	return iterator(_Base::insert(__position.base(), __x), this);
      }

#if __cplusplus >= 201103L
      iterator
      insert(const_iterator __position, value_type&& __x)
      {
	__glibcxx_check_insert(__position);
	return { _Base::insert(__position.base(), std::move(__x)), this };
      }
#endif

      template <typename _InputIterator>
	void
	insert(_InputIterator __first, _InputIterator __last)
	{
	  typename __gnu_debug::_Distance_traits<_InputIterator>::__type __dist;
	  __glibcxx_check_valid_range2(__first, __last, __dist);

	  if (__dist.second >= __gnu_debug::__dp_sign)
	    _Base::insert(__gnu_debug::__unsafe(__first),
			  __gnu_debug::__unsafe(__last));
	  else
	    _Base::insert(__first, __last);
	}

#if __cplusplus >= 201103L
      void
      insert(initializer_list<value_type> __l)
      { _Base::insert(__l); }
#endif

#if __cplusplus > 201402L
      using node_type = typename _Base::node_type;
      using insert_return_type = _Node_insert_return<iterator, node_type>;

      node_type
      extract(const_iterator __position)
      {
	__glibcxx_check_erase(__position);
	this->_M_invalidate_if(_Equal(__position.base()));
	return _Base::extract(__position.base());
      }

      node_type
      extract(const key_type& __key)
      {
	const auto __position = find(__key);
	if (__position != end())
	  return extract(__position);
	return {};
      }

      insert_return_type
      insert(node_type&& __nh)
      {
	auto __ret = _Base::insert(std::move(__nh));
	iterator __pos = iterator(__ret.position, this);
	return { __pos, __ret.inserted, std::move(__ret.node) };
      }

      iterator
      insert(const_iterator __hint, node_type&& __nh)
      {
	__glibcxx_check_insert(__hint);
	return { _Base::insert(__hint.base(), std::move(__nh)), this };
      }

      using _Base::merge;
#endif // C++17

#if __cplusplus >= 201103L
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(const_iterator __position)
      {
	__glibcxx_check_erase(__position);
	this->_M_invalidate_if(_Equal(__position.base()));
	return { _Base::erase(__position.base()), this };
      }
#else
      void
      erase(iterator __position)
      {
	__glibcxx_check_erase(__position);
	this->_M_invalidate_if(_Equal(__position.base()));
	_Base::erase(__position.base());
      }
#endif

      size_type
      erase(const key_type& __x)
      {
	_Base_iterator __victim = _Base::find(__x);
	if (__victim == _Base::end())
	  return 0;
	else
	  {
	    this->_M_invalidate_if(_Equal(__victim));
	    _Base::erase(__victim);
	    return 1;
	  }
      }

#if __cplusplus >= 201103L
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(const_iterator __first, const_iterator __last)
      {
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// 151. can't currently clear() empty container
	__glibcxx_check_erase_range(__first, __last);
	for (_Base_const_iterator __victim = __first.base();
	     __victim != __last.base(); ++__victim)
	  {
	    _GLIBCXX_DEBUG_VERIFY(__victim != _Base::cend(),
				  _M_message(__gnu_debug::__msg_valid_range)
				  ._M_iterator(__first, "first")
				  ._M_iterator(__last, "last"));
	    this->_M_invalidate_if(_Equal(__victim));
	  }

	return { _Base::erase(__first.base(), __last.base()), this };
      }
#else
      void
      erase(iterator __first, iterator __last)
      {
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// 151. can't currently clear() empty container
	__glibcxx_check_erase_range(__first, __last);
	for (_Base_iterator __victim = __first.base();
	     __victim != __last.base(); ++__victim)
	  {
	    _GLIBCXX_DEBUG_VERIFY(__victim != _Base::end(),
				  _M_message(__gnu_debug::__msg_valid_range)
				  ._M_iterator(__first, "first")
				  ._M_iterator(__last, "last"));
	    this->_M_invalidate_if(_Equal(__victim));
	  }
	_Base::erase(__first.base(), __last.base());
      }
#endif

      void
      swap(set& __x)
      _GLIBCXX_NOEXCEPT_IF( noexcept(declval<_Base&>().swap(__x)) )
      {
	_Safe::_M_swap(__x);
	_Base::swap(__x);
      }

      void
      clear() _GLIBCXX_NOEXCEPT
      {
	this->_M_invalidate_all();
	_Base::clear();
      }

      // observers:
      using _Base::key_comp;
      using _Base::value_comp;

      // set operations:
      iterator
      find(const key_type& __x)
      { return iterator(_Base::find(__x), this); }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 214. set::find() missing const overload
      const_iterator
      find(const key_type& __x) const
      { return const_iterator(_Base::find(__x), this); }

#if __cplusplus > 201103L
      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	iterator
	find(const _Kt& __x)
	{ return { _Base::find(__x), this }; }

      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	const_iterator
	find(const _Kt& __x) const
	{ return { _Base::find(__x), this }; }
#endif

      using _Base::count;

      iterator
      lower_bound(const key_type& __x)
      { return iterator(_Base::lower_bound(__x), this); }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 214. set::find() missing const overload
      const_iterator
      lower_bound(const key_type& __x) const
      { return const_iterator(_Base::lower_bound(__x), this); }

#if __cplusplus > 201103L
      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	iterator
	lower_bound(const _Kt& __x)
	{ return { _Base::lower_bound(__x), this }; }

      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	const_iterator
	lower_bound(const _Kt& __x) const
	{ return { _Base::lower_bound(__x), this }; }
#endif

      iterator
      upper_bound(const key_type& __x)
      { return iterator(_Base::upper_bound(__x), this); }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 214. set::find() missing const overload
      const_iterator
      upper_bound(const key_type& __x) const
      { return const_iterator(_Base::upper_bound(__x), this); }

#if __cplusplus > 201103L
      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	iterator
	upper_bound(const _Kt& __x)
	{ return { _Base::upper_bound(__x), this }; }

      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	const_iterator
	upper_bound(const _Kt& __x) const
	{ return { _Base::upper_bound(__x), this }; }
#endif

      std::pair<iterator, iterator>
      equal_range(const key_type& __x)
      {
	std::pair<_Base_iterator, _Base_iterator> __res =
	_Base::equal_range(__x);
	return std::make_pair(iterator(__res.first, this),
			      iterator(__res.second, this));
      }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 214. set::find() missing const overload
      std::pair<const_iterator, const_iterator>
      equal_range(const key_type& __x) const
      {
	std::pair<_Base_const_iterator, _Base_const_iterator> __res =
	_Base::equal_range(__x);
	return std::make_pair(const_iterator(__res.first, this),
			      const_iterator(__res.second, this));
      }

#if __cplusplus > 201103L
      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	std::pair<iterator, iterator>
	equal_range(const _Kt& __x)
	{
	  auto __res = _Base::equal_range(__x);
	  return { { __res.first, this }, { __res.second, this } };
	}

      template<typename _Kt,
	       typename _Req =
		 typename __has_is_transparent<_Compare, _Kt>::type>
	std::pair<const_iterator, const_iterator>
	equal_range(const _Kt& __x) const
	{
	  auto __res = _Base::equal_range(__x);
	  return { { __res.first, this }, { __res.second, this } };
	}
#endif

      _Base&
      _M_base() _GLIBCXX_NOEXCEPT	{ return *this; }

      const _Base&
      _M_base() const _GLIBCXX_NOEXCEPT	{ return *this; }
    };

#if __cpp_deduction_guides >= 201606

  template<typename _InputIterator,
	   typename _Compare =
	     less<typename iterator_traits<_InputIterator>::value_type>,
	   typename _Allocator =
	     allocator<typename iterator_traits<_InputIterator>::value_type>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireAllocator<_Allocator>>
    set(_InputIterator, _InputIterator,
	_Compare = _Compare(), _Allocator = _Allocator())
    -> set<typename iterator_traits<_InputIterator>::value_type,
	   _Compare, _Allocator>;

  template<typename _Key, typename _Compare = less<_Key>,
	   typename _Allocator = allocator<_Key>,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireAllocator<_Allocator>>
    set(initializer_list<_Key>,
	_Compare = _Compare(), _Allocator = _Allocator())
    -> set<_Key, _Compare, _Allocator>;

  template<typename _InputIterator, typename _Allocator,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireAllocator<_Allocator>>
    set(_InputIterator, _InputIterator, _Allocator)
    -> set<typename iterator_traits<_InputIterator>::value_type,
	   less<typename iterator_traits<_InputIterator>::value_type>,
	   _Allocator>;

  template<typename _Key, typename _Allocator,
	   typename = _RequireAllocator<_Allocator>>
    set(initializer_list<_Key>, _Allocator)
    -> set<_Key, less<_Key>, _Allocator>;

#endif // deduction guides

  template<typename _Key, typename _Compare, typename _Allocator>
    inline bool
    operator==(const set<_Key, _Compare, _Allocator>& __lhs,
	       const set<_Key, _Compare, _Allocator>& __rhs)
    { return __lhs._M_base() == __rhs._M_base(); }

#if __cpp_lib_three_way_comparison
  template<typename _Key, typename _Compare, typename _Alloc>
    inline __detail::__synth3way_t<_Key>
    operator<=>(const set<_Key, _Compare, _Alloc>& __lhs,
		const set<_Key, _Compare, _Alloc>& __rhs)
    { return __lhs._M_base() <=> __rhs._M_base(); }
#else
  template<typename _Key, typename _Compare, typename _Allocator>
    inline bool
    operator!=(const set<_Key, _Compare, _Allocator>& __lhs,
	       const set<_Key, _Compare, _Allocator>& __rhs)
    { return __lhs._M_base() != __rhs._M_base(); }

  template<typename _Key, typename _Compare, typename _Allocator>
    inline bool
    operator<(const set<_Key, _Compare, _Allocator>& __lhs,
	      const set<_Key, _Compare, _Allocator>& __rhs)
    { return __lhs._M_base() < __rhs._M_base(); }

  template<typename _Key, typename _Compare, typename _Allocator>
    inline bool
    operator<=(const set<_Key, _Compare, _Allocator>& __lhs,
	       const set<_Key, _Compare, _Allocator>& __rhs)
    { return __lhs._M_base() <= __rhs._M_base(); }

  template<typename _Key, typename _Compare, typename _Allocator>
    inline bool
    operator>=(const set<_Key, _Compare, _Allocator>& __lhs,
	       const set<_Key, _Compare, _Allocator>& __rhs)
    { return __lhs._M_base() >= __rhs._M_base(); }

  template<typename _Key, typename _Compare, typename _Allocator>
    inline bool
    operator>(const set<_Key, _Compare, _Allocator>& __lhs,
	      const set<_Key, _Compare, _Allocator>& __rhs)
    { return __lhs._M_base() > __rhs._M_base(); }
#endif // three-way comparison

  template<typename _Key, typename _Compare, typename _Allocator>
    void
    swap(set<_Key, _Compare, _Allocator>& __x,
	 set<_Key, _Compare, _Allocator>& __y)
    _GLIBCXX_NOEXCEPT_IF(noexcept(__x.swap(__y)))
    { return __x.swap(__y); }

} // namespace __debug
} // namespace std

#endif
