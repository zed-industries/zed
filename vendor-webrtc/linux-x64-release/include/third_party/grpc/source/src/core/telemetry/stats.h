//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_TELEMETRY_STATS_H
#define GRPC_SRC_CORE_TELEMETRY_STATS_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "src/core/telemetry/histogram_view.h"
#include "src/core/telemetry/stats_data.h"
#include "src/core/util/no_destruct.h"

namespace grpc_core {

inline GlobalStatsCollector& global_stats() {
  return *NoDestructSingleton<GlobalStatsCollector>::Get();
}

namespace stats_detail {
std::string StatsAsJson(absl::Span<const uint64_t> counters,
                        absl::Span<const absl::string_view> counter_name,
                        absl::Span<const HistogramView> histograms,
                        absl::Span<const absl::string_view> histogram_name);
}

template <typename T>
std::string StatsAsJson(T* data) {
  std::vector<HistogramView> histograms;
  for (int i = 0; i < static_cast<int>(T::Histogram::COUNT); i++) {
    histograms.push_back(
        data->histogram(static_cast<typename T::Histogram>(i)));
  }
  return stats_detail::StatsAsJson(
      absl::Span<const uint64_t>(data->counters,
                                 static_cast<int>(T::Counter::COUNT)),
      T::counter_name, histograms, T::histogram_name);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_TELEMETRY_STATS_H
