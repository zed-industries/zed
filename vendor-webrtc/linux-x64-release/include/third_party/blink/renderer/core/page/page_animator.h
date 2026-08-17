// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_ANIMATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_ANIMATOR_H_

#include "third_party/blink/public/common/metrics/document_update_reason.h"
#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/animation/animation_clock.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document_lifecycle.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace cc {
class AnimationHost;
}

namespace blink {

class LocalFrame;
class Page;
class TreeScope;
class ScriptedAnimationController;

using DocumentsVector = HeapVector<std::pair<Member<Document>, bool>>;
using ControllersVector =
    HeapVector<std::pair<Member<ScriptedAnimationController>, bool>>;

class CORE_EXPORT PageAnimator final : public GarbageCollected<PageAnimator> {
 public:
  explicit PageAnimator(Page&);

  void Trace(Visitor*) const;
  void ScheduleVisualUpdate(LocalFrame*);
  void ServiceScriptedAnimations(
      base::TimeTicks monotonic_animation_start_time);
  // Invokes callbacks, dispatches events, etc. The order is defined by HTML:
  // https://html.spec.whatwg.org/C/#event-loop-processing-model
  static void ServiceScriptedAnimations(
      base::TimeTicks monotonic_time_now,
      const ControllersVector& documents_vector);
  void PostAnimate();

  bool IsServicingAnimations() const { return servicing_animations_; }

  // TODO(alancutter): Remove the need for this by implementing frame request
  // suppression logic at the BeginMainFrame level. This is a temporary
  // workaround to fix a perf regression.
  // DO NOT use this outside of crbug.com/704763.
  void SetSuppressFrameRequestsWorkaroundFor704763Only(bool);

  // See documents of methods with the same names in LocalFrameView class.
  void UpdateAllLifecyclePhases(LocalFrame& root_frame,
                                DocumentUpdateReason reason);
  void UpdateAllLifecyclePhasesExceptPaint(LocalFrame& root_frame,
                                           DocumentUpdateReason reason);
  void UpdateLifecycleToLayoutClean(LocalFrame& root_frame,
                                    DocumentUpdateReason reason);
  AnimationClock& Clock() { return animation_clock_; }
  HeapVector<Member<Animation>> GetAnimations(const TreeScope&);
  void SetHasCanvasInvalidation();
  bool has_canvas_invalidation_for_test() const {
    return has_canvas_invalidation_;
  }
  void SetHasInlineStyleMutation();
  bool has_inline_style_mutation_for_test() const {
    return has_inline_style_mutation_;
  }
  void SetHasSmilAnimation();
  void SetCurrentFrameHadRaf();
  void SetNextFrameHasPendingRaf();
  void SetHasViewTransition(bool);
  void ReportFrameAnimations(cc::AnimationHost* animation_host);

 private:
  Member<Page> page_;
  bool servicing_animations_;
  bool updating_layout_and_style_for_painting_;
  bool suppress_frame_requests_workaround_for704763_only_ = false;
  AnimationClock animation_clock_;

  // True if there is inline style mutation in the current frame.
  bool has_inline_style_mutation_ = false;
  // True if the current main frame has canvas invalidation.
  bool has_canvas_invalidation_ = false;
  // True if the current main frame has svg smil animation.
  bool has_smil_animation_ = false;
  // True if there is a raf scheduled in this frame.
  bool current_frame_had_raf_ = false;
  // True if there is a raf scheduled for the next frame.
  bool next_frame_has_pending_raf_ = false;
  // True if there is an ongoing view transition.
  bool has_view_transition_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PAGE_ANIMATOR_H_
