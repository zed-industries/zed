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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TIME_CONV_RECORD_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TIME_CONV_RECORD_H_

#include <cstdint>

namespace perfetto::trace_processor::perf_importer {

struct TimeConvRecord {
  uint64_t time_shift;
  uint64_t time_mult;
  uint64_t time_zero;
  uint64_t time_cycles;
  uint64_t time_mask;
  uint8_t cap_user_time_zero;
  uint8_t cap_user_time_short;
  uint8_t reserved[6];  // alignment

  uint64_t ConvertTscToPerfTime(uint64_t cycles) const {
    uint64_t quot, rem;

    if (cap_user_time_short) {
      cycles = time_cycles + ((cycles - time_cycles) & time_mask);
    }

    quot = cycles >> time_shift;
    rem = cycles & ((uint64_t(1) << time_shift) - 1);
    return time_zero + quot * time_mult + ((rem * time_mult) >> time_shift);
  }
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TIME_CONV_RECORD_H_
