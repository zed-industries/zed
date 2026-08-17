/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_STACK_FUNCTIONS_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_STACK_FUNCTIONS_H_

#include <sqlite3.h>
#include <cstddef>
#include <cstdint>

#include "perfetto/base/status.h"

namespace perfetto {
namespace trace_processor {

class PerfettoSqlEngine;
class TraceProcessorContext;

// Registers the stack manipulation related functions:
//
// STACK_FROM_STACK_PROFILE_FRAME(frame_id LONG)
// Creates a stack with just the frame referenced by frame_id (reference to the
// stack_profile_frame table)
//
// STACK_FROM_STACK_PROFILE_CALLSITE(callsite_id LONG, [annotate BOOLEAN])
// Creates a stack by taking a callsite_id (reference to the
// stack_profile_callsite table) and generating a list of frames (by walking the
// stack_profile_callsite table)
// Optionally annotates frames (annotate param has a default value of false)
// *Important*: Annotations might interfere with certain aggregations, as we
// will could have a frame that is annotated with different annotations. That
// will lead to multiple functions being generated (same name, line etc, but
// different annotation).
//
// CAT_STACKS(root BLOB/STRING, level_1 BLOB/STRING, …, leaf BLOB/STRING)
// Creates a Stack by concatenating other Stacks. Also accepts strings for which
// it generates a fake Frame
//
// See protos/perfetto/trace_processor/stack.proto
base::Status RegisterStackFunctions(PerfettoSqlEngine* engine,
                                    TraceProcessorContext* context);

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_STACK_FUNCTIONS_H_
