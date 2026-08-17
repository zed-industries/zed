#ifndef MEDIAPIPE_UTIL_LOG_FATAL_TO_BREAKPAD_H_
#define MEDIAPIPE_UTIL_LOG_FATAL_TO_BREAKPAD_H_

#include "absl/log/log_sink.h"

namespace mediapipe {

// Returns a singleton instance of a log sink that sends FATAL log messages to
// Breakpad. This log sink is enabled by default when this library is included
// in your binary.
absl::LogSink* GetBreakpadFatalLogSink();

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_LOG_FATAL_TO_BREAKPAD_H_
