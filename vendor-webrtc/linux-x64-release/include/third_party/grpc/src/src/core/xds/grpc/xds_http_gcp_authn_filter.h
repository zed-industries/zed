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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_GCP_AUTHN_FILTER_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_GCP_AUTHN_FILTER_H

#include <optional>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/grpc/xds_common_types.h"
#include "src/core/xds/grpc/xds_http_filter.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "upb/reflection/def.h"

namespace grpc_core {

class XdsHttpGcpAuthnFilter final : public XdsHttpFilterImpl {
 public:
  absl::string_view ConfigProtoName() const override;
  absl::string_view OverrideConfigProtoName() const override;
  void PopulateSymtab(upb_DefPool* symtab) const override;
  std::optional<FilterConfig> GenerateFilterConfig(
      absl::string_view instance_name,
      const XdsResourceType::DecodeContext& context, XdsExtension extension,
      ValidationErrors* errors) const override;
  std::optional<FilterConfig> GenerateFilterConfigOverride(
      absl::string_view instance_name,
      const XdsResourceType::DecodeContext& context, XdsExtension extension,
      ValidationErrors* errors) const override;
  void AddFilter(InterceptionChainBuilder& builder) const override;
  const grpc_channel_filter* channel_filter() const override;
  ChannelArgs ModifyChannelArgs(const ChannelArgs& args) const override;
  absl::StatusOr<ServiceConfigJsonEntry> GenerateMethodConfig(
      const FilterConfig& hcm_filter_config,
      const FilterConfig* filter_config_override) const override;
  absl::StatusOr<ServiceConfigJsonEntry> GenerateServiceConfig(
      const FilterConfig& hcm_filter_config) const override;
  bool IsSupportedOnClients() const override { return true; }
  bool IsSupportedOnServers() const override { return false; }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_HTTP_GCP_AUTHN_FILTER_H
