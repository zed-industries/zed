#ifndef MEDIAPIPE_TASKS_CC_METADATA_METADATA_VERSION_UTIL_H_
#define MEDIAPIPE_TASKS_CC_METADATA_METADATA_VERSION_UTIL_H_

#include "absl/strings/string_view.h"

namespace mediapipe {
namespace tasks {
namespace metadata {

// Compares two versions. The version format is "**.**.**" such as "1.12.3".
// If version_a is newer than version_b, return 1; if version_a is
// older than version_b, return -1; if version_a equals to version_b,
// returns 0. For example, if version_a = 1.12.3 and version_b = 1.12.1,
// version_a is newer than version_b, and the function return is 1.
int CompareVersions(absl::string_view version_a, absl::string_view version_b);

}  // namespace metadata
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_METADATA_METADATA_VERSION_UTIL_H_
