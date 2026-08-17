// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLER_H_

#include <functional>
#include <map>
#include <string>

#include "base/containers/flat_map.h"
#include "base/time/time.h"

namespace power_sampler {

// Concrete sampler classes override this interface.
class Sampler {
 public:
  using DatumNameUnits = base::flat_map<std::string, std::string>;
  using Sample = base::flat_map<std::string, double>;

  Sampler() = default;
  virtual ~Sampler() = 0;

  // Returns the name of the sampler.
  virtual std::string GetName() = 0;

  // Returns the names and units of the datums provided by this sampler.
  virtual DatumNameUnits GetDatumNameUnits() = 0;

  // Subclasses override to return their sample, |sample_time| is the time
  // when the controller started the acquisition of this sample.
  // Returns the new sample.
  virtual Sample GetSample(base::TimeTicks sample_time) = 0;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLER_H_
