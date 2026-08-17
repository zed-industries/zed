// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_SMC_SAMPLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_SMC_SAMPLER_H_

#include <memory>

#include "tools/mac/power/power_sampler/sampler.h"

namespace power_metrics {
class SMCReader;
}

namespace power_sampler {

// The SMC sampler samples power usage from various hardware components from the
// System Management Controller (SMC).
class SMCSampler : public Sampler {
 public:
  static constexpr char kSamplerName[] = "smc";

  ~SMCSampler() override;

  // Creates and initializes a new sampler, if possible.
  // Returns nullptr on failure.
  static std::unique_ptr<SMCSampler> Create();

  // Sampler implementation.
  std::string GetName() override;
  DatumNameUnits GetDatumNameUnits() override;
  Sample GetSample(base::TimeTicks sample_time) override;

 private:
  friend class SMCSamplerTest;

  SMCSampler(std::unique_ptr<power_metrics::SMCReader> smc_reader);

  std::unique_ptr<power_metrics::SMCReader> smc_reader_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_BATTERY_SAMPLER_H_
