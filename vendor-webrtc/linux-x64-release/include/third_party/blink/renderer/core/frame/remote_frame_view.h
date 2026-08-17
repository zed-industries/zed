// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_VIEW_H_

#include <optional>

#include "base/check.h"
#include "base/time/time.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/viewport_intersection_state.mojom-blink.h"
#include "third_party/blink/renderer/core/frame/embedded_content_view.h"
#include "third_party/blink/renderer/core/frame/frame_view.h"
#include "third_party/blink/renderer/core/layout/natural_sizing_info.h"
#include "third_party/blink/renderer/core/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "ui/gfx/geometry/rect.h"

namespace cc {
class PaintCanvas;
}

namespace gfx {
class Vector2d;
}

namespace blink {
class CullRect;
class GraphicsContext;
class LocalFrameView;
class RemoteFrame;

class RemoteFrameView final : public GarbageCollected<RemoteFrameView>,
                              public FrameView {
 public:
  explicit RemoteFrameView(RemoteFrame*);
  ~RemoteFrameView() override;

  void AttachToLayout() override;
  void DetachFromLayout() override;

  LocalFrameView* ParentFrameView() const override;
  LayoutEmbeddedContent* GetLayoutEmbeddedContent() const override;
  RemoteFrame& GetFrame() const {
    DCHECK(remote_frame_);
    return *remote_frame_;
  }

  void Dispose() override;
  void SetFrameRect(const gfx::Rect&) override;
  void PropagateFrameRects() override;
  void ZoomFactorChanged(float zoom_factor) override;
  void Paint(GraphicsContext&,
             PaintFlags,
             const CullRect&,
             const gfx::Vector2d& paint_offset) const override;
  void UpdateGeometry() override;
  void Hide() override;
  void Show() override;

  bool UpdateViewportIntersectionsForSubtree(
      unsigned parent_flags,
      ComputeIntersectionsContext&) override;
  void SetNeedsOcclusionTracking(bool);
  bool NeedsOcclusionTracking() const { return needs_occlusion_tracking_; }

  std::optional<NaturalSizingInfo> GetNaturalDimensions() const override;

  void SetNaturalDimensions(const NaturalSizingInfo& size_info);

  bool CanThrottleRendering() const override;
  void VisibilityForThrottlingChanged() override;
  void VisibilityChanged(blink::mojom::FrameVisibility visibility) override;

  // Compute the interest rect of this frame in its unscrolled space. This may
  // be used by the OOPIF's compositor to limit the amount of rastered tiles,
  // and reduce the number of paint-ops generated. UpdateCompositingRect must be
  // called before the parent frame commits a compositor frame.
  void UpdateCompositingRect();
  gfx::Rect GetCompositingRect() const { return compositing_rect_; }

  void UpdateCompositingScaleFactor();
  float GetCompositingScaleFactor() const { return compositing_scale_factor_; }

  uint32_t Print(const gfx::Rect&, cc::PaintCanvas*) const;
  uint32_t CapturePaintPreview(const gfx::Rect&, cc::PaintCanvas*) const;

  void Trace(Visitor*) const override;

  void ResetFrozenSize() { frozen_size_ = std::nullopt; }

 protected:
  bool NeedsViewportOffset() const override { return true; }
  // This is used to service IntersectionObservers in an OOPIF child document.
  void SetViewportIntersection(const mojom::blink::ViewportIntersectionState&
                                   intersection_state) override;
  void ParentVisibleChanged() override;

 private:
  // This function returns the LocalFrameView associated with the parent frame's
  // local root, or nullptr if the parent frame is not a local frame. For
  // fenced frames, this will return the local root associated with the fenced
  // frame's owner.
  LocalFrameView* ParentLocalRootFrameView() const;

  // This provides the rectangle that the embedded compositor should raster
  // based on its screen space rect. This takes into account the frame's
  // viewport intersection and a buffer area to prevent checkerboarding during
  // animations.
  gfx::Rect ComputeCompositingRect() const;

  // Fetch the frozen size, if any, from the associated LayoutObject.
  void UpdateFrozenSize();

  // The properties and handling of the cycle between RemoteFrame
  // and its RemoteFrameView corresponds to that between LocalFrame
  // and LocalFrameView. Please see the LocalFrameView::frame_ comment for
  // details.
  Member<RemoteFrame> remote_frame_;
  mojom::blink::ViewportIntersectionState last_intersection_state_;
  gfx::Rect compositing_rect_;
  std::optional<gfx::Size> frozen_size_;
  float compositing_scale_factor_ = 1.0f;

  std::optional<NaturalSizingInfo> natural_sizing_info_;
  bool needs_occlusion_tracking_ = false;
  bool needs_frame_rect_propagation_ = false;
};

template <>
struct DowncastTraits<RemoteFrameView> {
  static bool AllowFrom(const EmbeddedContentView& embedded_content_view) {
    return !embedded_content_view.IsLocalFrameView() &&
           !embedded_content_view.IsPluginView();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_FRAME_VIEW_H_
