//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_RESOLVER_XDS_XDS_CONFIG_H
#define GRPC_SRC_CORE_RESOLVER_XDS_XDS_CONFIG_H

#include <memory>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/util/ref_counted.h"
#include "src/core/xds/grpc/xds_cluster.h"
#include "src/core/xds/grpc/xds_endpoint.h"
#include "src/core/xds/grpc/xds_listener.h"
#include "src/core/xds/grpc/xds_route_config.h"

namespace grpc_core {

// A complete gRPC client-side xDS config containing all necessary
// resources.
struct XdsConfig : public RefCounted<XdsConfig> {
  // Listener resource.  Always non-null.
  std::shared_ptr<const XdsListenerResource> listener;
  // RouteConfig resource.  Will be populated even if RouteConfig is
  // inlined into the Listener resource.
  std::shared_ptr<const XdsRouteConfigResource> route_config;
  // Virtual host.  Points into route_config.  Will always be non-null.
  const XdsRouteConfigResource::VirtualHost* virtual_host;

  struct ClusterConfig {
    // Cluster resource.  Always non-null.
    std::shared_ptr<const XdsClusterResource> cluster;
    // Endpoint info for EDS and LOGICAL_DNS clusters.  If there was an
    // error, endpoints will be null and resolution_note will be set.
    struct EndpointConfig {
      std::shared_ptr<const XdsEndpointResource> endpoints;
      std::string resolution_note;

      EndpointConfig(std::shared_ptr<const XdsEndpointResource> endpoints,
                     std::string resolution_note)
          : endpoints(std::move(endpoints)),
            resolution_note(std::move(resolution_note)) {}
      bool operator==(const EndpointConfig& other) const {
        return endpoints == other.endpoints &&
               resolution_note == other.resolution_note;
      }
    };
    // The list of leaf clusters for an aggregate cluster.
    struct AggregateConfig {
      std::vector<absl::string_view> leaf_clusters;

      explicit AggregateConfig(std::vector<absl::string_view> leaf_clusters)
          : leaf_clusters(std::move(leaf_clusters)) {}
      bool operator==(const AggregateConfig& other) const {
        return leaf_clusters == other.leaf_clusters;
      }
    };
    std::variant<EndpointConfig, AggregateConfig> children;

    // Ctor for leaf clusters.
    ClusterConfig(std::shared_ptr<const XdsClusterResource> cluster,
                  std::shared_ptr<const XdsEndpointResource> endpoints,
                  std::string resolution_note);
    // Ctor for aggregate clusters.
    ClusterConfig(std::shared_ptr<const XdsClusterResource> cluster,
                  std::vector<absl::string_view> leaf_clusters);

    bool operator==(const ClusterConfig& other) const {
      return cluster == other.cluster && children == other.children;
    }
  };
  // Cluster map.  A cluster will have a non-OK status if either
  // (a) there was an error and we did not already have a valid
  // resource or (b) the resource does not exist.
  absl::flat_hash_map<std::string, absl::StatusOr<ClusterConfig>> clusters;

  std::string ToString() const;

  static absl::string_view ChannelArgName() {
    return GRPC_ARG_NO_SUBCHANNEL_PREFIX "xds_config";
  }
  static int ChannelArgsCompare(const XdsConfig* a, const XdsConfig* b) {
    return QsortCompare(a, b);
  }
  static constexpr bool ChannelArgUseConstPtr() { return true; }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_XDS_XDS_CONFIG_H
