// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_ANIMATION_COMPOSITOR_ANIMATION_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_ANIMATION_COMPOSITOR_ANIMATION_DELEGATE_H_

#include <memory>
#include <optional>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/animation/keyframe/animation_curve.h"

namespace blink {

class PLATFORM_EXPORT CompositorAnimationDelegate {
 public:
  virtual ~CompositorAnimationDelegate() = default;

  virtual void NotifyAnimationStarted(base::TimeDelta monotonic_time,
                                      int group) = 0;
  virtual void NotifyAnimationFinished(base::TimeDelta monotonic_time,
                                       int group) = 0;
  virtual void NotifyAnimationAborted(base::TimeDelta monotonic_time,
                                      int group) = 0;
  // In the current state of things, notifyAnimationTakeover only applies to
  // scroll offset animations since main thread scrolling reasons can be added
  // while the compositor is animating. Keeping this non-pure virtual since
  // it doesn't apply to CSS animations.
  virtual void NotifyAnimationTakeover(
      double monotonic_time,
      double animation_start_time,
      std::unique_ptr<gfx::AnimationCurve> curve) {}
  virtual void NotifyLocalTimeUpdated(
      std::optional<base::TimeDelta> local_time) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_ANIMATION_COMPOSITOR_ANIMATION_DELEGATE_H_
