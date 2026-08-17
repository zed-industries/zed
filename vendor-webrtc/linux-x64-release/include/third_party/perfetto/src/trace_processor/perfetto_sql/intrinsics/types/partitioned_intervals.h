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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_PARTITIONED_INTERVALS_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_PARTITIONED_INTERVALS_H_

#include <cstdint>
#include <string>
#include <vector>
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/containers/interval_intersector.h"

namespace perfetto::trace_processor::perfetto_sql {

struct Partition {
  std::vector<Interval> intervals;
  std::vector<SqlValue> sql_values;
  bool is_nonoverlapping = true;

  uint64_t last_interval = 0;
};

using Partitions =
    base::FlatHashMap<uint64_t, Partition, base::AlreadyHashed<uint64_t>>;

struct PartitionedTable {
  static constexpr char kName[] = "INTERVAL_TREE_PARTITIONS";
  Partitions partitions_map;

  std::vector<std::string> partition_column_names;
};

}  // namespace perfetto::trace_processor::perfetto_sql

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_PARTITIONED_INTERVALS_H_
