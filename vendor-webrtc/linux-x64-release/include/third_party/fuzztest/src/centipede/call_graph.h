// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_CALL_GRAPH_H_
#define THIRD_PARTY_CENTIPEDE_CALL_GRAPH_H_

#include <cstdint>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "./centipede/control_flow.h"
#include "./centipede/pc_info.h"
#include "./common/logging.h"

namespace fuzztest::internal {

class CallGraph {
 public:
  // Reads in the CfTable from __sancov_cfs section. On error it crashes, if the
  // section is not available, the hash maps will be empty.
  void InitializeCallGraph(const CFTable& cf_table, const PCTable& pc_table);

  const std::vector<uintptr_t>& GetFunctionCallees(uintptr_t pc) const {
    CHECK(IsFunctionEntry(pc)) << VV(pc) << " is not a function entry.";
    const auto it = call_graph_.find(pc);
    if (it == call_graph_.cend()) return empty_;
    return it->second;
  }
  const std::vector<uintptr_t>& GetBasicBlockCallees(uintptr_t pc) const {
    CHECK(basic_blocks_.contains(pc)) << VV(pc) << " is not a basic block.";
    const auto it = basic_block_callees_.find(pc);
    if (it == basic_block_callees_.cend()) return empty_;
    return it->second;
  }
  const absl::flat_hash_set<uintptr_t>& GetFunctionEntries() const {
    return function_entries_;
  }

  bool IsFunctionEntry(uintptr_t pc) const {
    return function_entries_.contains(pc);
  }

 private:
  // call_graph_: the key is function entry PC and value is all the
  // callees of that function. It keep only non-zero vectors in a map. Meaning
  // that if a function does not have any callee, it won't be in this map.
  absl::flat_hash_map<uintptr_t, std::vector<uintptr_t>> call_graph_;
  // bb_callees_: the key is a basic block PC and value is all callees in
  // that basic block. It keep only non-zero vectors in a map. Meaning that if a
  // basic_block does not have any callee, it won't be in this map.
  absl::flat_hash_map<uintptr_t, std::vector<uintptr_t>> basic_block_callees_;
  absl::flat_hash_set<uintptr_t> function_entries_;
  absl::flat_hash_set<uintptr_t> basic_blocks_;
  std::vector<uintptr_t> empty_;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_CALL_GRAPH_H_
