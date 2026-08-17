#ifndef MEDIAPIPE_FRAMEWORK_VLOG_UTILS_H_
#define MEDIAPIPE_FRAMEWORK_VLOG_UTILS_H_

#include "absl/strings/string_view.h"

namespace mediapipe {

// Helper to log a message with a large number of lines on mobile (Android).
//
// On Android, the logcat will truncate the log if the message is larger than
// 4096 bytes. This function splits the message by new lines and logs each
// line separately. To ensure the log message is only generated when VLOG is
// turned on, use this function in a VLOG_IS_ON() block:
// if (VLOG_IS_ON(1)) {
//   VlogLargeMessage(
//       /*verbose_level=*/1, GenerateDebugString());
// }
void VlogLargeMessage(int verbose_level, absl::string_view message);

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_VLOG_UTILS_H_
