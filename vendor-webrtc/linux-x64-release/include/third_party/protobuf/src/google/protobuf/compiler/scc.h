// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_SCC_H__
#define GOOGLE_PROTOBUF_COMPILER_SCC_H__

#include <memory>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/absl_check.h"
#include "absl/memory/memory.h"
#include "google/protobuf/descriptor.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {

// Description of each strongly connected component. Note that the order
// of both the descriptors in this SCC and the order of children is
// deterministic.
struct SCC {
  std::vector<const Descriptor*> descriptors;
  std::vector<const SCC*> children;

  const Descriptor* GetRepresentative() const { return descriptors[0]; }

  // All messages must necessarily be in the same file.
  const FileDescriptor* GetFile() const { return descriptors[0]->file(); }
};

// This class is used for analyzing the SCC for each message, to ensure linear
// instead of quadratic performance, if we do this per message we would get
// O(V*(V+E)).
template <class DepsGenerator>
class PROTOC_EXPORT SCCAnalyzer {
 public:
  explicit SCCAnalyzer() : index_(0) {}
  SCCAnalyzer(const SCCAnalyzer&) = delete;
  SCCAnalyzer& operator=(const SCCAnalyzer&) = delete;

  const SCC* GetSCC(const Descriptor* descriptor) {
    auto it = cache_.find(descriptor);
    if (it == cache_.end()) {
      return DFS(descriptor).scc;
    }
    return it->second->scc;
  }

 private:
  struct NodeData {
    const SCC* scc;  // if null it means its still on the stack
    int index;
    int lowlink;
  };

  absl::flat_hash_map<const Descriptor*, std::unique_ptr<NodeData>> cache_;
  std::vector<const Descriptor*> stack_;
  int index_;
  std::vector<std::unique_ptr<SCC>> garbage_bin_;

  SCC* CreateSCC() {
    garbage_bin_.emplace_back(new SCC());
    return garbage_bin_.back().get();
  }

  // Tarjan's Strongly Connected Components algo
  NodeData DFS(const Descriptor* descriptor) {
    // Mark visited by inserting in map.
    auto ins = cache_.try_emplace(descriptor, absl::make_unique<NodeData>());
    // Must not have visited already.
    ABSL_DCHECK(ins.second);
    NodeData& result = *ins.first->second;
    // Initialize data structures.
    result.index = result.lowlink = index_++;
    stack_.push_back(descriptor);

    // Recurse the fields / nodes in graph
    for (const auto* dep : DepsGenerator()(descriptor)) {
      ABSL_CHECK(dep);
      auto it = cache_.find(dep);
      if (it == cache_.end()) {
        // unexplored node
        NodeData child_data = DFS(dep);
        result.lowlink = std::min(result.lowlink, child_data.lowlink);
      } else {
        NodeData& child_data = *it->second;
        if (child_data.scc == nullptr) {
          // Still in the stack_ so we found a back edge
          result.lowlink = std::min(result.lowlink, child_data.index);
        }
      }
    }
    if (result.index == result.lowlink) {
      // This is the root of a strongly connected component
      SCC* scc = CreateSCC();
      while (true) {
        const Descriptor* scc_desc = stack_.back();
        scc->descriptors.push_back(scc_desc);
        // Remove from stack
        stack_.pop_back();
        cache_[scc_desc]->scc = scc;

        if (scc_desc == descriptor) break;
      }

      // The order of descriptors is random and depends how this SCC was
      // discovered. In-order to ensure maximum stability we sort it by name.
      std::sort(scc->descriptors.begin(), scc->descriptors.end(),
                [](const Descriptor* a, const Descriptor* b) {
                  return a->full_name() < b->full_name();
                });
      AddChildren(scc);
    }
    return result;
  }

  // Add the SCC's that are children of this SCC to its children.
  void AddChildren(SCC* scc) {
    absl::flat_hash_set<const SCC*> seen;
    for (auto descriptor : scc->descriptors) {
      for (auto child_msg : DepsGenerator()(descriptor)) {
        ABSL_CHECK(child_msg);
        const SCC* child = GetSCC(child_msg);
        if (child == scc) continue;
        if (seen.insert(child).second) {
          scc->children.push_back(child);
        }
      }
    }
  }
};

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_SCC_H__
