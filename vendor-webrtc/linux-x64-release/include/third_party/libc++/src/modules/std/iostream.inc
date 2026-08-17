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
  using std::cerr;
  using std::cin;
  using std::clog;
  using std::cout;

#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wcerr;
  using std::wcin;
  using std::wclog;
  using std::wcout;
#  endif
#endif // _LIBCPP_HAS_LOCALIZATION
} // namespace std
