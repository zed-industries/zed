// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_M1_SAMPLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_M1_SAMPLER_H_

#include <memory>

#include "tools/mac/power/power_sampler/sampler.h"

namespace power_metrics {
class M1SensorsReader;
}

namespace power_sampler {

// The M1 sensors sampler samples the temperature of M1 P-Cores and E-Cores.
class M1Sampler : public Sampler {
 public:
  static constexpr char kSamplerName[] = "m1";

  ~M1Sampler() override;

  // Creates and initializes a new sampler, if possible.
  // Returns nullptr on failure.
  static std::unique_ptr<M1Sampler> Create();

  // Sampler implementation.
  std::string GetName() override;
  DatumNameUnits GetDatumNameUnits() override;
  Sample GetSample(base::TimeTicks sample_time) override;

 private:
  friend class M1SamplerTest;

  M1Sampler(std::unique_ptr<power_metrics::M1SensorsReader> reader);

  std::unique_ptr<power_metrics::M1SensorsReader> reader_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_M1_SAMPLER_H_
