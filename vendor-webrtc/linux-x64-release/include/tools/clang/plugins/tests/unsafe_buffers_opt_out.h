// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_OPT_OUT_H_
#define TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_OPT_OUT_H_

// This header would be checked, but the pragma changes it.
#pragma allow_unsafe_buffers

#include <system_string.h>

inline int opt_out_bad_stuff(int* i, unsigned s) {
  return i[s];  // No warning, allow_unsafe_buffers disables it.
}

inline void* opt_out_bad_stuff2(int* i, unsigned s) {
  return memcpy(i, &s, sizeof(s));  // No warning, allow_unsafe_buffers implies allow_unsafe_libc_calls.
}

#endif  // TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_OPT_OUT_H_
