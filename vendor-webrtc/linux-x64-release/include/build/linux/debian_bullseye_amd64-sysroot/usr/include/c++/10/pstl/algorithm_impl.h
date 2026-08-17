// -*- C++ -*-
//===-- algorithm_impl.h --------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_ALGORITHM_IMPL_H
#define _PSTL_ALGORITHM_IMPL_H

#include <iterator>
#include <type_traits>
#include <utility>
#include <functional>
#include <algorithm>

#include "execution_impl.h"
#include "memory_impl.h"
#include "parallel_backend_utils.h"
#include "parallel_backend.h"
#include "parallel_impl.h"
#include "unseq_backend_simd.h"


namespace __pstl
{
namespace __internal
{

//------------------------------------------------------------------------
// any_of
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Pred>
bool
__brick_any_of(const _ForwardIterator __first, const _ForwardIterator __last, _Pred __pred,
               /*__is_vector=*/std::false_type) noexcept
{
    return std::any_of(__first, __last, __pred);
};

template <class _ForwardIterator, class _Pred>
bool
__brick_any_of(const _ForwardIterator __first, const _ForwardIterator __last, _Pred __pred,
               /*__is_vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_or(__first, __last - __first, __pred);
};

template <class _ExecutionPolicy, class _ForwardIterator, class _Pred, class _IsVector>
bool
__pattern_any_of(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Pred __pred,
                 _IsVector __is_vector, /*parallel=*/std::false_type) noexcept
{
    return __internal::__brick_any_of(__first, __last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Pred, class _IsVector>
bool
__pattern_any_of(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Pred __pred,
                 _IsVector __is_vector, /*parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        return __internal::__parallel_or(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                         [__pred, __is_vector](_ForwardIterator __i, _ForwardIterator __j) {
                                             return __internal::__brick_any_of(__i, __j, __pred, __is_vector);
                                         });
    });
}

// [alg.foreach]
// for_each_n with no policy

template <class _ForwardIterator, class _Size, class _Function>
_ForwardIterator
__for_each_n_it_serial(_ForwardIterator __first, _Size __n, _Function __f)
{
    for (; __n > 0; ++__first, --__n)
        __f(__first);
    return __first;
}

//------------------------------------------------------------------------
// walk1 (pseudo)
//
// walk1 evaluates f(x) for each dereferenced value x drawn from [first,last)
//------------------------------------------------------------------------
template <class _ForwardIterator, class _Function>
void
__brick_walk1(_ForwardIterator __first, _ForwardIterator __last, _Function __f, /*vector=*/std::false_type) noexcept
{
    std::for_each(__first, __last, __f);
}

template <class _RandomAccessIterator, class _Function>
void
__brick_walk1(_RandomAccessIterator __first, _RandomAccessIterator __last, _Function __f,
              /*vector=*/std::true_type) noexcept
{
    __unseq_backend::__simd_walk_1(__first, __last - __first, __f);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Function, class _IsVector>
void
__pattern_walk1(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Function __f,
                _IsVector __is_vector,
                /*parallel=*/std::false_type) noexcept
{
    __internal::__brick_walk1(__first, __last, __f, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Function, class _IsVector>
void
__pattern_walk1(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Function __f,
                _IsVector __is_vector,
                /*parallel=*/std::true_type)
{
    __internal::__except_handler([&]() {
        __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                      [__f, __is_vector](_ForwardIterator __i, _ForwardIterator __j) {
                                          __internal::__brick_walk1(__i, __j, __f, __is_vector);
                                      });
    });
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Brick>
void
__pattern_walk_brick(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Brick __brick,
                     /*parallel=*/std::false_type) noexcept
{
    __brick(__first, __last);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Brick>
void
__pattern_walk_brick(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Brick __brick,
                     /*parallel=*/std::true_type)
{
    __internal::__except_handler([&]() {
        __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                      [__brick](_ForwardIterator __i, _ForwardIterator __j) { __brick(__i, __j); });
    });
}

//------------------------------------------------------------------------
// walk1_n
//------------------------------------------------------------------------
template <class _ForwardIterator, class _Size, class _Function>
_ForwardIterator
__brick_walk1_n(_ForwardIterator __first, _Size __n, _Function __f, /*_IsVectorTag=*/std::false_type)
{
    return __internal::__for_each_n_it_serial(__first, __n,
                                              [&__f](_ForwardIterator __it) { __f(*__it); }); // calling serial version
}

template <class _RandomAccessIterator, class _DifferenceType, class _Function>
_RandomAccessIterator
__brick_walk1_n(_RandomAccessIterator __first, _DifferenceType __n, _Function __f,
                /*vectorTag=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_walk_1(__first, __n, __f);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Function, class _IsVector>
_ForwardIterator
__pattern_walk1_n(_ExecutionPolicy&&, _ForwardIterator __first, _Size __n, _Function __f, _IsVector __is_vector,
                  /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_walk1_n(__first, __n, __f, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Size, class _Function, class _IsVector>
_RandomAccessIterator
__pattern_walk1_n(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _Size __n, _Function __f,
                  _IsVector __is_vector,
                  /*is_parallel=*/std::true_type)
{
    __internal::__pattern_walk1(std::forward<_ExecutionPolicy>(__exec), __first, __first + __n, __f, __is_vector,
                                std::true_type());
    return __first + __n;
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Brick>
_ForwardIterator
__pattern_walk_brick_n(_ExecutionPolicy&&, _ForwardIterator __first, _Size __n, _Brick __brick,
                       /*is_parallel=*/std::false_type) noexcept
{
    return __brick(__first, __n);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Size, class _Brick>
_RandomAccessIterator
__pattern_walk_brick_n(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _Size __n, _Brick __brick,
                       /*is_parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        __par_backend::__parallel_for(
            std::forward<_ExecutionPolicy>(__exec), __first, __first + __n,
            [__brick](_RandomAccessIterator __i, _RandomAccessIterator __j) { __brick(__i, __j - __i); });
        return __first + __n;
    });
}

//------------------------------------------------------------------------
// walk2 (pseudo)
//
// walk2 evaluates f(x,y) for deferenced values (x,y) drawn from [first1,last1) and [first2,...)
//------------------------------------------------------------------------
template <class _ForwardIterator1, class _ForwardIterator2, class _Function>
_ForwardIterator2
__brick_walk2(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2, _Function __f,
              /*vector=*/std::false_type) noexcept
{
    for (; __first1 != __last1; ++__first1, ++__first2)
        __f(*__first1, *__first2);
    return __first2;
}

template <class _ForwardIterator1, class _ForwardIterator2, class _Function>
_ForwardIterator2
__brick_walk2(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2, _Function __f,
              /*vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_walk_2(__first1, __last1 - __first1, __first2, __f);
}

template <class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Function>
_ForwardIterator2
__brick_walk2_n(_ForwardIterator1 __first1, _Size __n, _ForwardIterator2 __first2, _Function __f,
                /*vector=*/std::false_type) noexcept
{
    for (; __n > 0; --__n, ++__first1, ++__first2)
        __f(*__first1, *__first2);
    return __first2;
}

template <class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Function>
_ForwardIterator2
__brick_walk2_n(_ForwardIterator1 __first1, _Size __n, _ForwardIterator2 __first2, _Function __f,
                /*vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_walk_2(__first1, __n, __first2, __f);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Function, class _IsVector>
_ForwardIterator2
__pattern_walk2(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                _Function __f, _IsVector __is_vector, /*parallel=*/std::false_type) noexcept
{
    return __internal::__brick_walk2(__first1, __last1, __first2, __f, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Function, class _IsVector>
_ForwardIterator2
__pattern_walk2(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                _ForwardIterator2 __first2, _Function __f, _IsVector __is_vector, /*parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        __par_backend::__parallel_for(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1,
            [__f, __first1, __first2, __is_vector](_ForwardIterator1 __i, _ForwardIterator1 __j) {
                __internal::__brick_walk2(__i, __j, __first2 + (__i - __first1), __f, __is_vector);
            });
        return __first2 + (__last1 - __first1);
    });
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Function,
          class _IsVector>
_ForwardIterator2
__pattern_walk2_n(_ExecutionPolicy&&, _ForwardIterator1 __first1, _Size __n, _ForwardIterator2 __first2, _Function __f,
                  _IsVector __is_vector, /*parallel=*/std::false_type) noexcept
{
    return __internal::__brick_walk2_n(__first1, __n, __first2, __f, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _Size, class _RandomAccessIterator2,
          class _Function, class _IsVector>
_RandomAccessIterator2
__pattern_walk2_n(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _Size __n, _RandomAccessIterator2 __first2,
                  _Function __f, _IsVector __is_vector, /*parallel=*/std::true_type)
{
    return __internal::__pattern_walk2(std::forward<_ExecutionPolicy>(__exec), __first1, __first1 + __n, __first2, __f,
                                       __is_vector, std::true_type());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Brick>
_ForwardIterator2
__pattern_walk2_brick(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                      _ForwardIterator2 __first2, _Brick __brick, /*parallel=*/std::false_type) noexcept
{
    return __brick(__first1, __last1, __first2);
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _Brick>
_RandomAccessIterator2
__pattern_walk2_brick(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                      _RandomAccessIterator2 __first2, _Brick __brick, /*parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        __par_backend::__parallel_for(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1,
            [__first1, __first2, __brick](_RandomAccessIterator1 __i, _RandomAccessIterator1 __j) {
                __brick(__i, __j, __first2 + (__i - __first1));
            });
        return __first2 + (__last1 - __first1);
    });
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _Size, class _RandomAccessIterator2, class _Brick>
_RandomAccessIterator2
__pattern_walk2_brick_n(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _Size __n,
                        _RandomAccessIterator2 __first2, _Brick __brick, /*parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        __par_backend::__parallel_for(
            std::forward<_ExecutionPolicy>(__exec), __first1, __first1 + __n,
            [__first1, __first2, __brick](_RandomAccessIterator1 __i, _RandomAccessIterator1 __j) {
                __brick(__i, __j - __i, __first2 + (__i - __first1));
            });
        return __first2 + __n;
    });
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Brick>
_ForwardIterator2
__pattern_walk2_brick_n(_ExecutionPolicy&&, _ForwardIterator1 __first1, _Size __n, _ForwardIterator2 __first2,
                        _Brick __brick, /*parallel=*/std::false_type) noexcept
{
    return __brick(__first1, __n, __first2);
}

//------------------------------------------------------------------------
// walk3 (pseudo)
//
// walk3 evaluates f(x,y,z) for (x,y,z) drawn from [first1,last1), [first2,...), [first3,...)
//------------------------------------------------------------------------
template <class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator3, class _Function>
_ForwardIterator3
__brick_walk3(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
              _ForwardIterator3 __first3, _Function __f, /*vector=*/std::false_type) noexcept
{
    for (; __first1 != __last1; ++__first1, ++__first2, ++__first3)
        __f(*__first1, *__first2, *__first3);
    return __first3;
}

template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _RandomAccessIterator3, class _Function>
_RandomAccessIterator3
__brick_walk3(_RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1, _RandomAccessIterator2 __first2,
              _RandomAccessIterator3 __first3, _Function __f, /*vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_walk_3(__first1, __last1 - __first1, __first2, __first3, __f);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator3,
          class _Function, class _IsVector>
_ForwardIterator3
__pattern_walk3(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                _ForwardIterator3 __first3, _Function __f, _IsVector __is_vector, /*parallel=*/std::false_type) noexcept
{
    return __internal::__brick_walk3(__first1, __last1, __first2, __first3, __f, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2,
          class _RandomAccessIterator3, class _Function, class _IsVector>
_RandomAccessIterator3
__pattern_walk3(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                _RandomAccessIterator2 __first2, _RandomAccessIterator3 __first3, _Function __f, _IsVector __is_vector,
                /*parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        __par_backend::__parallel_for(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1,
            [__f, __first1, __first2, __first3, __is_vector](_RandomAccessIterator1 __i, _RandomAccessIterator1 __j) {
                __internal::__brick_walk3(__i, __j, __first2 + (__i - __first1), __first3 + (__i - __first1), __f,
                                          __is_vector);
            });
        return __first3 + (__last1 - __first1);
    });
}

//------------------------------------------------------------------------
// equal
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
bool
__brick_equal(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
              _ForwardIterator2 __last2, _BinaryPredicate __p, /* IsVector = */ std::false_type) noexcept
{
    return std::equal(__first1, __last1, __first2, __last2, __p);
}

template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate>
bool
__brick_equal(_RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1, _RandomAccessIterator2 __first2,
              _RandomAccessIterator2 __last2, _BinaryPredicate __p, /* is_vector = */ std::true_type) noexcept
{
    if (__last1 - __first1 != __last2 - __first2)
        return false;

    return __unseq_backend::__simd_first(__first1, __last1 - __first1, __first2,
                                         __internal::__not_pred<_BinaryPredicate>(__p))
               .first == __last1;
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                _ForwardIterator2 __last2, _BinaryPredicate __p, _IsVector __is_vector, /* is_parallel = */
                std::false_type) noexcept
{
    return __internal::__brick_equal(__first1, __last1, __first2, __last2, __p, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                _RandomAccessIterator2 __first2, _RandomAccessIterator2 __last2, _BinaryPredicate __p,
                _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    if (__last1 - __first1 != __last2 - __first2)
        return false;

    return __internal::__except_handler([&]() {
        return !__internal::__parallel_or(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1,
            [__first1, __first2, __p, __is_vector](_RandomAccessIterator1 __i, _RandomAccessIterator1 __j) {
                return !__internal::__brick_equal(__i, __j, __first2 + (__i - __first1), __first2 + (__j - __first1),
                                                  __p, __is_vector);
            });
    });
}

//------------------------------------------------------------------------
// equal version for sequences with equal length
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
bool
__brick_equal(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2, _BinaryPredicate __p,
              /* IsVector = */ std::false_type) noexcept
{
    return std::equal(__first1, __last1, __first2, __p);
}

template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate>
bool
__brick_equal(_RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1, _RandomAccessIterator2 __first2,
              _BinaryPredicate __p, /* is_vector = */ std::true_type) noexcept
{
    return __unseq_backend::__simd_first(__first1, __last1 - __first1, __first2, __not_pred<_BinaryPredicate>(__p))
               .first == __last1;
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                _BinaryPredicate __p, _IsVector __is_vector, /* is_parallel = */ std::false_type) noexcept
{
    return __internal::__brick_equal(__first1, __last1, __first2, __p, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                _RandomAccessIterator2 __first2, _BinaryPredicate __p, _IsVector __is_vector,
                /*is_parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        return !__internal::__parallel_or(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1,
            [__first1, __first2, __p, __is_vector](_RandomAccessIterator1 __i, _RandomAccessIterator1 __j) {
                return !__internal::__brick_equal(__i, __j, __first2 + (__i - __first1), __p, __is_vector);
            });
    });
}

//------------------------------------------------------------------------
// find_if
//------------------------------------------------------------------------
template <class _ForwardIterator, class _Predicate>
_ForwardIterator
__brick_find_if(_ForwardIterator __first, _ForwardIterator __last, _Predicate __pred,
                /*is_vector=*/std::false_type) noexcept
{
    return std::find_if(__first, __last, __pred);
}

template <class _RandomAccessIterator, class _Predicate>
_RandomAccessIterator
__brick_find_if(_RandomAccessIterator __first, _RandomAccessIterator __last, _Predicate __pred,
                /*is_vector=*/std::true_type) noexcept
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::difference_type _SizeType;
    return __unseq_backend::__simd_first(
        __first, _SizeType(0), __last - __first,
        [&__pred](_RandomAccessIterator __it, _SizeType __i) { return __pred(__it[__i]); });
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
_ForwardIterator
__pattern_find_if(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred,
                  _IsVector __is_vector,
                  /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_find_if(__first, __last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
_ForwardIterator
__pattern_find_if(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred,
                  _IsVector __is_vector,
                  /*is_parallel=*/std::true_type)
{
    return __internal::__except_handler([&]() {
        return __internal::__parallel_find(
            std::forward<_ExecutionPolicy>(__exec), __first, __last,
            [__pred, __is_vector](_ForwardIterator __i, _ForwardIterator __j) {
                return __internal::__brick_find_if(__i, __j, __pred, __is_vector);
            },
            std::less<typename std::iterator_traits<_ForwardIterator>::difference_type>(),
            /*is_first=*/true);
    });
}

//------------------------------------------------------------------------
// find_end
//------------------------------------------------------------------------

// find the first occurrence of the subsequence [s_first, s_last)
//   or the  last occurrence of the subsequence in the range [first, last)
// b_first determines what occurrence we want to find (first or last)
template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate, class _IsVector>
_RandomAccessIterator1
__find_subrange(_RandomAccessIterator1 __first, _RandomAccessIterator1 __last, _RandomAccessIterator1 __global_last,
                _RandomAccessIterator2 __s_first, _RandomAccessIterator2 __s_last, _BinaryPredicate __pred,
                bool __b_first, _IsVector __is_vector) noexcept
{
    typedef typename std::iterator_traits<_RandomAccessIterator2>::value_type _ValueType;
    auto __n2 = __s_last - __s_first;
    if (__n2 < 1)
    {
        return __b_first ? __first : __last;
    }

    auto __n1 = __global_last - __first;
    if (__n1 < __n2)
    {
        return __last;
    }

    auto __cur = __last;
    while (__first != __last && (__global_last - __first >= __n2))
    {
        // find position of *s_first in [first, last) (it can be start of subsequence)
        __first = __internal::__brick_find_if(
            __first, __last, __equal_value_by_pred<_ValueType, _BinaryPredicate>(*__s_first, __pred), __is_vector);

        // if position that was found previously is the start of subsequence
        // then we can exit the loop (b_first == true) or keep the position
        // (b_first == false)
        if (__first != __last && (__global_last - __first >= __n2) &&
            __internal::__brick_equal(__s_first + 1, __s_last, __first + 1, __pred, __is_vector))
        {
            if (__b_first)
            {
                return __first;
            }
            else
            {
                __cur = __first;
            }
        }
        else if (__first == __last)
        {
            break;
        }
        else
        {
        }

        // in case of b_first == false we try to find new start position
        // for the next subsequence
        ++__first;
    }
    return __cur;
}

template <class _RandomAccessIterator, class _Size, class _Tp, class _BinaryPredicate, class _IsVector>
_RandomAccessIterator
__find_subrange(_RandomAccessIterator __first, _RandomAccessIterator __last, _RandomAccessIterator __global_last,
                _Size __count, const _Tp& __value, _BinaryPredicate __pred, _IsVector __is_vector) noexcept
{
    if (__global_last - __first < __count || __count < 1)
    {
        return __last; // According to the standard last shall be returned when count < 1
    }

    auto __n = __global_last - __first;
    auto __unary_pred = __equal_value_by_pred<_Tp, _BinaryPredicate>(__value, __pred);
    while (__first != __last && (__global_last - __first >= __count))
    {
        __first = __internal::__brick_find_if(__first, __last, __unary_pred, __is_vector);

        // check that all of elements in [first+1, first+count) equal to value
        if (__first != __last && (__global_last - __first >= __count) &&
            !__internal::__brick_any_of(__first + 1, __first + __count,
                                        __not_pred<decltype(__unary_pred)>(__unary_pred), __is_vector))
        {
            return __first;
        }
        else if (__first == __last)
        {
            break;
        }
        else
        {
            ++__first;
        }
    }
    return __last;
}

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1
__brick_find_end(_ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
                 _ForwardIterator2 __s_last, _BinaryPredicate __pred, /*__is_vector=*/std::false_type) noexcept
{
    return std::find_end(__first, __last, __s_first, __s_last, __pred);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1
__brick_find_end(_ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
                 _ForwardIterator2 __s_last, _BinaryPredicate __pred, /*__is_vector=*/std::true_type) noexcept
{
    return __find_subrange(__first, __last, __last, __s_first, __s_last, __pred, false, std::true_type());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_end(_ExecutionPolicy&&, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
                   _ForwardIterator2 __s_last, _BinaryPredicate __pred, _IsVector __is_vector,
                   /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_find_end(__first, __last, __s_first, __s_last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_end(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                   _ForwardIterator2 __s_first, _ForwardIterator2 __s_last, _BinaryPredicate __pred,
                   _IsVector __is_vector, /*is_parallel=*/std::true_type) noexcept
{
    if (__last - __first == __s_last - __s_first)
    {
        const bool __res = __internal::__pattern_equal(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                                       __s_first, __pred, __is_vector, std::true_type());
        return __res ? __first : __last;
    }
    else
    {
        return __internal::__except_handler([&]() {
            return __internal::__parallel_find(
                std::forward<_ExecutionPolicy>(__exec), __first, __last,
                [__last, __s_first, __s_last, __pred, __is_vector](_ForwardIterator1 __i, _ForwardIterator1 __j) {
                    return __internal::__find_subrange(__i, __j, __last, __s_first, __s_last, __pred, false,
                                                       __is_vector);
                },
                std::greater<typename std::iterator_traits<_ForwardIterator1>::difference_type>(), /*is_first=*/false);
        });
    }
}

//------------------------------------------------------------------------
// find_first_of
//------------------------------------------------------------------------
template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1
__brick_find_first_of(_ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
                      _ForwardIterator2 __s_last, _BinaryPredicate __pred, /*__is_vector=*/std::false_type) noexcept
{
    return std::find_first_of(__first, __last, __s_first, __s_last, __pred);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1
__brick_find_first_of(_ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
                      _ForwardIterator2 __s_last, _BinaryPredicate __pred, /*__is_vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_find_first_of(__first, __last, __s_first, __s_last, __pred);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_first_of(_ExecutionPolicy&&, _ForwardIterator1 __first, _ForwardIterator1 __last,
                        _ForwardIterator2 __s_first, _ForwardIterator2 __s_last, _BinaryPredicate __pred,
                        _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_find_first_of(__first, __last, __s_first, __s_last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_first_of(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                        _ForwardIterator2 __s_first, _ForwardIterator2 __s_last, _BinaryPredicate __pred,
                        _IsVector __is_vector, /*is_parallel=*/std::true_type) noexcept
{
    return __internal::__except_handler([&]() {
        return __internal::__parallel_find(
            std::forward<_ExecutionPolicy>(__exec), __first, __last,
            [__s_first, __s_last, __pred, __is_vector](_ForwardIterator1 __i, _ForwardIterator1 __j) {
                return __internal::__brick_find_first_of(__i, __j, __s_first, __s_last, __pred, __is_vector);
            },
            std::less<typename std::iterator_traits<_ForwardIterator1>::difference_type>(), /*is_first=*/true);
    });
}

//------------------------------------------------------------------------
// search
//------------------------------------------------------------------------
template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1
__brick_search(_ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
               _ForwardIterator2 __s_last, _BinaryPredicate __pred, /*vector=*/std::false_type) noexcept
{
    return std::search(__first, __last, __s_first, __s_last, __pred);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1
__brick_search(_ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
               _ForwardIterator2 __s_last, _BinaryPredicate __pred, /*vector=*/std::true_type) noexcept
{
    return __internal::__find_subrange(__first, __last, __last, __s_first, __s_last, __pred, true, std::true_type());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_search(_ExecutionPolicy&&, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
                 _ForwardIterator2 __s_last, _BinaryPredicate __pred, _IsVector __is_vector,
                 /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_search(__first, __last, __s_first, __s_last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_search(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                 _ForwardIterator2 __s_first, _ForwardIterator2 __s_last, _BinaryPredicate __pred,
                 _IsVector __is_vector,
                 /*is_parallel=*/std::true_type) noexcept
{
    if (__last - __first == __s_last - __s_first)
    {
        const bool __res = __internal::__pattern_equal(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                                       __s_first, __pred, __is_vector, std::true_type());
        return __res ? __first : __last;
    }
    else
    {
        return __internal::__except_handler([&]() {
            return __internal::__parallel_find(
                std::forward<_ExecutionPolicy>(__exec), __first, __last,
                [__last, __s_first, __s_last, __pred, __is_vector](_ForwardIterator1 __i, _ForwardIterator1 __j) {
                    return __internal::__find_subrange(__i, __j, __last, __s_first, __s_last, __pred, true,
                                                       __is_vector);
                },
                std::less<typename std::iterator_traits<_ForwardIterator1>::difference_type>(), /*is_first=*/true);
        });
    }
}

//------------------------------------------------------------------------
// search_n
//------------------------------------------------------------------------
template <class _ForwardIterator, class _Size, class _Tp, class _BinaryPredicate>
_ForwardIterator
__brick_search_n(_ForwardIterator __first, _ForwardIterator __last, _Size __count, const _Tp& __value,
                 _BinaryPredicate __pred, /*vector=*/std::false_type) noexcept
{
    return std::search_n(__first, __last, __count, __value, __pred);
}

template <class _ForwardIterator, class _Size, class _Tp, class _BinaryPredicate>
_ForwardIterator
__brick_search_n(_ForwardIterator __first, _ForwardIterator __last, _Size __count, const _Tp& __value,
                 _BinaryPredicate __pred, /*vector=*/std::true_type) noexcept
{
    return __internal::__find_subrange(__first, __last, __last, __count, __value, __pred, std::true_type());
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Tp, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator
__pattern_search_n(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Size __count,
                   const _Tp& __value, _BinaryPredicate __pred, _IsVector __is_vector,
                   /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_search_n(__first, __last, __count, __value, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Size, class _Tp, class _BinaryPredicate,
          class _IsVector>
_RandomAccessIterator
__pattern_search_n(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                   _Size __count, const _Tp& __value, _BinaryPredicate __pred, _IsVector __is_vector,
                   /*is_parallel=*/std::true_type) noexcept
{
    if (__last - __first == __count)
    {
        const bool __result = !__internal::__pattern_any_of(
            std::forward<_ExecutionPolicy>(__exec), __first, __last,
            [&__value, &__pred](const _Tp& __val) { return !__pred(__val, __value); }, __is_vector,
            /*is_parallel*/ std::true_type());
        return __result ? __first : __last;
    }
    else
    {
        return __internal::__except_handler([&__exec, __first, __last, __count, &__value, __pred, __is_vector]() {
            return __internal::__parallel_find(
                std::forward<_ExecutionPolicy>(__exec), __first, __last,
                [__last, __count, &__value, __pred, __is_vector](_RandomAccessIterator __i, _RandomAccessIterator __j) {
                    return __internal::__find_subrange(__i, __j, __last, __count, __value, __pred, __is_vector);
                },
                std::less<typename std::iterator_traits<_RandomAccessIterator>::difference_type>(), /*is_first=*/true);
        });
    }
}

//------------------------------------------------------------------------
// copy_n
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Size, class _OutputIterator>
_OutputIterator
__brick_copy_n(_ForwardIterator __first, _Size __n, _OutputIterator __result, /*vector=*/std::false_type) noexcept
{
    return std::copy_n(__first, __n, __result);
}

template <class _ForwardIterator, class _Size, class _OutputIterator>
_OutputIterator
__brick_copy_n(_ForwardIterator __first, _Size __n, _OutputIterator __result, /*vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_assign(
        __first, __n, __result, [](_ForwardIterator __first, _OutputIterator __result) { *__result = *__first; });
}

//------------------------------------------------------------------------
// copy
//------------------------------------------------------------------------
template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_copy(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
             /*vector=*/std::false_type) noexcept
{
    return std::copy(__first, __last, __result);
}

template <class _RandomAccessIterator, class _OutputIterator>
_OutputIterator
__brick_copy(_RandomAccessIterator __first, _RandomAccessIterator __last, _OutputIterator __result,
             /*vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_assign(
        __first, __last - __first, __result,
        [](_RandomAccessIterator __first, _OutputIterator __result) { *__result = *__first; });
}

//------------------------------------------------------------------------
// move
//------------------------------------------------------------------------
template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_move(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
             /*vector=*/std::false_type) noexcept
{
    return std::move(__first, __last, __result);
}

template <class _RandomAccessIterator, class _OutputIterator>
_OutputIterator
__brick_move(_RandomAccessIterator __first, _RandomAccessIterator __last, _OutputIterator __result,
             /*vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_assign(
        __first, __last - __first, __result,
        [](_RandomAccessIterator __first, _OutputIterator __result) { *__result = std::move(*__first); });
}

//------------------------------------------------------------------------
// swap_ranges
//------------------------------------------------------------------------
template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_swap_ranges(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                    /*vector=*/std::false_type) noexcept
{
    return std::swap_ranges(__first, __last, __result);
}

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_swap_ranges(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                    /*vector=*/std::true_type) noexcept
{
    using std::iter_swap;
    return __unseq_backend::__simd_assign(__first, __last - __first, __result,
                                          iter_swap<_ForwardIterator, _OutputIterator>);
}

//------------------------------------------------------------------------
// copy_if
//------------------------------------------------------------------------
template <class _ForwardIterator, class _OutputIterator, class _UnaryPredicate>
_OutputIterator
__brick_copy_if(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result, _UnaryPredicate __pred,
                /*vector=*/std::false_type) noexcept
{
    return std::copy_if(__first, __last, __result, __pred);
}

template <class _ForwardIterator, class _OutputIterator, class _UnaryPredicate>
_OutputIterator
__brick_copy_if(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result, _UnaryPredicate __pred,
                /*vector=*/std::true_type) noexcept
{
#if (_PSTL_MONOTONIC_PRESENT)
    return __unseq_backend::__simd_copy_if(__first, __last - __first, __result, __pred);
#else
    return std::copy_if(__first, __last, __result, __pred);
#endif
}

// TODO: Try to use transform_reduce for combining __brick_copy_if_phase1 on IsVector.
template <class _DifferenceType, class _ForwardIterator, class _UnaryPredicate>
std::pair<_DifferenceType, _DifferenceType>
__brick_calc_mask_1(_ForwardIterator __first, _ForwardIterator __last, bool* __restrict __mask, _UnaryPredicate __pred,
                    /*vector=*/std::false_type) noexcept
{
    auto __count_true = _DifferenceType(0);
    auto __size = __last - __first;

    static_assert(__is_random_access_iterator<_ForwardIterator>::value,
                  "Pattern-brick error. Should be a random access iterator.");

    for (; __first != __last; ++__first, ++__mask)
    {
        *__mask = __pred(*__first);
        if (*__mask)
        {
            ++__count_true;
        }
    }
    return std::make_pair(__count_true, __size - __count_true);
}

template <class _DifferenceType, class _RandomAccessIterator, class _UnaryPredicate>
std::pair<_DifferenceType, _DifferenceType>
__brick_calc_mask_1(_RandomAccessIterator __first, _RandomAccessIterator __last, bool* __mask, _UnaryPredicate __pred,
                    /*vector=*/std::true_type) noexcept
{
    auto __result = __unseq_backend::__simd_calc_mask_1(__first, __last - __first, __mask, __pred);
    return std::make_pair(__result, (__last - __first) - __result);
}

template <class _ForwardIterator, class _OutputIterator, class _Assigner>
void
__brick_copy_by_mask(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result, bool* __mask,
                     _Assigner __assigner, /*vector=*/std::false_type) noexcept
{
    for (; __first != __last; ++__first, ++__mask)
    {
        if (*__mask)
        {
            __assigner(__first, __result);
            ++__result;
        }
    }
}

template <class _ForwardIterator, class _OutputIterator, class _Assigner>
void
__brick_copy_by_mask(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                     bool* __restrict __mask, _Assigner __assigner, /*vector=*/std::true_type) noexcept
{
#if (_PSTL_MONOTONIC_PRESENT)
    __unseq_backend::__simd_copy_by_mask(__first, __last - __first, __result, __mask, __assigner);
#else
    __internal::__brick_copy_by_mask(__first, __last, __result, __mask, __assigner, std::false_type());
#endif
}

template <class _ForwardIterator, class _OutputIterator1, class _OutputIterator2>
void
__brick_partition_by_mask(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator1 __out_true,
                          _OutputIterator2 __out_false, bool* __mask, /*vector=*/std::false_type) noexcept
{
    for (; __first != __last; ++__first, ++__mask)
    {
        if (*__mask)
        {
            *__out_true = *__first;
            ++__out_true;
        }
        else
        {
            *__out_false = *__first;
            ++__out_false;
        }
    }
}

template <class _RandomAccessIterator, class _OutputIterator1, class _OutputIterator2>
void
__brick_partition_by_mask(_RandomAccessIterator __first, _RandomAccessIterator __last, _OutputIterator1 __out_true,
                          _OutputIterator2 __out_false, bool* __mask, /*vector=*/std::true_type) noexcept
{
#if (_PSTL_MONOTONIC_PRESENT)
    __unseq_backend::__simd_partition_by_mask(__first, __last - __first, __out_true, __out_false, __mask);
#else
    __internal::__brick_partition_by_mask(__first, __last, __out_true, __out_false, __mask, std::false_type());
#endif
}

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _UnaryPredicate, class _IsVector>
_OutputIterator
__pattern_copy_if(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                  _UnaryPredicate __pred, _IsVector __is_vector, /*parallel=*/std::false_type) noexcept
{
    return __internal::__brick_copy_if(__first, __last, __result, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator, class _UnaryPredicate,
          class _IsVector>
_OutputIterator
__pattern_copy_if(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                  _OutputIterator __result, _UnaryPredicate __pred, _IsVector __is_vector, /*parallel=*/std::true_type)
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::difference_type _DifferenceType;
    const _DifferenceType __n = __last - __first;
    if (_DifferenceType(1) < __n)
    {
        __par_backend::__buffer<bool> __mask_buf(__n);
        return __internal::__except_handler([&__exec, __n, __first, __result, __is_vector, __pred, &__mask_buf]() {
            bool* __mask = __mask_buf.get();
            _DifferenceType __m{};
            __par_backend::__parallel_strict_scan(
                std::forward<_ExecutionPolicy>(__exec), __n, _DifferenceType(0),
                [=](_DifferenceType __i, _DifferenceType __len) { // Reduce
                    return __internal::__brick_calc_mask_1<_DifferenceType>(__first + __i, __first + (__i + __len),
                                                                            __mask + __i, __pred, __is_vector)
                        .first;
                },
                std::plus<_DifferenceType>(),                                                // Combine
                [=](_DifferenceType __i, _DifferenceType __len, _DifferenceType __initial) { // Scan
                    __internal::__brick_copy_by_mask(
                        __first + __i, __first + (__i + __len), __result + __initial, __mask + __i,
                        [](_RandomAccessIterator __x, _OutputIterator __z) { *__z = *__x; }, __is_vector);
                },
                [&__m](_DifferenceType __total) { __m = __total; });
            return __result + __m;
        });
    }
    // trivial sequence - use serial algorithm
    return __internal::__brick_copy_if(__first, __last, __result, __pred, __is_vector);
}

//------------------------------------------------------------------------
// count
//------------------------------------------------------------------------
template <class _ForwardIterator, class _Predicate>
typename std::iterator_traits<_ForwardIterator>::difference_type
__brick_count(_ForwardIterator __first, _ForwardIterator __last, _Predicate __pred,
              /* is_vector = */ std::true_type) noexcept
{
    return __unseq_backend::__simd_count(__first, __last - __first, __pred);
}

template <class _ForwardIterator, class _Predicate>
typename std::iterator_traits<_ForwardIterator>::difference_type
__brick_count(_ForwardIterator __first, _ForwardIterator __last, _Predicate __pred,
              /* is_vector = */ std::false_type) noexcept
{
    return std::count_if(__first, __last, __pred);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
typename std::iterator_traits<_ForwardIterator>::difference_type
__pattern_count(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred,
                /* is_parallel */ std::false_type, _IsVector __is_vector) noexcept
{
    return __internal::__brick_count(__first, __last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
typename std::iterator_traits<_ForwardIterator>::difference_type
__pattern_count(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred,
                /* is_parallel */ std::true_type, _IsVector __is_vector)
{
    typedef typename std::iterator_traits<_ForwardIterator>::difference_type _SizeType;
    return __internal::__except_handler([&]() {
        return __par_backend::__parallel_reduce(
            std::forward<_ExecutionPolicy>(__exec), __first, __last, _SizeType(0),
            [__pred, __is_vector](_ForwardIterator __begin, _ForwardIterator __end, _SizeType __value) -> _SizeType {
                return __value + __internal::__brick_count(__begin, __end, __pred, __is_vector);
            },
            std::plus<_SizeType>());
    });
}

//------------------------------------------------------------------------
// unique
//------------------------------------------------------------------------

template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator
__brick_unique(_ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred,
               /*is_vector=*/std::false_type) noexcept
{
    return std::unique(__first, __last, __pred);
}

template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator
__brick_unique(_ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred,
               /*is_vector=*/std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::unique(__first, __last, __pred);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate, class _IsVector>
_ForwardIterator
__pattern_unique(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred,
                 _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_unique(__first, __last, __pred, __is_vector);
}

// That function is shared between two algorithms - remove_if (__pattern_remove_if) and unique (pattern unique). But a mask calculation is different.
// So, a caller passes _CalcMask brick into remove_elements.
template <class _ExecutionPolicy, class _ForwardIterator, class _CalcMask, class _IsVector>
_ForwardIterator
__remove_elements(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _CalcMask __calc_mask,
                  _IsVector __is_vector)
{
    typedef typename std::iterator_traits<_ForwardIterator>::difference_type _DifferenceType;
    typedef typename std::iterator_traits<_ForwardIterator>::value_type _Tp;
    _DifferenceType __n = __last - __first;
    __par_backend::__buffer<bool> __mask_buf(__n);
    // 1. find a first iterator that should be removed
    return __internal::__except_handler([&]() {
        bool* __mask = __mask_buf.get();
        _DifferenceType __min = __par_backend::__parallel_reduce(
            std::forward<_ExecutionPolicy>(__exec), _DifferenceType(0), __n, __n,
            [__first, __mask, &__calc_mask, __is_vector](_DifferenceType __i, _DifferenceType __j,
                                                         _DifferenceType __local_min) -> _DifferenceType {
                // Create mask
                __calc_mask(__mask + __i, __mask + __j, __first + __i);

                // if minimum was found in a previous range we shouldn't do anymore
                if (__local_min < __i)
                {
                    return __local_min;
                }
                // find first iterator that should be removed
                bool* __result = __internal::__brick_find_if(__mask + __i, __mask + __j,
                                                             [](bool __val) { return !__val; }, __is_vector);
                if (__result - __mask == __j)
                {
                    return __local_min;
                }
                return std::min(__local_min, _DifferenceType(__result - __mask));
            },
            [](_DifferenceType __local_min1, _DifferenceType __local_min2) -> _DifferenceType {
                return std::min(__local_min1, __local_min2);
            });

        // No elements to remove - exit
        if (__min == __n)
        {
            return __last;
        }
        __n -= __min;
        __first += __min;

        __par_backend::__buffer<_Tp> __buf(__n);
        _Tp* __result = __buf.get();
        __mask += __min;
        _DifferenceType __m{};
        // 2. Elements that doesn't satisfy pred are moved to result
        __par_backend::__parallel_strict_scan(
            std::forward<_ExecutionPolicy>(__exec), __n, _DifferenceType(0),
            [__mask, __is_vector](_DifferenceType __i, _DifferenceType __len) {
                return __internal::__brick_count(__mask + __i, __mask + __i + __len, [](bool __val) { return __val; },
                                                 __is_vector);
            },
            std::plus<_DifferenceType>(),
            [=](_DifferenceType __i, _DifferenceType __len, _DifferenceType __initial) {
                __internal::__brick_copy_by_mask(
                    __first + __i, __first + __i + __len, __result + __initial, __mask + __i,
                    [](_ForwardIterator __x, _Tp* __z) {
                        __internal::__invoke_if_else(std::is_trivial<_Tp>(), [&]() { *__z = std::move(*__x); },
                                                     [&]() { ::new (std::addressof(*__z)) _Tp(std::move(*__x)); });
                    },
                    __is_vector);
            },
            [&__m](_DifferenceType __total) { __m = __total; });

        // 3. Elements from result are moved to [first, last)
        __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __result, __result + __m,
                                      [__result, __first, __is_vector](_Tp* __i, _Tp* __j) {
                                          __internal::__brick_move(__i, __j, __first + (__i - __result), __is_vector);
                                      });
        return __first + __m;
    });
}

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate, class _IsVector>
_ForwardIterator
__pattern_unique(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred,
                 _IsVector __is_vector, /*is_parallel=*/std::true_type) noexcept
{
    typedef typename std::iterator_traits<_ForwardIterator>::reference _ReferenceType;

    if (__first == __last)
    {
        return __last;
    }
    if (__first + 1 == __last || __first + 2 == __last)
    {
        // Trivial sequence - use serial algorithm
        return __internal::__brick_unique(__first, __last, __pred, __is_vector);
    }
    return __internal::__remove_elements(
        std::forward<_ExecutionPolicy>(__exec), ++__first, __last,
        [&__pred, __is_vector](bool* __b, bool* __e, _ForwardIterator __it) {
            __internal::__brick_walk3(
                __b, __e, __it - 1, __it,
                [&__pred](bool& __x, _ReferenceType __y, _ReferenceType __z) { __x = !__pred(__y, __z); }, __is_vector);
        },
        __is_vector);
}

//------------------------------------------------------------------------
// unique_copy
//------------------------------------------------------------------------

template <class _ForwardIterator, class OutputIterator, class _BinaryPredicate>
OutputIterator
__brick_unique_copy(_ForwardIterator __first, _ForwardIterator __last, OutputIterator __result, _BinaryPredicate __pred,
                    /*vector=*/std::false_type) noexcept
{
    return std::unique_copy(__first, __last, __result, __pred);
}

template <class _RandomAccessIterator, class OutputIterator, class _BinaryPredicate>
OutputIterator
__brick_unique_copy(_RandomAccessIterator __first, _RandomAccessIterator __last, OutputIterator __result,
                    _BinaryPredicate __pred, /*vector=*/std::true_type) noexcept
{
#if (_PSTL_MONOTONIC_PRESENT)
    return __unseq_backend::__simd_unique_copy(__first, __last - __first, __result, __pred);
#else
    return std::unique_copy(__first, __last, __result, __pred);
#endif
}

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _BinaryPredicate,
          class _IsVector>
_OutputIterator
__pattern_unique_copy(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                      _BinaryPredicate __pred, _IsVector __is_vector, /*parallel=*/std::false_type) noexcept
{
    return __internal::__brick_unique_copy(__first, __last, __result, __pred, __is_vector);
}

template <class _DifferenceType, class _RandomAccessIterator, class _BinaryPredicate>
_DifferenceType
__brick_calc_mask_2(_RandomAccessIterator __first, _RandomAccessIterator __last, bool* __restrict __mask,
                    _BinaryPredicate __pred, /*vector=*/std::false_type) noexcept
{
    _DifferenceType __count = 0;
    for (; __first != __last; ++__first, ++__mask)
    {
        *__mask = !__pred(*__first, *(__first - 1));
        __count += *__mask;
    }
    return __count;
}

template <class _DifferenceType, class _RandomAccessIterator, class _BinaryPredicate>
_DifferenceType
__brick_calc_mask_2(_RandomAccessIterator __first, _RandomAccessIterator __last, bool* __restrict __mask,
                    _BinaryPredicate __pred, /*vector=*/std::true_type) noexcept
{
    return __unseq_backend::__simd_calc_mask_2(__first, __last - __first, __mask, __pred);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator, class _BinaryPredicate,
          class _IsVector>
_OutputIterator
__pattern_unique_copy(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                      _OutputIterator __result, _BinaryPredicate __pred, _IsVector __is_vector,
                      /*parallel=*/std::true_type)
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::difference_type _DifferenceType;
    const _DifferenceType __n = __last - __first;
    if (_DifferenceType(2) < __n)
    {
        __par_backend::__buffer<bool> __mask_buf(__n);
        if (_DifferenceType(2) < __n)
        {
            return __internal::__except_handler([&__exec, __n, __first, __result, __pred, __is_vector, &__mask_buf]() {
                bool* __mask = __mask_buf.get();
                _DifferenceType __m{};
                __par_backend::__parallel_strict_scan(
                    std::forward<_ExecutionPolicy>(__exec), __n, _DifferenceType(0),
                    [=](_DifferenceType __i, _DifferenceType __len) -> _DifferenceType { // Reduce
                        _DifferenceType __extra = 0;
                        if (__i == 0)
                        {
                            // Special boundary case
                            __mask[__i] = true;
                            if (--__len == 0)
                                return 1;
                            ++__i;
                            ++__extra;
                        }
                        return __internal::__brick_calc_mask_2<_DifferenceType>(__first + __i, __first + (__i + __len),
                                                                                __mask + __i, __pred, __is_vector) +
                               __extra;
                    },
                    std::plus<_DifferenceType>(),                                                // Combine
                    [=](_DifferenceType __i, _DifferenceType __len, _DifferenceType __initial) { // Scan
                        // Phase 2 is same as for __pattern_copy_if
                        __internal::__brick_copy_by_mask(
                            __first + __i, __first + (__i + __len), __result + __initial, __mask + __i,
                            [](_RandomAccessIterator __x, _OutputIterator __z) { *__z = *__x; }, __is_vector);
                    },
                    [&__m](_DifferenceType __total) { __m = __total; });
                return __result + __m;
            });
        }
    }
    // trivial sequence - use serial algorithm
    return __internal::__brick_unique_copy(__first, __last, __result, __pred, __is_vector);
}

//------------------------------------------------------------------------
// reverse
//------------------------------------------------------------------------
template <class _BidirectionalIterator>
void
__brick_reverse(_BidirectionalIterator __first, _BidirectionalIterator __last, /*__is_vector=*/std::false_type) noexcept
{
    std::reverse(__first, __last);
}

template <class _BidirectionalIterator>
void
__brick_reverse(_BidirectionalIterator __first, _BidirectionalIterator __last, /*__is_vector=*/std::true_type) noexcept
{
    typedef typename std::iterator_traits<_BidirectionalIterator>::reference _ReferenceType;

    const auto __n = (__last - __first) / 2;
    __unseq_backend::__simd_walk_2(__first, __n, std::reverse_iterator<_BidirectionalIterator>(__last),
                                   [](_ReferenceType __x, _ReferenceType __y) {
                                       using std::swap;
                                       swap(__x, __y);
                                   });
}

// this brick is called in parallel version, so we can use iterator arithmetic
template <class _BidirectionalIterator>
void
__brick_reverse(_BidirectionalIterator __first, _BidirectionalIterator __last, _BidirectionalIterator __d_last,
                /*is_vector=*/std::false_type) noexcept
{
    for (--__d_last; __first != __last; ++__first, --__d_last)
    {
        using std::iter_swap;
        iter_swap(__first, __d_last);
    }
}

// this brick is called in parallel version, so we can use iterator arithmetic
template <class _BidirectionalIterator>
void
__brick_reverse(_BidirectionalIterator __first, _BidirectionalIterator __last, _BidirectionalIterator __d_last,
                /*is_vector=*/std::true_type) noexcept
{
    typedef typename std::iterator_traits<_BidirectionalIterator>::reference _ReferenceType;

    __unseq_backend::__simd_walk_2(__first, __last - __first, std::reverse_iterator<_BidirectionalIterator>(__d_last),
                                   [](_ReferenceType __x, _ReferenceType __y) {
                                       using std::swap;
                                       swap(__x, __y);
                                   });
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _IsVector>
void
__pattern_reverse(_ExecutionPolicy&&, _BidirectionalIterator __first, _BidirectionalIterator __last,
                  _IsVector _is_vector,
                  /*is_parallel=*/std::false_type) noexcept
{
    __internal::__brick_reverse(__first, __last, _is_vector);
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _IsVector>
void
__pattern_reverse(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __last,
                  _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    __par_backend::__parallel_for(
        std::forward<_ExecutionPolicy>(__exec), __first, __first + (__last - __first) / 2,
        [__is_vector, __first, __last](_BidirectionalIterator __inner_first, _BidirectionalIterator __inner_last) {
            __internal::__brick_reverse(__inner_first, __inner_last, __last - (__inner_first - __first), __is_vector);
        });
}

//------------------------------------------------------------------------
// reverse_copy
//------------------------------------------------------------------------

template <class _BidirectionalIterator, class _OutputIterator>
_OutputIterator
__brick_reverse_copy(_BidirectionalIterator __first, _BidirectionalIterator __last, _OutputIterator __d_first,
                     /*is_vector=*/std::false_type) noexcept
{
    return std::reverse_copy(__first, __last, __d_first);
}

template <class _BidirectionalIterator, class _OutputIterator>
_OutputIterator
__brick_reverse_copy(_BidirectionalIterator __first, _BidirectionalIterator __last, _OutputIterator __d_first,
                     /*is_vector=*/std::true_type) noexcept
{
    typedef typename std::iterator_traits<_BidirectionalIterator>::reference _ReferenceType1;
    typedef typename std::iterator_traits<_OutputIterator>::reference _ReferenceType2;

    return __unseq_backend::__simd_walk_2(std::reverse_iterator<_BidirectionalIterator>(__last), __last - __first,
                                          __d_first, [](_ReferenceType1 __x, _ReferenceType2 __y) { __y = __x; });
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_reverse_copy(_ExecutionPolicy&&, _BidirectionalIterator __first, _BidirectionalIterator __last,
                       _OutputIterator __d_first, _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_reverse_copy(__first, __last, __d_first, __is_vector);
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_reverse_copy(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __last,
                       _OutputIterator __d_first, _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    auto __len = __last - __first;
    __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                  [__is_vector, __first, __len, __d_first](_BidirectionalIterator __inner_first,
                                                                           _BidirectionalIterator __inner_last) {
                                      __internal::__brick_reverse_copy(__inner_first, __inner_last,
                                                                       __d_first + (__len - (__inner_last - __first)),
                                                                       __is_vector);
                                  });
    return __d_first + __len;
}

//------------------------------------------------------------------------
// rotate
//------------------------------------------------------------------------
template <class _ForwardIterator>
_ForwardIterator
__brick_rotate(_ForwardIterator __first, _ForwardIterator __middle, _ForwardIterator __last,
               /*is_vector=*/std::false_type) noexcept
{
#if _PSTL_CPP11_STD_ROTATE_BROKEN
    std::rotate(__first, __middle, __last);
    return std::next(__first, std::distance(__middle, __last));
#else
    return std::rotate(__first, __middle, __last);
#endif
}

template <class _ForwardIterator>
_ForwardIterator
__brick_rotate(_ForwardIterator __first, _ForwardIterator __middle, _ForwardIterator __last,
               /*is_vector=*/std::true_type) noexcept
{
    auto __n = __last - __first;
    auto __m = __middle - __first;
    const _ForwardIterator __ret = __first + (__last - __middle);

    bool __is_left = (__m <= __n / 2);
    if (!__is_left)
        __m = __n - __m;

    while (__n > 1 && __m > 0)
    {
        using std::iter_swap;
        const auto __m_2 = __m * 2;
        if (__is_left)
        {
            for (; __last - __first >= __m_2; __first += __m)
            {
                __unseq_backend::__simd_assign(__first, __m, __first + __m,
                                               iter_swap<_ForwardIterator, _ForwardIterator>);
            }
        }
        else
        {
            for (; __last - __first >= __m_2; __last -= __m)
            {
                __unseq_backend::__simd_assign(__last - __m, __m, __last - __m_2,
                                               iter_swap<_ForwardIterator, _ForwardIterator>);
            }
        }
        __is_left = !__is_left;
        __m = __n % __m;
        __n = __last - __first;
    }

    return __ret;
}

template <class _ExecutionPolicy, class _ForwardIterator, class _IsVector>
_ForwardIterator
__pattern_rotate(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __middle, _ForwardIterator __last,
                 _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_rotate(__first, __middle, __last, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _IsVector>
_ForwardIterator
__pattern_rotate(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __middle,
                 _ForwardIterator __last, _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    typedef typename std::iterator_traits<_ForwardIterator>::value_type _Tp;
    auto __n = __last - __first;
    auto __m = __middle - __first;
    if (__m <= __n / 2)
    {
        __par_backend::__buffer<_Tp> __buf(__n - __m);
        return __internal::__except_handler([&__exec, __n, __m, __first, __middle, __last, __is_vector, &__buf]() {
            _Tp* __result = __buf.get();
            __par_backend::__parallel_for(
                std::forward<_ExecutionPolicy>(__exec), __middle, __last,
                [__middle, __result, __is_vector](_ForwardIterator __b, _ForwardIterator __e) {
                    __internal::__brick_uninitialized_move(__b, __e, __result + (__b - __middle), __is_vector);
                });

            __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __first, __middle,
                                          [__last, __middle, __is_vector](_ForwardIterator __b, _ForwardIterator __e) {
                                              __internal::__brick_move(__b, __e, __b + (__last - __middle),
                                                                       __is_vector);
                                          });

            __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __result, __result + (__n - __m),
                                          [__first, __result, __is_vector](_Tp* __b, _Tp* __e) {
                                              __internal::__brick_move(__b, __e, __first + (__b - __result),
                                                                       __is_vector);
                                          });

            return __first + (__last - __middle);
        });
    }
    else
    {
        __par_backend::__buffer<_Tp> __buf(__m);
        return __internal::__except_handler([&__exec, __n, __m, __first, __middle, __last, __is_vector, &__buf]() {
            _Tp* __result = __buf.get();
            __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __first, __middle,
                                          [__first, __result, __is_vector](_ForwardIterator __b, _ForwardIterator __e) {
                                              __internal::__brick_uninitialized_move(
                                                  __b, __e, __result + (__b - __first), __is_vector);
                                          });

            __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __middle, __last,
                                          [__first, __middle, __is_vector](_ForwardIterator __b, _ForwardIterator __e) {
                                              __internal::__brick_move(__b, __e, __first + (__b - __middle),
                                                                       __is_vector);
                                          });

            __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __result, __result + __m,
                                          [__n, __m, __first, __result, __is_vector](_Tp* __b, _Tp* __e) {
                                              __internal::__brick_move(
                                                  __b, __e, __first + ((__n - __m) + (__b - __result)), __is_vector);
                                          });

            return __first + (__last - __middle);
        });
    }
}

//------------------------------------------------------------------------
// rotate_copy
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_rotate_copy(_ForwardIterator __first, _ForwardIterator __middle, _ForwardIterator __last,
                    _OutputIterator __result, /*__is_vector=*/std::false_type) noexcept
{
    return std::rotate_copy(__first, __middle, __last, __result);
}

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_rotate_copy(_ForwardIterator __first, _ForwardIterator __middle, _ForwardIterator __last,
                    _OutputIterator __result, /*__is_vector=*/std::true_type) noexcept
{
    _OutputIterator __res = __internal::__brick_copy(__middle, __last, __result, std::true_type());
    return __internal::__brick_copy(__first, __middle, __res, std::true_type());
}

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_rotate_copy(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __middle, _ForwardIterator __last,
                      _OutputIterator __result, _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_rotate_copy(__first, __middle, __last, __result, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_rotate_copy(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __middle,
                      _ForwardIterator __last, _OutputIterator __result, _IsVector __is_vector,
                      /*is_parallel=*/std::true_type)
{
    __par_backend::__parallel_for(
        std::forward<_ExecutionPolicy>(__exec), __first, __last,
        [__first, __last, __middle, __result, __is_vector](_ForwardIterator __b, _ForwardIterator __e) {
            if (__b > __middle)
            {
                __internal::__brick_copy(__b, __e, __result + (__b - __middle), __is_vector);
            }
            else
            {
                _OutputIterator __new_result = __result + ((__last - __middle) + (__b - __first));
                if (__e < __middle)
                {
                    __internal::__brick_copy(__b, __e, __new_result, __is_vector);
                }
                else
                {
                    __internal::__brick_copy(__b, __middle, __new_result, __is_vector);
                    __internal::__brick_copy(__middle, __e, __result, __is_vector);
                }
            }
        });
    return __result + (__last - __first);
}

//------------------------------------------------------------------------
// is_partitioned
//------------------------------------------------------------------------

template <class _ForwardIterator, class _UnaryPredicate>
bool
__brick_is_partitioned(_ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                       /*is_vector=*/std::false_type) noexcept
{
    return std::is_partitioned(__first, __last, __pred);
}

template <class _ForwardIterator, class _UnaryPredicate>
bool
__brick_is_partitioned(_ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                       /*is_vector=*/std::true_type) noexcept
{
    typedef typename std::iterator_traits<_ForwardIterator>::difference_type _SizeType;
    if (__first == __last)
    {
        return true;
    }
    else
    {
        _ForwardIterator __result = __unseq_backend::__simd_first(
            __first, _SizeType(0), __last - __first,
            [&__pred](_ForwardIterator __it, _SizeType __i) { return !__pred(__it[__i]); });
        if (__result == __last)
        {
            return true;
        }
        else
        {
            ++__result;
            return !__unseq_backend::__simd_or(__result, __last - __result, __pred);
        }
    }
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
bool
__pattern_is_partitioned(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                         _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_is_partitioned(__first, __last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
bool
__pattern_is_partitioned(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last,
                         _UnaryPredicate __pred, _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    if (__first == __last)
    {
        return true;
    }
    else
    {
        return __internal::__except_handler([&]() {
            // State of current range:
            // broken     - current range is not partitioned by pred
            // all_true   - all elements in current range satisfy pred
            // all_false  - all elements in current range don't satisfy pred
            // true_false - elements satisfy pred are placed before elements that don't satisfy pred
            enum _ReduceType
            {
                __not_init = -1,
                __broken,
                __all_true,
                __all_false,
                __true_false
            };
            _ReduceType __init = __not_init;

            // Array with states that we'll have when state from the left branch is merged with state from the right branch.
            // State is calculated by formula: new_state = table[left_state * 4 + right_state]
            _ReduceType __table[] = {__broken,     __broken,     __broken,     __broken, __broken,    __all_true,
                                     __true_false, __true_false, __broken,     __broken, __all_false, __broken,
                                     __broken,     __broken,     __true_false, __broken};

            __init = __par_backend::__parallel_reduce(
                std::forward<_ExecutionPolicy>(__exec), __first, __last, __init,
                [&__pred, &__table, __is_vector](_ForwardIterator __i, _ForwardIterator __j,
                                                 _ReduceType __value) -> _ReduceType {
                    if (__value == __broken)
                    {
                        return __broken;
                    }
                    _ReduceType __res = __not_init;
                    // if first element satisfy pred
                    if (__pred(*__i))
                    {
                        // find first element that don't satisfy pred
                        _ForwardIterator __x =
                            __internal::__brick_find_if(__i + 1, __j, __not_pred<_UnaryPredicate>(__pred), __is_vector);
                        if (__x != __j)
                        {
                            // find first element after "x" that satisfy pred
                            _ForwardIterator __y = __internal::__brick_find_if(__x + 1, __j, __pred, __is_vector);
                            // if it was found then range isn't partitioned by pred
                            if (__y != __j)
                            {
                                return __broken;
                            }
                            else
                            {
                                __res = __true_false;
                            }
                        }
                        else
                        {
                            __res = __all_true;
                        }
                    }
                    else
                    { // if first element doesn't satisfy pred
                        // then we should find the first element that satisfy pred.
                        // If we found it then range isn't partitioned by pred
                        if (__internal::__brick_find_if(__i + 1, __j, __pred, __is_vector) != __j)
                        {
                            return __broken;
                        }
                        else
                        {
                            __res = __all_false;
                        }
                    }
                    // if we have value from left range then we should calculate the result
                    return (__value == -1) ? __res : __table[__value * 4 + __res];
                },

                [&__table](_ReduceType __val1, _ReduceType __val2) -> _ReduceType {
                    if (__val1 == __broken || __val2 == __broken)
                    {
                        return __broken;
                    }
                    // calculate the result for new big range
                    return __table[__val1 * 4 + __val2];
                });
            return __init != __broken;
        });
    }
}

//------------------------------------------------------------------------
// partition
//------------------------------------------------------------------------

template <class _ForwardIterator, class _UnaryPredicate>
_ForwardIterator
__brick_partition(_ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                  /*is_vector=*/std::false_type) noexcept
{
    return std::partition(__first, __last, __pred);
}

template <class _ForwardIterator, class _UnaryPredicate>
_ForwardIterator
__brick_partition(_ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                  /*is_vector=*/std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::partition(__first, __last, __pred);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_partition(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                    _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_partition(__first, __last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_partition(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last,
                    _UnaryPredicate __pred, _IsVector __is_vector, /*is_parallel=*/std::true_type)
{

    // partitioned range: elements before pivot satisfy pred (true part),
    //                    elements after pivot don't satisfy pred (false part)
    struct _PartitionRange
    {
        _ForwardIterator __begin;
        _ForwardIterator __pivot;
        _ForwardIterator __end;
    };

    return __internal::__except_handler([&]() {
        _PartitionRange __init{__last, __last, __last};

        // lambda for merging two partitioned ranges to one partitioned range
        auto __reductor = [&__exec, __is_vector](_PartitionRange __val1, _PartitionRange __val2) -> _PartitionRange {
            auto __size1 = __val1.__end - __val1.__pivot;
            auto __size2 = __val2.__pivot - __val2.__begin;
            auto __new_begin = __val2.__begin - (__val1.__end - __val1.__begin);

            // if all elements in left range satisfy pred then we can move new pivot to pivot of right range
            if (__val1.__end == __val1.__pivot)
            {
                return {__new_begin, __val2.__pivot, __val2.__end};
            }
            // if true part of right range greater than false part of left range
            // then we should swap the false part of left range and last part of true part of right range
            else if (__size2 > __size1)
            {
                __par_backend::__parallel_for(
                    std::forward<_ExecutionPolicy>(__exec), __val1.__pivot, __val1.__pivot + __size1,
                    [__val1, __val2, __size1, __is_vector](_ForwardIterator __i, _ForwardIterator __j) {
                        __internal::__brick_swap_ranges(__i, __j, (__val2.__pivot - __size1) + (__i - __val1.__pivot),
                                                        __is_vector);
                    });
                return {__new_begin, __val2.__pivot - __size1, __val2.__end};
            }
            // else we should swap the first part of false part of left range and true part of right range
            else
            {
                __par_backend::__parallel_for(
                    std::forward<_ExecutionPolicy>(__exec), __val1.__pivot, __val1.__pivot + __size2,
                    [__val1, __val2, __is_vector](_ForwardIterator __i, _ForwardIterator __j) {
                        __internal::__brick_swap_ranges(__i, __j, __val2.__begin + (__i - __val1.__pivot), __is_vector);
                    });
                return {__new_begin, __val1.__pivot + __size2, __val2.__end};
            }
        };

        _PartitionRange __result = __par_backend::__parallel_reduce(
            std::forward<_ExecutionPolicy>(__exec), __first, __last, __init,
            [__pred, __is_vector, __reductor](_ForwardIterator __i, _ForwardIterator __j,
                                              _PartitionRange __value) -> _PartitionRange {
                //1. serial partition
                _ForwardIterator __pivot = __internal::__brick_partition(__i, __j, __pred, __is_vector);

                // 2. merging of two ranges (left and right respectively)
                return __reductor(__value, {__i, __pivot, __j});
            },
            __reductor);
        return __result.__pivot;
    });
}

//------------------------------------------------------------------------
// stable_partition
//------------------------------------------------------------------------

template <class _BidirectionalIterator, class _UnaryPredicate>
_BidirectionalIterator
__brick_stable_partition(_BidirectionalIterator __first, _BidirectionalIterator __last, _UnaryPredicate __pred,
                         /*__is_vector=*/std::false_type) noexcept
{
    return std::stable_partition(__first, __last, __pred);
}

template <class _BidirectionalIterator, class _UnaryPredicate>
_BidirectionalIterator
__brick_stable_partition(_BidirectionalIterator __first, _BidirectionalIterator __last, _UnaryPredicate __pred,
                         /*__is_vector=*/std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::stable_partition(__first, __last, __pred);
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _UnaryPredicate, class _IsVector>
_BidirectionalIterator
__pattern_stable_partition(_ExecutionPolicy&&, _BidirectionalIterator __first, _BidirectionalIterator __last,
                           _UnaryPredicate __pred, _IsVector __is_vector,
                           /*is_parallelization=*/std::false_type) noexcept
{
    return __internal::__brick_stable_partition(__first, __last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _UnaryPredicate, class _IsVector>
_BidirectionalIterator
__pattern_stable_partition(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __last,
                           _UnaryPredicate __pred, _IsVector __is_vector,
                           /*is_parallelization=*/std::true_type) noexcept
{
    // partitioned range: elements before pivot satisfy pred (true part),
    //                    elements after pivot don't satisfy pred (false part)
    struct _PartitionRange
    {
        _BidirectionalIterator __begin;
        _BidirectionalIterator __pivot;
        _BidirectionalIterator __end;
    };

    return __internal::__except_handler([&]() {
        _PartitionRange __init{__last, __last, __last};

        // lambda for merging two partitioned ranges to one partitioned range
        auto __reductor = [__is_vector](_PartitionRange __val1, _PartitionRange __val2) -> _PartitionRange {
            auto __size1 = __val1.__end - __val1.__pivot;
            auto __new_begin = __val2.__begin - (__val1.__end - __val1.__begin);

            // if all elements in left range satisfy pred then we can move new pivot to pivot of right range
            if (__val1.__end == __val1.__pivot)
            {
                return {__new_begin, __val2.__pivot, __val2.__end};
            }
            // if true part of right range greater than false part of left range
            // then we should swap the false part of left range and last part of true part of right range
            else
            {
                __internal::__brick_rotate(__val1.__pivot, __val2.__begin, __val2.__pivot, __is_vector);
                return {__new_begin, __val2.__pivot - __size1, __val2.__end};
            }
        };

        _PartitionRange __result = __par_backend::__parallel_reduce(
            std::forward<_ExecutionPolicy>(__exec), __first, __last, __init,
            [&__pred, __is_vector, __reductor](_BidirectionalIterator __i, _BidirectionalIterator __j,
                                               _PartitionRange __value) -> _PartitionRange {
                //1. serial stable_partition
                _BidirectionalIterator __pivot = __internal::__brick_stable_partition(__i, __j, __pred, __is_vector);

                // 2. merging of two ranges (left and right respectively)
                return __reductor(__value, {__i, __pivot, __j});
            },
            __reductor);
        return __result.__pivot;
    });
}

//------------------------------------------------------------------------
// partition_copy
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator1, class _OutputIterator2, class _UnaryPredicate>
std::pair<_OutputIterator1, _OutputIterator2>
__brick_partition_copy(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator1 __out_true,
                       _OutputIterator2 __out_false, _UnaryPredicate __pred, /*is_vector=*/std::false_type) noexcept
{
    return std::partition_copy(__first, __last, __out_true, __out_false, __pred);
}

template <class _ForwardIterator, class _OutputIterator1, class _OutputIterator2, class _UnaryPredicate>
std::pair<_OutputIterator1, _OutputIterator2>
__brick_partition_copy(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator1 __out_true,
                       _OutputIterator2 __out_false, _UnaryPredicate __pred, /*is_vector=*/std::true_type) noexcept
{
#if (_PSTL_MONOTONIC_PRESENT)
    return __unseq_backend::__simd_partition_copy(__first, __last - __first, __out_true, __out_false, __pred);
#else
    return std::partition_copy(__first, __last, __out_true, __out_false, __pred);
#endif
}

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator1, class _OutputIterator2,
          class _UnaryPredicate, class _IsVector>
std::pair<_OutputIterator1, _OutputIterator2>
__pattern_partition_copy(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last,
                         _OutputIterator1 __out_true, _OutputIterator2 __out_false, _UnaryPredicate __pred,
                         _IsVector __is_vector, /*is_parallelization=*/std::false_type) noexcept
{
    return __internal::__brick_partition_copy(__first, __last, __out_true, __out_false, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator1, class _OutputIterator2,
          class _UnaryPredicate, class _IsVector>
std::pair<_OutputIterator1, _OutputIterator2>
__pattern_partition_copy(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                         _OutputIterator1 __out_true, _OutputIterator2 __out_false, _UnaryPredicate __pred,
                         _IsVector __is_vector, /*is_parallelization=*/std::true_type)
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::difference_type _DifferenceType;
    typedef std::pair<_DifferenceType, _DifferenceType> _ReturnType;
    const _DifferenceType __n = __last - __first;
    if (_DifferenceType(1) < __n)
    {
        __par_backend::__buffer<bool> __mask_buf(__n);
        return __internal::__except_handler([&__exec, __n, __first, __out_true, __out_false, __is_vector, __pred,
                                             &__mask_buf]() {
            bool* __mask = __mask_buf.get();
            _ReturnType __m{};
            __par_backend::__parallel_strict_scan(
                std::forward<_ExecutionPolicy>(__exec), __n, std::make_pair(_DifferenceType(0), _DifferenceType(0)),
                [=](_DifferenceType __i, _DifferenceType __len) { // Reduce
                    return __internal::__brick_calc_mask_1<_DifferenceType>(__first + __i, __first + (__i + __len),
                                                                            __mask + __i, __pred, __is_vector);
                },
                [](const _ReturnType& __x, const _ReturnType& __y) -> _ReturnType {
                    return std::make_pair(__x.first + __y.first, __x.second + __y.second);
                },                                                                       // Combine
                [=](_DifferenceType __i, _DifferenceType __len, _ReturnType __initial) { // Scan
                    __internal::__brick_partition_by_mask(__first + __i, __first + (__i + __len),
                                                          __out_true + __initial.first, __out_false + __initial.second,
                                                          __mask + __i, __is_vector);
                },
                [&__m](_ReturnType __total) { __m = __total; });
            return std::make_pair(__out_true + __m.first, __out_false + __m.second);
        });
    }
    // trivial sequence - use serial algorithm
    return __internal::__brick_partition_copy(__first, __last, __out_true, __out_false, __pred, __is_vector);
}

//------------------------------------------------------------------------
// sort
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector,
          class _IsMoveConstructible>
void
__pattern_sort(_ExecutionPolicy&&, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp,
               _IsVector /*is_vector*/, /*is_parallel=*/std::false_type, _IsMoveConstructible) noexcept
{
    std::sort(__first, __last, __comp);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp,
               _IsVector /*is_vector*/, /*is_parallel=*/std::true_type, /*is_move_constructible=*/std::true_type)
{
    __internal::__except_handler([&]() {
        __par_backend::__parallel_stable_sort(std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
                                              [](_RandomAccessIterator __first, _RandomAccessIterator __last,
                                                 _Compare __comp) { std::sort(__first, __last, __comp); },
                                              __last - __first);
    });
}

//------------------------------------------------------------------------
// stable_sort
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_stable_sort(_ExecutionPolicy&&, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp,
                      _IsVector /*is_vector*/, /*is_parallel=*/std::false_type) noexcept
{
    std::stable_sort(__first, __last, __comp);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_stable_sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                      _Compare __comp, _IsVector /*is_vector*/, /*is_parallel=*/std::true_type)
{
    __internal::__except_handler([&]() {
        __par_backend::__parallel_stable_sort(std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
                                              [](_RandomAccessIterator __first, _RandomAccessIterator __last,
                                                 _Compare __comp) { std::stable_sort(__first, __last, __comp); });
    });
}

//------------------------------------------------------------------------
// partial_sort
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_partial_sort(_ExecutionPolicy&&, _RandomAccessIterator __first, _RandomAccessIterator __middle,
                       _RandomAccessIterator __last, _Compare __comp, _IsVector,
                       /*is_parallel=*/std::false_type) noexcept
{
    std::partial_sort(__first, __middle, __last, __comp);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_partial_sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __middle,
                       _RandomAccessIterator __last, _Compare __comp, _IsVector, /*is_parallel=*/std::true_type)
{
    const auto __n = __middle - __first;
    __internal::__except_handler([&]() {
        __par_backend::__parallel_stable_sort(
            std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
            [__n](_RandomAccessIterator __begin, _RandomAccessIterator __end, _Compare __comp) {
                if (__n < __end - __begin)
                    std::partial_sort(__begin, __begin + __n, __end, __comp);
                else
                    std::sort(__begin, __end, __comp);
            },
            __n);
    });
}

//------------------------------------------------------------------------
// partial_sort_copy
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _ForwardIterator, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_partial_sort_copy(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last,
                            _RandomAccessIterator __d_first, _RandomAccessIterator __d_last, _Compare __comp, _IsVector,
                            /*is_parallel=*/std::false_type) noexcept
{
    return std::partial_sort_copy(__first, __last, __d_first, __d_last, __comp);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_partial_sort_copy(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last,
                            _RandomAccessIterator __d_first, _RandomAccessIterator __d_last, _Compare __comp,
                            _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    if (__last == __first || __d_last == __d_first)
    {
        return __d_first;
    }
    auto __n1 = __last - __first;
    auto __n2 = __d_last - __d_first;
    return __internal::__except_handler([&]() {
        if (__n2 >= __n1)
        {
            __par_backend::__parallel_stable_sort(
                std::forward<_ExecutionPolicy>(__exec), __d_first, __d_first + __n1, __comp,
                [__first, __d_first, __is_vector](_RandomAccessIterator __i, _RandomAccessIterator __j,
                                                  _Compare __comp) {
                    _ForwardIterator __i1 = __first + (__i - __d_first);
                    _ForwardIterator __j1 = __first + (__j - __d_first);

                // 1. Copy elements from input to output
#    if !_PSTL_ICC_18_OMP_SIMD_BROKEN
                    __internal::__brick_copy(__i1, __j1, __i, __is_vector);
#    else
                    std::copy(__i1, __j1, __i);
#    endif
                    // 2. Sort elements in output sequence
                    std::sort(__i, __j, __comp);
                },
                __n1);
            return __d_first + __n1;
        }
        else
        {
            typedef typename std::iterator_traits<_ForwardIterator>::value_type _T1;
            typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _T2;
            __par_backend::__buffer<_T1> __buf(__n1);
            _T1* __r = __buf.get();

            __par_backend::__parallel_stable_sort(std::forward<_ExecutionPolicy>(__exec), __r, __r + __n1, __comp,
                                                  [__n2, __first, __r](_T1* __i, _T1* __j, _Compare __comp) {
                                                      _ForwardIterator __it = __first + (__i - __r);

                                                      // 1. Copy elements from input to raw memory
                                                      for (_T1* __k = __i; __k != __j; ++__k, ++__it)
                                                      {
                                                          ::new (__k) _T2(*__it);
                                                      }

                                                      // 2. Sort elements in temporary __buffer
                                                      if (__n2 < __j - __i)
                                                          std::partial_sort(__i, __i + __n2, __j, __comp);
                                                      else
                                                          std::sort(__i, __j, __comp);
                                                  },
                                                  __n2);

            // 3. Move elements from temporary __buffer to output
            __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __r, __r + __n2,
                                          [__r, __d_first, __is_vector](_T1* __i, _T1* __j) {
                                              __internal::__brick_move(__i, __j, __d_first + (__i - __r), __is_vector);
                                          });
            return __d_first + __n2;
        }
    });
}

//------------------------------------------------------------------------
// adjacent_find
//------------------------------------------------------------------------
template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator
__brick_adjacent_find(_ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred,
                      /* IsVector = */ std::true_type, bool __or_semantic) noexcept
{
    return __unseq_backend::__simd_adjacent_find(__first, __last, __pred, __or_semantic);
}

template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator
__brick_adjacent_find(_ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred,
                      /* IsVector = */ std::false_type, bool __or_semantic) noexcept
{
    return std::adjacent_find(__first, __last, __pred);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate, class _IsVector>
_ForwardIterator
__pattern_adjacent_find(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred,
                        /* is_parallel */ std::false_type, _IsVector __is_vector, bool __or_semantic) noexcept
{
    return __internal::__brick_adjacent_find(__first, __last, __pred, __is_vector, __or_semantic);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _BinaryPredicate, class _IsVector>
_RandomAccessIterator
__pattern_adjacent_find(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                        _BinaryPredicate __pred, /* is_parallel */ std::true_type, _IsVector __is_vector,
                        bool __or_semantic)
{
    if (__last - __first < 2)
        return __last;

    return __internal::__except_handler([&]() {
        return __par_backend::__parallel_reduce(
            std::forward<_ExecutionPolicy>(__exec), __first, __last, __last,
            [__last, __pred, __is_vector, __or_semantic](_RandomAccessIterator __begin, _RandomAccessIterator __end,
                                                         _RandomAccessIterator __value) -> _RandomAccessIterator {
                // TODO: investigate performance benefits from the use of shared variable for the result,
                // checking (compare_and_swap idiom) its __value at __first.
                if (__or_semantic && __value < __last)
                { //found
                    __par_backend::__cancel_execution();
                    return __value;
                }

                if (__value > __begin)
                {
                    // modify __end to check the predicate on the boundary __values;
                    // TODO: to use a custom range with boundaries overlapping
                    // TODO: investigate what if we remove "if" below and run algorithm on range [__first, __last-1)
                    // then check the pair [__last-1, __last)
                    if (__end != __last)
                        ++__end;

                    //correct the global result iterator if the "brick" returns a local "__last"
                    const _RandomAccessIterator __res =
                        __internal::__brick_adjacent_find(__begin, __end, __pred, __is_vector, __or_semantic);
                    if (__res < __end)
                        __value = __res;
                }
                return __value;
            },
            [](_RandomAccessIterator __x, _RandomAccessIterator __y) -> _RandomAccessIterator {
                return __x < __y ? __x : __y;
            } //reduce a __value
        );
    });
}

//------------------------------------------------------------------------
// nth_element
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_nth_element(_ExecutionPolicy&&, _RandomAccessIterator __first, _RandomAccessIterator __nth,
                      _RandomAccessIterator __last, _Compare __comp, _IsVector,
                      /*is_parallel=*/std::false_type) noexcept
{
    std::nth_element(__first, __nth, __last, __comp);
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_nth_element(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __nth,
                      _RandomAccessIterator __last, _Compare __comp, _IsVector __is_vector,
                      /*is_parallel=*/std::true_type) noexcept
{
    if (__first == __last || __nth == __last)
    {
        return;
    }

    using std::iter_swap;
    typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _Tp;
    _RandomAccessIterator __x;
    do
    {
        __x = __internal::__pattern_partition(std::forward<_ExecutionPolicy>(__exec), __first + 1, __last,
                                              [&__comp, __first](const _Tp& __x) { return __comp(__x, *__first); },
                                              __is_vector,
                                              /*is_parallel=*/std::true_type());
        --__x;
        if (__x != __first)
        {
            iter_swap(__first, __x);
        }
        // if x > nth then our new range for partition is [first, x)
        if (__x - __nth > 0)
        {
            __last = __x;
        }
        // if x < nth then our new range for partition is [x, last)
        else if (__x - __nth < 0)
        {
            // if *x == *nth then we can start new partition with x+1
            if (!__comp(*__nth, *__x) && !__comp(*__x, *__nth))
            {
                ++__x;
            }
            else
            {
                iter_swap(__nth, __x);
            }
            __first = __x;
        }
    } while (__x != __nth);
}

//------------------------------------------------------------------------
// fill, fill_n
//------------------------------------------------------------------------
template <class _ForwardIterator, class _Tp>
void
__brick_fill(_ForwardIterator __first, _ForwardIterator __last, const _Tp& __value,
             /* __is_vector = */ std::true_type) noexcept
{
    __unseq_backend::__simd_fill_n(__first, __last - __first, __value);
}

template <class _ForwardIterator, class _Tp>
void
__brick_fill(_ForwardIterator __first, _ForwardIterator __last, const _Tp& __value,
             /* __is_vector = */ std::false_type) noexcept
{
    std::fill(__first, __last, __value);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _IsVector>
void
__pattern_fill(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, const _Tp& __value,
               /*is_parallel=*/std::false_type, _IsVector __is_vector) noexcept
{
    __internal::__brick_fill(__first, __last, __value, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _IsVector>
_ForwardIterator
__pattern_fill(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, const _Tp& __value,
               /*is_parallel=*/std::true_type, _IsVector __is_vector)
{
    return __internal::__except_handler([&__exec, __first, __last, &__value, __is_vector]() {
        __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                      [&__value, __is_vector](_ForwardIterator __begin, _ForwardIterator __end) {
                                          __internal::__brick_fill(__begin, __end, __value, __is_vector);
                                      });
        return __last;
    });
}

template <class _OutputIterator, class _Size, class _Tp>
_OutputIterator
__brick_fill_n(_OutputIterator __first, _Size __count, const _Tp& __value, /* __is_vector = */ std::true_type) noexcept
{
    return __unseq_backend::__simd_fill_n(__first, __count, __value);
}

template <class _OutputIterator, class _Size, class _Tp>
_OutputIterator
__brick_fill_n(_OutputIterator __first, _Size __count, const _Tp& __value, /* __is_vector = */ std::false_type) noexcept
{
    return std::fill_n(__first, __count, __value);
}

template <class _ExecutionPolicy, class _OutputIterator, class _Size, class _Tp, class _IsVector>
_OutputIterator
__pattern_fill_n(_ExecutionPolicy&&, _OutputIterator __first, _Size __count, const _Tp& __value,
                 /*is_parallel=*/std::false_type, _IsVector __is_vector) noexcept
{
    return __internal::__brick_fill_n(__first, __count, __value, __is_vector);
}

template <class _ExecutionPolicy, class _OutputIterator, class _Size, class _Tp, class _IsVector>
_OutputIterator
__pattern_fill_n(_ExecutionPolicy&& __exec, _OutputIterator __first, _Size __count, const _Tp& __value,
                 /*is_parallel=*/std::true_type, _IsVector __is_vector)
{
    return __internal::__pattern_fill(std::forward<_ExecutionPolicy>(__exec), __first, __first + __count, __value,
                                      std::true_type(), __is_vector);
}

//------------------------------------------------------------------------
// generate, generate_n
//------------------------------------------------------------------------
template <class _RandomAccessIterator, class _Generator>
void
__brick_generate(_RandomAccessIterator __first, _RandomAccessIterator __last, _Generator __g,
                 /* is_vector = */ std::true_type) noexcept
{
    __unseq_backend::__simd_generate_n(__first, __last - __first, __g);
}

template <class _ForwardIterator, class _Generator>
void
__brick_generate(_ForwardIterator __first, _ForwardIterator __last, _Generator __g,
                 /* is_vector = */ std::false_type) noexcept
{
    std::generate(__first, __last, __g);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Generator, class _IsVector>
void
__pattern_generate(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Generator __g,
                   /*is_parallel=*/std::false_type, _IsVector __is_vector) noexcept
{
    __internal::__brick_generate(__first, __last, __g, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Generator, class _IsVector>
_ForwardIterator
__pattern_generate(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Generator __g,
                   /*is_parallel=*/std::true_type, _IsVector __is_vector)
{
    return __internal::__except_handler([&]() {
        __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                      [__g, __is_vector](_ForwardIterator __begin, _ForwardIterator __end) {
                                          __internal::__brick_generate(__begin, __end, __g, __is_vector);
                                      });
        return __last;
    });
}

template <class OutputIterator, class Size, class _Generator>
OutputIterator
__brick_generate_n(OutputIterator __first, Size __count, _Generator __g, /* is_vector = */ std::true_type) noexcept
{
    return __unseq_backend::__simd_generate_n(__first, __count, __g);
}

template <class OutputIterator, class Size, class _Generator>
OutputIterator
__brick_generate_n(OutputIterator __first, Size __count, _Generator __g, /* is_vector = */ std::false_type) noexcept
{
    return std::generate_n(__first, __count, __g);
}

template <class _ExecutionPolicy, class _OutputIterator, class _Size, class _Generator, class _IsVector>
_OutputIterator
__pattern_generate_n(_ExecutionPolicy&&, _OutputIterator __first, _Size __count, _Generator __g,
                     /*is_parallel=*/std::false_type, _IsVector __is_vector) noexcept
{
    return __internal::__brick_generate_n(__first, __count, __g, __is_vector);
}

template <class _ExecutionPolicy, class _OutputIterator, class _Size, class _Generator, class _IsVector>
_OutputIterator
__pattern_generate_n(_ExecutionPolicy&& __exec, _OutputIterator __first, _Size __count, _Generator __g,
                     /*is_parallel=*/std::true_type, _IsVector __is_vector)
{
    static_assert(__is_random_access_iterator<_OutputIterator>::value,
                  "Pattern-brick error. Should be a random access iterator.");
    return __internal::__pattern_generate(std::forward<_ExecutionPolicy>(__exec), __first, __first + __count, __g,
                                          std::true_type(), __is_vector);
}

//------------------------------------------------------------------------
// remove
//------------------------------------------------------------------------

template <class _ForwardIterator, class _UnaryPredicate>
_ForwardIterator
__brick_remove_if(_ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                  /* __is_vector = */ std::false_type) noexcept
{
    return std::remove_if(__first, __last, __pred);
}

template <class _RandomAccessIterator, class _UnaryPredicate>
_RandomAccessIterator
__brick_remove_if(_RandomAccessIterator __first, _RandomAccessIterator __last, _UnaryPredicate __pred,
                  /* __is_vector = */ std::true_type) noexcept
{
#if _PSTL_MONOTONIC_PRESENT
    return __unseq_backend::__simd_remove_if(__first, __last - __first, __pred);
#else
    return std::remove_if(__first, __last, __pred);
#endif
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_remove_if(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
                    _IsVector __is_vector, /*is_parallel*/ std::false_type) noexcept
{
    return __internal::__brick_remove_if(__first, __last, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_remove_if(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last,
                    _UnaryPredicate __pred, _IsVector __is_vector, /*is_parallel*/ std::true_type) noexcept
{
    typedef typename std::iterator_traits<_ForwardIterator>::reference _ReferenceType;

    if (__first == __last || __first + 1 == __last)
    {
        // Trivial sequence - use serial algorithm
        return __internal::__brick_remove_if(__first, __last, __pred, __is_vector);
    }

    return __internal::__remove_elements(
        std::forward<_ExecutionPolicy>(__exec), __first, __last,
        [&__pred, __is_vector](bool* __b, bool* __e, _ForwardIterator __it) {
            __internal::__brick_walk2(__b, __e, __it, [&__pred](bool& __x, _ReferenceType __y) { __x = !__pred(__y); },
                                      __is_vector);
        },
        __is_vector);
}

//------------------------------------------------------------------------
// merge
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_merge(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
              _ForwardIterator2 __last2, _OutputIterator __d_first, _Compare __comp,
              /* __is_vector = */ std::false_type) noexcept
{
    return std::merge(__first1, __last1, __first2, __last2, __d_first, __comp);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_merge(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
              _ForwardIterator2 __last2, _OutputIterator __d_first, _Compare __comp,
              /* __is_vector = */ std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::merge(__first1, __last1, __first2, __last2, __d_first, __comp);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_merge(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                _ForwardIterator2 __last2, _OutputIterator __d_first, _Compare __comp, _IsVector __is_vector,
                /* is_parallel = */ std::false_type) noexcept
{
    return __internal::__brick_merge(__first1, __last1, __first2, __last2, __d_first, __comp, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_merge(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                _RandomAccessIterator2 __first2, _RandomAccessIterator2 __last2, _OutputIterator __d_first,
                _Compare __comp, _IsVector __is_vector, /* is_parallel = */ std::true_type)
{
    __par_backend::__parallel_merge(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __d_first, __comp,
        [__is_vector](_RandomAccessIterator1 __f1, _RandomAccessIterator1 __l1, _RandomAccessIterator2 __f2,
                      _RandomAccessIterator2 __l2, _OutputIterator __f3, _Compare __comp) {
            return __internal::__brick_merge(__f1, __l1, __f2, __l2, __f3, __comp, __is_vector);
        });
    return __d_first + (__last1 - __first1) + (__last2 - __first2);
}

//------------------------------------------------------------------------
// inplace_merge
//------------------------------------------------------------------------
template <class _BidirectionalIterator, class _Compare>
void
__brick_inplace_merge(_BidirectionalIterator __first, _BidirectionalIterator __middle, _BidirectionalIterator __last,
                      _Compare __comp, /* __is_vector = */ std::false_type) noexcept
{
    std::inplace_merge(__first, __middle, __last, __comp);
}

template <class _BidirectionalIterator, class _Compare>
void
__brick_inplace_merge(_BidirectionalIterator __first, _BidirectionalIterator __middle, _BidirectionalIterator __last,
                      _Compare __comp, /* __is_vector = */ std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial")
    std::inplace_merge(__first, __middle, __last, __comp);
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _Compare, class _IsVector>
void
__pattern_inplace_merge(_ExecutionPolicy&&, _BidirectionalIterator __first, _BidirectionalIterator __middle,
                        _BidirectionalIterator __last, _Compare __comp, _IsVector __is_vector,
                        /* is_parallel = */ std::false_type) noexcept
{
    __internal::__brick_inplace_merge(__first, __middle, __last, __comp, __is_vector);
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _Compare, class _IsVector>
void
__pattern_inplace_merge(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __middle,
                        _BidirectionalIterator __last, _Compare __comp, _IsVector __is_vector,
                        /*is_parallel=*/std::true_type)
{
    if (__first == __last || __first == __middle || __middle == __last)
    {
        return;
    }
    typedef typename std::iterator_traits<_BidirectionalIterator>::value_type _Tp;
    auto __n = __last - __first;
    __par_backend::__buffer<_Tp> __buf(__n);
    _Tp* __r = __buf.get();
    __internal::__except_handler([&]() {
        auto __move_values = [](_BidirectionalIterator __x, _Tp* __z) {
            __internal::__invoke_if_else(std::is_trivial<_Tp>(), [&]() { *__z = std::move(*__x); },
                                         [&]() { ::new (std::addressof(*__z)) _Tp(std::move(*__x)); });
        };

        auto __move_sequences = [](_BidirectionalIterator __first1, _BidirectionalIterator __last1, _Tp* __first2) {
            return __internal::__brick_uninitialized_move(__first1, __last1, __first2, _IsVector());
        };

        __par_backend::__parallel_merge(
            std::forward<_ExecutionPolicy>(__exec), __first, __middle, __middle, __last, __r, __comp,
            [__n, __move_values, __move_sequences](_BidirectionalIterator __f1, _BidirectionalIterator __l1,
                                                   _BidirectionalIterator __f2, _BidirectionalIterator __l2, _Tp* __f3,
                                                   _Compare __comp) {
                auto __func = __par_backend::__serial_move_merge<decltype(__move_values), decltype(__move_sequences)>(
                    __n, __move_values, __move_sequences);
                __func(__f1, __l1, __f2, __l2, __f3, __comp);
                return __f3 + (__l1 - __f1) + (__l2 - __f2);
            });

        __par_backend::__parallel_for(std::forward<_ExecutionPolicy>(__exec), __r, __r + __n,
                                      [__r, __first, __is_vector](_Tp* __i, _Tp* __j) {
                                          __internal::__brick_move(__i, __j, __first + (__i - __r), __is_vector);
                                      });
    });
}

//------------------------------------------------------------------------
// includes
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_includes(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                   _ForwardIterator2 __first2, _ForwardIterator2 __last2, _Compare __comp, _IsVector,
                   /*is_parallel=*/std::false_type) noexcept
{
    return std::includes(__first1, __last1, __first2, __last2, __comp);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_includes(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                   _ForwardIterator2 __first2, _ForwardIterator2 __last2, _Compare __comp, _IsVector __is_vector,
                   /*is_parallel=*/std::true_type)
{
    if (__first2 >= __last2)
        return true;

    if (__first1 >= __last1 || __comp(*__first2, *__first1) || __comp(*(__last1 - 1), *(__last2 - 1)))
        return false;

    __first1 = std::lower_bound(__first1, __last1, *__first2, __comp);
    if (__first1 == __last1)
        return false;

    if (__last2 - __first2 == 1)
        return !__comp(*__first1, *__first2) && !__comp(*__first2, *__first1);

    return __internal::__except_handler([&]() {
        return !__internal::__parallel_or(
            std::forward<_ExecutionPolicy>(__exec), __first2, __last2,
            [__first1, __last1, __first2, __last2, &__comp](_ForwardIterator2 __i, _ForwardIterator2 __j) {
                _PSTL_ASSERT(__j > __i);
                //assert(__j - __i > 1);

                //1. moving boundaries to "consume" subsequence of equal elements
                auto __is_equal = [&__comp](_ForwardIterator2 __a, _ForwardIterator2 __b) -> bool {
                    return !__comp(*__a, *__b) && !__comp(*__b, *__a);
                };

                //1.1 left bound, case "aaa[aaaxyz...]" - searching "x"
                if (__i > __first2 && __is_equal(__i, __i - 1))
                {
                    //whole subrange continues to content equal elements - return "no op"
                    if (__is_equal(__i, __j - 1))
                        return false;

                    __i = std::upper_bound(__i, __last2, *__i, __comp);
                }

                //1.2 right bound, case "[...aaa]aaaxyz" - searching "x"
                if (__j < __last2 && __is_equal(__j - 1, __j))
                    __j = std::upper_bound(__j, __last2, *__j, __comp);

                //2. testing is __a subsequence of the second range included into the first range
                auto __b = std::lower_bound(__first1, __last1, *__i, __comp);

                _PSTL_ASSERT(!__comp(*(__last1 - 1), *__b));
                _PSTL_ASSERT(!__comp(*(__j - 1), *__i));
                return !std::includes(__b, __last1, __i, __j, __comp);
            });
    });
}

constexpr auto __set_algo_cut_off = 1000;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector, class _SizeFunction, class _SetOP>
_OutputIterator
__parallel_set_op(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                  _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                  _SizeFunction __size_func, _SetOP __set_op, _IsVector __is_vector)
{
    typedef typename std::iterator_traits<_ForwardIterator1>::difference_type _DifferenceType;
    typedef typename std::iterator_traits<_OutputIterator>::value_type _T;

    struct _SetRange
    {
        _DifferenceType __pos, __len, __buf_pos;
        bool
        empty() const
        {
            return __len == 0;
        }
    };

    const _DifferenceType __n1 = __last1 - __first1;
    const _DifferenceType __n2 = __last2 - __first2;

    __par_backend::__buffer<_T> __buf(__size_func(__n1, __n2));

    return __internal::__except_handler([&__exec, __n1, __first1, __last1, __first2, __last2, __result, __is_vector,
                                         __comp, __size_func, __set_op, &__buf]() {
        auto __buffer = __buf.get();
        _DifferenceType __m{};
        auto __scan = [=](_DifferenceType, _DifferenceType, const _SetRange& __s) { // Scan
            if (!__s.empty())
                __internal::__brick_move(__buffer + __s.__buf_pos, __buffer + (__s.__buf_pos + __s.__len),
                                         __result + __s.__pos, __is_vector);
        };
        __par_backend::__parallel_strict_scan(
            std::forward<_ExecutionPolicy>(__exec), __n1, _SetRange{0, 0, 0}, //-1, 0},
            [=](_DifferenceType __i, _DifferenceType __len) {                 // Reduce
                //[__b; __e) - a subrange of the first sequence, to reduce
                _ForwardIterator1 __b = __first1 + __i, __e = __first1 + (__i + __len);

                //try searching for the first element which not equal to *__b
                if (__b != __first1)
                    __b = std::upper_bound(__b, __last1, *__b, __comp);

                //try searching for the first element which not equal to *__e
                if (__e != __last1)
                    __e = std::upper_bound(__e, __last1, *__e, __comp);

                //check is [__b; __e) empty
                if (__e - __b < 1)
                {
                    _ForwardIterator2 __bb = __last2;
                    if (__b != __last1)
                        __bb = std::lower_bound(__first2, __last2, *__b, __comp);

                    const _DifferenceType __buf_pos = __size_func((__b - __first1), (__bb - __first2));
                    return _SetRange{0, 0, __buf_pos};
                }

                //try searching for "corresponding" subrange [__bb; __ee) in the second sequence
                _ForwardIterator2 __bb = __first2;
                if (__b != __first1)
                    __bb = std::lower_bound(__first2, __last2, *__b, __comp);

                _ForwardIterator2 __ee = __last2;
                if (__e != __last1)
                    __ee = std::lower_bound(__bb, __last2, *__e, __comp);

                const _DifferenceType __buf_pos = __size_func((__b - __first1), (__bb - __first2));
                auto __buffer_b = __buffer + __buf_pos;
                auto __res = __set_op(__b, __e, __bb, __ee, __buffer_b, __comp);

                return _SetRange{0, __res - __buffer_b, __buf_pos};
            },
            [](const _SetRange& __a, const _SetRange& __b) { // Combine
                if (__b.__buf_pos > __a.__buf_pos || ((__b.__buf_pos == __a.__buf_pos) && !__b.empty()))
                    return _SetRange{__a.__pos + __a.__len + __b.__pos, __b.__len, __b.__buf_pos};
                return _SetRange{__b.__pos + __b.__len + __a.__pos, __a.__len, __a.__buf_pos};
            },
            __scan,                                     // Scan
            [&__m, &__scan](const _SetRange& __total) { // Apex
                //final scan
                __scan(0, 0, __total);
                __m = __total.__pos + __total.__len;
            });
        return __result + __m;
    });
}

//a shared parallel pattern for '__pattern_set_union' and '__pattern_set_symmetric_difference'
template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _SetUnionOp, class _IsVector>
_OutputIterator
__parallel_set_union_op(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                        _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result,
                        _Compare __comp, _SetUnionOp __set_union_op, _IsVector __is_vector)
{
    typedef typename std::iterator_traits<_ForwardIterator1>::difference_type _DifferenceType;

    const auto __n1 = __last1 - __first1;
    const auto __n2 = __last2 - __first2;

    auto __copy_range1 = [__is_vector](_ForwardIterator1 __begin, _ForwardIterator1 __end, _OutputIterator __res) {
        return __internal::__brick_copy(__begin, __end, __res, __is_vector);
    };
    auto __copy_range2 = [__is_vector](_ForwardIterator2 __begin, _ForwardIterator2 __end, _OutputIterator __res) {
        return __internal::__brick_copy(__begin, __end, __res, __is_vector);
    };

    // {1} {}: parallel copying just first sequence
    if (__n2 == 0)
        return __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __result,
                                                 __copy_range1, std::true_type());

    // {} {2}: parallel copying justmake  second sequence
    if (__n1 == 0)
        return __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first2, __last2, __result,
                                                 __copy_range2, std::true_type());

    // testing  whether the sequences are intersected
    _ForwardIterator1 __left_bound_seq_1 = std::lower_bound(__first1, __last1, *__first2, __comp);

    if (__left_bound_seq_1 == __last1)
    {
        //{1} < {2}: seq2 is wholly greater than seq1, so, do parallel copying seq1 and seq2
        __par_backend::__parallel_invoke(
            std::forward<_ExecutionPolicy>(__exec),
            [=] {
                __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __result,
                                                  __copy_range1, std::true_type());
            },
            [=] {
                __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first2, __last2,
                                                  __result + __n1, __copy_range2, std::true_type());
            });
        return __result + __n1 + __n2;
    }

    // testing  whether the sequences are intersected
    _ForwardIterator2 __left_bound_seq_2 = std::lower_bound(__first2, __last2, *__first1, __comp);

    if (__left_bound_seq_2 == __last2)
    {
        //{2} < {1}: seq2 is wholly greater than seq1, so, do parallel copying seq1 and seq2
        __par_backend::__parallel_invoke(
            std::forward<_ExecutionPolicy>(__exec),
            [=] {
                __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first2, __last2, __result,
                                                  __copy_range2, std::true_type());
            },
            [=] {
                __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first1, __last1,
                                                  __result + __n2, __copy_range1, std::true_type());
            });
        return __result + __n1 + __n2;
    }

    const auto __m1 = __left_bound_seq_1 - __first1;
    if (__m1 > __set_algo_cut_off)
    {
        auto __res_or = __result;
        __result += __m1; //we know proper offset due to [first1; left_bound_seq_1) < [first2; last2)
        __par_backend::__parallel_invoke(
            std::forward<_ExecutionPolicy>(__exec),
            //do parallel copying of [first1; left_bound_seq_1)
            [=] {
                __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first1, __left_bound_seq_1,
                                                  __res_or, __copy_range1, std::true_type());
            },
            [=, &__result] {
                __result = __internal::__parallel_set_op(
                    std::forward<_ExecutionPolicy>(__exec), __left_bound_seq_1, __last1, __first2, __last2, __result,
                    __comp, [](_DifferenceType __n, _DifferenceType __m) { return __n + __m; }, __set_union_op,
                    __is_vector);
            });
        return __result;
    }

    const auto __m2 = __left_bound_seq_2 - __first2;
    _PSTL_ASSERT(__m1 == 0 || __m2 == 0);
    if (__m2 > __set_algo_cut_off)
    {
        auto __res_or = __result;
        __result += __m2; //we know proper offset due to [first2; left_bound_seq_2) < [first1; last1)
        __par_backend::__parallel_invoke(
            std::forward<_ExecutionPolicy>(__exec),
            //do parallel copying of [first2; left_bound_seq_2)
            [=] {
                __internal::__pattern_walk2_brick(std::forward<_ExecutionPolicy>(__exec), __first2, __left_bound_seq_2,
                                                  __res_or, __copy_range2, std::true_type());
            },
            [=, &__result] {
                __result = __internal::__parallel_set_op(
                    std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __left_bound_seq_2, __last2, __result,
                    __comp, [](_DifferenceType __n, _DifferenceType __m) { return __n + __m; }, __set_union_op,
                    __is_vector);
            });
        return __result;
    }

    return __internal::__parallel_set_op(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
        [](_DifferenceType __n, _DifferenceType __m) { return __n + __m; }, __set_union_op, __is_vector);
}

//------------------------------------------------------------------------
// set_union
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_union(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                  _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                  /*__is_vector=*/std::false_type) noexcept
{
    return std::set_union(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_union(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                  _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                  /*__is_vector=*/std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::set_union(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_union(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                    _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                    _IsVector __is_vector,
                    /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_set_union(__first1, __last1, __first2, __last2, __result, __comp, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_union(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                    _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                    _IsVector __is_vector, /*__is_parallel=*/std::true_type)
{

    const auto __n1 = __last1 - __first1;
    const auto __n2 = __last2 - __first2;

    // use serial algorithm
    if (__n1 + __n2 <= __set_algo_cut_off)
        return std::set_union(__first1, __last1, __first2, __last2, __result, __comp);

    typedef typename std::iterator_traits<_OutputIterator>::value_type _T;
    return __internal::__parallel_set_union_op(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
        [](_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2, _ForwardIterator2 __last2,
           _T* __result,
           _Compare __comp) { return std::set_union(__first1, __last1, __first2, __last2, __result, __comp); },
        __is_vector);
}

//------------------------------------------------------------------------
// set_intersection
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_intersection(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                         _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                         /*__is_vector=*/std::false_type) noexcept
{
    return std::set_intersection(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_intersection(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                         _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                         /*__is_vector=*/std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::set_intersection(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_intersection(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                           _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result,
                           _Compare __comp, _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_set_intersection(__first1, __last1, __first2, __last2, __result, __comp, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_intersection(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                           _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result,
                           _Compare __comp, _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    typedef typename std::iterator_traits<_OutputIterator>::value_type _T;
    typedef typename std::iterator_traits<_ForwardIterator1>::difference_type _DifferenceType;

    const auto __n1 = __last1 - __first1;
    const auto __n2 = __last2 - __first2;

    // intersection is empty
    if (__n1 == 0 || __n2 == 0)
        return __result;

    // testing  whether the sequences are intersected
    _ForwardIterator1 __left_bound_seq_1 = std::lower_bound(__first1, __last1, *__first2, __comp);
    //{1} < {2}: seq 2 is wholly greater than seq 1, so, the intersection is empty
    if (__left_bound_seq_1 == __last1)
        return __result;

    // testing  whether the sequences are intersected
    _ForwardIterator2 __left_bound_seq_2 = std::lower_bound(__first2, __last2, *__first1, __comp);
    //{2} < {1}: seq 1 is wholly greater than seq 2, so, the intersection is empty
    if (__left_bound_seq_2 == __last2)
        return __result;

    const auto __m1 = __last1 - __left_bound_seq_1 + __n2;
    if (__m1 > __set_algo_cut_off)
    {
        //we know proper offset due to [first1; left_bound_seq_1) < [first2; last2)
        return __internal::__parallel_set_op(
            std::forward<_ExecutionPolicy>(__exec), __left_bound_seq_1, __last1, __first2, __last2, __result, __comp,
            [](_DifferenceType __n, _DifferenceType __m) { return std::min(__n, __m); },
            [](_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
               _ForwardIterator2 __last2, _T* __result, _Compare __comp) {
                return std::set_intersection(__first1, __last1, __first2, __last2, __result, __comp);
            },
            __is_vector);
    }

    const auto __m2 = __last2 - __left_bound_seq_2 + __n1;
    if (__m2 > __set_algo_cut_off)
    {
        //we know proper offset due to [first2; left_bound_seq_2) < [first1; last1)
        __result = __internal::__parallel_set_op(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __left_bound_seq_2, __last2, __result, __comp,
            [](_DifferenceType __n, _DifferenceType __m) { return std::min(__n, __m); },
            [](_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
               _ForwardIterator2 __last2, _T* __result, _Compare __comp) {
                return std::set_intersection(__first2, __last2, __first1, __last1, __result, __comp);
            },
            __is_vector);
        return __result;
    }

    // [left_bound_seq_1; last1) and [left_bound_seq_2; last2) - use serial algorithm
    return std::set_intersection(__left_bound_seq_1, __last1, __left_bound_seq_2, __last2, __result, __comp);
}

//------------------------------------------------------------------------
// set_difference
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_difference(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                       _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                       /*__is_vector=*/std::false_type) noexcept
{
    return std::set_difference(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_difference(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                       _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                       /*__is_vector=*/std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::set_difference(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_difference(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                         _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result,
                         _Compare __comp, _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_set_difference(__first1, __last1, __first2, __last2, __result, __comp, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                         _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result,
                         _Compare __comp, _IsVector __is_vector, /*is_parallel=*/std::true_type)
{
    typedef typename std::iterator_traits<_OutputIterator>::value_type _T;
    typedef typename std::iterator_traits<_ForwardIterator1>::difference_type _DifferenceType;

    const auto __n1 = __last1 - __first1;
    const auto __n2 = __last2 - __first2;

    // {} \ {2}: the difference is empty
    if (__n1 == 0)
        return __result;

    // {1} \ {}: parallel copying just first sequence
    if (__n2 == 0)
        return __internal::__pattern_walk2_brick(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __result,
            [__is_vector](_ForwardIterator1 __begin, _ForwardIterator1 __end, _OutputIterator __res) {
                return __internal::__brick_copy(__begin, __end, __res, __is_vector);
            },
            std::true_type());

    // testing  whether the sequences are intersected
    _ForwardIterator1 __left_bound_seq_1 = std::lower_bound(__first1, __last1, *__first2, __comp);
    //{1} < {2}: seq 2 is wholly greater than seq 1, so, parallel copying just first sequence
    if (__left_bound_seq_1 == __last1)
        return __internal::__pattern_walk2_brick(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __result,
            [__is_vector](_ForwardIterator1 __begin, _ForwardIterator1 __end, _OutputIterator __res) {
                return __internal::__brick_copy(__begin, __end, __res, __is_vector);
            },
            std::true_type());

    // testing  whether the sequences are intersected
    _ForwardIterator2 __left_bound_seq_2 = std::lower_bound(__first2, __last2, *__first1, __comp);
    //{2} < {1}: seq 1 is wholly greater than seq 2, so, parallel copying just first sequence
    if (__left_bound_seq_2 == __last2)
        return __internal::__pattern_walk2_brick(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __result,
            [__is_vector](_ForwardIterator1 __begin, _ForwardIterator1 __end, _OutputIterator __res) {
                return __internal::__brick_copy(__begin, __end, __res, __is_vector);
            },
            std::true_type());

    if (__n1 + __n2 > __set_algo_cut_off)
        return __internal::__parallel_set_op(
            std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
            [](_DifferenceType __n, _DifferenceType __m) { return __n; },
            [](_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
               _ForwardIterator2 __last2, _T* __result,
               _Compare __comp) { return std::set_difference(__first1, __last1, __first2, __last2, __result, __comp); },
            __is_vector);

    // use serial algorithm
    return std::set_difference(__first1, __last1, __first2, __last2, __result, __comp);
}

//------------------------------------------------------------------------
// set_symmetric_difference
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_symmetric_difference(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                                 _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                                 /*__is_vector=*/std::false_type) noexcept
{
    return std::set_symmetric_difference(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator
__brick_set_symmetric_difference(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                                 _ForwardIterator2 __last2, _OutputIterator __result, _Compare __comp,
                                 /*__is_vector=*/std::true_type) noexcept
{
    _PSTL_PRAGMA_MESSAGE("Vectorized algorithm unimplemented, redirected to serial");
    return std::set_symmetric_difference(__first1, __last1, __first2, __last2, __result, __comp);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_symmetric_difference(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                                   _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result,
                                   _Compare __comp, _IsVector __is_vector, /*is_parallel=*/std::false_type) noexcept
{
    return __internal::__brick_set_symmetric_difference(__first1, __last1, __first2, __last2, __result, __comp,
                                                        __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_symmetric_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                                   _ForwardIterator2 __first2, _ForwardIterator2 __last2, _OutputIterator __result,
                                   _Compare __comp, _IsVector __is_vector, /*is_parallel=*/std::true_type)
{

    const auto __n1 = __last1 - __first1;
    const auto __n2 = __last2 - __first2;

    // use serial algorithm
    if (__n1 + __n2 <= __set_algo_cut_off)
        return std::set_symmetric_difference(__first1, __last1, __first2, __last2, __result, __comp);

    typedef typename std::iterator_traits<_OutputIterator>::value_type _T;
    return __internal::__parallel_set_union_op(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
        [](_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2, _ForwardIterator2 __last2,
           _T* __result, _Compare __comp) {
            return std::set_symmetric_difference(__first1, __last1, __first2, __last2, __result, __comp);
        },
        __is_vector);
}

//------------------------------------------------------------------------
// is_heap_until
//------------------------------------------------------------------------

template <class _RandomAccessIterator, class _Compare>
_RandomAccessIterator
__brick_is_heap_until(_RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp,
                      /* __is_vector = */ std::false_type) noexcept
{
    return std::is_heap_until(__first, __last, __comp);
}

template <class _RandomAccessIterator, class _Compare>
_RandomAccessIterator
__brick_is_heap_until(_RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp,
                      /* __is_vector = */ std::true_type) noexcept
{
    if (__last - __first < 2)
        return __last;
    typedef typename std::iterator_traits<_RandomAccessIterator>::difference_type _SizeType;
    return __unseq_backend::__simd_first(
        __first, _SizeType(0), __last - __first,
        [&__comp](_RandomAccessIterator __it, _SizeType __i) { return __comp(__it[(__i - 1) / 2], __it[__i]); });
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_is_heap_until(_ExecutionPolicy&&, _RandomAccessIterator __first, _RandomAccessIterator __last,
                        _Compare __comp, _IsVector __is_vector, /* is_parallel = */ std::false_type) noexcept
{
    return __internal::__brick_is_heap_until(__first, __last, __comp, __is_vector);
}

template <class _RandomAccessIterator, class _DifferenceType, class _Compare>
_RandomAccessIterator
__is_heap_until_local(_RandomAccessIterator __first, _DifferenceType __begin, _DifferenceType __end, _Compare __comp,
                      /* __is_vector = */ std::false_type) noexcept
{
    _DifferenceType __i = __begin;
    for (; __i < __end; ++__i)
    {
        if (__comp(__first[(__i - 1) / 2], __first[__i]))
        {
            break;
        }
    }
    return __first + __i;
}

template <class _RandomAccessIterator, class _DifferenceType, class _Compare>
_RandomAccessIterator
__is_heap_until_local(_RandomAccessIterator __first, _DifferenceType __begin, _DifferenceType __end, _Compare __comp,
                      /* __is_vector = */ std::true_type) noexcept
{
    return __unseq_backend::__simd_first(
        __first, __begin, __end,
        [&__comp](_RandomAccessIterator __it, _DifferenceType __i) { return __comp(__it[(__i - 1) / 2], __it[__i]); });
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_is_heap_until(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                        _Compare __comp, _IsVector __is_vector, /* is_parallel = */ std::true_type) noexcept
{
    if (__last - __first < 2)
        return __last;

    return __internal::__except_handler([&]() {
        return __parallel_find(
            std::forward<_ExecutionPolicy>(__exec), __first, __last,
            [__first, __comp, __is_vector](_RandomAccessIterator __i, _RandomAccessIterator __j) {
                return __internal::__is_heap_until_local(__first, __i - __first, __j - __first, __comp, __is_vector);
            },
            std::less<typename std::iterator_traits<_RandomAccessIterator>::difference_type>(), /*is_first=*/true);
    });
}

//------------------------------------------------------------------------
// min_element
//------------------------------------------------------------------------

template <typename _ForwardIterator, typename _Compare>
_ForwardIterator
__brick_min_element(_ForwardIterator __first, _ForwardIterator __last, _Compare __comp,
                    /* __is_vector = */ std::false_type) noexcept
{
    return std::min_element(__first, __last, __comp);
}

template <typename _ForwardIterator, typename _Compare>
_ForwardIterator
__brick_min_element(_ForwardIterator __first, _ForwardIterator __last, _Compare __comp,
                    /* __is_vector = */ std::true_type) noexcept
{
#if _PSTL_UDR_PRESENT
    return __unseq_backend::__simd_min_element(__first, __last - __first, __comp);
#else
    return std::min_element(__first, __last, __comp);
#endif
}

template <typename _ExecutionPolicy, typename _ForwardIterator, typename _Compare, typename _IsVector>
_ForwardIterator
__pattern_min_element(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp,
                      _IsVector __is_vector, /* is_parallel = */ std::false_type) noexcept
{
    return __internal::__brick_min_element(__first, __last, __comp, __is_vector);
}

template <typename _ExecutionPolicy, typename _RandomAccessIterator, typename _Compare, typename _IsVector>
_RandomAccessIterator
__pattern_min_element(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last,
                      _Compare __comp, _IsVector __is_vector, /* is_parallel = */ std::true_type)
{
    if (__first == __last)
        return __last;

    return __internal::__except_handler([&]() {
        return __par_backend::__parallel_reduce(
            std::forward<_ExecutionPolicy>(__exec), __first + 1, __last, __first,
            [=](_RandomAccessIterator __begin, _RandomAccessIterator __end,
                _RandomAccessIterator __init) -> _RandomAccessIterator {
                const _RandomAccessIterator subresult =
                    __internal::__brick_min_element(__begin, __end, __comp, __is_vector);
                return __internal::__cmp_iterators_by_values(__init, subresult, __comp);
            },
            [=](_RandomAccessIterator __it1, _RandomAccessIterator __it2) -> _RandomAccessIterator {
                return __internal::__cmp_iterators_by_values(__it1, __it2, __comp);
            });
    });
}

//------------------------------------------------------------------------
// minmax_element
//------------------------------------------------------------------------

template <typename _ForwardIterator, typename _Compare>
std::pair<_ForwardIterator, _ForwardIterator>
__brick_minmax_element(_ForwardIterator __first, _ForwardIterator __last, _Compare __comp,
                       /* __is_vector = */ std::false_type) noexcept
{
    return std::minmax_element(__first, __last, __comp);
}

template <typename _ForwardIterator, typename _Compare>
std::pair<_ForwardIterator, _ForwardIterator>
__brick_minmax_element(_ForwardIterator __first, _ForwardIterator __last, _Compare __comp,
                       /* __is_vector = */ std::true_type) noexcept
{
#if _PSTL_UDR_PRESENT
    return __unseq_backend::__simd_minmax_element(__first, __last - __first, __comp);
#else
    return std::minmax_element(__first, __last, __comp);
#endif
}

template <typename _ExecutionPolicy, typename _ForwardIterator, typename _Compare, typename _IsVector>
std::pair<_ForwardIterator, _ForwardIterator>
__pattern_minmax_element(_ExecutionPolicy&&, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp,
                         _IsVector __is_vector, /* is_parallel = */ std::false_type) noexcept
{
    return __internal::__brick_minmax_element(__first, __last, __comp, __is_vector);
}

template <typename _ExecutionPolicy, typename _ForwardIterator, typename _Compare, typename _IsVector>
std::pair<_ForwardIterator, _ForwardIterator>
__pattern_minmax_element(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp,
                         _IsVector __is_vector, /* is_parallel = */ std::true_type)
{
    if (__first == __last)
        return std::make_pair(__first, __first);

    return __internal::__except_handler([&]() {
        typedef std::pair<_ForwardIterator, _ForwardIterator> _Result;

        return __par_backend::__parallel_reduce(
            std::forward<_ExecutionPolicy>(__exec), __first + 1, __last, std::make_pair(__first, __first),
            [=](_ForwardIterator __begin, _ForwardIterator __end, _Result __init) -> _Result {
                const _Result __subresult = __internal::__brick_minmax_element(__begin, __end, __comp, __is_vector);
                return std::make_pair(__internal::__cmp_iterators_by_values(__subresult.first, __init.first, __comp),
                                      __internal::__cmp_iterators_by_values(__init.second, __subresult.second,
                                                                            __not_pred<_Compare>(__comp)));
            },
            [=](_Result __p1, _Result __p2) -> _Result {
                return std::make_pair(
                    __internal::__cmp_iterators_by_values(__p1.first, __p2.first, __comp),
                    __internal::__cmp_iterators_by_values(__p2.second, __p1.second, __not_pred<_Compare>(__comp)));
            });
    });
}

//------------------------------------------------------------------------
// mismatch
//------------------------------------------------------------------------
template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
std::pair<_ForwardIterator1, _ForwardIterator2>
__mismatch_serial(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                  _ForwardIterator2 __last2, _BinaryPredicate __pred)
{
#if _PSTL_CPP14_2RANGE_MISMATCH_EQUAL_PRESENT
    return std::mismatch(__first1, __last1, __first2, __last2, __pred);
#else
    for (; __first1 != __last1 && __first2 != __last2 && __pred(*__first1, *__first2); ++__first1, ++__first2)
    {
    }
    return std::make_pair(__first1, __first2);
#endif
}

template <class _ForwardIterator1, class _ForwardIterator2, class _Predicate>
std::pair<_ForwardIterator1, _ForwardIterator2>
__brick_mismatch(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                 _ForwardIterator2 __last2, _Predicate __pred, /* __is_vector = */ std::false_type) noexcept
{
    return __mismatch_serial(__first1, __last1, __first2, __last2, __pred);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _Predicate>
std::pair<_ForwardIterator1, _ForwardIterator2>
__brick_mismatch(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                 _ForwardIterator2 __last2, _Predicate __pred, /* __is_vector = */ std::true_type) noexcept
{
    auto __n = std::min(__last1 - __first1, __last2 - __first2);
    return __unseq_backend::__simd_first(__first1, __n, __first2, __not_pred<_Predicate>(__pred));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Predicate, class _IsVector>
std::pair<_ForwardIterator1, _ForwardIterator2>
__pattern_mismatch(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                   _ForwardIterator2 __first2, _ForwardIterator2 __last2, _Predicate __pred, _IsVector __is_vector,
                   /* is_parallel = */ std::false_type) noexcept
{
    return __internal::__brick_mismatch(__first1, __last1, __first2, __last2, __pred, __is_vector);
}

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _Predicate,
          class _IsVector>
std::pair<_RandomAccessIterator1, _RandomAccessIterator2>
__pattern_mismatch(_ExecutionPolicy&& __exec, _RandomAccessIterator1 __first1, _RandomAccessIterator1 __last1,
                   _RandomAccessIterator2 __first2, _RandomAccessIterator2 __last2, _Predicate __pred,
                   _IsVector __is_vector, /* is_parallel = */ std::true_type) noexcept
{
    return __internal::__except_handler([&]() {
        auto __n = std::min(__last1 - __first1, __last2 - __first2);
        auto __result = __internal::__parallel_find(
            std::forward<_ExecutionPolicy>(__exec), __first1, __first1 + __n,
            [__first1, __first2, __pred, __is_vector](_RandomAccessIterator1 __i, _RandomAccessIterator1 __j) {
                return __internal::__brick_mismatch(__i, __j, __first2 + (__i - __first1), __first2 + (__j - __first1),
                                                    __pred, __is_vector)
                    .first;
            },
            std::less<typename std::iterator_traits<_RandomAccessIterator1>::difference_type>(), /*is_first=*/true);
        return std::make_pair(__result, __first2 + (__result - __first1));
    });
}

//------------------------------------------------------------------------
// lexicographical_compare
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _Compare>
bool
__brick_lexicographical_compare(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                                _ForwardIterator2 __last2, _Compare __comp,
                                /* __is_vector = */ std::false_type) noexcept
{
    return std::lexicographical_compare(__first1, __last1, __first2, __last2, __comp);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _Compare>
bool
__brick_lexicographical_compare(_ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
                                _ForwardIterator2 __last2, _Compare __comp, /* __is_vector = */ std::true_type) noexcept
{
    if (__first2 == __last2)
    { // if second sequence is empty
        return false;
    }
    else if (__first1 == __last1)
    { // if first sequence is empty
        return true;
    }
    else
    {
        typedef typename std::iterator_traits<_ForwardIterator1>::reference ref_type1;
        typedef typename std::iterator_traits<_ForwardIterator2>::reference ref_type2;
        --__last1;
        --__last2;
        auto __n = std::min(__last1 - __first1, __last2 - __first2);
        std::pair<_ForwardIterator1, _ForwardIterator2> __result = __unseq_backend::__simd_first(
            __first1, __n, __first2, [__comp](const ref_type1 __x, const ref_type2 __y) mutable {
                return __comp(__x, __y) || __comp(__y, __x);
            });

        if (__result.first == __last1 && __result.second != __last2)
        { // if first sequence shorter than second
            return !__comp(*__result.second, *__result.first);
        }
        else
        { // if second sequence shorter than first or both have the same number of elements
            return __comp(*__result.first, *__result.second);
        }
    }
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_lexicographical_compare(_ExecutionPolicy&&, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                                  _ForwardIterator2 __first2, _ForwardIterator2 __last2, _Compare __comp,
                                  _IsVector __is_vector, /* is_parallel = */ std::false_type) noexcept
{
    return __internal::__brick_lexicographical_compare(__first1, __last1, __first2, __last2, __comp, __is_vector);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_lexicographical_compare(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                                  _ForwardIterator2 __first2, _ForwardIterator2 __last2, _Compare __comp,
                                  _IsVector __is_vector, /* is_parallel = */ std::true_type) noexcept
{
    if (__first2 == __last2)
    { // if second sequence is empty
        return false;
    }
    else if (__first1 == __last1)
    { // if first sequence is empty
        return true;
    }
    else
    {
        typedef typename std::iterator_traits<_ForwardIterator1>::reference _RefType1;
        typedef typename std::iterator_traits<_ForwardIterator2>::reference _RefType2;
        --__last1;
        --__last2;
        auto __n = std::min(__last1 - __first1, __last2 - __first2);
        auto __result = __internal::__parallel_find(
            std::forward<_ExecutionPolicy>(__exec), __first1, __first1 + __n,
            [__first1, __first2, &__comp, __is_vector](_ForwardIterator1 __i, _ForwardIterator1 __j) {
                return __internal::__brick_mismatch(__i, __j, __first2 + (__i - __first1), __first2 + (__j - __first1),
                                                    [&__comp](const _RefType1 __x, const _RefType2 __y) {
                                                        return !__comp(__x, __y) && !__comp(__y, __x);
                                                    },
                                                    __is_vector)
                    .first;
            },
            std::less<typename std::iterator_traits<_ForwardIterator1>::difference_type>(), /*is_first=*/true);

        if (__result == __last1 && __first2 + (__result - __first1) != __last2)
        { // if first sequence shorter than second
            return !__comp(*(__first2 + (__result - __first1)), *__result);
        }
        else
        { // if second sequence shorter than first or both have the same number of elements
            return __comp(*__result, *(__first2 + (__result - __first1)));
        }
    }
}

} // namespace __internal
} // namespace __pstl

#endif /* _PSTL_ALGORITHM_IMPL_H */
