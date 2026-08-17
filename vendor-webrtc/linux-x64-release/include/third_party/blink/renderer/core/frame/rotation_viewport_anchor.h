// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ROTATION_VIEWPORT_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ROTATION_VIEWPORT_ANCHOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

class LocalFrameView;
class Node;
class PageScaleConstraintsSet;
class ScrollableArea;
class VisualViewport;

// The rotation anchor provides a way to anchor a viewport origin to a DOM node.
// In particular, the user supplies an anchor point (in view coordinates, e.g.,
// (0, 0) == viewport origin, (0.5, 0) == viewport top center). The anchor point
// tracks the underlying DOM node; as the node moves or the view is resized, the
// viewport anchor maintains its orientation relative to the node, and the
// viewport origin maintains its orientation relative to the anchor. If there is
// no node or it is lost during the resize, we fall back to the resize anchor
// logic.
class CORE_EXPORT RotationViewportAnchor {
  STACK_ALLOCATED();

 public:
  RotationViewportAnchor(LocalFrameView& root_frame_view,
                         VisualViewport&,
                         const gfx::PointF& anchor_in_inner_view_coords,
                         PageScaleConstraintsSet&);
  ~RotationViewportAnchor();

 private:
  void SetAnchor();
  void RestoreToAnchor();

  gfx::PointF GetInnerOrigin(const gfx::SizeF& inner_size) const;

  void ComputeOrigins(const gfx::SizeF& inner_size,
                      gfx::Point& main_frame_origin,
                      gfx::PointF& visual_viewport_origin) const;
  ScrollableArea& LayoutViewport() const;

  LocalFrameView* root_frame_view_;
  VisualViewport* visual_viewport_;

  float old_page_scale_factor_;
  float old_minimum_page_scale_factor_;

  // Inner viewport origin in the reference frame of the document in CSS pixels
  gfx::PointF visual_viewport_in_document_;

  // Inner viewport origin in the reference frame of the outer viewport
  // normalized to the outer viewport size.
  gfx::Vector2dF normalized_visual_viewport_offset_;

  Node* anchor_node_;

  // In Document coordinates.
  PhysicalRect anchor_node_bounds_;

  gfx::PointF anchor_in_inner_view_coords_;
  gfx::PointF anchor_in_node_coords_;

  PageScaleConstraintsSet* page_scale_constraints_set_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ROTATION_VIEWPORT_ANCHOR_H_
