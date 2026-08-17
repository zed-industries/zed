//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTING_SERVICE_SERVER_BUILDER_PLUGIN_H
#define GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTING_SERVICE_SERVER_BUILDER_PLUGIN_H

#include <grpc/support/port_platform.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/impl/server_builder_plugin.h>
#include <grpcpp/server_builder.h>
#include <grpcpp/support/channel_arguments.h>

#include <memory>
#include <string>

#include "src/cpp/server/load_reporter/load_reporter_async_service_impl.h"

namespace grpc {
namespace load_reporter {

// The plugin that registers and starts load reporting service when starting a
// server.
class LoadReportingServiceServerBuilderPlugin : public ServerBuilderPlugin {
 public:
  ~LoadReportingServiceServerBuilderPlugin() override = default;
  std::string name() override { return "load_reporting_service"; }

  // Creates a load reporting service.
  void UpdateServerBuilder(ServerBuilder* builder) override;

  // Registers the load reporter service.
  void InitServer(ServerInitializer* si) override;

  // Starts the load reporter service.
  void Finish(ServerInitializer* si) override;

  void ChangeArguments(const std::string& /*name*/, void* /*value*/) override {}
  void UpdateChannelArguments(grpc::ChannelArguments* /*args*/) override {}
  bool has_sync_methods() const override;
  bool has_async_methods() const override;

 private:
  std::shared_ptr<LoadReporterAsyncServiceImpl> service_;
};

std::unique_ptr<grpc::ServerBuilderPlugin>
CreateLoadReportingServiceServerBuilderPlugin();

}  // namespace load_reporter
}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTING_SERVICE_SERVER_BUILDER_PLUGIN_H
