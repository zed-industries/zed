// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_CLEAN_H_
#define TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_CLEAN_H_

int clean_bad_stuff(int* i, unsigned s) {
  return i[s];  // This is in a "clean" file, so it should make a warning.
}

int clean_guarded_bad_stuff(int* i, unsigned s) {
  return UNSAFE_BUFFERS(i[s]);  // Guarded so no warning.
}

UNSAFE_FN void unsafe_fn() {}

inline void call_unsafe_stuff() {
  unsafe_fn();  // Unannotated call causes error.
  unsafe_fn();  // Second one uses caching and still makes an error.

  // Annotated call is okay.
  UNSAFE_BUFFERS(unsafe_fn());
}

// Unrelated pragmas
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunused-variable"
#pragma clang diagnostic pop

#endif  // TOOLS_CLANG_PLUGINS_TESTS_UNSAFE_BUFFERS_CLEAN_H_
