// Copyright 2025 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TRANSPORT_CONTEXT_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TRANSPORT_CONTEXT_H

#include <grpc/event_engine/event_engine.h>

#include "src/core/channelz/channelz.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/telemetry/metrics.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core::chaotic_good {

struct TransportContext : public RefCounted<TransportContext> {
  TransportContext(const ChannelArgs& args,
                   RefCountedPtr<channelz::SocketNode> socket_node)
      : event_engine(
            args.GetObjectRef<grpc_event_engine::experimental::EventEngine>()),
        stats_plugin_group(
            args.GetObjectRef<GlobalStatsPluginRegistry::StatsPluginGroup>()),
        socket_node(std::move(socket_node)) {
    CHECK(this->event_engine != nullptr);
  }
  TransportContext(std::shared_ptr<grpc_event_engine::experimental::EventEngine>
                       event_engine,
                   RefCountedPtr<channelz::SocketNode> socket_node)
      : event_engine(std::move(event_engine)),
        stats_plugin_group(nullptr),
        socket_node(std::move(socket_node)) {
    CHECK(this->event_engine != nullptr);
  }
  const std::shared_ptr<grpc_event_engine::experimental::EventEngine>
      event_engine;
  const std::shared_ptr<GlobalStatsPluginRegistry::StatsPluginGroup>
      stats_plugin_group;
  const RefCountedPtr<channelz::SocketNode> socket_node;
};

using TransportContextPtr = RefCountedPtr<TransportContext>;

}  // namespace grpc_core::chaotic_good

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TRANSPORT_CONTEXT_H
