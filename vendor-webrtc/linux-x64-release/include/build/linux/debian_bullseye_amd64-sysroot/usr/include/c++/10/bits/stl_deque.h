// Deque implementation -*- C++ -*-

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

/*
 *
 * Copyright (c) 1994
 * Hewlett-Packard Company
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Hewlett-Packard Company makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 *
 * Copyright (c) 1997
 * Silicon Graphics Computer Systems, Inc.
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Silicon Graphics makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 */

/** @file bits/stl_deque.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{deque}
 */

#ifndef _STL_DEQUE_H
#define _STL_DEQUE_H 1

#include <bits/concept_check.h>
#include <bits/stl_iterator_base_types.h>
#include <bits/stl_iterator_base_funcs.h>
#if __cplusplus >= 201103L
#include <initializer_list>
#include <bits/stl_uninitialized.h> // for __is_bitwise_relocatable
#endif
#if __cplusplus > 201703L
# include <compare>
#endif

#include <debug/assertions.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
_GLIBCXX_BEGIN_NAMESPACE_CONTAINER

  /**
   *  @brief This function controls the size of memory nodes.
   *  @param  __size  The size of an element.
   *  @return   The number (not byte size) of elements per node.
   *
   *  This function started off as a compiler kludge from SGI, but
   *  seems to be a useful wrapper around a repeated constant
   *  expression.  The @b 512 is tunable (and no other code needs to
   *  change), but no investigation has been done since inheriting the
   *  SGI code.  Touch _GLIBCXX_DEQUE_BUF_SIZE only if you know what
   *  you are doing, however: changing it breaks the binary
   *  compatibility!!
  */

#ifndef _GLIBCXX_DEQUE_BUF_SIZE
#define _GLIBCXX_DEQUE_BUF_SIZE 512
#endif

  _GLIBCXX_CONSTEXPR inline size_t
  __deque_buf_size(size_t __size)
  { return (__size < _GLIBCXX_DEQUE_BUF_SIZE
	    ? size_t(_GLIBCXX_DEQUE_BUF_SIZE / __size) : size_t(1)); }


  /**
   *  @brief A deque::iterator.
   *
   *  Quite a bit of intelligence here.  Much of the functionality of
   *  deque is actually passed off to this class.  A deque holds two
   *  of these internally, marking its valid range.  Access to
   *  elements is done as offsets of either of those two, relying on
   *  operator overloading in this class.
   *
   *  All the functions are op overloads except for _M_set_node.
  */
  template<typename _Tp, typename _Ref, typename _Ptr>
    struct _Deque_iterator
    {
#if __cplusplus < 201103L
      typedef _Deque_iterator<_Tp, _Tp&, _Tp*>		   iterator;
      typedef _Deque_iterator<_Tp, const _Tp&, const _Tp*> const_iterator;
      typedef _Tp*					   _Elt_pointer;
      typedef _Tp**					   _Map_pointer;
#else
    private:
      template<typename _CvTp>
	using __iter = _Deque_iterator<_Tp, _CvTp&, __ptr_rebind<_Ptr, _CvTp>>;
    public:
      typedef __iter<_Tp>				   iterator;
      typedef __iter<const _Tp>				   const_iterator;
      typedef __ptr_rebind<_Ptr, _Tp>			   _Elt_pointer;
      typedef __ptr_rebind<_Ptr, _Elt_pointer>		   _Map_pointer;
#endif

      static size_t _S_buffer_size() _GLIBCXX_NOEXCEPT
      { return __deque_buf_size(sizeof(_Tp)); }

      typedef std::random_access_iterator_tag	iterator_category;
      typedef _Tp				value_type;
      typedef _Ptr				pointer;
      typedef _Ref				reference;
      typedef size_t				size_type;
      typedef ptrdiff_t				difference_type;
      typedef _Deque_iterator			_Self;

      _Elt_pointer _M_cur;
      _Elt_pointer _M_first;
      _Elt_pointer _M_last;
      _Map_pointer _M_node;

      _Deque_iterator(_Elt_pointer __x, _Map_pointer __y) _GLIBCXX_NOEXCEPT
      : _M_cur(__x), _M_first(*__y),
	_M_last(*__y + _S_buffer_size()), _M_node(__y) { }

      _Deque_iterator() _GLIBCXX_NOEXCEPT
      : _M_cur(), _M_first(), _M_last(), _M_node() { }

#if __cplusplus < 201103L
      // Conversion from iterator to const_iterator.
      _Deque_iterator(const iterator& __x) _GLIBCXX_NOEXCEPT
      : _M_cur(__x._M_cur), _M_first(__x._M_first),
	_M_last(__x._M_last), _M_node(__x._M_node) { }
#else
      // Conversion from iterator to const_iterator.
      template<typename _Iter,
	       typename = _Require<is_same<_Self, const_iterator>,
				   is_same<_Iter, iterator>>>
       _Deque_iterator(const _Iter& __x) noexcept
       : _M_cur(__x._M_cur), _M_first(__x._M_first),
	 _M_last(__x._M_last), _M_node(__x._M_node) { }

      _Deque_iterator(const _Deque_iterator& __x) noexcept
       : _M_cur(__x._M_cur), _M_first(__x._M_first),
	 _M_last(__x._M_last), _M_node(__x._M_node) { }

      _Deque_iterator& operator=(const _Deque_iterator&) = default;
#endif

      iterator
      _M_const_cast() const _GLIBCXX_NOEXCEPT
      { return iterator(_M_cur, _M_node); }

      reference
      operator*() const _GLIBCXX_NOEXCEPT
      { return *_M_cur; }

      pointer
      operator->() const _GLIBCXX_NOEXCEPT
      { return _M_cur; }

      _Self&
      operator++() _GLIBCXX_NOEXCEPT
      {
	++_M_cur;
	if (_M_cur == _M_last)
	  {
	    _M_set_node(_M_node + 1);
	    _M_cur = _M_first;
	  }
	return *this;
      }

      _Self
      operator++(int) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = *this;
	++*this;
	return __tmp;
      }

      _Self&
      operator--() _GLIBCXX_NOEXCEPT
      {
	if (_M_cur == _M_first)
	  {
	    _M_set_node(_M_node - 1);
	    _M_cur = _M_last;
	  }
	--_M_cur;
	return *this;
      }

      _Self
      operator--(int) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = *this;
	--*this;
	return __tmp;
      }

      _Self&
      operator+=(difference_type __n) _GLIBCXX_NOEXCEPT
      {
	const difference_type __offset = __n + (_M_cur - _M_first);
	if (__offset >= 0 && __offset < difference_type(_S_buffer_size()))
	  _M_cur += __n;
	else
	  {
	    const difference_type __node_offset =
	      __offset > 0 ? __offset / difference_type(_S_buffer_size())
			   : -difference_type((-__offset - 1)
					      / _S_buffer_size()) - 1;
	    _M_set_node(_M_node + __node_offset);
	    _M_cur = _M_first + (__offset - __node_offset
				 * difference_type(_S_buffer_size()));
	  }
	return *this;
      }

      _Self&
      operator-=(difference_type __n) _GLIBCXX_NOEXCEPT
      { return *this += -__n; }

      reference
      operator[](difference_type __n) const _GLIBCXX_NOEXCEPT
      { return *(*this + __n); }

      /**
       *  Prepares to traverse new_node.  Sets everything except
       *  _M_cur, which should therefore be set by the caller
       *  immediately afterwards, based on _M_first and _M_last.
       */
      void
      _M_set_node(_Map_pointer __new_node) _GLIBCXX_NOEXCEPT
      {
	_M_node = __new_node;
	_M_first = *__new_node;
	_M_last = _M_first + difference_type(_S_buffer_size());
      }

      friend bool
      operator==(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return __x._M_cur == __y._M_cur; }

      // Note: we also provide overloads whose operands are of the same type in
      // order to avoid ambiguous overload resolution when std::rel_ops
      // operators are in scope (for additional details, see libstdc++/3628)
      template<typename _RefR, typename _PtrR>
	friend bool
	operator==(const _Self& __x,
		   const _Deque_iterator<_Tp, _RefR, _PtrR>& __y)
	_GLIBCXX_NOEXCEPT
	{ return __x._M_cur == __y._M_cur; }

#if __cpp_lib_three_way_comparison
      friend strong_ordering
      operator<=>(const _Self& __x, const _Self& __y) noexcept
      {
	if (const auto __cmp = __x._M_node <=> __y._M_node; __cmp != 0)
	  return __cmp;
	return __x._M_cur <=> __y._M_cur;
      }
#else
      friend bool
      operator!=(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return !(__x == __y); }

      template<typename _RefR, typename _PtrR>
	friend bool
	operator!=(const _Self& __x,
		   const _Deque_iterator<_Tp, _RefR, _PtrR>& __y)
	_GLIBCXX_NOEXCEPT
	{ return !(__x == __y); }

      friend bool
      operator<(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      {
	return (__x._M_node == __y._M_node)
	  ? (__x._M_cur < __y._M_cur) : (__x._M_node < __y._M_node);
      }

      template<typename _RefR, typename _PtrR>
	friend bool
	operator<(const _Self& __x,
		  const _Deque_iterator<_Tp, _RefR, _PtrR>& __y)
	_GLIBCXX_NOEXCEPT
	{
	  return (__x._M_node == __y._M_node)
	    ? (__x._M_cur < __y._M_cur) : (__x._M_node < __y._M_node);
	}

      friend bool
      operator>(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return __y < __x; }

      template<typename _RefR, typename _PtrR>
	friend bool
	operator>(const _Self& __x,
		  const _Deque_iterator<_Tp, _RefR, _PtrR>& __y)
	_GLIBCXX_NOEXCEPT
	{ return __y < __x; }

      friend bool
      operator<=(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return !(__y < __x); }

      template<typename _RefR, typename _PtrR>
	friend bool
	operator<=(const _Self& __x,
		   const _Deque_iterator<_Tp, _RefR, _PtrR>& __y)
	_GLIBCXX_NOEXCEPT
	{ return !(__y < __x); }

      friend bool
      operator>=(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return !(__x < __y); }

      template<typename _RefR, typename _PtrR>
	friend bool
	operator>=(const _Self& __x,
		   const _Deque_iterator<_Tp, _RefR, _PtrR>& __y)
	_GLIBCXX_NOEXCEPT
	{ return !(__x < __y); }
#endif // three-way comparison

      friend difference_type
      operator-(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      {
	return difference_type(_S_buffer_size())
	  * (__x._M_node - __y._M_node - 1) + (__x._M_cur - __x._M_first)
	  + (__y._M_last - __y._M_cur);
      }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // According to the resolution of DR179 not only the various comparison
      // operators but also operator- must accept mixed iterator/const_iterator
      // parameters.
      template<typename _RefR, typename _PtrR>
	friend difference_type
	operator-(const _Self& __x,
		  const _Deque_iterator<_Tp, _RefR, _PtrR>& __y) _GLIBCXX_NOEXCEPT
	{
	  return difference_type(_S_buffer_size())
	    * (__x._M_node - __y._M_node - 1) + (__x._M_cur - __x._M_first)
	    + (__y._M_last - __y._M_cur);
	}

      friend _Self
      operator+(const _Self& __x, difference_type __n) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = __x;
	__tmp += __n;
	return __tmp;
      }

      friend _Self
      operator-(const _Self& __x, difference_type __n) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = __x;
	__tmp -= __n;
	return __tmp;
      }

      friend _Self
      operator+(difference_type __n, const _Self& __x) _GLIBCXX_NOEXCEPT
      { return __x + __n; }
    };

  /**
   *  Deque base class.  This class provides the unified face for %deque's
   *  allocation.  This class's constructor and destructor allocate and
   *  deallocate (but do not initialize) storage.  This makes %exception
   *  safety easier.
   *
   *  Nothing in this class ever constructs or destroys an actual Tp element.
   *  (Deque handles that itself.)  Only/All memory management is performed
   *  here.
  */
  template<typename _Tp, typename _Alloc>
    class _Deque_base
    {
    protected:
      typedef typename __gnu_cxx::__alloc_traits<_Alloc>::template
	rebind<_Tp>::other _Tp_alloc_type;
      typedef __gnu_cxx::__alloc_traits<_Tp_alloc_type>	 _Alloc_traits;

#if __cplusplus < 201103L
      typedef _Tp*					_Ptr;
      typedef const _Tp*				_Ptr_const;
#else
      typedef typename _Alloc_traits::pointer		_Ptr;
      typedef typename _Alloc_traits::const_pointer	_Ptr_const;
#endif

      typedef typename _Alloc_traits::template rebind<_Ptr>::other
	_Map_alloc_type;
      typedef __gnu_cxx::__alloc_traits<_Map_alloc_type> _Map_alloc_traits;

      typedef _Alloc		  allocator_type;

      allocator_type
      get_allocator() const _GLIBCXX_NOEXCEPT
      { return allocator_type(_M_get_Tp_allocator()); }

      typedef _Deque_iterator<_Tp, _Tp&, _Ptr>	  iterator;
      typedef _Deque_iterator<_Tp, const _Tp&, _Ptr_const>   const_iterator;

      _Deque_base()
      : _M_impl()
      { _M_initialize_map(0); }

      _Deque_base(size_t __num_elements)
      : _M_impl()
      { _M_initialize_map(__num_elements); }

      _Deque_base(const allocator_type& __a, size_t __num_elements)
      : _M_impl(__a)
      { _M_initialize_map(__num_elements); }

      _Deque_base(const allocator_type& __a)
      : _M_impl(__a)
      { /* Caller must initialize map. */ }

#if __cplusplus >= 201103L
      _Deque_base(_Deque_base&& __x)
      : _M_impl(std::move(__x._M_get_Tp_allocator()))
      {
	_M_initialize_map(0);
	if (__x._M_impl._M_map)
	  this->_M_impl._M_swap_data(__x._M_impl);
      }

      _Deque_base(_Deque_base&& __x, const allocator_type& __a)
      : _M_impl(std::move(__x._M_impl), _Tp_alloc_type(__a))
      { __x._M_initialize_map(0); }

      _Deque_base(_Deque_base&& __x, const allocator_type& __a, size_t __n)
      : _M_impl(__a)
      {
	if (__x.get_allocator() == __a)
	  {
	    if (__x._M_impl._M_map)
	      {
		_M_initialize_map(0);
		this->_M_impl._M_swap_data(__x._M_impl);
	      }
	  }
	else
	  {
	    _M_initialize_map(__n);
	  }
      }
#endif

      ~_Deque_base() _GLIBCXX_NOEXCEPT;

      typedef typename iterator::_Map_pointer _Map_pointer;

      struct _Deque_impl_data
      {
	_Map_pointer _M_map;
	size_t _M_map_size;
	iterator _M_start;
	iterator _M_finish;

	_Deque_impl_data() _GLIBCXX_NOEXCEPT
	: _M_map(), _M_map_size(), _M_start(), _M_finish()
	{ }

#if __cplusplus >= 201103L
	_Deque_impl_data(const _Deque_impl_data&) = default;
	_Deque_impl_data&
	operator=(const _Deque_impl_data&) = default;

	_Deque_impl_data(_Deque_impl_data&& __x) noexcept
	: _Deque_impl_data(__x)
	{ __x = _Deque_impl_data(); }
#endif

	void
	_M_swap_data(_Deque_impl_data& __x) _GLIBCXX_NOEXCEPT
	{
	  // Do not use std::swap(_M_start, __x._M_start), etc as it loses
	  // information used by TBAA.
	  std::swap(*this, __x);
	}
      };

      // This struct encapsulates the implementation of the std::deque
      // standard container and at the same time makes use of the EBO
      // for empty allocators.
      struct _Deque_impl
      : public _Tp_alloc_type, public _Deque_impl_data
      {
	_Deque_impl() _GLIBCXX_NOEXCEPT_IF(
	  is_nothrow_default_constructible<_Tp_alloc_type>::value)
	: _Tp_alloc_type()
	{ }

	_Deque_impl(const _Tp_alloc_type& __a) _GLIBCXX_NOEXCEPT
	: _Tp_alloc_type(__a)
	{ }

#if __cplusplus >= 201103L
	_Deque_impl(_Deque_impl&&) = default;

	_Deque_impl(_Tp_alloc_type&& __a) noexcept
	: _Tp_alloc_type(std::move(__a))
	{ }

	_Deque_impl(_Deque_impl&& __d, _Tp_alloc_type&& __a)
	: _Tp_alloc_type(std::move(__a)), _Deque_impl_data(std::move(__d))
	{ }
#endif
      };

      _Tp_alloc_type&
      _M_get_Tp_allocator() _GLIBCXX_NOEXCEPT
      { return this->_M_impl; }

      const _Tp_alloc_type&
      _M_get_Tp_allocator() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl; }

      _Map_alloc_type
      _M_get_map_allocator() const _GLIBCXX_NOEXCEPT
      { return _Map_alloc_type(_M_get_Tp_allocator()); }

      _Ptr
      _M_allocate_node()
      {
	typedef __gnu_cxx::__alloc_traits<_Tp_alloc_type> _Traits;
	return _Traits::allocate(_M_impl, __deque_buf_size(sizeof(_Tp)));
      }

      void
      _M_deallocate_node(_Ptr __p) _GLIBCXX_NOEXCEPT
      {
	typedef __gnu_cxx::__alloc_traits<_Tp_alloc_type> _Traits;
	_Traits::deallocate(_M_impl, __p, __deque_buf_size(sizeof(_Tp)));
      }

      _Map_pointer
      _M_allocate_map(size_t __n)
      {
	_Map_alloc_type __map_alloc = _M_get_map_allocator();
	return _Map_alloc_traits::allocate(__map_alloc, __n);
      }

      void
      _M_deallocate_map(_Map_pointer __p, size_t __n) _GLIBCXX_NOEXCEPT
      {
	_Map_alloc_type __map_alloc = _M_get_map_allocator();
	_Map_alloc_traits::deallocate(__map_alloc, __p, __n);
      }

      void _M_initialize_map(size_t);
      void _M_create_nodes(_Map_pointer __nstart, _Map_pointer __nfinish);
      void _M_destroy_nodes(_Map_pointer __nstart,
			    _Map_pointer __nfinish) _GLIBCXX_NOEXCEPT;
      enum { _S_initial_map_size = 8 };

      _Deque_impl _M_impl;
    };

  template<typename _Tp, typename _Alloc>
    _Deque_base<_Tp, _Alloc>::
    ~_Deque_base() _GLIBCXX_NOEXCEPT
    {
      if (this->_M_impl._M_map)
	{
	  _M_destroy_nodes(this->_M_impl._M_start._M_node,
			   this->_M_impl._M_finish._M_node + 1);
	  _M_deallocate_map(this->_M_impl._M_map, this->_M_impl._M_map_size);
	}
    }

  /**
   *  @brief Layout storage.
   *  @param  __num_elements  The count of T's for which to allocate space
   *                          at first.
   *  @return   Nothing.
   *
   *  The initial underlying memory layout is a bit complicated...
  */
  template<typename _Tp, typename _Alloc>
    void
    _Deque_base<_Tp, _Alloc>::
    _M_initialize_map(size_t __num_elements)
    {
      const size_t __num_nodes = (__num_elements / __deque_buf_size(sizeof(_Tp))
				  + 1);

      this->_M_impl._M_map_size = std::max((size_t) _S_initial_map_size,
					   size_t(__num_nodes + 2));
      this->_M_impl._M_map = _M_allocate_map(this->_M_impl._M_map_size);

      // For "small" maps (needing less than _M_map_size nodes), allocation
      // starts in the middle elements and grows outwards.  So nstart may be
      // the beginning of _M_map, but for small maps it may be as far in as
      // _M_map+3.

      _Map_pointer __nstart = (this->_M_impl._M_map
			       + (this->_M_impl._M_map_size - __num_nodes) / 2);
      _Map_pointer __nfinish = __nstart + __num_nodes;

      __try
	{ _M_create_nodes(__nstart, __nfinish); }
      __catch(...)
	{
	  _M_deallocate_map(this->_M_impl._M_map, this->_M_impl._M_map_size);
	  this->_M_impl._M_map = _Map_pointer();
	  this->_M_impl._M_map_size = 0;
	  __throw_exception_again;
	}

      this->_M_impl._M_start._M_set_node(__nstart);
      this->_M_impl._M_finish._M_set_node(__nfinish - 1);
      this->_M_impl._M_start._M_cur = _M_impl._M_start._M_first;
      this->_M_impl._M_finish._M_cur = (this->_M_impl._M_finish._M_first
					+ __num_elements
					% __deque_buf_size(sizeof(_Tp)));
    }

  template<typename _Tp, typename _Alloc>
    void
    _Deque_base<_Tp, _Alloc>::
    _M_create_nodes(_Map_pointer __nstart, _Map_pointer __nfinish)
    {
      _Map_pointer __cur;
      __try
	{
	  for (__cur = __nstart; __cur < __nfinish; ++__cur)
	    *__cur = this->_M_allocate_node();
	}
      __catch(...)
	{
	  _M_destroy_nodes(__nstart, __cur);
	  __throw_exception_again;
	}
    }

  template<typename _Tp, typename _Alloc>
    void
    _Deque_base<_Tp, _Alloc>::
    _M_destroy_nodes(_Map_pointer __nstart,
		     _Map_pointer __nfinish) _GLIBCXX_NOEXCEPT
    {
      for (_Map_pointer __n = __nstart; __n < __nfinish; ++__n)
	_M_deallocate_node(*__n);
    }

  /**
   *  @brief  A standard container using fixed-size memory allocation and
   *  constant-time manipulation of elements at either end.
   *
   *  @ingroup sequences
   *
   *  @tparam _Tp  Type of element.
   *  @tparam _Alloc  Allocator type, defaults to allocator<_Tp>.
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, a
   *  <a href="tables.html#66">reversible container</a>, and a
   *  <a href="tables.html#67">sequence</a>, including the
   *  <a href="tables.html#68">optional sequence requirements</a>.
   *
   *  In previous HP/SGI versions of deque, there was an extra template
   *  parameter so users could control the node size.  This extension turned
   *  out to violate the C++ standard (it can be detected using template
   *  template parameters), and it was removed.
   *
   *  Here's how a deque<Tp> manages memory.  Each deque has 4 members:
   *
   *  - Tp**        _M_map
   *  - size_t      _M_map_size
   *  - iterator    _M_start, _M_finish
   *
   *  map_size is at least 8.  %map is an array of map_size
   *  pointers-to-@a nodes.  (The name %map has nothing to do with the
   *  std::map class, and @b nodes should not be confused with
   *  std::list's usage of @a node.)
   *
   *  A @a node has no specific type name as such, but it is referred
   *  to as @a node in this file.  It is a simple array-of-Tp.  If Tp
   *  is very large, there will be one Tp element per node (i.e., an
   *  @a array of one).  For non-huge Tp's, node size is inversely
   *  related to Tp size: the larger the Tp, the fewer Tp's will fit
   *  in a node.  The goal here is to keep the total size of a node
   *  relatively small and constant over different Tp's, to improve
   *  allocator efficiency.
   *
   *  Not every pointer in the %map array will point to a node.  If
   *  the initial number of elements in the deque is small, the
   *  /middle/ %map pointers will be valid, and the ones at the edges
   *  will be unused.  This same situation will arise as the %map
   *  grows: available %map pointers, if any, will be on the ends.  As
   *  new nodes are created, only a subset of the %map's pointers need
   *  to be copied @a outward.
   *
   *  Class invariants:
   * - For any nonsingular iterator i:
   *    - i.node points to a member of the %map array.  (Yes, you read that
   *      correctly:  i.node does not actually point to a node.)  The member of
   *      the %map array is what actually points to the node.
   *    - i.first == *(i.node)    (This points to the node (first Tp element).)
   *    - i.last  == i.first + node_size
   *    - i.cur is a pointer in the range [i.first, i.last).  NOTE:
   *      the implication of this is that i.cur is always a dereferenceable
   *      pointer, even if i is a past-the-end iterator.
   * - Start and Finish are always nonsingular iterators.  NOTE: this
   * means that an empty deque must have one node, a deque with <N
   * elements (where N is the node buffer size) must have one node, a
   * deque with N through (2N-1) elements must have two nodes, etc.
   * - For every node other than start.node and finish.node, every
   * element in the node is an initialized object.  If start.node ==
   * finish.node, then [start.cur, finish.cur) are initialized
   * objects, and the elements outside that range are uninitialized
   * storage.  Otherwise, [start.cur, start.last) and [finish.first,
   * finish.cur) are initialized objects, and [start.first, start.cur)
   * and [finish.cur, finish.last) are uninitialized storage.
   * - [%map, %map + map_size) is a valid, non-empty range.
   * - [start.node, finish.node] is a valid range contained within
   *   [%map, %map + map_size).
   * - A pointer in the range [%map, %map + map_size) points to an allocated
   *   node if and only if the pointer is in the range
   *   [start.node, finish.node].
   *
   *  Here's the magic:  nothing in deque is @b aware of the discontiguous
   *  storage!
   *
   *  The memory setup and layout occurs in the parent, _Base, and the iterator
   *  class is entirely responsible for @a leaping from one node to the next.
   *  All the implementation routines for deque itself work only through the
   *  start and finish iterators.  This keeps the routines simple and sane,
   *  and we can use other standard algorithms as well.
  */
  template<typename _Tp, typename _Alloc = std::allocator<_Tp> >
    class deque : protected _Deque_base<_Tp, _Alloc>
    {
#ifdef _GLIBCXX_CONCEPT_CHECKS
      // concept requirements
      typedef typename _Alloc::value_type	_Alloc_value_type;
# if __cplusplus < 201103L
      __glibcxx_class_requires(_Tp, _SGIAssignableConcept)
# endif
      __glibcxx_class_requires2(_Tp, _Alloc_value_type, _SameTypeConcept)
#endif

#if __cplusplus >= 201103L
      static_assert(is_same<typename remove_cv<_Tp>::type, _Tp>::value,
	  "std::deque must have a non-const, non-volatile value_type");
# if __cplusplus > 201703L || defined __STRICT_ANSI__
      static_assert(is_same<typename _Alloc::value_type, _Tp>::value,
	  "std::deque must have the same value_type as its allocator");
# endif
#endif

      typedef _Deque_base<_Tp, _Alloc>			_Base;
      typedef typename _Base::_Tp_alloc_type		_Tp_alloc_type;
      typedef typename _Base::_Alloc_traits		_Alloc_traits;
      typedef typename _Base::_Map_pointer		_Map_pointer;

    public:
      typedef _Tp					value_type;
      typedef typename _Alloc_traits::pointer		pointer;
      typedef typename _Alloc_traits::const_pointer	const_pointer;
      typedef typename _Alloc_traits::reference		reference;
      typedef typename _Alloc_traits::const_reference	const_reference;
      typedef typename _Base::iterator			iterator;
      typedef typename _Base::const_iterator		const_iterator;
      typedef std::reverse_iterator<const_iterator>	const_reverse_iterator;
      typedef std::reverse_iterator<iterator>		reverse_iterator;
      typedef size_t					size_type;
      typedef ptrdiff_t					difference_type;
      typedef _Alloc					allocator_type;

    private:
      static size_t _S_buffer_size() _GLIBCXX_NOEXCEPT
      { return __deque_buf_size(sizeof(_Tp)); }

      // Functions controlling memory layout, and nothing else.
      using _Base::_M_initialize_map;
      using _Base::_M_create_nodes;
      using _Base::_M_destroy_nodes;
      using _Base::_M_allocate_node;
      using _Base::_M_deallocate_node;
      using _Base::_M_allocate_map;
      using _Base::_M_deallocate_map;
      using _Base::_M_get_Tp_allocator;

      /**
       *  A total of four data members accumulated down the hierarchy.
       *  May be accessed via _M_impl.*
       */
      using _Base::_M_impl;

    public:
      // [23.2.1.1] construct/copy/destroy
      // (assign() and get_allocator() are also listed in this section)

      /**
       *  @brief  Creates a %deque with no elements.
       */
#if __cplusplus >= 201103L
      deque() = default;
#else
      deque() { }
#endif

      /**
       *  @brief  Creates a %deque with no elements.
       *  @param  __a  An allocator object.
       */
      explicit
      deque(const allocator_type& __a)
      : _Base(__a, 0) { }

#if __cplusplus >= 201103L
      /**
       *  @brief  Creates a %deque with default constructed elements.
       *  @param  __n  The number of elements to initially create.
       *  @param  __a  An allocator.
       *
       *  This constructor fills the %deque with @a n default
       *  constructed elements.
       */
      explicit
      deque(size_type __n, const allocator_type& __a = allocator_type())
      : _Base(__a, _S_check_init_len(__n, __a))
      { _M_default_initialize(); }

      /**
       *  @brief  Creates a %deque with copies of an exemplar element.
       *  @param  __n  The number of elements to initially create.
       *  @param  __value  An element to copy.
       *  @param  __a  An allocator.
       *
       *  This constructor fills the %deque with @a __n copies of @a __value.
       */
      deque(size_type __n, const value_type& __value,
	    const allocator_type& __a = allocator_type())
      : _Base(__a, _S_check_init_len(__n, __a))
      { _M_fill_initialize(__value); }
#else
      /**
       *  @brief  Creates a %deque with copies of an exemplar element.
       *  @param  __n  The number of elements to initially create.
       *  @param  __value  An element to copy.
       *  @param  __a  An allocator.
       *
       *  This constructor fills the %deque with @a __n copies of @a __value.
       */
      explicit
      deque(size_type __n, const value_type& __value = value_type(),
	    const allocator_type& __a = allocator_type())
      : _Base(__a, _S_check_init_len(__n, __a))
      { _M_fill_initialize(__value); }
#endif

      /**
       *  @brief  %Deque copy constructor.
       *  @param  __x  A %deque of identical element and allocator types.
       *
       *  The newly-created %deque uses a copy of the allocator object used
       *  by @a __x (unless the allocator traits dictate a different object).
       */
      deque(const deque& __x)
      : _Base(_Alloc_traits::_S_select_on_copy(__x._M_get_Tp_allocator()),
	      __x.size())
      { std::__uninitialized_copy_a(__x.begin(), __x.end(),
				    this->_M_impl._M_start,
				    _M_get_Tp_allocator()); }

#if __cplusplus >= 201103L
      /**
       *  @brief  %Deque move constructor.
       *
       *  The newly-created %deque contains the exact contents of the
       *  moved instance.
       *  The contents of the moved instance are a valid, but unspecified
       *  %deque.
       */
      deque(deque&&) = default;

      /// Copy constructor with alternative allocator
      deque(const deque& __x, const allocator_type& __a)
      : _Base(__a, __x.size())
      { std::__uninitialized_copy_a(__x.begin(), __x.end(),
				    this->_M_impl._M_start,
				    _M_get_Tp_allocator()); }

      /// Move constructor with alternative allocator
      deque(deque&& __x, const allocator_type& __a)
      : deque(std::move(__x), __a, typename _Alloc_traits::is_always_equal{})
      { }

    private:
      deque(deque&& __x, const allocator_type& __a, true_type)
      : _Base(std::move(__x), __a)
      { }

      deque(deque&& __x, const allocator_type& __a, false_type)
      : _Base(std::move(__x), __a, __x.size())
      {
	if (__x.get_allocator() != __a && !__x.empty())
	  {
	    std::__uninitialized_move_a(__x.begin(), __x.end(),
					this->_M_impl._M_start,
					_M_get_Tp_allocator());
	    __x.clear();
	  }
      }

    public:
      /**
       *  @brief  Builds a %deque from an initializer list.
       *  @param  __l  An initializer_list.
       *  @param  __a  An allocator object.
       *
       *  Create a %deque consisting of copies of the elements in the
       *  initializer_list @a __l.
       *
       *  This will call the element type's copy constructor N times
       *  (where N is __l.size()) and do no memory reallocation.
       */
      deque(initializer_list<value_type> __l,
	    const allocator_type& __a = allocator_type())
      : _Base(__a)
      {
	_M_range_initialize(__l.begin(), __l.end(),
			    random_access_iterator_tag());
      }
#endif

      /**
       *  @brief  Builds a %deque from a range.
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *  @param  __a  An allocator object.
       *
       *  Create a %deque consisting of copies of the elements from [__first,
       *  __last).
       *
       *  If the iterators are forward, bidirectional, or random-access, then
       *  this will call the elements' copy constructor N times (where N is
       *  distance(__first,__last)) and do no memory reallocation.  But if only
       *  input iterators are used, then this will do at most 2N calls to the
       *  copy constructor, and logN memory reallocations.
       */
#if __cplusplus >= 201103L
      template<typename _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
	deque(_InputIterator __first, _InputIterator __last,
	      const allocator_type& __a = allocator_type())
	: _Base(__a)
	{
	  _M_range_initialize(__first, __last,
			      std::__iterator_category(__first));
	}
#else
      template<typename _InputIterator>
	deque(_InputIterator __first, _InputIterator __last,
	      const allocator_type& __a = allocator_type())
	: _Base(__a)
	{
	  // Check whether it's an integral type.  If so, it's not an iterator.
	  typedef typename std::__is_integer<_InputIterator>::__type _Integral;
	  _M_initialize_dispatch(__first, __last, _Integral());
	}
#endif

      /**
       *  The dtor only erases the elements, and note that if the elements
       *  themselves are pointers, the pointed-to memory is not touched in any
       *  way.  Managing the pointer is the user's responsibility.
       */
      ~deque()
      { _M_destroy_data(begin(), end(), _M_get_Tp_allocator()); }

      /**
       *  @brief  %Deque assignment operator.
       *  @param  __x  A %deque of identical element and allocator types.
       *
       *  All the elements of @a x are copied.
       *
       *  The newly-created %deque uses a copy of the allocator object used
       *  by @a __x (unless the allocator traits dictate a different object).
       */
      deque&
      operator=(const deque& __x);

#if __cplusplus >= 201103L
      /**
       *  @brief  %Deque move assignment operator.
       *  @param  __x  A %deque of identical element and allocator types.
       *
       *  The contents of @a __x are moved into this deque (without copying,
       *  if the allocators permit it).
       *  @a __x is a valid, but unspecified %deque.
       */
      deque&
      operator=(deque&& __x) noexcept(_Alloc_traits::_S_always_equal())
      {
	using __always_equal = typename _Alloc_traits::is_always_equal;
	_M_move_assign1(std::move(__x), __always_equal{});
	return *this;
      }

      /**
       *  @brief  Assigns an initializer list to a %deque.
       *  @param  __l  An initializer_list.
       *
       *  This function fills a %deque with copies of the elements in the
       *  initializer_list @a __l.
       *
       *  Note that the assignment completely changes the %deque and that the
       *  resulting %deque's size is the same as the number of elements
       *  assigned.
       */
      deque&
      operator=(initializer_list<value_type> __l)
      {
	_M_assign_aux(__l.begin(), __l.end(),
		      random_access_iterator_tag());
	return *this;
      }
#endif

      /**
       *  @brief  Assigns a given value to a %deque.
       *  @param  __n  Number of elements to be assigned.
       *  @param  __val  Value to be assigned.
       *
       *  This function fills a %deque with @a n copies of the given
       *  value.  Note that the assignment completely changes the
       *  %deque and that the resulting %deque's size is the same as
       *  the number of elements assigned.
       */
      void
      assign(size_type __n, const value_type& __val)
      { _M_fill_assign(__n, __val); }

      /**
       *  @brief  Assigns a range to a %deque.
       *  @param  __first  An input iterator.
       *  @param  __last   An input iterator.
       *
       *  This function fills a %deque with copies of the elements in the
       *  range [__first,__last).
       *
       *  Note that the assignment completely changes the %deque and that the
       *  resulting %deque's size is the same as the number of elements
       *  assigned.
       */
#if __cplusplus >= 201103L
      template<typename _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
	void
	assign(_InputIterator __first, _InputIterator __last)
	{ _M_assign_aux(__first, __last, std::__iterator_category(__first)); }
#else
      template<typename _InputIterator>
	void
	assign(_InputIterator __first, _InputIterator __last)
	{
	  typedef typename std::__is_integer<_InputIterator>::__type _Integral;
	  _M_assign_dispatch(__first, __last, _Integral());
	}
#endif

#if __cplusplus >= 201103L
      /**
       *  @brief  Assigns an initializer list to a %deque.
       *  @param  __l  An initializer_list.
       *
       *  This function fills a %deque with copies of the elements in the
       *  initializer_list @a __l.
       *
       *  Note that the assignment completely changes the %deque and that the
       *  resulting %deque's size is the same as the number of elements
       *  assigned.
       */
      void
      assign(initializer_list<value_type> __l)
      { _M_assign_aux(__l.begin(), __l.end(), random_access_iterator_tag()); }
#endif

      /// Get a copy of the memory allocation object.
      allocator_type
      get_allocator() const _GLIBCXX_NOEXCEPT
      { return _Base::get_allocator(); }

      // iterators
      /**
       *  Returns a read/write iterator that points to the first element in the
       *  %deque.  Iteration is done in ordinary element order.
       */
      iterator
      begin() _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_start; }

      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %deque.  Iteration is done in ordinary element order.
       */
      const_iterator
      begin() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_start; }

      /**
       *  Returns a read/write iterator that points one past the last
       *  element in the %deque.  Iteration is done in ordinary
       *  element order.
       */
      iterator
      end() _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_finish; }

      /**
       *  Returns a read-only (constant) iterator that points one past
       *  the last element in the %deque.  Iteration is done in
       *  ordinary element order.
       */
      const_iterator
      end() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_finish; }

      /**
       *  Returns a read/write reverse iterator that points to the
       *  last element in the %deque.  Iteration is done in reverse
       *  element order.
       */
      reverse_iterator
      rbegin() _GLIBCXX_NOEXCEPT
      { return reverse_iterator(this->_M_impl._M_finish); }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to the last element in the %deque.  Iteration is done in
       *  reverse element order.
       */
      const_reverse_iterator
      rbegin() const _GLIBCXX_NOEXCEPT
      { return const_reverse_iterator(this->_M_impl._M_finish); }

      /**
       *  Returns a read/write reverse iterator that points to one
       *  before the first element in the %deque.  Iteration is done
       *  in reverse element order.
       */
      reverse_iterator
      rend() _GLIBCXX_NOEXCEPT
      { return reverse_iterator(this->_M_impl._M_start); }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to one before the first element in the %deque.  Iteration is
       *  done in reverse element order.
       */
      const_reverse_iterator
      rend() const _GLIBCXX_NOEXCEPT
      { return const_reverse_iterator(this->_M_impl._M_start); }

#if __cplusplus >= 201103L
      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %deque.  Iteration is done in ordinary element order.
       */
      const_iterator
      cbegin() const noexcept
      { return this->_M_impl._M_start; }

      /**
       *  Returns a read-only (constant) iterator that points one past
       *  the last element in the %deque.  Iteration is done in
       *  ordinary element order.
       */
      const_iterator
      cend() const noexcept
      { return this->_M_impl._M_finish; }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to the last element in the %deque.  Iteration is done in
       *  reverse element order.
       */
      const_reverse_iterator
      crbegin() const noexcept
      { return const_reverse_iterator(this->_M_impl._M_finish); }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to one before the first element in the %deque.  Iteration is
       *  done in reverse element order.
       */
      const_reverse_iterator
      crend() const noexcept
      { return const_reverse_iterator(this->_M_impl._M_start); }
#endif

      // [23.2.1.2] capacity
      /**  Returns the number of elements in the %deque.  */
      size_type
      size() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_finish - this->_M_impl._M_start; }

      /**  Returns the size() of the largest possible %deque.  */
      size_type
      max_size() const _GLIBCXX_NOEXCEPT
      { return _S_max_size(_M_get_Tp_allocator()); }

#if __cplusplus >= 201103L
      /**
       *  @brief  Resizes the %deque to the specified number of elements.
       *  @param  __new_size  Number of elements the %deque should contain.
       *
       *  This function will %resize the %deque to the specified
       *  number of elements.  If the number is smaller than the
       *  %deque's current size the %deque is truncated, otherwise
       *  default constructed elements are appended.
       */
      void
      resize(size_type __new_size)
      {
	const size_type __len = size();
	if (__new_size > __len)
	  _M_default_append(__new_size - __len);
	else if (__new_size < __len)
	  _M_erase_at_end(this->_M_impl._M_start
			  + difference_type(__new_size));
      }

      /**
       *  @brief  Resizes the %deque to the specified number of elements.
       *  @param  __new_size  Number of elements the %deque should contain.
       *  @param  __x  Data with which new elements should be populated.
       *
       *  This function will %resize the %deque to the specified
       *  number of elements.  If the number is smaller than the
       *  %deque's current size the %deque is truncated, otherwise the
       *  %deque is extended and new elements are populated with given
       *  data.
       */
      void
      resize(size_type __new_size, const value_type& __x)
#else
      /**
       *  @brief  Resizes the %deque to the specified number of elements.
       *  @param  __new_size  Number of elements the %deque should contain.
       *  @param  __x  Data with which new elements should be populated.
       *
       *  This function will %resize the %deque to the specified
       *  number of elements.  If the number is smaller than the
       *  %deque's current size the %deque is truncated, otherwise the
       *  %deque is extended and new elements are populated with given
       *  data.
       */
      void
      resize(size_type __new_size, value_type __x = value_type())
#endif
      {
	const size_type __len = size();
	if (__new_size > __len)
	  _M_fill_insert(this->_M_impl._M_finish, __new_size - __len, __x);
	else if (__new_size < __len)
	  _M_erase_at_end(this->_M_impl._M_start
			  + difference_type(__new_size));
      }

#if __cplusplus >= 201103L
      /**  A non-binding request to reduce memory use.  */
      void
      shrink_to_fit() noexcept
      { _M_shrink_to_fit(); }
#endif

      /**
       *  Returns true if the %deque is empty.  (Thus begin() would
       *  equal end().)
       */
      _GLIBCXX_NODISCARD bool
      empty() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_finish == this->_M_impl._M_start; }

      // element access
      /**
       *  @brief Subscript access to the data contained in the %deque.
       *  @param __n The index of the element for which data should be
       *  accessed.
       *  @return  Read/write reference to data.
       *
       *  This operator allows for easy, array-style, data access.
       *  Note that data access with this operator is unchecked and
       *  out_of_range lookups are not defined. (For checked lookups
       *  see at().)
       */
      reference
      operator[](size_type __n) _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_subscript(__n);
	return this->_M_impl._M_start[difference_type(__n)];
      }

      /**
       *  @brief Subscript access to the data contained in the %deque.
       *  @param __n The index of the element for which data should be
       *  accessed.
       *  @return  Read-only (constant) reference to data.
       *
       *  This operator allows for easy, array-style, data access.
       *  Note that data access with this operator is unchecked and
       *  out_of_range lookups are not defined. (For checked lookups
       *  see at().)
       */
      const_reference
      operator[](size_type __n) const _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_subscript(__n);
	return this->_M_impl._M_start[difference_type(__n)];
      }

    protected:
      /// Safety check used only from at().
      void
      _M_range_check(size_type __n) const
      {
	if (__n >= this->size())
	  __throw_out_of_range_fmt(__N("deque::_M_range_check: __n "
				       "(which is %zu)>= this->size() "
				       "(which is %zu)"),
				   __n, this->size());
      }

    public:
      /**
       *  @brief  Provides access to the data contained in the %deque.
       *  @param __n The index of the element for which data should be
       *  accessed.
       *  @return  Read/write reference to data.
       *  @throw  std::out_of_range  If @a __n is an invalid index.
       *
       *  This function provides for safer data access.  The parameter
       *  is first checked that it is in the range of the deque.  The
       *  function throws out_of_range if the check fails.
       */
      reference
      at(size_type __n)
      {
	_M_range_check(__n);
	return (*this)[__n];
      }

      /**
       *  @brief  Provides access to the data contained in the %deque.
       *  @param __n The index of the element for which data should be
       *  accessed.
       *  @return  Read-only (constant) reference to data.
       *  @throw  std::out_of_range  If @a __n is an invalid index.
       *
       *  This function provides for safer data access.  The parameter is first
       *  checked that it is in the range of the deque.  The function throws
       *  out_of_range if the check fails.
       */
      const_reference
      at(size_type __n) const
      {
	_M_range_check(__n);
	return (*this)[__n];
      }

      /**
       *  Returns a read/write reference to the data at the first
       *  element of the %deque.
       */
      reference
      front() _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_nonempty();
	return *begin();
      }

      /**
       *  Returns a read-only (constant) reference to the data at the first
       *  element of the %deque.
       */
      const_reference
      front() const _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_nonempty();
	return *begin();
      }

      /**
       *  Returns a read/write reference to the data at the last element of the
       *  %deque.
       */
      reference
      back() _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_nonempty();
	iterator __tmp = end();
	--__tmp;
	return *__tmp;
      }

      /**
       *  Returns a read-only (constant) reference to the data at the last
       *  element of the %deque.
       */
      const_reference
      back() const _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_nonempty();
	const_iterator __tmp = end();
	--__tmp;
	return *__tmp;
      }

      // [23.2.1.2] modifiers
      /**
       *  @brief  Add data to the front of the %deque.
       *  @param  __x  Data to be added.
       *
       *  This is a typical stack operation.  The function creates an
       *  element at the front of the %deque and assigns the given
       *  data to it.  Due to the nature of a %deque this operation
       *  can be done in constant time.
       */
      void
      push_front(const value_type& __x)
      {
	if (this->_M_impl._M_start._M_cur != this->_M_impl._M_start._M_first)
	  {
	    _Alloc_traits::construct(this->_M_impl,
				     this->_M_impl._M_start._M_cur - 1,
				     __x);
	    --this->_M_impl._M_start._M_cur;
	  }
	else
	  _M_push_front_aux(__x);
      }

#if __cplusplus >= 201103L
      void
      push_front(value_type&& __x)
      { emplace_front(std::move(__x)); }

      template<typename... _Args>
#if __cplusplus > 201402L
	reference
#else
	void
#endif
	emplace_front(_Args&&... __args);
#endif

      /**
       *  @brief  Add data to the end of the %deque.
       *  @param  __x  Data to be added.
       *
       *  This is a typical stack operation.  The function creates an
       *  element at the end of the %deque and assigns the given data
       *  to it.  Due to the nature of a %deque this operation can be
       *  done in constant time.
       */
      void
      push_back(const value_type& __x)
      {
	if (this->_M_impl._M_finish._M_cur
	    != this->_M_impl._M_finish._M_last - 1)
	  {
	    _Alloc_traits::construct(this->_M_impl,
				     this->_M_impl._M_finish._M_cur, __x);
	    ++this->_M_impl._M_finish._M_cur;
	  }
	else
	  _M_push_back_aux(__x);
      }

#if __cplusplus >= 201103L
      void
      push_back(value_type&& __x)
      { emplace_back(std::move(__x)); }

      template<typename... _Args>
#if __cplusplus > 201402L
	reference
#else
	void
#endif
	emplace_back(_Args&&... __args);
#endif

      /**
       *  @brief  Removes first element.
       *
       *  This is a typical stack operation.  It shrinks the %deque by one.
       *
       *  Note that no data is returned, and if the first element's data is
       *  needed, it should be retrieved before pop_front() is called.
       */
      void
      pop_front() _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_nonempty();
	if (this->_M_impl._M_start._M_cur
	    != this->_M_impl._M_start._M_last - 1)
	  {
	    _Alloc_traits::destroy(_M_get_Tp_allocator(),
				   this->_M_impl._M_start._M_cur);
	    ++this->_M_impl._M_start._M_cur;
	  }
	else
	  _M_pop_front_aux();
      }

      /**
       *  @brief  Removes last element.
       *
       *  This is a typical stack operation.  It shrinks the %deque by one.
       *
       *  Note that no data is returned, and if the last element's data is
       *  needed, it should be retrieved before pop_back() is called.
       */
      void
      pop_back() _GLIBCXX_NOEXCEPT
      {
	__glibcxx_requires_nonempty();
	if (this->_M_impl._M_finish._M_cur
	    != this->_M_impl._M_finish._M_first)
	  {
	    --this->_M_impl._M_finish._M_cur;
	    _Alloc_traits::destroy(_M_get_Tp_allocator(),
				   this->_M_impl._M_finish._M_cur);
	  }
	else
	  _M_pop_back_aux();
      }

#if __cplusplus >= 201103L
      /**
       *  @brief  Inserts an object in %deque before specified iterator.
       *  @param  __position  A const_iterator into the %deque.
       *  @param  __args  Arguments.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert an object of type T constructed
       *  with T(std::forward<Args>(args)...) before the specified location.
       */
      template<typename... _Args>
	iterator
	emplace(const_iterator __position, _Args&&... __args);

      /**
       *  @brief  Inserts given value into %deque before specified iterator.
       *  @param  __position  A const_iterator into the %deque.
       *  @param  __x  Data to be inserted.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert a copy of the given value before the
       *  specified location.
       */
      iterator
      insert(const_iterator __position, const value_type& __x);
#else
      /**
       *  @brief  Inserts given value into %deque before specified iterator.
       *  @param  __position  An iterator into the %deque.
       *  @param  __x  Data to be inserted.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert a copy of the given value before the
       *  specified location.
       */
      iterator
      insert(iterator __position, const value_type& __x);
#endif

#if __cplusplus >= 201103L
      /**
       *  @brief  Inserts given rvalue into %deque before specified iterator.
       *  @param  __position  A const_iterator into the %deque.
       *  @param  __x  Data to be inserted.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert a copy of the given rvalue before the
       *  specified location.
       */
      iterator
      insert(const_iterator __position, value_type&& __x)
      { return emplace(__position, std::move(__x)); }

      /**
       *  @brief  Inserts an initializer list into the %deque.
       *  @param  __p  An iterator into the %deque.
       *  @param  __l  An initializer_list.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert copies of the data in the
       *  initializer_list @a __l into the %deque before the location
       *  specified by @a __p.  This is known as <em>list insert</em>.
       */
      iterator
      insert(const_iterator __p, initializer_list<value_type> __l)
      {
	auto __offset = __p - cbegin();
	_M_range_insert_aux(__p._M_const_cast(), __l.begin(), __l.end(),
			    std::random_access_iterator_tag());
	return begin() + __offset;
      }

      /**
       *  @brief  Inserts a number of copies of given data into the %deque.
       *  @param  __position  A const_iterator into the %deque.
       *  @param  __n  Number of elements to be inserted.
       *  @param  __x  Data to be inserted.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert a specified number of copies of the given
       *  data before the location specified by @a __position.
       */
      iterator
      insert(const_iterator __position, size_type __n, const value_type& __x)
      {
	difference_type __offset = __position - cbegin();
	_M_fill_insert(__position._M_const_cast(), __n, __x);
	return begin() + __offset;
      }
#else
      /**
       *  @brief  Inserts a number of copies of given data into the %deque.
       *  @param  __position  An iterator into the %deque.
       *  @param  __n  Number of elements to be inserted.
       *  @param  __x  Data to be inserted.
       *
       *  This function will insert a specified number of copies of the given
       *  data before the location specified by @a __position.
       */
      void
      insert(iterator __position, size_type __n, const value_type& __x)
      { _M_fill_insert(__position, __n, __x); }
#endif

#if __cplusplus >= 201103L
      /**
       *  @brief  Inserts a range into the %deque.
       *  @param  __position  A const_iterator into the %deque.
       *  @param  __first  An input iterator.
       *  @param  __last   An input iterator.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert copies of the data in the range
       *  [__first,__last) into the %deque before the location specified
       *  by @a __position.  This is known as <em>range insert</em>.
       */
      template<typename _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
	iterator
	insert(const_iterator __position, _InputIterator __first,
	       _InputIterator __last)
	{
	  difference_type __offset = __position - cbegin();
	  _M_range_insert_aux(__position._M_const_cast(), __first, __last,
			      std::__iterator_category(__first));
	  return begin() + __offset;
	}
#else
      /**
       *  @brief  Inserts a range into the %deque.
       *  @param  __position  An iterator into the %deque.
       *  @param  __first  An input iterator.
       *  @param  __last   An input iterator.
       *
       *  This function will insert copies of the data in the range
       *  [__first,__last) into the %deque before the location specified
       *  by @a __position.  This is known as <em>range insert</em>.
       */
      template<typename _InputIterator>
	void
	insert(iterator __position, _InputIterator __first,
	       _InputIterator __last)
	{
	  // Check whether it's an integral type.  If so, it's not an iterator.
	  typedef typename std::__is_integer<_InputIterator>::__type _Integral;
	  _M_insert_dispatch(__position, __first, __last, _Integral());
	}
#endif

      /**
       *  @brief  Remove element at given position.
       *  @param  __position  Iterator pointing to element to be erased.
       *  @return  An iterator pointing to the next element (or end()).
       *
       *  This function will erase the element at the given position and thus
       *  shorten the %deque by one.
       *
       *  The user is cautioned that
       *  this function only erases the element, and that if the element is
       *  itself a pointer, the pointed-to memory is not touched in any way.
       *  Managing the pointer is the user's responsibility.
       */
      iterator
#if __cplusplus >= 201103L
      erase(const_iterator __position)
#else
      erase(iterator __position)
#endif
      { return _M_erase(__position._M_const_cast()); }

      /**
       *  @brief  Remove a range of elements.
       *  @param  __first  Iterator pointing to the first element to be erased.
       *  @param  __last  Iterator pointing to one past the last element to be
       *                erased.
       *  @return  An iterator pointing to the element pointed to by @a last
       *           prior to erasing (or end()).
       *
       *  This function will erase the elements in the range
       *  [__first,__last) and shorten the %deque accordingly.
       *
       *  The user is cautioned that
       *  this function only erases the elements, and that if the elements
       *  themselves are pointers, the pointed-to memory is not touched in any
       *  way.  Managing the pointer is the user's responsibility.
       */
      iterator
#if __cplusplus >= 201103L
      erase(const_iterator __first, const_iterator __last)
#else
      erase(iterator __first, iterator __last)
#endif
      { return _M_erase(__first._M_const_cast(), __last._M_const_cast()); }

      /**
       *  @brief  Swaps data with another %deque.
       *  @param  __x  A %deque of the same element and allocator types.
       *
       *  This exchanges the elements between two deques in constant time.
       *  (Four pointers, so it should be quite fast.)
       *  Note that the global std::swap() function is specialized such that
       *  std::swap(d1,d2) will feed to this function.
       *
       *  Whether the allocators are swapped depends on the allocator traits.
       */
      void
      swap(deque& __x) _GLIBCXX_NOEXCEPT
      {
#if __cplusplus >= 201103L
	__glibcxx_assert(_Alloc_traits::propagate_on_container_swap::value
			 || _M_get_Tp_allocator() == __x._M_get_Tp_allocator());
#endif
	_M_impl._M_swap_data(__x._M_impl);
	_Alloc_traits::_S_on_swap(_M_get_Tp_allocator(),
				  __x._M_get_Tp_allocator());
      }

      /**
       *  Erases all the elements.  Note that this function only erases the
       *  elements, and that if the elements themselves are pointers, the
       *  pointed-to memory is not touched in any way.  Managing the pointer is
       *  the user's responsibility.
       */
      void
      clear() _GLIBCXX_NOEXCEPT
      { _M_erase_at_end(begin()); }

    protected:
      // Internal constructor functions follow.

#if __cplusplus < 201103L
      // called by the range constructor to implement [23.1.1]/9

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 438. Ambiguity in the "do the right thing" clause
      template<typename _Integer>
	void
	_M_initialize_dispatch(_Integer __n, _Integer __x, __true_type)
	{
	  _M_initialize_map(_S_check_init_len(static_cast<size_type>(__n),
					      _M_get_Tp_allocator()));
	  _M_fill_initialize(__x);
	}

      // called by the range constructor to implement [23.1.1]/9
      template<typename _InputIterator>
	void
	_M_initialize_dispatch(_InputIterator __first, _InputIterator __last,
			       __false_type)
	{
	  _M_range_initialize(__first, __last,
			      std::__iterator_category(__first));
	}
#endif

      static size_t
      _S_check_init_len(size_t __n, const allocator_type& __a)
      {
	if (__n > _S_max_size(__a))
	  __throw_length_error(
	      __N("cannot create std::deque larger than max_size()"));
	return __n;
      }

      static size_type
      _S_max_size(const _Tp_alloc_type& __a) _GLIBCXX_NOEXCEPT
      {
	const size_t __diffmax = __gnu_cxx::__numeric_traits<ptrdiff_t>::__max;
	const size_t __allocmax = _Alloc_traits::max_size(__a);
	return (std::min)(__diffmax, __allocmax);
      }

      // called by the second initialize_dispatch above
      //@{
      /**
       *  @brief Fills the deque with whatever is in [first,last).
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *  @return   Nothing.
       *
       *  If the iterators are actually forward iterators (or better), then the
       *  memory layout can be done all at once.  Else we move forward using
       *  push_back on each value from the iterator.
       */
      template<typename _InputIterator>
	void
	_M_range_initialize(_InputIterator __first, _InputIterator __last,
			    std::input_iterator_tag);

      // called by the second initialize_dispatch above
      template<typename _ForwardIterator>
	void
	_M_range_initialize(_ForwardIterator __first, _ForwardIterator __last,
			    std::forward_iterator_tag);
      //@}

      /**
       *  @brief Fills the %deque with copies of value.
       *  @param  __value  Initial value.
       *  @return   Nothing.
       *  @pre _M_start and _M_finish have already been initialized,
       *  but none of the %deque's elements have yet been constructed.
       *
       *  This function is called only when the user provides an explicit size
       *  (with or without an explicit exemplar value).
       */
      void
      _M_fill_initialize(const value_type& __value);

#if __cplusplus >= 201103L
      // called by deque(n).
      void
      _M_default_initialize();
#endif

      // Internal assign functions follow.  The *_aux functions do the actual
      // assignment work for the range versions.

#if __cplusplus < 201103L
      // called by the range assign to implement [23.1.1]/9

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 438. Ambiguity in the "do the right thing" clause
      template<typename _Integer>
	void
	_M_assign_dispatch(_Integer __n, _Integer __val, __true_type)
	{ _M_fill_assign(__n, __val); }

      // called by the range assign to implement [23.1.1]/9
      template<typename _InputIterator>
	void
	_M_assign_dispatch(_InputIterator __first, _InputIterator __last,
			   __false_type)
	{ _M_assign_aux(__first, __last, std::__iterator_category(__first)); }
#endif

      // called by the second assign_dispatch above
      template<typename _InputIterator>
	void
	_M_assign_aux(_InputIterator __first, _InputIterator __last,
		      std::input_iterator_tag);

      // called by the second assign_dispatch above
      template<typename _ForwardIterator>
	void
	_M_assign_aux(_ForwardIterator __first, _ForwardIterator __last,
		      std::forward_iterator_tag)
	{
	  const size_type __len = std::distance(__first, __last);
	  if (__len > size())
	    {
	      _ForwardIterator __mid = __first;
	      std::advance(__mid, size());
	      std::copy(__first, __mid, begin());
	      _M_range_insert_aux(end(), __mid, __last,
				  std::__iterator_category(__first));
	    }
	  else
	    _M_erase_at_end(std::copy(__first, __last, begin()));
	}

      // Called by assign(n,t), and the range assign when it turns out
      // to be the same thing.
      void
      _M_fill_assign(size_type __n, const value_type& __val)
      {
	if (__n > size())
	  {
	    std::fill(begin(), end(), __val);
	    _M_fill_insert(end(), __n - size(), __val);
	  }
	else
	  {
	    _M_erase_at_end(begin() + difference_type(__n));
	    std::fill(begin(), end(), __val);
	  }
      }

      //@{
      /// Helper functions for push_* and pop_*.
#if __cplusplus < 201103L
      void _M_push_back_aux(const value_type&);

      void _M_push_front_aux(const value_type&);
#else
      template<typename... _Args>
	void _M_push_back_aux(_Args&&... __args);

      template<typename... _Args>
	void _M_push_front_aux(_Args&&... __args);
#endif

      void _M_pop_back_aux();

      void _M_pop_front_aux();
      //@}

      // Internal insert functions follow.  The *_aux functions do the actual
      // insertion work when all shortcuts fail.

#if __cplusplus < 201103L
      // called by the range insert to implement [23.1.1]/9

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 438. Ambiguity in the "do the right thing" clause
      template<typename _Integer>
	void
	_M_insert_dispatch(iterator __pos,
			   _Integer __n, _Integer __x, __true_type)
	{ _M_fill_insert(__pos, __n, __x); }

      // called by the range insert to implement [23.1.1]/9
      template<typename _InputIterator>
	void
	_M_insert_dispatch(iterator __pos,
			   _InputIterator __first, _InputIterator __last,
			   __false_type)
	{
	  _M_range_insert_aux(__pos, __first, __last,
			      std::__iterator_category(__first));
	}
#endif

      // called by the second insert_dispatch above
      template<typename _InputIterator>
	void
	_M_range_insert_aux(iterator __pos, _InputIterator __first,
			    _InputIterator __last, std::input_iterator_tag);

      // called by the second insert_dispatch above
      template<typename _ForwardIterator>
	void
	_M_range_insert_aux(iterator __pos, _ForwardIterator __first,
			    _ForwardIterator __last, std::forward_iterator_tag);

      // Called by insert(p,n,x), and the range insert when it turns out to be
      // the same thing.  Can use fill functions in optimal situations,
      // otherwise passes off to insert_aux(p,n,x).
      void
      _M_fill_insert(iterator __pos, size_type __n, const value_type& __x);

      // called by insert(p,x)
#if __cplusplus < 201103L
      iterator
      _M_insert_aux(iterator __pos, const value_type& __x);
#else
      template<typename... _Args>
	iterator
	_M_insert_aux(iterator __pos, _Args&&... __args);
#endif

      // called by insert(p,n,x) via fill_insert
      void
      _M_insert_aux(iterator __pos, size_type __n, const value_type& __x);

      // called by range_insert_aux for forward iterators
      template<typename _ForwardIterator>
	void
	_M_insert_aux(iterator __pos,
		      _ForwardIterator __first, _ForwardIterator __last,
		      size_type __n);


      // Internal erase functions follow.

      void
      _M_destroy_data_aux(iterator __first, iterator __last);

      // Called by ~deque().
      // NB: Doesn't deallocate the nodes.
      template<typename _Alloc1>
	void
	_M_destroy_data(iterator __first, iterator __last, const _Alloc1&)
	{ _M_destroy_data_aux(__first, __last); }

      void
      _M_destroy_data(iterator __first, iterator __last,
		      const std::allocator<_Tp>&)
      {
	if (!__has_trivial_destructor(value_type))
	  _M_destroy_data_aux(__first, __last);
      }

      // Called by erase(q1, q2).
      void
      _M_erase_at_begin(iterator __pos)
      {
	_M_destroy_data(begin(), __pos, _M_get_Tp_allocator());
	_M_destroy_nodes(this->_M_impl._M_start._M_node, __pos._M_node);
	this->_M_impl._M_start = __pos;
      }

      // Called by erase(q1, q2), resize(), clear(), _M_assign_aux,
      // _M_fill_assign, operator=.
      void
      _M_erase_at_end(iterator __pos)
      {
	_M_destroy_data(__pos, end(), _M_get_Tp_allocator());
	_M_destroy_nodes(__pos._M_node + 1,
			 this->_M_impl._M_finish._M_node + 1);
	this->_M_impl._M_finish = __pos;
      }

      iterator
      _M_erase(iterator __pos);

      iterator
      _M_erase(iterator __first, iterator __last);

#if __cplusplus >= 201103L
      // Called by resize(sz).
      void
      _M_default_append(size_type __n);

      bool
      _M_shrink_to_fit();
#endif

      //@{
      /// Memory-handling helpers for the previous internal insert functions.
      iterator
      _M_reserve_elements_at_front(size_type __n)
      {
	const size_type __vacancies = this->_M_impl._M_start._M_cur
				      - this->_M_impl._M_start._M_first;
	if (__n > __vacancies)
	  _M_new_elements_at_front(__n - __vacancies);
	return this->_M_impl._M_start - difference_type(__n);
      }

      iterator
      _M_reserve_elements_at_back(size_type __n)
      {
	const size_type __vacancies = (this->_M_impl._M_finish._M_last
				       - this->_M_impl._M_finish._M_cur) - 1;
	if (__n > __vacancies)
	  _M_new_elements_at_back(__n - __vacancies);
	return this->_M_impl._M_finish + difference_type(__n);
      }

      void
      _M_new_elements_at_front(size_type __new_elements);

      void
      _M_new_elements_at_back(size_type __new_elements);
      //@}


      //@{
      /**
       *  @brief Memory-handling helpers for the major %map.
       *
       *  Makes sure the _M_map has space for new nodes.  Does not
       *  actually add the nodes.  Can invalidate _M_map pointers.
       *  (And consequently, %deque iterators.)
       */
      void
      _M_reserve_map_at_back(size_type __nodes_to_add = 1)
      {
	if (__nodes_to_add + 1 > this->_M_impl._M_map_size
	    - (this->_M_impl._M_finish._M_node - this->_M_impl._M_map))
	  _M_reallocate_map(__nodes_to_add, false);
      }

      void
      _M_reserve_map_at_front(size_type __nodes_to_add = 1)
      {
	if (__nodes_to_add > size_type(this->_M_impl._M_start._M_node
				       - this->_M_impl._M_map))
	  _M_reallocate_map(__nodes_to_add, true);
      }

      void
      _M_reallocate_map(size_type __nodes_to_add, bool __add_at_front);
      //@}

#if __cplusplus >= 201103L
      // Constant-time, nothrow move assignment when source object's memory
      // can be moved because the allocators are equal.
      void
      _M_move_assign1(deque&& __x, /* always equal: */ true_type) noexcept
      {
	this->_M_impl._M_swap_data(__x._M_impl);
	__x.clear();
	std::__alloc_on_move(_M_get_Tp_allocator(), __x._M_get_Tp_allocator());
      }

      // When the allocators are not equal the operation could throw, because
      // we might need to allocate a new map for __x after moving from it
      // or we might need to allocate new elements for *this.
      void
      _M_move_assign1(deque&& __x, /* always equal: */ false_type)
      {
	constexpr bool __move_storage =
	  _Alloc_traits::_S_propagate_on_move_assign();
	_M_move_assign2(std::move(__x), __bool_constant<__move_storage>());
      }

      // Destroy all elements and deallocate all memory, then replace
      // with elements created from __args.
      template<typename... _Args>
      void
      _M_replace_map(_Args&&... __args)
      {
	// Create new data first, so if allocation fails there are no effects.
	deque __newobj(std::forward<_Args>(__args)...);
	// Free existing storage using existing allocator.
	clear();
	_M_deallocate_node(*begin()._M_node); // one node left after clear()
	_M_deallocate_map(this->_M_impl._M_map, this->_M_impl._M_map_size);
	this->_M_impl._M_map = nullptr;
	this->_M_impl._M_map_size = 0;
	// Take ownership of replacement memory.
	this->_M_impl._M_swap_data(__newobj._M_impl);
      }

      // Do move assignment when the allocator propagates.
      void
      _M_move_assign2(deque&& __x, /* propagate: */ true_type)
      {
	// Make a copy of the original allocator state.
	auto __alloc = __x._M_get_Tp_allocator();
	// The allocator propagates so storage can be moved from __x,
	// leaving __x in a valid empty state with a moved-from allocator.
	_M_replace_map(std::move(__x));
	// Move the corresponding allocator state too.
	_M_get_Tp_allocator() = std::move(__alloc);
      }

      // Do move assignment when it may not be possible to move source
      // object's memory, resulting in a linear-time operation.
      void
      _M_move_assign2(deque&& __x, /* propagate: */ false_type)
      {
	if (__x._M_get_Tp_allocator() == this->_M_get_Tp_allocator())
	  {
	    // The allocators are equal so storage can be moved from __x,
	    // leaving __x in a valid empty state with its current allocator.
	    _M_replace_map(std::move(__x), __x.get_allocator());
	  }
	else
	  {
	    // The rvalue's allocator cannot be moved and is not equal,
	    // so we need to individually move each element.
	    _M_assign_aux(std::make_move_iterator(__x.begin()),
			  std::make_move_iterator(__x.end()),
			  std::random_access_iterator_tag());
	    __x.clear();
	  }
      }
#endif
    };

#if __cpp_deduction_guides >= 201606
  template<typename _InputIterator, typename _ValT
	     = typename iterator_traits<_InputIterator>::value_type,
	   typename _Allocator = allocator<_ValT>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireAllocator<_Allocator>>
    deque(_InputIterator, _InputIterator, _Allocator = _Allocator())
      -> deque<_ValT, _Allocator>;
#endif

  /**
   *  @brief  Deque equality comparison.
   *  @param  __x  A %deque.
   *  @param  __y  A %deque of the same type as @a __x.
   *  @return  True iff the size and elements of the deques are equal.
   *
   *  This is an equivalence relation.  It is linear in the size of the
   *  deques.  Deques are considered equivalent if their sizes are equal,
   *  and if corresponding elements compare equal.
  */
  template<typename _Tp, typename _Alloc>
    inline bool
    operator==(const deque<_Tp, _Alloc>& __x, const deque<_Tp, _Alloc>& __y)
    { return __x.size() == __y.size()
	     && std::equal(__x.begin(), __x.end(), __y.begin()); }

#if __cpp_lib_three_way_comparison
  /**
   *  @brief  Deque ordering relation.
   *  @param  __x  A `deque`.
   *  @param  __y  A `deque` of the same type as `__x`.
   *  @return  A value indicating whether `__x` is less than, equal to,
   *           greater than, or incomparable with `__y`.
   *
   *  See `std::lexicographical_compare_three_way()` for how the determination
   *  is made. This operator is used to synthesize relational operators like
   *  `<` and `>=` etc.
  */
  template<typename _Tp, typename _Alloc>
    inline __detail::__synth3way_t<_Tp>
    operator<=>(const deque<_Tp, _Alloc>& __x, const deque<_Tp, _Alloc>& __y)
    {
      return std::lexicographical_compare_three_way(__x.begin(), __x.end(),
						    __y.begin(), __y.end(),
						    __detail::__synth3way);
    }
#else
  /**
   *  @brief  Deque ordering relation.
   *  @param  __x  A %deque.
   *  @param  __y  A %deque of the same type as @a __x.
   *  @return  True iff @a x is lexicographically less than @a __y.
   *
   *  This is a total ordering relation.  It is linear in the size of the
   *  deques.  The elements must be comparable with @c <.
   *
   *  See std::lexicographical_compare() for how the determination is made.
  */
  template<typename _Tp, typename _Alloc>
    inline bool
    operator<(const deque<_Tp, _Alloc>& __x, const deque<_Tp, _Alloc>& __y)
    { return std::lexicographical_compare(__x.begin(), __x.end(),
					  __y.begin(), __y.end()); }

  /// Based on operator==
  template<typename _Tp, typename _Alloc>
    inline bool
    operator!=(const deque<_Tp, _Alloc>& __x, const deque<_Tp, _Alloc>& __y)
    { return !(__x == __y); }

  /// Based on operator<
  template<typename _Tp, typename _Alloc>
    inline bool
    operator>(const deque<_Tp, _Alloc>& __x, const deque<_Tp, _Alloc>& __y)
    { return __y < __x; }

  /// Based on operator<
  template<typename _Tp, typename _Alloc>
    inline bool
    operator<=(const deque<_Tp, _Alloc>& __x, const deque<_Tp, _Alloc>& __y)
    { return !(__y < __x); }

  /// Based on operator<
  template<typename _Tp, typename _Alloc>
    inline bool
    operator>=(const deque<_Tp, _Alloc>& __x, const deque<_Tp, _Alloc>& __y)
    { return !(__x < __y); }
#endif // three-way comparison

  /// See std::deque::swap().
  template<typename _Tp, typename _Alloc>
    inline void
    swap(deque<_Tp,_Alloc>& __x, deque<_Tp,_Alloc>& __y)
    _GLIBCXX_NOEXCEPT_IF(noexcept(__x.swap(__y)))
    { __x.swap(__y); }

#undef _GLIBCXX_DEQUE_BUF_SIZE

_GLIBCXX_END_NAMESPACE_CONTAINER

#if __cplusplus >= 201103L
  // std::allocator is safe, but it is not the only allocator
  // for which this is valid.
  template<class _Tp>
    struct __is_bitwise_relocatable<_GLIBCXX_STD_C::deque<_Tp>>
    : true_type { };
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif /* _STL_DEQUE_H */
