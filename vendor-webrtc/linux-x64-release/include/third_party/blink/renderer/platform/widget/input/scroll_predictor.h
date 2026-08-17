// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_SCROLL_PREDICTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_SCROLL_PREDICTOR_H_

#include <vector>

#include "third_party/blink/public/mojom/input/gesture_event.mojom-blink.h"
#include "third_party/blink/renderer/platform/widget/input/event_with_callback.h"
#include "third_party/blink/renderer/platform/widget/input/prediction/filter_factory.h"
#include "ui/base/prediction/input_predictor.h"
#include "ui/base/prediction/prediction_metrics_handler.h"

namespace blink {

namespace test {
class ScrollPredictorTest;
}

// This class handles resampling GestureScrollUpdate events on InputHandlerProxy
// at |BeginFrame| signal, before events been dispatched. The predictor use
// original events to update the prediction and align the aggregated event
// timestamp and delta_x/y to the VSync time.
class PLATFORM_EXPORT ScrollPredictor {
 public:
  // Select the predictor type from field trial params and initialize the
  // predictor.
  explicit ScrollPredictor();
  ScrollPredictor(const ScrollPredictor&) = delete;
  ScrollPredictor& operator=(const ScrollPredictor&) = delete;
  ~ScrollPredictor();

  // Reset the predictors on each GSB.
  void ResetOnGestureScrollBegin(const WebGestureEvent& event);

  // Resampling GestureScrollUpdate events. Updates the prediction with events
  // in original events list, and apply the prediction to the aggregated GSU
  // event if enable_resampling is true.
  std::unique_ptr<EventWithCallback> ResampleScrollEvents(
      std::unique_ptr<EventWithCallback> event_with_callback,
      base::TimeTicks frame_time,
      base::TimeDelta frame_interval);

  // Resamples the current GestureScrollUpdate events at the given `frame_time`.
  std::unique_ptr<EventWithCallback> GenerateSyntheticScrollUpdate(
      base::TimeTicks frame_time,
      base::TimeDelta frame_interval,
      mojom::blink::GestureDevice gesture_device,
      int modifiers);

  bool HasPrediction() const;

 private:
  friend class test::InputHandlerProxyEventQueueTest;
  friend class test::ScrollPredictorTest;

  // Reset predictor and clear accumulated delta. This should be called on
  // GestureScrollBegin.
  void Reset();

  // Update the prediction with GestureScrollUpdate deltaX and deltaY
  void UpdatePrediction(const WebInputEvent& event, base::TimeTicks frame_time);

  // Apply resampled deltaX/deltaY to gesture events.
  void ResampleEvent(base::TimeTicks frame_time,
                     base::TimeDelta frame_interval,
                     WebInputEvent* event);

  // Reports metrics scores UMA histogram based on the metrics defined
  // in |PredictionMetricsHandler|
  void EvaluatePrediction();

  std::unique_ptr<ui::InputPredictor> predictor_;
  std::unique_ptr<ui::InputFilter> filter_;

  std::unique_ptr<FilterFactory> filter_factory_;

  // Whether predicted scroll events should be filtered or not
  bool filtering_enabled_ = false;

  // Total scroll delta from original scroll update events, used for calculating
  // predictions. Reset on GestureScrollBegin.
  gfx::PointF current_event_accumulated_delta_;
  // Predicted accumulated delta from last vsync, use for calculating delta_x
  // and delta_y for the resampled/predicted event.
  gfx::PointF last_predicted_accumulated_delta_;

  // Whether current scroll event should be resampled.
  bool should_resample_scroll_events_ = false;

  // Handler used for evaluating the prediction
  ui::PredictionMetricsHandler metrics_handler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_SCROLL_PREDICTOR_H_
