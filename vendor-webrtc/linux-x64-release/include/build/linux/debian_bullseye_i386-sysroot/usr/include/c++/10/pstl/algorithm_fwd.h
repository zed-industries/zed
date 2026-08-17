// -*- C++ -*-
//===-- algorithm_fwd.h --------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_ALGORITHM_FWD_H
#define _PSTL_ALGORITHM_FWD_H

#include <type_traits>
#include <utility>

namespace __pstl
{
namespace __internal
{

//------------------------------------------------------------------------
// any_of
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Pred>
bool
__brick_any_of(const _ForwardIterator, const _ForwardIterator, _Pred,
               /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _Pred>
bool
__brick_any_of(const _ForwardIterator, const _ForwardIterator, _Pred,
               /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Pred, class _IsVector>
bool
__pattern_any_of(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Pred, _IsVector,
                 /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Pred, class _IsVector>
bool
__pattern_any_of(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Pred, _IsVector,
                 /*parallel=*/std::true_type);

//------------------------------------------------------------------------
// walk1 (pseudo)
//
// walk1 evaluates f(x) for each dereferenced value x drawn from [first,last)
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Function>
void __brick_walk1(_ForwardIterator, _ForwardIterator, _Function,
                   /*vector=*/std::false_type) noexcept;

template <class _RandomAccessIterator, class _Function>
void __brick_walk1(_RandomAccessIterator, _RandomAccessIterator, _Function,
                   /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Function, class _IsVector>
void
__pattern_walk1(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Function, _IsVector,
                /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Function, class _IsVector>
void
__pattern_walk1(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Function, _IsVector,
                /*parallel=*/std::true_type);

template <class _ExecutionPolicy, class _ForwardIterator, class _Brick>
void
__pattern_walk_brick(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Brick,
                     /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Brick>
void
__pattern_walk_brick(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Brick,
                     /*parallel=*/std::true_type);

//------------------------------------------------------------------------
// walk1_n
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Size, class _Function>
_ForwardIterator __brick_walk1_n(_ForwardIterator, _Size, _Function,
                                 /*_IsVectorTag=*/std::false_type);

template <class _RandomAccessIterator, class _DifferenceType, class _Function>
_RandomAccessIterator __brick_walk1_n(_RandomAccessIterator, _DifferenceType, _Function,
                                      /*vectorTag=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Function, class _IsVector>
_ForwardIterator
__pattern_walk1_n(_ExecutionPolicy&&, _ForwardIterator, _Size, _Function, _IsVector,
                  /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Size, class _Function, class _IsVector>
_RandomAccessIterator
__pattern_walk1_n(_ExecutionPolicy&&, _RandomAccessIterator, _Size, _Function, _IsVector,
                  /*is_parallel=*/std::true_type);

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Brick>
_ForwardIterator
__pattern_walk_brick_n(_ExecutionPolicy&&, _ForwardIterator, _Size, _Brick,
                       /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Size, class _Brick>
_RandomAccessIterator
__pattern_walk_brick_n(_ExecutionPolicy&&, _RandomAccessIterator, _Size, _Brick,
                       /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// walk2 (pseudo)
//
// walk2 evaluates f(x,y) for deferenced values (x,y) drawn from [first1,last1) and [first2,...)
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _Function>
_ForwardIterator2 __brick_walk2(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Function,
                                /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _Function>
_ForwardIterator2 __brick_walk2(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Function,
                                /*vector=*/std::true_type) noexcept;

template <class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Function>
_ForwardIterator2 __brick_walk2_n(_ForwardIterator1, _Size, _ForwardIterator2, _Function,
                                  /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Function>
_ForwardIterator2 __brick_walk2_n(_ForwardIterator1, _Size, _ForwardIterator2, _Function,
                                  /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Function, class _IsVector>
_ForwardIterator2
__pattern_walk2(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Function, _IsVector,
                /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Function, class _IsVector>
_ForwardIterator2
__pattern_walk2(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Function, _IsVector,
                /*parallel=*/std::true_type);

template <class _ExecutionPolicy, class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Function,
          class _IsVector>
_ForwardIterator2
__pattern_walk2_n(_ExecutionPolicy&&, _ForwardIterator1, _Size, _ForwardIterator2, _Function, _IsVector,
                  /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _Size, class _RandomAccessIterator2,
          class _Function, class _IsVector>
_RandomAccessIterator2
__pattern_walk2_n(_ExecutionPolicy&&, _RandomAccessIterator1, _Size, _RandomAccessIterator2, _Function, _IsVector,
                  /*parallel=*/std::true_type);

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Brick>
_ForwardIterator2
__pattern_walk2_brick(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _Brick,
                      /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _Brick>
_RandomAccessIterator2
__pattern_walk2_brick(_ExecutionPolicy&&, _RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                      _Brick,
                      /*parallel=*/std::true_type);

template <class _ExecutionPolicy, class _ForwardIterator1, class _Size, class _ForwardIterator2, class _Brick>
_ForwardIterator2
__pattern_walk2_brick_n(_ExecutionPolicy&&, _ForwardIterator1, _Size, _ForwardIterator2, _Brick,
                        /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _Size, class _RandomAccessIterator2, class _Brick>
_RandomAccessIterator2
__pattern_walk2_brick_n(_ExecutionPolicy&&, _RandomAccessIterator1, _Size, _RandomAccessIterator2, _Brick,
                        /*parallel=*/std::true_type);

//------------------------------------------------------------------------
// walk3 (pseudo)
//
// walk3 evaluates f(x,y,z) for (x,y,z) drawn from [first1,last1), [first2,...), [first3,...)
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator3, class _Function>
_ForwardIterator3 __brick_walk3(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator3, _Function,
                                /*vector=*/std::false_type) noexcept;

template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _RandomAccessIterator3, class _Function>
_RandomAccessIterator3 __brick_walk3(_RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                                     _RandomAccessIterator3, _Function,
                                     /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _ForwardIterator3,
          class _Function, class _IsVector>
_ForwardIterator3
__pattern_walk3(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator3,
                _Function, _IsVector,
                /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2,
          class _RandomAccessIterator3, class _Function, class _IsVector>
_RandomAccessIterator3
__pattern_walk3(_ExecutionPolicy&&, _RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                _RandomAccessIterator3, _Function, _IsVector, /*parallel=*/std::true_type);

//------------------------------------------------------------------------
// equal
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
bool __brick_equal(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _BinaryPredicate,
                   /* is_vector = */ std::false_type) noexcept;

template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate>
bool __brick_equal(_RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2, _BinaryPredicate,
                   /* is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _BinaryPredicate,
                _IsVector, /* is_parallel = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&&, _RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                _BinaryPredicate, _IsVector, /* is_parallel = */ std::true_type);

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
bool __brick_equal(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2, _BinaryPredicate,
                   /* is_vector = */ std::false_type) noexcept;

template <class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate>
bool __brick_equal(_RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2, _RandomAccessIterator2,
                   _BinaryPredicate, /* is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                _BinaryPredicate, _IsVector, /* is_parallel = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _BinaryPredicate,
          class _IsVector>
bool
__pattern_equal(_ExecutionPolicy&&, _RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                _RandomAccessIterator2, _BinaryPredicate, _IsVector, /* is_parallel = */ std::true_type);

//------------------------------------------------------------------------
// find_if
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Predicate>
_ForwardIterator __brick_find_if(_ForwardIterator, _ForwardIterator, _Predicate,
                                 /*is_vector=*/std::false_type) noexcept;

template <class _RandomAccessIterator, class _Predicate>
_RandomAccessIterator __brick_find_if(_RandomAccessIterator, _RandomAccessIterator, _Predicate,
                                      /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
_ForwardIterator
__pattern_find_if(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Predicate, _IsVector,
                  /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
_ForwardIterator
__pattern_find_if(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Predicate, _IsVector,
                  /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// find_end
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1 __brick_find_end(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                   _BinaryPredicate,
                                   /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1 __brick_find_end(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                   _BinaryPredicate,
                                   /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_end(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                   _BinaryPredicate, _IsVector,
                   /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_end(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                   _BinaryPredicate, _IsVector,
                   /*is_parallel=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// find_first_of
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1 __brick_find_first_of(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                        _BinaryPredicate,
                                        /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1 __brick_find_first_of(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                        _BinaryPredicate,
                                        /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_first_of(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                        _BinaryPredicate, _IsVector, /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_find_first_of(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                        _BinaryPredicate, _IsVector, /*is_parallel=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// search
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1 __brick_search(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                 _BinaryPredicate,
                                 /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate>
_ForwardIterator1 __brick_search(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                 _BinaryPredicate,
                                 /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_search(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                 _BinaryPredicate, _IsVector,
                 /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryPredicate,
          class _IsVector>
_ForwardIterator1
__pattern_search(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                 _BinaryPredicate, _IsVector,
                 /*is_parallel=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// search_n
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Size, class _Tp, class _BinaryPredicate>
_ForwardIterator
__brick_search_n(_ForwardIterator, _ForwardIterator, _Size, const _Tp&, _BinaryPredicate,
                 /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _Size, class _Tp, class _BinaryPredicate>
_ForwardIterator
__brick_search_n(_ForwardIterator, _ForwardIterator, _Size, const _Tp&, _BinaryPredicate,
                 /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Size, class _Tp, class _BinaryPredicate,
          class IsVector>
_ForwardIterator
__pattern_search_n(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Size, const _Tp&, _BinaryPredicate,
                   IsVector,
                   /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Size, class _Tp, class _BinaryPredicate,
          class IsVector>
_RandomAccessIterator
__pattern_search_n(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Size, const _Tp&,
                   _BinaryPredicate, IsVector,
                   /*is_parallel=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// copy_n
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Size, class _OutputIterator>
_OutputIterator __brick_copy_n(_ForwardIterator, _Size, _OutputIterator,
                               /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _Size, class _OutputIterator>
_OutputIterator __brick_copy_n(_ForwardIterator, _Size, _OutputIterator,
                               /*vector=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// copy
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator __brick_copy(_ForwardIterator, _ForwardIterator, _OutputIterator,
                             /*vector=*/std::false_type) noexcept;

template <class _RandomAccessIterator, class _OutputIterator>
_OutputIterator __brick_copy(_RandomAccessIterator, _RandomAccessIterator, _OutputIterator,
                             /*vector=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// move
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator __brick_move(_ForwardIterator, _ForwardIterator, _OutputIterator,
                             /*vector=*/std::false_type) noexcept;

template <class _RandomAccessIterator, class _OutputIterator>
_OutputIterator __brick_move(_RandomAccessIterator, _RandomAccessIterator, _OutputIterator,
                             /*vector=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// swap_ranges
//------------------------------------------------------------------------
template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_swap_ranges(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                    /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_swap_ranges(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                    /*vector=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// copy_if
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator, class _UnaryPredicate>
_OutputIterator __brick_copy_if(_ForwardIterator, _ForwardIterator, _OutputIterator, _UnaryPredicate,
                                /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _OutputIterator, class _UnaryPredicate>
_OutputIterator __brick_copy_if(_ForwardIterator, _ForwardIterator, _OutputIterator, _UnaryPredicate,
                                /*vector=*/std::true_type) noexcept;

template <class _DifferenceType, class _ForwardIterator, class _UnaryPredicate>
std::pair<_DifferenceType, _DifferenceType>
__brick_calc_mask_1(_ForwardIterator, _ForwardIterator, bool* __restrict, _UnaryPredicate,
                    /*vector=*/std::false_type) noexcept;
template <class _DifferenceType, class _RandomAccessIterator, class _UnaryPredicate>
std::pair<_DifferenceType, _DifferenceType>
__brick_calc_mask_1(_RandomAccessIterator, _RandomAccessIterator, bool* __restrict, _UnaryPredicate,
                    /*vector=*/std::true_type) noexcept;

template <class _ForwardIterator, class _OutputIterator>
void
__brick_copy_by_mask(_ForwardIterator, _ForwardIterator, _OutputIterator, bool*,
                     /*vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _OutputIterator>
void
__brick_copy_by_mask(_ForwardIterator, _ForwardIterator, _OutputIterator, bool* __restrict,
                     /*vector=*/std::true_type) noexcept;

template <class _ForwardIterator, class _OutputIterator1, class _OutputIterator2>
void
__brick_partition_by_mask(_ForwardIterator, _ForwardIterator, _OutputIterator1, _OutputIterator2, bool*,
                          /*vector=*/std::false_type) noexcept;

template <class _RandomAccessIterator, class _OutputIterator1, class _OutputIterator2>
void
__brick_partition_by_mask(_RandomAccessIterator, _RandomAccessIterator, _OutputIterator1, _OutputIterator2, bool*,
                          /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _UnaryPredicate, class _IsVector>
_OutputIterator
__pattern_copy_if(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _OutputIterator, _UnaryPredicate, _IsVector,
                  /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator, class _UnaryPredicate,
          class _IsVector>
_OutputIterator
__pattern_copy_if(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _OutputIterator, _UnaryPredicate,
                  _IsVector, /*parallel=*/std::true_type);

//------------------------------------------------------------------------
// count
//------------------------------------------------------------------------

template <class _ForwardIterator, class _Predicate>
typename std::iterator_traits<_ForwardIterator>::difference_type
    __brick_count(_ForwardIterator, _ForwardIterator, _Predicate,
                  /* is_vector = */ std::true_type) noexcept;

template <class _ForwardIterator, class _Predicate>
typename std::iterator_traits<_ForwardIterator>::difference_type
    __brick_count(_ForwardIterator, _ForwardIterator, _Predicate,
                  /* is_vector = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
typename std::iterator_traits<_ForwardIterator>::difference_type
__pattern_count(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Predicate,
                /* is_parallel */ std::false_type, _IsVector) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Predicate, class _IsVector>
typename std::iterator_traits<_ForwardIterator>::difference_type
__pattern_count(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Predicate,
                /* is_parallel */ std::true_type, _IsVector);

//------------------------------------------------------------------------
// unique
//------------------------------------------------------------------------

template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator __brick_unique(_ForwardIterator, _ForwardIterator, _BinaryPredicate,
                                /*is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator __brick_unique(_ForwardIterator, _ForwardIterator, _BinaryPredicate,
                                /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate, class _IsVector>
_ForwardIterator
__pattern_unique(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _BinaryPredicate, _IsVector,
                 /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate, class _IsVector>
_ForwardIterator
__pattern_unique(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _BinaryPredicate, _IsVector,
                 /*is_parallel=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// unique_copy
//------------------------------------------------------------------------

template <class _ForwardIterator, class OutputIterator, class _BinaryPredicate>
OutputIterator __brick_unique_copy(_ForwardIterator, _ForwardIterator, OutputIterator, _BinaryPredicate,
                                   /*vector=*/std::false_type) noexcept;

template <class _RandomAccessIterator, class _OutputIterator, class _BinaryPredicate>
_OutputIterator __brick_unique_copy(_RandomAccessIterator, _RandomAccessIterator, _OutputIterator, _BinaryPredicate,
                                    /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _BinaryPredicate,
          class _IsVector>
_OutputIterator
__pattern_unique_copy(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _OutputIterator, _BinaryPredicate,
                      _IsVector, /*parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _DifferenceType, class _RandomAccessIterator, class _BinaryPredicate>
_DifferenceType
__brick_calc_mask_2(_RandomAccessIterator, _RandomAccessIterator, bool* __restrict, _BinaryPredicate,
                    /*vector=*/std::false_type) noexcept;

template <class _DifferenceType, class _RandomAccessIterator, class _BinaryPredicate>
_DifferenceType
__brick_calc_mask_2(_RandomAccessIterator, _RandomAccessIterator, bool* __restrict, _BinaryPredicate,
                    /*vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator, class _BinaryPredicate,
          class _IsVector>
_OutputIterator
__pattern_unique_copy(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _OutputIterator,
                      _BinaryPredicate, _IsVector, /*parallel=*/std::true_type);

//------------------------------------------------------------------------
// reverse
//------------------------------------------------------------------------

template <class _BidirectionalIterator>
void __brick_reverse(_BidirectionalIterator, _BidirectionalIterator,
                     /*__is_vector=*/std::false_type) noexcept;

template <class _BidirectionalIterator>
void __brick_reverse(_BidirectionalIterator, _BidirectionalIterator,
                     /*__is_vector=*/std::true_type) noexcept;

template <class _BidirectionalIterator>
void __brick_reverse(_BidirectionalIterator, _BidirectionalIterator, _BidirectionalIterator,
                     /*is_vector=*/std::false_type) noexcept;

template <class _BidirectionalIterator>
void __brick_reverse(_BidirectionalIterator, _BidirectionalIterator, _BidirectionalIterator,
                     /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _IsVector>
void
__pattern_reverse(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _IsVector,
                  /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _IsVector>
void
__pattern_reverse(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _IsVector,
                  /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// reverse_copy
//------------------------------------------------------------------------

template <class _BidirectionalIterator, class _OutputIterator>
_OutputIterator __brick_reverse_copy(_BidirectionalIterator, _BidirectionalIterator, _OutputIterator,
                                     /*is_vector=*/std::false_type) noexcept;

template <class _BidirectionalIterator, class _OutputIterator>
_OutputIterator __brick_reverse_copy(_BidirectionalIterator, _BidirectionalIterator, _OutputIterator,
                                     /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_reverse_copy(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _OutputIterator, _IsVector,
                       /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_reverse_copy(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _OutputIterator, _IsVector,
                       /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// rotate
//------------------------------------------------------------------------

template <class _ForwardIterator>
_ForwardIterator __brick_rotate(_ForwardIterator, _ForwardIterator, _ForwardIterator,
                                /*is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator>
_ForwardIterator __brick_rotate(_ForwardIterator, _ForwardIterator, _ForwardIterator,
                                /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _IsVector>
_ForwardIterator
__pattern_rotate(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _ForwardIterator, _IsVector,
                 /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _IsVector>
_ForwardIterator
__pattern_rotate(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _ForwardIterator, _IsVector,
                 /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// rotate_copy
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator __brick_rotate_copy(_ForwardIterator, _ForwardIterator, _ForwardIterator, _OutputIterator,
                                    /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator __brick_rotate_copy(_ForwardIterator, _ForwardIterator, _ForwardIterator, _OutputIterator,
                                    /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_rotate_copy(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _ForwardIterator, _OutputIterator,
                      _IsVector,
                      /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator, class _IsVector>
_OutputIterator
__pattern_rotate_copy(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _ForwardIterator, _OutputIterator,
                      _IsVector,
                      /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// is_partitioned
//------------------------------------------------------------------------

template <class _ForwardIterator, class _UnaryPredicate>
bool __brick_is_partitioned(_ForwardIterator, _ForwardIterator, _UnaryPredicate,
                            /*is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _UnaryPredicate>
bool __brick_is_partitioned(_ForwardIterator, _ForwardIterator, _UnaryPredicate,
                            /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
bool
__pattern_is_partitioned(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _UnaryPredicate, _IsVector,
                         /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
bool
__pattern_is_partitioned(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _UnaryPredicate, _IsVector,
                         /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// partition
//------------------------------------------------------------------------

template <class _ForwardIterator, class _UnaryPredicate>
_ForwardIterator __brick_partition(_ForwardIterator, _ForwardIterator, _UnaryPredicate,
                                   /*is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _UnaryPredicate>
_ForwardIterator __brick_partition(_ForwardIterator, _ForwardIterator, _UnaryPredicate,
                                   /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_partition(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _UnaryPredicate, _IsVector,
                    /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_partition(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _UnaryPredicate, _IsVector,
                    /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// stable_partition
//------------------------------------------------------------------------

template <class _BidirectionalIterator, class _UnaryPredicate>
_BidirectionalIterator __brick_stable_partition(_BidirectionalIterator, _BidirectionalIterator, _UnaryPredicate,
                                                /*__is_vector=*/std::false_type) noexcept;

template <class _BidirectionalIterator, class _UnaryPredicate>
_BidirectionalIterator __brick_stable_partition(_BidirectionalIterator, _BidirectionalIterator, _UnaryPredicate,
                                                /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _UnaryPredicate, class _IsVector>
_BidirectionalIterator
__pattern_stable_partition(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _UnaryPredicate,
                           _IsVector,
                           /*is_parallelization=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _UnaryPredicate, class _IsVector>
_BidirectionalIterator
__pattern_stable_partition(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _UnaryPredicate,
                           _IsVector,
                           /*is_parallelization=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// partition_copy
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator1, class _OutputIterator2, class _UnaryPredicate>
std::pair<_OutputIterator1, _OutputIterator2>
    __brick_partition_copy(_ForwardIterator, _ForwardIterator, _OutputIterator1, _OutputIterator2, _UnaryPredicate,
                           /*is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator, class _OutputIterator1, class _OutputIterator2, class _UnaryPredicate>
std::pair<_OutputIterator1, _OutputIterator2>
    __brick_partition_copy(_ForwardIterator, _ForwardIterator, _OutputIterator1, _OutputIterator2, _UnaryPredicate,
                           /*is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _OutputIterator1, class _OutputIterator2,
          class _UnaryPredicate, class _IsVector>
std::pair<_OutputIterator1, _OutputIterator2>
__pattern_partition_copy(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _OutputIterator1, _OutputIterator2,
                         _UnaryPredicate, _IsVector,
                         /*is_parallelization=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _OutputIterator1, class _OutputIterator2,
          class _UnaryPredicate, class _IsVector>
std::pair<_OutputIterator1, _OutputIterator2>
__pattern_partition_copy(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _OutputIterator1,
                         _OutputIterator2, _UnaryPredicate, _IsVector,
                         /*is_parallelization=*/std::true_type);

//------------------------------------------------------------------------
// sort
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector,
          class _IsMoveConstructible>
void
__pattern_sort(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Compare, _IsVector /*is_vector*/,
               /*is_parallel=*/std::false_type, _IsMoveConstructible) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_sort(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Compare, _IsVector /*is_vector*/,
               /*is_parallel=*/std::true_type,
               /*is_move_constructible=*/std::true_type);

//------------------------------------------------------------------------
// stable_sort
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_stable_sort(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Compare,
                      _IsVector /*is_vector*/,
                      /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_stable_sort(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Compare,
                      _IsVector /*is_vector*/,
                      /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// partial_sort
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_partial_sort(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _RandomAccessIterator,
                       _Compare, _IsVector,
                       /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_partial_sort(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _RandomAccessIterator,
                       _Compare, _IsVector,
                       /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// partial_sort_copy
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _ForwardIterator, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_partial_sort_copy(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _RandomAccessIterator,
                            _RandomAccessIterator, _Compare, _IsVector,
                            /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_partial_sort_copy(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _RandomAccessIterator,
                            _RandomAccessIterator, _Compare, _IsVector,
                            /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// adjacent_find
//------------------------------------------------------------------------

template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator
__brick_adjacent_find(_ForwardIterator, _ForwardIterator, _BinaryPredicate,
                      /* IsVector = */ std::true_type, bool) noexcept;

template <class _ForwardIterator, class _BinaryPredicate>
_ForwardIterator
__brick_adjacent_find(_ForwardIterator, _ForwardIterator, _BinaryPredicate,
                      /* IsVector = */ std::false_type, bool) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _BinaryPredicate, class _IsVector>
_ForwardIterator
__pattern_adjacent_find(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _BinaryPredicate,
                        /* is_parallel */ std::false_type, _IsVector, bool) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _BinaryPredicate, class _IsVector>
_RandomAccessIterator
__pattern_adjacent_find(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _BinaryPredicate,
                        /* is_parallel */ std::true_type, _IsVector, bool);

//------------------------------------------------------------------------
// nth_element
//------------------------------------------------------------------------
template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_nth_element(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _RandomAccessIterator, _Compare,
                      _IsVector,
                      /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
void
__pattern_nth_element(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _RandomAccessIterator, _Compare,
                      _IsVector,
                      /*is_parallel=*/std::true_type) noexcept;

//------------------------------------------------------------------------
// fill, fill_n
//------------------------------------------------------------------------
template <class _ForwardIterator, class _Tp>
void
__brick_fill(_ForwardIterator, _ForwardIterator, const _Tp&,
             /* __is_vector = */ std::true_type) noexcept;

template <class _ForwardIterator, class _Tp>
void
__brick_fill(_ForwardIterator, _ForwardIterator, const _Tp&,
             /* __is_vector = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _IsVector>
void
__pattern_fill(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, const _Tp&,
               /*is_parallel=*/std::false_type, _IsVector) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _IsVector>
_ForwardIterator
__pattern_fill(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, const _Tp&,
               /*is_parallel=*/std::true_type, _IsVector);

template <class _OutputIterator, class _Size, class _Tp>
_OutputIterator
__brick_fill_n(_OutputIterator, _Size, const _Tp&,
               /* __is_vector = */ std::true_type) noexcept;

template <class _OutputIterator, class _Size, class _Tp>
_OutputIterator
__brick_fill_n(_OutputIterator, _Size, const _Tp&,
               /* __is_vector = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _OutputIterator, class _Size, class _Tp, class _IsVector>
_OutputIterator
__pattern_fill_n(_ExecutionPolicy&&, _OutputIterator, _Size, const _Tp&,
                 /*is_parallel=*/std::false_type, _IsVector) noexcept;

template <class _ExecutionPolicy, class _OutputIterator, class _Size, class _Tp, class _IsVector>
_OutputIterator
__pattern_fill_n(_ExecutionPolicy&&, _OutputIterator, _Size, const _Tp&,
                 /*is_parallel=*/std::true_type, _IsVector);

//------------------------------------------------------------------------
// generate, generate_n
//------------------------------------------------------------------------

template <class _RandomAccessIterator, class _Generator>
void __brick_generate(_RandomAccessIterator, _RandomAccessIterator, _Generator,
                      /* is_vector = */ std::true_type) noexcept;

template <class _ForwardIterator, class _Generator>
void __brick_generate(_ForwardIterator, _ForwardIterator, _Generator,
                      /* is_vector = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Generator, class _IsVector>
void
__pattern_generate(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Generator,
                   /*is_parallel=*/std::false_type, _IsVector) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _Generator, class _IsVector>
_ForwardIterator
__pattern_generate(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Generator,
                   /*is_parallel=*/std::true_type, _IsVector);

template <class OutputIterator, class Size, class _Generator>
OutputIterator __brick_generate_n(OutputIterator, Size, _Generator,
                                  /* is_vector = */ std::true_type) noexcept;

template <class OutputIterator, class Size, class _Generator>
OutputIterator __brick_generate_n(OutputIterator, Size, _Generator,
                                  /* is_vector = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class OutputIterator, class Size, class _Generator, class _IsVector>
OutputIterator
__pattern_generate_n(_ExecutionPolicy&&, OutputIterator, Size, _Generator,
                     /*is_parallel=*/std::false_type, _IsVector) noexcept;

template <class _ExecutionPolicy, class OutputIterator, class Size, class _Generator, class _IsVector>
OutputIterator
__pattern_generate_n(_ExecutionPolicy&&, OutputIterator, Size, _Generator,
                     /*is_parallel=*/std::true_type, _IsVector);

//------------------------------------------------------------------------
// remove
//------------------------------------------------------------------------
template <class _ForwardIterator, class _UnaryPredicate>
_ForwardIterator __brick_remove_if(_ForwardIterator, _ForwardIterator, _UnaryPredicate,
                                   /* __is_vector = */ std::false_type) noexcept;

template <class _RandomAccessIterator, class _UnaryPredicate>
_RandomAccessIterator __brick_remove_if(_RandomAccessIterator, _RandomAccessIterator, _UnaryPredicate,
                                        /* __is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_remove_if(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _UnaryPredicate, _IsVector,
                    /*is_parallel*/ std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator, class _UnaryPredicate, class _IsVector>
_ForwardIterator
__pattern_remove_if(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _UnaryPredicate, _IsVector,
                    /*is_parallel*/ std::true_type) noexcept;

//------------------------------------------------------------------------
// merge
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_merge(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                              _OutputIterator, _Compare,
                              /* __is_vector = */ std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_merge(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                              _OutputIterator, _Compare,
                              /* __is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_merge(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                _OutputIterator, _Compare, _IsVector, /* is_parallel = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_merge(_ExecutionPolicy&&, _RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                _RandomAccessIterator2, _OutputIterator, _Compare, _IsVector,
                /* is_parallel = */ std::true_type);

//------------------------------------------------------------------------
// inplace_merge
//------------------------------------------------------------------------

template <class _BidirectionalIterator, class _Compare>
void __brick_inplace_merge(_BidirectionalIterator, _BidirectionalIterator, _BidirectionalIterator, _Compare,
                           /* __is_vector = */ std::false_type) noexcept;

template <class _BidirectionalIterator, class _Compare>
void __brick_inplace_merge(_BidirectionalIterator, _BidirectionalIterator, _BidirectionalIterator, _Compare,
                           /* __is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _Compare, class _IsVector>
void
__pattern_inplace_merge(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _BidirectionalIterator,
                        _Compare, _IsVector,
                        /* is_parallel = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _BidirectionalIterator, class _Compare, class _IsVector>
void
__pattern_inplace_merge(_ExecutionPolicy&&, _BidirectionalIterator, _BidirectionalIterator, _BidirectionalIterator,
                        _Compare, _IsVector,
                        /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// includes
//------------------------------------------------------------------------

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_includes(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                   _Compare, _IsVector,
                   /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_includes(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                   _Compare, _IsVector,
                   /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// set_union
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_union(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                  _OutputIterator, _Compare,
                                  /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_union(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                  _OutputIterator, _Compare,
                                  /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_union(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                    _OutputIterator, _Compare, _IsVector, /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_union(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                    _OutputIterator, _Compare, _IsVector, /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// set_intersection
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_intersection(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                         _OutputIterator, _Compare,
                                         /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_intersection(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                         _OutputIterator, _Compare,
                                         /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_intersection(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                           _ForwardIterator2, _OutputIterator, _Compare, _IsVector,
                           /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_intersection(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                           _ForwardIterator2, _OutputIterator, _Compare, _IsVector, /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// set_difference
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_difference(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                       _OutputIterator, _Compare,
                                       /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_difference(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                       _OutputIterator, _Compare,
                                       /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_difference(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                         _OutputIterator, _Compare, _IsVector, /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_difference(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                         _OutputIterator, _Compare, _IsVector, /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// set_symmetric_difference
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_symmetric_difference(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator2, _OutputIterator, _Compare,
                                                 /*__is_vector=*/std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator, class _Compare>
_OutputIterator __brick_set_symmetric_difference(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                                                 _ForwardIterator2, _OutputIterator, _Compare,
                                                 /*__is_vector=*/std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_symmetric_difference(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                                   _ForwardIterator2, _OutputIterator, _Compare, _IsVector,
                                   /*is_parallel=*/std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _OutputIterator,
          class _Compare, class _IsVector>
_OutputIterator
__pattern_set_symmetric_difference(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                                   _ForwardIterator2, _OutputIterator, _Compare, _IsVector,
                                   /*is_parallel=*/std::true_type);

//------------------------------------------------------------------------
// is_heap_until
//------------------------------------------------------------------------

template <class _RandomAccessIterator, class _Compare>
_RandomAccessIterator __brick_is_heap_until(_RandomAccessIterator, _RandomAccessIterator, _Compare,
                                            /* __is_vector = */ std::false_type) noexcept;

template <class _RandomAccessIterator, class _Compare>
_RandomAccessIterator __brick_is_heap_until(_RandomAccessIterator, _RandomAccessIterator, _Compare,
                                            /* __is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_is_heap_until(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Compare, _IsVector,
                        /* is_parallel = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator, class _Compare, class _IsVector>
_RandomAccessIterator
__pattern_is_heap_until(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Compare, _IsVector,
                        /* is_parallel = */ std::true_type) noexcept;

//------------------------------------------------------------------------
// min_element
//------------------------------------------------------------------------

template <typename _ForwardIterator, typename _Compare>
_ForwardIterator __brick_min_element(_ForwardIterator, _ForwardIterator, _Compare,
                                     /* __is_vector = */ std::false_type) noexcept;

template <typename _ForwardIterator, typename _Compare>
_ForwardIterator __brick_min_element(_ForwardIterator, _ForwardIterator, _Compare,
                                     /* __is_vector = */ std::true_type) noexcept;

template <typename _ExecutionPolicy, typename _ForwardIterator, typename _Compare, typename _IsVector>
_ForwardIterator
__pattern_min_element(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Compare, _IsVector,
                      /* is_parallel = */ std::false_type) noexcept;

template <typename _ExecutionPolicy, typename _RandomAccessIterator, typename _Compare, typename _IsVector>
_RandomAccessIterator
__pattern_min_element(_ExecutionPolicy&&, _RandomAccessIterator, _RandomAccessIterator, _Compare, _IsVector,
                      /* is_parallel = */ std::true_type);

//------------------------------------------------------------------------
// minmax_element
//------------------------------------------------------------------------

template <typename _ForwardIterator, typename _Compare>
std::pair<_ForwardIterator, _ForwardIterator> __brick_minmax_element(_ForwardIterator, _ForwardIterator, _Compare,
                                                                     /* __is_vector = */ std::false_type) noexcept;

template <typename _ForwardIterator, typename _Compare>
std::pair<_ForwardIterator, _ForwardIterator> __brick_minmax_element(_ForwardIterator, _ForwardIterator, _Compare,
                                                                     /* __is_vector = */ std::true_type) noexcept;

template <typename _ExecutionPolicy, typename _ForwardIterator, typename _Compare, typename _IsVector>
std::pair<_ForwardIterator, _ForwardIterator>
__pattern_minmax_element(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Compare, _IsVector,
                         /* is_parallel = */ std::false_type) noexcept;

template <typename _ExecutionPolicy, typename _ForwardIterator, typename _Compare, typename _IsVector>
std::pair<_ForwardIterator, _ForwardIterator>
__pattern_minmax_element(_ExecutionPolicy&&, _ForwardIterator, _ForwardIterator, _Compare, _IsVector,
                         /* is_parallel = */ std::true_type);

//------------------------------------------------------------------------
// mismatch
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _Predicate>
std::pair<_ForwardIterator1, _ForwardIterator2> __brick_mismatch(_ForwardIterator1, _ForwardIterator1,
                                                                 _ForwardIterator2, _ForwardIterator2, _Predicate,
                                                                 /* __is_vector = */ std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _Predicate>
std::pair<_ForwardIterator1, _ForwardIterator2> __brick_mismatch(_ForwardIterator1, _ForwardIterator1,
                                                                 _ForwardIterator2, _ForwardIterator2, _Predicate,
                                                                 /* __is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Predicate, class _IsVector>
std::pair<_ForwardIterator1, _ForwardIterator2>
__pattern_mismatch(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                   _Predicate, _IsVector,
                   /* is_parallel = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _RandomAccessIterator1, class _RandomAccessIterator2, class _Predicate,
          class _IsVector>
std::pair<_RandomAccessIterator1, _RandomAccessIterator2>
__pattern_mismatch(_ExecutionPolicy&&, _RandomAccessIterator1, _RandomAccessIterator1, _RandomAccessIterator2,
                   _RandomAccessIterator2, _Predicate, _IsVector, /* is_parallel = */ std::true_type) noexcept;

//------------------------------------------------------------------------
// lexicographical_compare
//------------------------------------------------------------------------

template <class _ForwardIterator1, class _ForwardIterator2, class _Compare>
bool __brick_lexicographical_compare(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                     _Compare,
                                     /* __is_vector = */ std::false_type) noexcept;

template <class _ForwardIterator1, class _ForwardIterator2, class _Compare>
bool __brick_lexicographical_compare(_ForwardIterator1, _ForwardIterator1, _ForwardIterator2, _ForwardIterator2,
                                     _Compare,
                                     /* __is_vector = */ std::true_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_lexicographical_compare(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                                  _ForwardIterator2, _Compare, _IsVector, /* is_parallel = */ std::false_type) noexcept;

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Compare, class _IsVector>
bool
__pattern_lexicographical_compare(_ExecutionPolicy&&, _ForwardIterator1, _ForwardIterator1, _ForwardIterator2,
                                  _ForwardIterator2, _Compare, _IsVector, /* is_parallel = */ std::true_type) noexcept;

} // namespace __internal
} // namespace __pstl
#endif /* _PSTL_ALGORITHM_FWD_H */
