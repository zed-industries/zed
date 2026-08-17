// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

export namespace std {
#if _LIBCPP_HAS_THREADS
  // [stoptoken], class stop_­token
  using std::stop_token;

  // [stopsource], class stop_­source
  using std::stop_source;

  // no-shared-stop-state indicator
  using std::nostopstate;
  using std::nostopstate_t;

  // [stopcallback], class template stop_­callback
  using std::stop_callback;
#endif // _LIBCPP_HAS_THREADS
} // namespace std
