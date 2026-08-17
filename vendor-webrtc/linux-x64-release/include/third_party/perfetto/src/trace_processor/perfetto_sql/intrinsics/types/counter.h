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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_COUNTER_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_COUNTER_H_

#include <cstdint>
#include <limits>
#include <string>
#include <vector>
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/trace_processor/basic_types.h"

namespace perfetto::trace_processor::perfetto_sql {

struct CounterTrackPartition {
  std::vector<int64_t> id;
  std::vector<int64_t> ts;
  std::vector<double> val;
};

struct PartitionedCounter {
  static constexpr char kName[] = "COUNTER_TRACK_PARTITIONS";
  base::
      FlatHashMap<int64_t, CounterTrackPartition, base::AlreadyHashed<int64_t>>
          partitions_map;
};

}  // namespace perfetto::trace_processor::perfetto_sql

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_COUNTER_H_
