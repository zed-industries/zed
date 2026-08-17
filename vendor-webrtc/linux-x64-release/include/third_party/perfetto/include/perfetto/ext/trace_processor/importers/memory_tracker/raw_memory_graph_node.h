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

#ifndef INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_RAW_MEMORY_GRAPH_NODE_H_
#define INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_RAW_MEMORY_GRAPH_NODE_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "perfetto/base/export.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/memory_allocator_node_id.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/memory_graph_edge.h"

namespace perfetto {
namespace trace_processor {

// Describes the level of detail of the memory graph.
enum class LevelOfDetail : uint32_t {
  kFirst,

  // For background tracing mode. The node time is quick, and typically just the
  // totals are expected. Suballocations need not be specified. Node name must
  // contain only pre-defined strings and string arguments cannot be added.
  kBackground = kFirst,

  // For the levels below, MemoryNodeProvider instances must guarantee that the
  // total size reported in the root node is consistent. Only the granularity of
  // the child MemoryAllocatorNode(s) differs with the levels.

  // Few entries, typically a fixed number, per node.
  kLight,

  // Unrestricted amount of entries per node.
  kDetailed,

  kLast = kDetailed
};

// Data model for user-land memory nodes.
class PERFETTO_EXPORT_COMPONENT RawMemoryGraphNode {
 public:
  enum Flags {
    kDefault = 0,

    // A node marked weak will be discarded if there is no ownership edge exists
    // from a non-weak node.
    kWeak = 1 << 0,
  };

  // In the UI table each MemoryAllocatorNode becomes
  // a row and each Entry generates a column (if it doesn't already
  // exist).
  struct PERFETTO_EXPORT_COMPONENT MemoryNodeEntry {
    enum EntryType {
      kUint64,
      kString,
    };

    MemoryNodeEntry(const std::string& name,
                    const std::string& units,
                    uint64_t value);
    MemoryNodeEntry(const std::string& name,
                    const std::string& units,
                    const std::string& value);

    bool operator==(const MemoryNodeEntry& rhs) const;

    std::string name;
    std::string units;

    EntryType entry_type;

    uint64_t value_uint64;
    std::string value_string;
  };

  RawMemoryGraphNode(const std::string& absolute_name,
                     LevelOfDetail level,
                     MemoryAllocatorNodeId id);

  RawMemoryGraphNode(
      const std::string& absolute_name,
      LevelOfDetail level,
      MemoryAllocatorNodeId id,
      std::vector<RawMemoryGraphNode::MemoryNodeEntry>&& entries);

  // Standard attribute |name|s for the AddScalar and AddString() methods.
  static constexpr char kNameSize[] = "size";  // To represent allocated space.
  static constexpr char kNameObjectCount[] =
      "object_count";  // To represent number of objects.

  // Standard attribute |unit|s for the AddScalar and AddString() methods.
  static constexpr char kUnitsBytes[] =
      "bytes";  // Unit name to represent bytes.
  static constexpr char kUnitsObjects[] =
      "objects";  // Unit name to represent #objects.

  // Constants used only internally and by tests.
  static constexpr char kTypeScalar[] =
      "scalar";  // Type name for scalar attributes.
  static constexpr char kTypeString[] =
      "string";  // Type name for string attributes.

  // |id| is an optional global node identifier, unique across all processes
  // within the scope of a global node.
  // Subsequent MemoryAllocatorNode(s) with the same |absolute_name| are
  // expected to have the same id.
  MemoryAllocatorNodeId id() const { return id_; }

  // Absolute name, unique within the scope of an entire ProcessMemoryNode.
  const std::string& absolute_name() const { return absolute_name_; }

  const std::vector<MemoryNodeEntry>& entries() const { return entries_; }

  LevelOfDetail level_of_detail() const { return level_of_detail_; }

  // Use enum Flags to set values.
  void set_flags(int flags) { flags_ |= flags; }
  void clear_flags(int flags) { flags_ &= ~flags; }
  int flags() const { return flags_; }

 private:
  std::string absolute_name_;
  LevelOfDetail level_of_detail_;
  std::vector<MemoryNodeEntry> entries_;
  MemoryAllocatorNodeId id_;

  // A node marked weak will be discarded by TraceViewer.
  int flags_;  // See enum Flags.
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_RAW_MEMORY_GRAPH_NODE_H_
