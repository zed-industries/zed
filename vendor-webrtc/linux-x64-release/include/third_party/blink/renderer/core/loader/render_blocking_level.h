// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_LEVEL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_LEVEL_H_

namespace blink {

// The values must be contiguous for iterating through all the possible values.
enum class RenderBlockingLevel {
  kNone = 0,
  kLimitFrameRate = 1,
  kBlock = 2,
  kMax = kBlock,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_LEVEL_H_
