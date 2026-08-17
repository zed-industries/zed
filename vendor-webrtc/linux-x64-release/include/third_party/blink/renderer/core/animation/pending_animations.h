/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PENDING_ANIMATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PENDING_ANIMATIONS_H_

#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class PaintArtifactCompositor;

// Handles starting animations when they could potentially require
// interaction with the compositor. This can include both main-thread
// and compositor thread animations. For example, when the Document
// changes visibility state, all animations for the document's
// timeline are set to "compositor pending" which will include them in
// a consideration pass here.
//
// Manages the starting of pending animations on the compositor following a
// compositing update.
//
// For CSS Animations, used to synchronize the start of main-thread animations
// with compositor animations when both classes of CSS Animations are triggered
// by the same recalc.
class CORE_EXPORT PendingAnimations final
    : public GarbageCollected<PendingAnimations> {
 public:
  explicit PendingAnimations(Document& document)
      : timer_(document.GetTaskRunner(TaskType::kInternalDefault),
               this,
               &PendingAnimations::TimerFired),
        compositor_group_(1),
        inside_timer_fired_(false) {}

  void Add(Animation*);

  // Attempts to start/update pending composited and non-composited animations.
  // At the end of this process all pending animations will fall into one of the
  // following buckets:
  // - pending: already composited animations that cannot be restarted this
  //   cycle and are deferred to be tried next cycle.
  // - started on compositor: animations that could be composited and are
  //   successfully started on compositor.
  // - waiting on start time: animations whose start time needs to be
  //   synchronized with compositor. These may include non-composited
  //   animations.
  // - notified of start time: animations whose start time does not need to be
  //   synchronized with compositor and thus can be immediately notified.
  // - ignored: animations that are not started and don't need to be notified
  //   are simply ignored. This allows the rest of animation machinery to add
  //   animations to the pending list eagerly knowing that the logic here
  //   ignores them if no action needs to be taken.
  //
  // Returns whether we are waiting for an animation to start and should service
  // again on the next frame.
  bool Update(const PaintArtifactCompositor* paint_artifact_compositor,
              bool start_on_compositor = true);
  void NotifyCompositorAnimationStarted(double monotonic_animation_start_time,
                                        int compositor_group = 0);

  void Trace(Visitor*) const;

 private:
  void TimerFired(TimerBase*);
  int NextCompositorGroup();
  void FlushWaitingNonCompositedAnimations();

  HeapVector<Member<Animation>> pending_;
  HeapVector<Member<Animation>> waiting_for_compositor_animation_start_;
  HeapTaskRunnerTimer<PendingAnimations> timer_;
  int compositor_group_;
  bool inside_timer_fired_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PENDING_ANIMATIONS_H_
