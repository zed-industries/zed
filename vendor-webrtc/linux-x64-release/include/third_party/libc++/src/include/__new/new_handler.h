//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___NEW_NEW_HANDLER_H
#define _LIBCPP___NEW_NEW_HANDLER_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

#if defined(_LIBCPP_ABI_VCRUNTIME)
#  include <new.h>
#else
_LIBCPP_BEGIN_UNVERSIONED_NAMESPACE_STD
typedef void (*new_handler)();
_LIBCPP_EXPORTED_FROM_ABI new_handler set_new_handler(new_handler) _NOEXCEPT;
_LIBCPP_EXPORTED_FROM_ABI new_handler get_new_handler() _NOEXCEPT;
_LIBCPP_END_UNVERSIONED_NAMESPACE_STD
#endif // _LIBCPP_ABI_VCRUNTIME

#endif // _LIBCPP___NEW_NEW_HANDLER_H
