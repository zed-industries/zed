// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_USER_IDLE_LEVEL_SAMPLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_USER_IDLE_LEVEL_SAMPLER_H_

#include "tools/mac/power/power_sampler/sampler.h"

#include <memory>
#include <vector>

namespace power_sampler {

// Samples the machdep.user_idle_level sysctl value if it exists.
class UserIdleLevelSampler : public Sampler {
 public:
  static constexpr char kSamplerName[] = "user_idle_level";

  ~UserIdleLevelSampler() override;

  // Creates and initializes a new sampler, if possible.
  // Returns nullptr on failure.
  static std::unique_ptr<UserIdleLevelSampler> Create();

  // Sampler implementation.
  std::string GetName() override;
  DatumNameUnits GetDatumNameUnits() override;
  Sample GetSample(base::TimeTicks sample_time) override;

 private:
  explicit UserIdleLevelSampler(std::vector<int> mib_name);

  // The mib name of the machdep.user_idle_level sysctl value.
  const std::vector<int> mib_name_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_USER_IDLE_LEVEL_SAMPLER_H_
