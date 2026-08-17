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

#ifndef GRPC_SRC_CORE_RESOLVER_XDS_XDS_DEPENDENCY_MANAGER_H
#define GRPC_SRC_CORE_RESOLVER_XDS_XDS_DEPENDENCY_MANAGER_H

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"
#include "src/core/resolver/resolver.h"
#include "src/core/resolver/xds/xds_config.h"
#include "src/core/util/ref_counted.h"
#include "src/core/xds/grpc/xds_client_grpc.h"
#include "src/core/xds/grpc/xds_cluster.h"
#include "src/core/xds/grpc/xds_endpoint.h"
#include "src/core/xds/grpc/xds_listener.h"
#include "src/core/xds/grpc/xds_route_config.h"

namespace grpc_core {

// Watches all xDS resources and handles dependencies between them.
// Reports updates only when all necessary resources have been obtained.
class XdsDependencyManager final : public RefCounted<XdsDependencyManager>,
                                   public Orphanable {
 public:
  class Watcher {
   public:
    virtual ~Watcher() = default;

    virtual void OnUpdate(
        absl::StatusOr<RefCountedPtr<const XdsConfig>> config) = 0;
  };

  class ClusterSubscription final : public DualRefCounted<ClusterSubscription> {
   public:
    ClusterSubscription(absl::string_view cluster_name,
                        RefCountedPtr<XdsDependencyManager> dependency_mgr)
        : cluster_name_(cluster_name),
          dependency_mgr_(std::move(dependency_mgr)) {}

    absl::string_view cluster_name() const { return cluster_name_; }

   private:
    void Orphaned() override;

    std::string cluster_name_;
    RefCountedPtr<XdsDependencyManager> dependency_mgr_;
  };

  XdsDependencyManager(RefCountedPtr<GrpcXdsClient> xds_client,
                       std::shared_ptr<WorkSerializer> work_serializer,
                       std::unique_ptr<Watcher> watcher,
                       std::string data_plane_authority,
                       std::string listener_resource_name, ChannelArgs args,
                       grpc_pollset_set* interested_parties);

  void Orphan() override;

  // Gets an external cluster subscription.  This allows us to include
  // clusters in the config that are referenced by something other than
  // the route config (e.g., RLS).  The cluster will be included in the
  // config as long as the returned object is still referenced.
  RefCountedPtr<ClusterSubscription> GetClusterSubscription(
      absl::string_view cluster_name);

  void RequestReresolution();

  void ResetBackoff();

  static absl::string_view ChannelArgName() {
    return GRPC_ARG_NO_SUBCHANNEL_PREFIX "xds_dependency_manager";
  }
  static int ChannelArgsCompare(const XdsDependencyManager* a,
                                const XdsDependencyManager* b) {
    return QsortCompare(a, b);
  }

 private:
  class ListenerWatcher;
  class RouteConfigWatcher;
  class ClusterWatcher;
  class EndpointWatcher;

  class DnsResultHandler;

  struct ClusterWatcherState {
    // Pointer to watcher, to be used when cancelling.
    // Not owned, so do not dereference.
    ClusterWatcher* watcher = nullptr;
    // Most recent update obtained from this watcher.
    absl::StatusOr<std::shared_ptr<const XdsClusterResource>> update = nullptr;
    // Ambient error.
    std::string resolution_note;
  };

  struct EndpointConfig {
    // If there was an error, update will be null and resolution_note
    // will be non-empty.
    std::shared_ptr<const XdsEndpointResource> endpoints;
    std::string resolution_note;
  };

  struct EndpointWatcherState {
    // Pointer to watcher, to be used when cancelling.
    // Not owned, so do not dereference.
    EndpointWatcher* watcher = nullptr;
    // Most recent update obtained from this watcher.
    EndpointConfig update;
  };

  struct DnsState {
    OrphanablePtr<Resolver> resolver;
    // Most recent result from the resolver.
    EndpointConfig update;
  };

  // Event handlers.
  void OnListenerUpdate(
      absl::StatusOr<std::shared_ptr<const XdsListenerResource>> listener);
  void OnListenerAmbientError(absl::Status status);

  void OnRouteConfigUpdate(
      const std::string& name,
      absl::StatusOr<std::shared_ptr<const XdsRouteConfigResource>>
          route_config);
  void OnRouteConfigAmbientError(std::string name, absl::Status status);

  void OnClusterUpdate(
      const std::string& name,
      absl::StatusOr<std::shared_ptr<const XdsClusterResource>> cluster);
  void OnClusterAmbientError(const std::string& name, absl::Status status);

  void OnEndpointUpdate(
      const std::string& name,
      absl::StatusOr<std::shared_ptr<const XdsEndpointResource>> endpoint);
  void OnEndpointAmbientError(const std::string& name, absl::Status status);

  void OnDnsResult(const std::string& dns_name, Resolver::Result result);
  void PopulateDnsUpdate(const std::string& dns_name, Resolver::Result result,
                         DnsState* dns_state);

  std::string GenerateResolutionNoteForCluster(
      absl::string_view cluster_resolution_note,
      absl::string_view endpoint_resolution_note) const;

  // Starts CDS and EDS/DNS watches for the specified cluster if needed.
  // Adds an entry to cluster_config_map, which will contain the cluster
  // data if the data is available.
  // For each EDS cluster, adds the EDS resource to eds_resources_seen.
  // For each Logical DNS cluster, adds the DNS hostname to dns_names_seen.
  // For aggregate clusters, calls itself recursively.  If leaf_clusters is
  // non-null, populates it with a list of leaf clusters, or an error if
  // max depth is exceeded.
  // Returns true if all resources have been obtained.
  bool PopulateClusterConfigMap(
      absl::string_view name, int depth,
      absl::flat_hash_map<std::string,
                          absl::StatusOr<XdsConfig::ClusterConfig>>*
          cluster_config_map,
      std::set<absl::string_view>* eds_resources_seen,
      std::set<absl::string_view>* dns_names_seen,
      absl::StatusOr<std::vector<absl::string_view>>* leaf_clusters = nullptr);

  // Called when an external cluster subscription is unreffed.
  void OnClusterSubscriptionUnref(absl::string_view cluster_name,
                                  ClusterSubscription* subscription);

  // Checks whether all necessary resources have been obtained, and if
  // so reports an update to the watcher.
  void MaybeReportUpdate();

  void ReportError(absl::string_view resource_type,
                   absl::string_view resource_name, absl::string_view error);

  // Parameters passed into ctor.
  RefCountedPtr<GrpcXdsClient> xds_client_;
  std::shared_ptr<WorkSerializer> work_serializer_;
  std::unique_ptr<Watcher> watcher_;
  const std::string data_plane_authority_;
  const std::string listener_resource_name_;
  ChannelArgs args_;
  grpc_pollset_set* interested_parties_;

  // Listener state.
  ListenerWatcher* listener_watcher_ = nullptr;
  std::shared_ptr<const XdsListenerResource> current_listener_;
  std::string route_config_name_;
  std::string lds_resolution_note_;

  // RouteConfig state.
  RouteConfigWatcher* route_config_watcher_ = nullptr;
  std::shared_ptr<const XdsRouteConfigResource> current_route_config_;
  const XdsRouteConfigResource::VirtualHost* current_virtual_host_ = nullptr;
  absl::flat_hash_set<absl::string_view> clusters_from_route_config_;
  std::string rds_resolution_note_;

  // Cluster state.
  absl::flat_hash_map<std::string, ClusterWatcherState> cluster_watchers_;
  absl::flat_hash_map<absl::string_view, WeakRefCountedPtr<ClusterSubscription>>
      cluster_subscriptions_;

  // Endpoint state.
  absl::flat_hash_map<std::string, EndpointWatcherState> endpoint_watchers_;
  absl::flat_hash_map<std::string, DnsState> dns_resolvers_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_XDS_XDS_DEPENDENCY_MANAGER_H
