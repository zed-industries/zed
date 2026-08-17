//
//
// Copyright 2015 gRPC authors.
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
#ifndef GRPC_SRC_CORE_LIB_SURFACE_INIT_H
#define GRPC_SRC_CORE_LIB_SURFACE_INIT_H

#include <grpc/support/port_platform.h>

#include "absl/time/time.h"

void grpc_maybe_wait_for_async_shutdown(void);

// Returns false if the timeout expired before fully shut down.
bool grpc_wait_for_shutdown_with_timeout(absl::Duration timeout);

#endif  // GRPC_SRC_CORE_LIB_SURFACE_INIT_H
