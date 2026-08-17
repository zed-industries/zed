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

#ifndef INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_GRAPH_H_
#define INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_GRAPH_H_

#include <sys/types.h>

#include <forward_list>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "perfetto/base/export.h"
#include "perfetto/base/proc_utils.h"
#include "perfetto/ext/base/string_utils.h"
#include "perfetto/ext/trace_processor/importers/memory_tracker/memory_allocator_node_id.h"

namespace perfetto {
namespace trace_processor {

const base::PlatformProcessId kNullProcessId = 0;

// Contains processed node graphs for each process and in the global space.
// This class is also the arena which owns the nodes of the graph.
class PERFETTO_EXPORT_COMPONENT GlobalNodeGraph {
 public:
  class Node;
  class Edge;
  class PreOrderIterator;
  class PostOrderIterator;

  // Graph of nodes either associated with a process or with
  // the shared space.
  class PERFETTO_EXPORT_COMPONENT Process {
   public:
    Process(base::PlatformProcessId pid, GlobalNodeGraph* global_graph);
    ~Process();

    // Creates a node in the node graph which is associated with the
    // given |id|, |path| and |weak|ness and returns it.
    GlobalNodeGraph::Node* CreateNode(MemoryAllocatorNodeId id,
                                      const std::string& path,
                                      bool weak);

    // Returns the node in the graph at the given |path| or nullptr
    // if no such node exists in the provided |graph|.
    GlobalNodeGraph::Node* FindNode(const std::string& path);

    base::PlatformProcessId pid() const { return pid_; }
    GlobalNodeGraph* global_graph() const { return global_graph_; }
    GlobalNodeGraph::Node* root() const { return root_; }

   private:
    base::PlatformProcessId pid_;
    GlobalNodeGraph* global_graph_;
    GlobalNodeGraph::Node* root_;
    Process(const Process&) = delete;
    Process& operator=(const Process&) = delete;
  };

  // A single node in the graph of allocator nodes associated with a
  // certain path and containing the entries for this path.
  class PERFETTO_EXPORT_COMPONENT Node {
   public:
    // Auxilary data (a scalar number or a string) about this node each
    // associated with a key.
    struct PERFETTO_EXPORT_COMPONENT Entry {
      enum Type {
        kUInt64,
        kString,
      };

      // The units of the entry if the entry is a scalar. The scalar
      // refers to either a number of objects or a size in bytes.
      enum ScalarUnits {
        kObjects,
        kBytes,
      };

      // Creates the entry with the appropriate type.
      Entry(ScalarUnits units, uint64_t value);
      explicit Entry(const std::string& value);

      const Type type;
      const ScalarUnits units;

      // The value of the entry if this entry has a string type.
      const std::string value_string;

      // The value of the entry if this entry has a integer type.
      const uint64_t value_uint64;
    };

    explicit Node(GlobalNodeGraph::Process* node_graph, Node* parent);
    ~Node();

    // Gets the direct child of a node for the given |subpath|.
    Node* GetChild(const std::string& name) const;

    // Inserts the given |node| as a child of the current node
    // with the given |subpath| as the key.
    void InsertChild(const std::string& name, Node* node);

    // Creates a child for this node with the given |name| as the key.
    Node* CreateChild(const std::string& name);

    // Checks if the current node is a descendent (i.e. exists as a child,
    // child of a child, etc.) of the given node |possible_parent|.
    bool IsDescendentOf(const Node& possible_parent) const;

    // Adds an entry for this node node with the given |name|, |units| and
    // type.
    void AddEntry(const std::string& name,
                  Entry::ScalarUnits units,
                  uint64_t value);
    void AddEntry(const std::string& name, const std::string& value);

    // Adds an edge which indicates that this node is owned by
    // another node.
    void AddOwnedByEdge(Edge* edge);

    // Sets the edge indicates that this node owns another node.
    void SetOwnsEdge(Edge* edge);

    bool is_weak() const { return weak_; }
    void set_weak(bool weak) { weak_ = weak; }
    bool is_explicit() const { return explicit_; }
    void set_explicit(bool explicit_node) { explicit_ = explicit_node; }
    uint64_t not_owned_sub_size() const { return not_owned_sub_size_; }
    void add_not_owned_sub_size(uint64_t addition) {
      not_owned_sub_size_ += addition;
    }
    uint64_t not_owning_sub_size() const { return not_owning_sub_size_; }
    void add_not_owning_sub_size(uint64_t addition) {
      not_owning_sub_size_ += addition;
    }
    double owned_coefficient() const { return owned_coefficient_; }
    void set_owned_coefficient(double owned_coefficient) {
      owned_coefficient_ = owned_coefficient;
    }
    double owning_coefficient() const { return owning_coefficient_; }
    void set_owning_coefficient(double owning_coefficient) {
      owning_coefficient_ = owning_coefficient;
    }
    double cumulative_owned_coefficient() const {
      return cumulative_owned_coefficient_;
    }
    void set_cumulative_owned_coefficient(double cumulative_owned_coefficient) {
      cumulative_owned_coefficient_ = cumulative_owned_coefficient;
    }
    double cumulative_owning_coefficient() const {
      return cumulative_owning_coefficient_;
    }
    void set_cumulative_owning_coefficient(
        double cumulative_owning_coefficient) {
      cumulative_owning_coefficient_ = cumulative_owning_coefficient;
    }
    MemoryAllocatorNodeId id() const { return id_; }
    void set_id(MemoryAllocatorNodeId id) { id_ = id; }
    GlobalNodeGraph::Edge* owns_edge() const { return owns_edge_; }
    std::map<std::string, Node*>* children() { return &children_; }
    const std::map<std::string, Node*>& const_children() const {
      return children_;
    }
    std::vector<GlobalNodeGraph::Edge*>* owned_by_edges() {
      return &owned_by_edges_;
    }
    const Node* parent() const { return parent_; }
    const GlobalNodeGraph::Process* node_graph() const { return node_graph_; }
    std::map<std::string, Entry>* entries() { return &entries_; }
    const std::map<std::string, Entry>& const_entries() const {
      return entries_;
    }

   private:
    GlobalNodeGraph::Process* node_graph_;
    Node* const parent_;
    MemoryAllocatorNodeId id_;
    std::map<std::string, Entry> entries_;
    std::map<std::string, Node*> children_;
    bool explicit_ = false;
    bool weak_ = false;
    uint64_t not_owning_sub_size_ = 0;
    uint64_t not_owned_sub_size_ = 0;
    double owned_coefficient_ = 1;
    double owning_coefficient_ = 1;
    double cumulative_owned_coefficient_ = 1;
    double cumulative_owning_coefficient_ = 1;

    GlobalNodeGraph::Edge* owns_edge_;
    std::vector<GlobalNodeGraph::Edge*> owned_by_edges_;

    Node(const Node&) = delete;
    Node& operator=(const Node&) = delete;
  };

  // An edge in the node graph which indicates ownership between the
  // source and target nodes.
  class PERFETTO_EXPORT_COMPONENT Edge {
   public:
    Edge(GlobalNodeGraph::Node* source,
         GlobalNodeGraph::Node* target,
         int priority);

    GlobalNodeGraph::Node* source() const { return source_; }
    GlobalNodeGraph::Node* target() const { return target_; }
    int priority() const { return priority_; }

   private:
    GlobalNodeGraph::Node* const source_;
    GlobalNodeGraph::Node* const target_;
    const int priority_;
  };

  // An iterator-esque class which yields nodes in a depth-first pre order.
  class PERFETTO_EXPORT_COMPONENT PreOrderIterator {
   public:
    explicit PreOrderIterator(std::vector<Node*>&& root_nodes);
    PreOrderIterator(PreOrderIterator&& other);
    ~PreOrderIterator();

    // Yields the next node in the DFS post-order traversal.
    Node* next();

   private:
    std::vector<Node*> to_visit_;
    std::set<const Node*> visited_;
  };

  // An iterator-esque class which yields nodes in a depth-first post order.
  class PERFETTO_EXPORT_COMPONENT PostOrderIterator {
   public:
    explicit PostOrderIterator(std::vector<Node*>&& root_nodes);
    PostOrderIterator(PostOrderIterator&& other);
    ~PostOrderIterator();

    // Yields the next node in the DFS post-order traversal.
    Node* next();

   private:
    std::vector<Node*> to_visit_;
    std::set<Node*> visited_;
    std::vector<Node*> path_;
  };

  using ProcessNodeGraphMap =
      std::map<base::PlatformProcessId,
               std::unique_ptr<GlobalNodeGraph::Process>>;
  using IdNodeMap = std::map<MemoryAllocatorNodeId, Node*>;

  GlobalNodeGraph();
  ~GlobalNodeGraph();

  // Creates a container for all the node graphs for the process given
  // by the given |process_id|.
  GlobalNodeGraph::Process* CreateGraphForProcess(
      base::PlatformProcessId process_id);

  // Adds an edge in the node graph with the given source and target nodes
  // and edge priority.
  void AddNodeOwnershipEdge(Node* owner, Node* owned, int priority);

  // Returns an iterator which yields nodes in the nodes in this graph in
  // pre-order. That is, children and owners of nodes are returned after the
  // node itself.
  PreOrderIterator VisitInDepthFirstPreOrder();

  // Returns an iterator which yields nodes in the nodes in this graph in
  // post-order. That is, children and owners of nodes are returned before the
  // node itself.
  PostOrderIterator VisitInDepthFirstPostOrder();

  const IdNodeMap& nodes_by_id() const { return nodes_by_id_; }
  GlobalNodeGraph::Process* shared_memory_graph() const {
    return shared_memory_graph_.get();
  }
  const ProcessNodeGraphMap& process_node_graphs() const {
    return process_node_graphs_;
  }
  const std::forward_list<Edge>& edges() const { return all_edges_; }

 private:
  // Creates a node in the arena which is associated with the given
  // |process_graph| and for the given |parent|.
  Node* CreateNode(GlobalNodeGraph::Process* process_graph,
                   GlobalNodeGraph::Node* parent);

  std::forward_list<Node> all_nodes_;
  std::forward_list<Edge> all_edges_;
  IdNodeMap nodes_by_id_;
  std::unique_ptr<GlobalNodeGraph::Process> shared_memory_graph_;
  ProcessNodeGraphMap process_node_graphs_;
  GlobalNodeGraph(const GlobalNodeGraph&) = delete;
  GlobalNodeGraph& operator=(const GlobalNodeGraph&) = delete;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_GRAPH_H_
