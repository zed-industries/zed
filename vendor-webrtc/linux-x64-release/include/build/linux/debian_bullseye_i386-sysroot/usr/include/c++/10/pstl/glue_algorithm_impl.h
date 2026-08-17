// -*- C++ -*-
//===-- glue_algorithm_impl.h ---------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_GLUE_ALGORITHM_IMPL_H
#define _PSTL_GLUE_ALGORITHM_IMPL_H

#include <functional>

#include "execution_defs.h"
#include "utils.h"
#include "algorithm_fwd.h"
#include "numeric_fwd.h" /* count and count_if use __pattern_transform_reduce */

namespace std
{

// [alg.any_of]

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
any_of(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_any_of(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

// [alg.all_of]

template <class _ExecutionPolicy, class _ForwardIterator, class _Pred>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
all_of(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Pred __pred)
{
    return !std::any_of(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                        __pstl::__internal::__not_pred<_Pred>(__pred));
}

// [alg.none_of]

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
none_of(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred)
{
    return !std::any_of(std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred);
}

// [alg.foreach]

template <class _ExecutionPolicy, class _ForwardIterator, class _Function>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
for_each(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Function __f)
{
    using namespace __pstl;
    __internal::__pattern_walk1(std::forward<_ExecutionPolicy>(__exec), __first, __last, __f,
                                __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
                                __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Function>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
for_each_n(_ExecutionPolicy&& __exec, _ForwardIterator __first, _Size __n, _Function __f)
{
    using namespace __pstl;
    return __internal::__pattern_walk1_n(
        std::forward<_ExecutionPolicy>(__exec), __first, __n, __f,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

// [alg.find]

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
find_if(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_find_if(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
find_if_not(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred)
{
    return std::find_if(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                        __pstl::__internal::__not_pred<_Predicate>(__pred));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
find(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, const _Tp& __value)
{
    return std::find_if(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                        __pstl::__internal::__equal_value<_Tp>(__value));
}

// [alg.find.end]
template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator1>
find_end(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
         _ForwardIterator2 __s_last, _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_find_end(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __s_first, __s_last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator1>
find_end(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
         _ForwardIterator2 __s_last)
{
    return std::find_end(std::forward<_ExecutionPolicy>(__exec), __first, __last, __s_first, __s_last,
                         __pstl::__internal::__pstl_equal());
}

// [alg.find_first_of]
template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator1>
find_first_of(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
              _ForwardIterator2 __s_first, _ForwardIterator2 __s_last, _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_find_first_of(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __s_first, __s_last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator1>
find_first_of(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
              _ForwardIterator2 __s_first, _ForwardIterator2 __s_last)
{
    return std::find_first_of(std::forward<_ExecutionPolicy>(__exec), __first, __last, __s_first, __s_last,
                              __pstl::__internal::__pstl_equal());
}

// [alg.adjacent_find]
template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
adjacent_find(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    typedef typename iterator_traits<_ForwardIterator>::value_type _ValueType;
    using namespace __pstl;
    return __internal::__pattern_adjacent_find(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, std::equal_to<_ValueType>(),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec), /*first_semantic*/ false);
}

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
adjacent_find(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_adjacent_find(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec), /*first_semantic*/ false);
}

// [alg.count]

// Implementation note: count and count_if call the pattern directly instead of calling std::transform_reduce
// so that we do not have to include <numeric>.

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy,
                                                 typename iterator_traits<_ForwardIterator>::difference_type>
count(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, const _Tp& __value)
{
    typedef typename iterator_traits<_ForwardIterator>::value_type _ValueType;
    using namespace __pstl;
    return __internal::__pattern_count(
        std::forward<_ExecutionPolicy>(__exec), __first, __last,
        [&__value](const _ValueType& __x) { return __value == __x; },
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy,
                                                 typename iterator_traits<_ForwardIterator>::difference_type>
count_if(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Predicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_count(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

// [alg.search]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator1>
search(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
       _ForwardIterator2 __s_last, _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_search(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __s_first, __s_last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator1>
search(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
       _ForwardIterator2 __s_last)
{
    return std::search(std::forward<_ExecutionPolicy>(__exec), __first, __last, __s_first, __s_last,
                       __pstl::__internal::__pstl_equal());
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Tp, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
search_n(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Size __count,
         const _Tp& __value, _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_search_n(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __count, __value, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
search_n(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Size __count,
         const _Tp& __value)
{
    return std::search_n(std::forward<_ExecutionPolicy>(__exec), __first, __last, __count, __value,
                         std::equal_to<typename iterator_traits<_ForwardIterator>::value_type>());
}

// [alg.copy]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
copy(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __result)
{
    using namespace __pstl;
    const auto __is_vector =
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec);

    return __internal::__pattern_walk2_brick(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __result,
        [__is_vector](_ForwardIterator1 __begin, _ForwardIterator1 __end, _ForwardIterator2 __res) {
            return __internal::__brick_copy(__begin, __end, __res, __is_vector);
        },
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _Size, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
copy_n(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _Size __n, _ForwardIterator2 __result)
{
    using namespace __pstl;
    const auto __is_vector =
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec);

    return __internal::__pattern_walk2_brick_n(
        std::forward<_ExecutionPolicy>(__exec), __first, __n, __result,
        [__is_vector](_ForwardIterator1 __begin, _Size __sz, _ForwardIterator2 __res) {
            return __internal::__brick_copy_n(__begin, __sz, __res, __is_vector);
        },
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Predicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
copy_if(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __result,
        _Predicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_copy_if(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

// [alg.swap]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
swap_ranges(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
            _ForwardIterator2 __first2)
{
    using namespace __pstl;
    typedef typename iterator_traits<_ForwardIterator1>::reference _ReferenceType1;
    typedef typename iterator_traits<_ForwardIterator2>::reference _ReferenceType2;
    return __internal::__pattern_walk2(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2,
        [](_ReferenceType1 __x, _ReferenceType2 __y) {
            using std::swap;
            swap(__x, __y);
        },
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

// [alg.transform]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _UnaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
transform(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __result,
          _UnaryOperation __op)
{
    typedef typename iterator_traits<_ForwardIterator1>::reference _InputType;
    typedef typename iterator_traits<_ForwardIterator2>::reference _OutputType;
    using namespace __pstl;
    return __internal::__pattern_walk2(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __result,
        [__op](_InputType __x, _OutputType __y) mutable { __y = __op(__x); },
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator,
          class _BinaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
transform(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
          _ForwardIterator __result, _BinaryOperation __op)
{
    typedef typename iterator_traits<_ForwardIterator1>::reference _Input1Type;
    typedef typename iterator_traits<_ForwardIterator2>::reference _Input2Type;
    typedef typename iterator_traits<_ForwardIterator>::reference _OutputType;
    using namespace __pstl;
    return __internal::__pattern_walk3(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __result,
        [__op](_Input1Type x, _Input2Type y, _OutputType z) mutable { z = __op(x, y); },
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                   _ForwardIterator>(__exec));
}

// [alg.replace]

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
replace_if(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred,
           const _Tp& __new_value)
{
    using namespace __pstl;
    typedef typename iterator_traits<_ForwardIterator>::reference _ElementType;
    __internal::__pattern_walk1(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                                [&__pred, &__new_value](_ElementType __elem) {
                                    if (__pred(__elem))
                                    {
                                        __elem = __new_value;
                                    }
                                },
                                __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
                                __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
replace(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, const _Tp& __old_value,
        const _Tp& __new_value)
{
    std::replace_if(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                    __pstl::__internal::__equal_value<_Tp>(__old_value), __new_value);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _UnaryPredicate, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
replace_copy_if(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                _ForwardIterator2 __result, _UnaryPredicate __pred, const _Tp& __new_value)
{
    typedef typename iterator_traits<_ForwardIterator1>::reference _InputType;
    typedef typename iterator_traits<_ForwardIterator2>::reference _OutputType;
    using namespace __pstl;
    return __internal::__pattern_walk2(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __result,
        [__pred, &__new_value](_InputType __x, _OutputType __y) mutable { __y = __pred(__x) ? __new_value : __x; },
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
replace_copy(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __result,
             const _Tp& __old_value, const _Tp& __new_value)
{
    return std::replace_copy_if(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result,
                                __pstl::__internal::__equal_value<_Tp>(__old_value), __new_value);
}

// [alg.fill]

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
fill(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, const _Tp& __value)
{
    using namespace __pstl;
    __internal::__pattern_fill(std::forward<_ExecutionPolicy>(__exec), __first, __last, __value,
                               __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
                               __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
fill_n(_ExecutionPolicy&& __exec, _ForwardIterator __first, _Size __count, const _Tp& __value)
{
    if (__count <= 0)
        return __first;

    using namespace __pstl;
    return __internal::__pattern_fill_n(
        std::forward<_ExecutionPolicy>(__exec), __first, __count, __value,
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

// [alg.generate]
template <class _ExecutionPolicy, class _ForwardIterator, class _Generator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
generate(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Generator __g)
{
    using namespace __pstl;
    __internal::__pattern_generate(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __g,
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Generator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
generate_n(_ExecutionPolicy&& __exec, _ForwardIterator __first, _Size __count, _Generator __g)
{
    if (__count <= 0)
        return __first;

    using namespace __pstl;
    return __internal::__pattern_generate_n(
        std::forward<_ExecutionPolicy>(__exec), __first, __count, __g,
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

// [alg.remove]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Predicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
remove_copy_if(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
               _ForwardIterator2 __result, _Predicate __pred)
{
    return std::copy_if(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result,
                        __pstl::__internal::__not_pred<_Predicate>(__pred));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
remove_copy(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __result,
            const _Tp& __value)
{
    return std::copy_if(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result,
                        __pstl::__internal::__not_equal_value<_Tp>(__value));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
remove_if(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_remove_if(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
remove(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, const _Tp& __value)
{
    return std::remove_if(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                          __pstl::__internal::__equal_value<_Tp>(__value));
}

// [alg.unique]

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
unique(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_unique(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
unique(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    return std::unique(std::forward<_ExecutionPolicy>(__exec), __first, __last, __pstl::__internal::__pstl_equal());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
unique_copy(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __result,
            _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_unique_copy(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
unique_copy(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __result)
{
    return std::unique_copy(__exec, __first, __last, __result, __pstl::__internal::__pstl_equal());
}

// [alg.reverse]

template <class _ExecutionPolicy, class _BidirectionalIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
reverse(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __last)
{
    using namespace __pstl;
    __internal::__pattern_reverse(
        std::forward<_ExecutionPolicy>(__exec), __first, __last,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _BidirectionalIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _BidirectionalIterator>(__exec));
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
reverse_copy(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __last,
             _ForwardIterator __d_first)
{
    using namespace __pstl;
    return __internal::__pattern_reverse_copy(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __d_first,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _BidirectionalIterator, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _BidirectionalIterator, _ForwardIterator>(__exec));
}

// [alg.rotate]

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
rotate(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __middle, _ForwardIterator __last)
{
    using namespace __pstl;
    return __internal::__pattern_rotate(
        std::forward<_ExecutionPolicy>(__exec), __first, __middle, __last,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
rotate_copy(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __middle, _ForwardIterator1 __last,
            _ForwardIterator2 __result)
{
    using namespace __pstl;
    return __internal::__pattern_rotate_copy(
        std::forward<_ExecutionPolicy>(__exec), __first, __middle, __last, __result,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

// [alg.partitions]

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
is_partitioned(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_is_partitioned(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
partition(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _UnaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_partition(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _UnaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _BidirectionalIterator>
stable_partition(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __last,
                 _UnaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_stable_partition(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _BidirectionalIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _BidirectionalIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _ForwardIterator1, class _ForwardIterator2,
          class _UnaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, std::pair<_ForwardIterator1, _ForwardIterator2>>
partition_copy(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last,
               _ForwardIterator1 __out_true, _ForwardIterator2 __out_false, _UnaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_partition_copy(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __out_true, __out_false, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator, _ForwardIterator1,
                                                 _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator, _ForwardIterator1,
                                                   _ForwardIterator2>(__exec));
}

// [alg.sort]

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp)
{
    typedef typename iterator_traits<_RandomAccessIterator>::value_type _InputType;
    using namespace __pstl;
    return __internal::__pattern_sort(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec),
        typename std::is_move_constructible<_InputType>::type());
}

template <class _ExecutionPolicy, class _RandomAccessIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last)
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _InputType;
    std::sort(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_InputType>());
}

// [stable.sort]

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
stable_sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_stable_sort(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec));
}

template <class _ExecutionPolicy, class _RandomAccessIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
stable_sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last)
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _InputType;
    std::stable_sort(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_InputType>());
}

// [mismatch]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, std::pair<_ForwardIterator1, _ForwardIterator2>>
mismatch(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
         _ForwardIterator2 __last2, _BinaryPredicate __pred)
{
    using namespace __pstl;
    return __internal::__pattern_mismatch(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __pred,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, std::pair<_ForwardIterator1, _ForwardIterator2>>
mismatch(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
         _BinaryPredicate __pred)
{
    return std::mismatch(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2,
			 std::next(__first2, std::distance(__first1, __last1)), __pred);
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, std::pair<_ForwardIterator1, _ForwardIterator2>>
mismatch(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
         _ForwardIterator2 __last2)
{
    return std::mismatch(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2,
                         __pstl::__internal::__pstl_equal());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, std::pair<_ForwardIterator1, _ForwardIterator2>>
mismatch(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2)
{
    //TODO: to get rid of "distance"
    return std::mismatch(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2,
                         std::next(__first2, std::distance(__first1, __last1)));
}

// [alg.equal]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
equal(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
      _BinaryPredicate __p)
{
    using namespace __pstl;
    return __internal::__pattern_equal(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __p,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
equal(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2)
{
    return std::equal(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2,
                      __pstl::__internal::__pstl_equal());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
equal(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
      _ForwardIterator2 __last2, _BinaryPredicate __p)
{
    using namespace __pstl;
    return __internal::__pattern_equal(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __p,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
equal(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
      _ForwardIterator2 __last2)
{
    return std::equal(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2,
                      __pstl::__internal::__pstl_equal());
}

// [alg.move]
template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
move(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __d_first)
{
    using namespace __pstl;
    const auto __is_vector =
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec);

    return __internal::__pattern_walk2_brick(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __d_first,
        [__is_vector](_ForwardIterator1 __begin, _ForwardIterator1 __end, _ForwardIterator2 __res) {
            return __internal::__brick_move(__begin, __end, __res, __is_vector);
        },
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

// [partial.sort]

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
partial_sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __middle,
             _RandomAccessIterator __last, _Compare __comp)
{
    using namespace __pstl;
    __internal::__pattern_partial_sort(
        std::forward<_ExecutionPolicy>(__exec), __first, __middle, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec));
}

template <class _ExecutionPolicy, class _RandomAccessIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
partial_sort(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __middle,
             _RandomAccessIterator __last)
{
    typedef typename iterator_traits<_RandomAccessIterator>::value_type _InputType;
    std::partial_sort(std::forward<_ExecutionPolicy>(__exec), __first, __middle, __last, std::less<_InputType>());
}

// [partial.sort.copy]

template <class _ExecutionPolicy, class _ForwardIterator, class _RandomAccessIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _RandomAccessIterator>
partial_sort_copy(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last,
                  _RandomAccessIterator __d_first, _RandomAccessIterator __d_last, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_partial_sort_copy(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __d_first, __d_last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator, _RandomAccessIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator, _RandomAccessIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _RandomAccessIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _RandomAccessIterator>
partial_sort_copy(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last,
                  _RandomAccessIterator __d_first, _RandomAccessIterator __d_last)
{
    return std::partial_sort_copy(std::forward<_ExecutionPolicy>(__exec), __first, __last, __d_first, __d_last,
                                  __pstl::__internal::__pstl_less());
}

// [is.sorted]
template <class _ExecutionPolicy, class _ForwardIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
is_sorted_until(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp)
{
    using namespace __pstl;
    const _ForwardIterator __res = __internal::__pattern_adjacent_find(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __pstl::__internal::__reorder_pred<_Compare>(__comp),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec), /*first_semantic*/ false);
    return __res == __last ? __last : std::next(__res);
}

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
is_sorted_until(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    typedef typename std::iterator_traits<_ForwardIterator>::value_type _InputType;
    return is_sorted_until(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_InputType>());
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
is_sorted(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_adjacent_find(
               std::forward<_ExecutionPolicy>(__exec), __first, __last, __internal::__reorder_pred<_Compare>(__comp),
               __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
               __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
               /*or_semantic*/ true) == __last;
}

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
is_sorted(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    typedef typename std::iterator_traits<_ForwardIterator>::value_type _InputType;
    return std::is_sorted(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_InputType>());
}

// [alg.merge]
template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator,
          class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
merge(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
      _ForwardIterator2 __last2, _ForwardIterator __d_first, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_merge(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __d_first, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                   _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
merge(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
      _ForwardIterator2 __last2, _ForwardIterator __d_first)
{
    return std::merge(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __d_first,
                      __pstl::__internal::__pstl_less());
}

template <class _ExecutionPolicy, class _BidirectionalIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
inplace_merge(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __middle,
              _BidirectionalIterator __last, _Compare __comp)
{
    using namespace __pstl;
    __internal::__pattern_inplace_merge(
        std::forward<_ExecutionPolicy>(__exec), __first, __middle, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _BidirectionalIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _BidirectionalIterator>(__exec));
}

template <class _ExecutionPolicy, class _BidirectionalIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
inplace_merge(_ExecutionPolicy&& __exec, _BidirectionalIterator __first, _BidirectionalIterator __middle,
              _BidirectionalIterator __last)
{
    typedef typename std::iterator_traits<_BidirectionalIterator>::value_type _InputType;
    std::inplace_merge(std::forward<_ExecutionPolicy>(__exec), __first, __middle, __last, std::less<_InputType>());
}

// [includes]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
includes(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
         _ForwardIterator2 __last2, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_includes(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
includes(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
         _ForwardIterator2 __last2)
{
    return std::includes(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2,
                         __pstl::__internal::__pstl_less());
}

// [set.union]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator,
          class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_union(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
          _ForwardIterator2 __last2, _ForwardIterator __result, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_set_union(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                   _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_union(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1, _ForwardIterator2 __first2,
          _ForwardIterator2 __last2, _ForwardIterator __result)
{
    return std::set_union(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result,
                          __pstl::__internal::__pstl_less());
}

// [set.intersection]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator,
          class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_intersection(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                 _ForwardIterator2 __first2, _ForwardIterator2 __last2, _ForwardIterator __result, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_set_intersection(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                   _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_intersection(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                 _ForwardIterator2 __first2, _ForwardIterator2 __last2, _ForwardIterator __result)
{
    return std::set_intersection(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result,
                                 __pstl::__internal::__pstl_less());
}

// [set.difference]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator,
          class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
               _ForwardIterator2 __first2, _ForwardIterator2 __last2, _ForwardIterator __result, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_set_difference(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                   _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
               _ForwardIterator2 __first2, _ForwardIterator2 __last2, _ForwardIterator __result)
{
    return std::set_difference(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result,
                               __pstl::__internal::__pstl_less());
}

// [set.symmetric.difference]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator,
          class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_symmetric_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                         _ForwardIterator2 __first2, _ForwardIterator2 __last2, _ForwardIterator __result,
                         _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_set_symmetric_difference(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __result, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2,
                                                   _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
set_symmetric_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                         _ForwardIterator2 __first2, _ForwardIterator2 __last2, _ForwardIterator __result)
{
    return std::set_symmetric_difference(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2,
                                         __result, __pstl::__internal::__pstl_less());
}

// [is.heap]
template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _RandomAccessIterator>
is_heap_until(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_is_heap_until(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec));
}

template <class _ExecutionPolicy, class _RandomAccessIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _RandomAccessIterator>
is_heap_until(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last)
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _InputType;
    return std::is_heap_until(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_InputType>());
}

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
is_heap(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last, _Compare __comp)
{
    return std::is_heap_until(std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp) == __last;
}

template <class _ExecutionPolicy, class _RandomAccessIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
is_heap(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __last)
{
    typedef typename std::iterator_traits<_RandomAccessIterator>::value_type _InputType;
    return std::is_heap(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_InputType>());
}

// [alg.min.max]

template <class _ExecutionPolicy, class _ForwardIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
min_element(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_min_element(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
min_element(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    typedef typename std::iterator_traits<_ForwardIterator>::value_type _InputType;
    return std::min_element(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_InputType>());
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
max_element(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp)
{
    return min_element(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                       __pstl::__internal::__reorder_pred<_Compare>(__comp));
}

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator>
max_element(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    typedef typename std::iterator_traits<_ForwardIterator>::value_type _InputType;
    return std::min_element(std::forward<_ExecutionPolicy>(__exec), __first, __last,
                            __pstl::__internal::__reorder_pred<std::less<_InputType>>(std::less<_InputType>()));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, std::pair<_ForwardIterator, _ForwardIterator>>
minmax_element(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_minmax_element(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, std::pair<_ForwardIterator, _ForwardIterator>>
minmax_element(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    typedef typename iterator_traits<_ForwardIterator>::value_type _ValueType;
    return std::minmax_element(std::forward<_ExecutionPolicy>(__exec), __first, __last, std::less<_ValueType>());
}

// [alg.nth.element]

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
nth_element(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __nth,
            _RandomAccessIterator __last, _Compare __comp)
{
    using namespace __pstl;
    __internal::__pattern_nth_element(
        std::forward<_ExecutionPolicy>(__exec), __first, __nth, __last, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _RandomAccessIterator>(__exec));
}

template <class _ExecutionPolicy, class _RandomAccessIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, void>
nth_element(_ExecutionPolicy&& __exec, _RandomAccessIterator __first, _RandomAccessIterator __nth,
            _RandomAccessIterator __last)
{
    typedef typename iterator_traits<_RandomAccessIterator>::value_type _InputType;
    std::nth_element(std::forward<_ExecutionPolicy>(__exec), __first, __nth, __last, std::less<_InputType>());
}

// [alg.lex.comparison]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
lexicographical_compare(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                        _ForwardIterator2 __first2, _ForwardIterator2 __last2, _Compare __comp)
{
    using namespace __pstl;
    return __internal::__pattern_lexicographical_compare(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2, __comp,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, bool>
lexicographical_compare(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                        _ForwardIterator2 __first2, _ForwardIterator2 __last2)
{
    return std::lexicographical_compare(std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __last2,
                                        __pstl::__internal::__pstl_less());
}

} // namespace std

#endif /* _PSTL_GLUE_ALGORITHM_IMPL_H */
