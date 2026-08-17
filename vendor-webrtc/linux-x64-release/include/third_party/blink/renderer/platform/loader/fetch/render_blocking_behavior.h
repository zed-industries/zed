// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RENDER_BLOCKING_BEHAVIOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RENDER_BLOCKING_BEHAVIOR_H_

#include <cstdint>

namespace blink {
enum class RenderBlockingBehavior : uint8_t {
  kUnset,                 // Render blocking value was not set.
  kBlocking,              // Render Blocking resource.
  kNonBlocking,           // Non-blocking resource.
  kNonBlockingDynamic,    // Dynamically injected non-blocking resource.
  kPotentiallyBlocking,   // Dynamically injected non-blocking resource.
  kInBodyParserBlocking,  // Blocks parser below element declaration.
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RENDER_BLOCKING_BEHAVIOR_H_
