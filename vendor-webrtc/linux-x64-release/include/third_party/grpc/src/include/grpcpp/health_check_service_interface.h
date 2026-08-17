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

#ifndef GRPCPP_HEALTH_CHECK_SERVICE_INTERFACE_H
#define GRPCPP_HEALTH_CHECK_SERVICE_INTERFACE_H

#include <grpcpp/support/config.h>

namespace grpc {

const char kHealthCheckServiceInterfaceArg[] =
    "grpc.health_check_service_interface";

/// The gRPC server uses this interface to expose the health checking service
/// without depending on protobuf.
class HealthCheckServiceInterface {
 public:
  virtual ~HealthCheckServiceInterface() {}

  /// Set or change the serving status of the given \a service_name.
  virtual void SetServingStatus(const std::string& service_name,
                                bool serving) = 0;
  /// Apply to all registered service names.
  virtual void SetServingStatus(bool serving) = 0;

  /// Set all registered service names to not serving and prevent future
  /// state changes.
  virtual void Shutdown() {}
};

/// Enable/disable the default health checking service. This applies to all C++
/// servers created afterwards. For each server, user can override the default
/// with a HealthCheckServiceServerBuilderOption.
/// NOT thread safe.
void EnableDefaultHealthCheckService(bool enable);

/// Returns whether the default health checking service is enabled.
/// NOT thread safe.
bool DefaultHealthCheckServiceEnabled();

}  // namespace grpc

#endif  // GRPCPP_HEALTH_CHECK_SERVICE_INTERFACE_H
