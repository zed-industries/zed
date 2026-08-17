// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LOGICAL_DIRECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LOGICAL_DIRECTION_H_

#include <cstdint>

namespace blink {

enum class LogicalDirection : uint8_t {
  kBlockStart = 0,
  kBlockEnd,
  kInlineStart,
  kInlineEnd
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LOGICAL_DIRECTION_H_
