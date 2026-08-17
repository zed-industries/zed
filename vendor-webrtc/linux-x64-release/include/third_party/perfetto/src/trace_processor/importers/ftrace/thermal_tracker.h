/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_THERMAL_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_THERMAL_TRACKER_H_

#include <cstddef>
#include <cstdint>

#include "src/trace_processor/util/descriptors.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class ThermalTracker {
 public:
  explicit ThermalTracker(TraceProcessorContext*);

  void ParseThermalTemperature(int64_t timestamp, protozero::ConstBytes blob);
  void ParseCdevUpdate(int64_t timestamp, protozero::ConstBytes);
  void ParseThermalExynosAcpmBulk(protozero::ConstBytes blob);
  void ParseThermalExynosAcpmHighOverhead(int64_t timestamp,
                                          protozero::ConstBytes blob);

 private:
  TraceProcessorContext* context_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_THERMAL_TRACKER_H_
