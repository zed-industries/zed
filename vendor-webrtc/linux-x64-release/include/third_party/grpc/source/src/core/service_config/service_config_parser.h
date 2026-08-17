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

#ifndef GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_PARSER_H
#define GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_PARSER_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <algorithm>
#include <memory>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/json/json.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {

// Service config parser registry.
// See service_config.h for more information.
class ServiceConfigParser {
 public:
  /// This is the base class that all service config parsers MUST use to store
  /// parsed service config data.
  class ParsedConfig {
   public:
    virtual ~ParsedConfig() = default;
  };

  /// This is the base class that all service config parsers should derive from.
  class Parser {
   public:
    virtual ~Parser() = default;

    virtual absl::string_view name() const = 0;

    virtual std::unique_ptr<ParsedConfig> ParseGlobalParams(
        const ChannelArgs& /*args*/, const Json& /*json*/,
        ValidationErrors* /*errors*/) {
      return nullptr;
    }

    virtual std::unique_ptr<ParsedConfig> ParsePerMethodParams(
        const ChannelArgs& /*args*/, const Json& /*json*/,
        ValidationErrors* /*errors*/) {
      return nullptr;
    }
  };

  using ServiceConfigParserList = std::vector<std::unique_ptr<Parser>>;
  using ParsedConfigVector = std::vector<std::unique_ptr<ParsedConfig>>;

  class Builder final {
   public:
    /// Globally register a service config parser. Each new service config
    /// update will go through all the registered parser. Each parser is
    /// responsible for reading the service config json and returning a parsed
    /// config.
    void RegisterParser(std::unique_ptr<Parser> parser);

    ServiceConfigParser Build();

   private:
    ServiceConfigParserList registered_parsers_;
  };

  ParsedConfigVector ParseGlobalParameters(const ChannelArgs& args,
                                           const Json& json,
                                           ValidationErrors* errors) const;

  ParsedConfigVector ParsePerMethodParameters(const ChannelArgs& args,
                                              const Json& json,
                                              ValidationErrors* errors) const;

  // Return the index for a given registered parser.
  // If there is an error, return -1.
  size_t GetParserIndex(absl::string_view name) const;

 private:
  explicit ServiceConfigParser(ServiceConfigParserList registered_parsers)
      : registered_parsers_(std::move(registered_parsers)) {}
  ServiceConfigParserList registered_parsers_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_SERVICE_CONFIG_SERVICE_CONFIG_PARSER_H
