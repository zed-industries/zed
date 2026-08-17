// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_TENSORFLOW_TEXT_SHIMS_TENSORFLOW_CORE_LIB_CORE_ERRORS_H_
#define THIRD_PARTY_TENSORFLOW_TEXT_SHIMS_TENSORFLOW_CORE_LIB_CORE_ERRORS_H_

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"

#define TF_PREDICT_FALSE(x) (x)
#define TF_PREDICT_TRUE(x) (x)

// For propagating errors when calling a function.
#define TF_RETURN_IF_ERROR(...)             \
  do {                                      \
    ::absl::Status _status = (__VA_ARGS__); \
    if (TF_PREDICT_FALSE(!_status.ok())) {  \
      return _status;                       \
    }                                       \
  } while (0)

namespace tensorflow::errors {

template <typename Arg1, typename Arg2>
::absl::Status InvalidArgument(Arg1 arg1, Arg2 arg2) {
  return absl::Status(absl::StatusCode::kInvalidArgument,
                      absl::StrCat(arg1, arg2));
}

template <typename Arg1, typename Arg2, typename Arg3, typename Arg4>
::absl::Status InvalidArgument(Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4) {
  return absl::Status(absl::StatusCode::kInvalidArgument,
                      absl::StrCat(arg1, arg2, arg3, arg4));
}

template <typename... Args>
absl::Status FailedPrecondition(Args... args) {
  return absl::Status(absl::StatusCode::kFailedPrecondition,
                      absl::StrCat(args...));
}

}  // namespace tensorflow::errors

#endif  // THIRD_PARTY_TENSORFLOW_TEXT_SHIMS_TENSORFLOW_CORE_LIB_CORE_ERRORS_H_
