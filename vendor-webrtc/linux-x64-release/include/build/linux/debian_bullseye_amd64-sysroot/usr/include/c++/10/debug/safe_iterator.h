// Safe iterator implementation  -*- C++ -*-

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

/** @file debug/safe_iterator.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_SAFE_ITERATOR_H
#define _GLIBCXX_DEBUG_SAFE_ITERATOR_H 1

#include <debug/assertions.h>
#include <debug/macros.h>
#include <debug/functions.h>
#include <debug/safe_base.h>
#include <bits/stl_pair.h>
#include <ext/type_traits.h>
#if __cplusplus > 201703L
# include <compare>
#endif

#define _GLIBCXX_DEBUG_VERIFY_OPERANDS(_Lhs, _Rhs, _BadMsgId, _DiffMsgId) \
  _GLIBCXX_DEBUG_VERIFY(!_Lhs._M_singular() && !_Rhs._M_singular(),	\
			_M_message(_BadMsgId)				\
			._M_iterator(_Lhs, #_Lhs)			\
			._M_iterator(_Rhs, #_Rhs));			\
  _GLIBCXX_DEBUG_VERIFY(_Lhs._M_can_compare(_Rhs),			\
			_M_message(_DiffMsgId)				\
			._M_iterator(_Lhs, #_Lhs)			\
			._M_iterator(_Rhs, #_Rhs))

#define _GLIBCXX_DEBUG_VERIFY_EQ_OPERANDS(_Lhs, _Rhs)			\
  _GLIBCXX_DEBUG_VERIFY_OPERANDS(_Lhs, _Rhs, __msg_iter_compare_bad,	\
				 __msg_compare_different)

#define _GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(_Lhs, _Rhs)		\
  _GLIBCXX_DEBUG_VERIFY_OPERANDS(_Lhs, _Rhs, __msg_iter_order_bad,	\
				 __msg_order_different)

#define _GLIBCXX_DEBUG_VERIFY_DIST_OPERANDS(_Lhs, _Rhs)			\
  _GLIBCXX_DEBUG_VERIFY_OPERANDS(_Lhs, _Rhs, __msg_distance_bad,	\
				 __msg_distance_different)

namespace __gnu_debug
{
  /** Helper struct to deal with sequence offering a before_begin
   *  iterator.
   **/
  template<typename _Sequence>
    struct _BeforeBeginHelper
    {
      template<typename _Iterator, typename _Category>
	static bool
	_S_Is(const _Safe_iterator<_Iterator, _Sequence, _Category>&)
	{ return false; }

      template<typename _Iterator, typename _Category>
	static bool
	_S_Is_Beginnest(const _Safe_iterator<_Iterator, _Sequence, _Category>& __it)
	{ return __it.base() == __it._M_get_sequence()->_M_base().begin(); }
    };

  /** Sequence traits giving the size of a container if possible. */
  template<typename _Sequence>
    struct _Sequence_traits
    {
      typedef _Distance_traits<typename _Sequence::iterator> _DistTraits;

      static typename _DistTraits::__type
      _S_size(const _Sequence& __seq)
      { return std::make_pair(__seq.size(), __dp_exact); }
    };

  /** \brief Safe iterator wrapper.
   *
   *  The class template %_Safe_iterator is a wrapper around an
   *  iterator that tracks the iterator's movement among sequences and
   *  checks that operations performed on the "safe" iterator are
   *  legal. In additional to the basic iterator operations (which are
   *  validated, and then passed to the underlying iterator),
   *  %_Safe_iterator has member functions for iterator invalidation,
   *  attaching/detaching the iterator from sequences, and querying
   *  the iterator's state.
   *
   *  Note that _Iterator must be the first base class so that it gets
   *  initialized before the iterator is being attached to the container's list
   *  of iterators and it is being detached before _Iterator get
   *  destroyed. Otherwise it would result in a data race.
   */
  template<typename _Iterator, typename _Sequence, typename _Category
	   = typename std::iterator_traits<_Iterator>::iterator_category>
    class _Safe_iterator
    : private _Iterator,
      public _Safe_iterator_base
    {
      typedef _Iterator _Iter_base;
      typedef _Safe_iterator_base _Safe_base;

      typedef std::iterator_traits<_Iterator> _Traits;

    protected:
      typedef std::__are_same<typename _Sequence::_Base::const_iterator,
			      _Iterator> _IsConstant;

      typedef typename __gnu_cxx::__conditional_type<
	_IsConstant::__value,
	typename _Sequence::_Base::iterator,
	typename _Sequence::_Base::const_iterator>::__type _OtherIterator;

      struct _Attach_single
      { };

      _Safe_iterator(_Iterator __i, _Safe_sequence_base* __seq, _Attach_single)
      _GLIBCXX_NOEXCEPT
      : _Iter_base(__i)
      { _M_attach_single(__seq); }

    public:
      typedef _Iterator					iterator_type;
      typedef typename _Traits::iterator_category	iterator_category;
      typedef typename _Traits::value_type		value_type;
      typedef typename _Traits::difference_type		difference_type;
      typedef typename _Traits::reference		reference;
      typedef typename _Traits::pointer			pointer;

#if __cplusplus > 201703L && __cpp_lib_concepts
      using iterator_concept = std::__detail::__iter_concept<_Iterator>;
#endif

      /// @post the iterator is singular and unattached
      _Safe_iterator() _GLIBCXX_NOEXCEPT : _Iter_base() { }

      /**
       * @brief Safe iterator construction from an unsafe iterator and
       * its sequence.
       *
       * @pre @p seq is not NULL
       * @post this is not singular
       */
      _Safe_iterator(_Iterator __i, const _Safe_sequence_base* __seq)
      _GLIBCXX_NOEXCEPT
      : _Iter_base(__i), _Safe_base(__seq, _S_constant())
      {
	_GLIBCXX_DEBUG_VERIFY(!this->_M_singular(),
			      _M_message(__msg_init_singular)
			      ._M_iterator(*this, "this"));
      }

      /**
       * @brief Copy construction.
       */
      _Safe_iterator(const _Safe_iterator& __x) _GLIBCXX_NOEXCEPT
      : _Iter_base(__x.base())
      {
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// DR 408. Is vector<reverse_iterator<char*> > forbidden?
	_GLIBCXX_DEBUG_VERIFY(!__x._M_singular()
			      || __x.base() == _Iterator(),
			      _M_message(__msg_init_copy_singular)
			      ._M_iterator(*this, "this")
			      ._M_iterator(__x, "other"));
	_M_attach(__x._M_sequence);
      }

#if __cplusplus >= 201103L
      /**
       * @brief Move construction.
       * @post __x is singular and unattached
       */
      _Safe_iterator(_Safe_iterator&& __x) noexcept
      : _Iter_base()
      {
	_GLIBCXX_DEBUG_VERIFY(!__x._M_singular()
			      || __x.base() == _Iterator(),
			      _M_message(__msg_init_copy_singular)
			      ._M_iterator(*this, "this")
			      ._M_iterator(__x, "other"));
	_Safe_sequence_base* __seq = __x._M_sequence;
	__x._M_detach();
	std::swap(base(), __x.base());
	_M_attach(__seq);
      }
#endif

      /**
       *  @brief Converting constructor from a mutable iterator to a
       *  constant iterator.
      */
      template<typename _MutableIterator>
	_Safe_iterator(
	  const _Safe_iterator<_MutableIterator, _Sequence,
	    typename __gnu_cxx::__enable_if<_IsConstant::__value &&
	      std::__are_same<_MutableIterator, _OtherIterator>::__value,
			       _Category>::__type>& __x)
	_GLIBCXX_NOEXCEPT
	: _Iter_base(__x.base())
	{
	  // _GLIBCXX_RESOLVE_LIB_DEFECTS
	  // DR 408. Is vector<reverse_iterator<char*> > forbidden?
	  _GLIBCXX_DEBUG_VERIFY(!__x._M_singular()
				|| __x.base() == _MutableIterator(),
				_M_message(__msg_init_const_singular)
				._M_iterator(*this, "this")
				._M_iterator(__x, "other"));
	  _M_attach(__x._M_sequence);
	}

      /**
       * @brief Copy assignment.
       */
      _Safe_iterator&
      operator=(const _Safe_iterator& __x) _GLIBCXX_NOEXCEPT
      {
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// DR 408. Is vector<reverse_iterator<char*> > forbidden?
	_GLIBCXX_DEBUG_VERIFY(!__x._M_singular()
			      || __x.base() == _Iterator(),
			      _M_message(__msg_copy_singular)
			      ._M_iterator(*this, "this")
			      ._M_iterator(__x, "other"));

	if (this->_M_sequence && this->_M_sequence == __x._M_sequence)
	  {
	    __gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	    base() = __x.base();
	    _M_version = __x._M_sequence->_M_version;
	  }
	else
	  {
	    _M_detach();
	    base() = __x.base();
	    _M_attach(__x._M_sequence);
	  }

	return *this;
      }

#if __cplusplus >= 201103L
      /**
       * @brief Move assignment.
       * @post __x is singular and unattached
       */
      _Safe_iterator&
      operator=(_Safe_iterator&& __x) noexcept
      {
	_GLIBCXX_DEBUG_VERIFY(this != &__x,
			      _M_message(__msg_self_move_assign)
			      ._M_iterator(*this, "this"));
	_GLIBCXX_DEBUG_VERIFY(!__x._M_singular()
			      || __x.base() == _Iterator(),
			      _M_message(__msg_copy_singular)
			      ._M_iterator(*this, "this")
			      ._M_iterator(__x, "other"));

	if (this->_M_sequence && this->_M_sequence == __x._M_sequence)
	  {
	    __gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	    base() = __x.base();
	    _M_version = __x._M_sequence->_M_version;
	  }
	else
	  {
	    _M_detach();
	    base() = __x.base();
	    _M_attach(__x._M_sequence);
	  }

	__x._M_detach();
	__x.base() = _Iterator();
	return *this;
      }
#endif

      /**
       *  @brief Iterator dereference.
       *  @pre iterator is dereferenceable
       */
      reference
      operator*() const _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_dereferenceable(),
			      _M_message(__msg_bad_deref)
			      ._M_iterator(*this, "this"));
	return *base();
      }

      /**
       *  @brief Iterator dereference.
       *  @pre iterator is dereferenceable
       */
      pointer
      operator->() const _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_dereferenceable(),
			      _M_message(__msg_bad_deref)
			      ._M_iterator(*this, "this"));
	return base().operator->();
      }

      // ------ Input iterator requirements ------
      /**
       *  @brief Iterator preincrement
       *  @pre iterator is incrementable
       */
      _Safe_iterator&
      operator++() _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_incrementable(),
			      _M_message(__msg_bad_inc)
			      ._M_iterator(*this, "this"));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	++base();
	return *this;
      }

      /**
       *  @brief Iterator postincrement
       *  @pre iterator is incrementable
       */
      _Safe_iterator
      operator++(int) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_incrementable(),
			      _M_message(__msg_bad_inc)
			      ._M_iterator(*this, "this"));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	return _Safe_iterator(base()++, this->_M_sequence, _Attach_single());
      }

      // ------ Utilities ------

      /// Determine if this is a constant iterator.
      static _GLIBCXX_CONSTEXPR bool
      _S_constant()
      { return _IsConstant::__value; }

      /**
       * @brief Return the underlying iterator
       */
      _Iterator&
      base() _GLIBCXX_NOEXCEPT { return *this; }

      const _Iterator&
      base() const _GLIBCXX_NOEXCEPT { return *this; }

      /**
       * @brief Conversion to underlying non-debug iterator to allow
       * better interaction with non-debug containers.
       */
      operator _Iterator() const _GLIBCXX_NOEXCEPT { return *this; }

      /** Attach iterator to the given sequence. */
      void
      _M_attach(_Safe_sequence_base* __seq)
      { _Safe_base::_M_attach(__seq, _S_constant()); }

      /** Likewise, but not thread-safe. */
      void
      _M_attach_single(_Safe_sequence_base* __seq)
      { _Safe_base::_M_attach_single(__seq, _S_constant()); }

      /// Is the iterator dereferenceable?
      bool
      _M_dereferenceable() const
      { return !this->_M_singular() && !_M_is_end() && !_M_is_before_begin(); }

      /// Is the iterator before a dereferenceable one?
      bool
      _M_before_dereferenceable() const
      {
	if (this->_M_incrementable())
	{
	  _Iterator __base = base();
	  return ++__base != _M_get_sequence()->_M_base().end();
	}
	return false;
      }

      /// Is the iterator incrementable?
      bool
      _M_incrementable() const
      { return !this->_M_singular() && !_M_is_end(); }

      // Can we advance the iterator @p __n steps (@p __n may be negative)
      bool
      _M_can_advance(difference_type __n, bool __strict = false) const;

      // Is the iterator range [*this, __rhs) valid?
      bool
      _M_valid_range(const _Safe_iterator& __rhs,
		     std::pair<difference_type, _Distance_precision>& __dist,
		     bool __check_dereferenceable = true) const;

      // The sequence this iterator references.
      typename __gnu_cxx::__conditional_type<
	_IsConstant::__value, const _Sequence*, _Sequence*>::__type
      _M_get_sequence() const
      { return static_cast<_Sequence*>(_M_sequence); }

      // Get distance to __rhs.
      typename _Distance_traits<_Iterator>::__type
      _M_get_distance_to(const _Safe_iterator& __rhs) const;

      // Get distance from sequence begin up to *this.
      typename _Distance_traits<_Iterator>::__type
      _M_get_distance_from_begin() const;

      // Get distance from *this to sequence end.
      typename _Distance_traits<_Iterator>::__type
      _M_get_distance_to_end() const;

      /// Is this iterator equal to the sequence's begin() iterator?
      bool
      _M_is_begin() const
      { return base() == _M_get_sequence()->_M_base().begin(); }

      /// Is this iterator equal to the sequence's end() iterator?
      bool
      _M_is_end() const
      { return base() == _M_get_sequence()->_M_base().end(); }

      /// Is this iterator equal to the sequence's before_begin() iterator if
      /// any?
      bool
      _M_is_before_begin() const
      { return _BeforeBeginHelper<_Sequence>::_S_Is(*this); }

      /// Is this iterator equal to the sequence's before_begin() iterator if
      /// any or begin() otherwise?
      bool
      _M_is_beginnest() const
      { return _BeforeBeginHelper<_Sequence>::_S_Is_Beginnest(*this); }

      // ------ Operators ------

      typedef _Safe_iterator<_Iterator, _Sequence, iterator_category> _Self;

      friend bool
      operator==(const _Self& __lhs, const _Self& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_EQ_OPERANDS(__lhs, __rhs);
	return __lhs.base() == __rhs.base();
      }

      template<typename _IteR>
	friend bool
	operator==(const _Self& __lhs,
	  const _Safe_iterator<_IteR, _Sequence, iterator_category>& __rhs)
	_GLIBCXX_NOEXCEPT
	{
	  _GLIBCXX_DEBUG_VERIFY_EQ_OPERANDS(__lhs, __rhs);
	  return __lhs.base() == __rhs.base();
	}

#if ! __cpp_lib_three_way_comparison
      friend bool
      operator!=(const _Self& __lhs, const _Self& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_EQ_OPERANDS(__lhs, __rhs);
	return __lhs.base() != __rhs.base();
      }

      template<typename _IteR>
	friend bool
	operator!=(const _Self& __lhs,
	  const _Safe_iterator<_IteR, _Sequence, iterator_category>& __rhs)
	_GLIBCXX_NOEXCEPT
	{
	  _GLIBCXX_DEBUG_VERIFY_EQ_OPERANDS(__lhs, __rhs);
	  return __lhs.base() != __rhs.base();
	}
#endif // three-way comparison
    };

  template<typename _Iterator, typename _Sequence>
    class _Safe_iterator<_Iterator, _Sequence, std::bidirectional_iterator_tag>
    : public _Safe_iterator<_Iterator, _Sequence, std::forward_iterator_tag>
    {
      typedef _Safe_iterator<_Iterator, _Sequence,
			     std::forward_iterator_tag> _Safe_base;

    protected:
      typedef typename _Safe_base::_OtherIterator _OtherIterator;
      typedef typename _Safe_base::_Attach_single _Attach_single;

      _Safe_iterator(_Iterator __i, _Safe_sequence_base* __seq, _Attach_single)
      _GLIBCXX_NOEXCEPT
      : _Safe_base(__i, __seq, _Attach_single())
      { }

    public:
      /// @post the iterator is singular and unattached
      _Safe_iterator() _GLIBCXX_NOEXCEPT { }

      /**
       * @brief Safe iterator construction from an unsafe iterator and
       * its sequence.
       *
       * @pre @p seq is not NULL
       * @post this is not singular
       */
      _Safe_iterator(_Iterator __i, const _Safe_sequence_base* __seq)
      _GLIBCXX_NOEXCEPT
      : _Safe_base(__i, __seq)
      { }

      /**
       * @brief Copy construction.
       */
      _Safe_iterator(const _Safe_iterator& __x) _GLIBCXX_NOEXCEPT
      : _Safe_base(__x)
      { }

#if __cplusplus >= 201103L
      /** @brief Move construction. */
      _Safe_iterator(_Safe_iterator&&) = default;
#endif

      /**
       *  @brief Converting constructor from a mutable iterator to a
       *  constant iterator.
      */
      template<typename _MutableIterator>
	_Safe_iterator(
	  const _Safe_iterator<_MutableIterator, _Sequence,
	    typename __gnu_cxx::__enable_if<_Safe_base::_IsConstant::__value &&
	      std::__are_same<_MutableIterator, _OtherIterator>::__value,
			       std::bidirectional_iterator_tag>::__type>& __x)
	_GLIBCXX_NOEXCEPT
	: _Safe_base(__x)
        { }

#if __cplusplus >= 201103L
      /** @brief Copy assignment. */
      _Safe_iterator&
      operator=(const _Safe_iterator&) = default;

      /** @brief Move assignment. */
      _Safe_iterator&
      operator=(_Safe_iterator&&) = default;
#else
      /** @brief Copy assignment. */
      _Safe_iterator&
      operator=(const _Safe_iterator& __x)
      {
	_Safe_base::operator=(__x);
	return *this;
      }
#endif

      // ------ Input iterator requirements ------
      /**
       *  @brief Iterator preincrement
       *  @pre iterator is incrementable
       */
      _Safe_iterator&
      operator++() _GLIBCXX_NOEXCEPT
      {
	_Safe_base::operator++();
	return *this;
      }

      /**
       *  @brief Iterator postincrement
       *  @pre iterator is incrementable
       */
      _Safe_iterator
      operator++(int) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_incrementable(),
			      _M_message(__msg_bad_inc)
			      ._M_iterator(*this, "this"));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	return _Safe_iterator(this->base()++, this->_M_sequence,
			      _Attach_single());
      }

      // ------ Bidirectional iterator requirements ------
      /**
       *  @brief Iterator predecrement
       *  @pre iterator is decrementable
       */
      _Safe_iterator&
      operator--() _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_decrementable(),
			      _M_message(__msg_bad_dec)
			      ._M_iterator(*this, "this"));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	--this->base();
	return *this;
      }

      /**
       *  @brief Iterator postdecrement
       *  @pre iterator is decrementable
       */
      _Safe_iterator
      operator--(int) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_decrementable(),
			      _M_message(__msg_bad_dec)
			      ._M_iterator(*this, "this"));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	return _Safe_iterator(this->base()--, this->_M_sequence,
			      _Attach_single());
      }

      // ------ Utilities ------

      // Is the iterator decrementable?
      bool
      _M_decrementable() const
      { return !this->_M_singular() && !this->_M_is_begin(); }
    };

  template<typename _Iterator, typename _Sequence>
    class _Safe_iterator<_Iterator, _Sequence, std::random_access_iterator_tag>
    : public _Safe_iterator<_Iterator, _Sequence,
			    std::bidirectional_iterator_tag>
    {
      typedef _Safe_iterator<_Iterator, _Sequence,
			     std::bidirectional_iterator_tag> _Safe_base;
      typedef typename _Safe_base::_OtherIterator _OtherIterator;

      typedef typename _Safe_base::_Self _Self;
      typedef _Safe_iterator<_OtherIterator, _Sequence,
			     std::random_access_iterator_tag> _OtherSelf;

      typedef typename _Safe_base::_Attach_single _Attach_single;

      _Safe_iterator(_Iterator __i, _Safe_sequence_base* __seq, _Attach_single)
      _GLIBCXX_NOEXCEPT
      : _Safe_base(__i, __seq, _Attach_single())
      { }

    public:
      typedef typename _Safe_base::difference_type	difference_type;
      typedef typename _Safe_base::reference		reference;

      /// @post the iterator is singular and unattached
      _Safe_iterator() _GLIBCXX_NOEXCEPT { }

      /**
       * @brief Safe iterator construction from an unsafe iterator and
       * its sequence.
       *
       * @pre @p seq is not NULL
       * @post this is not singular
       */
      _Safe_iterator(_Iterator __i, const _Safe_sequence_base* __seq)
      _GLIBCXX_NOEXCEPT
      : _Safe_base(__i, __seq)
      { }

      /**
       * @brief Copy construction.
       */
      _Safe_iterator(const _Safe_iterator& __x) _GLIBCXX_NOEXCEPT
      : _Safe_base(__x)
      { }

#if __cplusplus >= 201103L
      /** @brief Move construction. */
      _Safe_iterator(_Safe_iterator&&) = default;
#endif

      /**
       *  @brief Converting constructor from a mutable iterator to a
       *  constant iterator.
      */
      template<typename _MutableIterator>
	_Safe_iterator(
	  const _Safe_iterator<_MutableIterator, _Sequence,
	    typename __gnu_cxx::__enable_if<_Safe_base::_IsConstant::__value &&
	      std::__are_same<_MutableIterator, _OtherIterator>::__value,
			       std::random_access_iterator_tag>::__type>& __x)
	_GLIBCXX_NOEXCEPT
	: _Safe_base(__x)
        { }

#if __cplusplus >= 201103L
      /** @brief Copy assignment. */
      _Safe_iterator&
      operator=(const _Safe_iterator&) = default;

      /** @brief Move assignment. */
      _Safe_iterator&
      operator=(_Safe_iterator&&) = default;
#else
      /** @brief Copy assignment. */
      _Safe_iterator&
      operator=(const _Safe_iterator& __x)
      {
	_Safe_base::operator=(__x);
	return *this;
      }
#endif

      // Is the iterator range [*this, __rhs) valid?
      bool
      _M_valid_range(const _Safe_iterator& __rhs,
		     std::pair<difference_type,
			       _Distance_precision>& __dist) const;

      // ------ Input iterator requirements ------
      /**
       *  @brief Iterator preincrement
       *  @pre iterator is incrementable
       */
      _Safe_iterator&
      operator++() _GLIBCXX_NOEXCEPT
      {
	_Safe_base::operator++();
	return *this;
      }

      /**
       *  @brief Iterator postincrement
       *  @pre iterator is incrementable
       */
      _Safe_iterator
      operator++(int) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_incrementable(),
			      _M_message(__msg_bad_inc)
			      ._M_iterator(*this, "this"));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	return _Safe_iterator(this->base()++, this->_M_sequence,
			      _Attach_single());
      }

      // ------ Bidirectional iterator requirements ------
      /**
       *  @brief Iterator predecrement
       *  @pre iterator is decrementable
       */
      _Safe_iterator&
      operator--() _GLIBCXX_NOEXCEPT
      {
	_Safe_base::operator--();
	return *this;
      }

      /**
       *  @brief Iterator postdecrement
       *  @pre iterator is decrementable
       */
      _Safe_iterator
      operator--(int) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_decrementable(),
			      _M_message(__msg_bad_dec)
			      ._M_iterator(*this, "this"));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	return _Safe_iterator(this->base()--, this->_M_sequence,
			      _Attach_single());
      }

      // ------ Random access iterator requirements ------
      reference
      operator[](difference_type __n) const _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_can_advance(__n)
			      && this->_M_can_advance(__n + 1),
			      _M_message(__msg_iter_subscript_oob)
			      ._M_iterator(*this)._M_integer(__n));
	return this->base()[__n];
      }

      _Safe_iterator&
      operator+=(difference_type __n) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_can_advance(__n),
			      _M_message(__msg_advance_oob)
			      ._M_iterator(*this)._M_integer(__n));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	this->base() += __n;
	return *this;
      }

      _Safe_iterator&
      operator-=(difference_type __n) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(this->_M_can_advance(-__n),
			      _M_message(__msg_retreat_oob)
			      ._M_iterator(*this)._M_integer(__n));
	__gnu_cxx::__scoped_lock __l(this->_M_get_mutex());
	this->base() -= __n;
	return *this;
      }

#if __cpp_lib_three_way_comparison
      friend auto
      operator<=>(const _Self& __lhs, const _Self& __rhs) noexcept
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() <=> __rhs.base();
      }

      friend auto
      operator<=>(const _Self& __lhs, const _OtherSelf& __rhs) noexcept
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() <=> __rhs.base();
      }
#else
      friend bool
      operator<(const _Self& __lhs, const _Self& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() < __rhs.base();
      }

      friend bool
      operator<(const _Self& __lhs, const _OtherSelf& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() < __rhs.base();
      }

      friend bool
      operator<=(const _Self& __lhs, const _Self& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() <= __rhs.base();
      }

      friend bool
      operator<=(const _Self& __lhs, const _OtherSelf& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() <= __rhs.base();
      }

      friend bool
      operator>(const _Self& __lhs, const _Self& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() > __rhs.base();
      }

      friend bool
      operator>(const _Self& __lhs, const _OtherSelf& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() > __rhs.base();
      }

      friend bool
      operator>=(const _Self& __lhs, const _Self& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() >= __rhs.base();
      }

      friend bool
      operator>=(const _Self& __lhs, const _OtherSelf& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_REL_OPERANDS(__lhs, __rhs);
	return __lhs.base() >= __rhs.base();
      }
#endif // three-way comparison

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // According to the resolution of DR179 not only the various comparison
      // operators but also operator- must accept mixed iterator/const_iterator
      // parameters.
      friend difference_type
      operator-(const _Self& __lhs, const _OtherSelf& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_DIST_OPERANDS(__lhs, __rhs);
	return __lhs.base() - __rhs.base();
      }

      friend difference_type
      operator-(const _Self& __lhs, const _Self& __rhs) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY_DIST_OPERANDS(__lhs, __rhs);
	return __lhs.base() - __rhs.base();
      }

      friend _Self
      operator+(const _Self& __x, difference_type __n) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(__x._M_can_advance(__n),
			      _M_message(__msg_advance_oob)
			      ._M_iterator(__x)._M_integer(__n));
	return _Safe_iterator(__x.base() + __n, __x._M_sequence);
      }

      friend _Self
      operator+(difference_type __n, const _Self& __x) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(__x._M_can_advance(__n),
			      _M_message(__msg_advance_oob)
			      ._M_iterator(__x)._M_integer(__n));
	return _Safe_iterator(__n + __x.base(), __x._M_sequence);
      }

      friend _Self
      operator-(const _Self& __x, difference_type __n) _GLIBCXX_NOEXCEPT
      {
	_GLIBCXX_DEBUG_VERIFY(__x._M_can_advance(-__n),
			      _M_message(__msg_retreat_oob)
			      ._M_iterator(__x)._M_integer(__n));
	return _Safe_iterator(__x.base() - __n, __x._M_sequence);
      }
    };

  /** Safe iterators know how to check if they form a valid range. */
  template<typename _Iterator, typename _Sequence, typename _Category>
    inline bool
    __valid_range(const _Safe_iterator<_Iterator, _Sequence,
				       _Category>& __first,
		  const _Safe_iterator<_Iterator, _Sequence,
				       _Category>& __last,
		  typename _Distance_traits<_Iterator>::__type& __dist)
    { return __first._M_valid_range(__last, __dist); }

  template<typename _Iterator, typename _Sequence, typename _Category>
    inline bool
    __valid_range(const _Safe_iterator<_Iterator, _Sequence,
				       _Category>& __first,
		  const _Safe_iterator<_Iterator, _Sequence,
				       _Category>& __last)
    {
      typename _Distance_traits<_Iterator>::__type __dist;
      return __first._M_valid_range(__last, __dist);
    }

  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _Size>
    inline bool
    __can_advance(const _Safe_iterator<_Iterator, _Sequence, _Category>& __it,
		  _Size __n)
    { return __it._M_can_advance(__n); }

  template<typename _Iterator, typename _Sequence>
    _Iterator
    __base(const _Safe_iterator<_Iterator, _Sequence,
				std::random_access_iterator_tag>& __it)
    { return __it.base(); }

#if __cplusplus < 201103L
  template<typename _Iterator, typename _Sequence>
    struct _Unsafe_type<_Safe_iterator<_Iterator, _Sequence> >
    { typedef _Iterator _Type; };
#endif

  template<typename _Iterator, typename _Sequence>
    inline _Iterator
    __unsafe(const _Safe_iterator<_Iterator, _Sequence>& __it)
    { return __it.base(); }

} // namespace __gnu_debug

#undef _GLIBCXX_DEBUG_VERIFY_DIST_OPERANDS
#undef _GLIBCXX_DEBUG_VERIFY_REL_OPERANDS
#undef _GLIBCXX_DEBUG_VERIFY_EQ_OPERANDS
#undef _GLIBCXX_DEBUG_VERIFY_OPERANDS

#include <debug/safe_iterator.tcc>

#endif
