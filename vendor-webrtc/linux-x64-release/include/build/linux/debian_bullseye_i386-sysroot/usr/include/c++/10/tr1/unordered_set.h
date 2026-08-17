// TR1 unordered_set implementation -*- C++ -*-

// Copyright (C) 2010-2020 Free Software Foundation, Inc.
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

/** @file tr1/unordered_set.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{tr1/unordered_set}
 */

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace tr1
{
  // NB: When we get typedef templates these class definitions
  // will be unnecessary.
  template<class _Value,
	   class _Hash = hash<_Value>,
	   class _Pred = std::equal_to<_Value>,
	   class _Alloc = std::allocator<_Value>,
	   bool __cache_hash_code = false>
    class __unordered_set
    : public _Hashtable<_Value, _Value, _Alloc,
			std::_Identity<_Value>, _Pred,
			_Hash, __detail::_Mod_range_hashing,
			__detail::_Default_ranged_hash,
			__detail::_Prime_rehash_policy,
			__cache_hash_code, true, true>
    {
      typedef _Hashtable<_Value, _Value, _Alloc,
			 std::_Identity<_Value>, _Pred,
			 _Hash, __detail::_Mod_range_hashing,
			 __detail::_Default_ranged_hash,
			 __detail::_Prime_rehash_policy,
			 __cache_hash_code, true, true>
	_Base;

    public:
      typedef typename _Base::size_type       size_type;
      typedef typename _Base::hasher          hasher;
      typedef typename _Base::key_equal       key_equal;
      typedef typename _Base::allocator_type  allocator_type;

      explicit
      __unordered_set(size_type __n = 10,
		      const hasher& __hf = hasher(),
		      const key_equal& __eql = key_equal(),
		      const allocator_type& __a = allocator_type())
      : _Base(__n, __hf, __detail::_Mod_range_hashing(),
	      __detail::_Default_ranged_hash(), __eql,
	      std::_Identity<_Value>(), __a)
      { }

      template<typename _InputIterator>
	__unordered_set(_InputIterator __f, _InputIterator __l,
			size_type __n = 10,
			const hasher& __hf = hasher(),
			const key_equal& __eql = key_equal(),
			const allocator_type& __a = allocator_type())
	: _Base(__f, __l, __n, __hf, __detail::_Mod_range_hashing(),
		__detail::_Default_ranged_hash(), __eql,
		std::_Identity<_Value>(), __a)
	{ }
    };

  template<class _Value,
	   class _Hash = hash<_Value>,
	   class _Pred = std::equal_to<_Value>,
	   class _Alloc = std::allocator<_Value>,
	   bool __cache_hash_code = false>
    class __unordered_multiset
    : public _Hashtable<_Value, _Value, _Alloc,
			std::_Identity<_Value>, _Pred,
			_Hash, __detail::_Mod_range_hashing,
			__detail::_Default_ranged_hash,
			__detail::_Prime_rehash_policy,
			__cache_hash_code, true, false>
    {
      typedef _Hashtable<_Value, _Value, _Alloc,
			 std::_Identity<_Value>, _Pred,
			 _Hash, __detail::_Mod_range_hashing,
			 __detail::_Default_ranged_hash,
			 __detail::_Prime_rehash_policy,
			 __cache_hash_code, true, false>
	_Base;

    public:
      typedef typename _Base::size_type       size_type;
      typedef typename _Base::hasher          hasher;
      typedef typename _Base::key_equal       key_equal;
      typedef typename _Base::allocator_type  allocator_type;

      explicit
      __unordered_multiset(size_type __n = 10,
			   const hasher& __hf = hasher(),
			   const key_equal& __eql = key_equal(),
			   const allocator_type& __a = allocator_type())
      : _Base(__n, __hf, __detail::_Mod_range_hashing(),
	      __detail::_Default_ranged_hash(), __eql,
	      std::_Identity<_Value>(), __a)
      { }


      template<typename _InputIterator>
	__unordered_multiset(_InputIterator __f, _InputIterator __l,
			     typename _Base::size_type __n = 0,
			     const hasher& __hf = hasher(),
			     const key_equal& __eql = key_equal(),
			     const allocator_type& __a = allocator_type())
	: _Base(__f, __l, __n, __hf, __detail::_Mod_range_hashing(),
		__detail::_Default_ranged_hash(), __eql,
		std::_Identity<_Value>(), __a)
	{ }
    };

  template<class _Value, class _Hash, class _Pred, class _Alloc,
	   bool __cache_hash_code>
    inline void
    swap(__unordered_set<_Value, _Hash, _Pred, _Alloc, __cache_hash_code>& __x,
	 __unordered_set<_Value, _Hash, _Pred, _Alloc, __cache_hash_code>& __y)
    { __x.swap(__y); }

  template<class _Value, class _Hash, class _Pred, class _Alloc,
	   bool __cache_hash_code>
    inline void
    swap(__unordered_multiset<_Value, _Hash, _Pred,
	 _Alloc, __cache_hash_code>& __x,
	 __unordered_multiset<_Value, _Hash, _Pred,
	 _Alloc, __cache_hash_code>& __y)
    { __x.swap(__y); }


  /**
   *  @brief A standard container composed of unique keys (containing
   *  at most one of each key value) in which the elements' keys are
   *  the elements themselves.
   *
   *  @ingroup unordered_associative_containers
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, and
   *  <a href="tables.html#xx">unordered associative container</a>
   *
   *  @param  Value  Type of key objects.
   *  @param  Hash  Hashing function object type, defaults to hash<Value>.
   *  @param  Pred  Predicate function object type, defaults to equal_to<Value>.
   *  @param  Alloc  Allocator type, defaults to allocator<Key>.
   */
  template<class _Value,
	   class _Hash = hash<_Value>,
	   class _Pred = std::equal_to<_Value>,
	   class _Alloc = std::allocator<_Value> >
    class unordered_set
    : public __unordered_set<_Value, _Hash, _Pred, _Alloc>
    {
      typedef __unordered_set<_Value, _Hash, _Pred, _Alloc>  _Base;

    public:
      typedef typename _Base::value_type      value_type;
      typedef typename _Base::size_type       size_type;
      typedef typename _Base::hasher          hasher;
      typedef typename _Base::key_equal       key_equal;
      typedef typename _Base::allocator_type  allocator_type;

      explicit
      unordered_set(size_type __n = 10,
		    const hasher& __hf = hasher(),
		    const key_equal& __eql = key_equal(),
		    const allocator_type& __a = allocator_type())
      : _Base(__n, __hf, __eql, __a)
      { }

      template<typename _InputIterator>
	unordered_set(_InputIterator __f, _InputIterator __l,
		      size_type __n = 10,
		      const hasher& __hf = hasher(),
		      const key_equal& __eql = key_equal(),
		      const allocator_type& __a = allocator_type())
	: _Base(__f, __l, __n, __hf, __eql, __a)
	{ }
    };

  /**
   *  @brief A standard container composed of equivalent keys
   *  (possibly containing multiple of each key value) in which the
   *  elements' keys are the elements themselves.
   *
   *  @ingroup unordered_associative_containers
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, and
   *  <a href="tables.html#xx">unordered associative container</a>
   *
   *  @param  Value  Type of key objects.
   *  @param  Hash  Hashing function object type, defaults to hash<Value>.
   *  @param  Pred  Predicate function object type, defaults to equal_to<Value>.
   *  @param  Alloc  Allocator type, defaults to allocator<Key>.
   */
  template<class _Value,
	   class _Hash = hash<_Value>,
	   class _Pred = std::equal_to<_Value>,
	   class _Alloc = std::allocator<_Value> >
    class unordered_multiset
    : public __unordered_multiset<_Value, _Hash, _Pred, _Alloc>
    {
      typedef __unordered_multiset<_Value, _Hash, _Pred, _Alloc>  _Base;

    public:
      typedef typename _Base::value_type      value_type;
      typedef typename _Base::size_type       size_type;
      typedef typename _Base::hasher          hasher;
      typedef typename _Base::key_equal       key_equal;
      typedef typename _Base::allocator_type  allocator_type;

      explicit
      unordered_multiset(size_type __n = 10,
			 const hasher& __hf = hasher(),
			 const key_equal& __eql = key_equal(),
			 const allocator_type& __a = allocator_type())
      : _Base(__n, __hf, __eql, __a)
      { }


      template<typename _InputIterator>
	unordered_multiset(_InputIterator __f, _InputIterator __l,
			   typename _Base::size_type __n = 0,
			   const hasher& __hf = hasher(),
			   const key_equal& __eql = key_equal(),
			   const allocator_type& __a = allocator_type())
	: _Base(__f, __l, __n, __hf, __eql, __a)
	{ }
    };

  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline void
    swap(unordered_set<_Value, _Hash, _Pred, _Alloc>& __x,
	 unordered_set<_Value, _Hash, _Pred, _Alloc>& __y)
    { __x.swap(__y); }

  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline void
    swap(unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __x,
	 unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __y)
    { __x.swap(__y); }
}

_GLIBCXX_END_NAMESPACE_VERSION
}
