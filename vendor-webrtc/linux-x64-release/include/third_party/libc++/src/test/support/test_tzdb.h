// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_TEST_TZDB_H
#define SUPPORT_TEST_TZDB_H

#include <string_view>

#if defined(_LIBCPP_VERSION)

_LIBCPP_BEGIN_NAMESPACE_STD

namespace chrono {

// This function is marked as "overridable" in libc++ only for the test
// suite. Therefore the declaration is not in <chrono>.
_LIBCPP_AVAILABILITY_TZDB _LIBCPP_OVERRIDABLE_FUNC_VIS string_view __libcpp_tzdb_directory();

} // namespace chrono

_LIBCPP_END_NAMESPACE_STD

#endif

#endif // SUPPORT_TEST_TZDB_H
