//
// Copyright 2021 the gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_TIME_UTIL_H
#define GRPC_SRC_CORE_UTIL_TIME_UTIL_H

#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>

#include "absl/time/time.h"

namespace grpc_core {

// Converts absl::Duration to gpr_timespec(GPR_TIMESPAN)
gpr_timespec ToGprTimeSpec(absl::Duration duration);

// Converts absl::Time to gpr_timespec(GPR_CLOCK_REALTIME)
gpr_timespec ToGprTimeSpec(absl::Time time);

// Converts gpr_timespec(GPR_TIMESPAN) to absl::Duration
absl::Duration ToAbslDuration(gpr_timespec ts);

// Converts gpr_timespec(GPR_CLOCK_[MONOTONIC|REALTIME|PRECISE]) to absl::Time
absl::Time ToAbslTime(gpr_timespec ts);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_TIME_UTIL_H
