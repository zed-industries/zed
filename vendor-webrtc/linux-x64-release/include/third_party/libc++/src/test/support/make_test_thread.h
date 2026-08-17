//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_MAKE_TEST_THREAD_H
#define TEST_SUPPORT_MAKE_TEST_THREAD_H

#include <thread>
#include <utility>

#include "test_macros.h"

namespace support {

// These functions are used to mock the creation of threads within the test suite.
//
// This provides a vendor-friendly way of making the test suite work even on platforms
// where the standard thread constructors don't work (e.g. embedded environments where
// creating a thread requires additional information like setting attributes).
//
// Vendors can keep a downstream diff in this file to create threads however they
// need on their platform, and the majority of the test suite will work out of the
// box. Of course, tests that exercise the standard thread constructors won't work,
// but any other test that only creates threads as a side effect of testing should
// work if they use the utilities in this file.

template <class F, class... Args>
std::thread make_test_thread(F&& f, Args&&... args) {
  return std::thread(std::forward<F>(f), std::forward<Args>(args)...);
}

#if TEST_STD_VER >= 20
#  ifdef _LIBCPP_VERSION
#    define TEST_AVAILABILITY_SYNC _LIBCPP_AVAILABILITY_SYNC
#  else
#    define TEST_AVAILABILITY_SYNC
#  endif

template <class F, class... Args>
TEST_AVAILABILITY_SYNC std::jthread make_test_jthread(F&& f, Args&&... args) {
  return std::jthread(std::forward<F>(f), std::forward<Args>(args)...);
}
#endif

} // namespace support

#endif // TEST_SUPPORT_MAKE_TEST_THREAD_H
