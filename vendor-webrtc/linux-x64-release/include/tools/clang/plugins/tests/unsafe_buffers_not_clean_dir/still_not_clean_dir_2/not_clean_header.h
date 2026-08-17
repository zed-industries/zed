// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_STILL_NOT_CLEAN_DIR_2_NOT_CLEAN_HEADER_H_
#define TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_STILL_NOT_CLEAN_DIR_2_NOT_CLEAN_HEADER_H_

inline int unchecked_violation_2(int* i, unsigned s) {
  return i[s];  // This is in a known-bad header, so no error is emitted.
}

#endif  // TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_NOT_CLEAN_DIR_STILL_NOT_CLEAN_DIR_2_NOT_CLEAN_HEADER_H_
