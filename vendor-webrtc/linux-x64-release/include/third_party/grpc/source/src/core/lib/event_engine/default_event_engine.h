// Copyright 2021 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_DEFAULT_EVENT_ENGINE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_DEFAULT_EVENT_ENGINE_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <memory>

#include "src/core/config/core_configuration.h"
#include "src/core/util/debug_location.h"

namespace grpc_event_engine::experimental {

/// On ingress, ensure that an EventEngine exists in channel args via
/// preconditioning.
void RegisterEventEngineChannelArgPreconditioning(
    grpc_core::CoreConfiguration::Builder* builder);

/// Register a default EventEngine that is reset and destroyed when this object
/// is destroyed.
///
/// Usage:
///
///   {
///     DefaultEventEngineScope engine_holder(std::make_shared<MyEngine>());
///     // returns the instance of MyEngine
///     auto engine = GetDefaultEventEngine();
///   }
///   // returns some default internal instance. The previous instance has been
///   // destroyed.
///   auto engine = GetDefaultEventEngine();
///
class DefaultEventEngineScope {
 public:
  explicit DefaultEventEngineScope(std::shared_ptr<EventEngine> engine) {
    SetDefaultEventEngine(std::move(engine));
  }
  ~DefaultEventEngineScope() { ShutdownDefaultEventEngine(); }
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_DEFAULT_EVENT_ENGINE_H
