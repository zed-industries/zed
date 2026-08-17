// -*- C++ -*-
//===-- unseq_backend_simd.h ----------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_UNSEQ_BACKEND_SIMD_H
#define _PSTL_UNSEQ_BACKEND_SIMD_H

#include <type_traits>

#include "utils.h"

// This header defines the minimum set of vector routines required
// to support parallel STL.
namespace __pstl
{
namespace __unseq_backend
{

// Expect vector width up to 64 (or 512 bit)
const std::size_t __lane_size = 64;

template <class _Iterator, class _DifferenceType, class _Function>
_Iterator
__simd_walk_1(_Iterator __first, _DifferenceType __n, _Function __f) noexcept
{
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
        __f(__first[__i]);

    return __first + __n;
}

template <class _Iterator1, class _DifferenceType, class _Iterator2, class _Function>
_Iterator2
__simd_walk_2(_Iterator1 __first1, _DifferenceType __n, _Iterator2 __first2, _Function __f) noexcept
{
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
        __f(__first1[__i], __first2[__i]);
    return __first2 + __n;
}

template <class _Iterator1, class _DifferenceType, class _Iterator2, class _Iterator3, class _Function>
_Iterator3
__simd_walk_3(_Iterator1 __first1, _DifferenceType __n, _Iterator2 __first2, _Iterator3 __first3,
              _Function __f) noexcept
{
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
        __f(__first1[__i], __first2[__i], __first3[__i]);
    return __first3 + __n;
}

// TODO: check whether __simd_first() can be used here
template <class _Index, class _DifferenceType, class _Pred>
bool
__simd_or(_Index __first, _DifferenceType __n, _Pred __pred) noexcept
{
#if _PSTL_EARLYEXIT_PRESENT
    _DifferenceType __i;
    _PSTL_PRAGMA_VECTOR_UNALIGNED
    _PSTL_PRAGMA_SIMD_EARLYEXIT
    for (__i = 0; __i < __n; ++__i)
        if (__pred(__first[__i]))
            break;
    return __i < __n;
#else
    _DifferenceType __block_size = 4 < __n ? 4 : __n;
    const _Index __last = __first + __n;
    while (__last != __first)
    {
        int32_t __flag = 1;
        _PSTL_PRAGMA_SIMD_REDUCTION(& : __flag)
        for (_DifferenceType __i = 0; __i < __block_size; ++__i)
            if (__pred(*(__first + __i)))
                __flag = 0;
        if (!__flag)
            return true;

        __first += __block_size;
        if (__last - __first >= __block_size << 1)
        {
            // Double the block _Size.  Any unnecessary iterations can be amortized against work done so far.
            __block_size <<= 1;
        }
        else
        {
            __block_size = __last - __first;
        }
    }
    return false;
#endif
}

template <class _Index, class _DifferenceType, class _Compare>
_Index
__simd_first(_Index __first, _DifferenceType __begin, _DifferenceType __end, _Compare __comp) noexcept
{
#if _PSTL_EARLYEXIT_PRESENT
    _DifferenceType __i = __begin;
    _PSTL_PRAGMA_VECTOR_UNALIGNED // Do not generate peel loop part
        _PSTL_PRAGMA_SIMD_EARLYEXIT for (; __i < __end; ++__i)
    {
        if (__comp(__first, __i))
        {
            break;
        }
    }
    return __first + __i;
#else
    // Experiments show good block sizes like this
    const _DifferenceType __block_size = 8;
    alignas(__lane_size) _DifferenceType __lane[__block_size] = {0};
    while (__end - __begin >= __block_size)
    {
        _DifferenceType __found = 0;
        _PSTL_PRAGMA_VECTOR_UNALIGNED // Do not generate peel loop part
            _PSTL_PRAGMA_SIMD_REDUCTION(|
                                        : __found) for (_DifferenceType __i = __begin; __i < __begin + __block_size;
                                                        ++__i)
        {
            const _DifferenceType __t = __comp(__first, __i);
            __lane[__i - __begin] = __t;
            __found |= __t;
        }
        if (__found)
        {
            _DifferenceType __i;
            // This will vectorize
            for (__i = 0; __i < __block_size; ++__i)
            {
                if (__lane[__i])
                {
                    break;
                }
            }
            return __first + __begin + __i;
        }
        __begin += __block_size;
    }

    //Keep remainder scalar
    while (__begin != __end)
    {
        if (__comp(__first, __begin))
        {
            return __first + __begin;
        }
        ++__begin;
    }
    return __first + __end;
#endif //_PSTL_EARLYEXIT_PRESENT
}

template <class _Index1, class _DifferenceType, class _Index2, class _Pred>
std::pair<_Index1, _Index2>
__simd_first(_Index1 __first1, _DifferenceType __n, _Index2 __first2, _Pred __pred) noexcept
{
#if _PSTL_EARLYEXIT_PRESENT
    _DifferenceType __i = 0;
    _PSTL_PRAGMA_VECTOR_UNALIGNED
    _PSTL_PRAGMA_SIMD_EARLYEXIT
    for (; __i < __n; ++__i)
        if (__pred(__first1[__i], __first2[__i]))
            break;
    return std::make_pair(__first1 + __i, __first2 + __i);
#else
    const _Index1 __last1 = __first1 + __n;
    const _Index2 __last2 = __first2 + __n;
    // Experiments show good block sizes like this
    const _DifferenceType __block_size = 8;
    alignas(__lane_size) _DifferenceType __lane[__block_size] = {0};
    while (__last1 - __first1 >= __block_size)
    {
        _DifferenceType __found = 0;
        _DifferenceType __i;
        _PSTL_PRAGMA_VECTOR_UNALIGNED // Do not generate peel loop part
            _PSTL_PRAGMA_SIMD_REDUCTION(|
                                         : __found) for (__i = 0; __i < __block_size; ++__i)
        {
            const _DifferenceType __t = __pred(__first1[__i], __first2[__i]);
            __lane[__i] = __t;
            __found |= __t;
        }
        if (__found)
        {
            _DifferenceType __i;
            // This will vectorize
            for (__i = 0; __i < __block_size; ++__i)
            {
                if (__lane[__i])
                    break;
            }
            return std::make_pair(__first1 + __i, __first2 + __i);
        }
        __first1 += __block_size;
        __first2 += __block_size;
    }

    //Keep remainder scalar
    for (; __last1 != __first1; ++__first1, ++__first2)
        if (__pred(*(__first1), *(__first2)))
            return std::make_pair(__first1, __first2);

    return std::make_pair(__last1, __last2);
#endif //_PSTL_EARLYEXIT_PRESENT
}

template <class _Index, class _DifferenceType, class _Pred>
_DifferenceType
__simd_count(_Index __index, _DifferenceType __n, _Pred __pred) noexcept
{
    _DifferenceType __count = 0;
    _PSTL_PRAGMA_SIMD_REDUCTION(+ : __count)
    for (_DifferenceType __i = 0; __i < __n; ++__i)
        if (__pred(*(__index + __i)))
            ++__count;

    return __count;
}

template <class _InputIterator, class _DifferenceType, class _OutputIterator, class _BinaryPredicate>
_OutputIterator
__simd_unique_copy(_InputIterator __first, _DifferenceType __n, _OutputIterator __result,
                   _BinaryPredicate __pred) noexcept
{
    if (__n == 0)
        return __result;

    _DifferenceType __cnt = 1;
    __result[0] = __first[0];

    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 1; __i < __n; ++__i)
    {
        _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC(__cnt : 1)
        if (!__pred(__first[__i], __first[__i - 1]))
        {
            __result[__cnt] = __first[__i];
            ++__cnt;
        }
    }
    return __result + __cnt;
}

template <class _InputIterator, class _DifferenceType, class _OutputIterator, class _Assigner>
_OutputIterator
__simd_assign(_InputIterator __first, _DifferenceType __n, _OutputIterator __result, _Assigner __assigner) noexcept
{
    _PSTL_USE_NONTEMPORAL_STORES_IF_ALLOWED
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
        __assigner(__first + __i, __result + __i);
    return __result + __n;
}

template <class _InputIterator, class _DifferenceType, class _OutputIterator, class _UnaryPredicate>
_OutputIterator
__simd_copy_if(_InputIterator __first, _DifferenceType __n, _OutputIterator __result, _UnaryPredicate __pred) noexcept
{
    _DifferenceType __cnt = 0;

    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
    {
        _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC(__cnt : 1)
        if (__pred(__first[__i]))
        {
            __result[__cnt] = __first[__i];
            ++__cnt;
        }
    }
    return __result + __cnt;
}

template <class _InputIterator, class _DifferenceType, class _BinaryPredicate>
_DifferenceType
__simd_calc_mask_2(_InputIterator __first, _DifferenceType __n, bool* __mask, _BinaryPredicate __pred) noexcept
{
    _DifferenceType __count = 0;

    _PSTL_PRAGMA_SIMD_REDUCTION(+ : __count)
    for (_DifferenceType __i = 0; __i < __n; ++__i)
    {
        __mask[__i] = !__pred(__first[__i], __first[__i - 1]);
        __count += __mask[__i];
    }
    return __count;
}

template <class _InputIterator, class _DifferenceType, class _UnaryPredicate>
_DifferenceType
__simd_calc_mask_1(_InputIterator __first, _DifferenceType __n, bool* __mask, _UnaryPredicate __pred) noexcept
{
    _DifferenceType __count = 0;

    _PSTL_PRAGMA_SIMD_REDUCTION(+ : __count)
    for (_DifferenceType __i = 0; __i < __n; ++__i)
    {
        __mask[__i] = __pred(__first[__i]);
        __count += __mask[__i];
    }
    return __count;
}

template <class _InputIterator, class _DifferenceType, class _OutputIterator, class _Assigner>
void
__simd_copy_by_mask(_InputIterator __first, _DifferenceType __n, _OutputIterator __result, bool* __mask,
                    _Assigner __assigner) noexcept
{
    _DifferenceType __cnt = 0;
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
    {
        if (__mask[__i])
        {
            _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC(__cnt : 1)
            {
                __assigner(__first + __i, __result + __cnt);
                ++__cnt;
            }
        }
    }
}

template <class _InputIterator, class _DifferenceType, class _OutputIterator1, class _OutputIterator2>
void
__simd_partition_by_mask(_InputIterator __first, _DifferenceType __n, _OutputIterator1 __out_true,
                         _OutputIterator2 __out_false, bool* __mask) noexcept
{
    _DifferenceType __cnt_true = 0, __cnt_false = 0;
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
    {
        _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC_2ARGS(__cnt_true : 1, __cnt_false : 1)
        if (__mask[__i])
        {
            __out_true[__cnt_true] = __first[__i];
            ++__cnt_true;
        }
        else
        {
            __out_false[__cnt_false] = __first[__i];
            ++__cnt_false;
        }
    }
}

template <class _Index, class _DifferenceType, class _Tp>
_Index
__simd_fill_n(_Index __first, _DifferenceType __n, const _Tp& __value) noexcept
{
    _PSTL_USE_NONTEMPORAL_STORES_IF_ALLOWED
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
        __first[__i] = __value;
    return __first + __n;
}

template <class _Index, class _DifferenceType, class _Generator>
_Index
__simd_generate_n(_Index __first, _DifferenceType __size, _Generator __g) noexcept
{
    _PSTL_USE_NONTEMPORAL_STORES_IF_ALLOWED
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __size; ++__i)
        __first[__i] = __g();
    return __first + __size;
}

template <class _Index, class _BinaryPredicate>
_Index
__simd_adjacent_find(_Index __first, _Index __last, _BinaryPredicate __pred, bool __or_semantic) noexcept
{
    if (__last - __first < 2)
        return __last;

    typedef typename std::iterator_traits<_Index>::difference_type _DifferenceType;
    _DifferenceType __i = 0;

#if _PSTL_EARLYEXIT_PRESENT
    //Some compiler versions fail to compile the following loop when iterators are used. Indices are used instead
    const _DifferenceType __n = __last - __first - 1;
    _PSTL_PRAGMA_VECTOR_UNALIGNED
    _PSTL_PRAGMA_SIMD_EARLYEXIT
    for (; __i < __n; ++__i)
        if (__pred(__first[__i], __first[__i + 1]))
            break;

    return __i < __n ? __first + __i : __last;
#else
    // Experiments show good block sizes like this
    //TODO: to consider tuning block_size for various data types
    const _DifferenceType __block_size = 8;
    alignas(__lane_size) _DifferenceType __lane[__block_size] = {0};
    while (__last - __first >= __block_size)
    {
        _DifferenceType __found = 0;
        _PSTL_PRAGMA_VECTOR_UNALIGNED // Do not generate peel loop part
            _PSTL_PRAGMA_SIMD_REDUCTION(|
                                         : __found) for (__i = 0; __i < __block_size - 1; ++__i)
        {
            //TODO: to improve SIMD vectorization
            const _DifferenceType __t = __pred(*(__first + __i), *(__first + __i + 1));
            __lane[__i] = __t;
            __found |= __t;
        }

        //Process a pair of elements on a boundary of a data block
        if (__first + __block_size < __last && __pred(*(__first + __i), *(__first + __i + 1)))
            __lane[__i] = __found = 1;

        if (__found)
        {
            if (__or_semantic)
                return __first;

            // This will vectorize
            for (__i = 0; __i < __block_size; ++__i)
                if (__lane[__i])
                    break;
            return __first + __i; //As far as found is true a __result (__lane[__i] is true) is guaranteed
        }
        __first += __block_size;
    }
    //Process the rest elements
    for (; __last - __first > 1; ++__first)
        if (__pred(*__first, *(__first + 1)))
            return __first;

    return __last;
#endif
}

// It was created to reduce the code inside std::enable_if
template <typename _Tp, typename _BinaryOperation>
using is_arithmetic_plus = std::integral_constant<bool, std::is_arithmetic<_Tp>::value &&
                                                            std::is_same<_BinaryOperation, std::plus<_Tp>>::value>;

template <typename _DifferenceType, typename _Tp, typename _BinaryOperation, typename _UnaryOperation>
typename std::enable_if<is_arithmetic_plus<_Tp, _BinaryOperation>::value, _Tp>::type
__simd_transform_reduce(_DifferenceType __n, _Tp __init, _BinaryOperation, _UnaryOperation __f) noexcept
{
    _PSTL_PRAGMA_SIMD_REDUCTION(+ : __init)
    for (_DifferenceType __i = 0; __i < __n; ++__i)
        __init += __f(__i);
    return __init;
}

template <typename _Size, typename _Tp, typename _BinaryOperation, typename _UnaryOperation>
typename std::enable_if<!is_arithmetic_plus<_Tp, _BinaryOperation>::value, _Tp>::type
__simd_transform_reduce(_Size __n, _Tp __init, _BinaryOperation __binary_op, _UnaryOperation __f) noexcept
{
    const _Size __block_size = __lane_size / sizeof(_Tp);
    if (__n > 2 * __block_size && __block_size > 1)
    {
        alignas(__lane_size) char __lane_[__lane_size];
        _Tp* __lane = reinterpret_cast<_Tp*>(__lane_);

        // initializer
        _PSTL_PRAGMA_SIMD
        for (_Size __i = 0; __i < __block_size; ++__i)
        {
            ::new (__lane + __i) _Tp(__binary_op(__f(__i), __f(__block_size + __i)));
        }
        // main loop
        _Size __i = 2 * __block_size;
        const _Size last_iteration = __block_size * (__n / __block_size);
        for (; __i < last_iteration; __i += __block_size)
        {
            _PSTL_PRAGMA_SIMD
            for (_Size __j = 0; __j < __block_size; ++__j)
            {
                __lane[__j] = __binary_op(__lane[__j], __f(__i + __j));
            }
        }
        // remainder
        _PSTL_PRAGMA_SIMD
        for (_Size __j = 0; __j < __n - last_iteration; ++__j)
        {
            __lane[__j] = __binary_op(__lane[__j], __f(last_iteration + __j));
        }
        // combiner
        for (_Size __i = 0; __i < __block_size; ++__i)
        {
            __init = __binary_op(__init, __lane[__i]);
        }
        // destroyer
        _PSTL_PRAGMA_SIMD
        for (_Size __i = 0; __i < __block_size; ++__i)
        {
            __lane[__i].~_Tp();
        }
    }
    else
    {
        for (_Size __i = 0; __i < __n; ++__i)
        {
            __init = __binary_op(__init, __f(__i));
        }
    }
    return __init;
}

// Exclusive scan for "+" and arithmetic types
template <class _InputIterator, class _Size, class _OutputIterator, class _UnaryOperation, class _Tp,
          class _BinaryOperation>
typename std::enable_if<is_arithmetic_plus<_Tp, _BinaryOperation>::value, std::pair<_OutputIterator, _Tp>>::type
__simd_scan(_InputIterator __first, _Size __n, _OutputIterator __result, _UnaryOperation __unary_op, _Tp __init,
            _BinaryOperation, /*Inclusive*/ std::false_type)
{
    _PSTL_PRAGMA_SIMD_SCAN(+ : __init)
    for (_Size __i = 0; __i < __n; ++__i)
    {
        __result[__i] = __init;
        _PSTL_PRAGMA_SIMD_EXCLUSIVE_SCAN(__init)
        __init += __unary_op(__first[__i]);
    }
    return std::make_pair(__result + __n, __init);
}

// As soon as we cannot call __binary_op in "combiner" we create a wrapper over _Tp to encapsulate __binary_op
template <typename _Tp, typename _BinaryOp>
struct _Combiner
{
    _Tp __value;
    _BinaryOp* __bin_op; // Here is a pointer to function because of default ctor

    _Combiner() : __value{}, __bin_op(nullptr) {}
    _Combiner(const _Tp& value, const _BinaryOp* bin_op) : __value(value), __bin_op(const_cast<_BinaryOp*>(bin_op)) {}
    _Combiner(const _Combiner& __obj) : __value{}, __bin_op(__obj.__bin_op) {}

    void
    operator()(const _Combiner& __obj)
    {
        __value = (*__bin_op)(__value, __obj.__value);
    }
};

// Exclusive scan for other binary operations and types
template <class _InputIterator, class _Size, class _OutputIterator, class _UnaryOperation, class _Tp,
          class _BinaryOperation>
typename std::enable_if<!is_arithmetic_plus<_Tp, _BinaryOperation>::value, std::pair<_OutputIterator, _Tp>>::type
__simd_scan(_InputIterator __first, _Size __n, _OutputIterator __result, _UnaryOperation __unary_op, _Tp __init,
            _BinaryOperation __binary_op, /*Inclusive*/ std::false_type)
{
    typedef _Combiner<_Tp, _BinaryOperation> _CombinerType;
    _CombinerType __init_{__init, &__binary_op};

    _PSTL_PRAGMA_DECLARE_REDUCTION(__bin_op, _CombinerType)

    _PSTL_PRAGMA_SIMD_SCAN(__bin_op : __init_)
    for (_Size __i = 0; __i < __n; ++__i)
    {
        __result[__i] = __init_.__value;
        _PSTL_PRAGMA_SIMD_EXCLUSIVE_SCAN(__init_)
        _PSTL_PRAGMA_FORCEINLINE
        __init_.__value = __binary_op(__init_.__value, __unary_op(__first[__i]));
    }
    return std::make_pair(__result + __n, __init_.__value);
}

// Inclusive scan for "+" and arithmetic types
template <class _InputIterator, class _Size, class _OutputIterator, class _UnaryOperation, class _Tp,
          class _BinaryOperation>
typename std::enable_if<is_arithmetic_plus<_Tp, _BinaryOperation>::value, std::pair<_OutputIterator, _Tp>>::type
__simd_scan(_InputIterator __first, _Size __n, _OutputIterator __result, _UnaryOperation __unary_op, _Tp __init,
            _BinaryOperation, /*Inclusive*/ std::true_type)
{
    _PSTL_PRAGMA_SIMD_SCAN(+ : __init)
    for (_Size __i = 0; __i < __n; ++__i)
    {
        __init += __unary_op(__first[__i]);
        _PSTL_PRAGMA_SIMD_INCLUSIVE_SCAN(__init)
        __result[__i] = __init;
    }
    return std::make_pair(__result + __n, __init);
}

// Inclusive scan for other binary operations and types
template <class _InputIterator, class _Size, class _OutputIterator, class _UnaryOperation, class _Tp,
          class _BinaryOperation>
typename std::enable_if<!is_arithmetic_plus<_Tp, _BinaryOperation>::value, std::pair<_OutputIterator, _Tp>>::type
__simd_scan(_InputIterator __first, _Size __n, _OutputIterator __result, _UnaryOperation __unary_op, _Tp __init,
            _BinaryOperation __binary_op, std::true_type)
{
    typedef _Combiner<_Tp, _BinaryOperation> _CombinerType;
    _CombinerType __init_{__init, &__binary_op};

    _PSTL_PRAGMA_DECLARE_REDUCTION(__bin_op, _CombinerType)

    _PSTL_PRAGMA_SIMD_SCAN(__bin_op : __init_)
    for (_Size __i = 0; __i < __n; ++__i)
    {
        _PSTL_PRAGMA_FORCEINLINE
        __init_.__value = __binary_op(__init_.__value, __unary_op(__first[__i]));
        _PSTL_PRAGMA_SIMD_INCLUSIVE_SCAN(__init_)
        __result[__i] = __init_.__value;
    }
    return std::make_pair(__result + __n, __init_.__value);
}

// [restriction] - std::iterator_traits<_ForwardIterator>::value_type should be DefaultConstructible.
// complexity [violation] - We will have at most (__n-1 + number_of_lanes) comparisons instead of at most __n-1.
template <typename _ForwardIterator, typename _Size, typename _Compare>
_ForwardIterator
__simd_min_element(_ForwardIterator __first, _Size __n, _Compare __comp) noexcept
{
    if (__n == 0)
    {
        return __first;
    }

    typedef typename std::iterator_traits<_ForwardIterator>::value_type _ValueType;
    struct _ComplexType
    {
        _ValueType __min_val;
        _Size __min_ind;
        _Compare* __min_comp;

        _ComplexType() : __min_val{}, __min_ind{}, __min_comp(nullptr) {}
        _ComplexType(const _ValueType& val, const _Compare* comp)
            : __min_val(val), __min_ind(0), __min_comp(const_cast<_Compare*>(comp))
        {
        }
        _ComplexType(const _ComplexType& __obj)
            : __min_val(__obj.__min_val), __min_ind(__obj.__min_ind), __min_comp(__obj.__min_comp)
        {
        }

        _PSTL_PRAGMA_DECLARE_SIMD
        void
        operator()(const _ComplexType& __obj)
        {
            if (!(*__min_comp)(__min_val, __obj.__min_val) &&
                ((*__min_comp)(__obj.__min_val, __min_val) || __obj.__min_ind - __min_ind < 0))
            {
                __min_val = __obj.__min_val;
                __min_ind = __obj.__min_ind;
            }
        }
    };

    _ComplexType __init{*__first, &__comp};

    _PSTL_PRAGMA_DECLARE_REDUCTION(__min_func, _ComplexType)

    _PSTL_PRAGMA_SIMD_REDUCTION(__min_func : __init)
    for (_Size __i = 1; __i < __n; ++__i)
    {
        const _ValueType __min_val = __init.__min_val;
        const _ValueType __current = __first[__i];
        if (__comp(__current, __min_val))
        {
            __init.__min_val = __current;
            __init.__min_ind = __i;
        }
    }
    return __first + __init.__min_ind;
}

// [restriction] - std::iterator_traits<_ForwardIterator>::value_type should be DefaultConstructible.
// complexity [violation] - We will have at most (2*(__n-1) + 4*number_of_lanes) comparisons instead of at most [1.5*(__n-1)].
template <typename _ForwardIterator, typename _Size, typename _Compare>
std::pair<_ForwardIterator, _ForwardIterator>
__simd_minmax_element(_ForwardIterator __first, _Size __n, _Compare __comp) noexcept
{
    if (__n == 0)
    {
        return std::make_pair(__first, __first);
    }
    typedef typename std::iterator_traits<_ForwardIterator>::value_type _ValueType;

    struct _ComplexType
    {
        _ValueType __min_val;
        _ValueType __max_val;
        _Size __min_ind;
        _Size __max_ind;
        _Compare* __minmax_comp;

        _ComplexType() : __min_val{}, __max_val{}, __min_ind{}, __max_ind{}, __minmax_comp(nullptr) {}
        _ComplexType(const _ValueType& min_val, const _ValueType& max_val, const _Compare* comp)
            : __min_val(min_val), __max_val(max_val), __min_ind(0), __max_ind(0),
              __minmax_comp(const_cast<_Compare*>(comp))
        {
        }
        _ComplexType(const _ComplexType& __obj)
            : __min_val(__obj.__min_val), __max_val(__obj.__max_val), __min_ind(__obj.__min_ind),
              __max_ind(__obj.__max_ind), __minmax_comp(__obj.__minmax_comp)
        {
        }

        void
        operator()(const _ComplexType& __obj)
        {
            // min
            if ((*__minmax_comp)(__obj.__min_val, __min_val))
            {
                __min_val = __obj.__min_val;
                __min_ind = __obj.__min_ind;
            }
            else if (!(*__minmax_comp)(__min_val, __obj.__min_val))
            {
                __min_val = __obj.__min_val;
                __min_ind = (__min_ind - __obj.__min_ind < 0) ? __min_ind : __obj.__min_ind;
            }

            // max
            if ((*__minmax_comp)(__max_val, __obj.__max_val))
            {
                __max_val = __obj.__max_val;
                __max_ind = __obj.__max_ind;
            }
            else if (!(*__minmax_comp)(__obj.__max_val, __max_val))
            {
                __max_val = __obj.__max_val;
                __max_ind = (__max_ind - __obj.__max_ind < 0) ? __obj.__max_ind : __max_ind;
            }
        }
    };

    _ComplexType __init{*__first, *__first, &__comp};

    _PSTL_PRAGMA_DECLARE_REDUCTION(__min_func, _ComplexType);

    _PSTL_PRAGMA_SIMD_REDUCTION(__min_func : __init)
    for (_Size __i = 1; __i < __n; ++__i)
    {
        auto __min_val = __init.__min_val;
        auto __max_val = __init.__max_val;
        auto __current = __first + __i;
        if (__comp(*__current, __min_val))
        {
            __init.__min_val = *__current;
            __init.__min_ind = __i;
        }
        else if (!__comp(*__current, __max_val))
        {
            __init.__max_val = *__current;
            __init.__max_ind = __i;
        }
    }
    return std::make_pair(__first + __init.__min_ind, __first + __init.__max_ind);
}

template <class _InputIterator, class _DifferenceType, class _OutputIterator1, class _OutputIterator2,
          class _UnaryPredicate>
std::pair<_OutputIterator1, _OutputIterator2>
__simd_partition_copy(_InputIterator __first, _DifferenceType __n, _OutputIterator1 __out_true,
                      _OutputIterator2 __out_false, _UnaryPredicate __pred) noexcept
{
    _DifferenceType __cnt_true = 0, __cnt_false = 0;

    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 0; __i < __n; ++__i)
    {
        _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC_2ARGS(__cnt_true : 1, __cnt_false : 1)
        if (__pred(__first[__i]))
        {
            __out_true[__cnt_true] = __first[__i];
            ++__cnt_true;
        }
        else
        {
            __out_false[__cnt_false] = __first[__i];
            ++__cnt_false;
        }
    }
    return std::make_pair(__out_true + __cnt_true, __out_false + __cnt_false);
}

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1
__simd_find_first_of(_ForwardIterator1 __first, _ForwardIterator1 __last, _ForwardIterator2 __s_first,
                     _ForwardIterator2 __s_last, _BinaryPredicate __pred) noexcept
{
    typedef typename std::iterator_traits<_ForwardIterator1>::difference_type _DifferencType;

    const _DifferencType __n1 = __last - __first;
    const _DifferencType __n2 = __s_last - __s_first;
    if (__n1 == 0 || __n2 == 0)
    {
        return __last; // according to the standard
    }

    // Common case
    // If first sequence larger than second then we'll run simd_first with parameters of first sequence.
    // Otherwise, vice versa.
    if (__n1 < __n2)
    {
        for (; __first != __last; ++__first)
        {
  	    if (__unseq_backend::__simd_or(__s_first, __n2,
                          __internal::__equal_value_by_pred<decltype(*__first), _BinaryPredicate>(*__first, __pred)))
            {
                return __first;
            }
        }
    }
    else
    {
        for (; __s_first != __s_last; ++__s_first)
        {
  	    const auto __result = __unseq_backend::__simd_first(__first, _DifferencType(0), __n1,
                                               [__s_first, &__pred](_ForwardIterator1 __it, _DifferencType __i) {
                                                   return __pred(__it[__i], *__s_first);
                                               });
            if (__result != __last)
            {
                return __result;
            }
        }
    }
    return __last;
}

template <class _RandomAccessIterator, class _DifferenceType, class _UnaryPredicate>
_RandomAccessIterator
__simd_remove_if(_RandomAccessIterator __first, _DifferenceType __n, _UnaryPredicate __pred) noexcept
{
    // find first element we need to remove
    auto __current =
        __unseq_backend::__simd_first(__first, _DifferenceType(0), __n,
                     [&__pred](_RandomAccessIterator __it, _DifferenceType __i) { return __pred(__it[__i]); });
    __n -= __current - __first;

    // if we have in sequence only one element that pred(__current[1]) != false we can exit the function
    if (__n < 2)
    {
        return __current;
    }

    _DifferenceType __cnt = 0;
    _PSTL_PRAGMA_SIMD
    for (_DifferenceType __i = 1; __i < __n; ++__i)
    {
        _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC(__cnt : 1)
        if (!__pred(__current[__i]))
        {
            __current[__cnt] = std::move(__current[__i]);
            ++__cnt;
        }
    }
    return __current + __cnt;
}
} // namespace __unseq_backend
} // namespace __pstl

#endif /* _PSTL_UNSEQ_BACKEND_SIMD_H */
