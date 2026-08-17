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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_CLUSTER_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_CLUSTER_H

#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "src/core/load_balancing/outlier_detection/outlier_detection.h"
#include "src/core/util/json/json.h"
#include "src/core/xds/grpc/xds_common_types.h"
#include "src/core/xds/grpc/xds_health_status.h"
#include "src/core/xds/grpc/xds_metadata.h"
#include "src/core/xds/grpc/xds_server_grpc.h"
#include "src/core/xds/xds_client/xds_backend_metric_propagation.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "src/core/xds/xds_client/xds_resource_type_impl.h"

namespace grpc_core {

inline bool LrsServersEqual(
    const std::shared_ptr<const GrpcXdsServerTarget>& lrs_server1,
    const std::shared_ptr<const GrpcXdsServerTarget>& lrs_server2) {
  if (lrs_server1 == nullptr) return lrs_server2 == nullptr;
  if (lrs_server2 == nullptr) return false;
  // Neither one is null, so compare them.
  return *lrs_server1 == *lrs_server2;
}

inline bool LrsBackendMetricPropagationEqual(
    const RefCountedPtr<const BackendMetricPropagation>& p1,
    const RefCountedPtr<const BackendMetricPropagation>& p2) {
  if (p1 == nullptr) return p2 == nullptr;
  if (p2 == nullptr) return false;
  // Neither one is null, so compare them.
  return *p1 == *p2;
}

struct XdsClusterResource : public XdsResourceType::ResourceData {
  struct Eds {
    // If empty, defaults to the cluster name.
    std::string eds_service_name;

    bool operator==(const Eds& other) const {
      return eds_service_name == other.eds_service_name;
    }
  };

  struct LogicalDns {
    // The hostname to lookup in DNS.
    std::string hostname;

    bool operator==(const LogicalDns& other) const {
      return hostname == other.hostname;
    }
  };

  struct Aggregate {
    // Prioritized list of cluster names.
    std::vector<std::string> prioritized_cluster_names;

    bool operator==(const Aggregate& other) const {
      return prioritized_cluster_names == other.prioritized_cluster_names;
    }
  };

  std::variant<Eds, LogicalDns, Aggregate> type;

  // The LB policy to use for locality and endpoint picking.
  Json::Array lb_policy_config;

  // Note: Remaining fields are not used for aggregate clusters.

  // The LRS server to use for load reporting.
  // If null, load reporting will be disabled.
  std::shared_ptr<const GrpcXdsServerTarget> lrs_load_reporting_server;
  // The set of metrics to propagate from ORCA to LRS.
  RefCountedPtr<const BackendMetricPropagation> lrs_backend_metric_propagation;

  bool use_http_connect = false;

  // Tls Context used by clients
  CommonTlsContext common_tls_context;

  // Connection idle timeout.  Currently used only for SSA.
  Duration connection_idle_timeout = Duration::Hours(1);

  // Maximum number of outstanding requests can be made to the upstream
  // cluster.
  uint32_t max_concurrent_requests = 1024;

  std::optional<OutlierDetectionConfig> outlier_detection;

  XdsHealthStatusSet override_host_statuses;

  XdsMetadataMap metadata;

  bool operator==(const XdsClusterResource& other) const {
    return type == other.type && lb_policy_config == other.lb_policy_config &&
           LrsServersEqual(lrs_load_reporting_server,
                           other.lrs_load_reporting_server) &&
           LrsBackendMetricPropagationEqual(
               lrs_backend_metric_propagation,
               other.lrs_backend_metric_propagation) &&
           common_tls_context == other.common_tls_context &&
           connection_idle_timeout == other.connection_idle_timeout &&
           max_concurrent_requests == other.max_concurrent_requests &&
           outlier_detection == other.outlier_detection &&
           override_host_statuses == other.override_host_statuses &&
           metadata == other.metadata;
  }

  std::string ToString() const;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_CLUSTER_H
