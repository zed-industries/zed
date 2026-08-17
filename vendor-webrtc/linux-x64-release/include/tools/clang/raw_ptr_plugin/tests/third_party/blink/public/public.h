// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_BLINK_PUBLIC_PUBLIC_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_BLINK_PUBLIC_PUBLIC_H_

#include <vector>

namespace blink {

// Blink public API is allowed to use type aliases that resolve to STL
// containers.
using BlinkPublicType = std::vector<int>;

}  // namespace blink

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_BLINK_RENDERER_PUBLIC_PUBLIC_H_
