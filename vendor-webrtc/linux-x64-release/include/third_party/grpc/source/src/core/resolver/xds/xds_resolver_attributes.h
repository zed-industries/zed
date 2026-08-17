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

#ifndef GRPC_SRC_CORE_RESOLVER_XDS_XDS_RESOLVER_ATTRIBUTES_H
#define GRPC_SRC_CORE_RESOLVER_XDS_XDS_RESOLVER_ATTRIBUTES_H

#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"
#include "src/core/service_config/service_config_call_data.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/xds/grpc/xds_route_config.h"

namespace grpc_core {

class XdsClusterAttribute final
    : public ServiceConfigCallData::CallAttributeInterface {
 public:
  static UniqueTypeName TypeName() {
    static UniqueTypeName::Factory kFactory("xds_cluster_name");
    return kFactory.Create();
  }

  explicit XdsClusterAttribute(absl::string_view cluster) : cluster_(cluster) {}

  absl::string_view cluster() const { return cluster_; }
  void set_cluster(absl::string_view cluster) { cluster_ = cluster; }

 private:
  UniqueTypeName type() const override { return TypeName(); }

  absl::string_view cluster_;
};

class XdsRouteStateAttribute
    : public ServiceConfigCallData::CallAttributeInterface {
 public:
  static UniqueTypeName TypeName() {
    static UniqueTypeName::Factory factory("xds_route_state");
    return factory.Create();
  }

  UniqueTypeName type() const override { return TypeName(); }

  virtual bool HasClusterForRoute(absl::string_view cluster_name) const = 0;

  virtual const XdsRouteConfigResource::Route& route() const = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_XDS_XDS_RESOLVER_ATTRIBUTES_H
