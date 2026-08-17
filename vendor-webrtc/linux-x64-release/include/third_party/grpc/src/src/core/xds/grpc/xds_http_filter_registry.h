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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_FILTER_REGISTRY_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_FILTER_REGISTRY_H

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/call/interception_chain.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/grpc/xds_common_types.h"
#include "src/core/xds/grpc/xds_http_filter.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "upb/reflection/def.h"

namespace grpc_core {

// Exposed for testing purposes only.
class XdsHttpRouterFilter final : public XdsHttpFilterImpl {
 public:
  absl::string_view ConfigProtoName() const override;
  absl::string_view OverrideConfigProtoName() const override;
  void PopulateSymtab(upb_DefPool* symtab) const override;
  std::optional<FilterConfig> GenerateFilterConfig(
      absl::string_view /*instance_name*/,
      const XdsResourceType::DecodeContext& context, XdsExtension extension,
      ValidationErrors* errors) const override;
  std::optional<FilterConfig> GenerateFilterConfigOverride(
      absl::string_view /*instance_name*/,
      const XdsResourceType::DecodeContext& context, XdsExtension extension,
      ValidationErrors* errors) const override;
  void AddFilter(InterceptionChainBuilder& /*builder*/) const override {}
  const grpc_channel_filter* channel_filter() const override { return nullptr; }
  absl::StatusOr<ServiceConfigJsonEntry> GenerateMethodConfig(
      const FilterConfig& /*hcm_filter_config*/,
      const FilterConfig* /*filter_config_override*/) const override {
    // This will never be called, since channel_filter() returns null.
    return absl::UnimplementedError("router filter should never be called");
  }
  absl::StatusOr<ServiceConfigJsonEntry> GenerateServiceConfig(
      const FilterConfig& /*hcm_filter_config*/) const override {
    // This will never be called, since channel_filter() returns null.
    return absl::UnimplementedError("router filter should never be called");
  }
  bool IsSupportedOnClients() const override { return true; }
  bool IsSupportedOnServers() const override { return true; }
  bool IsTerminalFilter() const override { return true; }
};

class XdsHttpFilterRegistry final {
 public:
  explicit XdsHttpFilterRegistry(bool register_builtins = true);

  // Not copyable.
  XdsHttpFilterRegistry(const XdsHttpFilterRegistry&) = delete;
  XdsHttpFilterRegistry& operator=(const XdsHttpFilterRegistry&) = delete;

  // Movable.
  XdsHttpFilterRegistry(XdsHttpFilterRegistry&& other) noexcept
      : owning_list_(std::move(other.owning_list_)),
        registry_map_(std::move(other.registry_map_)) {}
  XdsHttpFilterRegistry& operator=(XdsHttpFilterRegistry&& other) noexcept {
    owning_list_ = std::move(other.owning_list_);
    registry_map_ = std::move(other.registry_map_);
    return *this;
  }

  void RegisterFilter(std::unique_ptr<XdsHttpFilterImpl> filter);

  const XdsHttpFilterImpl* GetFilterForType(
      absl::string_view proto_type_name) const;

  void PopulateSymtab(upb_DefPool* symtab) const;

 private:
  std::vector<std::unique_ptr<XdsHttpFilterImpl>> owning_list_;
  std::map<absl::string_view, XdsHttpFilterImpl*> registry_map_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_FILTER_REGISTRY_H
