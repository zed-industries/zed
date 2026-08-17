/*
 * Copyright (C) 2008, 2011 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLABLE_AREA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLABLE_AREA_H_

#include <set>

#include "base/functional/callback_helpers.h"
#include "base/gtest_prod_util.h"
#include "base/notreached.h"
#include "cc/input/scroll_snap_data.h"
#include "cc/input/snap_selection_strategy.h"
#include "cc/paint/element_id.h"
#include "third_party/blink/public/common/input/web_gesture_device.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_scroll_behavior.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/loader/history_item.h"
#include "third_party/blink/renderer/core/scroll/scrollbar.h"
#include "third_party/blink/renderer/core/style/scroll_start_data.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/graphics/overlay_scrollbar_clip_behavior.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/quad_f.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace cc {
class AnimationHost;
class AnimationTimeline;
class Layer;
}  // namespace cc

namespace blink {
class ChromeClient;
class Document;
class LayoutBox;
class LayoutObject;
class LocalFrame;
class MacScrollbarAnimator;
class Node;
class PaintLayer;
class ProgrammaticScrollAnimator;
class ScrollAnchor;
class ScrollAnimatorBase;
struct SerializedAnchor;
class ScrollMarkerGroupPseudoElement;

using MainThreadScrollingReasons = uint32_t;

enum IncludeScrollbarsInRect {
  kExcludeScrollbars,
  kIncludeScrollbars,
};

class CORE_EXPORT ScrollableArea : public GarbageCollectedMixin {
  USING_PRE_FINALIZER(ScrollableArea, Dispose);

 public:
  // This enum indicates whether a scroll animation was
  // interrupted by another scroll animation. We use this to decide
  // whether or not to fire scrollend.
  enum class ScrollCompletionMode {
    kFinished,
    kInterruptedByScroll,
    kZeroDelta
  };
  using ScrollCallback =
      base::OnceCallback<void(ScrollableArea::ScrollCompletionMode)>;

  ScrollableArea(const ScrollableArea&) = delete;
  ScrollableArea& operator=(const ScrollableArea&) = delete;

  static int PixelsPerLineStep(LocalFrame*);

  // Convert a non-finite scroll value (Infinity, -Infinity, NaN) to 0 as
  // per https://drafts.csswg.org/cssom-view/#normalize-non-finite-values.
  static float NormalizeNonFiniteScroll(float value) {
    return std::isfinite(value) ? value : 0.0;
  }

  virtual ChromeClient* GetChromeClient() const { return nullptr; }

  // Used to scale a length in dip units into a length in layout/paint units.
  virtual float ScaleFromDIP() const;

  virtual ScrollResult UserScroll(ui::ScrollGranularity,
                                  const ScrollOffset&,
                                  ScrollCallback on_finish);

  // See https://crbug.com/413002675: `on_finish` is not always executed at the
  // end of the scroll (example: it may be executed while the scroll is in
  // progress for animated programmatic scrolls).
  virtual bool SetScrollOffset(const ScrollOffset&,
                               mojom::blink::ScrollType,
                               mojom::blink::ScrollBehavior,
                               ScrollCallback on_finish,
                               bool targeted_scroll = false);
  virtual bool SetScrollOffset(
      const ScrollOffset&,
      mojom::blink::ScrollType,
      mojom::blink::ScrollBehavior = mojom::blink::ScrollBehavior::kInstant);
  void ScrollBy(
      const ScrollOffset&,
      mojom::blink::ScrollType,
      mojom::blink::ScrollBehavior = mojom::blink::ScrollBehavior::kInstant);

  virtual void SetPendingHistoryRestoreScrollOffset(
      const HistoryItem::ViewState& view_state,
      bool should_restore_scroll,
      mojom::blink::ScrollBehavior scroll_behavior) {}
  virtual void ApplyPendingHistoryRestoreScrollOffset() {}

  virtual bool HasPendingHistoryRestoreScrollOffset() { return false; }

  // Scrolls the area so that the given rect, given in absolute coordinates,
  // such that it's visible in the area. Returns the new location of the input
  // rect in absolute coordinates.
  virtual PhysicalRect ScrollIntoView(
      const PhysicalRect&,
      const PhysicalBoxStrut& scroll_margin,
      const mojom::blink::ScrollIntoViewParamsPtr&);

  virtual PhysicalOffset LocalToScrollOriginOffset() const = 0;

  static mojom::blink::ScrollBehavior V8EnumToScrollBehavior(
      V8ScrollBehavior::Enum);

  // Register a callback that will be invoked when the next scroll completes -
  // this includes the scroll animation time.
  void RegisterScrollCompleteCallback(ScrollCallback callback);
  void RunScrollCompleteCallbacks(ScrollCompletionMode);

  void MouseEnteredScrollbar(Scrollbar&);
  void MouseExitedScrollbar(Scrollbar&);
  void MouseCapturedScrollbar();
  void MouseReleasedScrollbar();

  virtual const cc::SnapContainerData* GetSnapContainerData() const {
    return nullptr;
  }
  virtual void SetSnapContainerData(std::optional<cc::SnapContainerData>) {}
  virtual bool SetTargetSnapAreaElementIds(cc::TargetSnapAreaElementIds) {
    return false;
  }
  virtual bool SnapContainerDataNeedsUpdate() const { return false; }
  virtual void SetSnapContainerDataNeedsUpdate(bool) {}
  void SnapAfterScrollbarScrolling(ScrollbarOrientation);
  virtual void UpdateFocusDataForSnapAreas() {}

  // SnapAtCurrentPosition(), SnapForEndPosition(), SnapForDirection(), and
  // SnapForEndAndDirection() return true if snapping was performed, and false
  // otherwise. Note that this does not necessarily mean that any scrolling was
  // performed as a result e.g., if we are already at the snap point.
  // The scroll callback parameter is used to set the hover state dirty and
  // send a scroll end event when the scroll ends without snap or the snap
  // point is the same as the scroll position.
  //
  // SnapAtCurrentPosition() calls SnapForEndPosition() with the current
  // scroll position.
  bool SnapAtCurrentPosition(
      bool scrolled_x,
      bool scrolled_y,
      base::ScopedClosureRunner on_finish = base::ScopedClosureRunner());
  bool SnapForEndPosition(
      const gfx::PointF& end_position,
      bool scrolled_x,
      bool scrolled_y,
      base::ScopedClosureRunner on_finish = base::ScopedClosureRunner());
  bool SnapForDirection(
      const ScrollOffset& delta,
      base::ScopedClosureRunner on_finish = base::ScopedClosureRunner());
  bool SnapForEndAndDirection(const ScrollOffset& delta);
  void SnapAfterLayout();

  // Tries to find a target snap position. If found, returns the target
  // position. This *does not* update the target snap area element id and should
  // only be used when querying what the snap position would be. When scrolling
  // to the snap position GetSnapPositionAndSetTarget should be used instead.
  virtual std::optional<cc::SnapPositionData> GetSnapPosition(
      const cc::SnapSelectionStrategy& strategy) const {
    return std::nullopt;
  }

  // Tries to find a target snap position. If found, returns the target position
  // and updates the last target snap area element id for the snap container's
  // data. If not found, then clears the last target snap area element id.
  //
  // NOTE: If a target position is found, then it is expected that this position
  // will be scrolled to.
  virtual std::optional<gfx::PointF> GetSnapPositionAndSetTarget(
      const cc::SnapSelectionStrategy& strategy) {
    return std::nullopt;
  }

  virtual void DidAddScrollbar(Scrollbar&, ScrollbarOrientation);
  virtual void WillRemoveScrollbar(Scrollbar&, ScrollbarOrientation);

  // Called when this ScrollableArea becomes or unbecomes the global root
  // scroller.
  virtual void DidChangeGlobalRootScroller() {}

  virtual void ContentsResized() {}

  // This is for platform overlay scrollbars only, doesn't include
  // overflow:overlay scrollbars. Probably this should be renamed to
  // HasPlatformOverlayScrollbars() but we don't bother it because
  // overflow:overlay might be deprecated soon.
  bool HasOverlayScrollbars() const;
  void SetOverlayScrollbarColorScheme(mojom::blink::ColorScheme);
  void RecalculateOverlayScrollbarColorScheme();
  mojom::blink::ColorScheme GetOverlayScrollbarColorScheme() const {
    return static_cast<mojom::blink::ColorScheme>(
        overlay_scrollbar_color_scheme__);
  }

  // This getter will create a MacScrollAnimator if it doesn't already exist,
  // only on MacOS.
  MacScrollbarAnimator* GetMacScrollbarAnimator() const;

  // This getter will create a ScrollAnimatorBase if it doesn't already exist.
  ScrollAnimatorBase& GetScrollAnimator() const;

  // This getter will return null if the ScrollAnimatorBase hasn't been created
  // yet.
  ScrollAnimatorBase* ExistingScrollAnimator() const {
    return scroll_animator_.Get();
  }

  ProgrammaticScrollAnimator& GetProgrammaticScrollAnimator() const;
  ProgrammaticScrollAnimator* ExistingProgrammaticScrollAnimator() const {
    return programmatic_scroll_animator_.Get();
  }

  virtual cc::AnimationHost* GetCompositorAnimationHost() const {
    return nullptr;
  }
  virtual cc::AnimationTimeline* GetCompositorAnimationTimeline() const {
    return nullptr;
  }

  // This is used to determine whether the incoming fractional scroll offset
  // should be truncated to integer. Current rule is that if
  // preferCompositingToLCDTextEnabled() is disabled (which is true on low-dpi
  // device by default) we should do the truncation.  The justification is that
  // non-composited elements using fractional scroll offsets is causing too much
  // nasty bugs but does not add too benefit on low-dpi devices.
  // TODO(szager): Now that scroll offsets are floats everywhere, can we get rid
  // of this?
  virtual bool ShouldUseIntegerScrollOffset() const {
    return !RuntimeEnabledFeatures::FractionalScrollOffsetsEnabled();
  }

  virtual bool IsActive() const = 0;
  // Returns true if the frame this ScrollableArea is attached to is being
  // throttled for lifecycle updates. In this case it should also not be
  // painted.
  virtual bool IsThrottled() const = 0;
  virtual int ScrollSize(ScrollbarOrientation) const = 0;
  virtual bool IsScrollCornerVisible() const = 0;
  virtual gfx::Rect ScrollCornerRect() const = 0;
  virtual bool HasTickmarks() const { return false; }
  virtual Vector<gfx::Rect> GetTickmarks() const { return Vector<gfx::Rect>(); }

  // Note that this function just set the scrollbar itself needs repaint by
  // blink during paint, but doesn't set the scrollbar parts (thumb or track)
  // of an accelerated scrollbar needing repaint by the compositor.
  // Use Scrollbar::SetNeedsPaintInvalidation() instead.
  void SetScrollbarNeedsPaintInvalidation(ScrollbarOrientation);

  virtual void SetScrollCornerNeedsPaintInvalidation();

  // Set all scrollbars and their parts and the scroll corner needs full paint
  // invalidation.
  void SetScrollControlsNeedFullPaintInvalidation();

  // Convert points and rects between the scrollbar and its containing
  // EmbeddedContentView. The client needs to implement these in order to be
  // aware of layout effects like CSS transforms.
  virtual gfx::Rect ConvertFromScrollbarToContainingEmbeddedContentView(
      const Scrollbar& scrollbar,
      const gfx::Rect& scrollbar_rect) const {
    gfx::Rect local_rect = scrollbar_rect;
    local_rect.Offset(scrollbar.Location().OffsetFromOrigin());
    return local_rect;
  }
  virtual gfx::Point ConvertFromContainingEmbeddedContentViewToScrollbar(
      const Scrollbar& scrollbar,
      const gfx::Point& parent_point) const {
    NOTREACHED();
  }
  virtual gfx::Point ConvertFromScrollbarToContainingEmbeddedContentView(
      const Scrollbar& scrollbar,
      const gfx::Point& scrollbar_point) const {
    NOTREACHED();
  }
  virtual gfx::Point ConvertFromRootFrame(
      const gfx::Point& point_in_root_frame) const {
    NOTREACHED();
  }
  virtual gfx::Point ConvertFromRootFrameToVisualViewport(
      const gfx::Point& point_in_root_frame) const {
    NOTREACHED();
  }

  virtual Scrollbar* HorizontalScrollbar() const { return nullptr; }
  virtual Scrollbar* VerticalScrollbar() const { return nullptr; }
  virtual Scrollbar* CreateScrollbar(ScrollbarOrientation) { return nullptr; }
  Scrollbar* GetScrollbar(ScrollbarOrientation) const;

  virtual PaintLayer* Layer() const { return nullptr; }

  // "Scroll offset" is in content-flow-aware coordinates, "Scroll position" is
  // in physical (i.e., not flow-aware) coordinates. Among ScrollableArea
  // sub-classes, only PaintLayerScrollableArea has a real distinction between
  // the two. For a more detailed explanation of ScrollPosition, ScrollOffset,
  // and ScrollOrigin, see core/layout/README.md.
  // Note that in Chrome code except for blink renderer, there is no concept of
  // "scroll origin", and the term "scroll offset" has the same definition as
  // "scroll position" here. When a "scroll offset" is requested from outside
  // of blink renderer, we should pass "scroll origin". Similarly, when "scroll
  // offset" is set from outside of blink renderer, we should set "scroll
  // position" here.
  gfx::PointF ScrollPosition() const {
    return ScrollOffsetToPosition(GetScrollOffset());
  }
  virtual gfx::PointF ScrollOffsetToPosition(const ScrollOffset& offset) const {
    return gfx::PointAtOffsetFromOrigin(offset);
  }
  virtual ScrollOffset ScrollPositionToOffset(
      const gfx::PointF& position) const {
    return position.OffsetFromOrigin();
  }
  virtual gfx::Vector2d ScrollOffsetInt() const = 0;
  virtual ScrollOffset GetScrollOffset() const = 0;
  // Returns a floored version of the scroll offset as the web-exposed scroll
  // offset to ensure web compatibility in DOM APIs.
  virtual ScrollOffset GetWebExposedScrollOffset() const;
  ScrollOffset GetScrollOffsetForScrollMarkerUpdate();
  virtual gfx::Vector2d MinimumScrollOffsetInt() const = 0;
  virtual ScrollOffset MinimumScrollOffset() const {
    return ScrollOffset(MinimumScrollOffsetInt());
  }
  virtual gfx::Vector2d MaximumScrollOffsetInt() const = 0;
  virtual ScrollOffset MaximumScrollOffset() const {
    return ScrollOffset(MaximumScrollOffsetInt());
  }

  virtual gfx::Rect VisibleContentRect(
      IncludeScrollbarsInRect = kExcludeScrollbars) const = 0;
  virtual int VisibleHeight() const { return VisibleContentRect().height(); }
  virtual int VisibleWidth() const { return VisibleContentRect().width(); }
  virtual gfx::Size ContentsSize() const = 0;

  // scroll snapport is the area of the scrollport that is used as the alignment
  // container for the scroll snap areas when calculating snap positions. It's
  // the box's scrollport contracted by its scroll-padding.
  // https://drafts.csswg.org/css-scroll-snap-1/#scroll-padding
  virtual PhysicalRect VisibleScrollSnapportRect(
      IncludeScrollbarsInRect scrollbar_inclusion = kExcludeScrollbars) const {
    return PhysicalRect(VisibleContentRect(scrollbar_inclusion));
  }

  virtual gfx::Point LastKnownMousePosition() const { return gfx::Point(); }

  virtual bool ShouldSuspendScrollAnimations() const { return true; }
  virtual bool ScrollbarsCanBeActive() const = 0;

  virtual CompositorElementId GetScrollElementId() const = 0;

  virtual CompositorElementId GetScrollbarElementId(
      ScrollbarOrientation orientation);

  virtual bool ScrollAnimatorEnabled() const { return false; }

  gfx::Vector2d ClampScrollOffset(const gfx::Vector2d&) const;
  ScrollOffset ClampScrollOffset(const ScrollOffset&) const;

  // Let subclasses provide a way of asking for and servicing scroll
  // animations.
  virtual bool ScheduleAnimation() { return false; }
  virtual void ServiceScrollAnimations(double monotonic_time);
  virtual void UpdateCompositorScrollAnimations();
  virtual void RegisterForAnimation() {}
  virtual void DeregisterForAnimation() {}

  // TODO(crbug.com/40517276): Remove this function after launching
  // RasterInducingScroll.
  virtual bool UsesCompositedScrolling() const = 0;
  virtual bool ShouldScrollOnMainThread() const { return false; }

  // Overlay scrollbars can "fade-out" when inactive. This value should only be
  // updated if BlinkControlsOverlayVisibility is true in the
  // ScrollbarTheme. On Mac, where it is false, this can only be updated from
  // the MacScrollbarAnimatorImpl painting code which will do so via
  // SetScrollbarsHiddenFromExternalAnimator.
  virtual bool ScrollbarsHiddenIfOverlay() const;
  void SetScrollbarsHiddenIfOverlay(bool);

  // This should only be called from Mac's painting code.
  void SetScrollbarsHiddenFromExternalAnimator(bool);

  void SetScrollbarsHiddenForTesting(bool);

  virtual bool UserInputScrollable(ScrollbarOrientation) const = 0;
  virtual bool ShouldPlaceVerticalScrollbarOnLeft() const = 0;

  // Convenience functions
  float MinimumScrollOffset(ScrollbarOrientation orientation) {
    return orientation == kHorizontalScrollbar ? MinimumScrollOffset().x()
                                               : MinimumScrollOffset().y();
  }
  float MaximumScrollOffset(ScrollbarOrientation orientation) {
    return orientation == kHorizontalScrollbar ? MaximumScrollOffset().x()
                                               : MaximumScrollOffset().y();
  }
  float ClampScrollOffset(ScrollbarOrientation orientation, float offset) {
    return ClampTo(offset, MinimumScrollOffset(orientation),
                   MaximumScrollOffset(orientation));
  }

  // These methods always return nullptr except for VisualViewport.
  virtual cc::Layer* LayerForHorizontalScrollbar() const { return nullptr; }
  virtual cc::Layer* LayerForVerticalScrollbar() const { return nullptr; }
  virtual cc::Layer* LayerForScrollCorner() const { return nullptr; }
  bool HasLayerForHorizontalScrollbar() const;
  bool HasLayerForVerticalScrollbar() const;
  bool HasLayerForScrollCorner() const;

  bool HorizontalScrollbarNeedsPaintInvalidation() const {
    return horizontal_scrollbar_needs_paint_invalidation_;
  }
  bool VerticalScrollbarNeedsPaintInvalidation() const {
    return vertical_scrollbar_needs_paint_invalidation_;
  }
  bool ScrollCornerNeedsPaintInvalidation() const {
    return scroll_corner_needs_paint_invalidation_;
  }

  void CancelScrollAnimation();
  virtual void CancelProgrammaticScrollAnimation();

  virtual ~ScrollableArea();

  void Dispose();
  virtual void DisposeImpl() {}

  // Called when any of horizontal scrollbar, vertical scrollbar and scroll
  // corner is setNeedsPaintInvalidation.
  virtual void ScrollControlWasSetNeedsPaintInvalidation() = 0;

  // Returns the default scroll style this area should scroll with when not
  // explicitly specified. E.g. The scrolling behavior of an element can be
  // specified in CSS.
  virtual mojom::blink::ScrollBehavior ScrollBehaviorStyle() const {
    return mojom::blink::ScrollBehavior::kInstant;
  }

  virtual mojom::blink::ColorScheme UsedColorSchemeScrollbars() const = 0;

  // Subtracts space occupied by this ScrollableArea's scrollbars.
  // Does nothing if overlay scrollbars are enabled.
  gfx::Size ExcludeScrollbars(const gfx::Size&) const;

  virtual int VerticalScrollbarWidth(
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize) const;
  virtual int HorizontalScrollbarHeight(
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize) const;

  virtual LayoutBox* GetLayoutBox() const { return nullptr; }

  // Maps a quad from the coordinate system of a LayoutObject contained by the
  // ScrollableArea to the coordinate system of the ScrollableArea's visible
  // content rect.  If the LayoutObject* argument is null, the argument quad is
  // considered to be in the coordinate space of the overflow rect.
  virtual gfx::QuadF LocalToVisibleContentQuad(const gfx::QuadF&,
                                               const LayoutObject*,
                                               unsigned = 0) const;

  virtual bool IsPaintLayerScrollableArea() const { return false; }
  virtual bool IsRootFrameViewport() const { return false; }

  // Returns true if this is the layout viewport associated with the
  // RootFrameViewport.
  virtual bool IsRootFrameLayoutViewport() const { return false; }

  virtual bool VisualViewportSuppliesScrollbars() const { return false; }

  // Returns true if the scroller adjusts the scroll offset to compensate
  // for layout movements (bit.ly/scroll-anchoring).
  virtual bool ShouldPerformScrollAnchoring() const { return false; }

  void Trace(Visitor*) const override;

  virtual void ClearScrollableArea();

  virtual bool RestoreScrollAnchor(const SerializedAnchor&) { return false; }
  virtual ScrollAnchor* GetScrollAnchor() { return nullptr; }

  virtual void DidScrollWithScrollbar(ScrollbarPart,
                                      ScrollbarOrientation,
                                      WebInputEvent::Type) {}

  // Returns the task runner to be used for scrollable area timers.
  // Ideally a frame-specific throttled one can be used.
  virtual scoped_refptr<base::SingleThreadTaskRunner> GetTimerTaskRunner()
      const = 0;

  // Callback for compositor-side scrolling.
  virtual void DidCompositorScroll(const gfx::PointF& position);

  virtual void ScrollbarFrameRectChanged() {}

  virtual ScrollbarTheme& GetPageScrollbarTheme() const = 0;

  void OnScrollFinished(bool enqueue_scrollend);

  float ScrollStep(ui::ScrollGranularity, ScrollbarOrientation) const;

  // Injects a gesture scroll event based on the given parameters for mouse
  // events on a scrollbar of this scrollable area.
  void InjectScrollbarGestureScroll(ScrollOffset delta,
                                    ui::ScrollGranularity granularity,
                                    WebInputEvent::Type gesture_type) const;
  // If the layout box is a global root scroller then the root frame view's
  // ScrollableArea is returned. Otherwise, the layout box's
  // PaintLayerScrollableArea (which can be null) is returned.
  static ScrollableArea* GetForScrolling(const LayoutBox* layout_box);

  // Returns a Node at which 'scroll' events should be dispatched.
  // For <fieldset>, a ScrollableArea is associated to its internal anonymous
  // box. GetLayoutBox()->GetNode() doesn't work in this case.
  Node* EventTargetNode() const;

  ScrollOffset PendingScrollAnchorAdjustment() const;
  void ClearPendingScrollAnchorAdjustment();

  scoped_refptr<base::SingleThreadTaskRunner> GetCompositorTaskRunner();
  void EnqueueScrollSnapChangeEvent() const;

  ScrollOffset ScrollOffsetFromScrollStartData(
      const ScrollStartData& block_value,
      const ScrollStartData& inline_value) const;
  void ApplyScrollStart();
  bool ScrollStartIsDefault() const;
  virtual bool IsApplyingScrollStart() const { return false; }

  virtual void SetScrollsnapchangeTargetIds(
      std::optional<cc::TargetSnapAreaElementIds>) {}
  virtual void UpdateSnappedTargetsAndEnqueueScrollSnapChange() {}

  bool ScrollOffsetIsNoop(const ScrollOffset& offset) const;

  void EnqueueScrollSnapChangingEvent() const;
  virtual std::optional<cc::TargetSnapAreaElementIds>
  GetScrollsnapchangingTargetIds() const {
    return std::nullopt;
  }
  virtual void SetScrollsnapchangingTargetIds(
      std::optional<cc::TargetSnapAreaElementIds>) {}
  virtual void UpdateScrollSnapChangingTargetsAndEnqueueScrollSnapChanging(
      const cc::TargetSnapAreaElementIds& ids) {}
  virtual const cc::SnapSelectionStrategy* GetImplSnapStrategy() const {
    return nullptr;
  }
  virtual void SetImplSnapStrategy(std::unique_ptr<cc::SnapSelectionStrategy>) {
  }
  virtual void EnqueueScrollSnapChangingEventFromImplIfNeeded() {}

  virtual std::optional<cc::ElementId> GetTargetedSnapAreaId() {
    return std::nullopt;
  }
  virtual void SetTargetedSnapAreaId(const std::optional<cc::ElementId>&) {}

  virtual void DropCompositorScrollDeltaNextCommit() {}

  virtual void SetSnappedQueryTargetIds(
      std::optional<cc::TargetSnapAreaElementIds>) {}
  virtual bool IsGlobalRootNonOverlayScroller() const { return false; }

  virtual ScrollMarkerGroupPseudoElement* GetScrollMarkerGroup() const {
    return nullptr;
  }

  virtual void UpdateScrollMarkers() {}

 protected:
  // Deduces the mojom::blink::ScrollBehavior based on the
  // element style and the parameter set by programmatic scroll into either
  // instant or smooth scroll.
  static mojom::blink::ScrollBehavior DetermineScrollBehavior(
      mojom::blink::ScrollBehavior behavior_from_style,
      mojom::blink::ScrollBehavior behavior_from_param);

  explicit ScrollableArea(
      scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner);

  ScrollbarOrientation ScrollbarOrientationFromDirection(
      ScrollDirectionPhysical) const;

  // Needed to let the animators call scrollOffsetChanged.
  friend class ScrollAnimatorCompositorCoordinator;
  void ScrollOffsetChanged(const ScrollOffset&, mojom::blink::ScrollType);

  void ClearNeedsPaintInvalidationForScrollControls() {
    horizontal_scrollbar_needs_paint_invalidation_ = false;
    vertical_scrollbar_needs_paint_invalidation_ = false;
    scroll_corner_needs_paint_invalidation_ = false;
  }

  void ShowNonMacOverlayScrollbars();

  // Called when scrollbar hides/shows for overlay scrollbars. This callback
  // shouldn't do any significant work as it can be called unexpectedly often
  // on Mac. This happens because painting code has to set alpha to 1, paint,
  // then reset to alpha, causing spurious "visibilityChanged" calls.
  virtual void ScrollbarVisibilityChanged() {}

  bool HasBeenDisposed() const { return has_been_disposed_; }

  virtual const Document* GetDocument() const;

  // Resolves into un-zoomed physical pixels a scroll |delta| based on its
  // ScrollGranularity units.
  ScrollOffset ResolveScrollDelta(ui::ScrollGranularity,
                                  const ScrollOffset& delta);

  virtual void StopApplyingScrollStart() {}
  const LayoutObject* GetScrollInitialTarget() const;

  virtual Node* GetSnapEventTargetAlongAxis(const AtomicString& type,
                                            cc::SnapAxis) const {
    return nullptr;
  }

 private:
  FRIEND_TEST_ALL_PREFIXES(ScrollableAreaTest,
                           PopupOverlayScrollbarShouldNotFadeOut);
  FRIEND_TEST_ALL_PREFIXES(ScrollableAreaTest,
                           FilterIncomingScrollDuringSmoothUserScroll);

  void SetScrollbarsHiddenIfOverlayInternal(bool);

  bool ProgrammaticScrollHelper(const ScrollOffset&,
                                mojom::blink::ScrollBehavior,
                                gfx::Vector2d animation_adjustment,
                                ScrollCallback on_finish);
  void UserScrollHelper(const ScrollOffset&, mojom::blink::ScrollBehavior);

  void FadeOverlayScrollbarsTimerFired(TimerBase*);

  // This function should be overridden by subclasses to perform the actual
  // scroll of the content.
  virtual void UpdateScrollOffset(const ScrollOffset&,
                                  mojom::blink::ScrollType) = 0;

  float ScrollStartValueToOffsetAlongAxis(const ScrollStartData&,
                                          cc::SnapAxis) const;

  virtual int LineStep(ScrollbarOrientation) const;
  virtual int PageStep(ScrollbarOrientation) const;
  virtual int DocumentStep(ScrollbarOrientation) const;
  virtual float PixelStep(ScrollbarOrientation) const;

  // Returns true if a snap point was found.
  bool PerformSnapping(
      const cc::SnapSelectionStrategy& strategy,
      mojom::blink::ScrollBehavior behavior =
          mojom::blink::ScrollBehavior::kSmooth,
      base::ScopedClosureRunner on_finish = base::ScopedClosureRunner(),
      bool preserve_pinned_marker = false);

  void ScrollToScrollInitialTarget(const LayoutObject*);

  bool ShouldFilterIncomingScroll(mojom::blink::ScrollType incoming_type) {
    auto old_type = active_smooth_scroll_type_;

    // Allow the incoming scroll to co-exist if its scroll type is
    // kClamping, or kAnchoring.
    if (incoming_type == mojom::blink::ScrollType::kClamping ||
        incoming_type == mojom::blink::ScrollType::kAnchoring) {
      return false;
    }
    // If the current smooth scroll is a kUser scroll, i.e. a smooth scroll
    // triggered by find-in-page, filter the incoming scroll unless it is:
    // - another find-in-page scroll (kUser),
    // - a gesture scroll (kCompositor), or
    // - an update from the current find-in-page scroll animation running on the
    //   compositor (kCompositor).
    // See crbug.com/913009, crbug.com/365493092 for more details.
    if (old_type == mojom::blink::ScrollType::kUser &&
        incoming_type != mojom::blink::ScrollType::kUser &&
        incoming_type != mojom::blink::ScrollType::kCompositor) {
      return true;
    }
    // TODO(crbug.com/325081538, crbug.com/342093060): Ideally, if the incoming
    // scroll is a gesture scroll we'd cancel the current animation here.
    // But to do that, we must be able to distinguish between compositor updates
    // due to gesture scrolls from compositor updates due to impl-ticked
    // programmatic scrolls. So we'd need to:
    //   - split kCompositor ScrollType into kCompositorUser and
    //     kCompositorProgrammatic and
    //   - pass the ScrollType from the compositor to the main thread.
    return false;
  }

  void set_active_smooth_scroll_type_for_testing(
      mojom::blink::ScrollType type) {
    active_smooth_scroll_type_ = type;
  }

  // This animator is used to handle painting animations for MacOS scrollbars
  // using AppKit-specific code (Cocoa APIs). It requires input from
  // ScrollableArea about changes on scrollbars. For other platforms, painting
  // is done by blink, and this member will be a nullptr.
  mutable Member<MacScrollbarAnimator> mac_scrollbar_animator_;

  mutable Member<ScrollAnimatorBase> scroll_animator_;
  mutable Member<ProgrammaticScrollAnimator> programmatic_scroll_animator_;

  Member<DisallowNewWrapper<HeapTaskRunnerTimer<ScrollableArea>>>
      fade_overlay_scrollbars_timer_;

  Vector<ScrollCallback> pending_scroll_complete_callbacks_;

  ScrollOffset pending_scroll_anchor_adjustment_;

  unsigned overlay_scrollbar_color_scheme__ : 2;

  unsigned horizontal_scrollbar_needs_paint_invalidation_ : 1;
  unsigned vertical_scrollbar_needs_paint_invalidation_ : 1;
  unsigned scroll_corner_needs_paint_invalidation_ : 1;
  unsigned scrollbars_hidden_if_overlay_ : 1;
  unsigned scrollbar_captured_ : 1;
  unsigned mouse_over_scrollbar_ : 1;
  unsigned has_been_disposed_ : 1;

  std::optional<mojom::blink::ScrollType> active_smooth_scroll_type_;

  scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLABLE_AREA_H_
