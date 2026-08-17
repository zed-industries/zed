// RB tree implementation -*- C++ -*-

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
 * Copyright (c) 1996,1997
 * Silicon Graphics Computer Systems, Inc.
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Silicon Graphics makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
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
 */

/** @file bits/stl_tree.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{map,set}
 */

#ifndef _STL_TREE_H
#define _STL_TREE_H 1

#pragma GCC system_header

#include <bits/stl_algobase.h>
#include <bits/allocator.h>
#include <bits/stl_function.h>
#include <bits/cpp_type_traits.h>
#include <ext/alloc_traits.h>
#if __cplusplus >= 201103L
# include <ext/aligned_buffer.h>
#endif
#if __cplusplus > 201402L
# include <bits/node_handle.h>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

#if __cplusplus > 201103L
# define __cpp_lib_generic_associative_lookup 201304
#endif

  // Red-black tree class, designed for use in implementing STL
  // associative containers (set, multiset, map, and multimap). The
  // insertion and deletion algorithms are based on those in Cormen,
  // Leiserson, and Rivest, Introduction to Algorithms (MIT Press,
  // 1990), except that
  //
  // (1) the header cell is maintained with links not only to the root
  // but also to the leftmost node of the tree, to enable constant
  // time begin(), and to the rightmost node of the tree, to enable
  // linear time performance when used with the generic set algorithms
  // (set_union, etc.)
  //
  // (2) when a node being deleted has two children its successor node
  // is relinked into its place, rather than copied, so that the only
  // iterators invalidated are those referring to the deleted node.

  enum _Rb_tree_color { _S_red = false, _S_black = true };

  struct _Rb_tree_node_base
  {
    typedef _Rb_tree_node_base* _Base_ptr;
    typedef const _Rb_tree_node_base* _Const_Base_ptr;

    _Rb_tree_color	_M_color;
    _Base_ptr		_M_parent;
    _Base_ptr		_M_left;
    _Base_ptr		_M_right;

    static _Base_ptr
    _S_minimum(_Base_ptr __x) _GLIBCXX_NOEXCEPT
    {
      while (__x->_M_left != 0) __x = __x->_M_left;
      return __x;
    }

    static _Const_Base_ptr
    _S_minimum(_Const_Base_ptr __x) _GLIBCXX_NOEXCEPT
    {
      while (__x->_M_left != 0) __x = __x->_M_left;
      return __x;
    }

    static _Base_ptr
    _S_maximum(_Base_ptr __x) _GLIBCXX_NOEXCEPT
    {
      while (__x->_M_right != 0) __x = __x->_M_right;
      return __x;
    }

    static _Const_Base_ptr
    _S_maximum(_Const_Base_ptr __x) _GLIBCXX_NOEXCEPT
    {
      while (__x->_M_right != 0) __x = __x->_M_right;
      return __x;
    }
  };

  // Helper type offering value initialization guarantee on the compare functor.
  template<typename _Key_compare>
    struct _Rb_tree_key_compare
    {
      _Key_compare		_M_key_compare;

      _Rb_tree_key_compare()
      _GLIBCXX_NOEXCEPT_IF(
	is_nothrow_default_constructible<_Key_compare>::value)
      : _M_key_compare()
      { }

      _Rb_tree_key_compare(const _Key_compare& __comp)
      : _M_key_compare(__comp)
      { }

#if __cplusplus >= 201103L
      // Copy constructor added for consistency with C++98 mode.
      _Rb_tree_key_compare(const _Rb_tree_key_compare&) = default;

      _Rb_tree_key_compare(_Rb_tree_key_compare&& __x)
	noexcept(is_nothrow_copy_constructible<_Key_compare>::value)
      : _M_key_compare(__x._M_key_compare)
      { }
#endif
    };

  // Helper type to manage default initialization of node count and header.
  struct _Rb_tree_header
  {
    _Rb_tree_node_base	_M_header;
    size_t		_M_node_count; // Keeps track of size of tree.

    _Rb_tree_header() _GLIBCXX_NOEXCEPT
    {
      _M_header._M_color = _S_red;
      _M_reset();
    }

#if __cplusplus >= 201103L
    _Rb_tree_header(_Rb_tree_header&& __x) noexcept
    {
      if (__x._M_header._M_parent != nullptr)
	_M_move_data(__x);
      else
	{
	  _M_header._M_color = _S_red;
	  _M_reset();
	}
    }
#endif

    void
    _M_move_data(_Rb_tree_header& __from)
    {
      _M_header._M_color = __from._M_header._M_color;
      _M_header._M_parent = __from._M_header._M_parent;
      _M_header._M_left = __from._M_header._M_left;
      _M_header._M_right = __from._M_header._M_right;
      _M_header._M_parent->_M_parent = &_M_header;
      _M_node_count = __from._M_node_count;

      __from._M_reset();
    }

    void
    _M_reset()
    {
      _M_header._M_parent = 0;
      _M_header._M_left = &_M_header;
      _M_header._M_right = &_M_header;
      _M_node_count = 0;
    }
  };

  template<typename _Val>
    struct _Rb_tree_node : public _Rb_tree_node_base
    {
      typedef _Rb_tree_node<_Val>* _Link_type;

#if __cplusplus < 201103L
      _Val _M_value_field;

      _Val*
      _M_valptr()
      { return std::__addressof(_M_value_field); }

      const _Val*
      _M_valptr() const
      { return std::__addressof(_M_value_field); }
#else
      __gnu_cxx::__aligned_membuf<_Val> _M_storage;

      _Val*
      _M_valptr()
      { return _M_storage._M_ptr(); }

      const _Val*
      _M_valptr() const
      { return _M_storage._M_ptr(); }
#endif
    };

  _GLIBCXX_PURE _Rb_tree_node_base*
  _Rb_tree_increment(_Rb_tree_node_base* __x) throw ();

  _GLIBCXX_PURE const _Rb_tree_node_base*
  _Rb_tree_increment(const _Rb_tree_node_base* __x) throw ();

  _GLIBCXX_PURE _Rb_tree_node_base*
  _Rb_tree_decrement(_Rb_tree_node_base* __x) throw ();

  _GLIBCXX_PURE const _Rb_tree_node_base*
  _Rb_tree_decrement(const _Rb_tree_node_base* __x) throw ();

  template<typename _Tp>
    struct _Rb_tree_iterator
    {
      typedef _Tp  value_type;
      typedef _Tp& reference;
      typedef _Tp* pointer;

      typedef bidirectional_iterator_tag iterator_category;
      typedef ptrdiff_t			 difference_type;

      typedef _Rb_tree_iterator<_Tp>		_Self;
      typedef _Rb_tree_node_base::_Base_ptr	_Base_ptr;
      typedef _Rb_tree_node<_Tp>*		_Link_type;

      _Rb_tree_iterator() _GLIBCXX_NOEXCEPT
      : _M_node() { }

      explicit
      _Rb_tree_iterator(_Base_ptr __x) _GLIBCXX_NOEXCEPT
      : _M_node(__x) { }

      reference
      operator*() const _GLIBCXX_NOEXCEPT
      { return *static_cast<_Link_type>(_M_node)->_M_valptr(); }

      pointer
      operator->() const _GLIBCXX_NOEXCEPT
      { return static_cast<_Link_type> (_M_node)->_M_valptr(); }

      _Self&
      operator++() _GLIBCXX_NOEXCEPT
      {
	_M_node = _Rb_tree_increment(_M_node);
	return *this;
      }

      _Self
      operator++(int) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = *this;
	_M_node = _Rb_tree_increment(_M_node);
	return __tmp;
      }

      _Self&
      operator--() _GLIBCXX_NOEXCEPT
      {
	_M_node = _Rb_tree_decrement(_M_node);
	return *this;
      }

      _Self
      operator--(int) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = *this;
	_M_node = _Rb_tree_decrement(_M_node);
	return __tmp;
      }

      friend bool
      operator==(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return __x._M_node == __y._M_node; }

#if ! __cpp_lib_three_way_comparison
      friend bool
      operator!=(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return __x._M_node != __y._M_node; }
#endif

      _Base_ptr _M_node;
  };

  template<typename _Tp>
    struct _Rb_tree_const_iterator
    {
      typedef _Tp	 value_type;
      typedef const _Tp& reference;
      typedef const _Tp* pointer;

      typedef _Rb_tree_iterator<_Tp> iterator;

      typedef bidirectional_iterator_tag iterator_category;
      typedef ptrdiff_t			 difference_type;

      typedef _Rb_tree_const_iterator<_Tp>		_Self;
      typedef _Rb_tree_node_base::_Const_Base_ptr	_Base_ptr;
      typedef const _Rb_tree_node<_Tp>*			_Link_type;

      _Rb_tree_const_iterator() _GLIBCXX_NOEXCEPT
      : _M_node() { }

      explicit
      _Rb_tree_const_iterator(_Base_ptr __x) _GLIBCXX_NOEXCEPT
      : _M_node(__x) { }

      _Rb_tree_const_iterator(const iterator& __it) _GLIBCXX_NOEXCEPT
      : _M_node(__it._M_node) { }

      iterator
      _M_const_cast() const _GLIBCXX_NOEXCEPT
      { return iterator(const_cast<typename iterator::_Base_ptr>(_M_node)); }

      reference
      operator*() const _GLIBCXX_NOEXCEPT
      { return *static_cast<_Link_type>(_M_node)->_M_valptr(); }

      pointer
      operator->() const _GLIBCXX_NOEXCEPT
      { return static_cast<_Link_type>(_M_node)->_M_valptr(); }

      _Self&
      operator++() _GLIBCXX_NOEXCEPT
      {
	_M_node = _Rb_tree_increment(_M_node);
	return *this;
      }

      _Self
      operator++(int) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = *this;
	_M_node = _Rb_tree_increment(_M_node);
	return __tmp;
      }

      _Self&
      operator--() _GLIBCXX_NOEXCEPT
      {
	_M_node = _Rb_tree_decrement(_M_node);
	return *this;
      }

      _Self
      operator--(int) _GLIBCXX_NOEXCEPT
      {
	_Self __tmp = *this;
	_M_node = _Rb_tree_decrement(_M_node);
	return __tmp;
      }

      friend bool
      operator==(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return __x._M_node == __y._M_node; }

#if ! __cpp_lib_three_way_comparison
      friend bool
      operator!=(const _Self& __x, const _Self& __y) _GLIBCXX_NOEXCEPT
      { return __x._M_node != __y._M_node; }
#endif

      _Base_ptr _M_node;
    };

  void
  _Rb_tree_insert_and_rebalance(const bool __insert_left,
				_Rb_tree_node_base* __x,
				_Rb_tree_node_base* __p,
				_Rb_tree_node_base& __header) throw ();

  _Rb_tree_node_base*
  _Rb_tree_rebalance_for_erase(_Rb_tree_node_base* const __z,
			       _Rb_tree_node_base& __header) throw ();

#if __cplusplus >= 201402L
  template<typename _Cmp, typename _SfinaeType, typename = __void_t<>>
    struct __has_is_transparent
    { };

  template<typename _Cmp, typename _SfinaeType>
    struct __has_is_transparent<_Cmp, _SfinaeType,
				__void_t<typename _Cmp::is_transparent>>
    { typedef void type; };

  template<typename _Cmp, typename _SfinaeType>
    using __has_is_transparent_t
      = typename __has_is_transparent<_Cmp, _SfinaeType>::type;
#endif

#if __cplusplus > 201402L
  template<typename _Tree1, typename _Cmp2>
    struct _Rb_tree_merge_helper { };
#endif

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc = allocator<_Val> >
    class _Rb_tree
    {
      typedef typename __gnu_cxx::__alloc_traits<_Alloc>::template
	rebind<_Rb_tree_node<_Val> >::other _Node_allocator;

      typedef __gnu_cxx::__alloc_traits<_Node_allocator> _Alloc_traits;

    protected:
      typedef _Rb_tree_node_base* 		_Base_ptr;
      typedef const _Rb_tree_node_base* 	_Const_Base_ptr;
      typedef _Rb_tree_node<_Val>* 		_Link_type;
      typedef const _Rb_tree_node<_Val>*	_Const_Link_type;

    private:
      // Functor recycling a pool of nodes and using allocation once the pool
      // is empty.
      struct _Reuse_or_alloc_node
      {
	_Reuse_or_alloc_node(_Rb_tree& __t)
	: _M_root(__t._M_root()), _M_nodes(__t._M_rightmost()), _M_t(__t)
	{
	  if (_M_root)
	    {
	      _M_root->_M_parent = 0;

	      if (_M_nodes->_M_left)
		_M_nodes = _M_nodes->_M_left;
	    }
	  else
	    _M_nodes = 0;
	}

#if __cplusplus >= 201103L
	_Reuse_or_alloc_node(const _Reuse_or_alloc_node&) = delete;
#endif

	~_Reuse_or_alloc_node()
	{ _M_t._M_erase(static_cast<_Link_type>(_M_root)); }

	template<typename _Arg>
	  _Link_type
#if __cplusplus < 201103L
	  operator()(const _Arg& __arg)
#else
	  operator()(_Arg&& __arg)
#endif
	  {
	    _Link_type __node = static_cast<_Link_type>(_M_extract());
	    if (__node)
	      {
		_M_t._M_destroy_node(__node);
		_M_t._M_construct_node(__node, _GLIBCXX_FORWARD(_Arg, __arg));
		return __node;
	      }

	    return _M_t._M_create_node(_GLIBCXX_FORWARD(_Arg, __arg));
	  }

      private:
	_Base_ptr
	_M_extract()
	{
	  if (!_M_nodes)
	    return _M_nodes;

	  _Base_ptr __node = _M_nodes;
	  _M_nodes = _M_nodes->_M_parent;
	  if (_M_nodes)
	    {
	      if (_M_nodes->_M_right == __node)
		{
		  _M_nodes->_M_right = 0;

		  if (_M_nodes->_M_left)
		    {
		      _M_nodes = _M_nodes->_M_left;

		      while (_M_nodes->_M_right)
			_M_nodes = _M_nodes->_M_right;

		      if (_M_nodes->_M_left)
			_M_nodes = _M_nodes->_M_left;
		    }
		}
	      else // __node is on the left.
		_M_nodes->_M_left = 0;
	    }
	  else
	    _M_root = 0;

	  return __node;
	}

	_Base_ptr _M_root;
	_Base_ptr _M_nodes;
	_Rb_tree& _M_t;
      };

      // Functor similar to the previous one but without any pool of nodes to
      // recycle.
      struct _Alloc_node
      {
	_Alloc_node(_Rb_tree& __t)
	: _M_t(__t) { }

	template<typename _Arg>
	  _Link_type
#if __cplusplus < 201103L
	  operator()(const _Arg& __arg) const
#else
	  operator()(_Arg&& __arg) const
#endif
	  { return _M_t._M_create_node(_GLIBCXX_FORWARD(_Arg, __arg)); }

      private:
	_Rb_tree& _M_t;
      };

    public:
      typedef _Key 				key_type;
      typedef _Val 				value_type;
      typedef value_type* 			pointer;
      typedef const value_type* 		const_pointer;
      typedef value_type& 			reference;
      typedef const value_type& 		const_reference;
      typedef size_t 				size_type;
      typedef ptrdiff_t 			difference_type;
      typedef _Alloc 				allocator_type;

      _Node_allocator&
      _M_get_Node_allocator() _GLIBCXX_NOEXCEPT
      { return this->_M_impl; }

      const _Node_allocator&
      _M_get_Node_allocator() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl; }

      allocator_type
      get_allocator() const _GLIBCXX_NOEXCEPT
      { return allocator_type(_M_get_Node_allocator()); }

    protected:
      _Link_type
      _M_get_node()
      { return _Alloc_traits::allocate(_M_get_Node_allocator(), 1); }

      void
      _M_put_node(_Link_type __p) _GLIBCXX_NOEXCEPT
      { _Alloc_traits::deallocate(_M_get_Node_allocator(), __p, 1); }

#if __cplusplus < 201103L
      void
      _M_construct_node(_Link_type __node, const value_type& __x)
      {
	__try
	  { get_allocator().construct(__node->_M_valptr(), __x); }
	__catch(...)
	  {
	    _M_put_node(__node);
	    __throw_exception_again;
	  }
      }

      _Link_type
      _M_create_node(const value_type& __x)
      {
	_Link_type __tmp = _M_get_node();
	_M_construct_node(__tmp, __x);
	return __tmp;
      }
#else
      template<typename... _Args>
	void
	_M_construct_node(_Link_type __node, _Args&&... __args)
	{
	  __try
	    {
	      ::new(__node) _Rb_tree_node<_Val>;
	      _Alloc_traits::construct(_M_get_Node_allocator(),
				       __node->_M_valptr(),
				       std::forward<_Args>(__args)...);
	    }
	  __catch(...)
	    {
	      __node->~_Rb_tree_node<_Val>();
	      _M_put_node(__node);
	      __throw_exception_again;
	    }
	}

      template<typename... _Args>
	_Link_type
	_M_create_node(_Args&&... __args)
	{
	  _Link_type __tmp = _M_get_node();
	  _M_construct_node(__tmp, std::forward<_Args>(__args)...);
	  return __tmp;
	}
#endif

      void
      _M_destroy_node(_Link_type __p) _GLIBCXX_NOEXCEPT
      {
#if __cplusplus < 201103L
	get_allocator().destroy(__p->_M_valptr());
#else
	_Alloc_traits::destroy(_M_get_Node_allocator(), __p->_M_valptr());
	__p->~_Rb_tree_node<_Val>();
#endif
      }

      void
      _M_drop_node(_Link_type __p) _GLIBCXX_NOEXCEPT
      {
	_M_destroy_node(__p);
	_M_put_node(__p);
      }

      template<typename _NodeGen>
	_Link_type
	_M_clone_node(_Const_Link_type __x, _NodeGen& __node_gen)
	{
	  _Link_type __tmp = __node_gen(*__x->_M_valptr());
	  __tmp->_M_color = __x->_M_color;
	  __tmp->_M_left = 0;
	  __tmp->_M_right = 0;
	  return __tmp;
	}

    protected:
#if _GLIBCXX_INLINE_VERSION
      template<typename _Key_compare>
#else
      // Unused _Is_pod_comparator is kept as it is part of mangled name.
      template<typename _Key_compare,
	       bool /* _Is_pod_comparator */ = __is_pod(_Key_compare)>
#endif
	struct _Rb_tree_impl
	: public _Node_allocator
	, public _Rb_tree_key_compare<_Key_compare>
	, public _Rb_tree_header
	{
	  typedef _Rb_tree_key_compare<_Key_compare> _Base_key_compare;

	  _Rb_tree_impl()
	    _GLIBCXX_NOEXCEPT_IF(
		is_nothrow_default_constructible<_Node_allocator>::value
		&& is_nothrow_default_constructible<_Base_key_compare>::value )
	  : _Node_allocator()
	  { }

	  _Rb_tree_impl(const _Rb_tree_impl& __x)
	  : _Node_allocator(_Alloc_traits::_S_select_on_copy(__x))
	  , _Base_key_compare(__x._M_key_compare)
	  { }

#if __cplusplus < 201103L
	  _Rb_tree_impl(const _Key_compare& __comp, const _Node_allocator& __a)
	  : _Node_allocator(__a), _Base_key_compare(__comp)
	  { }
#else
	  _Rb_tree_impl(_Rb_tree_impl&&) = default;

	  explicit
	  _Rb_tree_impl(_Node_allocator&& __a)
	  : _Node_allocator(std::move(__a))
	  { }

	  _Rb_tree_impl(_Rb_tree_impl&& __x, _Node_allocator&& __a)
	  : _Node_allocator(std::move(__a)),
	    _Base_key_compare(std::move(__x)),
	    _Rb_tree_header(std::move(__x))
	  { }

	  _Rb_tree_impl(const _Key_compare& __comp, _Node_allocator&& __a)
	  : _Node_allocator(std::move(__a)), _Base_key_compare(__comp)
	  { }
#endif
	};

      _Rb_tree_impl<_Compare> _M_impl;

    protected:
      _Base_ptr&
      _M_root() _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_header._M_parent; }

      _Const_Base_ptr
      _M_root() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_header._M_parent; }

      _Base_ptr&
      _M_leftmost() _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_header._M_left; }

      _Const_Base_ptr
      _M_leftmost() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_header._M_left; }

      _Base_ptr&
      _M_rightmost() _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_header._M_right; }

      _Const_Base_ptr
      _M_rightmost() const _GLIBCXX_NOEXCEPT
      { return this->_M_impl._M_header._M_right; }

      _Link_type
      _M_begin() _GLIBCXX_NOEXCEPT
      { return static_cast<_Link_type>(this->_M_impl._M_header._M_parent); }

      _Const_Link_type
      _M_begin() const _GLIBCXX_NOEXCEPT
      {
	return static_cast<_Const_Link_type>
	  (this->_M_impl._M_header._M_parent);
      }

      _Base_ptr
      _M_end() _GLIBCXX_NOEXCEPT
      { return &this->_M_impl._M_header; }

      _Const_Base_ptr
      _M_end() const _GLIBCXX_NOEXCEPT
      { return &this->_M_impl._M_header; }

      static const _Key&
      _S_key(_Const_Link_type __x)
      {
#if __cplusplus >= 201103L
	// If we're asking for the key we're presumably using the comparison
	// object, and so this is a good place to sanity check it.
	static_assert(__is_invocable<_Compare&, const _Key&, const _Key&>{},
		      "comparison object must be invocable "
		      "with two arguments of key type");
# if __cplusplus >= 201703L
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// 2542. Missing const requirements for associative containers
	if constexpr (__is_invocable<_Compare&, const _Key&, const _Key&>{})
	  static_assert(
	      is_invocable_v<const _Compare&, const _Key&, const _Key&>,
	      "comparison object must be invocable as const");
# endif // C++17
#endif // C++11

	return _KeyOfValue()(*__x->_M_valptr());
      }

      static _Link_type
      _S_left(_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return static_cast<_Link_type>(__x->_M_left); }

      static _Const_Link_type
      _S_left(_Const_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return static_cast<_Const_Link_type>(__x->_M_left); }

      static _Link_type
      _S_right(_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return static_cast<_Link_type>(__x->_M_right); }

      static _Const_Link_type
      _S_right(_Const_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return static_cast<_Const_Link_type>(__x->_M_right); }

      static const _Key&
      _S_key(_Const_Base_ptr __x)
      { return _S_key(static_cast<_Const_Link_type>(__x)); }

      static _Base_ptr
      _S_minimum(_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return _Rb_tree_node_base::_S_minimum(__x); }

      static _Const_Base_ptr
      _S_minimum(_Const_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return _Rb_tree_node_base::_S_minimum(__x); }

      static _Base_ptr
      _S_maximum(_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return _Rb_tree_node_base::_S_maximum(__x); }

      static _Const_Base_ptr
      _S_maximum(_Const_Base_ptr __x) _GLIBCXX_NOEXCEPT
      { return _Rb_tree_node_base::_S_maximum(__x); }

    public:
      typedef _Rb_tree_iterator<value_type>       iterator;
      typedef _Rb_tree_const_iterator<value_type> const_iterator;

      typedef std::reverse_iterator<iterator>       reverse_iterator;
      typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

#if __cplusplus > 201402L
      using node_type = _Node_handle<_Key, _Val, _Node_allocator>;
      using insert_return_type = _Node_insert_return<
	conditional_t<is_same_v<_Key, _Val>, const_iterator, iterator>,
	node_type>;
#endif

      pair<_Base_ptr, _Base_ptr>
      _M_get_insert_unique_pos(const key_type& __k);

      pair<_Base_ptr, _Base_ptr>
      _M_get_insert_equal_pos(const key_type& __k);

      pair<_Base_ptr, _Base_ptr>
      _M_get_insert_hint_unique_pos(const_iterator __pos,
				    const key_type& __k);

      pair<_Base_ptr, _Base_ptr>
      _M_get_insert_hint_equal_pos(const_iterator __pos,
				   const key_type& __k);

    private:
#if __cplusplus >= 201103L
      template<typename _Arg, typename _NodeGen>
	iterator
	_M_insert_(_Base_ptr __x, _Base_ptr __y, _Arg&& __v, _NodeGen&);

      iterator
      _M_insert_node(_Base_ptr __x, _Base_ptr __y, _Link_type __z);

      template<typename _Arg>
	iterator
	_M_insert_lower(_Base_ptr __y, _Arg&& __v);

      template<typename _Arg>
	iterator
	_M_insert_equal_lower(_Arg&& __x);

      iterator
      _M_insert_lower_node(_Base_ptr __p, _Link_type __z);

      iterator
      _M_insert_equal_lower_node(_Link_type __z);
#else
      template<typename _NodeGen>
	iterator
	_M_insert_(_Base_ptr __x, _Base_ptr __y,
		   const value_type& __v, _NodeGen&);

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 233. Insertion hints in associative containers.
      iterator
      _M_insert_lower(_Base_ptr __y, const value_type& __v);

      iterator
      _M_insert_equal_lower(const value_type& __x);
#endif

      template<typename _NodeGen>
	_Link_type
	_M_copy(_Const_Link_type __x, _Base_ptr __p, _NodeGen&);

      template<typename _NodeGen>
	_Link_type
	_M_copy(const _Rb_tree& __x, _NodeGen& __gen)
	{
	  _Link_type __root = _M_copy(__x._M_begin(), _M_end(), __gen);
	  _M_leftmost() = _S_minimum(__root);
	  _M_rightmost() = _S_maximum(__root);
	  _M_impl._M_node_count = __x._M_impl._M_node_count;
	  return __root;
	}

      _Link_type
      _M_copy(const _Rb_tree& __x)
      {
	_Alloc_node __an(*this);
	return _M_copy(__x, __an);
      }

      void
      _M_erase(_Link_type __x);

      iterator
      _M_lower_bound(_Link_type __x, _Base_ptr __y,
		     const _Key& __k);

      const_iterator
      _M_lower_bound(_Const_Link_type __x, _Const_Base_ptr __y,
		     const _Key& __k) const;

      iterator
      _M_upper_bound(_Link_type __x, _Base_ptr __y,
		     const _Key& __k);

      const_iterator
      _M_upper_bound(_Const_Link_type __x, _Const_Base_ptr __y,
		     const _Key& __k) const;

    public:
      // allocation/deallocation
#if __cplusplus < 201103L
      _Rb_tree() { }
#else
      _Rb_tree() = default;
#endif

      _Rb_tree(const _Compare& __comp,
	       const allocator_type& __a = allocator_type())
      : _M_impl(__comp, _Node_allocator(__a)) { }

      _Rb_tree(const _Rb_tree& __x)
      : _M_impl(__x._M_impl)
      {
	if (__x._M_root() != 0)
	  _M_root() = _M_copy(__x);
      }

#if __cplusplus >= 201103L
      _Rb_tree(const allocator_type& __a)
      : _M_impl(_Node_allocator(__a))
      { }

      _Rb_tree(const _Rb_tree& __x, const allocator_type& __a)
      : _M_impl(__x._M_impl._M_key_compare, _Node_allocator(__a))
      {
	if (__x._M_root() != nullptr)
	  _M_root() = _M_copy(__x);
      }

      _Rb_tree(_Rb_tree&&) = default;

      _Rb_tree(_Rb_tree&& __x, const allocator_type& __a)
      : _Rb_tree(std::move(__x), _Node_allocator(__a))
      { }

    private:
      _Rb_tree(_Rb_tree&& __x, _Node_allocator&& __a, true_type)
      noexcept(is_nothrow_default_constructible<_Compare>::value)
      : _M_impl(std::move(__x._M_impl), std::move(__a))
      { }

      _Rb_tree(_Rb_tree&& __x, _Node_allocator&& __a, false_type)
      : _M_impl(__x._M_impl._M_key_compare, std::move(__a))
      {
	if (__x._M_root() != nullptr)
	  _M_move_data(__x, false_type{});
      }

    public:
      _Rb_tree(_Rb_tree&& __x, _Node_allocator&& __a)
      noexcept( noexcept(
	_Rb_tree(std::declval<_Rb_tree&&>(), std::declval<_Node_allocator&&>(),
		 std::declval<typename _Alloc_traits::is_always_equal>())) )
      : _Rb_tree(std::move(__x), std::move(__a),
		 typename _Alloc_traits::is_always_equal{})
      { }
#endif

      ~_Rb_tree() _GLIBCXX_NOEXCEPT
      { _M_erase(_M_begin()); }

      _Rb_tree&
      operator=(const _Rb_tree& __x);

      // Accessors.
      _Compare
      key_comp() const
      { return _M_impl._M_key_compare; }

      iterator
      begin() _GLIBCXX_NOEXCEPT
      { return iterator(this->_M_impl._M_header._M_left); }

      const_iterator
      begin() const _GLIBCXX_NOEXCEPT
      { return const_iterator(this->_M_impl._M_header._M_left); }

      iterator
      end() _GLIBCXX_NOEXCEPT
      { return iterator(&this->_M_impl._M_header); }

      const_iterator
      end() const _GLIBCXX_NOEXCEPT
      { return const_iterator(&this->_M_impl._M_header); }

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

      _GLIBCXX_NODISCARD bool
      empty() const _GLIBCXX_NOEXCEPT
      { return _M_impl._M_node_count == 0; }

      size_type
      size() const _GLIBCXX_NOEXCEPT
      { return _M_impl._M_node_count; }

      size_type
      max_size() const _GLIBCXX_NOEXCEPT
      { return _Alloc_traits::max_size(_M_get_Node_allocator()); }

      void
      swap(_Rb_tree& __t)
      _GLIBCXX_NOEXCEPT_IF(__is_nothrow_swappable<_Compare>::value);

      // Insert/erase.
#if __cplusplus >= 201103L
      template<typename _Arg>
	pair<iterator, bool>
	_M_insert_unique(_Arg&& __x);

      template<typename _Arg>
	iterator
	_M_insert_equal(_Arg&& __x);

      template<typename _Arg, typename _NodeGen>
	iterator
	_M_insert_unique_(const_iterator __pos, _Arg&& __x, _NodeGen&);

      template<typename _Arg>
	iterator
	_M_insert_unique_(const_iterator __pos, _Arg&& __x)
	{
	  _Alloc_node __an(*this);
	  return _M_insert_unique_(__pos, std::forward<_Arg>(__x), __an);
	}

      template<typename _Arg, typename _NodeGen>
	iterator
	_M_insert_equal_(const_iterator __pos, _Arg&& __x, _NodeGen&);

      template<typename _Arg>
	iterator
	_M_insert_equal_(const_iterator __pos, _Arg&& __x)
	{
	  _Alloc_node __an(*this);
	  return _M_insert_equal_(__pos, std::forward<_Arg>(__x), __an);
	}

      template<typename... _Args>
	pair<iterator, bool>
	_M_emplace_unique(_Args&&... __args);

      template<typename... _Args>
	iterator
	_M_emplace_equal(_Args&&... __args);

      template<typename... _Args>
	iterator
	_M_emplace_hint_unique(const_iterator __pos, _Args&&... __args);

      template<typename... _Args>
	iterator
	_M_emplace_hint_equal(const_iterator __pos, _Args&&... __args);

      template<typename _Iter>
	using __same_value_type
	  = is_same<value_type, typename iterator_traits<_Iter>::value_type>;

      template<typename _InputIterator>
	__enable_if_t<__same_value_type<_InputIterator>::value>
	_M_insert_range_unique(_InputIterator __first, _InputIterator __last)
	{
	  _Alloc_node __an(*this);
	  for (; __first != __last; ++__first)
	    _M_insert_unique_(end(), *__first, __an);
	}

      template<typename _InputIterator>
	__enable_if_t<!__same_value_type<_InputIterator>::value>
	_M_insert_range_unique(_InputIterator __first, _InputIterator __last)
	{
	  for (; __first != __last; ++__first)
	    _M_emplace_unique(*__first);
	}

      template<typename _InputIterator>
	__enable_if_t<__same_value_type<_InputIterator>::value>
	_M_insert_range_equal(_InputIterator __first, _InputIterator __last)
	{
	  _Alloc_node __an(*this);
	  for (; __first != __last; ++__first)
	    _M_insert_equal_(end(), *__first, __an);
	}

      template<typename _InputIterator>
	__enable_if_t<!__same_value_type<_InputIterator>::value>
	_M_insert_range_equal(_InputIterator __first, _InputIterator __last)
	{
	  _Alloc_node __an(*this);
	  for (; __first != __last; ++__first)
	    _M_emplace_equal(*__first);
	}
#else
      pair<iterator, bool>
      _M_insert_unique(const value_type& __x);

      iterator
      _M_insert_equal(const value_type& __x);

      template<typename _NodeGen>
	iterator
	_M_insert_unique_(const_iterator __pos, const value_type& __x,
			  _NodeGen&);

      iterator
      _M_insert_unique_(const_iterator __pos, const value_type& __x)
      {
	_Alloc_node __an(*this);
	return _M_insert_unique_(__pos, __x, __an);
      }

      template<typename _NodeGen>
	iterator
	_M_insert_equal_(const_iterator __pos, const value_type& __x,
			 _NodeGen&);
      iterator
      _M_insert_equal_(const_iterator __pos, const value_type& __x)
      {
	_Alloc_node __an(*this);
	return _M_insert_equal_(__pos, __x, __an);
      }

      template<typename _InputIterator>
	void
	_M_insert_range_unique(_InputIterator __first, _InputIterator __last)
	{
	  _Alloc_node __an(*this);
	  for (; __first != __last; ++__first)
	    _M_insert_unique_(end(), *__first, __an);
	}

      template<typename _InputIterator>
	void
	_M_insert_range_equal(_InputIterator __first, _InputIterator __last)
	{
	  _Alloc_node __an(*this);
	  for (; __first != __last; ++__first)
	    _M_insert_equal_(end(), *__first, __an);
	}
#endif

    private:
      void
      _M_erase_aux(const_iterator __position);

      void
      _M_erase_aux(const_iterator __first, const_iterator __last);

    public:
#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 130. Associative erase should return an iterator.
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(const_iterator __position)
      {
	__glibcxx_assert(__position != end());
	const_iterator __result = __position;
	++__result;
	_M_erase_aux(__position);
	return __result._M_const_cast();
      }

      // LWG 2059.
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(iterator __position)
      {
	__glibcxx_assert(__position != end());
	iterator __result = __position;
	++__result;
	_M_erase_aux(__position);
	return __result;
      }
#else
      void
      erase(iterator __position)
      {
	__glibcxx_assert(__position != end());
	_M_erase_aux(__position);
      }

      void
      erase(const_iterator __position)
      {
	__glibcxx_assert(__position != end());
	_M_erase_aux(__position);
      }
#endif

      size_type
      erase(const key_type& __x);

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 130. Associative erase should return an iterator.
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(const_iterator __first, const_iterator __last)
      {
	_M_erase_aux(__first, __last);
	return __last._M_const_cast();
      }
#else
      void
      erase(iterator __first, iterator __last)
      { _M_erase_aux(__first, __last); }

      void
      erase(const_iterator __first, const_iterator __last)
      { _M_erase_aux(__first, __last); }
#endif

      void
      clear() _GLIBCXX_NOEXCEPT
      {
	_M_erase(_M_begin());
	_M_impl._M_reset();
      }

      // Set operations.
      iterator
      find(const key_type& __k);

      const_iterator
      find(const key_type& __k) const;

      size_type
      count(const key_type& __k) const;

      iterator
      lower_bound(const key_type& __k)
      { return _M_lower_bound(_M_begin(), _M_end(), __k); }

      const_iterator
      lower_bound(const key_type& __k) const
      { return _M_lower_bound(_M_begin(), _M_end(), __k); }

      iterator
      upper_bound(const key_type& __k)
      { return _M_upper_bound(_M_begin(), _M_end(), __k); }

      const_iterator
      upper_bound(const key_type& __k) const
      { return _M_upper_bound(_M_begin(), _M_end(), __k); }

      pair<iterator, iterator>
      equal_range(const key_type& __k);

      pair<const_iterator, const_iterator>
      equal_range(const key_type& __k) const;

#if __cplusplus >= 201402L
      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	iterator
	_M_find_tr(const _Kt& __k)
	{
	  const _Rb_tree* __const_this = this;
	  return __const_this->_M_find_tr(__k)._M_const_cast();
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	const_iterator
	_M_find_tr(const _Kt& __k) const
	{
	  auto __j = _M_lower_bound_tr(__k);
	  if (__j != end() && _M_impl._M_key_compare(__k, _S_key(__j._M_node)))
	    __j = end();
	  return __j;
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	size_type
	_M_count_tr(const _Kt& __k) const
	{
	  auto __p = _M_equal_range_tr(__k);
	  return std::distance(__p.first, __p.second);
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	iterator
	_M_lower_bound_tr(const _Kt& __k)
	{
	  const _Rb_tree* __const_this = this;
	  return __const_this->_M_lower_bound_tr(__k)._M_const_cast();
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	const_iterator
	_M_lower_bound_tr(const _Kt& __k) const
	{
	  auto __x = _M_begin();
	  auto __y = _M_end();
	  while (__x != 0)
	    if (!_M_impl._M_key_compare(_S_key(__x), __k))
	      {
		__y = __x;
		__x = _S_left(__x);
	      }
	    else
	      __x = _S_right(__x);
	  return const_iterator(__y);
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	iterator
	_M_upper_bound_tr(const _Kt& __k)
	{
	  const _Rb_tree* __const_this = this;
	  return __const_this->_M_upper_bound_tr(__k)._M_const_cast();
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	const_iterator
	_M_upper_bound_tr(const _Kt& __k) const
	{
	  auto __x = _M_begin();
	  auto __y = _M_end();
	  while (__x != 0)
	    if (_M_impl._M_key_compare(__k, _S_key(__x)))
	      {
		__y = __x;
		__x = _S_left(__x);
	      }
	    else
	      __x = _S_right(__x);
	  return const_iterator(__y);
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	pair<iterator, iterator>
	_M_equal_range_tr(const _Kt& __k)
	{
	  const _Rb_tree* __const_this = this;
	  auto __ret = __const_this->_M_equal_range_tr(__k);
	  return { __ret.first._M_const_cast(), __ret.second._M_const_cast() };
	}

      template<typename _Kt,
	       typename _Req = __has_is_transparent_t<_Compare, _Kt>>
	pair<const_iterator, const_iterator>
	_M_equal_range_tr(const _Kt& __k) const
	{
	  auto __low = _M_lower_bound_tr(__k);
	  auto __high = __low;
	  auto& __cmp = _M_impl._M_key_compare;
	  while (__high != end() && !__cmp(__k, _S_key(__high._M_node)))
	    ++__high;
	  return { __low, __high };
	}
#endif

      // Debugging.
      bool
      __rb_verify() const;

#if __cplusplus >= 201103L
      _Rb_tree&
      operator=(_Rb_tree&&)
      noexcept(_Alloc_traits::_S_nothrow_move()
	       && is_nothrow_move_assignable<_Compare>::value);

      template<typename _Iterator>
	void
	_M_assign_unique(_Iterator, _Iterator);

      template<typename _Iterator>
	void
	_M_assign_equal(_Iterator, _Iterator);

    private:
      // Move elements from container with equal allocator.
      void
      _M_move_data(_Rb_tree& __x, true_type)
      { _M_impl._M_move_data(__x._M_impl); }

      // Move elements from container with possibly non-equal allocator,
      // which might result in a copy not a move.
      void
      _M_move_data(_Rb_tree&, false_type);

      // Move assignment from container with equal allocator.
      void
      _M_move_assign(_Rb_tree&, true_type);

      // Move assignment from container with possibly non-equal allocator,
      // which might result in a copy not a move.
      void
      _M_move_assign(_Rb_tree&, false_type);
#endif

#if __cplusplus > 201402L
    public:
      /// Re-insert an extracted node.
      insert_return_type
      _M_reinsert_node_unique(node_type&& __nh)
      {
	insert_return_type __ret;
	if (__nh.empty())
	  __ret.position = end();
	else
	  {
	    __glibcxx_assert(_M_get_Node_allocator() == *__nh._M_alloc);

	    auto __res = _M_get_insert_unique_pos(__nh._M_key());
	    if (__res.second)
	      {
		__ret.position
		  = _M_insert_node(__res.first, __res.second, __nh._M_ptr);
		__nh._M_ptr = nullptr;
		__ret.inserted = true;
	      }
	    else
	      {
		__ret.node = std::move(__nh);
		__ret.position = iterator(__res.first);
		__ret.inserted = false;
	      }
	  }
	return __ret;
      }

      /// Re-insert an extracted node.
      iterator
      _M_reinsert_node_equal(node_type&& __nh)
      {
	iterator __ret;
	if (__nh.empty())
	  __ret = end();
	else
	  {
	    __glibcxx_assert(_M_get_Node_allocator() == *__nh._M_alloc);
	    auto __res = _M_get_insert_equal_pos(__nh._M_key());
	    if (__res.second)
	      __ret = _M_insert_node(__res.first, __res.second, __nh._M_ptr);
	    else
	      __ret = _M_insert_equal_lower_node(__nh._M_ptr);
	    __nh._M_ptr = nullptr;
	  }
	return __ret;
      }

      /// Re-insert an extracted node.
      iterator
      _M_reinsert_node_hint_unique(const_iterator __hint, node_type&& __nh)
      {
	iterator __ret;
	if (__nh.empty())
	  __ret = end();
	else
	  {
	    __glibcxx_assert(_M_get_Node_allocator() == *__nh._M_alloc);
	    auto __res = _M_get_insert_hint_unique_pos(__hint, __nh._M_key());
	    if (__res.second)
	      {
		__ret = _M_insert_node(__res.first, __res.second, __nh._M_ptr);
		__nh._M_ptr = nullptr;
	      }
	    else
	      __ret = iterator(__res.first);
	  }
	return __ret;
      }

      /// Re-insert an extracted node.
      iterator
      _M_reinsert_node_hint_equal(const_iterator __hint, node_type&& __nh)
      {
	iterator __ret;
	if (__nh.empty())
	  __ret = end();
	else
	  {
	    __glibcxx_assert(_M_get_Node_allocator() == *__nh._M_alloc);
	    auto __res = _M_get_insert_hint_equal_pos(__hint, __nh._M_key());
	    if (__res.second)
	      __ret = _M_insert_node(__res.first, __res.second, __nh._M_ptr);
	    else
	      __ret = _M_insert_equal_lower_node(__nh._M_ptr);
	    __nh._M_ptr = nullptr;
	  }
	return __ret;
      }

      /// Extract a node.
      node_type
      extract(const_iterator __pos)
      {
	auto __ptr = _Rb_tree_rebalance_for_erase(
	    __pos._M_const_cast()._M_node, _M_impl._M_header);
	--_M_impl._M_node_count;
	return { static_cast<_Link_type>(__ptr), _M_get_Node_allocator() };
      }

      /// Extract a node.
      node_type
      extract(const key_type& __k)
      {
	node_type __nh;
	auto __pos = find(__k);
	if (__pos != end())
	  __nh = extract(const_iterator(__pos));
	return __nh;
      }

      template<typename _Compare2>
	using _Compatible_tree
	  = _Rb_tree<_Key, _Val, _KeyOfValue, _Compare2, _Alloc>;

      template<typename, typename>
	friend class _Rb_tree_merge_helper;

      /// Merge from a compatible container into one with unique keys.
      template<typename _Compare2>
	void
	_M_merge_unique(_Compatible_tree<_Compare2>& __src) noexcept
	{
	  using _Merge_helper = _Rb_tree_merge_helper<_Rb_tree, _Compare2>;
	  for (auto __i = __src.begin(), __end = __src.end(); __i != __end;)
	    {
	      auto __pos = __i++;
	      auto __res = _M_get_insert_unique_pos(_KeyOfValue()(*__pos));
	      if (__res.second)
		{
		  auto& __src_impl = _Merge_helper::_S_get_impl(__src);
		  auto __ptr = _Rb_tree_rebalance_for_erase(
		      __pos._M_node, __src_impl._M_header);
		  --__src_impl._M_node_count;
		  _M_insert_node(__res.first, __res.second,
				 static_cast<_Link_type>(__ptr));
		}
	    }
	}

      /// Merge from a compatible container into one with equivalent keys.
      template<typename _Compare2>
	void
	_M_merge_equal(_Compatible_tree<_Compare2>& __src) noexcept
	{
	  using _Merge_helper = _Rb_tree_merge_helper<_Rb_tree, _Compare2>;
	  for (auto __i = __src.begin(), __end = __src.end(); __i != __end;)
	    {
	      auto __pos = __i++;
	      auto __res = _M_get_insert_equal_pos(_KeyOfValue()(*__pos));
	      if (__res.second)
		{
		  auto& __src_impl = _Merge_helper::_S_get_impl(__src);
		  auto __ptr = _Rb_tree_rebalance_for_erase(
		      __pos._M_node, __src_impl._M_header);
		  --__src_impl._M_node_count;
		  _M_insert_node(__res.first, __res.second,
				 static_cast<_Link_type>(__ptr));
		}
	    }
	}
#endif // C++17

      friend bool
      operator==(const _Rb_tree& __x, const _Rb_tree& __y)
      {
	return __x.size() == __y.size()
	  && std::equal(__x.begin(), __x.end(), __y.begin());
      }

#if __cpp_lib_three_way_comparison
      friend auto
      operator<=>(const _Rb_tree& __x, const _Rb_tree& __y)
      {
	if constexpr (requires { typename __detail::__synth3way_t<_Val>; })
	  return std::lexicographical_compare_three_way(__x.begin(), __x.end(),
							__y.begin(), __y.end(),
							__detail::__synth3way);
      }
#else
      friend bool
      operator<(const _Rb_tree& __x, const _Rb_tree& __y)
      {
	return std::lexicographical_compare(__x.begin(), __x.end(),
					    __y.begin(), __y.end());
      }

      friend bool _GLIBCXX_DEPRECATED
      operator!=(const _Rb_tree& __x, const _Rb_tree& __y)
      { return !(__x == __y); }

      friend bool _GLIBCXX_DEPRECATED
      operator>(const _Rb_tree& __x, const _Rb_tree& __y)
      { return __y < __x; }

      friend bool _GLIBCXX_DEPRECATED
      operator<=(const _Rb_tree& __x, const _Rb_tree& __y)
      { return !(__y < __x); }

      friend bool _GLIBCXX_DEPRECATED
      operator>=(const _Rb_tree& __x, const _Rb_tree& __y)
      { return !(__x < __y); }
#endif
    };

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    inline void
    swap(_Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>& __x,
	 _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>& __y)
    { __x.swap(__y); }

#if __cplusplus >= 201103L
  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    void
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_move_data(_Rb_tree& __x, false_type)
    {
      if (_M_get_Node_allocator() == __x._M_get_Node_allocator())
	_M_move_data(__x, true_type());
      else
	{
	  _Alloc_node __an(*this);
	  auto __lbd =
	    [&__an](const value_type& __cval)
	    {
	      auto& __val = const_cast<value_type&>(__cval);
	      return __an(std::move_if_noexcept(__val));
	    };
	  _M_root() = _M_copy(__x, __lbd);
	}
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    inline void
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_move_assign(_Rb_tree& __x, true_type)
    {
      clear();
      if (__x._M_root() != nullptr)
	_M_move_data(__x, true_type());
      std::__alloc_on_move(_M_get_Node_allocator(),
			   __x._M_get_Node_allocator());
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    void
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_move_assign(_Rb_tree& __x, false_type)
    {
      if (_M_get_Node_allocator() == __x._M_get_Node_allocator())
	return _M_move_assign(__x, true_type{});

      // Try to move each node reusing existing nodes and copying __x nodes
      // structure.
      _Reuse_or_alloc_node __roan(*this);
      _M_impl._M_reset();
      if (__x._M_root() != nullptr)
	{
	  auto __lbd =
	    [&__roan](const value_type& __cval)
	    {
	      auto& __val = const_cast<value_type&>(__cval);
	      return __roan(std::move(__val));
	    };
	  _M_root() = _M_copy(__x, __lbd);
	  __x.clear();
	}
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    inline _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>&
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    operator=(_Rb_tree&& __x)
    noexcept(_Alloc_traits::_S_nothrow_move()
	     && is_nothrow_move_assignable<_Compare>::value)
    {
      _M_impl._M_key_compare = std::move(__x._M_impl._M_key_compare);
      _M_move_assign(__x, __bool_constant<_Alloc_traits::_S_nothrow_move()>());
      return *this;
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    template<typename _Iterator>
      void
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_assign_unique(_Iterator __first, _Iterator __last)
      {
	_Reuse_or_alloc_node __roan(*this);
	_M_impl._M_reset();
	for (; __first != __last; ++__first)
	  _M_insert_unique_(end(), *__first, __roan);
      }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    template<typename _Iterator>
      void
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_assign_equal(_Iterator __first, _Iterator __last)
      {
	_Reuse_or_alloc_node __roan(*this);
	_M_impl._M_reset();
	for (; __first != __last; ++__first)
	  _M_insert_equal_(end(), *__first, __roan);
      }
#endif

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>&
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    operator=(const _Rb_tree& __x)
    {
      if (this != &__x)
	{
	  // Note that _Key may be a constant type.
#if __cplusplus >= 201103L
	  if (_Alloc_traits::_S_propagate_on_copy_assign())
	    {
	      auto& __this_alloc = this->_M_get_Node_allocator();
	      auto& __that_alloc = __x._M_get_Node_allocator();
	      if (!_Alloc_traits::_S_always_equal()
		  && __this_alloc != __that_alloc)
		{
		  // Replacement allocator cannot free existing storage, we need
		  // to erase nodes first.
		  clear();
		  std::__alloc_on_copy(__this_alloc, __that_alloc);
		}
	    }
#endif

	  _Reuse_or_alloc_node __roan(*this);
	  _M_impl._M_reset();
	  _M_impl._M_key_compare = __x._M_impl._M_key_compare;
	  if (__x._M_root() != 0)
	    _M_root() = _M_copy(__x, __roan);
	}

      return *this;
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
#if __cplusplus >= 201103L
    template<typename _Arg, typename _NodeGen>
#else
    template<typename _NodeGen>
#endif
      typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_insert_(_Base_ptr __x, _Base_ptr __p,
#if __cplusplus >= 201103L
		 _Arg&& __v,
#else
		 const _Val& __v,
#endif
		 _NodeGen& __node_gen)
      {
	bool __insert_left = (__x != 0 || __p == _M_end()
			      || _M_impl._M_key_compare(_KeyOfValue()(__v),
							_S_key(__p)));

	_Link_type __z = __node_gen(_GLIBCXX_FORWARD(_Arg, __v));

	_Rb_tree_insert_and_rebalance(__insert_left, __z, __p,
				      this->_M_impl._M_header);
	++_M_impl._M_node_count;
	return iterator(__z);
      }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
#if __cplusplus >= 201103L
    template<typename _Arg>
#endif
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
#if __cplusplus >= 201103L
    _M_insert_lower(_Base_ptr __p, _Arg&& __v)
#else
    _M_insert_lower(_Base_ptr __p, const _Val& __v)
#endif
    {
      bool __insert_left = (__p == _M_end()
			    || !_M_impl._M_key_compare(_S_key(__p),
						       _KeyOfValue()(__v)));

      _Link_type __z = _M_create_node(_GLIBCXX_FORWARD(_Arg, __v));

      _Rb_tree_insert_and_rebalance(__insert_left, __z, __p,
				    this->_M_impl._M_header);
      ++_M_impl._M_node_count;
      return iterator(__z);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
#if __cplusplus >= 201103L
    template<typename _Arg>
#endif
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
#if __cplusplus >= 201103L
    _M_insert_equal_lower(_Arg&& __v)
#else
    _M_insert_equal_lower(const _Val& __v)
#endif
    {
      _Link_type __x = _M_begin();
      _Base_ptr __y = _M_end();
      while (__x != 0)
	{
	  __y = __x;
	  __x = !_M_impl._M_key_compare(_S_key(__x), _KeyOfValue()(__v)) ?
		_S_left(__x) : _S_right(__x);
	}
      return _M_insert_lower(__y, _GLIBCXX_FORWARD(_Arg, __v));
    }

  template<typename _Key, typename _Val, typename _KoV,
	   typename _Compare, typename _Alloc>
    template<typename _NodeGen>
      typename _Rb_tree<_Key, _Val, _KoV, _Compare, _Alloc>::_Link_type
      _Rb_tree<_Key, _Val, _KoV, _Compare, _Alloc>::
      _M_copy(_Const_Link_type __x, _Base_ptr __p, _NodeGen& __node_gen)
      {
	// Structural copy. __x and __p must be non-null.
	_Link_type __top = _M_clone_node(__x, __node_gen);
	__top->_M_parent = __p;

	__try
	  {
	    if (__x->_M_right)
	      __top->_M_right = _M_copy(_S_right(__x), __top, __node_gen);
	    __p = __top;
	    __x = _S_left(__x);

	    while (__x != 0)
	      {
		_Link_type __y = _M_clone_node(__x, __node_gen);
		__p->_M_left = __y;
		__y->_M_parent = __p;
		if (__x->_M_right)
		  __y->_M_right = _M_copy(_S_right(__x), __y, __node_gen);
		__p = __y;
		__x = _S_left(__x);
	      }
	  }
	__catch(...)
	  {
	    _M_erase(__top);
	    __throw_exception_again;
	  }
	return __top;
      }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    void
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_erase(_Link_type __x)
    {
      // Erase without rebalancing.
      while (__x != 0)
	{
	  _M_erase(_S_right(__x));
	  _Link_type __y = _S_left(__x);
	  _M_drop_node(__x);
	  __x = __y;
	}
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue,
		      _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_lower_bound(_Link_type __x, _Base_ptr __y,
		   const _Key& __k)
    {
      while (__x != 0)
	if (!_M_impl._M_key_compare(_S_key(__x), __k))
	  __y = __x, __x = _S_left(__x);
	else
	  __x = _S_right(__x);
      return iterator(__y);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue,
		      _Compare, _Alloc>::const_iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_lower_bound(_Const_Link_type __x, _Const_Base_ptr __y,
		   const _Key& __k) const
    {
      while (__x != 0)
	if (!_M_impl._M_key_compare(_S_key(__x), __k))
	  __y = __x, __x = _S_left(__x);
	else
	  __x = _S_right(__x);
      return const_iterator(__y);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue,
		      _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_upper_bound(_Link_type __x, _Base_ptr __y,
		   const _Key& __k)
    {
      while (__x != 0)
	if (_M_impl._M_key_compare(__k, _S_key(__x)))
	  __y = __x, __x = _S_left(__x);
	else
	  __x = _S_right(__x);
      return iterator(__y);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue,
		      _Compare, _Alloc>::const_iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_upper_bound(_Const_Link_type __x, _Const_Base_ptr __y,
		   const _Key& __k) const
    {
      while (__x != 0)
	if (_M_impl._M_key_compare(__k, _S_key(__x)))
	  __y = __x, __x = _S_left(__x);
	else
	  __x = _S_right(__x);
      return const_iterator(__y);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::iterator,
	 typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::iterator>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    equal_range(const _Key& __k)
    {
      _Link_type __x = _M_begin();
      _Base_ptr __y = _M_end();
      while (__x != 0)
	{
	  if (_M_impl._M_key_compare(_S_key(__x), __k))
	    __x = _S_right(__x);
	  else if (_M_impl._M_key_compare(__k, _S_key(__x)))
	    __y = __x, __x = _S_left(__x);
	  else
	    {
	      _Link_type __xu(__x);
	      _Base_ptr __yu(__y);
	      __y = __x, __x = _S_left(__x);
	      __xu = _S_right(__xu);
	      return pair<iterator,
			  iterator>(_M_lower_bound(__x, __y, __k),
				    _M_upper_bound(__xu, __yu, __k));
	    }
	}
      return pair<iterator, iterator>(iterator(__y),
				      iterator(__y));
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::const_iterator,
	 typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::const_iterator>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    equal_range(const _Key& __k) const
    {
      _Const_Link_type __x = _M_begin();
      _Const_Base_ptr __y = _M_end();
      while (__x != 0)
	{
	  if (_M_impl._M_key_compare(_S_key(__x), __k))
	    __x = _S_right(__x);
	  else if (_M_impl._M_key_compare(__k, _S_key(__x)))
	    __y = __x, __x = _S_left(__x);
	  else
	    {
	      _Const_Link_type __xu(__x);
	      _Const_Base_ptr __yu(__y);
	      __y = __x, __x = _S_left(__x);
	      __xu = _S_right(__xu);
	      return pair<const_iterator,
			  const_iterator>(_M_lower_bound(__x, __y, __k),
					  _M_upper_bound(__xu, __yu, __k));
	    }
	}
      return pair<const_iterator, const_iterator>(const_iterator(__y),
						  const_iterator(__y));
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    void
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    swap(_Rb_tree& __t)
    _GLIBCXX_NOEXCEPT_IF(__is_nothrow_swappable<_Compare>::value)
    {
      if (_M_root() == 0)
	{
	  if (__t._M_root() != 0)
	    _M_impl._M_move_data(__t._M_impl);
	}
      else if (__t._M_root() == 0)
	__t._M_impl._M_move_data(_M_impl);
      else
	{
	  std::swap(_M_root(),__t._M_root());
	  std::swap(_M_leftmost(),__t._M_leftmost());
	  std::swap(_M_rightmost(),__t._M_rightmost());

	  _M_root()->_M_parent = _M_end();
	  __t._M_root()->_M_parent = __t._M_end();
	  std::swap(this->_M_impl._M_node_count, __t._M_impl._M_node_count);
	}
      // No need to swap header's color as it does not change.
      std::swap(this->_M_impl._M_key_compare, __t._M_impl._M_key_compare);

      _Alloc_traits::_S_on_swap(_M_get_Node_allocator(),
				__t._M_get_Node_allocator());
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr,
	 typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_get_insert_unique_pos(const key_type& __k)
    {
      typedef pair<_Base_ptr, _Base_ptr> _Res;
      _Link_type __x = _M_begin();
      _Base_ptr __y = _M_end();
      bool __comp = true;
      while (__x != 0)
	{
	  __y = __x;
	  __comp = _M_impl._M_key_compare(__k, _S_key(__x));
	  __x = __comp ? _S_left(__x) : _S_right(__x);
	}
      iterator __j = iterator(__y);
      if (__comp)
	{
	  if (__j == begin())
	    return _Res(__x, __y);
	  else
	    --__j;
	}
      if (_M_impl._M_key_compare(_S_key(__j._M_node), __k))
	return _Res(__x, __y);
      return _Res(__j._M_node, 0);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr,
	 typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_get_insert_equal_pos(const key_type& __k)
    {
      typedef pair<_Base_ptr, _Base_ptr> _Res;
      _Link_type __x = _M_begin();
      _Base_ptr __y = _M_end();
      while (__x != 0)
	{
	  __y = __x;
	  __x = _M_impl._M_key_compare(__k, _S_key(__x)) ?
		_S_left(__x) : _S_right(__x);
	}
      return _Res(__x, __y);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
#if __cplusplus >= 201103L
    template<typename _Arg>
#endif
    pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::iterator, bool>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
#if __cplusplus >= 201103L
    _M_insert_unique(_Arg&& __v)
#else
    _M_insert_unique(const _Val& __v)
#endif
    {
      typedef pair<iterator, bool> _Res;
      pair<_Base_ptr, _Base_ptr> __res
	= _M_get_insert_unique_pos(_KeyOfValue()(__v));

      if (__res.second)
	{
	  _Alloc_node __an(*this);
	  return _Res(_M_insert_(__res.first, __res.second,
				 _GLIBCXX_FORWARD(_Arg, __v), __an),
		      true);
	}

      return _Res(iterator(__res.first), false);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
#if __cplusplus >= 201103L
    template<typename _Arg>
#endif
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
#if __cplusplus >= 201103L
    _M_insert_equal(_Arg&& __v)
#else
    _M_insert_equal(const _Val& __v)
#endif
    {
      pair<_Base_ptr, _Base_ptr> __res
	= _M_get_insert_equal_pos(_KeyOfValue()(__v));
      _Alloc_node __an(*this);
      return _M_insert_(__res.first, __res.second,
			_GLIBCXX_FORWARD(_Arg, __v), __an);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr,
	 typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_get_insert_hint_unique_pos(const_iterator __position,
				  const key_type& __k)
    {
      iterator __pos = __position._M_const_cast();
      typedef pair<_Base_ptr, _Base_ptr> _Res;

      // end()
      if (__pos._M_node == _M_end())
	{
	  if (size() > 0
	      && _M_impl._M_key_compare(_S_key(_M_rightmost()), __k))
	    return _Res(0, _M_rightmost());
	  else
	    return _M_get_insert_unique_pos(__k);
	}
      else if (_M_impl._M_key_compare(__k, _S_key(__pos._M_node)))
	{
	  // First, try before...
	  iterator __before = __pos;
	  if (__pos._M_node == _M_leftmost()) // begin()
	    return _Res(_M_leftmost(), _M_leftmost());
	  else if (_M_impl._M_key_compare(_S_key((--__before)._M_node), __k))
	    {
	      if (_S_right(__before._M_node) == 0)
		return _Res(0, __before._M_node);
	      else
		return _Res(__pos._M_node, __pos._M_node);
	    }
	  else
	    return _M_get_insert_unique_pos(__k);
	}
      else if (_M_impl._M_key_compare(_S_key(__pos._M_node), __k))
	{
	  // ... then try after.
	  iterator __after = __pos;
	  if (__pos._M_node == _M_rightmost())
	    return _Res(0, _M_rightmost());
	  else if (_M_impl._M_key_compare(__k, _S_key((++__after)._M_node)))
	    {
	      if (_S_right(__pos._M_node) == 0)
		return _Res(0, __pos._M_node);
	      else
		return _Res(__after._M_node, __after._M_node);
	    }
	  else
	    return _M_get_insert_unique_pos(__k);
	}
      else
	// Equivalent keys.
	return _Res(__pos._M_node, 0);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
#if __cplusplus >= 201103L
    template<typename _Arg, typename _NodeGen>
#else
    template<typename _NodeGen>
#endif
      typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_insert_unique_(const_iterator __position,
#if __cplusplus >= 201103L
			_Arg&& __v,
#else
			const _Val& __v,
#endif
			_NodeGen& __node_gen)
    {
      pair<_Base_ptr, _Base_ptr> __res
	= _M_get_insert_hint_unique_pos(__position, _KeyOfValue()(__v));

      if (__res.second)
	return _M_insert_(__res.first, __res.second,
			  _GLIBCXX_FORWARD(_Arg, __v),
			  __node_gen);
      return iterator(__res.first);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr,
	 typename _Rb_tree<_Key, _Val, _KeyOfValue,
			   _Compare, _Alloc>::_Base_ptr>
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_get_insert_hint_equal_pos(const_iterator __position, const key_type& __k)
    {
      iterator __pos = __position._M_const_cast();
      typedef pair<_Base_ptr, _Base_ptr> _Res;

      // end()
      if (__pos._M_node == _M_end())
	{
	  if (size() > 0
	      && !_M_impl._M_key_compare(__k, _S_key(_M_rightmost())))
	    return _Res(0, _M_rightmost());
	  else
	    return _M_get_insert_equal_pos(__k);
	}
      else if (!_M_impl._M_key_compare(_S_key(__pos._M_node), __k))
	{
	  // First, try before...
	  iterator __before = __pos;
	  if (__pos._M_node == _M_leftmost()) // begin()
	    return _Res(_M_leftmost(), _M_leftmost());
	  else if (!_M_impl._M_key_compare(__k, _S_key((--__before)._M_node)))
	    {
	      if (_S_right(__before._M_node) == 0)
		return _Res(0, __before._M_node);
	      else
		return _Res(__pos._M_node, __pos._M_node);
	    }
	  else
	    return _M_get_insert_equal_pos(__k);
	}
      else
	{
	  // ... then try after.
	  iterator __after = __pos;
	  if (__pos._M_node == _M_rightmost())
	    return _Res(0, _M_rightmost());
	  else if (!_M_impl._M_key_compare(_S_key((++__after)._M_node), __k))
	    {
	      if (_S_right(__pos._M_node) == 0)
		return _Res(0, __pos._M_node);
	      else
		return _Res(__after._M_node, __after._M_node);
	    }
	  else
	    return _Res(0, 0);
	}
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
#if __cplusplus >= 201103L
    template<typename _Arg, typename _NodeGen>
#else
    template<typename _NodeGen>
#endif
      typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_insert_equal_(const_iterator __position,
#if __cplusplus >= 201103L
		       _Arg&& __v,
#else
		       const _Val& __v,
#endif
		       _NodeGen& __node_gen)
      {
	pair<_Base_ptr, _Base_ptr> __res
	  = _M_get_insert_hint_equal_pos(__position, _KeyOfValue()(__v));

	if (__res.second)
	  return _M_insert_(__res.first, __res.second,
			    _GLIBCXX_FORWARD(_Arg, __v),
			    __node_gen);

	return _M_insert_equal_lower(_GLIBCXX_FORWARD(_Arg, __v));
      }

#if __cplusplus >= 201103L
  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_insert_node(_Base_ptr __x, _Base_ptr __p, _Link_type __z)
    {
      bool __insert_left = (__x != 0 || __p == _M_end()
			    || _M_impl._M_key_compare(_S_key(__z),
						      _S_key(__p)));

      _Rb_tree_insert_and_rebalance(__insert_left, __z, __p,
				    this->_M_impl._M_header);
      ++_M_impl._M_node_count;
      return iterator(__z);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_insert_lower_node(_Base_ptr __p, _Link_type __z)
    {
      bool __insert_left = (__p == _M_end()
			    || !_M_impl._M_key_compare(_S_key(__p),
						       _S_key(__z)));

      _Rb_tree_insert_and_rebalance(__insert_left, __z, __p,
				    this->_M_impl._M_header);
      ++_M_impl._M_node_count;
      return iterator(__z);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_insert_equal_lower_node(_Link_type __z)
    {
      _Link_type __x = _M_begin();
      _Base_ptr __y = _M_end();
      while (__x != 0)
	{
	  __y = __x;
	  __x = !_M_impl._M_key_compare(_S_key(__x), _S_key(__z)) ?
		_S_left(__x) : _S_right(__x);
	}
      return _M_insert_lower_node(__y, __z);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    template<typename... _Args>
      pair<typename _Rb_tree<_Key, _Val, _KeyOfValue,
			     _Compare, _Alloc>::iterator, bool>
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_emplace_unique(_Args&&... __args)
      {
	_Link_type __z = _M_create_node(std::forward<_Args>(__args)...);

	__try
	  {
	    typedef pair<iterator, bool> _Res;
	    auto __res = _M_get_insert_unique_pos(_S_key(__z));
	    if (__res.second)
	      return _Res(_M_insert_node(__res.first, __res.second, __z), true);
	
	    _M_drop_node(__z);
	    return _Res(iterator(__res.first), false);
	  }
	__catch(...)
	  {
	    _M_drop_node(__z);
	    __throw_exception_again;
	  }
      }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    template<typename... _Args>
      typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_emplace_equal(_Args&&... __args)
      {
	_Link_type __z = _M_create_node(std::forward<_Args>(__args)...);

	__try
	  {
	    auto __res = _M_get_insert_equal_pos(_S_key(__z));
	    return _M_insert_node(__res.first, __res.second, __z);
	  }
	__catch(...)
	  {
	    _M_drop_node(__z);
	    __throw_exception_again;
	  }
      }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    template<typename... _Args>
      typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_emplace_hint_unique(const_iterator __pos, _Args&&... __args)
      {
	_Link_type __z = _M_create_node(std::forward<_Args>(__args)...);

	__try
	  {
	    auto __res = _M_get_insert_hint_unique_pos(__pos, _S_key(__z));

	    if (__res.second)
	      return _M_insert_node(__res.first, __res.second, __z);

	    _M_drop_node(__z);
	    return iterator(__res.first);
	  }
	__catch(...)
	  {
	    _M_drop_node(__z);
	    __throw_exception_again;
	  }
      }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    template<typename... _Args>
      typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::iterator
      _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
      _M_emplace_hint_equal(const_iterator __pos, _Args&&... __args)
      {
	_Link_type __z = _M_create_node(std::forward<_Args>(__args)...);

	__try
	  {
	    auto __res = _M_get_insert_hint_equal_pos(__pos, _S_key(__z));

	    if (__res.second)
	      return _M_insert_node(__res.first, __res.second, __z);

	    return _M_insert_equal_lower_node(__z);
	  }
	__catch(...)
	  {
	    _M_drop_node(__z);
	    __throw_exception_again;
	  }
      }
#endif


  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    void
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_erase_aux(const_iterator __position)
    {
      _Link_type __y =
	static_cast<_Link_type>(_Rb_tree_rebalance_for_erase
				(const_cast<_Base_ptr>(__position._M_node),
				 this->_M_impl._M_header));
      _M_drop_node(__y);
      --_M_impl._M_node_count;
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    void
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    _M_erase_aux(const_iterator __first, const_iterator __last)
    {
      if (__first == begin() && __last == end())
	clear();
      else
	while (__first != __last)
	  _M_erase_aux(__first++);
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::size_type
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    erase(const _Key& __x)
    {
      pair<iterator, iterator> __p = equal_range(__x);
      const size_type __old_size = size();
      _M_erase_aux(__p.first, __p.second);
      return __old_size - size();
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue,
		      _Compare, _Alloc>::iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    find(const _Key& __k)
    {
      iterator __j = _M_lower_bound(_M_begin(), _M_end(), __k);
      return (__j == end()
	      || _M_impl._M_key_compare(__k,
					_S_key(__j._M_node))) ? end() : __j;
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue,
		      _Compare, _Alloc>::const_iterator
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    find(const _Key& __k) const
    {
      const_iterator __j = _M_lower_bound(_M_begin(), _M_end(), __k);
      return (__j == end()
	      || _M_impl._M_key_compare(__k,
					_S_key(__j._M_node))) ? end() : __j;
    }

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    typename _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::size_type
    _Rb_tree<_Key, _Val, _KeyOfValue, _Compare, _Alloc>::
    count(const _Key& __k) const
    {
      pair<const_iterator, const_iterator> __p = equal_range(__k);
      const size_type __n = std::distance(__p.first, __p.second);
      return __n;
    }

  _GLIBCXX_PURE unsigned int
  _Rb_tree_black_count(const _Rb_tree_node_base* __node,
		       const _Rb_tree_node_base* __root) throw ();

  template<typename _Key, typename _Val, typename _KeyOfValue,
	   typename _Compare, typename _Alloc>
    bool
    _Rb_tree<_Key,_Val,_KeyOfValue,_Compare,_Alloc>::__rb_verify() const
    {
      if (_M_impl._M_node_count == 0 || begin() == end())
	return _M_impl._M_node_count == 0 && begin() == end()
	       && this->_M_impl._M_header._M_left == _M_end()
	       && this->_M_impl._M_header._M_right == _M_end();

      unsigned int __len = _Rb_tree_black_count(_M_leftmost(), _M_root());
      for (const_iterator __it = begin(); __it != end(); ++__it)
	{
	  _Const_Link_type __x = static_cast<_Const_Link_type>(__it._M_node);
	  _Const_Link_type __L = _S_left(__x);
	  _Const_Link_type __R = _S_right(__x);

	  if (__x->_M_color == _S_red)
	    if ((__L && __L->_M_color == _S_red)
		|| (__R && __R->_M_color == _S_red))
	      return false;

	  if (__L && _M_impl._M_key_compare(_S_key(__x), _S_key(__L)))
	    return false;
	  if (__R && _M_impl._M_key_compare(_S_key(__R), _S_key(__x)))
	    return false;

	  if (!__L && !__R && _Rb_tree_black_count(__x, _M_root()) != __len)
	    return false;
	}

      if (_M_leftmost() != _Rb_tree_node_base::_S_minimum(_M_root()))
	return false;
      if (_M_rightmost() != _Rb_tree_node_base::_S_maximum(_M_root()))
	return false;
      return true;
    }

#if __cplusplus > 201402L
  // Allow access to internals of compatible _Rb_tree specializations.
  template<typename _Key, typename _Val, typename _Sel, typename _Cmp1,
	   typename _Alloc, typename _Cmp2>
    struct _Rb_tree_merge_helper<_Rb_tree<_Key, _Val, _Sel, _Cmp1, _Alloc>,
				 _Cmp2>
    {
    private:
      friend class _Rb_tree<_Key, _Val, _Sel, _Cmp1, _Alloc>;

      static auto&
      _S_get_impl(_Rb_tree<_Key, _Val, _Sel, _Cmp2, _Alloc>& __tree)
      { return __tree._M_impl; }
    };
#endif // C++17

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
