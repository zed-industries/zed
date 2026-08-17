// -*- C++ -*-
//===-- execution_defs.h --------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_EXECUTION_POLICY_DEFS_H
#define _PSTL_EXECUTION_POLICY_DEFS_H

#include <type_traits>

namespace __pstl
{
namespace execution
{
inline namespace v1
{

// 2.4, Sequential execution policy
class sequenced_policy
{
  public:
    // For internal use only
    static constexpr std::false_type
    __allow_unsequenced()
    {
        return std::false_type{};
    }
    static constexpr std::false_type
    __allow_vector()
    {
        return std::false_type{};
    }
    static constexpr std::false_type
    __allow_parallel()
    {
        return std::false_type{};
    }
};

// 2.5, Parallel execution policy
class parallel_policy
{
  public:
    // For internal use only
    static constexpr std::false_type
    __allow_unsequenced()
    {
        return std::false_type{};
    }
    static constexpr std::false_type
    __allow_vector()
    {
        return std::false_type{};
    }
    static constexpr std::true_type
    __allow_parallel()
    {
        return std::true_type{};
    }
};

// 2.6, Parallel+Vector execution policy
class parallel_unsequenced_policy
{
  public:
    // For internal use only
    static constexpr std::true_type
    __allow_unsequenced()
    {
        return std::true_type{};
    }
    static constexpr std::true_type
    __allow_vector()
    {
        return std::true_type{};
    }
    static constexpr std::true_type
    __allow_parallel()
    {
        return std::true_type{};
    }
};

class unsequenced_policy
{
  public:
    // For internal use only
    static constexpr std::true_type
    __allow_unsequenced()
    {
        return std::true_type{};
    }
    static constexpr std::true_type
    __allow_vector()
    {
        return std::true_type{};
    }
    static constexpr std::false_type
    __allow_parallel()
    {
        return std::false_type{};
    }
};

// 2.8, Execution policy objects
constexpr sequenced_policy seq{};
constexpr parallel_policy par{};
constexpr parallel_unsequenced_policy par_unseq{};
constexpr unsequenced_policy unseq{};

// 2.3, Execution policy type trait
template <class _Tp>
struct is_execution_policy : std::false_type
{
};

template <>
struct is_execution_policy<__pstl::execution::sequenced_policy> : std::true_type
{
};
template <>
struct is_execution_policy<__pstl::execution::parallel_policy> : std::true_type
{
};
template <>
struct is_execution_policy<__pstl::execution::parallel_unsequenced_policy> : std::true_type
{
};
template <>
struct is_execution_policy<__pstl::execution::unsequenced_policy> : std::true_type
{
};

#if _PSTL_CPP14_VARIABLE_TEMPLATES_PRESENT
template <class _Tp>
constexpr bool is_execution_policy_v = __pstl::execution::is_execution_policy<_Tp>::value;
#endif

} // namespace v1
} // namespace execution

namespace __internal
{
template <class _ExecPolicy, class _Tp>
#if _GLIBCXX_RELEASE >= 9
using __enable_if_execution_policy =
    typename std::enable_if<__pstl::execution::is_execution_policy<std::__remove_cvref_t<_ExecPolicy>>::value,
                            _Tp>::type;
#else
using __enable_if_execution_policy =
    typename std::enable_if<__pstl::execution::is_execution_policy<typename std::decay<_ExecPolicy>::type>::value,
                            _Tp>::type;
#endif
} // namespace __internal

} // namespace __pstl

#endif /* _PSTL_EXECUTION_POLICY_DEFS_H */
