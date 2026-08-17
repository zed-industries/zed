/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZER_COMMON_H_
#define RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZER_COMMON_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>

#include "api/function_view.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/rtc_event_log_parser.h"
#include "rtc_tools/rtc_event_log_visualizer/plot_base.h"

namespace webrtc {

constexpr int kNumMicrosecsPerSec = 1000000;
constexpr int kNumMillisecsPerSec = 1000;
constexpr float kLeftMargin = 0.01f;
constexpr float kRightMargin = 0.02f;
constexpr float kBottomMargin = 0.02f;
constexpr float kTopMargin = 0.05f;

class AnalyzerConfig {
 public:
  float GetCallTimeSec(Timestamp timestamp) const {
    Timestamp offset = normalize_time_ ? begin_time_ : Timestamp::Zero();
    return static_cast<float>((timestamp - offset).us()) / 1000000;
  }

  float GetCallTimeSecFromMs(int64_t timestamp_ms) const {
    return GetCallTimeSec(Timestamp::Millis(timestamp_ms));
  }

  float CallBeginTimeSec() const { return GetCallTimeSec(begin_time_); }

  float CallEndTimeSec() const { return GetCallTimeSec(end_time_); }

  int64_t CallTimeToUtcOffsetMs() {
    if (normalize_time_) {
      Timestamp utc_begin_time_ = begin_time_ + rtc_to_utc_offset_;
      return utc_begin_time_.ms();
    }
    return rtc_to_utc_offset_.ms();
  }

  // Window and step size used for calculating moving averages, e.g. bitrate.
  // The generated data points will be `step_.ms()` milliseconds apart.
  // Only events occurring at most `window_duration_.ms()` milliseconds before
  // the current data point will be part of the average.
  TimeDelta window_duration_ = TimeDelta::Millis(250);
  TimeDelta step_ = TimeDelta::Millis(10);

  // First and last events of the log.
  Timestamp begin_time_ = Timestamp::MinusInfinity();
  Timestamp end_time_ = Timestamp::MinusInfinity();
  TimeDelta rtc_to_utc_offset_ = TimeDelta::Zero();
  bool normalize_time_;
};

struct LayerDescription {
  LayerDescription(uint32_t ssrc, uint8_t spatial_layer, uint8_t temporal_layer)
      : ssrc(ssrc),
        spatial_layer(spatial_layer),
        temporal_layer(temporal_layer) {}
  bool operator<(const LayerDescription& other) const {
    if (ssrc != other.ssrc)
      return ssrc < other.ssrc;
    if (spatial_layer != other.spatial_layer)
      return spatial_layer < other.spatial_layer;
    return temporal_layer < other.temporal_layer;
  }
  uint32_t ssrc;
  uint8_t spatial_layer;
  uint8_t temporal_layer;
};

bool IsRtxSsrc(const ParsedRtcEventLog& parsed_log,
               PacketDirection direction,
               uint32_t ssrc);
bool IsVideoSsrc(const ParsedRtcEventLog& parsed_log,
                 PacketDirection direction,
                 uint32_t ssrc);
bool IsAudioSsrc(const ParsedRtcEventLog& parsed_log,
                 PacketDirection direction,
                 uint32_t ssrc);

std::string GetStreamName(const ParsedRtcEventLog& parsed_log,
                          PacketDirection direction,
                          uint32_t ssrc);
std::string GetLayerName(LayerDescription layer);

// For each element in data_view, use `f()` to extract a y-coordinate and
// store the result in a TimeSeries.
template <typename DataType, typename IterableType>
void ProcessPoints(FunctionView<float(const DataType&)> fx,
                   FunctionView<std::optional<float>(const DataType&)> fy,
                   const IterableType& data_view,
                   TimeSeries* result) {
  for (size_t i = 0; i < data_view.size(); i++) {
    const DataType& elem = data_view[i];
    float x = fx(elem);
    std::optional<float> y = fy(elem);
    if (y)
      result->points.emplace_back(x, *y);
  }
}

// For each pair of adjacent elements in `data`, use `f()` to extract a
// y-coordinate and store the result in a TimeSeries. Note that the x-coordinate
// will be the time of the second element in the pair.
template <typename DataType, typename ResultType, typename IterableType>
void ProcessPairs(FunctionView<float(const DataType&)> fx,
                  FunctionView<std::optional<ResultType>(const DataType&,
                                                         const DataType&)> fy,
                  const IterableType& data,
                  TimeSeries* result) {
  for (size_t i = 1; i < data.size(); i++) {
    float x = fx(data[i]);
    std::optional<ResultType> y = fy(data[i - 1], data[i]);
    if (y)
      result->points.emplace_back(x, static_cast<float>(*y));
  }
}

// For each pair of adjacent elements in `data`, use `f()` to extract a
// y-coordinate and store the result in a TimeSeries. Note that the x-coordinate
// will be the time of the second element in the pair.
template <typename DataType, typename ResultType, typename IterableType>
void AccumulatePairs(
    FunctionView<float(const DataType&)> fx,
    FunctionView<std::optional<ResultType>(const DataType&, const DataType&)>
        fy,
    const IterableType& data,
    TimeSeries* result) {
  ResultType sum = 0;
  for (size_t i = 1; i < data.size(); i++) {
    float x = fx(data[i]);
    std::optional<ResultType> y = fy(data[i - 1], data[i]);
    if (y) {
      sum += *y;
      result->points.emplace_back(x, static_cast<float>(sum));
    }
  }
}

// Calculates a moving average of `data` and stores the result in a TimeSeries.
// A data point is generated every `step` microseconds from `begin_time`
// to `end_time`. The value of each data point is the average of the data
// during the preceding `window_duration_us` microseconds.
template <typename DataType, typename ResultType, typename IterableType>
void MovingAverage(FunctionView<std::optional<ResultType>(const DataType&)> fy,
                   const IterableType& data_view,
                   AnalyzerConfig config,
                   TimeSeries* result) {
  size_t window_index_begin = 0;
  size_t window_index_end = 0;
  ResultType sum_in_window = 0;

  for (Timestamp t = config.begin_time_; t < config.end_time_ + config.step_;
       t += config.step_) {
    while (window_index_end < data_view.size() &&
           data_view[window_index_end].log_time() < t) {
      std::optional<ResultType> value = fy(data_view[window_index_end]);
      if (value)
        sum_in_window += *value;
      ++window_index_end;
    }
    while (window_index_begin < data_view.size() &&
           data_view[window_index_begin].log_time() <
               t - config.window_duration_) {
      std::optional<ResultType> value = fy(data_view[window_index_begin]);
      if (value)
        sum_in_window -= *value;
      ++window_index_begin;
    }
    float window_duration_s =
        static_cast<float>(config.window_duration_.us()) / kNumMicrosecsPerSec;
    float x = config.GetCallTimeSec(t);
    float y = sum_in_window / window_duration_s;
    result->points.emplace_back(x, y);
  }
}

}  // namespace webrtc

#endif  // RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZER_COMMON_H_
