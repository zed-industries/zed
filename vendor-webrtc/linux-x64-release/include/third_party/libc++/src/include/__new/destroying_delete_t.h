//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___NEW_DESTROYING_DELETE_T_H
#define _LIBCPP___NEW_DESTROYING_DELETE_T_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

#if _LIBCPP_STD_VER >= 20
_LIBCPP_BEGIN_UNVERSIONED_NAMESPACE_STD
// Enable the declaration even if the compiler doesn't support the language
// feature.
struct destroying_delete_t {
  explicit destroying_delete_t() = default;
};
inline constexpr destroying_delete_t destroying_delete{};
_LIBCPP_END_UNVERSIONED_NAMESPACE_STD
#endif

#endif // _LIBCPP___NEW_DESTROYING_DELETE_T_H
