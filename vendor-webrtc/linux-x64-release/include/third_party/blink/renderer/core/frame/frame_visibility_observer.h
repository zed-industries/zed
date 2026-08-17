// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VISIBILITY_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VISIBILITY_OBSERVER_H_

#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LocalFrame;

// This is an observer to observe changes to the in-viewport visibility of a
// given frame.
class CORE_EXPORT FrameVisibilityObserver : public GarbageCollectedMixin {
 public:
  virtual ~FrameVisibilityObserver() = default;

  virtual void FrameVisibilityChanged(mojom::blink::FrameVisibility) = 0;

 protected:
  explicit FrameVisibilityObserver(LocalFrame*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VISIBILITY_OBSERVER_H_
