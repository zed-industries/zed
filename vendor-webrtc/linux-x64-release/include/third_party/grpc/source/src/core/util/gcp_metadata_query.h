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

#ifndef GRPC_SRC_CORE_UTIL_GCP_METADATA_QUERY_H
#define GRPC_SRC_CORE_UTIL_GCP_METADATA_QUERY_H

#include <grpc/support/port_platform.h>

#include <string>

#include "absl/functional/any_invocable.h"
#include "absl/status/statusor.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/util/http_client/httpcli.h"
#include "src/core/util/http_client/parser.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/time.h"

namespace grpc_core {

// Fetches the value of an attribute from the MetadataServer on a GCP
// environment.
class GcpMetadataQuery : public InternallyRefCounted<GcpMetadataQuery> {
 public:
  static constexpr const char kZoneAttribute[] =
      "/computeMetadata/v1/instance/zone";
  static constexpr const char kClusterNameAttribute[] =
      "/computeMetadata/v1/instance/attributes/cluster-name";
  static constexpr const char kRegionAttribute[] =
      "/computeMetadata/v1/instance/region";
  static constexpr const char kInstanceIdAttribute[] =
      "/computeMetadata/v1/instance/id";
  static constexpr const char kIPv6Attribute[] =
      "/computeMetadata/v1/instance/network-interfaces/0/ipv6s";

  GcpMetadataQuery(
      std::string attribute, grpc_polling_entity* pollent,
      absl::AnyInvocable<void(std::string /* attribute */,
                              absl::StatusOr<std::string> /* result */)>
          callback,
      Duration timeout);

  // Alternative ctor allows overriding the metadata server address, mainly
  // to inject fakes in tests
  GcpMetadataQuery(
      std::string metadata_server_name, std::string attribute,
      grpc_polling_entity* pollent,
      absl::AnyInvocable<void(std::string /* attribute */,
                              absl::StatusOr<std::string> /* result */)>
          callback,
      Duration timeout);

  ~GcpMetadataQuery() override;

  void Orphan() override;

 private:
  static void OnDone(void* arg, grpc_error_handle error);

  grpc_closure on_done_;
  std::string attribute_;
  absl::AnyInvocable<void(std::string /* attribute */,
                          absl::StatusOr<std::string> /* result */)>
      callback_;
  OrphanablePtr<HttpRequest> http_request_;
  grpc_http_response response_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_GCP_METADATA_QUERY_H
