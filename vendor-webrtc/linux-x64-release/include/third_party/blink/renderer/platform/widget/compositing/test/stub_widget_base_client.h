// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_TEST_STUB_WIDGET_BASE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_TEST_STUB_WIDGET_BASE_CLIENT_H_

#include "cc/input/overscroll_behavior.h"
#include "cc/trees/layer_tree_frame_sink.h"
#include "components/viz/common/frame_sinks/begin_frame_args.h"
#include "third_party/blink/public/common/input/web_gesture_event.h"
#include "third_party/blink/public/common/input/web_mouse_event.h"
#include "third_party/blink/public/common/widget/visual_properties.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"
#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/public/web/web_lifecycle_update.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/widget/widget_base_client.h"
#include "ui/display/screen_infos.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {
class StubWidgetBaseClient : public WidgetBaseClient {
 public:
  void OnCommitRequested() override {}
  void BeginMainFrame(const viz::BeginFrameArgs& args) override {}
  void UpdateLifecycle(WebLifecycleUpdate, DocumentUpdateReason) override {}
  std::unique_ptr<cc::LayerTreeFrameSink> AllocateNewLayerTreeFrameSink()
      override {
    return nullptr;
  }
  KURL GetURLForDebugTrace() override { return {}; }
  WebInputEventResult DispatchBufferedTouchEvents() override {
    return WebInputEventResult::kNotHandled;
  }
  WebInputEventResult HandleInputEvent(const WebCoalescedInputEvent&) override {
    return WebInputEventResult::kNotHandled;
  }
  bool SupportsBufferedTouchEvents() override { return false; }
  void WillHandleGestureEvent(const WebGestureEvent&, bool* suppress) override {
  }
  void WillHandleMouseEvent(const WebMouseEvent&) override {}
  void ObserveGestureEventAndResult(const WebGestureEvent&,
                                    const gfx::Vector2dF&,
                                    const cc::OverscrollBehavior&,
                                    bool) override {}
  void FocusChanged(mojom::blink::FocusState) override {}
  void UpdateVisualProperties(
      const VisualProperties& visual_properties) override {}
  const display::ScreenInfos& GetOriginalScreenInfos() override {
    return screen_infos_;
  }
  gfx::Rect ViewportVisibleRect() override { return gfx::Rect(); }

 private:
  display::ScreenInfos screen_infos_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_TEST_STUB_WIDGET_BASE_CLIENT_H_
