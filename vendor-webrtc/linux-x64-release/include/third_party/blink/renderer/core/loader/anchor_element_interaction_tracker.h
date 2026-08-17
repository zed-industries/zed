// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_ANCHOR_ELEMENT_INTERACTION_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_ANCHOR_ELEMENT_INTERACTION_TRACKER_H_

#include "base/metrics/field_trial_params.h"
#include "third_party/blink/public/mojom/preloading/anchor_element_interaction_host.mojom-blink.h"
#include "third_party/blink/renderer/core/html/anchor_element_viewport_position_tracker.h"
#include "third_party/blink/renderer/core/html/html_anchor_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace blink {

class Document;
class EventTarget;
class HTMLAnchorElementBase;
class KURL;
class MouseEvent;
class Node;
class PointerEvent;

CORE_EXPORT BASE_DECLARE_FEATURE(kNoSamePageFragmentPreloadingAnchorTracking);

// Tracks pointerdown events anywhere on a document.  On receiving a pointerdown
// event, the tracker will retrieve the valid href from the anchor element from
// the event and will report the href value to the browser process via Mojo. The
// browser process can use this information to preload (e.g. preconnect to the
// origin) the URL in order to improve performance.
class BLINK_EXPORT AnchorElementInteractionTracker
    : public GarbageCollected<AnchorElementInteractionTracker>,
      public AnchorElementViewportPositionTracker::Observer {
 public:
  class BLINK_EXPORT MouseMotionEstimator
      : public GarbageCollected<MouseMotionEstimator> {
   public:
    MouseMotionEstimator() = delete;
    explicit MouseMotionEstimator(
        scoped_refptr<base::SingleThreadTaskRunner> task_runner);
    ~MouseMotionEstimator() = default;

    void Trace(Visitor* visitor) const;
    void OnTimer(TimerBase*);
    void OnMouseMoveEvent(gfx::PointF position);
    void SetTaskRunnerForTesting(
        scoped_refptr<base::SingleThreadTaskRunner> task_runner,
        const base::TickClock* clock);
    bool IsEmpty() { return mouse_position_and_timestamps_.empty(); }

    void SetMouseAccelerationForTesting(gfx::Vector2dF acceleration) {
      acceleration_ = acceleration;
    }
    void SetMouseVelocityForTesting(gfx::Vector2dF velocity) {
      velocity_ = velocity;
    }
    gfx::Vector2dF GetMouseAcceleration() const { return acceleration_; }
    gfx::Vector2dF GetMouseVelocity() const { return velocity_; }
    double GetMouseTangentialAcceleration() const;

   private:
    void AddDataPoint(base::TimeTicks timestamp, gfx::PointF position);
    void RemoveOldDataPoints(base::TimeTicks now);
    void Update();

    struct MousePositionAndTimeStamp {
      gfx::PointF position;
      base::TimeTicks ts;
    };
    // Mouse acceleration in (pixels/second**2).
    gfx::Vector2dF acceleration_;
    // Mouse velocity in (pixels/second).
    gfx::Vector2dF velocity_;
    WTF::Deque<MousePositionAndTimeStamp> mouse_position_and_timestamps_;
    HeapTaskRunnerTimer<AnchorElementInteractionTracker::MouseMotionEstimator>
        update_timer_;
    const base::TickClock* clock_;
  };

  explicit AnchorElementInteractionTracker(Document& document);
  virtual ~AnchorElementInteractionTracker();

  static base::TimeDelta GetHoverDwellTime();

  void OnMouseMoveEvent(const WebMouseEvent& mouse_event);
  void OnPointerEvent(EventTarget& target, const PointerEvent& pointer_event);
  void OnClickEvent(HTMLAnchorElementBase& anchor,
                    const MouseEvent& click_event);

  void HoverTimerFired(TimerBase*);
  void Trace(Visitor* visitor) const override;
  void SetTaskRunnerForTesting(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      const base::TickClock* clock);
  Document* GetDocument() { return document_.Get(); }

 private:
  HTMLAnchorElementBase* FirstAnchorElementIncludingSelf(Node* node);

  // Gets the `anchor's` href attribute if it is part
  // of the HTTP family
  KURL GetHrefEligibleForPreloading(const HTMLAnchorElementBase& anchor);
  void ViewportHeuristicTimerFired(TimerBase*);

  // AnchorElementViewportPositionTracker::Observer overrides
  void AnchorPositionsUpdated(
      HeapVector<Member<AnchorPositionUpdate>>& position_updates) override;

  Member<MouseMotionEstimator> mouse_motion_estimator_;
  HeapMojoRemote<mojom::blink::AnchorElementInteractionHost> interaction_host_;
  // This hashmap contains the anchor element's url, whether the pointer event
  // was from a mouse and the timetick at which a hover event should be reported
  // if not canceled.
  struct HoverEventCandidate {
    bool is_mouse;
    uint32_t anchor_id;
    base::TimeTicks timestamp;
  };
  HashMap<KURL, HoverEventCandidate> hover_event_candidates_;
  HeapTaskRunnerTimer<AnchorElementInteractionTracker> hover_timer_;
  const base::TickClock* clock_;
  Member<Document> document_;
  // Stores y-coordinate of the two most recent pointerdowns (first entry is
  // the most recent pointer down).
  std::array<std::optional<double>, 2> last_pointer_down_locations_ = {
      std::nullopt, std::nullopt};
  // Stores the largest anchor in the viewport determined by the viewport
  // heuristic.
  WeakMember<HTMLAnchorElementBase> largest_anchor_element_in_viewport_;
  HeapTaskRunnerTimer<AnchorElementInteractionTracker>
      viewport_heuristic_timer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_ANCHOR_ELEMENT_INTERACTION_TRACKER_H_
