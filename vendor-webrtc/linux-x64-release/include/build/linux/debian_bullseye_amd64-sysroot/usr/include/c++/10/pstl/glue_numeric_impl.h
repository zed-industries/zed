// -*- C++ -*-
//===-- glue_numeric_impl.h -----------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_GLUE_NUMERIC_IMPL_H
#define _PSTL_GLUE_NUMERIC_IMPL_H

#include <functional>

#include "utils.h"
#include "numeric_fwd.h"

namespace std
{

// [reduce]

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _BinaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _Tp>
reduce(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Tp __init,
       _BinaryOperation __binary_op)
{
    return transform_reduce(std::forward<_ExecutionPolicy>(__exec), __first, __last, __init, __binary_op,
                            __pstl::__internal::__no_op());
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _Tp>
reduce(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Tp __init)
{
    return transform_reduce(std::forward<_ExecutionPolicy>(__exec), __first, __last, __init, std::plus<_Tp>(),
                            __pstl::__internal::__no_op());
}

template <class _ExecutionPolicy, class _ForwardIterator>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy,
                                                 typename iterator_traits<_ForwardIterator>::value_type>
reduce(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last)
{
    typedef typename iterator_traits<_ForwardIterator>::value_type _ValueType;
    return transform_reduce(std::forward<_ExecutionPolicy>(__exec), __first, __last, _ValueType{},
                            std::plus<_ValueType>(), __pstl::__internal::__no_op());
}

// [transform.reduce]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _Tp>
transform_reduce(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                 _ForwardIterator2 __first2, _Tp __init)
{
    typedef typename iterator_traits<_ForwardIterator1>::value_type _InputType;
    using namespace __pstl;
    return __internal::__pattern_transform_reduce(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __init, std::plus<_InputType>(),
        std::multiplies<_InputType>(),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp, class _BinaryOperation1,
          class _BinaryOperation2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _Tp>
transform_reduce(_ExecutionPolicy&& __exec, _ForwardIterator1 __first1, _ForwardIterator1 __last1,
                 _ForwardIterator2 __first2, _Tp __init, _BinaryOperation1 __binary_op1, _BinaryOperation2 __binary_op2)
{
    using namespace __pstl;
    return __internal::__pattern_transform_reduce(
        std::forward<_ExecutionPolicy>(__exec), __first1, __last1, __first2, __init, __binary_op1, __binary_op2,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator, class _Tp, class _BinaryOperation, class _UnaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _Tp>
transform_reduce(_ExecutionPolicy&& __exec, _ForwardIterator __first, _ForwardIterator __last, _Tp __init,
                 _BinaryOperation __binary_op, _UnaryOperation __unary_op)
{
    using namespace __pstl;
    return __internal::__pattern_transform_reduce(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __init, __binary_op, __unary_op,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator>(__exec));
}

// [exclusive.scan]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
exclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
               _ForwardIterator2 __result, _Tp __init)
{
    return transform_exclusive_scan(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __init,
                                    std::plus<_Tp>(), __pstl::__internal::__no_op());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp, class _BinaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
exclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
               _ForwardIterator2 __result, _Tp __init, _BinaryOperation __binary_op)
{
    return transform_exclusive_scan(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __init,
                                    __binary_op, __pstl::__internal::__no_op());
}

// [inclusive.scan]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
inclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
               _ForwardIterator2 __result)
{
    typedef typename iterator_traits<_ForwardIterator1>::value_type _InputType;
    return transform_inclusive_scan(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result,
                                    std::plus<_InputType>(), __pstl::__internal::__no_op());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
inclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
               _ForwardIterator2 __result, _BinaryOperation __binary_op)
{
    return transform_inclusive_scan(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __binary_op,
                                    __pstl::__internal::__no_op());
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp, class _BinaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
inclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
               _ForwardIterator2 __result, _BinaryOperation __binary_op, _Tp __init)
{
    return transform_inclusive_scan(std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __binary_op,
                                    __pstl::__internal::__no_op(), __init);
}

// [transform.exclusive.scan]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _Tp, class _BinaryOperation,
          class _UnaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
transform_exclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                         _ForwardIterator2 __result, _Tp __init, _BinaryOperation __binary_op,
                         _UnaryOperation __unary_op)
{
    using namespace __pstl;
    return __internal::__pattern_transform_scan(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __unary_op, __init, __binary_op,
        /*inclusive=*/std::false_type(),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

// [transform.inclusive.scan]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryOperation,
          class _UnaryOperation, class _Tp>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
transform_inclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                         _ForwardIterator2 __result, _BinaryOperation __binary_op, _UnaryOperation __unary_op,
                         _Tp __init)
{
    using namespace __pstl;
    return __internal::__pattern_transform_scan(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __result, __unary_op, __init, __binary_op,
        /*inclusive=*/std::true_type(),
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _UnaryOperation,
          class _BinaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
transform_inclusive_scan(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                         _ForwardIterator2 __result, _BinaryOperation __binary_op, _UnaryOperation __unary_op)
{
    if (__first != __last)
    {
        auto __tmp = __unary_op(*__first);
        *__result = __tmp;
        return transform_inclusive_scan(std::forward<_ExecutionPolicy>(__exec), ++__first, __last, ++__result,
                                        __binary_op, __unary_op, __tmp);
    }
    else
    {
        return __result;
    }
}

// [adjacent.difference]

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2, class _BinaryOperation>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
adjacent_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                    _ForwardIterator2 __d_first, _BinaryOperation __op)
{

    if (__first == __last)
        return __d_first;

    using namespace __pstl;
    return __internal::__pattern_adjacent_difference(
        std::forward<_ExecutionPolicy>(__exec), __first, __last, __d_first, __op,
        __internal::__is_vectorization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec),
        __internal::__is_parallelization_preferred<_ExecutionPolicy, _ForwardIterator1, _ForwardIterator2>(__exec));
}

template <class _ExecutionPolicy, class _ForwardIterator1, class _ForwardIterator2>
__pstl::__internal::__enable_if_execution_policy<_ExecutionPolicy, _ForwardIterator2>
adjacent_difference(_ExecutionPolicy&& __exec, _ForwardIterator1 __first, _ForwardIterator1 __last,
                    _ForwardIterator2 __d_first)
{
    typedef typename iterator_traits<_ForwardIterator1>::value_type _ValueType;
    return adjacent_difference(std::forward<_ExecutionPolicy>(__exec), __first, __last, __d_first,
                               std::minus<_ValueType>());
}

} // namespace std

#endif /* _PSTL_GLUE_NUMERIC_IMPL_H_ */
