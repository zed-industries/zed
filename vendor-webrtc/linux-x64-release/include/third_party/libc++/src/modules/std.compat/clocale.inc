// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

export {
#if _LIBCPP_HAS_LOCALIZATION
  using ::lconv _LIBCPP_USING_IF_EXISTS;

  using ::localeconv _LIBCPP_USING_IF_EXISTS;
  using ::setlocale _LIBCPP_USING_IF_EXISTS;
#endif // _LIBCPP_HAS_LOCALIZATION
} // export
