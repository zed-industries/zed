// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_THIRD_PARTY_BLINK_RENDERER_DISCOURAGED_TYPE_TEST_H_
#define TOOLS_CLANG_PLUGINS_TESTS_THIRD_PARTY_BLINK_RENDERER_DISCOURAGED_TYPE_TEST_H_

// The plugin should ignore this file because it has "_test." in its filename.

#include <vector>

namespace blink {

struct FooTest {
  std::vector<int> v1;

  using FloatVector = std::vector<float>;
  FloatVector v2;

  std::vector<char> v_array[4][4];
};

}  // namespace blink

#endif  // TOOLS_CLANG_PLUGINS_TESTS_THIRD_PARTY_BLINK_RENDERER_DISCOURAGED_TYPE_TEST_H_
