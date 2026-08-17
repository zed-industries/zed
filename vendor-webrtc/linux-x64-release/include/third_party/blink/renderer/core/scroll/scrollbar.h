/*
 * Copyright (C) 2004, 2006 Apple Computer, Inc.  All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_H_

#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/style_scrollbar_color.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "ui/events/types/scroll_types.h"

namespace gfx {
class Rect;
}

namespace ui {
class ColorProvider;
}

namespace blink {

class LayoutBox;
class LayoutObject;
class ScrollableArea;
class ScrollbarTheme;
class WebGestureEvent;
class WebMouseEvent;
class WebPointerEvent;

class CORE_EXPORT Scrollbar : public GarbageCollected<Scrollbar>,
                              public DisplayItemClient {
 public:
  // Theme object ownership remains with the caller and it must outlive the
  // scrollbar.
  static Scrollbar* CreateForTesting(ScrollableArea* scrollable_area,
                                     ScrollbarOrientation orientation,
                                     ScrollbarTheme* theme);

  Scrollbar(ScrollableArea*,
            ScrollbarOrientation,
            const LayoutObject* style_source,
            ScrollbarTheme* = nullptr);
  ~Scrollbar() override;

  int X() const { return frame_rect_.x(); }
  int Y() const { return frame_rect_.y(); }
  int Width() const { return frame_rect_.width(); }
  int Height() const { return frame_rect_.height(); }
  gfx::Size Size() const { return frame_rect_.size(); }
  gfx::Point Location() const { return frame_rect_.origin(); }

  void SetFrameRect(const gfx::Rect&);
  const gfx::Rect& FrameRect() const { return frame_rect_; }

  bool HasTickmarks() const;
  Vector<gfx::Rect> GetTickmarks() const;
  bool IsScrollableAreaActive() const;

  gfx::Point ConvertFromRootFrame(const gfx::Point&) const;

  virtual bool IsCustomScrollbar() const { return false; }
  ScrollbarOrientation Orientation() const { return orientation_; }
  bool IsLeftSideVerticalScrollbar() const;

  int Value() const { return static_cast<int>(lroundf(current_pos_)); }
  float CurrentPos() const { return current_pos_; }
  int VisibleSize() const { return visible_size_; }
  int TotalSize() const { return total_size_; }
  int Maximum() const;

  ScrollbarPart PressedPart() const { return pressed_part_; }
  ScrollbarPart HoveredPart() const { return hovered_part_; }

  virtual void StyleChanged() {}
  void SetScrollbarsHiddenFromExternalAnimator(bool);
  bool Enabled() const { return enabled_; }
  virtual void SetEnabled(bool);

  int ScrollbarThickness() const;

  // Called by the ScrollableArea when the scroll offset changes.
  // Will trigger paint invalidation if required.
  virtual void OffsetDidChange(mojom::blink::ScrollType scroll_type);

  virtual void DisconnectFromScrollableArea();

  int PressedPos() const { return pressed_pos_; }

  virtual void SetHoveredPart(ScrollbarPart);
  virtual void SetPressedPart(ScrollbarPart, WebInputEvent::Type);

  void SetProportion(int visible_size, int total_size);
  void SetPressedPos(int p) { pressed_pos_ = p; }

  virtual bool IsSolidColor() const;

  // Returns true if the scrollbar is a overlay scrollbar. This doesn't include
  // overflow:overlay scrollbars. Probably this should be renamed to
  // IsPlatformOverlayScrollbar() but we don't bother it because
  // overflow:overlay might be deprecated soon.
  virtual bool IsOverlayScrollbar() const;
  virtual bool IsFluentOverlayScrollbarMinimalMode() const;

  // Returns `true` if the scrollbar bounds are larger than the canvas'. In
  // this scenario, the scrollbar scaling will be done by using nine-patch
  // scaling in the compositor thread.
  // If the scrollbar's thickness is being affected (height for horizontal
  // scrollbars, width for vertical), the function returns `false` as scrollbars
  // will need to re-paint the arrows.
  bool UsesNinePatchTrackAndCanSkipRepaint(
      const gfx::Rect& new_frame_rect) const;

  bool ShouldParticipateInHitTesting();

  bool IsWindowActive() const;

  // Return if the gesture event (tap/press) was handled.
  bool HandleGestureTapOrPress(const WebGestureEvent&);

  bool HandlePointerEvent(const WebPointerEvent&);

  // These methods are used for platform scrollbars to give :hover feedback.
  // They will not get called when the mouse went down in a scrollbar, since it
  // is assumed the scrollbar will start
  // grabbing all events in that case anyway.
  void MouseMoved(const WebMouseEvent&);
  void MouseEntered();
  void MouseExited();

  // Used by some platform scrollbars to know when they've been released from
  // capture.
  void MouseUp(const WebMouseEvent&);
  void MouseDown(const WebMouseEvent&);

  ScrollbarTheme& GetTheme() const { return theme_; }

  gfx::Rect ConvertToContainingEmbeddedContentView(const gfx::Rect&) const;
  gfx::Point ConvertFromContainingEmbeddedContentView(const gfx::Point&) const;

  void MoveThumb(int pos, bool dragging_document = false);

  float ElasticOverscroll() const { return elastic_overscroll_; }
  void SetElasticOverscroll(float elastic_overscroll) {
    elastic_overscroll_ = elastic_overscroll;
  }

  // Use SetNeedsPaintInvalidation to cause the scrollbar (or parts thereof)
  // to repaint.
  bool TrackAndButtonsNeedRepaint() const {
    return track_and_buttons_need_repaint_;
  }
  void ClearTrackAndButtonsNeedRepaint() {
    track_and_buttons_need_repaint_ = false;
  }
  bool ThumbNeedsRepaint() const { return thumb_needs_repaint_; }
  void ClearThumbNeedsRepaint() { thumb_needs_repaint_ = false; }

  // Returns true if either the track or the thumb needs repaint, or the thumb
  // moved (which doesn't need to repaint the track or the thumb in some
  // scrollbar themes).
  bool NeedsUpdateDisplay() const { return needs_update_display_; }
  void ClearNeedsUpdateDisplay() { needs_update_display_ = false; }

  // DisplayItemClient.
  String DebugName() const final {
    return orientation_ == kHorizontalScrollbar ? "HorizontalScrollbar"
                                                : "VerticalScrollbar";
  }

  // Marks the scrollbar as needing to be redrawn.
  //
  // If invalid parts are provided, then those parts will also be repainted.
  // Otherwise, the ScrollableArea may redraw using cached renderings of
  // individual parts. For instance, if the scrollbar is composited, the thumb
  // may be cached in a GPU texture (and is only guaranteed to be repainted if
  // ThumbPart is invalidated).
  //
  // Even if no parts are invalidated, the scrollbar may need to be redrawn
  // if, for instance, the thumb moves without changing the appearance of any
  // part.
  void SetNeedsPaintInvalidation(ScrollbarPart invalid_parts);

  CompositorElementId GetElementId() const;

  // Used to scale a length in dip units into a length in layout/paint units.
  float ScaleFromDIP() const;

  float EffectiveZoom() const;
  bool ContainerIsRightToLeft() const;
  bool ContainerIsFormControl() const;

  // scrollbar-width CSS property
  EScrollbarWidth CSSScrollbarWidth() const;
  // scrollbar-color CSS property
  std::optional<blink::Color> ScrollbarThumbColor() const;
  std::optional<blink::Color> ScrollbarTrackColor() const;

  virtual bool IsOpaque() const;

  // The LayoutObject that supplies our style information. If the scrollbar is
  // for a document, this is:
  // 1. the LayoutView (with some scrollbar related styles propagated from the
  //    document element and/or the <body>), or
  // 2. the <body> or document element's layout object if it has webkit custom
  //    scrollbar styles.
  // Otherwise, it is the LayoutBox that owns our PaintLayerScrollableArea.
  const LayoutObject* StyleSource() const { return style_source_.Get(); }

  mojom::blink::ColorScheme UsedColorScheme() const;

  void Trace(Visitor*) const override;

  LayoutBox* GetLayoutBox() const;
  bool IsScrollCornerVisible() const;
  bool ShouldPaint() const;
  bool LastKnownMousePositionInFrameRect() const;

  // Returns the color provider for this scrollbar.
  const ui::ColorProvider* GetColorProvider(mojom::blink::ColorScheme) const;
  // Returns the forced colors state for this scrollbar.
  bool InForcedColorsMode() const;

 protected:
  void AutoscrollTimerFired(TimerBase*);
  void StartTimerIfNeeded(base::TimeDelta delay);
  void StopTimerIfNeeded();
  void AutoscrollPressedPart(base::TimeDelta delay);
  bool HandleTapGesture();
  void InjectScrollGestureForPressedPart(WebInputEvent::Type gesture_type);
  void InjectGestureScrollUpdateForThumbMove(float single_axis_target_offset);
  void InjectScrollGesture(WebInputEvent::Type type,
                           ScrollOffset delta,
                           ui::ScrollGranularity granularity);
  ScrollDirectionPhysical PressedPartScrollDirectionPhysical();
  ui::ScrollGranularity PressedPartScrollGranularity();

  Member<ScrollableArea> scrollable_area_;
  ScrollbarOrientation orientation_;
  ScrollbarTheme& theme_;

  int visible_size_;
  int total_size_;
  float current_pos_;
  float drag_origin_;

  ScrollbarPart hovered_part_;
  ScrollbarPart pressed_part_;
  int pressed_pos_;
  float scroll_pos_;
  bool dragging_document_;
  int document_drag_pos_;

  bool enabled_;

  HeapTaskRunnerTimer<Scrollbar> scroll_timer_;

  float elastic_overscroll_;

 private:
  float ScrollableAreaCurrentPos() const;
  float ScrollableAreaTargetPos() const;
  bool ThumbWillBeUnderMouse() const;
  bool DeltaWillScroll(ScrollOffset delta) const;

  // Theme color set as a web pref that will only be applied to root scrollbars
  // when no other modification is present (high contrast or css styling).
  std::optional<blink::Color> RootScrollbarThemeColor() const;

  bool track_and_buttons_need_repaint_ = true;
  bool thumb_needs_repaint_ = true;
  bool needs_update_display_ = true;

  bool injected_gesture_scroll_begin_;

  // This is set based on the event modifiers. In scenarios like scrolling or
  // layout, the element that the cursor is over can change without the cursor
  // itself moving. In these cases, a "fake" mouse move may be dispatched (see
  // MouseEventManager::RecomputeMouseHoverState) in order to apply hover etc.
  // Such mouse events do not have the modifier set and hence, maintaining this
  // additional state is necessary.
  bool scrollbar_manipulation_in_progress_on_cc_thread_;

  gfx::Rect frame_rect_;
  WeakMember<const LayoutObject> style_source_;

  // Tracks scroll delta that has been injected into the compositor thread as a
  // GestureScrollUpdate but hasn't yet updated the scroll position on main.
  // Scrollbar::MouseMoved needs this to calculate deltas during thumb drags.
  // In particular we often process two mousemoves in the same frame thanks to
  // MouseEventManager::RecomputeMouseHoverState sending fake ones.
  ScrollOffset pending_injected_delta_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_H_
