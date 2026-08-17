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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_METADATA_PARSER_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_METADATA_PARSER_H

#include "envoy/config/core/v3/base.upb.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/grpc/xds_metadata.h"
#include "src/core/xds/xds_client/xds_resource_type.h"

namespace grpc_core {

bool XdsGcpAuthFilterEnabled();

XdsMetadataMap ParseXdsMetadataMap(
    const XdsResourceType::DecodeContext& context,
    const envoy_config_core_v3_Metadata* metadata, ValidationErrors* errors);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_METADATA_PARSER_H
