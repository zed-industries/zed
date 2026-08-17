// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_OVERLAY_SCROLLBAR_CLIP_BEHAVIOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_OVERLAY_SCROLLBAR_CLIP_BEHAVIOR_H_

namespace blink {

enum OverlayScrollbarClipBehavior {
  kIgnoreOverlayScrollbarSize,
  kExcludeOverlayScrollbarSizeForHitTesting
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_OVERLAY_SCROLLBAR_CLIP_BEHAVIOR_H_
