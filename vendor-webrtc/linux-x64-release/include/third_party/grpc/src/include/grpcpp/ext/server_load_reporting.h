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

#ifndef GRPCPP_EXT_SERVER_LOAD_REPORTING_H
#define GRPCPP_EXT_SERVER_LOAD_REPORTING_H

#include <grpc/load_reporting.h>
#include <grpc/support/port_platform.h>
#include <grpcpp/impl/server_builder_option.h>
#include <grpcpp/server_context.h>
#include <grpcpp/support/config.h>

namespace grpc {
namespace load_reporter {
namespace experimental {

// The ServerBuilderOption to enable server-side load reporting feature. To
// enable the feature, please make sure the binary builds with the
// grpcpp_server_load_reporting library and set this option in the
// ServerBuilder.
class LoadReportingServiceServerBuilderOption
    : public grpc::ServerBuilderOption {
 public:
  void UpdateArguments(grpc::ChannelArguments* args) override;
  void UpdatePlugins(std::vector<std::unique_ptr<grpc::ServerBuilderPlugin>>*
                         plugins) override;
};

// Adds the load reporting cost with \a cost_name and \a cost_value in the
// trailing metadata of the server context.
void AddLoadReportingCost(grpc::ServerContext* ctx,
                          const std::string& cost_name, double cost_value);

}  // namespace experimental
}  // namespace load_reporter
}  // namespace grpc

#endif  // GRPCPP_EXT_SERVER_LOAD_REPORTING_H
