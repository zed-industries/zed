// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VIEW_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/viewport_intersection_state.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/embedded_content_view.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class Frame;
class ComputeIntersectionsContext;
struct NaturalSizingInfo;

class CORE_EXPORT FrameView : public EmbeddedContentView {
 public:
  explicit FrameView(const gfx::Rect& frame_rect);
  ~FrameView() override = default;

  // parent_flags is the result of calling GetIntersectionObservationFlags on
  // the LocalFrameView parent of this FrameView (if any). It contains dirty
  // bits based on whether geometry may have changed in the parent frame.
  // Returns true if the frame needs occlusion tracking (i.e. trackVisibility()
  // is true for any tracked observer in the frame subtree).
  virtual bool UpdateViewportIntersectionsForSubtree(
      unsigned parent_flags,
      ComputeIntersectionsContext&) = 0;

  virtual std::optional<NaturalSizingInfo> GetNaturalDimensions() const = 0;

  // Returns true if this frame could potentially skip rendering and avoid
  // scheduling visual updates.
  virtual bool CanThrottleRendering() const = 0;

  // A display:none iframe cannot be throttled, but its child frames can be
  // throttled. This method will return 'true' for the the display:none iframe.
  // It is used to set the subtree_throttled_ flag on child frames.
  bool CanThrottleRenderingForPropagation() const;

  bool IsFrameView() const override { return true; }
  virtual bool ShouldReportMainFrameIntersection() const { return false; }

  Frame& GetFrame() const;
  std::optional<mojom::blink::FrameVisibility> GetFrameVisibility() const {
    return frame_visibility_;
  }

  // This is used to control render throttling, which determines whether
  // lifecycle updates in the child frame will skip rendering work.
  bool IsHiddenForThrottling() const { return hidden_for_throttling_; }
  bool IsSubtreeThrottled() const { return subtree_throttled_; }
  // This indicates whether this is an iframe whose contents are display-locked
  // due to an active DisplayLock in the parent frame. Note that this value must
  // be stable between main frames, and only gets updated based on the current
  // state of display locking in the parent frame when
  // UpdateViewportIntersection is run during post-lifecycle steps.
  bool IsDisplayLocked() const { return display_locked_; }
  virtual void UpdateRenderThrottlingStatus(bool hidden_for_throttling,
                                            bool subtree_throttled,
                                            bool display_locked,
                                            bool recurse = false);

  bool RectInParentIsStable(const base::TimeTicks& timestamp) const;

  // See kTargetFrameMovedRecentlyForIOv2 in web_input_event.h.
  bool RectInParentIsStableForIOv2(const base::TimeTicks& timestamp) const;

 protected:
  virtual bool NeedsViewportOffset() const { return false; }
  virtual void SetViewportIntersection(
      const mojom::blink::ViewportIntersectionState& intersection_state) = 0;
  virtual void VisibilityForThrottlingChanged() = 0;
  virtual bool LifecycleUpdatesThrottled() const { return false; }
  void UpdateViewportIntersection(unsigned flags,
                                  bool needs_occlusion_tracking);

  // FrameVisibility is tracked by the browser process, which may suppress
  // lifecycle updates for a frame outside the viewport.
  void UpdateFrameVisibility(bool);

  bool DisplayLockedInParentFrame();

  virtual void VisibilityChanged(mojom::blink::FrameVisibility visibilty) = 0;
  std::optional<mojom::blink::FrameVisibility> frame_visibility() const {
    return frame_visibility_;
  }

 private:
  PhysicalRect rect_in_parent_;
  PhysicalRect rect_in_parent_for_iov2_;
  base::TimeTicks rect_in_parent_stable_since_;
  base::TimeTicks rect_in_parent_stable_since_for_iov2_;
  // The visibility of this frame, which takes into account the intersection
  // with the viewport. Nullopt means this is not known yet.
  std::optional<mojom::blink::FrameVisibility> frame_visibility_;
  bool hidden_for_throttling_ = false;
  bool subtree_throttled_ = false;
  bool display_locked_ = false;
};

template <>
struct DowncastTraits<FrameView> {
  static bool AllowFrom(const EmbeddedContentView& embedded_content_view) {
    return embedded_content_view.IsFrameView();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VIEW_H_
