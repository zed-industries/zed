// -*- C++ -*-
//===-- numeric_fwd.h --------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_NUMERIC_FWD_H
#define _PSTL_NUMERIC_FWD_H

#include <type_traits>
#include <utility>

namespace __pstl
{
namespace __internal
{

//------------------------------------------------------------------------
// transform_reduce (version with two binary functions, according to draft N4659)
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _Tp, class _BinaryOperation1, class _BinaryOperation2>
_Tp __brick_transform_reduce(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Tp, _BinaryOperation1,
                             _BinaryOperation2,
                             /*__is_vector=*/std::true_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _Tp, class _BinaryOperation1, class _BinaryOperation2>
_Tp __brick_transform_reduce(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Tp, _BinaryOperation1,
                             _BinaryOperation2,
                             /*__is_vector=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp, class _BinaryOperation1,
          class _BinaryOperation2, class _IsVector>
_Tp
__pattern_transform_reduce(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Tp,
                           _BinaryOperation1, _BinaryOperation2, _IsVector,
                           /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _Tp,
          class _BinaryOperation1, class _BinaryOperation2, class _IsVector>
_Tp
__pattern_transform_reduce(_ExecutionPolicy&&, _RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                           _Tp, _BinaryOperation1, _BinaryOperation2, _IsVector __is_vector,
                           /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// transform_reduce (version with unary and binary functions)
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Tp, class _UnaryOperation, class _BinaryOperation>
_Tp __brick_transform_reduce(_ForwardIterator, _ForwardIterator, _Tp, _BinaryOperation, _UnaryOperation,
                             /*is_vector=*/std::true_type) noexcept;

template <class _ForwardIterator, class _Tp, class _BinaryOperation, class _UnaryOperation>
_Tp __brick_transform_reduce(_ForwardIterator, _ForwardIterator, _Tp, _BinaryOperation, _UnaryOperation,
                             /*is_vector=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _BinaryOperation, class _UnaryOperation,
          class _IsVector>
_Tp
__pattern_transform_reduce(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Tp, _BinaryOperation,
                           _UnaryOperation, _IsVector,
                           /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _BinaryOperation, class _UnaryOperation,
          class _IsVector>
_Tp
__pattern_transform_reduce(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Tp, _BinaryOperation,
                           _UnaryOperation, _IsVector,
                           /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// transform_exclusive_scan
//
// walk3 evaluates f(x,y,z) for (x,y,z) drawn from [first1,last1), [first2,...), [first3,...)
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator, class _UnaryOperation, class _Tp, class _BinaryOperation>
std::pair<_OutputIterator, _Tp> __brick_transform_scan(_ForwardIterator, _ForwardIterator, _OutputIterator,
                                                       _UnaryOperation, _Tp, _BinaryOperation,
                                                       /*Inclusive*/ std::false_type) noexcept;

template <class _ForwardIterator, class _OutputIterator, class _UnaryOperation, class _Tp, class _BinaryOperation>
std::pair<_OutputIterator, _Tp> __brick_transform_scan(_ForwardIterator, _ForwardIterator, _OutputIterator,
                                                       _UnaryOperation, _Tp, _BinaryOperation,
                                                       /*Inclusive*/ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _UnaryOperation, class _Tp,
          class _BinaryOperation, class _Inclusive, class _IsVector>
_OutputIterator
__pattern_transform_scan(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _OutputIterator, _UnaryOperation, _Tp,
                         _BinaryOperation, _Inclusive, _IsVector,
                         /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator, class _UnaryOperation, class _Tp,
          class _BinaryOperation, class _Inclusive, class _IsVector>
typename std::enable_if<!std::is_floating_point<_Tp>::value, _OutputIterator>::type
__pattern_transform_scan(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _OutputIterator,
                         _UnaryOperation, _Tp, _BinaryOperation, _Inclusive, _IsVector, /*is_parallel=*/std::true_type);

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator, class _UnaryOperation, class _Tp,
          class _BinaryOperation, class _Inclusive, class _IsVector>
typename std::enable_if<std::is_floating_point<_Tp>::value, _OutputIterator>::type
__pattern_transform_scan(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _OutputIterator,
                         _UnaryOperation, _Tp, _BinaryOperation, _Inclusive, _IsVector, /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// adjacent_difference
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator, class _BinaryOperation>
_OutputIterator __brick_adjacent_difference(_ForwardIterator, _ForwardIterator, _OutputIterator, _BinaryOperation,
                                            /*is_vector*/ std::false_type) noexcept;

template <class _ForwardIterator, class _OutputIterator, class _BinaryOperation>
_OutputIterator __brick_adjacent_difference(_ForwardIterator, _ForwardIterator, _OutputIterator, _BinaryOperation,
                                            /*is_vector*/ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _BinaryOperation,
          class _IsVector>
_OutputIterator
__pattern_adjacent_difference(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _OutputIterator, _BinaryOperation,
                              _IsVector, /*is_parallel*/ std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _BinaryOperation,
          class _IsVector>
_OutputIterator
__pattern_adjacent_difference(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _OutputIterator, _BinaryOperation,
                              _IsVector, /*is_parallel*/ std::true_type);

} // namespace __internal
} // namespace __pstl
#endif /* _PSTL_NUMERIC_FWD_H */
