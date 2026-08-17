// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_LIBC_CALLS_OPT_OUT_H_
#define TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_LIBC_CALLS_OPT_OUT_H_

// This header would be checked, but the pragma changes it.
#pragma allow_unsafe_libc_calls

#include <system_string.h>

inline void* opt_out_bad_memcpy(int* i, unsigned s) {
  return memcpy(i, &s, sizeof(s));  // No warning, allow_unsafe_libc_calls disables it.
}

inline int opt_out_bad_buffers(int* i, unsigned s) {
  return i[s];  // Gives warning, allow_unafe_libc_calls does not imply allow_unsafe_buffers.
}

#endif  // TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_LIBC_CALLS_OPT_OUT_H_
