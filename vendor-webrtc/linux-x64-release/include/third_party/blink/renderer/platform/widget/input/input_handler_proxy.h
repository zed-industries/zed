// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_INPUT_HANDLER_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_INPUT_HANDLER_PROXY_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "base/types/optional_ref.h"
#include "cc/input/browser_controls_offset_tag_modifications.h"
#include "cc/input/browser_controls_state.h"
#include "cc/input/input_handler.h"
#include "cc/input/snap_fling_controller.h"
#include "cc/paint/element_id.h"
#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/public/common/input/web_gesture_event.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class TickClock;
}

namespace cc {
class EventMetrics;
}

namespace ui {
class LatencyInfo;
}

namespace blink {
class WebInputEventAttribution;
class WebMouseWheelEvent;
class WebTouchEvent;
class ElasticOverscrollController;
}  // namespace blink

namespace blink {

namespace test {
class InputHandlerProxyTest;
class InputHandlerProxyEventQueueTest;
class InputHandlerProxyMomentumScrollJankTest;
class InputHandlerProxyForceHandlingOnMainThread;
class TestInputHandlerProxy;
class UnifiedScrollingInputHandlerProxyTest;
}  // namespace test

class CompositorThreadEventQueue;
class EventWithCallback;
class InputHandlerProxyClient;
class ScrollPredictor;
class CursorControlHandler;

class SynchronousInputHandler {
 public:
  virtual ~SynchronousInputHandler() {}

  // Informs the Android WebView embedder of the current root scroll and page
  // scale state.
  virtual void UpdateRootLayerState(const gfx::PointF& total_scroll_offset,
                                    const gfx::PointF& max_scroll_offset,
                                    const gfx::SizeF& scrollable_size,
                                    float page_scale_factor,
                                    float min_page_scale_factor,
                                    float max_page_scale_factor) = 0;
};

// This class is a proxy between the blink web input events for a WebWidget and
// the compositor's input handling logic. InputHandlerProxy instances live
// entirely on the compositor thread if one exists; however, it can exist on
// the main thread in web tests where only a single thread is used.
// Each InputHandler instance handles input events intended for a specific
// WebWidget.
//
// Android WebView requires synchronous scrolling from the WebView application.
// This class provides support for that behaviour. The WebView embedder will
// act as the InputHandler for controlling the timing of input (fling)
// animations.
class PLATFORM_EXPORT InputHandlerProxy : public cc::InputHandlerClient,
                                          public cc::SnapFlingClient {
 public:
  InputHandlerProxy(cc::InputHandler& input_handler,
                    InputHandlerProxyClient* client);
  InputHandlerProxy(const InputHandlerProxy&) = delete;
  InputHandlerProxy& operator=(const InputHandlerProxy&) = delete;
  ~InputHandlerProxy() override;

  ElasticOverscrollController* elastic_overscroll_controller() {
    return elastic_overscroll_controller_.get();
  }

  // TODO(dtapuska): Eventually move this to mojo.
  struct DidOverscrollParams {
    gfx::Vector2dF accumulated_overscroll;
    gfx::Vector2dF latest_overscroll_delta;
    gfx::Vector2dF current_fling_velocity;
    gfx::PointF causal_event_viewport_point;
    cc::OverscrollBehavior overscroll_behavior;
  };

  // Result codes returned to the client indicating the status of handling the
  // event on the compositor. Used to determine further event handling behavior
  // (i.e. should the event be forwarded to the main thread, ACK'ed to the
  // browser, etc.).
  enum EventDisposition {
    // The event was handled on the compositor and should not be forwarded to
    // the main thread.
    DID_HANDLE,

    // The compositor could not handle the event but the event may still be
    // valid for handling so it should be forwarded to the main thread.
    DID_NOT_HANDLE,

    // Set only from a touchstart that occurred while a fling was in progress.
    // Indicates that the rest of the touch stream should be sent non-blocking
    // to ensure the scroll remains smooth. Since it's non-blocking, the event
    // will be ACK'ed to the browser before being dispatched to the main
    // thread.
    // TODO(bokan): It's not clear that we need a separate status for this
    // case, why can't we just use the DID_NOT_HANDLE_NON_BLOCKING below?
    DID_NOT_HANDLE_NON_BLOCKING_DUE_TO_FLING,

    // Set to indicate that the event needs to be sent to the main thread (e.g.
    // because the touch event hits a touch-event handler) but the compositor
    // has determined it shouldn't be cancellable (e.g. the event handler is
    // passive). Because it isn't cancellable, the event (and future events)
    // will be sent non-blocking and be acked to the browser before being
    // dispatchehd to the main thread.
    DID_NOT_HANDLE_NON_BLOCKING,

    // The compositor didn't handle the event but has determined the main
    // thread doesn't care about the event either (e.g. it's a touch event and
    // the hit point doesn't have a touch handler). In this case, we should ACK
    // the event immediately. Both this and DID_HANDLE will avoid forwarding
    // the event to the main thread and ACK immediately; the difference is that
    // DROP_EVENT tells the client the event wasn't consumed. For example, the
    // browser may choose to use this to avoid forwarding touch events if there
    // isn't a consumer for them (and send only the scroll events).
    DROP_EVENT,

    // Used only in scroll unification; the compositor couldn't determine the
    // scroll node to handle the event and requires a second try with an
    // ElementId provided by a hit test in Blink.
    REQUIRES_MAIN_THREAD_HIT_TEST,
  };
  using EventDispositionCallback = base::OnceCallback<void(
      EventDisposition,
      std::unique_ptr<blink::WebCoalescedInputEvent> event,
      std::unique_ptr<DidOverscrollParams>,
      const blink::WebInputEventAttribution&,
      std::unique_ptr<cc::EventMetrics> metrics)>;
  // Virtual for mocking in tests.
  virtual void HandleInputEventWithLatencyInfo(
      std::unique_ptr<blink::WebCoalescedInputEvent> event,
      std::unique_ptr<cc::EventMetrics> metrics,
      EventDispositionCallback callback);

  // In scroll unification, a scroll begin event may initially return unhandled
  // due to requiring the main thread to perform a hit test. In that case, the
  // client will perform the hit test by calling into Blink. When it has a
  // result, it can try handling the event again by calling back through this
  // method.
  void ContinueScrollBeginAfterMainThreadHitTest(
      std::unique_ptr<blink::WebCoalescedInputEvent> event,
      std::unique_ptr<cc::EventMetrics> metrics,
      EventDispositionCallback callback,
      cc::ElementId hit_tests_result);

  // Handles creating synthetic gesture events. It is currently used for
  // creating gesture event equivalents for mouse events on a composited
  // scrollbar. `original_metrics` contains metrics for the original mouse event
  // and is used to generated metrics for the new gesture event.
  void InjectScrollbarGestureScroll(
      const blink::WebInputEvent::Type type,
      const gfx::PointF& position_in_widget,
      const cc::InputHandlerPointerResult& pointer_result,
      const ui::LatencyInfo& latency_info,
      const base::TimeTicks now,
      const cc::EventMetrics* original_metrics);

  // Attempts to perform attribution of the given WebInputEvent to a target
  // frame. Intended for simple impl-side hit testing.
  blink::WebInputEventAttribution PerformEventAttribution(
      const blink::WebInputEvent& event);

  // SynchronousInputHandler needs to be informed of root layer updates.
  void SetSynchronousInputHandler(
      SynchronousInputHandler* synchronous_input_handler);

  // Called when the synchronous input handler wants to change the root scroll
  // offset. Since it has the final say, this overrides values from compositor-
  // controlled behaviour. After the offset is applied, the
  // SynchronousInputHandler should be given back the result in case it differs
  // from what was sent.
  void SynchronouslySetRootScrollOffset(
      const gfx::PointF& root_offset);

  // Similar to SetRootScrollOffset above, to control the zoom level, ie scale
  // factor. Note |magnify_delta| is an incremental rather than absolute value.
  // SynchronousInputHandler should be given back the resulting absolute value.
  void SynchronouslyZoomBy(float magnify_delta,
                           const gfx::Point& anchor);

  // Defers posting BeginMainFrame tasks. This is used during the main thread
  // hit test for a GestureScrollBegin, to avoid posting a frame before the
  // compositor thread has had a chance to update the scroll offset.
  void SetDeferBeginMainFrame(bool defer_begin_main_frame) const;

  void RequestCallbackAfterEventQueueFlushed(base::OnceClosure callback);

  // cc::InputHandlerClient implementation.
  void WillShutdown() override;
  void Animate(base::TimeTicks time) override;
  void ReconcileElasticOverscrollAndRootScroll() override;
  void SetPrefersReducedMotion(bool prefers_reduced_motion) override;
  void UpdateRootLayerStateForSynchronousInputHandler(
      const gfx::PointF& total_scroll_offset,
      const gfx::PointF& max_scroll_offset,
      const gfx::SizeF& scrollable_size,
      float page_scale_factor,
      float min_page_scale_factor,
      float max_page_scale_factor) override;
  void DeliverInputForBeginFrame(const viz::BeginFrameArgs& args) override;
  void DeliverInputForHighLatencyMode() override;
  void DeliverInputForDeadline() override;
  void DidFinishImplFrame() override;
  bool HasQueuedInput() const override;
  void SetScrollEventDispatchMode(
      cc::InputHandlerClient::ScrollEventDispatchMode mode,
      double scroll_deadline_ratio) override;

  // SnapFlingClient implementation.
  bool GetSnapFlingInfoAndSetAnimatingSnapTarget(
      const gfx::Vector2dF& current_delta,
      const gfx::Vector2dF& natural_displacement,
      gfx::PointF* initial_offset,
      gfx::PointF* target_offset) const override;
  gfx::PointF ScrollByForSnapFling(const gfx::Vector2dF& delta) override;
  void ScrollEndForSnapFling(bool did_finish) override;
  void RequestAnimationForSnapFling() override;

  void UpdateBrowserControlsState(
      cc::BrowserControlsState constraints,
      cc::BrowserControlsState current,
      bool animate,
      base::optional_ref<const cc::BrowserControlsOffsetTagModifications>
          offset_tag_modifications);

  bool gesture_scroll_on_impl_thread_for_testing() const {
    return handling_gesture_on_impl_thread_;
  }

  blink::WebGestureDevice currently_active_gesture_device() const {
    return currently_active_gesture_device_.value();
  }

  // Immediately dispatches all queued events.
  void FlushQueuedEventsForTesting();

 private:
  friend class test::TestInputHandlerProxy;
  friend class test::InputHandlerProxyTest;
  friend class test::UnifiedScrollingInputHandlerProxyTest;
  friend class test::InputHandlerProxyEventQueueTest;
  friend class test::InputHandlerProxyMomentumScrollJankTest;
  friend class test::InputHandlerProxyForceHandlingOnMainThread;

  void DispatchSingleInputEvent(std::unique_ptr<EventWithCallback>);
  void DispatchQueuedInputEvents(bool frame_aligned);
  void UpdateElasticOverscroll();

  // Helper functions for handling more complicated input events.
  EventDisposition HandleMouseWheel(const blink::WebMouseWheelEvent& event);
  EventDisposition HandleGestureScrollBegin(
      const blink::WebGestureEvent& event);
  EventDisposition HandleGestureScrollUpdate(
      const blink::WebGestureEvent& event,
      cc::EventMetrics* metrics,
      int64_t trace_id);
  EventDisposition HandleGestureScrollEnd(const blink::WebGestureEvent& event);
  EventDisposition HandleTouchStart(EventWithCallback* event_with_callback);
  EventDisposition HandleTouchMove(EventWithCallback* event_with_callback);
  EventDisposition HandleTouchEnd(EventWithCallback* event_with_callback);

  const cc::InputHandlerPointerResult HandlePointerDown(
      EventWithCallback* event_with_callback,
      const gfx::PointF& position);
  const cc::InputHandlerPointerResult HandlePointerMove(
      EventWithCallback* event_with_callback,
      const gfx::PointF& position,
      bool should_cancel_scrollbar_drag);
  const cc::InputHandlerPointerResult HandlePointerUp(
      EventWithCallback* event_with_callback,
      const gfx::PointF& position);

  void InputHandlerScrollEnd();

  // Request a frame of animation from the InputHandler or
  // SynchronousInputHandler. They can provide that by calling Animate().
  void RequestAnimation();

  // Used to send overscroll messages to the browser. It bundles the overscroll
  // params with with event ack.
  void HandleOverscroll(const gfx::PointF& causal_event_viewport_point,
                        const cc::InputHandlerScrollResult& scroll_result);

  // Update the elastic overscroll controller with |gesture_event|.
  void HandleScrollElasticityOverscroll(
      const blink::WebGestureEvent& gesture_event,
      const cc::InputHandlerScrollResult& scroll_result);

  // Overrides the internal clock for testing.
  // This doesn't take the ownership of the clock. |tick_clock| must outlive the
  // InputHandlerProxy instance.
  void SetTickClockForTesting(const base::TickClock* tick_clock);

  // |is_touching_scrolling_layer| indicates if one of the points that has
  // been touched hits a currently scrolling layer. |allowed_touch_action| is
  // the touch_action we are sure will be allowed for the given touch event.
  EventDisposition HitTestTouchEvent(const blink::WebTouchEvent& touch_event,
                                     bool* is_touching_scrolling_layer,
                                     cc::TouchAction* allowed_touch_action);

  EventDisposition RouteToTypeSpecificHandler(
      EventWithCallback* event_with_callback);

  void set_event_attribution_enabled(bool enabled) {
    event_attribution_enabled_ = enabled;
  }

  void RecordScrollBegin(blink::WebGestureDevice device,
                         uint32_t main_thread_hit_tested_reasons,
                         uint32_t main_thread_repaint_reasons,
                         bool raster_inducing = false);

  bool HasQueuedEventsReadyForDispatch(bool frame_aligned) const;

  // If `scroll_predictor_` can generate a new prediction, this will generate
  // a synthetic GestureScrollUpdate using previous input events. This will then
  // be dispatched. We only do this while scrolling and after main-thread hit
  // testing has completed.
  void GenerateAndDispatchSytheticScrollPrediction(
      const viz::BeginFrameArgs& args);

  raw_ptr<InputHandlerProxyClient> client_;

  // The input handler object is owned by the compositor delegate. The input
  // handler must call WillShutdown() on this class before it is deleted at
  // which point this pointer will be cleared.
  raw_ptr<cc::InputHandler> input_handler_;

  raw_ptr<SynchronousInputHandler> synchronous_input_handler_;

  // This should be true when a pinch is in progress. The sequence of events is
  // as follows: GSB GPB GSU GPU ... GPE GSE.
  bool handling_gesture_on_impl_thread_;

  bool gesture_pinch_in_progress_ = false;
  bool in_inertial_scrolling_ = false;
  bool scroll_sequence_ignored_;
  std::optional<EventDisposition> main_thread_touch_sequence_start_disposition_;

  // Used to animate rubber-band/bounce over-scroll effect.
  std::unique_ptr<ElasticOverscrollController> elastic_overscroll_controller_;

  // The merged result of the last touch event with previous touch events
  // within a single touch sequence. This value will get returned for
  // subsequent TouchMove events to allow passive events not to block
  // scrolling.
  std::optional<EventDisposition> touch_result_;

  // The result of the last mouse wheel event in a wheel phase sequence. This
  // value is used to determine whether the next wheel scroll is blocked on the
  // Main thread or not.
  std::optional<EventDisposition> mouse_wheel_result_;

  // Used to record overscroll notifications while an event is being
  // dispatched.  If the event causes overscroll, the overscroll metadata is
  // bundled in the event ack, saving an IPC.
  std::unique_ptr<DidOverscrollParams> current_overscroll_params_;

  std::unique_ptr<CompositorThreadEventQueue> compositor_event_queue_;

  // Set only when the compositor input handler is handling a gesture. Tells
  // which source device is currently performing a gesture based scroll.
  std::optional<blink::WebGestureDevice> currently_active_gesture_device_;
  // Set only when the compositor input handler is handling a gesture. Denotes
  // which modifiers were present on the `WebInputEvent` so they can be applied
  // in GenerateAndDispatchSytheticScrollPrediction.
  std::optional<int> currently_active_gesture_scroll_modifiers_;

  base::OnceClosure queue_flushed_callback_;

  // Tracks whether the first scroll update gesture event has been seen after a
  // scroll begin. This is set/reset when scroll gestures are processed in
  // HandleInputEventWithLatencyInfo and shouldn't be used outside the scope
  // of that method.
  bool has_seen_first_gesture_scroll_update_after_begin_;

  // Whether the last injected scroll gesture was a GestureScrollBegin. Used to
  // determine which GestureScrollUpdate is the first in a gesture sequence for
  // latency classification. This is separate from
  // |is_first_gesture_scroll_update_| and is used to determine which type of
  // latency component should be added for injected GestureScrollUpdates.
  bool last_injected_gesture_was_begin_;

  raw_ptr<const base::TickClock> tick_clock_;

  std::unique_ptr<cc::SnapFlingController> snap_fling_controller_;

  std::unique_ptr<ScrollPredictor> scroll_predictor_;

  // These flags are set for the SkipTouchEventFilter experiment. The
  // experiment either skips filtering discrete (touch start/end) events to the
  // main thread, or all events (touch start/end/move).
  bool skip_touch_filter_discrete_ = false;
  bool skip_touch_filter_all_ = false;

  // This is set when the input handler proxy has requested that the client
  // perform a hit test for a scroll begin on the main thread. During that
  // time, scroll updates need to be queued. The reply from the main thread
  // will come by calling ContinueScrollBeginAfterMainThreadHitTest where the
  // queue will be flushed and this bit cleared. Used only in scroll
  // unification.
  uint32_t scroll_begin_main_thread_hit_test_reasons_ =
      cc::MainThreadScrollingReason::kNotScrollingOnMain;

  // This bit can be used to disable event attribution in cases where the
  // hit test information is unnecessary (e.g. tests).
  bool event_attribution_enabled_ = true;

  // This tracks whether the user has set prefers reduced motion.
  bool prefers_reduced_motion_ = false;

  // Swipe to move cursor feature.
  std::unique_ptr<CursorControlHandler> cursor_control_handler_;

  // The most recent viz::BeginFrameArgs that was received in
  // DeliverInputForBeginFrame. Which will be the active frame for all
  // subsequent events arriving in HandleInputEventWithLatencyInfo. If frame
  // production stops this will be outdated.
  viz::BeginFrameArgs current_begin_frame_args_;

  // When true, scroll events arriving in HandleInputEventWithLatencyInfo
  // will be enqueued to be dispatched during the next
  // DeliverInputForBeginFrame. When false, the scroll events will be dispatched
  // immediately. This will occur if DeliverInputForBeginFrame was called while
  // scrolling, with an empty `compositor_event_queue_`, until frame production
  // has started, or completed.
  bool enqueue_scroll_events_ = true;

  // `cc::InputHandlerClient::ScrollEventDispatchMode::kEnqueueScrollEvents`:
  // Scroll events arriving in `HandleInputEventWithLatencyInfo` will be
  // enqueued to be dispatched during the next `DeliverInputForBeginFrame`.
  //
  // `cc::InputHandlerClient::ScrollEventDispatchMode::kDispatchScrollEventsImmediately`:
  // Scroll events arriving in HandleInputEventWithLatencyInfo will be
  // dispatched immediately, if `DeliverInputForBeginFrame` was called while
  // scrolling, with no input events in the queue. This will occur until frame
  // production has started, or completed.
  //
  // `cc::InputHandlerClient::ScrollEventDispatchMode::kUseScrollPredictorForEmptyQueue`:
  // If `compositor_event_queue_` is empty when `DeliverInputForBeginFrame` is
  // called, while we are scrolling. We will use `scroll_predictor_` to
  // generate a new prediction. We will then dispatch a synthetic
  // `GestureScrollUpdate` using the prediction.
  cc::InputHandlerClient::ScrollEventDispatchMode scroll_event_dispatch_mode_ =
      cc::InputHandlerClient::ScrollEventDispatchMode::kEnqueueScrollEvents;

  double scroll_deadline_ratio_ = 0.333;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_INPUT_HANDLER_PROXY_H_
