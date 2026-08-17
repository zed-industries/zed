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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_LRS_CLIENT_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_LRS_CLIENT_H

#include <grpc/event_engine/event_engine.h>

#include <atomic>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/load_balancing/backend_metric_data.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/per_cpu.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"
#include "src/core/util/uri.h"
#include "src/core/util/work_serializer.h"
#include "src/core/xds/xds_client/xds_api.h"
#include "src/core/xds/xds_client/xds_backend_metric_propagation.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"
#include "src/core/xds/xds_client/xds_locality.h"
#include "src/core/xds/xds_client/xds_metrics.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "src/core/xds/xds_client/xds_transport.h"
#include "upb/reflection/def.hpp"

namespace grpc_core {

bool XdsOrcaLrsPropagationChangesEnabled();

class LrsClient : public DualRefCounted<LrsClient> {
 public:
  // Drop stats for an xds cluster.
  class ClusterDropStats final : public RefCounted<ClusterDropStats> {
   public:
    // The total number of requests dropped for any reason is the sum of
    // uncategorized_drops, and dropped_requests map.
    using CategorizedDropsMap = std::map<std::string /* category */, uint64_t>;
    struct Snapshot {
      uint64_t uncategorized_drops = 0;
      // The number of requests dropped for the specific drop categories
      // outlined in the drop_overloads field in the EDS response.
      CategorizedDropsMap categorized_drops;

      Snapshot& operator+=(const Snapshot& other) {
        uncategorized_drops += other.uncategorized_drops;
        for (const auto& [category, drops] : other.categorized_drops) {
          categorized_drops[category] += drops;
        }
        return *this;
      }

      bool IsZero() const {
        if (uncategorized_drops != 0) return false;
        for (const auto& [_, drops] : categorized_drops) {
          if (drops != 0) return false;
        }
        return true;
      }
    };

    ClusterDropStats(RefCountedPtr<LrsClient> lrs_client,
                     absl::string_view lrs_server,
                     absl::string_view cluster_name,
                     absl::string_view eds_service_name);
    ~ClusterDropStats() override;

    // Returns a snapshot of this instance and resets all the counters.
    Snapshot GetSnapshotAndReset();

    void AddUncategorizedDrops();
    void AddCallDropped(const std::string& category);

   private:
    RefCountedPtr<LrsClient> lrs_client_;
    absl::string_view lrs_server_;
    absl::string_view cluster_name_;
    absl::string_view eds_service_name_;
    std::atomic<uint64_t> uncategorized_drops_{0};
    // Protects categorized_drops_. A mutex is necessary because the length of
    // dropped_requests can be accessed by both the picker (from data plane
    // mutex) and the load reporting thread (from the control plane combiner).
    Mutex mu_;
    CategorizedDropsMap categorized_drops_ ABSL_GUARDED_BY(mu_);
  };

  // Locality stats for an xds cluster.
  class ClusterLocalityStats final : public RefCounted<ClusterLocalityStats> {
   public:
    struct BackendMetric {
      uint64_t num_requests_finished_with_metric = 0;
      double total_metric_value = 0;

      BackendMetric() = default;

      BackendMetric(uint64_t num_requests_finished, double value)
          : num_requests_finished_with_metric(num_requests_finished),
            total_metric_value(value) {}

      BackendMetric(BackendMetric&& other) noexcept
          : num_requests_finished_with_metric(
                std::exchange(other.num_requests_finished_with_metric, 0)),
            total_metric_value(std::exchange(other.total_metric_value, 0)) {}

      BackendMetric& operator=(BackendMetric&& other) noexcept {
        num_requests_finished_with_metric =
            std::exchange(other.num_requests_finished_with_metric, 0);
        total_metric_value = std::exchange(other.total_metric_value, 0);
        return *this;
      }

      BackendMetric& operator+=(const BackendMetric& other) {
        num_requests_finished_with_metric +=
            other.num_requests_finished_with_metric;
        total_metric_value += other.total_metric_value;
        return *this;
      }

      bool IsZero() const {
        return num_requests_finished_with_metric == 0 &&
               total_metric_value == 0;
      }
    };

    struct Snapshot {
      uint64_t total_successful_requests = 0;
      uint64_t total_requests_in_progress = 0;
      uint64_t total_error_requests = 0;
      uint64_t total_issued_requests = 0;
      BackendMetric cpu_utilization;
      BackendMetric mem_utilization;
      BackendMetric application_utilization;
      std::map<std::string, BackendMetric> backend_metrics;

      Snapshot& operator+=(const Snapshot& other) {
        total_successful_requests += other.total_successful_requests;
        total_requests_in_progress += other.total_requests_in_progress;
        total_error_requests += other.total_error_requests;
        total_issued_requests += other.total_issued_requests;
        cpu_utilization += other.cpu_utilization;
        mem_utilization += other.mem_utilization;
        application_utilization += other.application_utilization;
        for (const auto& [name, value] : other.backend_metrics) {
          backend_metrics[name] += value;
        }
        return *this;
      }

      bool IsZero() const {
        if (total_successful_requests != 0 || total_requests_in_progress != 0 ||
            total_error_requests != 0 || total_issued_requests != 0 ||
            !cpu_utilization.IsZero() || !mem_utilization.IsZero() ||
            !application_utilization.IsZero()) {
          return false;
        }
        for (const auto& [_, value] : backend_metrics) {
          if (!value.IsZero()) return false;
        }
        return true;
      }
    };

    ClusterLocalityStats(RefCountedPtr<LrsClient> lrs_client,
                         absl::string_view lrs_server,
                         absl::string_view cluster_name,
                         absl::string_view eds_service_name,
                         RefCountedPtr<XdsLocalityName> name,
                         RefCountedPtr<const BackendMetricPropagation>
                             backend_metric_propagation);
    ~ClusterLocalityStats() override;

    // Returns a snapshot of this instance and resets all the counters.
    Snapshot GetSnapshotAndReset();

    void AddCallStarted();
    void AddCallFinished(const BackendMetricData* backend_metrics,
                         bool fail = false);

    XdsLocalityName* locality_name() const { return name_.get(); }

   private:
    struct Stats {
      std::atomic<uint64_t> total_successful_requests{0};
      std::atomic<uint64_t> total_requests_in_progress{0};
      std::atomic<uint64_t> total_error_requests{0};
      std::atomic<uint64_t> total_issued_requests{0};

      Mutex backend_metrics_mu;
      BackendMetric cpu_utilization ABSL_GUARDED_BY(backend_metrics_mu);
      BackendMetric mem_utilization ABSL_GUARDED_BY(backend_metrics_mu);
      BackendMetric application_utilization ABSL_GUARDED_BY(backend_metrics_mu);
      std::map<std::string, BackendMetric> backend_metrics
          ABSL_GUARDED_BY(backend_metrics_mu);
    };

    RefCountedPtr<LrsClient> lrs_client_;
    absl::string_view lrs_server_;
    absl::string_view cluster_name_;
    absl::string_view eds_service_name_;
    RefCountedPtr<XdsLocalityName> name_;
    RefCountedPtr<const BackendMetricPropagation> backend_metric_propagation_;
    PerCpu<Stats> stats_{PerCpuOptions().SetMaxShards(32).SetCpusPerShard(4)};
  };

  LrsClient(
      std::shared_ptr<XdsBootstrap> bootstrap, std::string user_agent_name,
      std::string user_agent_version,
      RefCountedPtr<XdsTransportFactory> transport_factory,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine);
  ~LrsClient() override;

  // Adds drop stats for cluster_name and eds_service_name.
  RefCountedPtr<ClusterDropStats> AddClusterDropStats(
      std::shared_ptr<const XdsBootstrap::XdsServerTarget> lrs_server,
      absl::string_view cluster_name, absl::string_view eds_service_name);

  // Adds locality stats for cluster_name and eds_service_name for the
  // specified locality with the specified backend metric propagation.
  RefCountedPtr<ClusterLocalityStats> AddClusterLocalityStats(
      std::shared_ptr<const XdsBootstrap::XdsServerTarget> lrs_server,
      absl::string_view cluster_name, absl::string_view eds_service_name,
      RefCountedPtr<XdsLocalityName> locality,
      RefCountedPtr<const BackendMetricPropagation> backend_metric_propagation);

  // Resets connection backoff state.
  void ResetBackoff();

  XdsTransportFactory* transport_factory() const {
    return transport_factory_.get();
  }

  grpc_event_engine::experimental::EventEngine* engine() {
    return engine_.get();
  }

 private:
  // Contains a channel to the LRS server and all the data related to the
  // channel.
  class LrsChannel final : public DualRefCounted<LrsChannel> {
   public:
    template <typename T>
    class RetryableCall;

    class LrsCall;

    LrsChannel(WeakRefCountedPtr<LrsClient> lrs_client,
               std::shared_ptr<const XdsBootstrap::XdsServerTarget> server);
    ~LrsChannel() override;

    LrsClient* lrs_client() const { return lrs_client_.get(); }

    void ResetBackoff();

    void MaybeStartLrsCall();

    absl::string_view server_uri() const { return server_->server_uri(); }

   private:
    void Orphaned() override;

    void StopLrsCallLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&LrsClient::mu_);

    // The owning LrsClient.
    WeakRefCountedPtr<LrsClient> lrs_client_;

    std::shared_ptr<const XdsBootstrap::XdsServerTarget> server_;

    RefCountedPtr<XdsTransportFactory::XdsTransport> transport_;

    // The retryable LRS call.
    OrphanablePtr<RetryableCall<LrsCall>> lrs_call_;
  };

  struct LoadReportState {
    struct LocalityState {
      std::map<RefCountedPtr<const BackendMetricPropagation>,
               ClusterLocalityStats*, BackendMetricPropagation::Less>
          propagation_stats;
      ClusterLocalityStats::Snapshot deleted_locality_stats;
    };

    ClusterDropStats* drop_stats = nullptr;
    ClusterDropStats::Snapshot deleted_drop_stats;
    std::map<RefCountedPtr<XdsLocalityName>, LocalityState,
             XdsLocalityName::Less>
        locality_stats;
    Timestamp last_report_time = Timestamp::Now();
  };

  // Load report data.
  using LoadReportMap = std::map<
      std::pair<std::string /*cluster_name*/, std::string /*eds_service_name*/>,
      LoadReportState>;

  struct LoadReportServer {
    RefCountedPtr<LrsChannel> lrs_channel;
    LoadReportMap load_report_map;
  };

  struct ClusterLoadReport {
    ClusterDropStats::Snapshot dropped_requests;
    std::map<RefCountedPtr<XdsLocalityName>, ClusterLocalityStats::Snapshot,
             XdsLocalityName::Less>
        locality_stats;
    Duration load_report_interval;
  };
  using ClusterLoadReportMap = std::map<
      std::pair<std::string /*cluster_name*/, std::string /*eds_service_name*/>,
      ClusterLoadReport>;

  void Orphaned() override;

  ClusterLoadReportMap BuildLoadReportSnapshotLocked(
      const XdsBootstrap::XdsServerTarget& lrs_server, bool send_all_clusters,
      const std::set<std::string>& clusters) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  RefCountedPtr<LrsChannel> GetOrCreateLrsChannelLocked(
      std::shared_ptr<const XdsBootstrap::XdsServerTarget> server,
      const char* reason) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  static bool LoadReportCountersAreZero(const ClusterLoadReportMap& snapshot);

  void RemoveClusterDropStats(absl::string_view lrs_server,
                              absl::string_view cluster_name,
                              absl::string_view eds_service_name,
                              ClusterDropStats* cluster_drop_stats);

  void RemoveClusterLocalityStats(
      absl::string_view lrs_server, absl::string_view cluster_name,
      absl::string_view eds_service_name,
      const RefCountedPtr<XdsLocalityName>& locality,
      const RefCountedPtr<const BackendMetricPropagation>&
          backend_metric_propagation,
      ClusterLocalityStats* cluster_locality_stats);

  // Creates an initial LRS request.
  std::string CreateLrsInitialRequest() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  // Creates an LRS request sending a client-side load report.
  std::string CreateLrsRequest(ClusterLoadReportMap cluster_load_report_map)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  // Parses the LRS response and populates send_all_clusters,
  // cluster_names, and load_reporting_interval.
  absl::Status ParseLrsResponse(absl::string_view encoded_response,
                                bool* send_all_clusters,
                                std::set<std::string>* cluster_names,
                                Duration* load_reporting_interval)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  std::shared_ptr<XdsBootstrap> bootstrap_;
  const std::string user_agent_name_;
  const std::string user_agent_version_;
  RefCountedPtr<XdsTransportFactory> transport_factory_;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine_;

  Mutex mu_;
  upb::DefPool def_pool_ ABSL_GUARDED_BY(mu_);
  // Map of existing LRS channels.
  std::map<std::string /*XdsServer key*/, LrsChannel*> lrs_channel_map_
      ABSL_GUARDED_BY(mu_);
  std::map<std::string /*XdsServer key*/, LoadReportServer, std::less<>>
      load_report_map_ ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_LRS_CLIENT_H
