#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_WEB_PERFORMANCE_PROFILING_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_WEB_PERFORMANCE_PROFILING_H_

#if MEDIAPIPE_WEB_PROFILING_ENABLED && __EMSCRIPTEN__
#include <emscripten/emscripten.h>

#include "absl/strings/str_cat.h"

// This records MediaPipe profiling events in the browser's performance trace.
// To use, build with:
//   --define MEDIAPIPE_PROFILING=1 --define MEDIAPIPE_WEB_PROFILING=1

namespace mediapipe {

class WepPerformanceTraceScope {
 public:
  explicit WepPerformanceTraceScope(TraceEvent::EventType event_type,
                                    const char* event_type_str,
                                    CalculatorContext* cc)
      : event_type_str_(event_type_str), cc_(cc) {
    const auto& calculator_name = cc->NodeName();
    std::string start_name =
        absl::StrCat(calculator_name, "::", event_type_str_, "_start");
    std::string timestamp_str = cc->InputTimestamp().DebugString();
    EM_ASM(
        {
          const startName = UTF8ToString($0);
          const timestamp = UTF8ToString($1);
          performance.mark(startName, {mp_timestamp : timestamp});
        },
        start_name.c_str(), timestamp_str.c_str());
  }

  ~WepPerformanceTraceScope() {
    const auto& calculator_name = cc_->NodeName();
    std::string start_name =
        absl::StrCat(calculator_name, "::", event_type_str_, "_start");
    std::string end_name =
        absl::StrCat(calculator_name, "::", event_type_str_, "_end");
    std::string measure_name =
        absl::StrCat(calculator_name, "::", event_type_str_);
    EM_ASM(
        {
          const startName = UTF8ToString($0);
          const endName = UTF8ToString($1);
          const measureName = UTF8ToString($2);
          performance.mark(endName);
          performance.measure(measureName, startName, endName);
        },
        start_name.c_str(), end_name.c_str(), measure_name.c_str());
  }

 private:
  const char* event_type_str_;
  CalculatorContext* cc_;
};

}  // namespace mediapipe

#define MEDIAPIPE_WEB_PERFORMANCE_SCOPE(event_type, calculator_context) \
  mediapipe::WepPerformanceTraceScope web_trace_scope(                  \
      mediapipe::TraceEvent::event_type, #event_type, calculator_context)

#else
#define MEDIAPIPE_WEB_PERFORMANCE_SCOPE(event_type, calculator_context)
#endif  // MEDIAPIPE_WEB_PROFILING_ENABLED && __EMSCRIPTEN__

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_WEB_PERFORMANCE_PROFILING_H_
