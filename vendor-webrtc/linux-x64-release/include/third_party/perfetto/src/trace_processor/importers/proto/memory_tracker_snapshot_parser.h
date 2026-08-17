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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MEMORY_TRACKER_SNAPSHOT_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MEMORY_TRACKER_SNAPSHOT_PARSER_H_

#include "perfetto/ext/trace_processor/importers/memory_tracker/graph_processor.h"
#include "protos/perfetto/trace/memory_graph.pbzero.h"
#include "src/trace_processor/importers/common/process_tracker.h"
#include "src/trace_processor/importers/common/track_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;

class MemoryTrackerSnapshotParser {
 public:
  explicit MemoryTrackerSnapshotParser(TraceProcessorContext* context);
  void ParseMemoryTrackerSnapshot(int64_t ts, protozero::ConstBytes blob);

  void NotifyEndOfFile();

 private:
  using RawMemoryNodeMap =
      std::map<base::PlatformProcessId, std::unique_ptr<RawProcessMemoryNode>>;
  using IdNodeMap =
      std::map<MemoryAllocatorNodeId, tables::MemorySnapshotNodeTable::Id>;
  using ConstBytes = protozero::ConstBytes;

  class ChildNode {
   public:
    ChildNode() : table_index_(-1) {}
    GlobalNodeGraph::Node* node_;
    std::string path_;
    uint64_t size_;
    uint64_t effective_size_;
    int32_t table_index_;
  };

  // Reads the proto-encoded memory snapshot of a process (message
  // MemoryTrackerSnapshot) in given |blob| in order to get:
  // - map of RawProcessMemoryNode containers |raw_nodes|. It
  //   is need to generates GlobalNodeGraph via GraphProcessor.
  // - level of detail of the memory graph |level_of_detail|.
  void ReadProtoSnapshot(ConstBytes blob,
                         RawMemoryNodeMap& raw_nodes,
                         LevelOfDetail& level_of_detail);

  // Generates GlobalNodeGraph via GraphProcessor for given map |raw_nodes|.
  std::unique_ptr<GlobalNodeGraph> GenerateGraph(RawMemoryNodeMap& raw_nodes);

  // Fills out MemorySnapshotTable, ProcessMemorySnapshotTable,
  // MemorySnapshotNodeTable, MemorySnapshotEdgeTable with given
  // timestamp |ts|, |graph|, |level_of_detail|.
  void EmitRows(int64_t ts,
                GlobalNodeGraph& graph,
                LevelOfDetail level_of_detail);

  // Fills out MemorySnapshotNodeTable for given root node
  // |root_node_graph| and ProcessMemorySnapshotId |proc_snapshot_row_id|.
  // Generates map of MemoryAllocatorNodeId and MemorySnapshotNodeTable::Id
  // |id_node_map| which is used at time of filling out of
  // MemorySnapshotEdgeTable.
  void EmitMemorySnapshotNodeRows(GlobalNodeGraph::Node& root_node_graph,
                                  ProcessMemorySnapshotId& proc_snapshot_row_id,
                                  IdNodeMap& id_node_map);

  // Recursively traverses through list of children of |node| to generate full
  // |path| to every node in MemorySnapshotNodeTable for given
  // ProcessMemorySnapshotId |proc_snapshot_row_id|.
  // Generates map of MemoryAllocatorNodeId and MemorySnapshotNodeTable::Id
  // |id_node_map| which is used at time of filling out of
  // MemorySnapshotEdgeTable.
  void EmitMemorySnapshotNodeRowsRecursively(
      GlobalNodeGraph::Node& node,
      const std::string&,
      std::optional<tables::MemorySnapshotNodeTable::Id> parent_node_row_id,
      ProcessMemorySnapshotId& proc_snapshot_row_id,
      IdNodeMap& id_node_map);

  // Fills out MemorySnapshotNodeTable for given Node
  // |node|, |path|, MemorySnapshotNodeTable::Id |parent_node_row_id| and
  // ProcessMemorySnapshotId |proc_snapshot_row_id|. Generates map of
  // MemoryAllocatorNodeId and MemorySnapshotNodeTable::Id |id_node_map| which
  // is used at time of filling out of MemorySnapshotEdgeTable.
  std::optional<tables::MemorySnapshotNodeTable::Id> EmitNode(
      const GlobalNodeGraph::Node& node,
      const std::string& path,
      std::optional<tables::MemorySnapshotNodeTable::Id> parent_node_row_id,
      ProcessMemorySnapshotId& proc_snapshot_row_id,
      IdNodeMap& id_node_map);

  void GenerateGraphFromRawNodesAndEmitRows();

  TraceProcessorContext* context_;
  std::array<StringId, 3> level_of_detail_ids_;
  std::array<StringId, 2> unit_ids_;
  RawMemoryNodeMap aggregate_raw_nodes_;
  int64_t last_snapshot_timestamp_;
  LevelOfDetail last_snapshot_level_of_detail_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MEMORY_TRACKER_SNAPSHOT_PARSER_H_
