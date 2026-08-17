// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_ANCHOR_ELEMENT_VIEWPORT_POSITION_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_ANCHOR_ELEMENT_VIEWPORT_POSITION_TRACKER_H_

#include "third_party/blink/public/mojom/loader/navigation_predictor.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/local_frame_view.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Document;
class HTMLAnchorElementBase;
class IntersectionObserver;
class IntersectionObserverEntry;
class PointerEvent;

// This class uses an IntersectionObserver to track the position of <a> and
// <area> elements in the viewport. It notifies observers of when observed
// anchors enter/leave the viewport; and computes and reports their positions
// after a scroll completes.
class CORE_EXPORT AnchorElementViewportPositionTracker
    : public GarbageCollected<AnchorElementViewportPositionTracker>,
      public LocalFrameView::LifecycleNotificationObserver,
      public Supplement<Document> {
 public:
  class Observer : public GarbageCollectedMixin {
   public:
    // Called after an IntersectionObserver update to report anchors that
    // have entered/left the viewport.
    virtual void ViewportIntersectionUpdate(
        const HeapVector<Member<const HTMLAnchorElementBase>>& entered_viewport,
        const HeapVector<Member<const HTMLAnchorElementBase>>& left_viewport) {}

    struct AnchorPositionUpdate
        : public GarbageCollected<AnchorPositionUpdate> {
      Member<const HTMLAnchorElementBase> anchor_element;
      // The vertical position of the `anchor_element`'s center in the viewport
      // (as a ratio of the viewport height).
      float vertical_position;
      // The vertical distance between `anchor_element`'s center and the last
      // recorded pointer down (as a ratio of the screen height).
      std::optional<float> distance_from_pointer_down;
      // Visible area (in viewport coordinates) of `anchor_element`.
      int size_in_viewport;

      void Trace(Visitor* visitor) const;
    };
    // Called asynchronously after a scroll ends to report metrics related to
    // the anchor's position in the viewport. `position_updates` will have an
    // entry for every observed anchor that's currently in the viewport.
    virtual void AnchorPositionsUpdated(
        HeapVector<Member<AnchorPositionUpdate>>& position_updates) {}
  };

  static const char kSupplementName[];

  explicit AnchorElementViewportPositionTracker(Document&);
  AnchorElementViewportPositionTracker(
      const AnchorElementViewportPositionTracker&) = delete;
  AnchorElementViewportPositionTracker& operator=(
      const AnchorElementViewportPositionTracker&) = delete;
  virtual ~AnchorElementViewportPositionTracker();

  void Trace(Visitor*) const override;

  // Returns an AnchorElementViewportPositionTracker associated with a document
  // (creating one if necessary). Documents that are detached, or not in the
  // outermost main frame, will not have a tracker, and this will return
  // nullptr.
  static AnchorElementViewportPositionTracker* MaybeGetOrCreateFor(Document&);
  void AddObserver(Observer* observer);

  // Called to start observing a new anchor. The anchor may not be observed if
  // the number of anchors currently being observed is
  // `max_number_of_observations`. If a previously observed anchor is
  // unobserved, the unobserved anchor is returned, otherwise this method
  // returns nullptr.
  HTMLAnchorElementBase* MaybeObserveAnchor(
      HTMLAnchorElementBase& anchor,
      const mojom::blink::AnchorElementMetrics& metrics);
  // Called when an anchor is removed from the document.
  void RemoveAnchor(HTMLAnchorElementBase& anchor);
  // Called when a pointerdown is about to be dispatched to any Node in |this|'s
  // document or local subframes. Record the location of the pointer event for
  // future position metrics computation.
  void RecordPointerDown(const PointerEvent& pointer_event);
  // Called when a scroll completes. Triggers computation of position related
  // metrics for all observed anchors that are currently in the viewport.
  void OnScrollEnd();
  void OnFirstContentfulPaint();

  IntersectionObserver* GetIntersectionObserverForTesting();

 private:
  void UpdateVisibleAnchors(
      const HeapVector<Member<IntersectionObserverEntry>>& entries);
  void PositionUpdateTimerFired(TimerBase*);
  void DidFinishLifecycleUpdate(
      const LocalFrameView& local_frame_view) override;
  void DispatchAnchorElementsPositionUpdates();
  void RegisterForLifecycleNotifications();
  void InitializeIntersectionObserver();
  void PostFCPDelayTimerFired(TimerBase*);

  Member<IntersectionObserver> intersection_observer_;
  // Maximum number of observations for `intersection_observer_`.
  const wtf_size_t max_number_of_observations_;
  // Delay for `intersection_observer_`. Also the timeout value used for
  // `position_update_timer_`.
  const base::TimeDelta intersection_observer_delay_;

  // These two sets, together, contain the anchors sampled in to be observed
  // by `intersection_observer_, ordered by their priority (currently,
  // `ratio_area`).
  //
  // The top `max_number_of_observations_` entries are observed at any one
  // time (and exist in `observed_anchors_`).
  //
  //  non_observed_anchors_     observed_anchors_ (size capped)
  // +-----------------------+ +-------------------------------+
  // | .1 A1 | .2 A2 | .3 A3 | | .4 A4 | .5 A5 | .6 A6 | .7 A7 |
  // +-----------------------+ +-------------------------------+
  //
  // If an anchor is added, the first element of `observed_anchors_`
  // might be moved to `non_observed_anchors_` to make room.
  // If an observed anchor is removed, the last element of
  // `non_observed_anchors_` is moved to `observed_anchors_`
  struct AnchorObservation {
    // mojom::blink::AnchorElementMetrics::ratio_area * 100 (see documentation
    // in navigation_predictor.mojom).
    int percent_area;
    // DOMNodeId for the anchor this AnchorObservation is created for.
    DOMNodeId dom_node_id;

    bool operator==(const AnchorObservation&) const = default;
    auto operator<=>(const AnchorObservation&) const = default;
  };
  std::set<AnchorObservation> observed_anchors_
      ALLOW_DISCOURAGED_TYPE("WTF::HashSet lacks key sorting.");
  std::set<AnchorObservation> not_observed_anchors_
      ALLOW_DISCOURAGED_TYPE("WTF::HashSet lacks key sorting.");

  // Observed anchors that are currently in the viewport.
  HeapHashSet<WeakMember<const HTMLAnchorElementBase>> anchors_in_viewport_;

  // Indicates that we have registered for a lifecycle update notification.
  bool is_registered_for_lifecycle_notifications_ = false;

  // The y-coordinate of the last pointerdown (in the visual viewport coordinate
  // space and offset by the height of the browser top-controls), reported in
  // `RecordPointerDown`. Used to compute
  // `AnchorPositionUpdate::distance_from_pointer_down`.
  std::optional<float> last_pointer_down_ = std::nullopt;

  // Used to timeout waiting for an intersection observer update
  // (`UpdateVisibleAnchors`) after `OnScrollEnd` is called. The timer is
  // stopped when `UpdateVisibleAnchors` is called.
  HeapTaskRunnerTimer<AnchorElementViewportPositionTracker>
      position_update_timer_;

  // Used to wait for a period of time after OnFirstContentfulPaint() is called,
  // (to approximate waiting for LCP) before initializing
  // |intersection_observer_|.
  HeapTaskRunnerTimer<AnchorElementViewportPositionTracker>
      post_fcp_delay_timer_;

  HeapHashSet<WeakMember<Observer>> observers_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_ANCHOR_ELEMENT_VIEWPORT_POSITION_TRACKER_H_
