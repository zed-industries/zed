// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_SYSTEM_UNSAFE_BUFFERS_H_
#define TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_SYSTEM_UNSAFE_BUFFERS_H_

inline int system_bad_stuff(int* i, unsigned s) {
  return i[s];  // This is in a system header, so no warning is emitted.
}

#endif  // TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_SYSTEM_UNSAFE_BUFFERS_H_
