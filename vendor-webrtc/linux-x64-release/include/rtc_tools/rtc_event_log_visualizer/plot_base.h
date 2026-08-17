/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_PLOT_BASE_H_
#define RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_PLOT_BASE_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/strings/string_view.h"

// Generated at build-time by the protobuf compiler.
#include "rtc_tools/rtc_event_log_visualizer/proto/chart.pb.h"

namespace webrtc {

enum class LineStyle {
  kNone,  // No line connecting the points. Used to create scatter plots.
  kLine,  // Straight line between consecutive points.
  kStep,  // Horizontal line until the next value. Used for state changes.
  kBar    // Vertical bars from the x-axis to the point.
};

enum class PointStyle {
  kNone,      // Don't draw the points.
  kHighlight  // Draw circles or dots to highlight the points.
};

struct TimeSeriesPoint {
  TimeSeriesPoint(float x, float y) : x(x), y(y) {}
  float x;
  float y;
};

struct TimeSeries {
  TimeSeries() = default;  // TODO(terelius): Remove the default constructor.
  TimeSeries(const char* label,
             LineStyle line_style,
             PointStyle point_style = PointStyle::kNone)
      : label(label), line_style(line_style), point_style(point_style) {}
  TimeSeries(const std::string& label,
             LineStyle line_style,
             PointStyle point_style = PointStyle::kNone)
      : label(label), line_style(line_style), point_style(point_style) {}
  TimeSeries(TimeSeries&& other)
      : label(std::move(other.label)),
        line_style(other.line_style),
        point_style(other.point_style),
        points(std::move(other.points)) {}
  TimeSeries& operator=(TimeSeries&& other) {
    label = std::move(other.label);
    line_style = other.line_style;
    point_style = other.point_style;
    points = std::move(other.points);
    return *this;
  }

  std::string label;
  LineStyle line_style = LineStyle::kLine;
  PointStyle point_style = PointStyle::kNone;
  std::vector<TimeSeriesPoint> points;
};

struct Interval {
  Interval() = default;
  Interval(double begin, double end) : begin(begin), end(end) {}

  double begin;
  double end;
};

struct IntervalSeries {
  enum Orientation { kHorizontal, kVertical };

  IntervalSeries() = default;
  IntervalSeries(const std::string& label,
                 const std::string& color,
                 IntervalSeries::Orientation orientation)
      : label(label), color(color), orientation(orientation) {}

  std::string label;
  std::string color;
  Orientation orientation;
  std::vector<Interval> intervals;
};

// A container that represents a general graph, with axes, title and one or
// more data series. A subclass should define the output format by overriding
// the Draw() method.
class Plot {
 public:
  virtual ~Plot() {}

  ABSL_DEPRECATED("Use PrintPythonCode() or ExportProtobuf() instead.")
  virtual void Draw() {}

  // Sets the lower x-axis limit to min_value (if left_margin == 0).
  // Sets the upper x-axis limit to max_value (if right_margin == 0).
  // The margins are measured as fractions of the interval
  // (max_value - min_value) and are added to either side of the plot.
  void SetXAxis(float min_value,
                float max_value,
                std::string label,
                float left_margin = 0,
                float right_margin = 0);

  // Sets the lower and upper x-axis limits based on min_value and max_value,
  // but modified such that all points in the data series can be represented
  // on the x-axis. The margins are measured as fractions of the range of
  // x-values and are added to either side of the plot.
  void SetSuggestedXAxis(float min_value,
                         float max_value,
                         std::string label,
                         float left_margin = 0,
                         float right_margin = 0);

  // Sets the lower y-axis limit to min_value (if bottom_margin == 0).
  // Sets the upper y-axis limit to max_value (if top_margin == 0).
  // The margins are measured as fractions of the interval
  // (max_value - min_value) and are added to either side of the plot.
  void SetYAxis(float min_value,
                float max_value,
                std::string label,
                float bottom_margin = 0,
                float top_margin = 0);

  // Sets the lower and upper y-axis limits based on min_value and max_value,
  // but modified such that all points in the data series can be represented
  // on the y-axis. The margins are measured as fractions of the range of
  // y-values and are added to either side of the plot.
  void SetSuggestedYAxis(float min_value,
                         float max_value,
                         std::string label,
                         float bottom_margin = 0,
                         float top_margin = 0);

  void SetYAxisTickLabels(
      const std::vector<std::pair<float, std::string>>& labels);

  // Sets the title of the plot.
  void SetTitle(const std::string& title);

  // Sets an unique ID for the plot. The ID is similar to the title except that
  // the title might change in future releases whereas the ID should be stable
  // over time.
  void SetId(const std::string& id);
  void SetId(absl::string_view id);

  // Add a new TimeSeries to the plot.
  void AppendTimeSeries(TimeSeries&& time_series);

  // Add a new IntervalSeries to the plot.
  void AppendIntervalSeries(IntervalSeries&& interval_series);

  // Add a new TimeSeries to the plot if the series contains contains data.
  // Otherwise, the call has no effect and the timeseries is destroyed.
  void AppendTimeSeriesIfNotEmpty(TimeSeries&& time_series);

  // Replaces PythonPlot::Draw()
  void PrintPythonCode(
      absl::string_view figure_output_path = absl::string_view()) const;

  // Replaces ProtobufPlot::Draw()
  void ExportProtobuf(webrtc::analytics::Chart* chart) const;

 protected:
  float xaxis_min_;
  float xaxis_max_;
  std::string xaxis_label_;
  float yaxis_min_;
  float yaxis_max_;
  std::string yaxis_label_;
  std::vector<std::pair<float, std::string>> yaxis_tick_labels_;
  std::string title_;
  std::string id_;
  std::vector<TimeSeries> series_list_;
  std::vector<IntervalSeries> interval_list_;
};

class PlotCollection {
 public:
  virtual ~PlotCollection() {}

  ABSL_DEPRECATED("Use PrintPythonCode() or ExportProtobuf() instead.")
  virtual void Draw() {}

  virtual Plot* AppendNewPlot();

  virtual Plot* AppendNewPlot(absl::string_view);

  void SetCallTimeToUtcOffsetMs(int64_t calltime_to_utc_ms) {
    calltime_to_utc_ms_ = calltime_to_utc_ms;
  }

  // Replaces PythonPlotCollection::Draw()
  void PrintPythonCode(
      bool shared_xaxis,
      absl::string_view figure_output_path = absl::string_view()) const;

  // Replaces ProtobufPlotCollections::Draw()
  void ExportProtobuf(webrtc::analytics::ChartCollection* collection) const;

 protected:
  std::vector<std::unique_ptr<Plot>> plots_;
  std::optional<int64_t> calltime_to_utc_ms_;
};

}  // namespace webrtc

#endif  // RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_PLOT_BASE_H_
