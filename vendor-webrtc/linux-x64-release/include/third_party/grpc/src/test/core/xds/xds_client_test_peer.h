//
// Copyright 2022 gRPC authors.
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

#ifndef GRPC_TEST_CORE_XDS_XDS_CLIENT_TEST_PEER_H
#define GRPC_TEST_CORE_XDS_XDS_CLIENT_TEST_PEER_H

#include <grpc/support/port_platform.h>

#include <set>

#include "absl/functional/function_ref.h"
#include "absl/strings/str_cat.h"
#include "src/core/xds/xds_client/xds_client.h"

namespace grpc_core {
namespace testing {

class XdsClientTestPeer {
 public:
  explicit XdsClientTestPeer(XdsClient* xds_client) : xds_client_(xds_client) {}

  std::string TestDumpClientConfig() {
    upb::Arena arena;
    auto* client_config = envoy_service_status_v3_ClientConfig_new(arena.ptr());
    std::set<std::string> string_pool;
    MutexLock lock(xds_client_->mu());
    xds_client_->DumpClientConfig(&string_pool, arena.ptr(), client_config);
    size_t output_length;
    char* output = envoy_service_status_v3_ClientConfig_serialize(
        client_config, arena.ptr(), &output_length);
    return std::string(output, output_length);
  }

  struct ResourceCountLabels {
    std::string xds_authority;
    std::string resource_type;
    std::string cache_state;

    std::string ToString() const {
      return absl::StrCat("xds_authority=\"", xds_authority,
                          "\" resource_type=\"", resource_type,
                          "\" cache_state=\"", cache_state, "\"");
    }
  };
  void TestReportResourceCounts(
      absl::FunctionRef<void(const ResourceCountLabels&, uint64_t)> func) {
    MutexLock lock(xds_client_->mu());
    xds_client_->ReportResourceCounts(
        [&](const XdsClient::ResourceCountLabels& labels, uint64_t count) {
          ResourceCountLabels labels_copy = {std::string(labels.xds_authority),
                                             std::string(labels.resource_type),
                                             std::string(labels.cache_state)};
          func(labels_copy, count);
        });
  }

  void TestReportServerConnections(
      absl::FunctionRef<void(absl::string_view, bool)> func) {
    MutexLock lock(xds_client_->mu());
    xds_client_->ReportServerConnections(func);
  }

 private:
  XdsClient* xds_client_;
};

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_XDS_XDS_CLIENT_TEST_PEER_H
