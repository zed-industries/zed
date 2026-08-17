// -*- C++ -*-
//===-- execution_impl.h --------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_EXECUTION_IMPL_H
#define _PSTL_EXECUTION_IMPL_H

#include <iterator>
#include <type_traits>

#include "execution_defs.h"

namespace __pstl
{
namespace __internal
{

using namespace __pstl::execution;

/* predicate */

template <typename _Tp>
std::false_type __lazy_and(_Tp, std::false_type)
{
    return std::false_type{};
};

template <typename _Tp>
inline _Tp
__lazy_and(_Tp __a, std::true_type)
{
    return __a;
}

template <typename _Tp>
std::true_type __lazy_or(_Tp, std::true_type)
{
    return std::true_type{};
};

template <typename _Tp>
inline _Tp
__lazy_or(_Tp __a, std::false_type)
{
    return __a;
}

/* iterator */
template <typename _IteratorType, typename... _OtherIteratorTypes>
struct __is_random_access_iterator
{
    static constexpr bool value = __internal::__is_random_access_iterator<_IteratorType>::value &&
                                  __internal::__is_random_access_iterator<_OtherIteratorTypes...>::value;
    typedef std::integral_constant<bool, value> type;
};

template <typename _IteratorType>
struct __is_random_access_iterator<_IteratorType>
    : std::is_same<typename std::iterator_traits<_IteratorType>::iterator_category, std::random_access_iterator_tag>
{
};

/* policy */
template <typename _Policy>
struct __policy_traits
{
};

template <>
struct __policy_traits<sequenced_policy>
{
    typedef std::false_type allow_parallel;
    typedef std::false_type allow_unsequenced;
    typedef std::false_type allow_vector;
};

template <>
struct __policy_traits<unsequenced_policy>
{
    typedef std::false_type allow_parallel;
    typedef std::true_type allow_unsequenced;
    typedef std::true_type allow_vector;
};

template <>
struct __policy_traits<parallel_policy>
{
    typedef std::true_type allow_parallel;
    typedef std::false_type allow_unsequenced;
    typedef std::false_type allow_vector;
};

template <>
struct __policy_traits<parallel_unsequenced_policy>
{
    typedef std::true_type allow_parallel;
    typedef std::true_type allow_unsequenced;
    typedef std::true_type allow_vector;
};

template <typename _ExecutionPolicy>
using __collector_t =
    typename __internal::__policy_traits<typename std::decay<_ExecutionPolicy>::type>::__collector_type;

template <typename _ExecutionPolicy>
using __allow_vector =
    typename __internal::__policy_traits<typename std::decay<_ExecutionPolicy>::type>::__allow_vector;

template <typename _ExecutionPolicy>
using __allow_unsequenced =
    typename __internal::__policy_traits<typename std::decay<_ExecutionPolicy>::type>::__allow_unsequenced;

template <typename _ExecutionPolicy>
using __allow_parallel =
    typename __internal::__policy_traits<typename std::decay<_ExecutionPolicy>::type>::__allow_parallel;

template <typename _ExecutionPolicy, typename... _IteratorTypes>
auto
__is_vectorization_preferred(_ExecutionPolicy&& __exec)
    -> decltype(__internal::__lazy_and(__exec.__allow_vector(),
                                       typename __internal::__is_random_access_iterator<_IteratorTypes...>::type()))
{
    return __internal::__lazy_and(__exec.__allow_vector(),
                                  typename __internal::__is_random_access_iterator<_IteratorTypes...>::type());
}

template <typename _ExecutionPolicy, typename... _IteratorTypes>
auto
__is_parallelization_preferred(_ExecutionPolicy&& __exec)
    -> decltype(__internal::__lazy_and(__exec.__allow_parallel(),
                                       typename __internal::__is_random_access_iterator<_IteratorTypes...>::type()))
{
    return __internal::__lazy_and(__exec.__allow_parallel(),
                                  typename __internal::__is_random_access_iterator<_IteratorTypes...>::type());
}

template <typename policy, typename... _IteratorTypes>
struct __prefer_unsequenced_tag
{
    static constexpr bool value = __internal::__allow_unsequenced<policy>::value &&
                                  __internal::__is_random_access_iterator<_IteratorTypes...>::value;
    typedef std::integral_constant<bool, value> type;
};

template <typename policy, typename... _IteratorTypes>
struct __prefer_parallel_tag
{
    static constexpr bool value = __internal::__allow_parallel<policy>::value &&
                                  __internal::__is_random_access_iterator<_IteratorTypes...>::value;
    typedef std::integral_constant<bool, value> type;
};

} // namespace __internal
} // namespace __pstl

#endif /* _PSTL_EXECUTION_IMPL_H */
