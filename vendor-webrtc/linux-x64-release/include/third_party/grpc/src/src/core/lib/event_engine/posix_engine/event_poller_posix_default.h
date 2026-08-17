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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_EVENT_POLLER_POSIX_DEFAULT_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_EVENT_POLLER_POSIX_DEFAULT_H

#include <grpc/support/port_platform.h>

#include <memory>

namespace grpc_event_engine::experimental {

class PosixEventPoller;
class Scheduler;

// Return an instance of an event poller which is tied to the specified
// scheduler.
std::shared_ptr<PosixEventPoller> MakeDefaultPoller(Scheduler* scheduler);

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_EVENT_POLLER_POSIX_DEFAULT_H
