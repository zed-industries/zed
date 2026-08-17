//
// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_RBAC_RBAC_SERVICE_CONFIG_PARSER_H
#define GRPC_SRC_CORE_EXT_FILTERS_RBAC_RBAC_SERVICE_CONFIG_PARSER_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <algorithm>
#include <memory>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/config/core_configuration.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/security/authorization/grpc_authorization_engine.h"
#include "src/core/lib/security/authorization/rbac_policy.h"
#include "src/core/service_config/service_config_parser.h"
#include "src/core/util/json/json.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {

// Channel arg key for enabling parsing RBAC via method config.
#define GRPC_ARG_PARSE_RBAC_METHOD_CONFIG \
  "grpc.internal.parse_rbac_method_config"

class RbacMethodParsedConfig : public ServiceConfigParser::ParsedConfig {
 public:
  explicit RbacMethodParsedConfig(std::vector<Rbac> rbac_policies) {
    for (auto& rbac_policy : rbac_policies) {
      authorization_engines_.emplace_back(std::move(rbac_policy));
    }
  }

  // Returns the authorization engine for a rbac policy at a certain index. For
  // a connection on the server, multiple RBAC policies might be active. The
  // RBAC filter uses this method to get the RBAC policy configured for a
  // instance at a particular instance.
  const GrpcAuthorizationEngine* authorization_engine(size_t index) const {
    if (index >= authorization_engines_.size()) {
      return nullptr;
    }
    return &authorization_engines_[index];
  }

 private:
  std::vector<GrpcAuthorizationEngine> authorization_engines_;
};

class RbacServiceConfigParser : public ServiceConfigParser::Parser {
 public:
  absl::string_view name() const override { return parser_name(); }
  // Parses the per-method service config for rbac filter.
  std::unique_ptr<ServiceConfigParser::ParsedConfig> ParsePerMethodParams(
      const ChannelArgs& args, const Json& json,
      ValidationErrors* errors) override;
  // Returns the parser index for RbacServiceConfigParser.
  static size_t ParserIndex();
  // Registers RbacServiceConfigParser to ServiceConfigParser.
  static void Register(CoreConfiguration::Builder* builder);

 private:
  static absl::string_view parser_name() { return "rbac"; }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_RBAC_RBAC_SERVICE_CONFIG_PARSER_H
