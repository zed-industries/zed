// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_JSON_EXPORTER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_JSON_EXPORTER_H_

#include <memory>
#include <vector>

#include "base/files/file_path.h"
#include "base/time/time.h"
#include "base/values.h"
#include "tools/mac/power/power_sampler/monitor.h"

namespace power_sampler {

// Listens to Monitor notifications and write data in JSON format in a file upon
// OnEndSession().
class JsonExporter : public Monitor {
 public:
  // Creates and initializes a new exporter, if possible.
  // Returns nullptr on failure.
  static std::unique_ptr<JsonExporter> Create(base::FilePath file_path,
                                              base::TimeTicks time_base);

  ~JsonExporter() override;

  // Monitor implementation:
  void OnStartSession(const DataColumnKeyUnits& data_columns_units) override;
  bool OnSample(base::TimeTicks sample_time, const DataRow& data_row) override;
  void OnEndSession() override;

  const base::Value& GetColumnLabelsForTesting() const {
    return column_labels_;
  }
  base::Value GetDataRowsForTesting() const {
    return base::Value(data_rows_.Clone());
  }

 private:
  JsonExporter(base::FilePath file_path, base::TimeTicks time_base);

  base::FilePath file_path_;
  base::TimeTicks time_base_;
  base::Value column_labels_;
  base::Value::List data_rows_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_JSON_EXPORTER_H_
