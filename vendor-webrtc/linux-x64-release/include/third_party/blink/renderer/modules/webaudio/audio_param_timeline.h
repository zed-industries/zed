/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_TIMELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_TIMELINE_H_

#include <tuple>

#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webaudio/audio_destination_node.h"
#include "third_party/blink/renderer/modules/webaudio/base_audio_context.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioParamTimeline {
  DISALLOW_NEW();

 public:
  AudioParamTimeline() = default;

  void SetValueAtTime(float value, double time, ExceptionState&);
  void LinearRampToValueAtTime(float value,
                               double time,
                               float initial_value,
                               double call_time,
                               ExceptionState&);
  void ExponentialRampToValueAtTime(float value,
                                    double time,
                                    float initial_value,
                                    double call_time,
                                    ExceptionState&);
  void SetTargetAtTime(float target,
                       double time,
                       double time_constant,
                       ExceptionState&);
  void SetValueCurveAtTime(const Vector<float>& curve,
                           double time,
                           double duration,
                           ExceptionState&);
  void CancelScheduledValues(double start_time, ExceptionState&);
  void CancelAndHoldAtTime(double cancel_time, ExceptionState&);

  // Compute the value from this AudioParamHandler at the current context frame.
  // Returns two values:
  //
  //   bool has_value - to indicate if the value could be computed from the
  //                    timeline
  //   float value    - the timeline value if `has_value` is true; otherwise
  //                    `default_value` is returned.
  std::tuple<bool, float> ValueForContextTime(AudioDestinationHandler&,
                                              float default_value,
                                              float min_value,
                                              float max_value,
                                              unsigned render_quantum_frames);

  // Given the time range in frames, calculates parameter values into the values
  // buffer and returns the last parameter value calculated for "values" or the
  // defaultValue if none were calculated.  controlRate is the rate (number per
  // second) at which parameter values will be calculated.  It should equal
  // sampleRate for sample-accurate parameter changes, and otherwise will
  // usually match the render quantum size such that the parameter value changes
  // once per render quantum.
  float ValuesForFrameRange(size_t start_frame,
                            size_t end_frame,
                            float default_value,
                            float* values,
                            unsigned number_of_values,
                            double sample_rate,
                            double control_rate,
                            float min_value,
                            float max_value,
                            unsigned render_quantum_frames);

  // Returns true if the AudioParam timeline needs to run in this
  // rendering quantum.  This means some automation is already running
  // or is scheduled to run in the current rendering quantuym.
  bool HasValues(size_t current_frame,
                 double sample_rate,
                 unsigned render_quantum_frames) const;

 private:
  class ParamEvent {
   public:
    enum class Type {
      kSetValue,
      kLinearRampToValue,
      kExponentialRampToValue,
      kSetTarget,
      kSetValueCurve,
      // For cancelValuesAndHold
      kCancelValues,
      // Special marker for the end of a `kSetValueCurve` event.
      kSetValueCurveEnd,
      kLastType
    };

    static std::unique_ptr<ParamEvent> CreateLinearRampEvent(
        float value,
        double time,
        float initial_value,
        double call_time);
    static std::unique_ptr<ParamEvent> CreateExponentialRampEvent(
        float value,
        double time,
        float initial_value,
        double call_time);
    static std::unique_ptr<ParamEvent> CreateSetValueEvent(float value,
                                                           double time);
    static std::unique_ptr<ParamEvent>
    CreateSetTargetEvent(float value, double time, double time_constant);
    static std::unique_ptr<ParamEvent> CreateSetValueCurveEvent(
        const Vector<float>& curve,
        double time,
        double duration);
    static std::unique_ptr<ParamEvent> CreateSetValueCurveEndEvent(float value,
                                                                   double time);
    static std::unique_ptr<ParamEvent> CreateCancelValuesEvent(
        double time,
        std::unique_ptr<ParamEvent> saved_event);
    // Needed for creating a saved event where we want to supply all
    // the possible parameters because we're mostly copying an
    // existing event.
    static std::unique_ptr<ParamEvent> CreateGeneralEvent(
        Type,
        float value,
        double time,
        float initial_value,
        double call_time,
        double time_constant,
        double duration,
        Vector<float>& curve,
        double curve_points_per_second,
        float curve_end_value,
        std::unique_ptr<ParamEvent> saved_event);

    static bool EventPreceeds(const std::unique_ptr<ParamEvent>& a,
                              const std::unique_ptr<ParamEvent>& b) {
      return a->Time() < b->Time();
    }

    Type GetType() const { return type_; }
    float Value() const { return value_; }
    double Time() const { return time_; }
    void SetTime(double new_time) { time_ = new_time; }
    double TimeConstant() const { return time_constant_; }
    double Duration() const { return duration_; }
    const Vector<float>& Curve() const { return curve_; }
    Vector<float>& Curve() { return curve_; }
    float InitialValue() const { return initial_value_; }
    double CallTime() const { return call_time_; }

    double CurvePointsPerSecond() const { return curve_points_per_second_; }
    float CurveEndValue() const { return curve_end_value_; }

    // For CancelValues events. Not valid for any other event.
    ParamEvent* SavedEvent() const;
    bool HasDefaultCancelledValue() const;
    void SetCancelledValue(float);

   private:
    // General event
    ParamEvent(Type type,
               float value,
               double time,
               float initial_value,
               double call_time,
               double time_constant,
               double duration,
               Vector<float>& curve,
               double curve_points_per_second,
               float curve_end_value,
               std::unique_ptr<ParamEvent> saved_event);

    // Create simplest event needing just a value and time, like
    // setValueAtTime.
    ParamEvent(Type, float value, double time);

    // Create a linear or exponential ramp that requires an initial
    // value and time in case there is no actual event that preceeds
    // this event.
    ParamEvent(Type,
               float value,
               double time,
               float initial_value,
               double call_time);

    // Create an event needing a time constant (setTargetAtTime)
    ParamEvent(Type, float value, double time, double time_constant);

    // Create a setValueCurve event
    ParamEvent(Type,
               double time,
               double duration,
               const Vector<float>& curve,
               double curve_points_per_second,
               float curve_end_value);

    // Create CancelValues event
    ParamEvent(Type, double time, std::unique_ptr<ParamEvent> saved_event);

    Type type_;

    // The value for the event.  The interpretation of this depends on
    // the event type. Not used for SetValueCurve. For CancelValues,
    // it is the end value to use when cancelling a LinearRampToValue
    // or ExponentialRampToValue event.
    float value_;

    // The time for the event. The interpretation of this depends on
    // the event type.
    double time_;

    // Initial value and time to use for linear and exponential ramps that don't
    // have a preceding event.
    float initial_value_;
    double call_time_;

    // Only used for SetTarget events
    double time_constant_;

    // The following items are only used for SetValueCurve events.
    //
    // The duration of the curve.
    double duration_;
    // The array of curve points.
    Vector<float> curve_;
    // The number of curve points per second. it is used to compute
    // the curve index step when running the automation.
    double curve_points_per_second_;
    // The default value to use at the end of the curve.  Normally
    // it's the last entry in m_curve, but cancelling a SetValueCurve
    // will set this to a new value.
    float curve_end_value_;

    // For CancelValues. If CancelValues is in the middle of an event, this
    // holds the event that is being cancelled, so that processing can
    // continue as if the event still existed up until we reach the actual
    // scheduled cancel time.
    std::unique_ptr<ParamEvent> saved_event_;

    // True if a default value has been assigned to the CancelValues event.
    bool has_default_cancelled_value_;
  };

  // State of the timeline for the current event.
  struct AutomationState {
    // Parameters for the current automation request.  Number of
    // values to be computed for the automation request
    const unsigned number_of_values;
    // Start and end frames for this automation request
    const size_t start_frame;
    const size_t end_frame;

    // Sample rate and control rate for this request
    const double sample_rate;
    const double control_rate;

    // Parameters needed for processing the current event.
    const unsigned fill_to_frame;
    const size_t fill_to_end_frame;

    // Value and time for the current event
    const float value1;
    const double time1;

    // Value and time for the next event, if any.
    const float value2;
    const double time2;

    // The current event, and its index in the event vector.
    raw_ptr<const ParamEvent> event;
    const int event_index;
  };

  void InsertEvent(std::unique_ptr<ParamEvent>, ExceptionState&)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);
  float ValuesForFrameRangeImpl(size_t start_frame,
                                size_t end_frame,
                                float default_value,
                                float* values,
                                unsigned number_of_values,
                                double sample_rate,
                                double control_rate,
                                unsigned render_quantum_frames)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Produce a nice string describing the event in human-readable form.
  String EventToString(const ParamEvent&) const;

  // Automation functions that compute the vlaue of the specified
  // automation at the specified time.
  float LinearRampAtTime(double t,
                         float value1,
                         double time1,
                         float value2,
                         double time2);
  float ExponentialRampAtTime(double t,
                              float value1,
                              double time1,
                              float value2,
                              double time2);
  float TargetValueAtTime(double t,
                          float value1,
                          double time1,
                          float value2,
                          float time_constant);
  float ValueCurveAtTime(double t,
                         double time1,
                         double duration,
                         const float* curve_data,
                         unsigned curve_length);

  // Handles the special case where the first event in the timeline
  // starts after `start_frame`.  These initial values are filled using
  // `default_value`.  The updated `current_frame` and `write_index` is
  // returned.
  std::tuple<size_t, unsigned> HandleFirstEvent(float* values,
                                                float default_value,
                                                unsigned number_of_values,
                                                size_t start_frame,
                                                size_t end_frame,
                                                double sample_rate,
                                                size_t current_frame,
                                                unsigned write_index)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Return true if `current_event` starts after `current_frame`, but
  // also takes into account the `next_event` if any.
  bool IsEventCurrent(const ParamEvent* current_event,
                      const ParamEvent* next_event,
                      size_t current_frame,
                      double sample_rate) const;

  // Clamp times to current time, if needed for any new events.  Note,
  // this method can mutate `events_`, so do call this only in safe
  // places.
  void ClampNewEventsToCurrentTime(double current_time)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Handle the case where the last event in the timeline is in the
  // past.  Returns false if any event is not in the past. Otherwise,
  // return true and also fill in `values` with `default_value`.
  // `default_value` may be updated with a new value.
  bool HandleAllEventsInThePast(double current_time,
                                double sample_rate,
                                float& default_value,
                                unsigned number_of_values,
                                float* values,
                                unsigned render_quantum_frames)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Handle processing of CancelValue event. If cancellation happens, value2,
  // time2, and nextEventType will be updated with the new value due to
  // cancellation.  Note that `next_event` or its member can be null.
  std::tuple<float, double, ParamEvent::Type> HandleCancelValues(
      const ParamEvent* current_event,
      ParamEvent* next_event,
      float value2,
      double time2) EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Process a SetTarget event and the next event is a
  // LinearRampToValue or ExponentialRampToValue event.  This requires
  // special handling because the ramp should start at whatever value
  // the SetTarget event has reached at this time, instead of using
  // the value of the SetTarget event.
  void ProcessSetTargetFollowedByRamp(int event_index,
                                      ParamEvent*& current_event,
                                      ParamEvent::Type next_event_type,
                                      size_t current_frame,
                                      double sample_rate,
                                      double control_rate,
                                      float& value)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Handle processing of LinearRampEvent, writing the appropriate
  // values to `values`.  Returns the updated `current_frame`, last
  // computed `value`, and the updated `write_index`.
  std::tuple<size_t, float, unsigned> ProcessLinearRamp(
      const AutomationState& current_state,
      float* values,
      size_t current_frame,
      float value,
      unsigned write_index);

  // Handle processing of ExponentialRampEvent, writing the appropriate
  // values to `values`.  Returns the updated `current_frame`, last
  // computed `value`, and the updated `write_index`.
  std::tuple<size_t, float, unsigned> ProcessExponentialRamp(
      const AutomationState& current_state,
      float* values,
      size_t current_frame,
      float value,
      unsigned write_index);

  // Handle processing of SetTargetEvent, writing the appropriate
  // values to `values`.  Returns the updated `current_frame`, last
  // computed `value`, and the updated `write_index`.
  std::tuple<size_t, float, unsigned> ProcessSetTarget(
      const AutomationState& current_state,
      float* values,
      size_t current_frame,
      float value,
      unsigned write_index);

  // Handle processing of SetValueCurveEvent, writing the appropriate
  // values to `values`.  Returns the updated `current_frame`, last
  // computed `value`, and the updated `write_index`.
  std::tuple<size_t, float, unsigned> ProcessSetValueCurve(
      const AutomationState& current_state,
      float* values,
      size_t current_frame,
      float value,
      unsigned write_index);

  // Handle processing of CancelValuesEvent, writing the appropriate
  // values to `values`.  Returns the updated `current_frame`, last
  // computed `value`, and the updated `write_index`.
  std::tuple<size_t, float, unsigned> ProcessCancelValues(
      const AutomationState& current_state,
      float* values,
      size_t current_frame,
      float value,
      unsigned write_index) EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Fill the output vector `values` with the value `default_value`,
  // starting at `write_index` and continuing up to `end_frame`
  // (exclusive).  `write_index` is updated with the new index.
  uint32_t FillWithDefault(float* values,
                           float default_value,
                           uint32_t end_frame,
                           uint32_t write_index);

  // When cancelling events, remove the items from `events_` starting
  // at the given index.  Update `new_events_` too.
  void RemoveCancelledEvents(wtf_size_t first_event_to_remove)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Remove old events, but always leave at least one event in the timeline.
  // This is needed in case a new event is added (like linearRamp) that would
  // use a previous event to compute the automation.
  void RemoveOldEvents(wtf_size_t n_events)
      EXCLUSIVE_LOCKS_REQUIRED(events_lock_);

  // Vector of all automation events for the AudioParam.
  Vector<std::unique_ptr<ParamEvent>> events_ GUARDED_BY(events_lock_);

  // Vector of raw pointers to the actual ParamEvent that was
  // inserted.  As new events are added, `new_events_` is updated with
  // the new event.  When the timline is processed, these events are
  // clamped to current time by `ClampNewEventsToCurrentTime`. Access
  // must be locked via `events_lock_`.  Must be maintained together
  // with `events_`.
  HashSet<ParamEvent*> new_events_ GUARDED_BY(events_lock_);

  mutable base::Lock events_lock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_TIMELINE_H_
