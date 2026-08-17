// -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the terms
// of the GNU General Public License as published by the Free Software
// Foundation; either version 3, or (at your option) any later
// version.

// This library is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file parallel/iterator.h
 * @brief Helper iterator classes for the std::transform() functions.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_ITERATOR_H
#define _GLIBCXX_PARALLEL_ITERATOR_H 1

#include <parallel/basic_iterator.h>
#include <bits/stl_pair.h>

namespace __gnu_parallel
{
  /** @brief A pair of iterators. The usual iterator operations are
   *  applied to both child iterators.
   */
  template<typename _Iterator1, typename _Iterator2,
           typename _IteratorCategory>
    class _IteratorPair : public std::pair<_Iterator1, _Iterator2>
    {
    private:
      typedef std::pair<_Iterator1, _Iterator2> _Base;

    public:
      typedef _IteratorCategory iterator_category;
      typedef void value_type;

      typedef std::iterator_traits<_Iterator1> _TraitsType;
      typedef typename _TraitsType::difference_type difference_type;
      typedef _IteratorPair* pointer;
      typedef _IteratorPair& reference;

      _IteratorPair() { }

      _IteratorPair(const _Iterator1& __first, const _Iterator2& __second)
      : _Base(__first, __second) { }

      // Pre-increment operator.
      _IteratorPair&
      operator++()
      {
        ++_Base::first;
        ++_Base::second;
        return *this;
      }

      // Post-increment operator.
      const _IteratorPair
      operator++(int)
      { return _IteratorPair(_Base::first++, _Base::second++); }

      // Pre-decrement operator.
      _IteratorPair&
      operator--()
      {
        --_Base::first;
        --_Base::second;
        return *this;
      }

      // Post-decrement operator.
      const _IteratorPair
      operator--(int)
      { return _IteratorPair(_Base::first--, _Base::second--); }

      // Type conversion.
      operator _Iterator2() const
      { return _Base::second; }

      _IteratorPair&
      operator=(const _IteratorPair& __other)
      {
        _Base::first = __other.first;
        _Base::second = __other.second;
        return *this;
      }

      _IteratorPair
      operator+(difference_type __delta) const
      { return _IteratorPair(_Base::first + __delta, _Base::second + __delta);
        }

      difference_type
      operator-(const _IteratorPair& __other) const
      { return _Base::first - __other.first; }
  };


  /** @brief A triple of iterators. The usual iterator operations are
      applied to all three child iterators.
   */
  template<typename _Iterator1, typename _Iterator2, typename _Iterator3,
           typename _IteratorCategory>
    class _IteratorTriple
    {
    public:
      typedef _IteratorCategory iterator_category;
      typedef void value_type;
      typedef typename std::iterator_traits<_Iterator1>::difference_type
                                                            difference_type;
      typedef _IteratorTriple* pointer;
      typedef _IteratorTriple& reference;

      _Iterator1 _M_first;
      _Iterator2 _M_second;
      _Iterator3 _M_third;

      _IteratorTriple() { }

      _IteratorTriple(const _Iterator1& __first, const _Iterator2& __second,
                      const _Iterator3& __third)
      {
        _M_first = __first;
        _M_second = __second;
        _M_third = __third;
      }

      // Pre-increment operator.
      _IteratorTriple&
      operator++()
      {
        ++_M_first;
        ++_M_second;
        ++_M_third;
        return *this;
      }

      // Post-increment operator.
      const _IteratorTriple
      operator++(int)
      { return _IteratorTriple(_M_first++, _M_second++, _M_third++); }

      // Pre-decrement operator.
      _IteratorTriple&
      operator--()
      {
        --_M_first;
        --_M_second;
        --_M_third;
        return *this;
      }

      // Post-decrement operator.
      const _IteratorTriple
      operator--(int)
      { return _IteratorTriple(_M_first--, _M_second--, _M_third--); }

      // Type conversion.
      operator _Iterator3() const
      { return _M_third; }

      _IteratorTriple&
      operator=(const _IteratorTriple& __other)
      {
        _M_first = __other._M_first;
        _M_second = __other._M_second;
        _M_third = __other._M_third;
        return *this;
      }

      _IteratorTriple
      operator+(difference_type __delta) const
      { return _IteratorTriple(_M_first + __delta, _M_second + __delta,
                               _M_third + __delta); }

      difference_type
      operator-(const _IteratorTriple& __other) const
      { return _M_first - __other._M_first; }
  };
}

#endif /* _GLIBCXX_PARALLEL_ITERATOR_H */
