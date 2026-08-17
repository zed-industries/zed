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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_DOMINATOR_TREE_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_DOMINATOR_TREE_H_

#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/sqlite/bindings/sqlite_aggregate_function.h"

namespace perfetto::trace_processor {

// An SQL aggregate-function which computes the dominator-tree [1] of a graph.
//
// Arguments:
//  1) |source_node_id|: a non-null uint32 corresponding to the source of edge.
//  2) |dest_node_id|: a non-null uint32 corresponding to the destination of
//     the edge.
//  3) |start_node_id|: a non-null uint32 corresponding to of the start node in
//     the graph from which reachability should be computed.
//
// Returns:
//  A table with the dominator tree of the input graph. The schema of the table
//  is (node_id int64_t, dominator_node_id optional<int64_t>).
//
// Note: as this function takes table columns as an argument, it is not intended
// to be used directly from SQL: instead a "dominator_tree" macro exists in
// the standard library, wrapping it and making it user-friendly.
//
// Implementation notes:
// This class implements the Lengauer-Tarjan Dominators algorithm [2]. This was
// chosen as it runs on O(nlog(n)) time: as we expect this class to be used on
// large tables (i.e. tables containing Java heap graphs), it's important that
// the code is efficient.
//
// As Lengauer-Tarjan Dominators is not the most intuitive algorithm [3] might
// be a useful resource for grasping the key principles behind it.
//
// [1] https://en.wikipedia.org/wiki/Dominator_(graph_theory)
// [2] https://dl.acm.org/doi/10.1145/357062.357071
class DominatorTree : public SqliteAggregateFunction<DominatorTree> {
 public:
  static constexpr char kName[] = "__intrinsic_dominator_tree";
  static constexpr int kArgCount = 3;
  using UserDataContext = StringPool;

  static void Step(sqlite3_context*, int argc, sqlite3_value** argv);
  static void Final(sqlite3_context* ctx);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_DOMINATOR_TREE_H_
