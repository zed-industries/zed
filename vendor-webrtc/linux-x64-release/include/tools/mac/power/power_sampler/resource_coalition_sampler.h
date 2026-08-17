// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_RESOURCE_COALITION_SAMPLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_RESOURCE_COALITION_SAMPLER_H_

#include <mach/mach_time.h>
#include <stdint.h>

#include <memory>
#include <optional>

#include "base/process/process_handle.h"
#include "base/time/time.h"
#include "components/power_metrics/energy_impact_mac.h"
#include "components/power_metrics/resource_coalition_mac.h"
#include "tools/mac/power/power_sampler/sampler.h"

namespace power_sampler {

class ResourceCoalitionSamplerTest;

// The resource coalition sampler provides resource usage data for a group of
// tasks that are part of a "resource coalition", including those that have
// died. Typically, a "resource coalition" includes a root process and its
// descendants. "Resource coalition" is an undocumented mechanism available in
// macOS. Some information is available in the source
// (https://github.com/apple/darwin-xnu/blob/main/osfmk/kern/coalition.c).
class ResourceCoalitionSampler : public Sampler {
 public:
  static constexpr char kSamplerName[] = "resource_coalition";

  ~ResourceCoalitionSampler() override;

  // Creates and initializes a new sampler. |pid| is the pid of any process in
  // the "resource coalition" to sample. |start_time| is the time at which this
  // is invoked. Returns nullptr on failure.
  static std::unique_ptr<ResourceCoalitionSampler> Create(
      base::ProcessId pid,
      base::TimeTicks start_time);

  // Sampler implementation.
  std::string GetName() override;
  DatumNameUnits GetDatumNameUnits() override;
  Sample GetSample(base::TimeTicks sample_time) override;

 private:
  friend class power_sampler::ResourceCoalitionSamplerTest;

  // Exposed for testing only.
  using GetProcessCoalitionIdFn =
      std::optional<uint64_t> (*)(base::ProcessId pid);
  using GetCoalitionResourceUsageFn =
      std::unique_ptr<coalition_resource_usage> (*)(int64_t coalition_id);

  static std::unique_ptr<ResourceCoalitionSampler> Create(
      base::ProcessId pid,
      base::TimeTicks start_time,
      GetProcessCoalitionIdFn get_process_coalition_id_fn,
      GetCoalitionResourceUsageFn get_coalition_resource_usage_fn,
      mach_timebase_info_data_t timebase);

  ResourceCoalitionSampler(
      uint64_t coalition_id,
      base::TimeTicks now,
      GetCoalitionResourceUsageFn get_coalition_resource_usage_fn,
      mach_timebase_info_data_t timebase);

  const uint64_t coalition_id_;
  const GetCoalitionResourceUsageFn get_coalition_resource_usage_fn_;
  const mach_timebase_info_data_t timebase_;
  std::optional<power_metrics::EnergyImpactCoefficients>
      energy_impact_coefficients_;

  base::TimeTicks previous_time_;
  std::unique_ptr<coalition_resource_usage> previous_stats_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_RESOURCE_COALITION_SAMPLER_H_
