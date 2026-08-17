// Copyright 2025 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_EXTENSIONS_CHANNELZ_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_EXTENSIONS_CHANNELZ_H

#include <memory>
#include <utility>

#include "absl/strings/string_view.h"
#include "src/core/channelz/channelz.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_event_engine::experimental {

class ChannelzExtension {
 public:
  virtual ~ChannelzExtension() = default;
  static absl::string_view EndpointExtensionName() {
    return "io.grpc.event_engine.extension.channelz";
  }
  virtual void AddJson(grpc_core::channelz::DataSink& sink) = 0;

  void SetSocketNode(
      grpc_core::RefCountedPtr<grpc_core::channelz::SocketNode> socket_node) {
    data_source_ =
        std::make_unique<EndpointDataSource>(std::move(socket_node), this);
  }

 private:
  class EndpointDataSource final : public grpc_core::channelz::DataSource {
   public:
    EndpointDataSource(
        grpc_core::RefCountedPtr<grpc_core::channelz::SocketNode> socket_node,
        ChannelzExtension* ep)
        : grpc_core::channelz::DataSource(std::move(socket_node)), ep_(ep) {}
    void AddData(grpc_core::channelz::DataSink& sink) override {
      ep_->AddJson(sink);
    }

   private:
    ChannelzExtension* ep_;
  };

  std::unique_ptr<EndpointDataSource> data_source_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_EXTENSIONS_CHANNELZ_H
