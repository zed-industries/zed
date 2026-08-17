//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPCPP_SUPPORT_VALIDATE_SERVICE_CONFIG_H
#define GRPCPP_SUPPORT_VALIDATE_SERVICE_CONFIG_H

#include <grpcpp/support/config.h>

namespace grpc {

namespace experimental {
/// Validates \a service_config_json. If valid, returns an empty string.
/// Otherwise, returns the validation error.
/// TODO(yashykt): Promote it to out of experimental once it is proved useful
/// and gRFC is accepted.
std::string ValidateServiceConfigJSON(const std::string& service_config_json);
}  // namespace experimental

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_VALIDATE_SERVICE_CONFIG_H
