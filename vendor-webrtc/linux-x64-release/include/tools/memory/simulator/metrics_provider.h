// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_METRICS_PROVIDER_H_
#define TOOLS_MEMORY_SIMULATOR_METRICS_PROVIDER_H_

#include <map>
#include <string>
#include <vector>

#include "base/time/time.h"

namespace memory_simulator {

class MetricsProvider {
 public:
  MetricsProvider();
  virtual ~MetricsProvider();

  // Returns the list of metrics that can be returned by this provider. This
  // must always return the same metrics, in the same order.
  virtual std::vector<std::string> GetMetricNames() = 0;

  // Returns metric values. All keys must be part of `GetMetricNames()`.
  virtual std::map<std::string, double> GetMetricValues(
      base::TimeTicks now) = 0;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_METRICS_PROVIDER_H_
