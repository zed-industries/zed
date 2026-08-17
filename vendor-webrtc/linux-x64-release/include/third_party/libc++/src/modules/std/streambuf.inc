// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

export namespace std {
#if _LIBCPP_HAS_LOCALIZATION
  using std::basic_streambuf;
  using std::streambuf;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wstreambuf;
#  endif
#endif // _LIBCPP_HAS_LOCALIZATION
} // namespace std
