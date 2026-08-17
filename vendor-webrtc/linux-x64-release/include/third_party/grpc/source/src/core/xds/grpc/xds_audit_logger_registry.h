//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_AUDIT_LOGGER_REGISTRY_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_AUDIT_LOGGER_REGISTRY_H

#include <grpc/support/port_platform.h>

#include <map>
#include <memory>

#include "absl/strings/string_view.h"
#include "envoy/config/rbac/v3/rbac.upb.h"
#include "src/core/util/json/json.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/xds_client/xds_resource_type.h"

namespace grpc_core {

// A registry that maintains a set of converters that are able to map xDS
// RBAC audit logger configuration to gRPC's JSON format.
class XdsAuditLoggerRegistry final {
 public:
  class ConfigFactory {
   public:
    virtual ~ConfigFactory() = default;
    virtual Json::Object ConvertXdsAuditLoggerConfig(
        const XdsResourceType::DecodeContext& context,
        absl::string_view configuration, ValidationErrors* errors) = 0;
    // The full proto message name for the logger config.
    virtual absl::string_view type() = 0;
    // The logger name used for the gRPC registry.
    virtual absl::string_view name() = 0;
  };

  XdsAuditLoggerRegistry();

  Json ConvertXdsAuditLoggerConfig(
      const XdsResourceType::DecodeContext& context,
      const envoy_config_rbac_v3_RBAC_AuditLoggingOptions_AuditLoggerConfig*
          logger_config,
      ValidationErrors* errors) const;

 private:
  // A map of config factories that goes from the type of the audit logging
  // config to the config factory.
  std::map<absl::string_view /* Owned by ConfigFactory */,
           std::unique_ptr<ConfigFactory>>
      audit_logger_config_factories_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_AUDIT_LOGGER_REGISTRY_H
