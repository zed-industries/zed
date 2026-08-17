// Copyright (C) 2023 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_LAYOUT_FUNCTIONS_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_LAYOUT_FUNCTIONS_H_

#include <sqlite3.h>

#include "perfetto/base/status.h"
#include "src/trace_processor/perfetto_sql/engine/perfetto_sql_engine.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

// Implements INTERNAL_LAYOUT(ts, dur) window aggregate function.
// This function takes a set of slices (ordered by ts) and computes depths
// allowing them to be displayed on a single track in a non-overlapping manner,
// while trying to minimise the total height.
//
// TODO(altimin): this should support grouping sets of sets of slices (aka
// "tracks") by passing 'track_id' parameter. The complication is that we will
// need to know the max depth for each "track", so it's punted for now.
base::Status RegisterLayoutFunctions(PerfettoSqlEngine& engine);

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_LAYOUT_FUNCTIONS_H_
