//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_IMPL_H
#define GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_IMPL_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/slice/slice_internal.h"
#include "src/core/service_config/service_config.h"
#include "src/core/service_config/service_config_parser.h"
#include "src/core/util/json/json.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/validation_errors.h"

// The main purpose of the code here is to parse the service config in
// JSON form, which will look like this:
//
// {
//   "loadBalancingPolicy": "string",  // optional
//   "methodConfig": [  // array of one or more method_config objects
//     {
//       "name": [  // array of one or more name objects
//         {
//           "service": "string",  // required
//           "method": "string",  // optional
//         }
//       ],
//       // remaining fields are optional.
//       // see https://developers.google.com/protocol-buffers/docs/proto3#json
//       // for format details.
//       "waitForReady": bool,
//       "timeout": "duration_string",
//       "maxRequestMessageBytes": "int64_string",
//       "maxResponseMessageBytes": "int64_string",
//     }
//   ]
// }

namespace grpc_core {

class ServiceConfigImpl final : public ServiceConfig {
 public:
  /// Creates a new service config from parsing \a json_string.
  static absl::StatusOr<RefCountedPtr<ServiceConfig>> Create(
      const ChannelArgs& args, absl::string_view json_string);

  // Alternate forms that are useful in edge cases.
  static RefCountedPtr<ServiceConfig> Create(const ChannelArgs& args,
                                             const Json& json,
                                             absl::string_view json_string,
                                             ValidationErrors* errors);
  static RefCountedPtr<ServiceConfig> Create(const ChannelArgs& args,
                                             const Json& json,
                                             ValidationErrors* errors);

  ~ServiceConfigImpl() override;

  absl::string_view json_string() const override { return json_string_; }

  /// Retrieves the global parsed config at index \a index. The
  /// lifetime of the returned object is tied to the lifetime of the
  /// ServiceConfig object.
  ServiceConfigParser::ParsedConfig* GetGlobalParsedConfig(
      size_t index) override {
    DCHECK(index < parsed_global_configs_.size());
    return parsed_global_configs_[index].get();
  }

  /// Retrieves the vector of parsed configs for the method identified
  /// by \a path.  The lifetime of the returned vector and contained objects
  /// is tied to the lifetime of the ServiceConfig object.
  const ServiceConfigParser::ParsedConfigVector* GetMethodParsedConfigVector(
      const grpc_slice& path) const override;

 private:
  std::string json_string_;
  Json json_;

  ServiceConfigParser::ParsedConfigVector parsed_global_configs_;
  // A map from the method name to the parsed config vector. Note that we are
  // using a raw pointer and not a unique pointer so that we can use the same
  // vector for multiple names.
  std::unordered_map<grpc_slice, const ServiceConfigParser::ParsedConfigVector*,
                     SliceHash>
      parsed_method_configs_map_;
  // Default method config.
  const ServiceConfigParser::ParsedConfigVector* default_method_config_vector_ =
      nullptr;
  // Storage for all the vectors that are being used in
  // parsed_method_configs_map_ and default_method_config_vector_.
  std::vector<ServiceConfigParser::ParsedConfigVector>
      parsed_method_config_vectors_storage_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_IMPL_H
