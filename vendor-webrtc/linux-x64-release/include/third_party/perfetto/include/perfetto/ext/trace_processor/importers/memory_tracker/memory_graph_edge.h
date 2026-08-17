/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_MEMORY_GRAPH_EDGE_H_
#define INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_MEMORY_GRAPH_EDGE_H_

#include <stdint.h>

#include "perfetto/base/export.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/memory_allocator_node_id.h"

namespace perfetto {
namespace trace_processor {

class PERFETTO_EXPORT_COMPONENT MemoryGraphEdge {
 public:
  MemoryGraphEdge(MemoryAllocatorNodeId s,
                  MemoryAllocatorNodeId t,
                  int i,
                  bool o)
      : source(s), target(t), importance(i), overridable(o) {}

  MemoryGraphEdge& operator=(const MemoryGraphEdge& edge) {
    source = edge.source;
    target = edge.target;
    importance = edge.importance;
    overridable = edge.overridable;
    return *this;
  }

  MemoryAllocatorNodeId source;
  MemoryAllocatorNodeId target;
  int importance;
  bool overridable;

  // Deliberately copy-able.
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_MEMORY_GRAPH_EDGE_H_
