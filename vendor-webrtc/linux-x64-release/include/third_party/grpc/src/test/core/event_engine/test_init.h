// Copyright 2022 The gRPC Authors
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
#ifndef GRPC_TEST_CORE_EVENT_ENGINE_TEST_INIT_H
#define GRPC_TEST_CORE_EVENT_ENGINE_TEST_INIT_H
#include <grpc/support/port_platform.h>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"

namespace grpc_event_engine {
namespace experimental {

absl::Status InitializeTestingEventEngineFactory(absl::string_view engine);

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_EVENT_ENGINE_TEST_INIT_H
