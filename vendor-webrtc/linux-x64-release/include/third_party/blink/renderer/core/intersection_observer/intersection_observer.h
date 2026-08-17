// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_H_

#include <optional>

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/local_frame_ukm_aggregator.h"
#include "third_party/blink/renderer/core/intersection_observer/intersection_observation.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Document;
class Element;
class ExceptionState;
class IntersectionObserverDelegate;
class IntersectionObserverEntry;
class IntersectionObserverInit;
class Node;
class ScriptState;
class V8IntersectionObserverCallback;

class CORE_EXPORT IntersectionObserver final
    : public ScriptWrappable,
      public ActiveScriptWrappable<IntersectionObserver>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using EventCallback = base::RepeatingCallback<void(
      const HeapVector<Member<IntersectionObserverEntry>>&)>;

  // The IntersectionObserver can be configured to notify based on changes to
  // how much of the target element's area intersects with the root, or based on
  // changes to how much of the root element's area intersects with the
  // target. Examples illustrating the distinction:
  //
  //     1.0 of target,         0.5 of target,         1.0 of target,
  //      0.25 of root           0.5 of root            1.0 of root
  //  +------------------+   +------------------+   *~~~~~~~~~~~~~~~~~~*
  //  |   //////////     |   |                  |   ;//////////////////;
  //  |   //////////     |   |                  |   ;//////////////////;
  //  |   //////////     |   ;//////////////////;   ;//////////////////;
  //  |                  |   ;//////////////////;   ;//////////////////;
  //  +------------------+   *~~~~~~~~~~~~~~~~~~*   *~~~~~~~~~~~~~~~~~~*
  //                         ////////////////////
  //                         ////////////////////
  //                         ////////////////////
  enum ThresholdInterpretation { kFractionOfTarget, kFractionOfRoot };

  // This value can be used to detect transitions between non-intersecting or
  // edge-adjacent (i.e., zero area) state, and intersecting by any non-zero
  // number of pixels.
  static constexpr float kMinimumThreshold =
      IntersectionGeometry::kMinimumThreshold;

  // Used to specify when callbacks should be invoked with new notifications.
  // Blink-internal users of IntersectionObserver will have their callbacks
  // invoked synchronously either at the end of a lifecycle update or in the
  // middle of the lifecycle post layout. Javascript observers will PostTask to
  // invoke their callbacks.
  enum DeliveryBehavior {
    kDeliverDuringPostLayoutSteps,
    kDeliverDuringPostLifecycleSteps,
    kPostTaskToDeliver
  };

  // Used to specify whether the margins apply to the root element or the source
  // element. The effect of the root element margins is that intermediate
  // scrollers clip content by its bounding box without considering margins.
  // That is, margins only apply to the last scroller (root). The effect of
  // source element margins is that the margins apply to the first / deepest
  // clipper, but do not apply to any other clippers. Note that in a case of a
  // single clipper, the two approaches are equivalent.
  //
  // Note that the percentage margin is resolved against the root rect, even
  // when the margin is applied to the target.
  enum MarginTarget { kApplyMarginToRoot, kApplyMarginToTarget };

  static IntersectionObserver* Create(
      const IntersectionObserverInit*,
      IntersectionObserverDelegate&,
      std::optional<LocalFrameUkmAggregator::MetricId> ukm_metric_id,
      ExceptionState& = ASSERT_NO_EXCEPTION);
  static IntersectionObserver* Create(ScriptState*,
                                      V8IntersectionObserverCallback*,
                                      const IntersectionObserverInit*,
                                      ExceptionState& = ASSERT_NO_EXCEPTION);

  struct Params {
    STACK_ALLOCATED();

   public:
    Node* root;
    Vector<Length> margin;
    MarginTarget margin_target = kApplyMarginToRoot;
    Vector<Length> scroll_margin;

    // Elements should be in the range [0,1], and are interpreted according to
    // the given `semantics`.
    Vector<float> thresholds;
    ThresholdInterpretation semantics = kFractionOfTarget;

    DeliveryBehavior behavior = kDeliverDuringPostLifecycleSteps;
    // Specifies the minimum period between change notifications.
    base::TimeDelta delay;
    bool track_visibility = false;
    bool always_report_root_bounds = false;
    // Indicates whether the overflow clip edge should be used instead of the
    // bounding box if appropriate.
    bool use_overflow_clip_edge = false;
    bool needs_initial_observation_with_detached_target = true;

    // Indicates whether we should compute and expose the occluder node Id.
    // This only works if you've already set true `track_visibility`.
    bool expose_occluder_id = false;
  };

  // Creates an IntersectionObserver that monitors changes to the intersection
  // and notifies via the given |callback|.
  static IntersectionObserver* Create(
      const Document& document,
      EventCallback callback,
      std::optional<LocalFrameUkmAggregator::MetricId> ukm_metric_id,
      Params&& params);

  IntersectionObserver(
      IntersectionObserverDelegate& delegate,
      std::optional<LocalFrameUkmAggregator::MetricId> ukm_metric_id,
      Params&& params);

  // API methods.
  void observe(Element*, ExceptionState& = ASSERT_NO_EXCEPTION);
  void unobserve(Element*, ExceptionState& = ASSERT_NO_EXCEPTION);
  void disconnect(ExceptionState& = ASSERT_NO_EXCEPTION);
  HeapVector<Member<IntersectionObserverEntry>> takeRecords(
      ExceptionState& = ASSERT_NO_EXCEPTION);

  // API attributes.
  Node* root() const { return root_.Get(); }
  String rootMargin() const;
  String scrollMargin() const;
  const Vector<float>& thresholds() const { return thresholds_; }
  DOMHighResTimeStamp delay() const { return delay_.InMilliseconds(); }
  bool trackVisibility() const { return track_visibility_; }
  bool trackFractionOfRoot() const { return track_fraction_of_root_; }

  // An observer can either track intersections with an explicit root Node,
  // or with the the top-level frame's viewport (the "implicit root").  When
  // tracking the implicit root, root_ will be null, but because root_ is a
  // weak pointer, we cannot surmise that this observer tracks the implicit
  // root just because root_ is null.  Hence root_is_implicit_.
  bool RootIsImplicit() const { return root_is_implicit_; }

  bool HasObservations() const { return !observations_.empty(); }
  bool AlwaysReportRootBounds() const { return always_report_root_bounds_; }
  bool NeedsOcclusionTracking() const {
    return trackVisibility() && !observations_.empty();
  }

  bool ShouldExposeOccluderNodeId() const {
    return trackVisibility() && expose_occluder_id_;
  }

  base::TimeDelta GetEffectiveDelay() const;

  Vector<Length> RootMargin() const {
    return margin_target_ == kApplyMarginToRoot ? margin_ : Vector<Length>();
  }

  Vector<Length> TargetMargin() const {
    return margin_target_ == kApplyMarginToTarget ? margin_ : Vector<Length>();
  }

  Vector<Length> ScrollMargin() const { return scroll_margin_; }

  bool IsInternal() const;
  // The metric id for tracking update time via UpdateTime metrics, or null for
  // internal intersection observers without explicit metrics.
  std::optional<LocalFrameUkmAggregator::MetricId> GetUkmMetricId() const {
    return ukm_metric_id_;
  }

  void ReportUpdates(IntersectionObservation&);
  DeliveryBehavior GetDeliveryBehavior() const;
  void Deliver();

  // Returns false if this observer has an explicit root node which has been
  // deleted; true otherwise.
  bool RootIsValid() const;

  bool UseOverflowClipEdge() const { return use_overflow_clip_edge_ == 1; }

  // ScriptWrappable override:
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

  // Enable/disable throttling of visibility checking, so we don't have to add
  // sleep() calls to tests to wait for notifications to show up.
  static void SetThrottleDelayEnabledForTesting(bool);

  const HeapLinkedHashSet<WeakMember<IntersectionObservation>>& Observations() {
    return observations_;
  }

 private:
  bool NeedsDelivery() const { return !active_observations_.empty(); }
  void ProcessCustomWeakness(const LivenessBroker&);

  const Member<IntersectionObserverDelegate> delegate_;

  // See: `GetUkmMetricId()`.
  const std::optional<LocalFrameUkmAggregator::MetricId> ukm_metric_id_;

  // We use UntracedMember<> here to do custom weak processing.
  UntracedMember<Node> root_;

  HeapLinkedHashSet<WeakMember<IntersectionObservation>> observations_;
  // Observations that have updates waiting to be delivered
  HeapHashSet<Member<IntersectionObservation>> active_observations_;
  const Vector<float> thresholds_;
  const base::TimeDelta delay_;
  const Vector<Length> margin_;
  const Vector<Length> scroll_margin_;
  const MarginTarget margin_target_;
  const unsigned root_is_implicit_ : 1;
  const unsigned track_visibility_ : 1;
  const unsigned track_fraction_of_root_ : 1;
  const unsigned always_report_root_bounds_ : 1;
  const unsigned use_overflow_clip_edge_ : 1;
  const unsigned expose_occluder_id_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_OBSERVER_H_
