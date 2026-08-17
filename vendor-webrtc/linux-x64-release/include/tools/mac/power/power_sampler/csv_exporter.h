// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_CSV_EXPORTER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_CSV_EXPORTER_H_

#include <memory>

#include "base/containers/flat_set.h"
#include "base/files/file.h"
#include "base/time/time.h"
#include "tools/mac/power/power_sampler/monitor.h"

namespace power_sampler {

// Listens to Monitor notifications and writes data in CSV format as it arrives.
class CsvExporter : public Monitor {
 public:
  // Creates and initializes a new exporter, if possible.
  // Prefer using the |file_path| over the |file| option, except when using
  // special files such as e.g. STDOUT.
  // Returns nullptr on failure.
  static std::unique_ptr<CsvExporter> Create(base::TimeTicks time_base,
                                             base::FilePath file_path);
  static std::unique_ptr<CsvExporter> Create(base::TimeTicks time_base,
                                             base::File file);

  ~CsvExporter() override;

  // Monitor implementation:
  void OnStartSession(const DataColumnKeyUnits& data_columns_units) override;
  bool OnSample(base::TimeTicks sample_time, const DataRow& data_row) override;
  void OnEndSession() override;

 private:
  CsvExporter(base::TimeTicks time_base, base::File file);
  // Appends |text| to |file_|, returns true on success.
  bool Append(const std::string& text);

  const base::TimeTicks time_base_;
  base::File file_;

  // The column keys seen in |OnStartSession|.
  base::flat_set<DataColumnKey> column_keys_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_CSV_EXPORTER_H_
