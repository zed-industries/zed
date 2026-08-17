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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_STRUCTURAL_TREE_PARTITION_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_STRUCTURAL_TREE_PARTITION_H_

#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/sqlite/bindings/sqlite_aggregate_function.h"

namespace perfetto::trace_processor {

// An SQL aggregate-function which partitions a tree into a forest of trees
// based on a given grouping key (i.e. group) for each node.
//
// # Arguments:
//  1) |node_id|: The id of the node. Should be a non-null uint32.
//  2) |parent_node_ids|:  The id of the parent node in the tree. Should be a
//     possibly null uint32. Should be null iff it is the root of the tree.
//  3) |group|: The group of the node. Should be a non-null uint32 and dense
//     for performance reasons.
//
// # Returns:
//  A value table with the schema (id, parent_id, group) containing a forest of
//  trees created by partitioning the tree based on the value of |groups|.
//
//  Specifically, for each tree in the forest, all the node in the tree have the
//  same |group| and all ancestor and descendants of that node are precisely the
//  the same ancestors and descendants in the original tree which have the same
//  |group|.
//
// # Example
// ## Input
//   id | parent_id | group
//   ---+-----------+--------
//   1  | NULL      | 1
//   2  | 1         | 1
//   3  | 2         | 2
//   4  | 2         | 2
//   5  | 4         | 1
//   6  | 4         | 3
//   7  | 4         | 2
//
// Or as a graph:
//         1 (1)
//        /
//       2 (1)
//      /    |
//     3 (2) 4 (2)
//            |
//             5 (1)
//            /   |
//         6 (3)  7 (2)
//
// ## Possible Output (order of rows is implementation-defined)
//   id | parent_id | group
//   ---+-----------+-------
//   1  | NULL      | 1
//   2  | 1         | 1
//   3  | NULL      | 2
//   4  | NULL      | 2
//   5  | 2         | 1
//   6  | NULL      | 3
//   7  | 4         | 2
//
// Or as a forest:
//    1 (1)       3 (2)      4 (2)        6 (3)
//     |                      |
//    2 (1)                  7 (2)
//     |
//    5 (1)
//
// # Notes:
// - Exactly one input node must have |parent_id| NULL with that node acting
//   as the root of the tree.
// - Every node *must* have a valid parent id which appears somewhere in |ids|.
// - The ordering of output rows is not guaranteed and should not be relied
//   upon.
// - This function is not intended to be used directly from SQL: instead macros
//   exist in the standard library, wrapping it and making it user-friendly.
struct StructuralTreePartition
    : public SqliteAggregateFunction<StructuralTreePartition> {
  static constexpr char kName[] = "__intrinsic_structural_tree_partition";
  static constexpr int kArgCount = 3;
  using UserDataContext = StringPool;

  static void Step(sqlite3_context*, int argc, sqlite3_value** argv);
  static void Final(sqlite3_context* ctx);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_STRUCTURAL_TREE_PARTITION_H_
