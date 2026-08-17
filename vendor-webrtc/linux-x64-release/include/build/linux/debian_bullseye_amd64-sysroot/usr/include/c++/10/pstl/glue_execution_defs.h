// -*- C++ -*-
//===-- glue_execution_defs.h ---------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_GLUE_EXECUTION_DEFS_H
#define _PSTL_GLUE_EXECUTION_DEFS_H

#include <type_traits>

#include "execution_defs.h"

namespace std
{
// Type trait
using __pstl::execution::is_execution_policy;
#if _PSTL_CPP14_VARIABLE_TEMPLATES_PRESENT
#    if __INTEL_COMPILER
template <class T>
constexpr bool is_execution_policy_v = is_execution_policy<T>::value;
#    else
using __pstl::execution::is_execution_policy_v;
#    endif
#endif

namespace execution
{
// Standard C++ policy classes
using __pstl::execution::parallel_policy;
using __pstl::execution::parallel_unsequenced_policy;
using __pstl::execution::sequenced_policy;

// Standard predefined policy instances
using __pstl::execution::par;
using __pstl::execution::par_unseq;
using __pstl::execution::seq;

// Implementation-defined names
// Unsequenced policy is not yet standard, but for consistency
// we include it into namespace std::execution as well
using __pstl::execution::unseq;
using __pstl::execution::unsequenced_policy;
} // namespace execution
} // namespace std

#include "algorithm_impl.h"
#include "numeric_impl.h"
#include "parallel_backend.h"

#endif /* _PSTL_GLUE_EXECUTION_DEFS_H */
