//
// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_GCP_AUTHENTICATION_GCP_AUTHENTICATION_SERVICE_CONFIG_PARSER_H
#define GRPC_SRC_CORE_EXT_FILTERS_GCP_AUTHENTICATION_GCP_AUTHENTICATION_SERVICE_CONFIG_PARSER_H

#include <stddef.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/config/core_configuration.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/service_config/service_config_parser.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/validation_errors.h"

// Channel arg key for enabling parsing fault injection via method config.
#define GRPC_ARG_PARSE_GCP_AUTHENTICATION_METHOD_CONFIG \
  "grpc.internal.parse_gcp_authentication_method_config"

namespace grpc_core {

class GcpAuthenticationParsedConfig : public ServiceConfigParser::ParsedConfig {
 public:
  struct Config {
    std::string filter_instance_name;
    uint64_t cache_size = 10;

    static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
    void JsonPostLoad(const Json&, const JsonArgs&, ValidationErrors* errors);
  };

  // Returns the config at the specified index.  There might be multiple
  // GCP auth filters in the list of HTTP filters at the same time.
  // The order of the list is stable, and an index is used to keep track of
  // their relative positions.  Each filter instance uses this method to
  // access the appropriate parsed config for that instance.
  const Config* GetConfig(size_t index) const {
    if (index >= configs_.size()) return nullptr;
    return &configs_[index];
  }

  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);

 private:
  std::vector<Config> configs_;
};

class GcpAuthenticationServiceConfigParser final
    : public ServiceConfigParser::Parser {
 public:
  absl::string_view name() const override { return parser_name(); }
  std::unique_ptr<ServiceConfigParser::ParsedConfig> ParseGlobalParams(
      const ChannelArgs& args, const Json& json,
      ValidationErrors* errors) override;
  // Returns the parser index for the parser.
  static size_t ParserIndex();
  // Registers the parser.
  static void Register(CoreConfiguration::Builder* builder);

 private:
  static absl::string_view parser_name() { return "gcp_auth"; }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_GCP_AUTHENTICATION_GCP_AUTHENTICATION_SERVICE_CONFIG_PARSER_H
