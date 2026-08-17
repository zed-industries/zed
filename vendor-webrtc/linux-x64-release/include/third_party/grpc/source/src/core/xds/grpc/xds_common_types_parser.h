//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_COMMON_TYPES_PARSER_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_COMMON_TYPES_PARSER_H

#include <optional>

#include "envoy/config/core/v3/base.upb.h"
#include "envoy/extensions/transport_sockets/tls/v3/tls.upb.h"
#include "google/protobuf/any.upb.h"
#include "google/protobuf/duration.upb.h"
#include "google/protobuf/struct.upb.h"
#include "google/protobuf/wrappers.upb.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/util/time.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/grpc/xds_common_types.h"
#include "src/core/xds/xds_client/xds_resource_type.h"

namespace grpc_core {

Duration ParseDuration(const google_protobuf_Duration* proto_duration,
                       ValidationErrors* errors);

inline bool ParseBoolValue(const google_protobuf_BoolValue* bool_value_proto,
                           bool default_value = false) {
  if (bool_value_proto == nullptr) return default_value;
  return google_protobuf_BoolValue_value(bool_value_proto);
}

inline std::optional<uint64_t> ParseUInt64Value(
    const google_protobuf_UInt64Value* proto) {
  if (proto == nullptr) return std::nullopt;
  return google_protobuf_UInt64Value_value(proto);
}

inline std::optional<uint32_t> ParseUInt32Value(
    const google_protobuf_UInt32Value* proto) {
  if (proto == nullptr) return std::nullopt;
  return google_protobuf_UInt32Value_value(proto);
}

// Returns the IP address in URI form.
std::optional<grpc_resolved_address> ParseXdsAddress(
    const envoy_config_core_v3_Address* address, ValidationErrors* errors);

CommonTlsContext CommonTlsContextParse(
    const XdsResourceType::DecodeContext& context,
    const envoy_extensions_transport_sockets_tls_v3_CommonTlsContext*
        common_tls_context_proto,
    ValidationErrors* errors);

absl::StatusOr<Json> ParseProtobufStructToJson(
    const XdsResourceType::DecodeContext& context,
    const google_protobuf_Struct* resource);

std::optional<XdsExtension> ExtractXdsExtension(
    const XdsResourceType::DecodeContext& context,
    const google_protobuf_Any* any, ValidationErrors* errors);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_COMMON_TYPES_PARSER_H
