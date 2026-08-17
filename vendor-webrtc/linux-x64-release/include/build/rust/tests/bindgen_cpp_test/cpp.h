// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BUILD_RUST_TESTS_BINDGEN_CPP_TEST_CPP_H_
#define BUILD_RUST_TESTS_BINDGEN_CPP_TEST_CPP_H_

// We use some C++20 stuff to ensure the correct mode is being used.
template <class T>
concept AlwaysTrue = true;

namespace functions {

inline constexpr int kNumber = 2;

int template_fn(AlwaysTrue auto i) {
  return i;
}

int normal_fn(int i);

}  // namespace functions

#endif  // BUILD_RUST_TESTS_BINDGEN_CPP_TEST_CPP_H_
