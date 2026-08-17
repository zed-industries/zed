// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_DEPS_STATUS_H_
#define MEDIAPIPE_DEPS_STATUS_H_

#include <functional>
#include <iosfwd>
#include <memory>
#include <string>

#include "absl/base/attributes.h"
#include "absl/log/absl_log.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"

namespace mediapipe {

using Status ABSL_DEPRECATED("Use absl::Status directly") = absl::Status;
using StatusCode ABSL_DEPRECATED("Use absl::StatusCode directly") =
    absl::StatusCode;

ABSL_DEPRECATED("Use absl::OkStatus directly")
inline absl::Status OkStatus() { return absl::OkStatus(); }

extern std::string* MediaPipeCheckOpHelperOutOfLine(const absl::Status& v,
                                                    const char* msg);

inline std::string* MediaPipeCheckOpHelper(absl::Status v, const char* msg) {
  if (v.ok()) return nullptr;
  return MediaPipeCheckOpHelperOutOfLine(v, msg);
}

#define MEDIAPIPE_DO_CHECK_OK(val, level)                             \
  while (auto _result = mediapipe::MediaPipeCheckOpHelper(val, #val)) \
  ABSL_LOG(level) << *(_result)

#define MEDIAPIPE_CHECK_OK(val) MEDIAPIPE_DO_CHECK_OK(val, FATAL)
#define MEDIAPIPE_QCHECK_OK(val) MEDIAPIPE_DO_CHECK_OK(val, QFATAL)

#ifndef NDEBUG
#define MEDIAPIPE_DCHECK_OK(val) MEDIAPIPE_CHECK_OK(val)
#else
#define MEDIAPIPE_DCHECK_OK(val) \
  while (false && (absl::OkStatus() == (val))) ABSL_LOG(FATAL)
#endif

#define CHECK_OK MEDIAPIPE_CHECK_OK
#define QCHECK_OK MEDIAPIPE_QCHECK_OK
#define DCHECK_OK MEDIAPIPE_DCHECK_OK

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_STATUS_H_
