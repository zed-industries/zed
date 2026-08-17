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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_FILTER_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_FILTER_H

#include <optional>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "src/core/call/interception_chain.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_writer.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/grpc/xds_common_types.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "upb/reflection/def.h"

namespace grpc_core {

class XdsHttpFilterImpl {
 public:
  struct FilterConfig {
    absl::string_view config_proto_type_name;
    Json config;

    bool operator==(const FilterConfig& other) const {
      return config_proto_type_name == other.config_proto_type_name &&
             config == other.config;
    }
    std::string ToString() const {
      return absl::StrCat("{config_proto_type_name=", config_proto_type_name,
                          " config=", JsonDump(config), "}");
    }
  };

  // Service config data for the filter, returned by GenerateServiceConfig().
  struct ServiceConfigJsonEntry {
    // The top-level field name in the method config.
    // Filter implementations should use their primary config proto type
    // name for this.
    // The value of this field in the method config will be a JSON array,
    // which will be populated with the elements returned by each filter
    // instance.
    // Entry will be skipped if this field is empty.
    std::string service_config_field_name;
    // The element to add to the JSON array.
    std::string element;
  };

  virtual ~XdsHttpFilterImpl() = default;

  // Returns the top-level filter config proto message name.
  virtual absl::string_view ConfigProtoName() const = 0;

  // Returns the override filter config proto message name.
  // If empty, no override type is supported.
  virtual absl::string_view OverrideConfigProtoName() const = 0;

  // Loads the proto message into the upb symtab.
  virtual void PopulateSymtab(upb_DefPool* symtab) const = 0;

  // Generates a Config from the xDS filter config proto.
  // Used for the top-level config in the HCM HTTP filter list.
  virtual std::optional<FilterConfig> GenerateFilterConfig(
      absl::string_view instance_name,
      const XdsResourceType::DecodeContext& context, XdsExtension extension,
      ValidationErrors* errors) const = 0;

  // Generates a Config from the xDS filter config proto.
  // Used for the typed_per_filter_config override in VirtualHost and Route.
  virtual std::optional<FilterConfig> GenerateFilterConfigOverride(
      absl::string_view instance_name,
      const XdsResourceType::DecodeContext& context, XdsExtension extension,
      ValidationErrors* errors) const = 0;

  // C-core channel filter implementation.
  virtual void AddFilter(InterceptionChainBuilder& builder) const = 0;
  // TODO(roth): Remove this once the legacy filter stack goes away.
  virtual const grpc_channel_filter* channel_filter() const = 0;

  // Modifies channel args that may affect service config parsing (not
  // visible to the channel as a whole).
  virtual ChannelArgs ModifyChannelArgs(const ChannelArgs& args) const {
    return args;
  }

  // Function to convert the Configs into a JSON string to be added to the
  // per-method part of the service config.
  // The hcm_filter_config comes from the HttpConnectionManager config.
  // The filter_config_override comes from the first of the ClusterWeight,
  // Route, or VirtualHost entries that it is found in, or null if
  // there is no override in any of those locations.
  virtual absl::StatusOr<ServiceConfigJsonEntry> GenerateMethodConfig(
      const FilterConfig& hcm_filter_config,
      const FilterConfig* filter_config_override) const = 0;

  // Function to convert the Configs into a JSON string to be added to the
  // top level of the service config.
  // The hcm_filter_config comes from the HttpConnectionManager config.
  // Currently used only on the client side.
  virtual absl::StatusOr<ServiceConfigJsonEntry> GenerateServiceConfig(
      const FilterConfig& hcm_filter_config) const = 0;

  // Returns true if the filter is supported on clients; false otherwise
  virtual bool IsSupportedOnClients() const = 0;

  // Returns true if the filter is supported on servers; false otherwise
  virtual bool IsSupportedOnServers() const = 0;

  // Returns true if the filter must be the last filter in the chain.
  virtual bool IsTerminalFilter() const { return false; }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_FILTER_H
