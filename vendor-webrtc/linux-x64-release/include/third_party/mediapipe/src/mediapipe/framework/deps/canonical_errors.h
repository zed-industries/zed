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

#ifndef MEDIAPIPE_DEPS_CANONICAL_ERRORS_H_
#define MEDIAPIPE_DEPS_CANONICAL_ERRORS_H_

#include "mediapipe/framework/deps/status.h"

namespace mediapipe {

// Each of the functions below creates a canonical error with the given
// message. The error code of the returned status object matches the name of
// the function.
inline absl::Status AlreadyExistsError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kAlreadyExists, message);
}

inline absl::Status CancelledError() {
  return absl::Status(absl::StatusCode::kCancelled, "");
}

inline absl::Status CancelledError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kCancelled, message);
}

inline absl::Status InternalError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kInternal, message);
}

inline absl::Status InvalidArgumentError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kInvalidArgument, message);
}

inline absl::Status FailedPreconditionError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kFailedPrecondition, message);
}

inline absl::Status NotFoundError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kNotFound, message);
}

inline absl::Status OutOfRangeError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kOutOfRange, message);
}

inline absl::Status PermissionDeniedError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kPermissionDenied, message);
}

inline absl::Status UnimplementedError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kUnimplemented, message);
}

inline absl::Status UnknownError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kUnknown, message);
}

inline absl::Status UnavailableError(absl::string_view message) {
  return absl::Status(absl::StatusCode::kUnavailable, message);
}

inline bool IsCancelled(const absl::Status& status) {
  return status.code() == absl::StatusCode::kCancelled;
}

inline bool IsNotFound(const absl::Status& status) {
  return status.code() == absl::StatusCode::kNotFound;
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_CANONICAL_ERRORS_H_
