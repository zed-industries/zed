//===---------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef _LIBCPP___CSTDDEF_PTRDIFF_T_H
#define _LIBCPP___CSTDDEF_PTRDIFF_T_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

using ptrdiff_t = decltype(static_cast<int*>(nullptr) - static_cast<int*>(nullptr));

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___CSTDDEF_PTRDIFF_T_H
