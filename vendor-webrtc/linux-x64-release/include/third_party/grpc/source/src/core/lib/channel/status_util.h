//
//
// Copyright 2017 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_LIB_CHANNEL_STATUS_UTIL_H
#define GRPC_SRC_CORE_LIB_CHANNEL_STATUS_UTIL_H

#include <grpc/status.h>
#include <grpc/support/port_platform.h>

#include <string>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"

/// If \a status_str is a valid status string, sets \a status to the
/// corresponding status value and returns true.
bool grpc_status_code_from_string(const char* status_str,
                                  grpc_status_code* status);

/// Returns the string form of \a status, or "UNKNOWN" if invalid.
const char* grpc_status_code_to_string(grpc_status_code status);

// Converts an int to grpc_status_code. If the int is not a valid status code,
// sets the code to GRPC_STATUS_UNKNOWN and returns false. Otherwise, returns
// true.
bool grpc_status_code_from_int(int status_int, grpc_status_code* status);

namespace grpc_core {
namespace internal {

/// A set of grpc_status_code values.
class StatusCodeSet {
 public:
  bool Empty() const { return status_code_mask_ == 0; }

  StatusCodeSet& Add(grpc_status_code status) {
    status_code_mask_ |= (1 << status);
    return *this;
  }

  bool Contains(grpc_status_code status) const {
    return status_code_mask_ & (1 << status);
  }

  bool operator==(const StatusCodeSet& other) const {
    return status_code_mask_ == other.status_code_mask_;
  }

  std::string ToString() const;

 private:
  int status_code_mask_ = 0;  // A bitfield of status codes in the set.
};

}  // namespace internal

// Optionally rewrites a status as per
// https://github.com/grpc/proposal/blob/master/A54-restrict-control-plane-status-codes.md.
// The source parameter indicates where the status came from.
absl::Status MaybeRewriteIllegalStatusCode(absl::Status status,
                                           absl::string_view source);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_CHANNEL_STATUS_UTIL_H
