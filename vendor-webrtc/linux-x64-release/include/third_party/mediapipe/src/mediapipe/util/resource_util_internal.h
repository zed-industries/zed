#ifndef MEDIAPIPE_UTIL_RESOURCE_UTIL_INTERNAL_H_
#define MEDIAPIPE_UTIL_RESOURCE_UTIL_INTERNAL_H_

#include <string>

#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace internal {

// Tries to return the contents of a file given the path. Implementation is
// platform-dependent.
absl::Status DefaultGetResourceContents(const std::string& path,
                                        std::string* output,
                                        bool read_as_binary);

}  // namespace internal
}  // namespace mediapipe
#endif  // MEDIAPIPE_UTIL_RESOURCE_UTIL_INTERNAL_H_
