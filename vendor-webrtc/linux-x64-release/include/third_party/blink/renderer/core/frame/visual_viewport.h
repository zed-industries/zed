/*
 * copyright (c) 2013 google inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VISUAL_VIEWPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VISUAL_VIEWPORT_H_

#include <memory>

#include "base/notreached.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/core/scroll/scrollable_area.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/size.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/gfx/geometry/vector2d_conversions.h"

namespace cc {
class AnimationHost;
class AnimationTimeline;
class SolidColorScrollbarLayer;
}

namespace blink {

enum class PaintPropertyChangeType : unsigned char;
class EffectPaintPropertyNode;
class GraphicsContext;
class LocalFrame;
class Page;
class PaintArtifactCompositor;
class RootFrameViewport;
class ScrollPaintPropertyNode;
class TracedValue;
class TransformPaintPropertyNode;
struct PaintPropertyTreeBuilderFragmentContext;

enum class OverscrollType { kNone, kTransform };

// Represents the visual viewport the user is currently seeing the page through.
// This class corresponds to the InnerViewport on the compositor. It is a
// ScrollableArea; it's offset is set through the GraphicsLayer <-> CC sync
// mechanisms. Its contents is the page's main LocalFrameView, which corresponds
// to the outer viewport. The inner viewport is always contained in the outer
// viewport and can pan within it.
//
// When attached, we will create the following layers:
// - scroll_layer_ (transform: scroll_translation_node_)
// - scrollbar_layer_horizontal_ (optional, transform: DET_or_parent)
// - scrollbar_layer_vertical_ (optional, transform: DET_or_parent)
// (DET_or_parent: device_emulation_transform_node_ if exists,
//  or the parent transform state)
//
// After PrePaint, the property trees will look like this:
//
// Transform tree:
//  parent transform state
//  +- device_emulation_transform_node_ (optional)
//     +- overscroll_elasticity_transform_node_
//        +- page_scale_node__
//           +- scroll_translation_node_ (scroll: scroll_node_)
// Effect tree:
//  parent effect state
//  +- horizontal_scrollbar_effect_node_
//  +- vertical_scrollbar_effect_node_
//
// A VisualViewport is created for each blink::Page which means we'll have a
// VisualViewport for each renderer in a page. However, only the VisualViewport
// in the renderer containing the outermost main frame is considered active.
// VisualViewports that are remote to the outermost main frame are considered
// inert; their scale and location values cannot be changed. See the
// IsActiveViewport() method. Many methods in VisualViewport either return
// defaults or expect to never be called from an inert instance.
class CORE_EXPORT VisualViewport : public GarbageCollected<VisualViewport>,
                                   public ScrollableArea {
 public:
  explicit VisualViewport(Page&);
  ~VisualViewport() override;

  void Trace(Visitor*) const override;

  void InitializeScrollbars();

  // Sets the location of the visual viewport relative to the outer viewport.
  // The coordinates are in partial CSS pixels.
  void SetLocation(const gfx::PointF&);
  // FIXME: This should be called moveBy
  void Move(const ScrollOffset&);

  // The size of the Blink viewport area. See size_ for precise
  // definition.
  void SetSize(const gfx::Size&);
  gfx::Size Size() const { return size_; }

  // The area of the layout viewport rect visible in the visual viewport,
  // relative to the layout viewport's top-left corner. i.e. As the page scale
  // is increased, this rect shrinks. Does not account for browser-zoom (ctrl
  // +/- zooming).
  gfx::RectF VisibleRect(IncludeScrollbarsInRect = kExcludeScrollbars) const;

  // Resets the viewport to initial state.
  void Reset();

  // Let the viewport know that the main frame changed size (either through
  // screen rotation on Android or window resize elsewhere).
  void MainFrameDidChangeSize();

  // Sets scale and location in one operation, preventing intermediate clamping.
  void SetScaleAndLocation(float scale,
                           bool is_pinch_gesture_active,
                           const gfx::PointF& location);

  void SetScale(float);
  float Scale() const { return scale_; }
  bool IsPinchGestureActive() const { return is_pinch_gesture_active_; }

  PhysicalOffset LocalToScrollOriginOffset() const final;

  // Convert the given rect in the main LocalFrameView's coordinates into a rect
  // in the viewport. The given and returned rects are in CSS pixels, meaning
  // scale isn't applied.
  gfx::PointF ViewportCSSPixelsToRootFrame(const gfx::PointF&) const;

  // Clamp the given point, in document coordinates, to the maximum/minimum
  // scroll extents of the viewport within the document.
  gfx::Point ClampDocumentOffsetAtScale(const gfx::Point& offset, float scale);

  // FIXME: This is kind of a hack. Ideally, we would just resize the
  // viewports to account for browser controls. However, LocalFrameView includes
  // much more than just scrolling so we can't simply resize it without
  // incurring all sorts of side-effects. Until we can seperate out the
  // scrollability aspect from LocalFrameView, we use this method to let
  // VisualViewport make the necessary adjustments so that we don't incorrectly
  // clamp scroll offsets coming from the compositor. crbug.com/422328
  void SetBrowserControlsAdjustment(float);
  float BrowserControlsAdjustment() const;

  // Adjust the viewport's offset so that it remains bounded by the outer
  // viepwort.
  void ClampToBoundaries();

  // See
  // http://www.chromium.org/developers/design-documents/blink-coordinate-spaces.
  // These methods are used to convert coordinates from/to viewport to root
  // frame. Root frame coordinates x page scale(pinch zoom) -> Viewport
  gfx::RectF ViewportToRootFrame(const gfx::RectF&) const;
  gfx::Rect ViewportToRootFrame(const gfx::Rect&) const;
  gfx::RectF RootFrameToViewport(const gfx::RectF&) const;
  gfx::Rect RootFrameToViewport(const gfx::Rect&) const;

  gfx::PointF ViewportToRootFrame(const gfx::PointF&) const;
  gfx::PointF RootFrameToViewport(const gfx::PointF&) const;
  gfx::Point ViewportToRootFrame(const gfx::Point&) const;
  gfx::Point RootFrameToViewport(const gfx::Point&) const;

  // ScrollableArea implementation
  ChromeClient* GetChromeClient() const override;
  bool SetScrollOffset(const ScrollOffset&,
                       mojom::blink::ScrollType,
                       mojom::blink::ScrollBehavior,
                       ScrollCallback on_finish,
                       bool targeted_scroll = false) override;
  bool SetScrollOffset(const ScrollOffset&,
                       mojom::blink::ScrollType,
                       mojom::blink::ScrollBehavior =
                           mojom::blink::ScrollBehavior::kInstant) override;
  PhysicalRect ScrollIntoView(
      const PhysicalRect&,
      const PhysicalBoxStrut& scroll_margin,
      const mojom::blink::ScrollIntoViewParamsPtr&) override;
  bool IsThrottled() const override {
    // VisualViewport is always in the main frame, so the frame does not get
    // throttled.
    return false;
  }
  bool IsActive() const override { return false; }
  int ScrollSize(ScrollbarOrientation) const override;
  bool IsScrollCornerVisible() const override { return false; }
  gfx::Rect ScrollCornerRect() const override { return gfx::Rect(); }
  gfx::Vector2d ScrollOffsetInt() const override {
    return SnapScrollOffsetToPhysicalPixels(offset_);
  }
  ScrollOffset GetScrollOffset() const override { return offset_; }
  gfx::Vector2d MinimumScrollOffsetInt() const override;
  gfx::Vector2d MaximumScrollOffsetInt() const override;
  ScrollOffset MaximumScrollOffset() const override;
  ScrollOffset MaximumScrollOffsetAtScale(float scale) const;
  // Note: Because scrollbars are conceptually owned by the LayoutView,
  // ContentsSize includes the main frame's scrollbars. This is necessary for
  // correct cc Layer sizing.
  gfx::Size ContentsSize() const override;
  bool ScrollbarsCanBeActive() const override { return false; }
  bool UserInputScrollable(ScrollbarOrientation) const override;
  bool ShouldPlaceVerticalScrollbarOnLeft() const override { return false; }
  CompositorElementId GetScrollElementId() const override;
  bool ScrollAnimatorEnabled() const override;
  void ScrollControlWasSetNeedsPaintInvalidation() override {}
  void UpdateScrollOffset(const ScrollOffset&,
                          mojom::blink::ScrollType) override;
  cc::Layer* LayerForScrolling() const;
  cc::Layer* LayerForHorizontalScrollbar() const override;
  cc::Layer* LayerForVerticalScrollbar() const override;
  bool ScheduleAnimation() override;
  bool UsesCompositedScrolling() const override { return true; }
  cc::AnimationHost* GetCompositorAnimationHost() const override;
  cc::AnimationTimeline* GetCompositorAnimationTimeline() const override;
  gfx::Rect VisibleContentRect(
      IncludeScrollbarsInRect = kExcludeScrollbars) const override;
  scoped_refptr<base::SingleThreadTaskRunner> GetTimerTaskRunner()
      const override;
  mojom::blink::ColorScheme UsedColorSchemeScrollbars() const override;
  ScrollbarTheme& GetPageScrollbarTheme() const override;
  bool VisualViewportSuppliesScrollbars() const override;
  const Document* GetDocument() const override;

  // VisualViewport scrolling may involve pinch zoom and gets routed through
  // WebViewImpl explicitly rather than via
  // ScrollingCoordinator::DidCompositorScroll() since it needs to be set in
  // tandem with the page scale delta.
  void DidCompositorScroll(const gfx::PointF&) final { NOTREACHED(); }

  // Visual Viewport API implementation.
  double OffsetLeft() const;
  double OffsetTop() const;
  double Width() const;
  double Height() const;
  double ScaleForVisualViewport() const;

  // Used to calculate Width and Height above but do not update layout.
  double VisibleWidthCSSPx() const;
  double VisibleHeightCSSPx() const;

  // Heuristic-based function for determining if we should disable workarounds
  // for viewing websites that are not optimized for mobile devices.
  bool ShouldDisableDesktopWorkarounds() const;

  const TransformPaintPropertyNode* GetDeviceEmulationTransformNode() const;
  const TransformPaintPropertyNode* GetOverscrollElasticityTransformNode()
      const;
  const TransformPaintPropertyNode* GetPageScaleNode() const;
  const TransformPaintPropertyNode* GetScrollTranslationNode() const;
  const ScrollPaintPropertyNode* GetScrollNode() const;

  const TransformPaintPropertyNode* TransformNodeForViewportScrollbars() const;

  // Create/update the page scale translation, viewport scroll, and viewport
  // translation property nodes. Returns the maximum paint property change
  // type for any of the viewport's nodes.
  PaintPropertyChangeType UpdatePaintPropertyNodesIfNeeded(
      PaintPropertyTreeBuilderFragmentContext& context);

  void SetNeedsPaintPropertyUpdate() {
    DCHECK(IsActiveViewport());
    needs_paint_property_update_ = true;
  }
  bool NeedsPaintPropertyUpdate() const {
    DCHECK(IsActiveViewport());
    return needs_paint_property_update_;
  }

  void DisposeImpl() override;

  void Paint(GraphicsContext&) const;

  void UsedColorSchemeChanged();
  void ScrollbarColorChanged();

  // Returns whether this VisualViewport is "active", that is, whether it'll
  // affect paint property trees. If false, this renderer cannot be
  // independently scaled.
  //
  // A VisualViewport is created in renderers for remote frames / nested pages;
  // however, in those cases it is "inert", it cannot change scale or location
  // values. Only an outermost main frame can have an active viewport.
  bool IsActiveViewport() const;

  OverscrollType GetOverscrollType() const { return overscroll_type_; }
  void SetOverscrollTypeForTesting(OverscrollType type) {
    overscroll_type_ = type;
    SetNeedsPaintPropertyUpdate();
  }
  std::optional<blink::Color> CSSScrollbarThumbColor() const;

  void DropCompositorScrollDeltaNextCommit() override;

 private:
  bool DidSetScaleOrLocation(float scale,
                             bool is_pinch_gesture_active,
                             const gfx::PointF& location);

  void CreateLayers();

  void EnqueueScrollEvent();
  void EnqueueResizeEvent();

  EScrollbarWidth CSSScrollbarWidth() const;
  int ScrollbarThickness() const;
  void UpdateScrollbarLayer(ScrollbarOrientation);
  void UpdateScrollbarColor(cc::SolidColorScrollbarLayer&);

  void NotifyRootFrameViewport() const;

  RootFrameViewport* GetRootFrameViewport() const;

  // Returns the local main frame, this can only be called for an active
  // VisualViewport.
  LocalFrame& LocalMainFrame() const;

  Page& GetPage() const {
    DCHECK(page_);
    return *page_;
  }

  PaintArtifactCompositor* GetPaintArtifactCompositor() const;

  std::unique_ptr<TracedValue> ViewportToTracedValue() const;

  // Contracts the given size by the thickness of any visible scrollbars. Does
  // not contract the size if the scrollbar is overlay.
  // TODO(bokan): This does not work for a VisualViewport that is in a remote
  // renderer (i.e. !IsActiveViewport).
  gfx::Size ExcludeScrollbars(const gfx::Size&) const;

  Member<Page> page_;

  scoped_refptr<cc::Layer> scroll_layer_;
  scoped_refptr<cc::SolidColorScrollbarLayer> scrollbar_layer_horizontal_;
  scoped_refptr<cc::SolidColorScrollbarLayer> scrollbar_layer_vertical_;

  TraceablePropertyTreeStateOrAlias parent_property_tree_state_{
      TraceablePropertyTreeStateOrAlias::kUninitialized};
  Member<TransformPaintPropertyNode> device_emulation_transform_node_;
  Member<TransformPaintPropertyNode> overscroll_elasticity_transform_node_;
  Member<TransformPaintPropertyNode> page_scale_node_;
  Member<TransformPaintPropertyNode> scroll_translation_node_;
  Member<ScrollPaintPropertyNode> scroll_node_;
  Member<EffectPaintPropertyNode> horizontal_scrollbar_effect_node_;
  Member<EffectPaintPropertyNode> vertical_scrollbar_effect_node_;

  // Offset of the visual viewport from the main frame's origin, in CSS pixels.
  ScrollOffset offset_;
  float scale_;
  bool is_pinch_gesture_active_;

  // The Blink viewport size. This is effectively the size of the rect the
  // Blink WebView is rendering into and includes space consumed by scrollbars.
  // While it will not include the URL bar height, Blink is only informed of
  // changes to the URL bar once they're fully committed (all the way hidden or
  // shown). While they're animating or being dragged, size_ will not reflect
  // the changed visible content area. The transient URL bar-caused change to
  // the visible content area is tracked in browser_controls_adjustment.
  gfx::Size size_;

  // Blink is only resized as a result of showing/hiding the URL bar once
  // they're fully committed (all the way hidden or shown). While they're
  // animating or being dragged, browser_controls_adjustment_ tracks the amount
  // they expand or shrink the visible content height.
  float browser_controls_adjustment_;

  // For page scale animation on page_scale_node_.
  CompositorElementId page_scale_element_id_;
  // For scrolling, on scroll_layer_, scroll_node_, and scroll element ids of
  // scrollbar layers.
  CompositorElementId scroll_element_id_;

  bool needs_paint_property_update_;

  OverscrollType overscroll_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VISUAL_VIEWPORT_H_
