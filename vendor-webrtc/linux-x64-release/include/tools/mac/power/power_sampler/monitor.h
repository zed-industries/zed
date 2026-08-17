// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_MONITOR_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_MONITOR_H_

#include <memory>
#include <vector>

#include "base/containers/flat_map.h"
#include "base/time/time.h"

namespace power_sampler {

// A DataColumnKey uniquely identifies data columns given to Monitor.
struct DataColumnKey {
  std::string sampler_name;
  std::string column_name;

  bool operator==(const DataColumnKey& other) const {
    return std::make_pair(sampler_name, column_name) ==
           std::make_pair(other.sampler_name, other.column_name);
  }
  bool operator<(const DataColumnKey& other) const {
    return std::make_pair(sampler_name, column_name) <
           std::make_pair(other.sampler_name, other.column_name);
  }
};

using DataRow = base::flat_map<DataColumnKey, double>;

// Concrete monitor classes override this interface.
class Monitor {
 public:
  using DataColumnKeyUnits = base::flat_map<DataColumnKey, std::string>;

  Monitor() = default;
  virtual ~Monitor() = 0;

  // TODO(siggi): Add more callouts.
  //     - Add a callout for pre-session notification with all samplers.
  //     - Add a callot for post-session.
  //     - This will allow monitors to do the output, whether it's done as you
  //       go for e.g. CSV, or all-in-one for e.g. JSON.

  // Called once before any OnSample calls are made.
  // Can be used to e.g. open a file, output a file header or other
  // one-time setup.
  virtual void OnStartSession(const DataColumnKeyUnits& data_columns_units) = 0;

  // Called each time a new set of |samples| has been acquired. The
  // |sample_time| is the time when the acquisition of |samples| started.
  // |data_row| is a potentially sparse collection of named datums provided in
  // OnStartSession(). Returns true if the sampling session should be ended.
  virtual bool OnSample(base::TimeTicks sample_time,
                        const DataRow& data_row) = 0;

  // Called once after all OnSample calls have been made.
  // Can be used to e.g. close files, flush output or other one-time teardown.
  virtual void OnEndSession() = 0;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_MONITOR_H_
