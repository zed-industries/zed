// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLING_CONTROLLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLING_CONTROLLER_H_

#include <functional>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "base/containers/flat_map.h"

namespace power_sampler {

class Sampler;
class Monitor;
struct DataColumnKey;

// The sampling controller takes care of colleting datums from all samplers
// on a sampling event.
class SamplingController {
 public:
  SamplingController();
  ~SamplingController();

  // Adds |sampler| to this controller if it has a unique name.
  // Returns true if |sampler| was added to the controller.
  bool AddSampler(std::unique_ptr<Sampler> sampler);

  // Adds |monitor| to this controller.
  // Each monitor is called after a sample is acquired to evaluate whether the
  // sampling session should end.
  void AddMonitor(std::unique_ptr<Monitor> monitor);

  // Call once after all samplers and monitors have been added.
  // Will notify monitors that a session is starting.
  void StartSession();

  // Returns true iff this controller has all the samples it wants.
  bool OnSamplingEvent();

  // Returns true if any samplers have been added.
  bool HasSamplers();

  // Call once after the last call to OnSamplingEvent.
  // Will notify monitors that a session has ended.
  void EndSession();

  // TODO(siggi): We want to at least have a sampling event provider that
  //     ticks on IOPMPowerSource change notification, plus a simple timed
  //     source.
  // TODO(siggi): We want to output samples in CSV at least, maybe other
  //     formats? Outputting samples as they're collected seems the best
  //     strategy?

 private:
  using Samplers = std::vector<std::unique_ptr<Sampler>>;
  using Monitors = std::vector<std::unique_ptr<Monitor>>;

  Samplers samplers_;
  Monitors monitors_;

  base::flat_map<DataColumnKey, std::string> data_columns_units_;

  bool started_ = false;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLING_CONTROLLER_H_
