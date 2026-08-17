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

#ifndef INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_RAW_PROCESS_MEMORY_NODE_H_
#define INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_RAW_PROCESS_MEMORY_NODE_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "perfetto/base/export.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/memory_allocator_node_id.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/memory_graph_edge.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/raw_memory_graph_node.h"

namespace perfetto {
namespace trace_processor {

// ProcessMemoryNode is as a strongly typed container which holds the nodes
// produced by the MemoryNodeProvider(s) for a specific process.
class PERFETTO_EXPORT_COMPONENT RawProcessMemoryNode {
 public:
  // Maps allocator nodes absolute names (allocator_name/heap/subheap) to
  // MemoryAllocatorNode instances.
  using MemoryNodesMap =
      std::map<std::string, std::unique_ptr<RawMemoryGraphNode>>;

  // Stores allocator node edges indexed by source allocator node GUID.
  using AllocatorNodeEdgesMap =
      std::map<MemoryAllocatorNodeId, std::unique_ptr<MemoryGraphEdge>>;

  explicit RawProcessMemoryNode(
      LevelOfDetail level_of_detail,
      AllocatorNodeEdgesMap&& edges_map = AllocatorNodeEdgesMap{},
      MemoryNodesMap&& nodes_map = MemoryNodesMap{});
  RawProcessMemoryNode(RawProcessMemoryNode&&);
  ~RawProcessMemoryNode();
  RawProcessMemoryNode& operator=(RawProcessMemoryNode&&);

  // Looks up a MemoryAllocatorNode given its allocator and heap names, or
  // nullptr if not found.
  RawMemoryGraphNode* GetAllocatorNode(const std::string& absolute_name) const;

  // Returns the map of the MemoryAllocatorNodes added to this node.
  const MemoryNodesMap& allocator_nodes() const { return allocator_nodes_; }

  const AllocatorNodeEdgesMap& allocator_nodes_edges() const {
    return allocator_nodes_edges_;
  }

  const LevelOfDetail& level_of_detail() const { return level_of_detail_; }

 private:
  LevelOfDetail level_of_detail_;

  // Keeps track of relationships between MemoryAllocatorNode(s).
  AllocatorNodeEdgesMap allocator_nodes_edges_;

  // Level of detail of the current node.
  MemoryNodesMap allocator_nodes_;

  // This class is uncopyable and unassignable.
  RawProcessMemoryNode(const RawProcessMemoryNode&) = delete;
  RawProcessMemoryNode& operator=(const RawProcessMemoryNode&) = delete;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_RAW_PROCESS_MEMORY_NODE_H_
