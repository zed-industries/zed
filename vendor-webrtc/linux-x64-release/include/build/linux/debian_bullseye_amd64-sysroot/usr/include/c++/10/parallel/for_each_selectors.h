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

/** @file parallel/for_each_selectors.h
 *  @brief Functors representing different tasks to be plugged into the
 *  generic parallelization methods for embarrassingly parallel functions.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Felix Putze.

#ifndef _GLIBCXX_PARALLEL_FOR_EACH_SELECTORS_H
#define _GLIBCXX_PARALLEL_FOR_EACH_SELECTORS_H 1

#include <parallel/basic_iterator.h>

namespace __gnu_parallel
{
  /** @brief Generic __selector for embarrassingly parallel functions. */
  template<typename _It>
    struct __generic_for_each_selector
    {
      /** @brief _Iterator on last element processed; needed for some
       *  algorithms (e. g. std::transform()).
       */
      _It _M_finish_iterator;
    };

  /** @brief std::for_each() selector. */
  template<typename _It>
    struct __for_each_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __o Operator.
       *  @param __i iterator referencing object. */
      template<typename _Op>
        bool
        operator()(_Op& __o, _It __i)
        {
          __o(*__i);
          return true;
        }
    };

  /** @brief std::generate() selector. */
  template<typename _It>
    struct __generate_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __o Operator.
       *  @param __i iterator referencing object. */
      template<typename _Op>
        bool
        operator()(_Op& __o, _It __i)
        {
          *__i = __o();
          return true;
        }
    };

  /** @brief std::fill() selector. */
  template<typename _It>
    struct __fill_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __v Current value.
       *  @param __i iterator referencing object. */
      template<typename _ValueType>
        bool
        operator()(_ValueType& __v, _It __i)
        {
          *__i = __v;
          return true;
        }
    };

  /** @brief std::transform() __selector, one input sequence variant. */
  template<typename _It>
    struct __transform1_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __o Operator.
       *  @param __i iterator referencing object. */
      template<typename _Op>
        bool
        operator()(_Op& __o, _It __i)
        {
          *__i.second = __o(*__i.first);
          return true;
        }
    };

  /** @brief std::transform() __selector, two input sequences variant. */
  template<typename _It>
    struct __transform2_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __o Operator.
       *  @param __i iterator referencing object. */
      template<typename _Op>
        bool
        operator()(_Op& __o, _It __i)
        {
          *__i._M_third = __o(*__i._M_first, *__i._M_second);
          return true;
        }
    };

  /** @brief std::replace() selector. */
  template<typename _It, typename _Tp>
    struct __replace_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Value to replace with. */
      const _Tp& __new_val;

      /** @brief Constructor
       *  @param __new_val Value to replace with. */
      explicit
      __replace_selector(const _Tp &__new_val) : __new_val(__new_val) {}

      /** @brief Functor execution.
       *  @param __v Current value.
       *  @param __i iterator referencing object. */
      bool
      operator()(_Tp& __v, _It __i)
      {
        if (*__i == __v)
          *__i = __new_val;
        return true;
      }
    };

  /** @brief std::replace() selector. */
  template<typename _It, typename _Op, typename _Tp>
    struct __replace_if_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Value to replace with. */
      const _Tp& __new_val;

      /** @brief Constructor.
       *  @param __new_val Value to replace with. */
      explicit
      __replace_if_selector(const _Tp &__new_val) : __new_val(__new_val) { }

      /** @brief Functor execution.
       *  @param __o Operator.
       *  @param __i iterator referencing object. */
      bool
      operator()(_Op& __o, _It __i)
      {
        if (__o(*__i))
          *__i = __new_val;
        return true;
      }
    };

  /** @brief std::count() selector. */
  template<typename _It, typename _Diff>
    struct __count_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __v Current value.
       *  @param __i iterator referencing object.
       *  @return 1 if count, 0 if does not count. */
      template<typename _ValueType>
        _Diff
        operator()(_ValueType& __v, _It __i)
        { return (__v == *__i) ? 1 : 0; }
    };

  /** @brief std::count_if () selector. */
  template<typename _It, typename _Diff>
    struct __count_if_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __o Operator.
       *  @param __i iterator referencing object.
       *  @return 1 if count, 0 if does not count. */
      template<typename _Op>
        _Diff
        operator()(_Op& __o, _It __i)
        { return (__o(*__i)) ? 1 : 0; }
    };

  /** @brief std::accumulate() selector. */
  template<typename _It>
    struct __accumulate_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __o Operator (unused).
       *  @param __i iterator referencing object.
       *  @return The current value. */
      template<typename _Op>
        typename std::iterator_traits<_It>::value_type
        operator()(_Op __o, _It __i)
        { return *__i; }
    };

  /** @brief std::inner_product() selector. */
  template<typename _It, typename _It2, typename _Tp>
    struct __inner_product_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Begin iterator of first sequence. */
      _It  __begin1_iterator;

      /** @brief Begin iterator of second sequence. */
      _It2 __begin2_iterator;

      /** @brief Constructor.
       *  @param __b1 Begin iterator of first sequence.
       *  @param __b2 Begin iterator of second sequence. */
      explicit
      __inner_product_selector(_It __b1, _It2 __b2)
      : __begin1_iterator(__b1), __begin2_iterator(__b2) { }

      /** @brief Functor execution.
       *  @param __mult Multiplication functor.
       *  @param __current iterator referencing object.
       *  @return Inner product elemental __result. */
      template<typename _Op>
        _Tp
        operator()(_Op __mult, _It __current)
        {
          typename std::iterator_traits<_It>::difference_type __position
            = __current - __begin1_iterator;
          return __mult(*__current, *(__begin2_iterator + __position));
        }
    };

  /** @brief Selector that just returns the passed iterator. */
  template<typename _It>
    struct __identity_selector : public __generic_for_each_selector<_It>
    {
      /** @brief Functor execution.
       *  @param __o Operator (unused).
       *  @param __i iterator referencing object.
       *  @return Passed iterator. */
      template<typename _Op>
        _It
        operator()(_Op __o, _It __i)
        { return __i; }
    };

  /** @brief Selector that returns the difference between two adjacent
   *  __elements.
   */
  template<typename _It>
    struct __adjacent_difference_selector
    : public __generic_for_each_selector<_It>
    {
      template<typename _Op>
        bool
        operator()(_Op& __o, _It __i)
        {
          typename _It::first_type __go_back_one = __i.first;
          --__go_back_one;
          *__i.second = __o(*__i.first, *__go_back_one);
          return true;
        }
    };

  /** @brief Functor doing nothing
   *
   *  For some __reduction tasks (this is not a function object, but is
   *  passed as __selector __dummy parameter.
   */
  struct _Nothing
  {
    /** @brief Functor execution.
     *  @param __i iterator referencing object. */
    template<typename _It>
      void
      operator()(_It __i) { }
  };

  /** @brief Reduction function doing nothing. */
  struct _DummyReduct
  {
    bool
    operator()(bool, bool) const
    { return true; }
  };

  /** @brief Reduction for finding the maximum element, using a comparator. */
  template<typename _Compare, typename _It>
    struct __min_element_reduct
    {
      _Compare& __comp;

      explicit
      __min_element_reduct(_Compare &__c) : __comp(__c) { }

      _It
      operator()(_It __x, _It __y)
      { return (__comp(*__x, *__y)) ? __x : __y; }
    };

  /** @brief Reduction for finding the maximum element, using a comparator. */
  template<typename _Compare, typename _It>
    struct __max_element_reduct
    {
      _Compare& __comp;

      explicit
      __max_element_reduct(_Compare& __c) : __comp(__c) { }

      _It
      operator()(_It __x, _It __y)
      { return (__comp(*__x, *__y)) ? __y : __x; }
    };

  /** @brief General reduction, using a binary operator. */
  template<typename _BinOp>
    struct __accumulate_binop_reduct
    {
      _BinOp& __binop;

      explicit
      __accumulate_binop_reduct(_BinOp& __b) : __binop(__b) { }

      template<typename _Result, typename _Addend>
        _Result
        operator()(const _Result& __x, const _Addend& __y)
        { return __binop(__x, __y); }
    };
}

#endif /* _GLIBCXX_PARALLEL_FOR_EACH_SELECTORS_H */
