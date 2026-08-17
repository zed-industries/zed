// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BUILD_RUST_TESTS_TEST_RS_BINDINGS_FROM_CC_SELF_CONTAINED_TARGET_HEADER1_H_
#define BUILD_RUST_TESTS_TEST_RS_BINDINGS_FROM_CC_SELF_CONTAINED_TARGET_HEADER1_H_

inline int MultiplyViaCc(int x, int y) {
  return x * y;
}

#endif  // BUILD_RUST_TESTS_TEST_RS_BINDINGS_FROM_CC_SELF_CONTAINED_TARGET_HEADER1_H_
