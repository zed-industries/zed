// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_TEST_SUITE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_TEST_SUITE_H_

#include "base/task/single_thread_task_runner.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/platform/platform.h"
#include "third_party/blink/public/platform/scheduler/test/renderer_scheduler_test_support.h"
#include "third_party/blink/renderer/core/loader/empty_clients.h"
#include "third_party/blink/renderer/core/scroll/scrollable_area.h"
#include "third_party/blink/renderer/core/scroll/scrollbar.h"
#include "third_party/blink/renderer/core/scroll/scrollbar_theme.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_scheduler.h"
#include "ui/gfx/geometry/vector2d_conversions.h"

namespace blink {

class MockPlatformChromeClient : public EmptyChromeClient {
 public:
  MockPlatformChromeClient() : is_popup_(false) {}

  bool IsPopup() override { return is_popup_; }

  void SetIsPopup(bool is_popup) { is_popup_ = is_popup; }

  float WindowToViewportScalar(LocalFrame*, const float) const override {
    return 0;
  }

 private:
  bool is_popup_;
};

class MockScrollableArea : public GarbageCollected<MockScrollableArea>,
                           public ScrollableArea {
 public:
  static MockScrollableArea* Create() {
    return MakeGarbageCollected<MockScrollableArea>();
  }

  static MockScrollableArea* Create(const ScrollOffset& maximum_scroll_offset) {
    MockScrollableArea* mock = Create();
    mock->SetMaximumScrollOffset(maximum_scroll_offset);
    return mock;
  }

  static MockScrollableArea* Create(const ScrollOffset& maximum_scroll_offset,
                                    const ScrollOffset& minimum_scroll_offset) {
    MockScrollableArea* mock = Create();
    mock->SetMaximumScrollOffset(maximum_scroll_offset);
    mock->SetMinimumScrollOffset(minimum_scroll_offset);
    return mock;
  }

  explicit MockScrollableArea()
      : ScrollableArea(blink::scheduler::GetSingleThreadTaskRunnerForTesting()),
        maximum_scroll_offset_(ScrollOffset(0, 100)),
        chrome_client_(MakeGarbageCollected<MockPlatformChromeClient>()) {}
  explicit MockScrollableArea(const ScrollOffset& offset)
      : ScrollableArea(blink::scheduler::GetSingleThreadTaskRunnerForTesting()),
        maximum_scroll_offset_(offset),
        chrome_client_(MakeGarbageCollected<MockPlatformChromeClient>()) {}

  MOCK_CONST_METHOD0(IsActive, bool());
  MOCK_CONST_METHOD0(IsThrottled, bool());
  MOCK_CONST_METHOD1(ScrollSize, int(ScrollbarOrientation));
  MOCK_CONST_METHOD0(IsScrollCornerVisible, bool());
  MOCK_CONST_METHOD0(ScrollCornerRect, gfx::Rect());
  MOCK_CONST_METHOD0(EnclosingScrollableArea, ScrollableArea*());
  MOCK_CONST_METHOD1(VisibleContentRect, gfx::Rect(IncludeScrollbarsInRect));
  MOCK_CONST_METHOD0(ContentsSize, gfx::Size());
  MOCK_CONST_METHOD0(LayerForHorizontalScrollbar, cc::Layer*());
  MOCK_CONST_METHOD0(LayerForVerticalScrollbar, cc::Layer*());
  MOCK_CONST_METHOD0(HorizontalScrollbar, Scrollbar*());
  MOCK_CONST_METHOD0(VerticalScrollbar, Scrollbar*());
  MOCK_CONST_METHOD0(ScrollbarsHiddenIfOverlay, bool());
  MOCK_METHOD0(ScheduleAnimation, bool());
  MOCK_CONST_METHOD0(UsedColorSchemeScrollbars, mojom::blink::ColorScheme());
  MOCK_CONST_METHOD0(UsesCompositedScrolling, bool());

  PhysicalOffset LocalToScrollOriginOffset() const override { return {}; }
  bool UserInputScrollable(ScrollbarOrientation) const override { return true; }
  bool ScrollbarsCanBeActive() const override { return true; }
  bool ShouldPlaceVerticalScrollbarOnLeft() const override { return false; }
  void UpdateScrollOffset(const ScrollOffset& offset,
                          mojom::blink::ScrollType) override {
    scroll_offset_ = offset;
    scroll_offset_.SetToMin(maximum_scroll_offset_);
  }
  gfx::Vector2d ScrollOffsetInt() const override {
    return SnapScrollOffsetToPhysicalPixels(scroll_offset_);
  }
  ScrollOffset GetScrollOffset() const override {
    return ScrollOffset(ScrollOffsetInt());
  }
  gfx::Vector2d MinimumScrollOffsetInt() const override {
    return gfx::ToFlooredVector2d(minimum_scroll_offset_);
  }
  gfx::Vector2d MaximumScrollOffsetInt() const override {
    return gfx::ToFlooredVector2d(maximum_scroll_offset_);
  }
  int VisibleHeight() const override { return 768; }
  int VisibleWidth() const override { return 1024; }
  CompositorElementId GetScrollElementId() const override {
    return CompositorElementId();
  }
  bool ScrollAnimatorEnabled() const override { return true; }
  int PageStep(ScrollbarOrientation) const override { return 0; }
  void ScrollControlWasSetNeedsPaintInvalidation() override {}
  gfx::Point ConvertFromRootFrame(
      const gfx::Point& point_in_root_frame) const override {
    return point_in_root_frame;
  }
  gfx::Point ConvertFromContainingEmbeddedContentViewToScrollbar(
      const Scrollbar& scrollbar,
      const gfx::Point& parent_point) const override {
    return parent_point;
  }

  scoped_refptr<base::SingleThreadTaskRunner> GetTimerTaskRunner() const final {
    return blink::scheduler::GetSingleThreadTaskRunnerForTesting();
  }

  ChromeClient* GetChromeClient() const override {
    return chrome_client_.Get();
  }

  void SetIsPopup() { chrome_client_->SetIsPopup(true); }

  ScrollbarTheme& GetPageScrollbarTheme() const override {
    return ScrollbarTheme::GetTheme();
  }

  float ScaleFromDIP() const override { return scale_from_dip_; }
  void SetScaleFromDIP(float scale_from_dip) {
    scale_from_dip_ = scale_from_dip;
  }

  using ScrollableArea::ClearNeedsPaintInvalidationForScrollControls;
  using ScrollableArea::HorizontalScrollbarNeedsPaintInvalidation;
  using ScrollableArea::ShowNonMacOverlayScrollbars;
  using ScrollableArea::VerticalScrollbarNeedsPaintInvalidation;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(chrome_client_);
    ScrollableArea::Trace(visitor);
  }

 protected:
  void SetMaximumScrollOffset(const ScrollOffset& maximum_scroll_offset) {
    maximum_scroll_offset_ = maximum_scroll_offset;
  }
  void SetMinimumScrollOffset(const ScrollOffset& minimum_scroll_offset) {
    minimum_scroll_offset_ = minimum_scroll_offset;
  }

 private:
  ScrollOffset scroll_offset_;
  ScrollOffset maximum_scroll_offset_;
  ScrollOffset minimum_scroll_offset_;
  Member<MockPlatformChromeClient> chrome_client_;
  float scale_from_dip_ = 1.f;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLLBAR_TEST_SUITE_H_
