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

#ifndef INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_GRAPH_PROCESSOR_H_
#define INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_GRAPH_PROCESSOR_H_

#include <sys/types.h>

#include <map>
#include <memory>
#include <set>
#include <string>

#include "perfetto/base/proc_utils.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/graph.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/raw_process_memory_node.h"

namespace perfetto {
namespace trace_processor {

class PERFETTO_EXPORT_COMPONENT GraphProcessor {
 public:
  // This map does not own the pointers inside.
  using RawMemoryNodeMap =
      std::map<base::PlatformProcessId, std::unique_ptr<RawProcessMemoryNode>>;

  static std::unique_ptr<GlobalNodeGraph> CreateMemoryGraph(
      const RawMemoryNodeMap& process_nodes);

  static void RemoveWeakNodesFromGraph(GlobalNodeGraph* global_graph);

  static void AddOverheadsAndPropagateEntries(GlobalNodeGraph* global_graph);

  static void CalculateSizesForGraph(GlobalNodeGraph* global_graph);

  static std::map<base::PlatformProcessId, uint64_t>
  ComputeSharedFootprintFromGraph(const GlobalNodeGraph& global_graph);

 private:
  friend class GraphProcessorTest;

  static void CollectAllocatorNodes(const RawProcessMemoryNode& source,
                                    GlobalNodeGraph* global_graph,
                                    GlobalNodeGraph::Process* process_graph);

  static void AddEdges(const RawProcessMemoryNode& source,
                       GlobalNodeGraph* global_graph);

  static void MarkImplicitWeakParentsRecursively(GlobalNodeGraph::Node* node);

  static void MarkWeakOwnersAndChildrenRecursively(
      GlobalNodeGraph::Node* node,
      std::set<const GlobalNodeGraph::Node*>* nodes);

  static void RemoveWeakNodesRecursively(GlobalNodeGraph::Node* parent);

  static void AssignTracingOverhead(const std::string& allocator,
                                    GlobalNodeGraph* global_graph,
                                    GlobalNodeGraph::Process* process);

  static GlobalNodeGraph::Node::Entry AggregateNumericWithNameForNode(
      GlobalNodeGraph::Node* node,
      const std::string& name);

  static void AggregateNumericsRecursively(GlobalNodeGraph::Node* node);

  static void PropagateNumericsAndDiagnosticsRecursively(
      GlobalNodeGraph::Node* node);

  static std::optional<uint64_t> AggregateSizeForDescendantNode(
      GlobalNodeGraph::Node* root,
      GlobalNodeGraph::Node* descendant);

  static void CalculateSizeForNode(GlobalNodeGraph::Node* node);

  /**
   * Calculate not-owned and not-owning sub-sizes of a memory allocator node
   * from its children's (sub-)sizes.
   *
   * Not-owned sub-size refers to the aggregated memory of all children which
   * is not owned by other MADs. Conversely, not-owning sub-size is the
   * aggregated memory of all children which do not own another MAD. The
   * diagram below illustrates these two concepts:
   *
   *     ROOT 1                         ROOT 2
   *     size: 4                        size: 5
   *     not-owned sub-size: 4          not-owned sub-size: 1 (!)
   *     not-owning sub-size: 0 (!)     not-owning sub-size: 5
   *
   *      ^                              ^
   *      |                              |
   *
   *     PARENT 1   ===== owns =====>   PARENT 2
   *     size: 4                        size: 5
   *     not-owned sub-size: 4          not-owned sub-size: 5
   *     not-owning sub-size: 4         not-owning sub-size: 5
   *
   *      ^                              ^
   *      |                              |
   *
   *     CHILD 1                        CHILD 2
   *     size [given]: 4                size [given]: 5
   *     not-owned sub-size: 4          not-owned sub-size: 5
   *     not-owning sub-size: 4         not-owning sub-size: 5
   *
   * This method assumes that (1) the size of the node, its children, and its
   * owners [see calculateSizes()] and (2) the not-owned and not-owning
   * sub-sizes of both the children and owners of the node have already been
   * calculated [depth-first post-order traversal].
   */
  static void CalculateNodeSubSizes(GlobalNodeGraph::Node* node);

  /**
   * Calculate owned and owning coefficients of a memory allocator node and
   * its owners.
   *
   * The owning coefficient refers to the proportion of a node's not-owning
   * sub-size which is attributed to the node (only relevant to owning MADs).
   * Conversely, the owned coefficient is the proportion of a node's
   * not-owned sub-size, which is attributed to it (only relevant to owned
   * MADs).
   *
   * The not-owned size of the owned node is split among its owners in the
   * order of the ownership importance as demonstrated by the following
   * example:
   *
   *                                          memory allocator nodes
   *                                   OWNED  OWNER1  OWNER2  OWNER3 OWNER4
   *       not-owned sub-size [given]     10       -       -       - -
   *      not-owning sub-size [given]      -       6       7       5 8
   *               importance [given]      -       2       2       1 0
   *    attributed not-owned sub-size      2       -       -       - -
   *   attributed not-owning sub-size      -       3       4       0 1
   *                owned coefficient   2/10       -       -       - -
   *               owning coefficient      -     3/6     4/7     0/5 1/8
   *
   * Explanation: Firstly, 6 bytes are split equally among OWNER1 and OWNER2
   * (highest importance). OWNER2 owns one more byte, so its attributed
   * not-owning sub-size is 6/2 + 1 = 4 bytes. OWNER3 is attributed no size
   * because it is smaller than the owners with higher priority. However,
   * OWNER4 is larger, so it's attributed the difference 8 - 7 = 1 byte.
   * Finally, 2 bytes remain unattributed and are hence kept in the OWNED
   * node as attributed not-owned sub-size. The coefficients are then
   * directly calculated as fractions of the sub-sizes and corresponding
   * attributed sub-sizes.
   *
   * Note that we always assume that all ownerships of a node overlap (e.g.
   * OWNER3 is subsumed by both OWNER1 and OWNER2). Hence, the table could
   * be alternatively represented as follows:
   *
   *                                 owned memory range
   *              0   1   2    3    4    5    6        7        8   9  10
   *   Priority 2 |  OWNER1 + OWNER2 (split)  | OWNER2 |
   *   Priority 1 | (already attributed) |
   *   Priority 0 | - - -  (already attributed)  - - - | OWNER4 |
   *    Remainder | - - - - - (already attributed) - - - - - -  | OWNED |
   *
   * This method assumes that (1) the size of the node [see calculateSizes()]
   * and (2) the not-owned size of the node and not-owning sub-sizes of its
   * owners [see the first step of calculateEffectiveSizes()] have already
   * been calculated. Note that the method doesn't make any assumptions about
   * the order in which nodes are visited.
   */
  static void CalculateNodeOwnershipCoefficient(GlobalNodeGraph::Node* node);

  /**
   * Calculate cumulative owned and owning coefficients of a memory allocator
   * node from its (non-cumulative) owned and owning coefficients and the
   * cumulative coefficients of its parent and/or owned node.
   *
   * The cumulative coefficients represent the total effect of all
   * (non-strict) ancestor ownerships on a memory allocator node. The
   * cumulative owned coefficient of a MAD can be calculated simply as:
   *
   *   cumulativeOwnedC(M) = ownedC(M) * cumulativeOwnedC(parent(M))
   *
   * This reflects the assumption that if a parent of a child MAD is
   * (partially) owned, then the parent's owner also indirectly owns (a part
   * of) the child MAD.
   *
   * The cumulative owning coefficient of a MAD depends on whether the MAD
   * owns another node:
   *
   *                           [if M doesn't own another MAD]
   *                         / cumulativeOwningC(parent(M))
   *   cumulativeOwningC(M) =
   *                         \ [if M owns another MAD]
   *                           owningC(M) * cumulativeOwningC(owned(M))
   *
   * The reasoning behind the first case is similar to the one for cumulative
   * owned coefficient above. The only difference is that we don't need to
   * include the node's (non-cumulative) owning coefficient because it is
   * implicitly 1.
   *
   * The formula for the second case is derived as follows: Since the MAD
   * owns another node, its memory is not included in its parent's not-owning
   * sub-size and hence shouldn't be affected by the parent's corresponding
   * cumulative coefficient. Instead, the MAD indirectly owns everything
   * owned by its owned node (and so it should be affected by the
   * corresponding coefficient).
   *
   * Note that undefined coefficients (and coefficients of non-existent
   * nodes) are implicitly assumed to be 1.
   *
   * This method assumes that (1) the size of the node [see calculateSizes()],
   * (2) the (non-cumulative) owned and owning coefficients of the node [see
   * the second step of calculateEffectiveSizes()], and (3) the cumulative
   * coefficients of the node's parent and owned MADs (if present)
   * [depth-first pre-order traversal] have already been calculated.
   */
  static void CalculateNodeCumulativeOwnershipCoefficient(
      GlobalNodeGraph::Node* node);

  /**
   * Calculate the effective size of a memory allocator node.
   *
   * In order to simplify the (already complex) calculation, we use the fact
   * that effective size is cumulative (unlike regular size), i.e. the
   * effective size of a non-leaf node is equal to the sum of effective sizes
   * of its children. The effective size of a leaf MAD is calculated as:
   *
   *   effectiveSize(M) = size(M) * cumulativeOwningC(M) * cumulativeOwnedC(M)
   *
   * This method assumes that (1) the size of the node and its children [see
   * calculateSizes()] and (2) the cumulative owning and owned coefficients
   * of the node (if it's a leaf node) [see the third step of
   * calculateEffectiveSizes()] or the effective sizes of its children (if
   * it's a non-leaf node) [depth-first post-order traversal] have already
   * been calculated.
   */
  static void CalculateNodeEffectiveSize(GlobalNodeGraph::Node* node);
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_GRAPH_PROCESSOR_H_
