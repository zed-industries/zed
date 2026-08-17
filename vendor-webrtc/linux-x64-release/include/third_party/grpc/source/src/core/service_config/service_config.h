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

#ifndef GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_H
#define GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "absl/strings/string_view.h"
#include "src/core/service_config/service_config_parser.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/useful.h"

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

#define GRPC_ARG_SERVICE_CONFIG_OBJ "grpc.internal.service_config_obj"

namespace grpc_core {

// TODO(roth): Consider stripping this down further to the completely minimal
// interface required to be exposed as part of the resolver API.
class ServiceConfig : public RefCounted<ServiceConfig> {
 public:
  static absl::string_view ChannelArgName() {
    return GRPC_ARG_SERVICE_CONFIG_OBJ;
  }
  static int ChannelArgsCompare(const ServiceConfig* a,
                                const ServiceConfig* b) {
    return QsortCompare(a, b);
  }

  virtual absl::string_view json_string() const = 0;

  /// Retrieves the global parsed config at index \a index. The
  /// lifetime of the returned object is tied to the lifetime of the
  /// ServiceConfig object.
  virtual ServiceConfigParser::ParsedConfig* GetGlobalParsedConfig(
      size_t index) = 0;

  /// Retrieves the vector of parsed configs for the method identified
  /// by \a path.  The lifetime of the returned vector and contained objects
  /// is tied to the lifetime of the ServiceConfig object.
  virtual const ServiceConfigParser::ParsedConfigVector*
  GetMethodParsedConfigVector(const grpc_slice& path) const = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_H
