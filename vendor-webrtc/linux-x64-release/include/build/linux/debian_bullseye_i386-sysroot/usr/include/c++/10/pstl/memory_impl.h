// -*- C++ -*-
//===-- memory_impl.h -----------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_MEMORY_IMPL_H
#define _PSTL_MEMORY_IMPL_H

#include <iterator>

#include "unseq_backend_simd.h"

namespace __pstl
{
namespace __internal
{

//------------------------------------------------------------------------
// uninitialized_move
//------------------------------------------------------------------------

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_uninitialized_move(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                           /*vector=*/std::false_type) noexcept
{
    typedef typename std::iterator_traits<_OutputIterator>::value_type _ValueType2;
    for (; __first != __last; ++__first, ++__result)
    {
        ::new (std::addressof(*__result)) _ValueType2(std::move(*__first));
    }
    return __result;
}

template <class _ForwardIterator, class _OutputIterator>
_OutputIterator
__brick_uninitialized_move(_ForwardIterator __first, _ForwardIterator __last, _OutputIterator __result,
                           /*vector=*/std::true_type) noexcept
{
    typedef typename std::iterator_traits<_OutputIterator>::value_type __ValueType2;
    typedef typename std::iterator_traits<_ForwardIterator>::reference _ReferenceType1;
    typedef typename std::iterator_traits<_OutputIterator>::reference _ReferenceType2;

    return __unseq_backend::__simd_walk_2(
        __first, __last - __first, __result,
        [](_ReferenceType1 __x, _ReferenceType2 __y) { ::new (std::addressof(__y)) __ValueType2(std::move(__x)); });
}

} // namespace __internal
} // namespace __pstl

#endif /* _PSTL_MEMORY_IMPL_H */
