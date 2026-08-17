// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_VIDEO_VISIBILITY_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_VIDEO_VISIBILITY_TRACKER_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/frame/local_frame_view.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/hit_test_request.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client_types.h"
#include "third_party/blink/renderer/platform/heap/heap_traits.h"
#include "third_party/skia/include/core/SkRect.h"

namespace blink {

class Document;
class HTMLVideoElement;

// This class tracks the area of an HTMLVideoElement, measured in square CSS
// pixels, that is visible to the user and reports whether the element's
// visibility is greater or equal than a given threshold of the visible area
// (`visibility_threshold_`) or not.
//
// "Visible" in this context is defined as intersecting with the viewport and
// not occluded by other html elements within the page, with the exception of
// MediaControls.
class CORE_EXPORT MediaVideoVisibilityTracker final
    : public NativeEventListener,
      public LocalFrameView::LifecycleNotificationObserver {
 public:
  // Struct to hold various counts, only used for metrics collection.
  struct Metrics {
    // Total number of hit tested nodes.
    int total_hit_tested_nodes = 0;

    // Total number of occluding rects.
    int total_occluding_rects = 0;

    // Total number of hit tested nodes that contribute to occlusion.
    int total_hit_tested_nodes_contributing_to_occlusion = 0;

    // Total number of ignored hit tested nodes that are in the shadow tree and
    // of user agent type.
    int total_ignored_nodes_user_agent_shadow_root = 0;

    // Total number of ignored hit tested nodes that are not opaque.
    int total_ignored_nodes_not_opaque = 0;
  };

  // Struct to hold various variables used during occlusion computations.
  struct OcclusionState {
    float occluded_area = 0.0;
    VectorOf<SkIRect> occluding_rects;
    PhysicalRect intersection_rect;
    PhysicalRect video_element_rect;
  };

  // Indicates if the |ReportVisibilityCb| should be executed, or not.
  enum class ShouldReportVisibility {
    kNo,
    kYes,
  };

  static constexpr base::TimeDelta kMinimumAllowedHitTestInterval =
      base::Milliseconds(500);

  using ReportVisibilityCb = base::RepeatingCallback<void(bool)>;
  using TrackerAttachedToDocument = WeakMember<Document>;
  using ClientIdsSet = WTF::HashSet<DisplayItemClientId>;

  // `RequestVisibilityCallback` is used to enable computing video visibility
  // on-demand, in response to calls to the MediaPlayer interface
  // `RequestVisibility` method.
  //
  // The boolean parameter represents whether a video element meets
  // `visibility_threshold_`.
  using RequestVisibilityCallback = base::OnceCallback<void(bool)>;

  MediaVideoVisibilityTracker(
      HTMLVideoElement& video,
      int visibility_threshold,
      ReportVisibilityCb report_visibility_cb,
      base::TimeDelta hit_test_interval = kMinimumAllowedHitTestInterval);
  ~MediaVideoVisibilityTracker() override;

  // Updates the visibility tracker state by attaching/detaching the tracker as
  // needed. It is safe to call this method regardless of whether the tracker is
  // already attached/detached.
  void UpdateVisibilityTrackerState();

  // Called by the |HTMLVideoElement| |DidMoveToNewDocument| method to detach
  // the visibility tracker.
  void ElementDidMoveToNewDocument();

  // EventListener implementation.
  void Invoke(ExecutionContext*, Event*) override;

  void MaybeAddFullscreenEventListeners();
  void MaybeRemoveFullscreenEventListeners();

  // Takes the `RequestVisibilityCallback` and either computes visibility
  // immediately, or schedules the computation for later, depending on the the
  // document lifecycle state.
  //
  // If this method is called multiple times in a row, the newest callback
  // always takes precedence. Previous ones are immediately run with `false`.
  void RequestVisibility(RequestVisibilityCallback request_visibility_callback);

  void Trace(Visitor*) const override;

 private:
  // Friend class for testing.
  friend class MediaVideoVisibilityTrackerTest;
  friend class HTMLMediaElementTest;
  friend class HTMLVideoElementTest;

  HTMLVideoElement& VideoElement() const { return *video_element_; }

  // Registers the tracker for lifecycle notifications.
  void Attach();
  void Detach();

  // Returns a set of DisplayItemClientId s starting after
  // `start_after_display_item_client_id` display item. The following are not
  // included in the set:
  //  * `start_after_display_item_client_id`
  //  * DisplayItemClientId s that are not of content type (e.g. viewport
  //  scroll, scrollbars, etc.)
  //
  // If a node's `LayoutObject` Id (`DisplayItemClientId) is not in the set,
  // this indicates that the given `LayoutObject` does not draw any content on
  // the screen.
  const ClientIdsSet GetClientIdsSet(
      DisplayItemClientId start_after_display_item_client_id) const;

  ListBasedHitTestBehavior ComputeOcclusion(const ClientIdsSet& client_ids_set,
                                            Metrics&,
                                            const Node& node,
                                            DOMNodeId node_id);
  bool MeetsVisibilityThreshold(Metrics& counters, const PhysicalRect& rect);
  void ReportVisibility(bool meets_visibility_threshold);
  bool ComputeVisibility();

  // Resets the various member variables used by `ComputeOcclusion()`.
  void ResetMembers();

  // Computes the area of the video element that is occluded by the viewport.
  void ComputeAreaOccludedByViewport(const LocalFrameView& local_frame_view);

  // Computes and reports visibility as appropriate. This method is called when
  // either computing visibility on demand, or continuously. When called to
  // compute visibility on demand, if the document lifecycle is in the
  // `DocumentLifecycle::kPaintClean` state, visibility is computed immediately,
  // otherwise the computation will take place during
  // `DidFinishLifecycleUpdate`.
  void MaybeComputeVisibility(ShouldReportVisibility should_report_visibility);

  // LocalFrameView::LifecycleNotificationObserver
  void DidFinishLifecycleUpdate(const LocalFrameView&) override;

  // `video_element_` creates |this|.
  Member<HTMLVideoElement> video_element_;

  // Threshold used to report whether a video element is sufficiently visible or
  // not. A video element with a visible area (in square pixels) greater or
  // equal than `visibility_threshold_` is considered to meet the visibility
  // threshold.
  //
  // There are no considerations for how this area is distributed, as long as
  // the visible area is >= `visibility_threshold_`, the video element will be
  // considered sufficiently visible.
  const int visibility_threshold_;
  OcclusionState occlusion_state_;
  ReportVisibilityCb report_visibility_cb_;
  RequestVisibilityCallback request_visibility_callback_;
  base::TimeTicks last_hit_test_timestamp_;
  const base::TimeDelta hit_test_interval_;
  bool meets_visibility_threshold_ = false;

  // Keeps track of the |Document| to which the tracker has registered for
  // lifecycle notifications.
  TrackerAttachedToDocument tracker_attached_to_document_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_VIDEO_VISIBILITY_TRACKER_H_
