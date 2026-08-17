//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPCPP_EXT_HEALTH_CHECK_SERVICE_SERVER_BUILDER_OPTION_H
#define GRPCPP_EXT_HEALTH_CHECK_SERVICE_SERVER_BUILDER_OPTION_H

#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/impl/server_builder_option.h>
#include <grpcpp/support/config.h>

#include <memory>

namespace grpc {

class HealthCheckServiceServerBuilderOption : public ServerBuilderOption {
 public:
  /// The ownership of \a hc will be taken and transferred to the grpc server.
  /// To explicitly disable default service, pass in a nullptr.
  explicit HealthCheckServiceServerBuilderOption(
      std::unique_ptr<HealthCheckServiceInterface> hc);
  ~HealthCheckServiceServerBuilderOption() override {}
  void UpdateArguments(ChannelArguments* args) override;
  void UpdatePlugins(
      std::vector<std::unique_ptr<ServerBuilderPlugin>>* plugins) override;

 private:
  std::unique_ptr<HealthCheckServiceInterface> hc_;
};

}  // namespace grpc

#endif  // GRPCPP_EXT_HEALTH_CHECK_SERVICE_SERVER_BUILDER_OPTION_H
