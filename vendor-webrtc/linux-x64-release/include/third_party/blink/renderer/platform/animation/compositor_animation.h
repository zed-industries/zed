// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_ANIMATION_COMPOSITOR_ANIMATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_ANIMATION_COMPOSITOR_ANIMATION_H_

#include <memory>
#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "cc/animation/animation.h"
#include "cc/animation/animation_delegate.h"
#include "cc/animation/worklet_animation.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace gfx {
class AnimationCurve;
}

namespace blink {

class CompositorAnimationDelegate;

// A compositor representation for Animation.
class PLATFORM_EXPORT CompositorAnimation : public cc::AnimationDelegate {
 public:
  // If this CompositorAnimation is being created to replace an
  // existing cc::Animation, the existing Animation's id should be
  // passed in to ensure the same id is used.
  static std::unique_ptr<CompositorAnimation> Create(
      std::optional<int> replaced_cc_animation_id = std::nullopt);
  static std::unique_ptr<CompositorAnimation> CreateWorkletAnimation(
      cc::WorkletAnimationId,
      const String& name,
      double playback_rate,
      std::unique_ptr<cc::AnimationOptions>,
      std::unique_ptr<cc::AnimationEffectTimings> effect_timings);

  explicit CompositorAnimation(scoped_refptr<cc::Animation>);
  CompositorAnimation(const CompositorAnimation&) = delete;
  CompositorAnimation& operator=(const CompositorAnimation&) = delete;
  ~CompositorAnimation() override;

  cc::Animation* CcAnimation() const;
  int CcAnimationId() const;

  // An animation delegate is notified when animations are started and stopped.
  // The CompositorAnimation does not take ownership of the delegate, and
  // it is the responsibility of the client to reset the layer's delegate before
  // deleting the delegate.
  void SetAnimationDelegate(CompositorAnimationDelegate*);

  void AttachElement(const CompositorElementId&);
  void AttachPaintWorkletElement();
  void DetachElement();
  bool IsElementAttached() const;

  void AddKeyframeModel(std::unique_ptr<cc::KeyframeModel>);
  void RemoveKeyframeModel(int keyframe_model_id);
  void PauseKeyframeModel(int keyframe_model_id, base::TimeDelta time_offset);
  void AbortKeyframeModel(int keyframe_model_id);

  void UpdatePlaybackRate(double playback_rate);

 private:
  // cc::AnimationDelegate implementation.
  void NotifyAnimationStarted(base::TimeTicks monotonic_time,
                              int target_property,
                              int group) override;
  void NotifyAnimationFinished(base::TimeTicks monotonic_time,
                               int target_property,
                               int group) override;
  void NotifyAnimationAborted(base::TimeTicks monotonic_time,
                              int target_property,
                              int group) override;
  void NotifyAnimationTakeover(base::TimeTicks monotonic_time,
                               int target_property,
                               base::TimeTicks animation_start_time,
                               std::unique_ptr<gfx::AnimationCurve>) override;
  void NotifyLocalTimeUpdated(
      std::optional<base::TimeDelta> local_time) override;

  scoped_refptr<cc::Animation> animation_;
  raw_ptr<CompositorAnimationDelegate> delegate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_ANIMATION_COMPOSITOR_ANIMATION_H_
