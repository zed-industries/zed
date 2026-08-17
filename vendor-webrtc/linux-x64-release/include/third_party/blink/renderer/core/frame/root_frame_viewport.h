// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ROOT_FRAME_VIEWPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ROOT_FRAME_VIEWPORT_H_

#include "base/gtest_prod_util.h"
#include "base/task/single_thread_task_runner.h"
#include "cc/input/scroll_snap_data.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scrollable_area.h"
#include "third_party/blink/renderer/platform/graphics/overlay_scrollbar_clip_behavior.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class LocalFrameView;
struct PhysicalRect;

// ScrollableArea for the root frame's viewport. This class ties together the
// concepts of layout and visual viewports, used in pinch-to-zoom. This class
// takes two ScrollableAreas, one for the visual viewport and one for the
// layout viewport, and delegates and composes the ScrollableArea API as needed
// between them. For most scrolling APIs, this class will split the scroll up
// between the two viewports in accord with the pinch-zoom semantics. For other
// APIs that don't make sense on the combined viewport, the call is delegated to
// the layout viewport. Thus, we could say this class is a decorator on the
// LocalFrameView scrollable area that adds pinch-zoom semantics to scrolling.
class CORE_EXPORT RootFrameViewport final
    : public GarbageCollected<RootFrameViewport>,
      public ScrollableArea {
 public:
  RootFrameViewport(ScrollableArea& visual_viewport,
                    ScrollableArea& layout_viewport);

  void Trace(Visitor*) const override;

  void SetLayoutViewport(ScrollableArea&);
  ScrollableArea& LayoutViewport() const;

  // Convert from the root content document's coordinate space, into the
  // coordinate space of the layout viewport's content. In the normal case,
  // this will be a no-op since the root LocalFrameView is the layout viewport
  // and so the root content is the layout viewport's content but if the page
  // sets a custom root scroller via document.rootScroller, another element
  // may be the layout viewport.
  PhysicalRect RootContentsToLayoutViewportContents(
      LocalFrameView& root_frame_view,
      const PhysicalRect&) const;

  void RestoreToAnchor(const ScrollOffset&);

  // Callback whenever the visual viewport changes scroll position or scale.
  void DidUpdateVisualViewport();

  // ScrollableArea Implementation
  PhysicalOffset LocalToScrollOriginOffset() const final;
  bool IsRootFrameViewport() const override { return true; }
  bool SetScrollOffset(const ScrollOffset&,
                       mojom::blink::ScrollType,
                       mojom::blink::ScrollBehavior,
                       ScrollCallback on_finish,
                       bool targeted_scroll = false) override;
  PhysicalRect ScrollIntoView(
      const PhysicalRect&,
      const PhysicalBoxStrut& scroll_margin,
      const mojom::blink::ScrollIntoViewParamsPtr&) override;
  gfx::Rect VisibleContentRect(
      IncludeScrollbarsInRect = kExcludeScrollbars) const override;
  PhysicalRect VisibleScrollSnapportRect(
      IncludeScrollbarsInRect = kExcludeScrollbars) const override;
  bool ShouldUseIntegerScrollOffset() const override;
  bool IsThrottled() const override {
    // RootFrameViewport is always in the main frame, so the frame does not get
    // throttled.
    return false;
  }
  bool IsActive() const override;
  int ScrollSize(ScrollbarOrientation) const override;
  bool IsScrollCornerVisible() const override;
  gfx::Rect ScrollCornerRect() const override;
  void UpdateScrollOffset(const ScrollOffset&,
                          mojom::blink::ScrollType) override;
  gfx::PointF ScrollOffsetToPosition(const ScrollOffset& offset) const override;
  ScrollOffset ScrollPositionToOffset(
      const gfx::PointF& position) const override;
  gfx::Vector2d ScrollOffsetInt() const override;
  ScrollOffset GetScrollOffset() const override;
  gfx::Vector2d MinimumScrollOffsetInt() const override;
  gfx::Vector2d MaximumScrollOffsetInt() const override;
  ScrollOffset MaximumScrollOffset() const override;
  gfx::Size ContentsSize() const override;
  bool UsesCompositedScrolling() const override;
  bool ShouldScrollOnMainThread() const override;
  bool ScrollbarsCanBeActive() const override;
  bool UserInputScrollable(ScrollbarOrientation) const override;
  bool ShouldPlaceVerticalScrollbarOnLeft() const override;
  void ScrollControlWasSetNeedsPaintInvalidation() override;
  cc::Layer* LayerForHorizontalScrollbar() const override;
  cc::Layer* LayerForVerticalScrollbar() const override;
  cc::Layer* LayerForScrollCorner() const override;
  int HorizontalScrollbarHeight(OverlayScrollbarClipBehavior =
                                    kIgnoreOverlayScrollbarSize) const override;
  int VerticalScrollbarWidth(OverlayScrollbarClipBehavior =
                                 kIgnoreOverlayScrollbarSize) const override;
  ScrollResult UserScroll(ui::ScrollGranularity,
                          const ScrollOffset&,
                          ScrollableArea::ScrollCallback on_finish) override;
  CompositorElementId GetScrollElementId() const override;
  CompositorElementId GetScrollbarElementId(
      ScrollbarOrientation orientation) override;
  bool ScrollAnimatorEnabled() const override;
  ChromeClient* GetChromeClient() const override;
  void ServiceScrollAnimations(double) override;
  void UpdateCompositorScrollAnimations() override;
  void CancelProgrammaticScrollAnimation() override;
  mojom::blink::ScrollBehavior ScrollBehaviorStyle() const override;
  mojom::blink::ColorScheme UsedColorSchemeScrollbars() const override;
  void ClearScrollableArea() override;
  LayoutBox* GetLayoutBox() const override;
  gfx::QuadF LocalToVisibleContentQuad(const gfx::QuadF&,
                                       const LayoutObject*,
                                       unsigned = 0) const final;
  scoped_refptr<base::SingleThreadTaskRunner> GetTimerTaskRunner() const final;
  ScrollbarTheme& GetPageScrollbarTheme() const override;

  // RootFrameViewport delegates these scroll-snap methods to its layout
  // viewport.
  const cc::SnapContainerData* GetSnapContainerData() const override;
  void SetSnapContainerData(std::optional<cc::SnapContainerData>) override;
  bool SetTargetSnapAreaElementIds(cc::TargetSnapAreaElementIds) override;
  bool SnapContainerDataNeedsUpdate() const override;
  void SetSnapContainerDataNeedsUpdate(bool) override;
  std::optional<cc::SnapPositionData> GetSnapPosition(
      const cc::SnapSelectionStrategy& strategy) const override;
  std::optional<gfx::PointF> GetSnapPositionAndSetTarget(
      const cc::SnapSelectionStrategy& strategy) override;
  void UpdateSnappedTargetsAndEnqueueScrollSnapChange() override;
  std::optional<cc::TargetSnapAreaElementIds> GetScrollsnapchangingTargetIds()
      const override;
  void SetScrollsnapchangeTargetIds(
      std::optional<cc::TargetSnapAreaElementIds>) override;
  void SetScrollsnapchangingTargetIds(
      std::optional<cc::TargetSnapAreaElementIds>) override;
  const cc::SnapSelectionStrategy* GetImplSnapStrategy() const override;
  void SetImplSnapStrategy(
      std::unique_ptr<cc::SnapSelectionStrategy> strategy) override;
  void EnqueueScrollSnapChangingEventFromImplIfNeeded() override;
  void UpdateScrollSnapChangingTargetsAndEnqueueScrollSnapChanging(
      const cc::TargetSnapAreaElementIds& new_target_ids) override;
  void SetSnappedQueryTargetIds(
      std::optional<cc::TargetSnapAreaElementIds> new_target_ids) override;

  void SetPendingHistoryRestoreScrollOffset(
      const HistoryItem::ViewState& view_state,
      bool should_restore_scroll,
      mojom::blink::ScrollBehavior scroll_behavior) override {
    pending_view_state_ = view_state;
    should_restore_scroll_ = should_restore_scroll;
  }

  void ApplyPendingHistoryRestoreScrollOffset() override;

  bool HasPendingHistoryRestoreScrollOffset() override {
    return !!pending_view_state_;
  }

  // A sequence of UserScrolls may occur close enough to each other (e.g.
  // repeated keypresses) to produce a single scroll.
  // This function returns true if any UserScroll in a sequence of
  // UserScrolls applies a non-zero scroll delta to the LayoutViewport.
  bool ScrollAffectsLayoutViewport() {
    return user_scroll_sequence_affects_layout_viewport_;
  }

  std::optional<cc::ElementId> GetTargetedSnapAreaId() override;
  void SetTargetedSnapAreaId(const std::optional<cc::ElementId>&) override;

  void DropCompositorScrollDeltaNextCommit() override;

 private:
  FRIEND_TEST_ALL_PREFIXES(RootFrameViewportTest, DistributeScrollOrder);

  enum ViewportToScrollFirst { kVisualViewport, kLayoutViewport };

  ScrollOffset ScrollOffsetFromScrollAnimators() const;

  bool DistributeScrollBetweenViewports(
      const ScrollOffset&,
      mojom::blink::ScrollType,
      mojom::blink::ScrollBehavior,
      ViewportToScrollFirst,
      ScrollCallback on_finish = ScrollCallback());

  // If either of the layout or visual viewports are scrolled explicitly (i.e.
  // not through this class), their updated offset will not be reflected in this
  // class' animator so use this method to pull updated values when necessary.
  void UpdateScrollAnimator();

  ScrollableArea& GetVisualViewport() const {
    DCHECK(visual_viewport_);
    return *visual_viewport_;
  }

  ScrollOffset ClampToUserScrollableOffset(const ScrollOffset&) const;

  Member<ScrollableArea> visual_viewport_;
  Member<ScrollableArea> layout_viewport_;
  std::optional<HistoryItem::ViewState> pending_view_state_;
  bool should_restore_scroll_;
  bool user_scroll_sequence_affects_layout_viewport_ = false;
};

template <>
struct DowncastTraits<RootFrameViewport> {
  static bool AllowFrom(const ScrollableArea& scrollable_area) {
    return scrollable_area.IsRootFrameViewport();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_ROOT_FRAME_VIEWPORT_H_
