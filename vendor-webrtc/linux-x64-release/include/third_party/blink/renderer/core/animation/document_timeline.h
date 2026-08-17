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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_DOCUMENT_TIMELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_DOCUMENT_TIMELINE_H_

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/core/animation/animation_timeline.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class DocumentTimelineOptions;

// DocumentTimeline is constructed and owned by Document, and tied to its
// lifecycle.
class CORE_EXPORT DocumentTimeline : public AnimationTimeline {
  DEFINE_WRAPPERTYPEINFO();

 public:
  class PlatformTiming : public GarbageCollected<PlatformTiming> {
   public:
    // Calls DocumentTimeline's wake() method after duration seconds.
    virtual void WakeAfter(base::TimeDelta duration) = 0;
    virtual ~PlatformTiming() = default;
    virtual void Trace(Visitor* visitor) const {}
  };

  // Web Animations API IDL constructor
  static DocumentTimeline* Create(ExecutionContext*,
                                  const DocumentTimelineOptions*);

  DocumentTimeline(Document*,
                   base::TimeDelta origin_time = base::TimeDelta(),
                   PlatformTiming* = nullptr);
  ~DocumentTimeline() override = default;

  bool IsDocumentTimeline() const final { return true; }

  void ScheduleNextService() override;

  bool IsActive() const override;
  std::optional<base::TimeDelta> InitialStartTimeForAnimations() override;
  bool HasPendingUpdates() const { return !animations_needing_update_.empty(); }

  // The zero time of DocumentTimeline is computed by adding a separate
  // |origin_time_| from DocumentTimelineOptions.
  // https://w3.org/TR/web-animations-1/#origin-time
  // TODO(crbug.com/1162960) Convert DocumentTimeline::zero_time_ from
  // base::TimeTicks to AnimationTimeDelta
  base::TimeTicks CalculateZeroTime();
  AnimationTimeDelta ZeroTime() override {
    return AnimationTimeDelta(CalculateZeroTime().since_origin());
  }

  void PauseAnimationsForTesting(AnimationTimeDelta);

  void InvalidateKeyframeEffects(const TreeScope&,
                                 const StyleChangeReasonForTracing&);

  void SetPlaybackRate(double);
  double PlaybackRate() const;

  void ResetForTesting();
  void SetTimingForTesting(PlatformTiming* timing);

  cc::AnimationTimeline* EnsureCompositorTimeline() override;

  void Trace(Visitor*) const override;

 protected:
  PhaseAndTime CurrentPhaseAndTime() override;

 private:
  // Origin time for the timeline relative to the time origin of the document.
  // Provided when the timeline is constructed. See
  // https://w3.org/TR/web-animations-1/#dom-documenttimelineoptions-origintime.
  base::TimeDelta origin_time_;
  // The origin time. This is computed by adding |origin_time_| to the time
  // origin of the document.
  // TODO(crbug.com/1162960) Convert DocumentTimeline::zero_time_ from
  // base::TimeTicks to AnimationTimeDelta
  base::TimeTicks zero_time_;

  double playback_rate_;

  friend class SMILTimeContainer;
  static const double kMinimumDelay;

  Member<PlatformTiming> timing_;

  bool zero_time_initialized_;

  class DocumentTimelineTiming final : public PlatformTiming {
   public:
    explicit DocumentTimelineTiming(DocumentTimeline* timeline)
        : timeline_(timeline),
          timer_(timeline->GetDocument()->GetTaskRunner(
                     TaskType::kInternalDefault),
                 this,
                 &DocumentTimelineTiming::TimerFired) {
      DCHECK(timeline_);
    }

    void WakeAfter(base::TimeDelta duration) override;

    void TimerFired(TimerBase*) { timeline_->ScheduleServiceOnNextFrame(); }

    void Trace(Visitor*) const override;

   private:
    Member<DocumentTimeline> timeline_;
    HeapTaskRunnerTimer<DocumentTimelineTiming> timer_;
  };

  friend class AnimationDocumentTimelineTest;
};

template <>
struct DowncastTraits<DocumentTimeline> {
  static bool AllowFrom(const AnimationTimeline& timeline) {
    return timeline.IsDocumentTimeline();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_DOCUMENT_TIMELINE_H_
