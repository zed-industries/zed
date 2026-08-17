// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_METRICS_PRINTER_H_
#define TOOLS_MEMORY_SIMULATOR_METRICS_PRINTER_H_

#include <memory>
#include <vector>

#include "base/time/time.h"
#include "tools/memory/simulator/metrics_provider.h"

namespace memory_simulator {

// Outputs memory stats in CSV format to stdout.
class MetricsPrinter {
 public:
  MetricsPrinter();
  ~MetricsPrinter();

  // Adds a metrics provider. This can only be called before the first call to
  // PrintHeader().
  void AddProvider(std::unique_ptr<MetricsProvider> provider);

  // Prints header row.
  void PrintHeader();

  // Prints stats row. This can only be called after PrintHeader() has been
  // called.
  void PrintStats();

 private:
  const base::TimeTicks start_time_ = base::TimeTicks::Now();

  bool did_print_header_ = false;

  std::vector<std::unique_ptr<MetricsProvider>> providers_;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_METRICS_PRINTER_H_
