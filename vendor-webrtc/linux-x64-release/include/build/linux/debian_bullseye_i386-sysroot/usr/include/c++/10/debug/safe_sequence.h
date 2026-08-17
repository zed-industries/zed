// Safe sequence implementation  -*- C++ -*-

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

/** @file debug/safe_sequence.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_SAFE_SEQUENCE_H
#define _GLIBCXX_DEBUG_SAFE_SEQUENCE_H 1

#include <debug/assertions.h>
#include <debug/macros.h>
#include <debug/functions.h>
#include <debug/safe_base.h>

namespace __gnu_debug
{
  /** A simple function object that returns true if the passed-in
   *  value is not equal to the stored value. It saves typing over
   *  using both bind1st and not_equal.
   */
  template<typename _Type>
    class _Not_equal_to
    {
      _Type __value;

    public:
      explicit _Not_equal_to(const _Type& __v) : __value(__v) { }

      bool
      operator()(const _Type& __x) const
      { return __value != __x; }
    };

  /** A simple function object that returns true if the passed-in
   *  value is equal to the stored value. */
  template <typename _Type>
    class _Equal_to
    {
      _Type __value;

    public:
      explicit _Equal_to(const _Type& __v) : __value(__v) { }

      bool
      operator()(const _Type& __x) const
      { return __value == __x; }
    };

  /** A function object that returns true when the given random access
      iterator is at least @c n steps away from the given iterator. */
  template<typename _Iterator>
    class _After_nth_from
    {
      typedef typename std::iterator_traits<_Iterator>::difference_type
      difference_type;

      _Iterator _M_base;
      difference_type _M_n;

    public:
      _After_nth_from(const difference_type& __n, const _Iterator& __base)
      : _M_base(__base), _M_n(__n) { }

      bool
      operator()(const _Iterator& __x) const
      { return __x - _M_base >= _M_n; }
    };

  /**
   * @brief Base class for constructing a @a safe sequence type that
   * tracks iterators that reference it.
   *
   * The class template %_Safe_sequence simplifies the construction of
   * @a safe sequences that track the iterators that reference the
   * sequence, so that the iterators are notified of changes in the
   * sequence that may affect their operation, e.g., if the container
   * invalidates its iterators or is destructed. This class template
   * may only be used by deriving from it and passing the name of the
   * derived class as its template parameter via the curiously
   * recurring template pattern. The derived class must have @c
   * iterator and @c const_iterator types that are instantiations of
   * class template _Safe_iterator for this sequence. Iterators will
   * then be tracked automatically.
   */
  template<typename _Sequence>
    class _Safe_sequence : public _Safe_sequence_base
    {
    public:
      /** Invalidates all iterators @c x that reference this sequence,
	  are not singular, and for which @c __pred(x) returns @c
	  true. @c __pred will be invoked with the normal iterators nested
	  in the safe ones. */
      template<typename _Predicate>
	void
	_M_invalidate_if(_Predicate __pred);

      /** Transfers all iterators @c x that reference @c from sequence,
	  are not singular, and for which @c __pred(x) returns @c
	  true. @c __pred will be invoked with the normal iterators nested
	  in the safe ones. */
      template<typename _Predicate>
	void
	_M_transfer_from_if(_Safe_sequence& __from, _Predicate __pred);
    };

  /// Like _Safe_sequence but with a special _M_invalidate_all implementation
  /// not invalidating past-the-end iterators. Used by node based sequence.
  template<typename _Sequence>
    class _Safe_node_sequence
    : public _Safe_sequence<_Sequence>
    {
    protected:
      void
      _M_invalidate_all()
      {
	typedef typename _Sequence::const_iterator _Const_iterator;
	typedef typename _Const_iterator::iterator_type _Base_const_iterator;
	typedef __gnu_debug::_Not_equal_to<_Base_const_iterator> _Not_equal;
	const _Sequence& __seq = *static_cast<_Sequence*>(this);
	this->_M_invalidate_if(_Not_equal(__seq._M_base().end()));
      }
    };

} // namespace __gnu_debug

#include <debug/safe_sequence.tcc>

#endif
