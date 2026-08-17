// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_ANIMATOR_COMPOSITOR_COORDINATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_ANIMATOR_COMPOSITOR_COORDINATOR_H_

#include <memory>
#include "base/gtest_prod_util.h"
#include "cc/animation/keyframe_model.h"
#include "cc/animation/scroll_offset_animations.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation_client.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation_delegate.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/animation/keyframe/animation_curve.h"

namespace cc {
class AnimationTimeline;
}

namespace blink {

class ScrollableArea;
class CompositorAnimation;

// ScrollAnimatorCompositorCoordinator is the common base class of user scroll
// animators and programmatic scroll animators, and holds logic related to
// scheduling and updating scroll animations running on the compositor.
//
// See ScrollAnimator.h for more information about scroll animations.

class CORE_EXPORT ScrollAnimatorCompositorCoordinator
    : public GarbageCollected<ScrollAnimatorCompositorCoordinator>,
      private CompositorAnimationClient,
      CompositorAnimationDelegate {
  USING_PRE_FINALIZER(ScrollAnimatorCompositorCoordinator, Dispose);

 public:
  enum class RunState {
    // No animation.
    kIdle,

    // Waiting to send an animation to the compositor. There might also
    // already be another animation running on the compositor that will need
    // to be canceled first.
    kWaitingToSendToCompositor,

    // Running an animation on the compositor.
    kRunningOnCompositor,

    // Running an animation on the compositor but needs update.
    kRunningOnCompositorButNeedsUpdate,

    // Running an animation on the main thread.
    kRunningOnMainThread,

    // Waiting to cancel the animation currently running on the compositor.
    // There is no pending animation to replace the canceled animation.
    kWaitingToCancelOnCompositor,

    // Finished an animation that was running on the main thread or the
    // compositor thread. When in this state, post animation cleanup can
    // be performed.
    kPostAnimationCleanup,

    // Running an animation on the compositor but need to continue it
    // on the main thread. This could happen if a main thread scrolling
    // reason is added while animating the scroll offset.
    kRunningOnCompositorButNeedsTakeover,

    // Waiting to cancel the animation currently running on the compositor
    // while another animation is requested. In this case, the currently
    // running animation is aborted and an animation to the new target
    // from the current offset is started.
    kWaitingToCancelOnCompositorButNewScroll,

    // Running an animation on the compositor but an adjustment to the
    // scroll offset was made on the main thread and the animation must
    // be updated.
    kRunningOnCompositorButNeedsAdjustment,
  };

  ScrollAnimatorCompositorCoordinator(
      const ScrollAnimatorCompositorCoordinator&) = delete;
  ScrollAnimatorCompositorCoordinator& operator=(
      const ScrollAnimatorCompositorCoordinator&) = delete;
  ~ScrollAnimatorCompositorCoordinator() override;

  bool HasAnimationThatRequiresService() const;
  void Dispose();
  String RunStateAsText() const;

  void DetachElement();

  virtual void ResetAnimationState();
  virtual void CancelAnimation();
  // Aborts the currently running scroll offset animation on the compositor
  // and continues it on the main thread. This should only be called when in
  // DocumentLifecycle::LifecycleState::CompositingClean state.
  virtual void TakeOverCompositorAnimation();
  virtual void UpdateCompositorAnimations();

  virtual ScrollableArea* GetScrollableArea() const = 0;
  virtual void TickAnimation(base::TimeTicks monotonic_time) = 0;
  virtual void NotifyCompositorAnimationFinished(int group_id) = 0;
  virtual void NotifyCompositorAnimationAborted(int group_id) = 0;

  RunState RunStateForTesting() { return run_state_; }

  bool HasRunningAnimation() const {
    return run_state_ != RunState::kPostAnimationCleanup &&
           (animation_curve_ ||
            run_state_ == RunState::kWaitingToSendToCompositor);
  }

  virtual void Trace(Visitor* visitor) const {}

 protected:
  explicit ScrollAnimatorCompositorCoordinator();

  void ScrollOffsetChanged(const ScrollOffset&, mojom::blink::ScrollType);

  void AdjustImplOnlyScrollOffsetAnimation(const gfx::Vector2d& adjustment);
  gfx::Vector2d ImplOnlyAnimationAdjustmentForTesting() {
    return impl_only_animation_adjustment_;
  }

  bool AddAnimation(std::unique_ptr<cc::KeyframeModel>);
  void RemoveAnimation();
  virtual void AbortAnimation();

  // "offset" in the cc scrolling code is analagous to "position" in the blink
  // scrolling code:
  // they both represent the distance from the top-left of the overflow rect to
  // the top-left
  // of the viewport.  In blink, "offset" refers to the distance of the viewport
  // from the
  // beginning of flow of the contents.  In left-to-right flows, blink "offset"
  // and "position" are
  // equivalent, but in right-to-left flows (including direction:rtl,
  // writing-mode:vertical-rl,
  // and flex-direction:row-reverse), they aren't.  See core/layout/README.md
  // for more info.
  gfx::PointF CompositorOffsetFromBlinkOffset(ScrollOffset);
  ScrollOffset BlinkOffsetFromCompositorOffset(gfx::PointF);

  void CompositorAnimationFinished(int group_id);
  // Returns true if the compositor animation was attached to a new layer.
  bool ReattachCompositorAnimationIfNeeded(cc::AnimationTimeline*);

  // CompositorAnimationDelegate implementation.
  void NotifyAnimationStarted(base::TimeDelta monotonic_time,
                              int group) override;
  void NotifyAnimationFinished(base::TimeDelta monotonic_time,
                               int group) override;
  void NotifyAnimationAborted(base::TimeDelta monotonic_time,
                              int group) override;
  void NotifyAnimationTakeover(double monotonic_time,
                               double animation_start_time,
                               std::unique_ptr<gfx::AnimationCurve>) override {}

  // CompositorAnimationClient implementation.
  CompositorAnimation* GetCompositorAnimation() const override;

  friend class Internals;
  friend class TestScrollAnimator;
  // TODO(ymalik): Tests are added as friends to access m_RunState. Use the
  // runStateForTesting accessor instead.
  FRIEND_TEST_ALL_PREFIXES(ScrollAnimatorTest, MainThreadStates);
  FRIEND_TEST_ALL_PREFIXES(ScrollAnimatorTest, AnimatedScrollTakeover);
  FRIEND_TEST_ALL_PREFIXES(ScrollAnimatorTest, CancellingAnimationResetsState);
  FRIEND_TEST_ALL_PREFIXES(ScrollAnimatorTest, CancellingCompositorAnimation);
  FRIEND_TEST_ALL_PREFIXES(ScrollAnimatorTest, ImplOnlyAnimationUpdatesCleared);
  FRIEND_TEST_ALL_PREFIXES(ScrollAnimatorTest,
                           UserScrollCallBackAtAnimationFinishOnMainThread);
  FRIEND_TEST_ALL_PREFIXES(ScrollAnimatorTest,
                           UserScrollCallBackAtAnimationFinishOnCompositor);
  FRIEND_TEST_ALL_PREFIXES(ScrollAnchorTest, ClampAdjustsAnchorAnimation);
  // TODO(crbug.com/1313270): Remove this when ScrollAnchorTest runs on Fuchsia.
  FRIEND_TEST_ALL_PREFIXES(DISABLED_ScrollAnchorTest,
                           ClampAdjustsAnchorAnimation);

  std::unique_ptr<CompositorAnimation> compositor_animation_;
  // The element id to which the compositor animation is attached when
  // the animation is present.
  CompositorElementId element_id_;
  RunState run_state_;
  int compositor_animation_id() const { return compositor_animation_id_; }

  // An adjustment to the scroll offset on the main thread that may affect
  // impl-only scroll offset animations.
  gfx::Vector2d impl_only_animation_adjustment_;

  // If set to true, sends a cc::ScrollOffsetAnimationUpdate to cc which will
  // abort the impl-only scroll offset animation and continue it on main
  // thread.
  bool impl_only_animation_takeover_;

  std::unique_ptr<cc::ScrollOffsetAnimationCurve> animation_curve_;

 private:
  bool element_detached_ = false;

  CompositorElementId GetScrollElementId() const;
  bool HasImplOnlyAnimationUpdate() const;
  void UpdateImplOnlyCompositorAnimations();
  // Accesses compositing state and should only be called when in or after
  // DocumentLifecycle::LifecycleState::CompositingClean.
  void TakeOverImplOnlyScrollOffsetAnimation();

  int compositor_animation_id_;
  int compositor_animation_group_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_ANIMATOR_COMPOSITOR_COORDINATOR_H_
