// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_NOT_CHECKED_HEADER_H_
#define TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_NOT_CHECKED_HEADER_H_

#include <system_unsafe_buffers.h>

#include "clean_header.h"
#include "not_clean_header.h"

// This file would not be checked, but the pragma opts in.
DO_UNSAFE_THING_FROM_CHECKED_HEADER(NotChecked, N, i, s);    // No error.
DO_UNSAFE_THING_FROM_UNCHECKED_HEADER(NotChecked, N, i, s);  // No error.

inline int allowed_bad_stuff(int* i, unsigned s) {
  auto a = UncheckStructThingTryToMakeScratchBufferNotChecked();
  auto b = CheckStructThingTryToMakeScratchBufferNotChecked();

  auto x = [&]() { return i; };
  // This is in a known-bad header, so no error is emitted.
  return MACRO_CALL_FUNCTION_FROM_CHECKED_HEADER(x)[s] +    // No error.
         MACRO_CALL_FUNCTION_FROM_UNCHECKED_HEADER(x)[s] +  // No error.
         i[s];                                              // No error.
}

// Lie about what file this is. The plugin should not care since it wants to
// apply paths to the file where the text is written, which is this file's path.
#line 38 "some_other_file.h"

int misdirected_bad_stuff(int* i, unsigned s) {
  return i[s];  // This should not make an error since it's in a known-bad
                // header.
}

#endif  // TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_NOT_CHECKED_HEADER_H_
