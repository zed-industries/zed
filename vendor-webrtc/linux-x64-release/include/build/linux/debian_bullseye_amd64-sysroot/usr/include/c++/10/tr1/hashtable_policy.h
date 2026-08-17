// Internal policy header for TR1 unordered_set and unordered_map -*- C++ -*-

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

/** @file tr1/hashtable_policy.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. 
 *  @headername{tr1/unordered_map, tr1/unordered_set}
 */

namespace std _GLIBCXX_VISIBILITY(default)
{ 
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace tr1
{
namespace __detail
{
  // Helper function: return distance(first, last) for forward
  // iterators, or 0 for input iterators.
  template<class _Iterator>
    inline typename std::iterator_traits<_Iterator>::difference_type
    __distance_fw(_Iterator __first, _Iterator __last,
		  std::input_iterator_tag)
    { return 0; }

  template<class _Iterator>
    inline typename std::iterator_traits<_Iterator>::difference_type
    __distance_fw(_Iterator __first, _Iterator __last,
		  std::forward_iterator_tag)
    { return std::distance(__first, __last); }

  template<class _Iterator>
    inline typename std::iterator_traits<_Iterator>::difference_type
    __distance_fw(_Iterator __first, _Iterator __last)
    {
      typedef typename std::iterator_traits<_Iterator>::iterator_category _Tag;
      return __distance_fw(__first, __last, _Tag());
    }

  // Auxiliary types used for all instantiations of _Hashtable: nodes
  // and iterators.
  
  // Nodes, used to wrap elements stored in the hash table.  A policy
  // template parameter of class template _Hashtable controls whether
  // nodes also store a hash code. In some cases (e.g. strings) this
  // may be a performance win.
  template<typename _Value, bool __cache_hash_code>
    struct _Hash_node;

  template<typename _Value>
    struct _Hash_node<_Value, true>
    {
      _Value       _M_v;
      std::size_t  _M_hash_code;
      _Hash_node*  _M_next;
    };

  template<typename _Value>
    struct _Hash_node<_Value, false>
    {
      _Value       _M_v;
      _Hash_node*  _M_next;
    };

  // Local iterators, used to iterate within a bucket but not between
  // buckets.
  template<typename _Value, bool __cache>
    struct _Node_iterator_base
    {
      _Node_iterator_base(_Hash_node<_Value, __cache>* __p)
      : _M_cur(__p) { }
      
      void
      _M_incr()
      { _M_cur = _M_cur->_M_next; }

      _Hash_node<_Value, __cache>*  _M_cur;
    };

  template<typename _Value, bool __cache>
    inline bool
    operator==(const _Node_iterator_base<_Value, __cache>& __x,
	       const _Node_iterator_base<_Value, __cache>& __y)
    { return __x._M_cur == __y._M_cur; }

  template<typename _Value, bool __cache>
    inline bool
    operator!=(const _Node_iterator_base<_Value, __cache>& __x,
	       const _Node_iterator_base<_Value, __cache>& __y)
    { return __x._M_cur != __y._M_cur; }

  template<typename _Value, bool __constant_iterators, bool __cache>
    struct _Node_iterator
    : public _Node_iterator_base<_Value, __cache>
    {
      typedef _Value                                   value_type;
      typedef typename
      __gnu_cxx::__conditional_type<__constant_iterators,
				    const _Value*, _Value*>::__type
                                                       pointer;
      typedef typename
      __gnu_cxx::__conditional_type<__constant_iterators,
				    const _Value&, _Value&>::__type
                                                       reference;
      typedef std::ptrdiff_t                           difference_type;
      typedef std::forward_iterator_tag                iterator_category;

      _Node_iterator()
      : _Node_iterator_base<_Value, __cache>(0) { }

      explicit
      _Node_iterator(_Hash_node<_Value, __cache>* __p)
      : _Node_iterator_base<_Value, __cache>(__p) { }

      reference
      operator*() const
      { return this->_M_cur->_M_v; }
  
      pointer
      operator->() const
      { return std::__addressof(this->_M_cur->_M_v); }

      _Node_iterator&
      operator++()
      { 
	this->_M_incr();
	return *this; 
      }
  
      _Node_iterator
      operator++(int)
      { 
	_Node_iterator __tmp(*this);
	this->_M_incr();
	return __tmp;
      }
    };

  template<typename _Value, bool __constant_iterators, bool __cache>
    struct _Node_const_iterator
    : public _Node_iterator_base<_Value, __cache>
    {
      typedef _Value                                   value_type;
      typedef const _Value*                            pointer;
      typedef const _Value&                            reference;
      typedef std::ptrdiff_t                           difference_type;
      typedef std::forward_iterator_tag                iterator_category;

      _Node_const_iterator()
      : _Node_iterator_base<_Value, __cache>(0) { }

      explicit
      _Node_const_iterator(_Hash_node<_Value, __cache>* __p)
      : _Node_iterator_base<_Value, __cache>(__p) { }

      _Node_const_iterator(const _Node_iterator<_Value, __constant_iterators,
			   __cache>& __x)
      : _Node_iterator_base<_Value, __cache>(__x._M_cur) { }

      reference
      operator*() const
      { return this->_M_cur->_M_v; }
  
      pointer
      operator->() const
      { return std::__addressof(this->_M_cur->_M_v); }

      _Node_const_iterator&
      operator++()
      { 
	this->_M_incr();
	return *this; 
      }
  
      _Node_const_iterator
      operator++(int)
      { 
	_Node_const_iterator __tmp(*this);
	this->_M_incr();
	return __tmp;
      }
    };

  template<typename _Value, bool __cache>
    struct _Hashtable_iterator_base
    {
      _Hashtable_iterator_base(_Hash_node<_Value, __cache>* __node,
			       _Hash_node<_Value, __cache>** __bucket)
      : _M_cur_node(__node), _M_cur_bucket(__bucket) { }

      void
      _M_incr()
      {
	_M_cur_node = _M_cur_node->_M_next;
	if (!_M_cur_node)
	  _M_incr_bucket();
      }

      void
      _M_incr_bucket();

      _Hash_node<_Value, __cache>*   _M_cur_node;
      _Hash_node<_Value, __cache>**  _M_cur_bucket;
    };

  // Global iterators, used for arbitrary iteration within a hash
  // table.  Larger and more expensive than local iterators.
  template<typename _Value, bool __cache>
    void
    _Hashtable_iterator_base<_Value, __cache>::
    _M_incr_bucket()
    {
      ++_M_cur_bucket;

      // This loop requires the bucket array to have a non-null sentinel.
      while (!*_M_cur_bucket)
	++_M_cur_bucket;
      _M_cur_node = *_M_cur_bucket;
    }

  template<typename _Value, bool __cache>
    inline bool
    operator==(const _Hashtable_iterator_base<_Value, __cache>& __x,
	       const _Hashtable_iterator_base<_Value, __cache>& __y)
    { return __x._M_cur_node == __y._M_cur_node; }

  template<typename _Value, bool __cache>
    inline bool
    operator!=(const _Hashtable_iterator_base<_Value, __cache>& __x,
	       const _Hashtable_iterator_base<_Value, __cache>& __y)
    { return __x._M_cur_node != __y._M_cur_node; }

  template<typename _Value, bool __constant_iterators, bool __cache>
    struct _Hashtable_iterator
    : public _Hashtable_iterator_base<_Value, __cache>
    {
      typedef _Value                                   value_type;
      typedef typename
      __gnu_cxx::__conditional_type<__constant_iterators,
				    const _Value*, _Value*>::__type
                                                       pointer;
      typedef typename
      __gnu_cxx::__conditional_type<__constant_iterators,
				    const _Value&, _Value&>::__type
                                                       reference;
      typedef std::ptrdiff_t                           difference_type;
      typedef std::forward_iterator_tag                iterator_category;

      _Hashtable_iterator()
      : _Hashtable_iterator_base<_Value, __cache>(0, 0) { }

      _Hashtable_iterator(_Hash_node<_Value, __cache>* __p,
			  _Hash_node<_Value, __cache>** __b)
      : _Hashtable_iterator_base<_Value, __cache>(__p, __b) { }

      explicit
      _Hashtable_iterator(_Hash_node<_Value, __cache>** __b)
      : _Hashtable_iterator_base<_Value, __cache>(*__b, __b) { }

      reference
      operator*() const
      { return this->_M_cur_node->_M_v; }
  
      pointer
      operator->() const
      { return std::__addressof(this->_M_cur_node->_M_v); }

      _Hashtable_iterator&
      operator++()
      { 
	this->_M_incr();
	return *this;
      }
  
      _Hashtable_iterator
      operator++(int)
      { 
	_Hashtable_iterator __tmp(*this);
	this->_M_incr();
	return __tmp;
      }
    };

  template<typename _Value, bool __constant_iterators, bool __cache>
    struct _Hashtable_const_iterator
    : public _Hashtable_iterator_base<_Value, __cache>
    {
      typedef _Value                                   value_type;
      typedef const _Value*                            pointer;
      typedef const _Value&                            reference;
      typedef std::ptrdiff_t                           difference_type;
      typedef std::forward_iterator_tag                iterator_category;

      _Hashtable_const_iterator()
      : _Hashtable_iterator_base<_Value, __cache>(0, 0) { }

      _Hashtable_const_iterator(_Hash_node<_Value, __cache>* __p,
				_Hash_node<_Value, __cache>** __b)
      : _Hashtable_iterator_base<_Value, __cache>(__p, __b) { }

      explicit
      _Hashtable_const_iterator(_Hash_node<_Value, __cache>** __b)
      : _Hashtable_iterator_base<_Value, __cache>(*__b, __b) { }

      _Hashtable_const_iterator(const _Hashtable_iterator<_Value,
				__constant_iterators, __cache>& __x)
      : _Hashtable_iterator_base<_Value, __cache>(__x._M_cur_node,
						  __x._M_cur_bucket) { }

      reference
      operator*() const
      { return this->_M_cur_node->_M_v; }
  
      pointer
      operator->() const
      { return std::__addressof(this->_M_cur_node->_M_v); }

      _Hashtable_const_iterator&
      operator++()
      { 
	this->_M_incr();
	return *this;
      }
  
      _Hashtable_const_iterator
      operator++(int)
      { 
	_Hashtable_const_iterator __tmp(*this);
	this->_M_incr();
	return __tmp;
      }
    };


  // Many of class template _Hashtable's template parameters are policy
  // classes.  These are defaults for the policies.

  // Default range hashing function: use division to fold a large number
  // into the range [0, N).
  struct _Mod_range_hashing
  {
    typedef std::size_t first_argument_type;
    typedef std::size_t second_argument_type;
    typedef std::size_t result_type;

    result_type
    operator()(first_argument_type __num, second_argument_type __den) const
    { return __num % __den; }
  };

  // Default ranged hash function H.  In principle it should be a
  // function object composed from objects of type H1 and H2 such that
  // h(k, N) = h2(h1(k), N), but that would mean making extra copies of
  // h1 and h2.  So instead we'll just use a tag to tell class template
  // hashtable to do that composition.
  struct _Default_ranged_hash { };

  // Default value for rehash policy.  Bucket size is (usually) the
  // smallest prime that keeps the load factor small enough.
  struct _Prime_rehash_policy
  {
    _Prime_rehash_policy(float __z = 1.0)
    : _M_max_load_factor(__z), _M_growth_factor(2.f), _M_next_resize(0) { }

    float
    max_load_factor() const
    { return _M_max_load_factor; }      

    // Return a bucket size no smaller than n.
    std::size_t
    _M_next_bkt(std::size_t __n) const;
    
    // Return a bucket count appropriate for n elements
    std::size_t
    _M_bkt_for_elements(std::size_t __n) const;
    
    // __n_bkt is current bucket count, __n_elt is current element count,
    // and __n_ins is number of elements to be inserted.  Do we need to
    // increase bucket count?  If so, return make_pair(true, n), where n
    // is the new bucket count.  If not, return make_pair(false, 0).
    std::pair<bool, std::size_t>
    _M_need_rehash(std::size_t __n_bkt, std::size_t __n_elt,
		   std::size_t __n_ins) const;

    enum { _S_n_primes = sizeof(unsigned long) != 8 ? 256 : 256 + 48 };

    float                _M_max_load_factor;
    float                _M_growth_factor;
    mutable std::size_t  _M_next_resize;
  };

  extern const unsigned long __prime_list[];

  // XXX This is a hack.  There's no good reason for any of
  // _Prime_rehash_policy's member functions to be inline.  

  // Return a prime no smaller than n.
  inline std::size_t
  _Prime_rehash_policy::
  _M_next_bkt(std::size_t __n) const
  {
    // Don't include the last prime in the search, so that anything
    // higher than the second-to-last prime returns a past-the-end
    // iterator that can be dereferenced to get the last prime.
    const unsigned long* __p
      = std::lower_bound(__prime_list, __prime_list + _S_n_primes - 1, __n);
    _M_next_resize = 
      static_cast<std::size_t>(__builtin_ceil(*__p * _M_max_load_factor));
    return *__p;
  }

  // Return the smallest prime p such that alpha p >= n, where alpha
  // is the load factor.
  inline std::size_t
  _Prime_rehash_policy::
  _M_bkt_for_elements(std::size_t __n) const
  {
    const float __min_bkts = __n / _M_max_load_factor;
    return _M_next_bkt(__builtin_ceil(__min_bkts));
  }

  // Finds the smallest prime p such that alpha p > __n_elt + __n_ins.
  // If p > __n_bkt, return make_pair(true, p); otherwise return
  // make_pair(false, 0).  In principle this isn't very different from 
  // _M_bkt_for_elements.

  // The only tricky part is that we're caching the element count at
  // which we need to rehash, so we don't have to do a floating-point
  // multiply for every insertion.

  inline std::pair<bool, std::size_t>
  _Prime_rehash_policy::
  _M_need_rehash(std::size_t __n_bkt, std::size_t __n_elt,
		 std::size_t __n_ins) const
  {
    if (__n_elt + __n_ins > _M_next_resize)
      {
	float __min_bkts = ((float(__n_ins) + float(__n_elt))
			    / _M_max_load_factor);
	if (__min_bkts > __n_bkt)
	  {
	    __min_bkts = std::max(__min_bkts, _M_growth_factor * __n_bkt);
	    return std::make_pair(true,
				  _M_next_bkt(__builtin_ceil(__min_bkts)));
	  }
	else 
	  {
	    _M_next_resize = static_cast<std::size_t>
	      (__builtin_ceil(__n_bkt * _M_max_load_factor));
	    return std::make_pair(false, 0);
	  }
      }
    else
      return std::make_pair(false, 0);
  }

  // Base classes for std::tr1::_Hashtable.  We define these base
  // classes because in some cases we want to do different things
  // depending on the value of a policy class.  In some cases the
  // policy class affects which member functions and nested typedefs
  // are defined; we handle that by specializing base class templates.
  // Several of the base class templates need to access other members
  // of class template _Hashtable, so we use the "curiously recurring
  // template pattern" for them.

  // class template _Map_base.  If the hashtable has a value type of the
  // form pair<T1, T2> and a key extraction policy that returns the
  // first part of the pair, the hashtable gets a mapped_type typedef.
  // If it satisfies those criteria and also has unique keys, then it
  // also gets an operator[].  
  template<typename _Key, typename _Value, typename _Ex, bool __unique,
	   typename _Hashtable>
    struct _Map_base { };
	  
  template<typename _Key, typename _Pair, typename _Hashtable>
    struct _Map_base<_Key, _Pair, std::_Select1st<_Pair>, false, _Hashtable>
    {
      typedef typename _Pair::second_type mapped_type;
    };

  template<typename _Key, typename _Pair, typename _Hashtable>
    struct _Map_base<_Key, _Pair, std::_Select1st<_Pair>, true, _Hashtable>
    {
      typedef typename _Pair::second_type mapped_type;
      
      mapped_type&
      operator[](const _Key& __k);
    };

  template<typename _Key, typename _Pair, typename _Hashtable>
    typename _Map_base<_Key, _Pair, std::_Select1st<_Pair>,
		       true, _Hashtable>::mapped_type&
    _Map_base<_Key, _Pair, std::_Select1st<_Pair>, true, _Hashtable>::
    operator[](const _Key& __k)
    {
      _Hashtable* __h = static_cast<_Hashtable*>(this);
      typename _Hashtable::_Hash_code_type __code = __h->_M_hash_code(__k);
      std::size_t __n = __h->_M_bucket_index(__k, __code,
					     __h->_M_bucket_count);

      typename _Hashtable::_Node* __p =
	__h->_M_find_node(__h->_M_buckets[__n], __k, __code);
      if (!__p)
	return __h->_M_insert_bucket(std::make_pair(__k, mapped_type()),
				     __n, __code)->second;
      return (__p->_M_v).second;
    }

  // class template _Rehash_base.  Give hashtable the max_load_factor
  // functions iff the rehash policy is _Prime_rehash_policy.
  template<typename _RehashPolicy, typename _Hashtable>
    struct _Rehash_base { };

  template<typename _Hashtable>
    struct _Rehash_base<_Prime_rehash_policy, _Hashtable>
    {
      float
      max_load_factor() const
      {
	const _Hashtable* __this = static_cast<const _Hashtable*>(this);
	return __this->__rehash_policy().max_load_factor();
      }

      void
      max_load_factor(float __z)
      {
	_Hashtable* __this = static_cast<_Hashtable*>(this);
	__this->__rehash_policy(_Prime_rehash_policy(__z));
      }
    };

  // Class template _Hash_code_base.  Encapsulates two policy issues that
  // aren't quite orthogonal.
  //   (1) the difference between using a ranged hash function and using
  //       the combination of a hash function and a range-hashing function.
  //       In the former case we don't have such things as hash codes, so
  //       we have a dummy type as placeholder.
  //   (2) Whether or not we cache hash codes.  Caching hash codes is
  //       meaningless if we have a ranged hash function.
  // We also put the key extraction and equality comparison function 
  // objects here, for convenience.
  
  // Primary template: unused except as a hook for specializations.  
  template<typename _Key, typename _Value,
	   typename _ExtractKey, typename _Equal,
	   typename _H1, typename _H2, typename _Hash,
	   bool __cache_hash_code>
    struct _Hash_code_base;

  // Specialization: ranged hash function, no caching hash codes.  H1
  // and H2 are provided but ignored.  We define a dummy hash code type.
  template<typename _Key, typename _Value,
	   typename _ExtractKey, typename _Equal,
	   typename _H1, typename _H2, typename _Hash>
    struct _Hash_code_base<_Key, _Value, _ExtractKey, _Equal, _H1, _H2,
			   _Hash, false>
    {
    protected:
      _Hash_code_base(const _ExtractKey& __ex, const _Equal& __eq,
		      const _H1&, const _H2&, const _Hash& __h)
      : _M_extract(__ex), _M_eq(__eq), _M_ranged_hash(__h) { }

      typedef void* _Hash_code_type;
  
      _Hash_code_type
      _M_hash_code(const _Key& __key) const
      { return 0; }
  
      std::size_t
      _M_bucket_index(const _Key& __k, _Hash_code_type,
		      std::size_t __n) const
      { return _M_ranged_hash(__k, __n); }

      std::size_t
      _M_bucket_index(const _Hash_node<_Value, false>* __p,
		      std::size_t __n) const
      { return _M_ranged_hash(_M_extract(__p->_M_v), __n); }
  
      bool
      _M_compare(const _Key& __k, _Hash_code_type,
		 _Hash_node<_Value, false>* __n) const
      { return _M_eq(__k, _M_extract(__n->_M_v)); }

      void
      _M_store_code(_Hash_node<_Value, false>*, _Hash_code_type) const
      { }

      void
      _M_copy_code(_Hash_node<_Value, false>*,
		   const _Hash_node<_Value, false>*) const
      { }
      
      void
      _M_swap(_Hash_code_base& __x)
      {
	std::swap(_M_extract, __x._M_extract);
	std::swap(_M_eq, __x._M_eq);
	std::swap(_M_ranged_hash, __x._M_ranged_hash);
      }

    protected:
      _ExtractKey  _M_extract;
      _Equal       _M_eq;
      _Hash        _M_ranged_hash;
    };


  // No specialization for ranged hash function while caching hash codes.
  // That combination is meaningless, and trying to do it is an error.
  
  
  // Specialization: ranged hash function, cache hash codes.  This
  // combination is meaningless, so we provide only a declaration
  // and no definition.  
  template<typename _Key, typename _Value,
	   typename _ExtractKey, typename _Equal,
	   typename _H1, typename _H2, typename _Hash>
    struct _Hash_code_base<_Key, _Value, _ExtractKey, _Equal, _H1, _H2,
			   _Hash, true>;

  // Specialization: hash function and range-hashing function, no
  // caching of hash codes.  H is provided but ignored.  Provides
  // typedef and accessor required by TR1.  
  template<typename _Key, typename _Value,
	   typename _ExtractKey, typename _Equal,
	   typename _H1, typename _H2>
    struct _Hash_code_base<_Key, _Value, _ExtractKey, _Equal, _H1, _H2,
			   _Default_ranged_hash, false>
    {
      typedef _H1 hasher;

      hasher
      hash_function() const
      { return _M_h1; }

    protected:
      _Hash_code_base(const _ExtractKey& __ex, const _Equal& __eq,
		      const _H1& __h1, const _H2& __h2,
		      const _Default_ranged_hash&)
      : _M_extract(__ex), _M_eq(__eq), _M_h1(__h1), _M_h2(__h2) { }

      typedef std::size_t _Hash_code_type;

      _Hash_code_type
      _M_hash_code(const _Key& __k) const
      { return _M_h1(__k); }
      
      std::size_t
      _M_bucket_index(const _Key&, _Hash_code_type __c,
		      std::size_t __n) const
      { return _M_h2(__c, __n); }

      std::size_t
      _M_bucket_index(const _Hash_node<_Value, false>* __p,
		      std::size_t __n) const
      { return _M_h2(_M_h1(_M_extract(__p->_M_v)), __n); }

      bool
      _M_compare(const _Key& __k, _Hash_code_type,
		 _Hash_node<_Value, false>* __n) const
      { return _M_eq(__k, _M_extract(__n->_M_v)); }

      void
      _M_store_code(_Hash_node<_Value, false>*, _Hash_code_type) const
      { }

      void
      _M_copy_code(_Hash_node<_Value, false>*,
		   const _Hash_node<_Value, false>*) const
      { }

      void
      _M_swap(_Hash_code_base& __x)
      {
	std::swap(_M_extract, __x._M_extract);
	std::swap(_M_eq, __x._M_eq);
	std::swap(_M_h1, __x._M_h1);
	std::swap(_M_h2, __x._M_h2);
      }

    protected:
      _ExtractKey  _M_extract;
      _Equal       _M_eq;
      _H1          _M_h1;
      _H2          _M_h2;
    };

  // Specialization: hash function and range-hashing function, 
  // caching hash codes.  H is provided but ignored.  Provides
  // typedef and accessor required by TR1.
  template<typename _Key, typename _Value,
	   typename _ExtractKey, typename _Equal,
	   typename _H1, typename _H2>
    struct _Hash_code_base<_Key, _Value, _ExtractKey, _Equal, _H1, _H2,
			   _Default_ranged_hash, true>
    {
      typedef _H1 hasher;
      
      hasher
      hash_function() const
      { return _M_h1; }

    protected:
      _Hash_code_base(const _ExtractKey& __ex, const _Equal& __eq,
		      const _H1& __h1, const _H2& __h2,
		      const _Default_ranged_hash&)
      : _M_extract(__ex), _M_eq(__eq), _M_h1(__h1), _M_h2(__h2) { }

      typedef std::size_t _Hash_code_type;
  
      _Hash_code_type
      _M_hash_code(const _Key& __k) const
      { return _M_h1(__k); }
  
      std::size_t
      _M_bucket_index(const _Key&, _Hash_code_type __c,
		      std::size_t __n) const
      { return _M_h2(__c, __n); }

      std::size_t
      _M_bucket_index(const _Hash_node<_Value, true>* __p,
		      std::size_t __n) const
      { return _M_h2(__p->_M_hash_code, __n); }

      bool
      _M_compare(const _Key& __k, _Hash_code_type __c,
		 _Hash_node<_Value, true>* __n) const
      { return __c == __n->_M_hash_code && _M_eq(__k, _M_extract(__n->_M_v)); }

      void
      _M_store_code(_Hash_node<_Value, true>* __n, _Hash_code_type __c) const
      { __n->_M_hash_code = __c; }

      void
      _M_copy_code(_Hash_node<_Value, true>* __to,
		   const _Hash_node<_Value, true>* __from) const
      { __to->_M_hash_code = __from->_M_hash_code; }

      void
      _M_swap(_Hash_code_base& __x)
      {
	std::swap(_M_extract, __x._M_extract);
	std::swap(_M_eq, __x._M_eq);
	std::swap(_M_h1, __x._M_h1);
	std::swap(_M_h2, __x._M_h2);
      }
      
    protected:
      _ExtractKey  _M_extract;
      _Equal       _M_eq;
      _H1          _M_h1;
      _H2          _M_h2;
    };
} // namespace __detail
}

_GLIBCXX_END_NAMESPACE_VERSION
}
