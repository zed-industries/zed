//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_ASSERT_TEST_HELPERS_H
#define TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_ASSERT_TEST_HELPERS_H

#if !__has_include(<unistd.h>) || !__has_include(<sys/wait.h>)
#  error "Requires UNIX headers"
#endif

#include "check_assertion.h"

template <typename StreamT>
inline void test_native_handle_assertion() {
  StreamT f;

  // non-const
  TEST_LIBCPP_ASSERT_FAILURE(f.native_handle(), "File must be opened");
  // const
  TEST_LIBCPP_ASSERT_FAILURE(std::as_const(f).native_handle(), "File must be opened");
}

#endif // TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_ASSERT_TEST_HELPERS_H
