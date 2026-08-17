// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_LIBC_CALLS_CLEAN_H_
#define TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_LIBC_CALLS_CLEAN_H_

#include <system_string.h>

void* clean_bad_memcpy(int* i, unsigned s) {
  return memcpy(i, &s, sizeof(s));  // Should generate a warning.
}

void* clean_guarded_bad_memcpy(int* i, unsigned s) {
  return UNSAFE_BUFFERS(memcpy(i, &s, sizeof(s))); // Guarded so no warning.
}

#endif  // TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_LIBC_CALLS_CLEAN_H_
