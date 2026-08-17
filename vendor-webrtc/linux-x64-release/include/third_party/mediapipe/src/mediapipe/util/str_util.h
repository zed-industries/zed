#ifndef MEDIAPIPE_UTIL_STR_UTIL_H_
#define MEDIAPIPE_UTIL_STR_UTIL_H_

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"

namespace mediapipe {

// Invokes `fn` for each line in `utf8_text`. (Separators used when looking for
// lines: "\r", "\n", "\r\n" or <EOF>)
void ForEachLine(absl::string_view utf8_text,
                 absl::AnyInvocable<void(absl::string_view line)> fn);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_STR_UTIL_H_
