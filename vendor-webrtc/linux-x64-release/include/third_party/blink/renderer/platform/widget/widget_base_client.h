// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_WIDGET_BASE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_WIDGET_BASE_CLIENT_H_

#include <vector>

#include "base/time/time.h"
#include "cc/metrics/begin_main_frame_metrics.h"
#include "cc/metrics/frame_sequence_tracker_collection.h"
#include "cc/paint/element_id.h"
#include "cc/trees/layer_tree_host_client.h"
#include "services/viz/public/mojom/hit_test/input_target_client.mojom-blink.h"
#include "third_party/blink/public/common/metrics/document_update_reason.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"
#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/public/platform/web_text_input_type.h"
#include "third_party/blink/public/web/web_lifecycle_update.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/widget/input/input_handler_proxy.h"
#include "ui/base/mojom/menu_source_type.mojom-blink-forward.h"
#include "ui/display/mojom/screen_orientation.mojom-blink.h"

namespace cc {
class LayerTreeFrameSink;
struct BeginMainFrameMetrics;
}  // namespace cc

namespace blink {

class FrameWidget;
class WebGestureEvent;
class WebMouseEvent;

// This class is part of the foundation of all widgets. It provides
// callbacks from the compositing infrastructure that the individual widgets
// will need to implement.
class WidgetBaseClient {
 public:
  // Called when a compositing update is first requested.
  virtual void OnCommitRequested() {}

  // Called to record the time taken to dispatch rAF aligned input.
  virtual void RecordDispatchRafAlignedInputTime(
      base::TimeTicks raf_aligned_input_start_time) {}

  // Called to update the document lifecycle, advance the state of animations
  // and dispatch rAF.
  virtual void BeginMainFrame(const viz::BeginFrameArgs& args) = 0;

  // Requests that the lifecycle of the widget be updated.
  virtual void UpdateLifecycle(WebLifecycleUpdate requested_update,
                               DocumentUpdateReason reason) = 0;

  // TODO(crbug.com/704763): Remove the need for this.
  virtual void SetSuppressFrameRequestsWorkaroundFor704763Only(bool) {}

  // Called when main frame metrics are desired. The local frame's UKM
  // aggregator must be informed that collection is starting for the
  // frame.
  virtual void RecordStartOfFrameMetrics() {}

  // Called when a main frame time metric should be emitted, along with
  // any metrics that depend upon the main frame total time.
  virtual void RecordEndOfFrameMetrics(
      base::TimeTicks frame_begin_time,
      cc::ActiveFrameSequenceTrackers trackers) {}

  // Return metrics information for the stages of BeginMainFrame. This is
  // ultimately implemented by Blink's LocalFrameUKMAggregator. It must be a
  // distinct call from the FrameMetrics above because the BeginMainFrameMetrics
  // for compositor latency must be gathered before the layer tree is
  // committed to the compositor, which is before the call to
  // RecordEndOfFrameMetrics.
  virtual std::unique_ptr<cc::BeginMainFrameMetrics>
  GetBeginMainFrameMetrics() {
    return nullptr;
  }

  // Methods called to mark the beginning and end of the
  // LayerTreeHost::UpdateLayers method. Only called when gathering main frame
  // UMA and UKM. That is, when RecordStartOfFrameMetrics has been called, and
  // before RecordEndOfFrameMetrics has been called.
  virtual void BeginUpdateLayers() {}
  virtual void EndUpdateLayers() {}

  // Methods called to mark the beginning and end of a commit to the impl
  // thread for a frame. Only called when gathering main frame
  // UMA and UKM. That is, when RecordStartOfFrameMetrics has been called, and
  // before RecordEndOfFrameMetrics has been called.
  virtual void BeginCommitCompositorFrame() {}
  virtual void EndCommitCompositorFrame(base::TimeTicks commit_start_time,
                                        base::TimeTicks commit_finish_time) {}

  // Applies viewport related properties during a commit from the compositor
  // thread.
  virtual void ApplyViewportChanges(const cc::ApplyViewportChangesArgs& args) {}

  virtual void UpdateCompositorScrollState(
      const cc::CompositorCommitData& commit_data) {}

  // Notifies that the layer tree host has completed a call to
  // RequestMainFrameUpdate in response to a BeginMainFrame.
  virtual void DidBeginMainFrame() {}
  virtual void DidCommitAndDrawCompositorFrame() {}

  virtual void DidObserveFirstScrollDelay(
      base::TimeDelta first_scroll_delay,
      base::TimeTicks first_scroll_timestamp) {}

  virtual void WillBeginMainFrame() {}
  virtual void DidCompletePageScaleAnimation() {}

  // Allocates a LayerTreeFrameSink to submit CompositorFrames to. Only
  // override this method if you wish to provide your own implementation
  // of LayerTreeFrameSinks (usually for tests). If this method returns null
  // a frame sink will be requested from the browser process (ie. default flow)
  virtual std::unique_ptr<cc::LayerTreeFrameSink>
  AllocateNewLayerTreeFrameSink() = 0;

  virtual void FocusChangeComplete() {}

  virtual WebInputEventResult DispatchBufferedTouchEvents() = 0;
  virtual WebInputEventResult HandleInputEvent(
      const WebCoalescedInputEvent&) = 0;
  virtual bool SupportsBufferedTouchEvents() = 0;

  virtual void DidHandleKeyEvent() {}
  virtual void WillHandleGestureEvent(const WebGestureEvent& event,
                                      bool* suppress) = 0;
  virtual void WillHandleMouseEvent(const WebMouseEvent& event) = 0;
  virtual void ObserveGestureEventAndResult(
      const WebGestureEvent& gesture_event,
      const gfx::Vector2dF& unused_delta,
      const cc::OverscrollBehavior& overscroll_behavior,
      bool event_processed) = 0;

  virtual WebTextInputType GetTextInputType() {
    return WebTextInputType::kWebTextInputTypeNone;
  }

  // Called to inform the Widget of the mouse cursor's visibility.
  virtual void SetCursorVisibilityState(bool is_visible) {}

  // The FrameWidget interface if this is a FrameWidget.
  virtual blink::FrameWidget* FrameWidget() { return nullptr; }

  // Called to inform the Widget that it has gained or lost keyboard focus.
  virtual void FocusChanged(mojom::blink::FocusState focus_state) {}

  // Call to request an animation frame from the compositor.
  virtual void ScheduleAnimation(bool urgent) {}

  // TODO(bokan): Temporary to unblock synthetic gesture events running under
  // VR. https://crbug.com/940063
  virtual bool ShouldAckSyntheticInputImmediately() { return false; }

  // Apply the visual properties.
  virtual void UpdateVisualProperties(
      const VisualProperties& visual_properties) = 0;

  // A callback to apply the updated screen rects, return true if it
  // was handled. If not handled WidgetBase will apply the screen
  // rects as the new values.
  virtual bool UpdateScreenRects(const gfx::Rect& widget_screen_rect,
                                 const gfx::Rect& window_screen_rect) {
    return false;
  }

  // Convert screen coordinates to device emulated coordinates (scaled
  // coordinates when devtools is used). This occurs for popups where their
  // window bounds are emulated.
  virtual void ScreenRectToEmulated(gfx::Rect& screen_rect) {}
  virtual void EmulatedToScreenRect(gfx::Rect& screen_rect) {}

  // Signal the orientation has changed.
  virtual void OrientationChanged() {}

  // Return the original (non-emulated) screen infos.
  virtual const display::ScreenInfos& GetOriginalScreenInfos() = 0;

  // Indication that the surface and screen were updated.
  virtual void DidUpdateSurfaceAndScreen(
      const display::ScreenInfos& previous_original_screen_infos) {}

  // Return the viewport visible rect.
  virtual gfx::Rect ViewportVisibleRect() = 0;

  // The screen orientation override.
  virtual std::optional<display::mojom::blink::ScreenOrientation>
  ScreenOrientationOverride() {
    return std::nullopt;
  }

  // Return the overridden device scale factor for testing.
  virtual float GetTestingDeviceScaleFactorOverride() { return 0.f; }

  // Test-specific methods below this point.
  virtual void ScheduleAnimationForWebTests() {}

  // Inform the widget that it was hidden.
  virtual void WasHidden() {}

  // Inform the widget that it was shown.
  virtual void WasShown(bool was_evicted) {}

  virtual void RunPaintBenchmark(int repeat_count,
                                 cc::PaintBenchmarkResult& result) {}

  // Called to indicate a synthetic event was queued.
  virtual void WillQueueSyntheticEvent(const WebCoalescedInputEvent& event) {}

  // When the WebWidget is part of a frame tree, returns the active url for
  // main frame of that tree, if the main frame is local in that tree. When
  // the WebWidget is of a different kind (e.g. a popup) it returns the active
  // url for the main frame of the frame tree that spawned the WebWidget, if
  // the main frame is local in that tree. When the relevant main frame is
  // remote in that frame tree, then the url is not known, and an empty url is
  // returned.
  virtual KURL GetURLForDebugTrace() = 0;

  // In EventTiming, we count the events invoked by user interactions. Some
  // touchstarts will be dropped before they get sent to the main thread.
  // Meanwhile, the corresponding pointerdown will not be fired. The following
  // pointerup will be captured in pointer_event_manager. The following touchend
  // will not be dispatched because there's no target which is always set by
  // touchstart. But we still want to count those touchstart, pointerdown and
  // touchend.
  virtual void CountDroppedPointerDownForEventTiming(unsigned count) {}

  // Whether to use ScrollPredictor to resample scroll events. This is false for
  // web_tests to ensure that scroll deltas are not timing-dependent.
  virtual bool AllowsScrollResampling() { return true; }

  virtual void ShowContextMenu(ui::mojom::blink::MenuSourceType source_type,
                               const gfx::Point& location) {}
  virtual void BindInputTargetClient(
      mojo::PendingReceiver<viz::mojom::blink::InputTargetClient> receiver) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_WIDGET_BASE_CLIENT_H_
