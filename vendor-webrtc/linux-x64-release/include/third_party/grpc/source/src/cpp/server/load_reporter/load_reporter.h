//
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
//

#ifndef GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTER_H
#define GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTER_H

#include <google/protobuf/repeated_ptr_field.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <atomic>
#include <chrono>
#include <cstdint>
#include <deque>
#include <memory>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "opencensus/stats/stats.h"
#include "opencensus/tags/tag_key.h"
#include "src/core/util/sync.h"
#include "src/cpp/server/load_reporter/load_data_store.h"
#include "src/proto/grpc/lb/v1/load_reporter.grpc.pb.h"

// IWYU pragma: no_include <ratio>

namespace grpc {
namespace load_reporter {

// The interface to get the Census stats. Abstracted for mocking.
class CensusViewProvider {
 public:
  // Maps from the view name to the view data.
  using ViewDataMap =
      std::unordered_map<std::string, ::opencensus::stats::ViewData>;
  // Maps from the view name to the view descriptor.
  using ViewDescriptorMap =
      std::unordered_map<std::string, ::opencensus::stats::ViewDescriptor>;

  CensusViewProvider();
  virtual ~CensusViewProvider() = default;

  // Fetches the view data accumulated since last fetching, and returns it as a
  // map from the view name to the view data.
  virtual ViewDataMap FetchViewData() = 0;

  // A helper function that gets a row with the input tag values from the view
  // data. Only used when we know that row must exist because we have seen a row
  // with the same tag values in a related view data. Several ViewData's are
  // considered related if their views are based on the measures that are always
  // recorded at the same time.
  static double GetRelatedViewDataRowDouble(
      const ViewDataMap& view_data_map, const char* view_name,
      size_t view_name_len, const std::vector<std::string>& tag_values);
  static uint64_t GetRelatedViewDataRowInt(
      const ViewDataMap& view_data_map, const char* view_name,
      size_t view_name_len, const std::vector<std::string>& tag_values);

 protected:
  const ViewDescriptorMap& view_descriptor_map() const {
    return view_descriptor_map_;
  }

 private:
  ViewDescriptorMap view_descriptor_map_;
  // Tag keys.
  ::opencensus::tags::TagKey tag_key_token_;
  ::opencensus::tags::TagKey tag_key_host_;
  ::opencensus::tags::TagKey tag_key_user_id_;
  ::opencensus::tags::TagKey tag_key_status_;
  ::opencensus::tags::TagKey tag_key_metric_name_;
};

// The default implementation fetches the real stats from Census.
class CensusViewProviderDefaultImpl : public CensusViewProvider {
 public:
  CensusViewProviderDefaultImpl();

  ViewDataMap FetchViewData() override;

 private:
  std::unordered_map<std::string, ::opencensus::stats::View> view_map_;
};

// The interface to get the CPU stats. Abstracted for mocking.
class CpuStatsProvider {
 public:
  // The used and total amounts of CPU usage.
  using CpuStatsSample = std::pair<uint64_t, uint64_t>;

  virtual ~CpuStatsProvider() = default;

  // Gets the cumulative used CPU and total CPU resource.
  virtual CpuStatsSample GetCpuStats() = 0;
};

// The default implementation reads CPU jiffies from the system to calculate CPU
// utilization.
class CpuStatsProviderDefaultImpl : public CpuStatsProvider {
 public:
  CpuStatsSample GetCpuStats() override;
};

// Maintains all the load data and load reporting streams.
class LoadReporter {
 public:
  // TODO(juanlishen): Allow config for providers from users.
  LoadReporter(uint32_t feedback_sample_window_seconds,
               std::unique_ptr<CensusViewProvider> census_view_provider,
               std::unique_ptr<CpuStatsProvider> cpu_stats_provider)
      : feedback_sample_window_seconds_(feedback_sample_window_seconds),
        census_view_provider_(std::move(census_view_provider)),
        cpu_stats_provider_(std::move(cpu_stats_provider)) {
    // Append the initial record so that the next real record can have a base.
    AppendNewFeedbackRecord(0, 0);
  }

  // Fetches the latest data from Census and merge it into the data store.
  // Also adds a new sample to the LB feedback sliding window.
  // Thread-unsafe. (1). The access to the load data store and feedback records
  // has locking. (2). The access to the Census view provider and CPU stats
  // provider lacks locking, but we only access these two members in this method
  // (in testing, we also access them when setting up expectation). So the
  // invocations of this method must be serialized.
  void FetchAndSample();

  // Generates a report for that host and balancer. The report contains
  // all the stats data accumulated between the last report (i.e., the last
  // consumption) and the last fetch from Census (i.e., the last production).
  // Thread-safe.
  ::google::protobuf::RepeatedPtrField<grpc::lb::v1::Load> GenerateLoads(
      const std::string& hostname, const std::string& lb_id);

  // The feedback is calculated from the stats data recorded in the sliding
  // window. Outdated records are discarded.
  // Thread-safe.
  grpc::lb::v1::LoadBalancingFeedback GenerateLoadBalancingFeedback();

  // Wrapper around LoadDataStore::ReportStreamCreated.
  // Thread-safe.
  void ReportStreamCreated(const std::string& hostname,
                           const std::string& lb_id,
                           const std::string& load_key);

  // Wrapper around LoadDataStore::ReportStreamClosed.
  // Thread-safe.
  void ReportStreamClosed(const std::string& hostname,
                          const std::string& lb_id);

  // Generates a unique LB ID of length kLbIdLength. Returns an empty string
  // upon failure. Thread-safe.
  std::string GenerateLbId();

  // Accessors only for testing.
  CensusViewProvider* census_view_provider() {
    return census_view_provider_.get();
  }
  CpuStatsProvider* cpu_stats_provider() { return cpu_stats_provider_.get(); }

 private:
  struct LoadBalancingFeedbackRecord {
    std::chrono::system_clock::time_point end_time;
    uint64_t rpcs;
    uint64_t errors;
    uint64_t cpu_usage;
    uint64_t cpu_limit;

    LoadBalancingFeedbackRecord(
        const std::chrono::system_clock::time_point& end_time, uint64_t rpcs,
        uint64_t errors, uint64_t cpu_usage, uint64_t cpu_limit)
        : end_time(end_time),
          rpcs(rpcs),
          errors(errors),
          cpu_usage(cpu_usage),
          cpu_limit(cpu_limit) {}
  };

  // Finds the view data about starting call from the view_data_map and merges
  // the data to the load data store.
  void ProcessViewDataCallStart(
      const CensusViewProvider::ViewDataMap& view_data_map);
  // Finds the view data about ending call from the view_data_map and merges the
  // data to the load data store.
  void ProcessViewDataCallEnd(
      const CensusViewProvider::ViewDataMap& view_data_map);
  // Finds the view data about the customized call metrics from the
  // view_data_map and merges the data to the load data store.
  void ProcessViewDataOtherCallMetrics(
      const CensusViewProvider::ViewDataMap& view_data_map);

  bool IsRecordInWindow(const LoadBalancingFeedbackRecord& record,
                        std::chrono::system_clock::time_point now) {
    return record.end_time > now - feedback_sample_window_seconds_;
  }

  void AppendNewFeedbackRecord(uint64_t rpcs, uint64_t errors);

  // Extracts an OrphanedLoadIdentifier from the per-balancer store and attaches
  // it to the load.
  void AttachOrphanLoadId(grpc::lb::v1::Load* load,
                          const PerBalancerStore& per_balancer_store);

  std::atomic<int64_t> next_lb_id_{0};
  const std::chrono::seconds feedback_sample_window_seconds_;
  grpc_core::Mutex feedback_mu_;
  std::deque<LoadBalancingFeedbackRecord> feedback_records_;
  // TODO(juanlishen): Lock in finer grain. Locking the whole store may be
  // too expensive.
  grpc_core::Mutex store_mu_;
  LoadDataStore load_data_store_;
  std::unique_ptr<CensusViewProvider> census_view_provider_;
  std::unique_ptr<CpuStatsProvider> cpu_stats_provider_;
};

}  // namespace load_reporter
}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTER_H
