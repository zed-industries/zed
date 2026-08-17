// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_CONTROLS_OVERLAY_CHANGED_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_CONTROLS_OVERLAY_CHANGED_DELEGATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace gfx {
class Rect;
}

namespace blink {

class LocalFrame;

// Extension point for observing changes to Window Controls Overlay geometry
// (e.g. the titlebar area); only supported on desktop platforms.
class CORE_EXPORT WindowControlsOverlayChangedDelegate
    : public GarbageCollectedMixin {
 public:
  // Notifies about a change to the Window Controls Overlay geometry.
  // `titlebar_area_rect` is the updated available titlebar area in the viewport
  // coordinate space.
  virtual void WindowControlsOverlayChanged(
      const gfx::Rect& titlebar_area_rect) = 0;

 protected:
  // If `LocalFrame` not null, `this` will be registered to observe window
  // controls overlay geometry changes. Otherwise, no notifications will be
  // dispatched.
  explicit WindowControlsOverlayChangedDelegate(LocalFrame*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_CONTROLS_OVERLAY_CHANGED_DELEGATE_H_
