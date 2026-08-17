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

#ifndef THIRD_PARTY_CENTIPEDE_CONTROL_FLOW_H_
#define THIRD_PARTY_CENTIPEDE_CONTROL_FLOW_H_

#include <cstddef>
#include <cstdint>
#include <istream>
#include <mutex>  //NOLINT
#include <ostream>
#include <string_view>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "./centipede/pc_info.h"
#include "./common/defs.h"
#include "./common/logging.h"

namespace fuzztest::internal {

class SymbolTable;  // To avoid mutual inclusion with symbol_table.h.

// Reads a PCTable from `file_path`, returns it. Returns empty table on error.
PCTable ReadPcTableFromFile(std::string_view file_path);

// Helper for GetPcTableFromBinary, for binaries built with
// -fsanitize-coverage=trace-pc. Returns the PCTable reconstructed from
// `binary_path` with `<objdump_path> -d`. May create a file `tmp_path`, but
// will delete it afterwards.
PCTable GetPcTableFromBinaryWithTracePC(std::string_view binary_path,
                                        std::string_view objdump_path,
                                        std::string_view tmp_path);

// PCIndex: an index into the PCTable.
// We use 32-bit int for compactness since PCTable is never too large.
using PCIndex = uint32_t;
// A set of PCIndex-es, order is not important.
using PCIndexVec = std::vector<PCIndex>;

// Array of elements in __sancov_cfs section.
// CFTable is created by the compiler/linker in the instrumented binary.
// https://clang.llvm.org/docs/SanitizerCoverage.html#tracing-control-flow.
using CFTable = std::vector<intptr_t>;

// Reads a CFTable from `file_path`, returns it. Returns empty table on error.
CFTable ReadCfTable(std::string_view file_path);

// Same as above but reads from a stream.
CFTable ReadCfTable(std::istream &in);

// Writes the `cf_table` to `out`.
void WriteCfTable(const CFTable &cf_table, std::ostream &out);

// Reads a DsoTable from `file_path`, returns it. Returns empty table on error.
DsoTable ReadDsoTableFromFile(std::string_view file_path);

class ControlFlowGraph {
 public:
  // Reads form __sancov_cfs section. On error it crashes, if the section is not
  // there, the graph_ will be empty.
  void InitializeControlFlowGraph(const CFTable &cf_table,
                                  const PCTable &pc_table);

  // Returns the vector of successor PCs for the given basic block PC.
  const std::vector<uintptr_t> &GetSuccessors(uintptr_t basic_block) const;

  // Returns the number of cfg entries.
  size_t size() const { return graph_.size(); }

  // Checks if basic_block is in cfg.
  bool exists(const uintptr_t basic_block) const {
    return graph_.contains(basic_block);
  }

  // Returns cyclomatic complexity of function PC. CHECK-fails if it is not a
  // valid function PC.
  uint32_t GetCyclomaticComplexity(uintptr_t pc) const {
    auto it = function_complexities_.find(pc);
    CHECK(it != function_complexities_.end());
    return it->second;
  }

  // Returns true if the given basic block is function entry.
  bool BlockIsFunctionEntry(PCIndex pc_index) const {
    // TODO(ussuri): Change the following to use CHECK_LE(pc_index,
    // func_entries_.size()) and have a death test.
    return pc_index < func_entries_.size() ? func_entries_[pc_index] : false;
  }

  // Returns the idx in pc_table associated with the PC, CHECK-fails if the PC
  // is not in the pc_table.
  PCIndex GetPcIndex(uintptr_t pc) const {
    auto it = pc_index_map_.find(pc);
    CHECK(it != pc_index_map_.end()) << VV(pc) << " is not in pc_table.";
    return it->second;
  }

  // Returns true if the PC is in PCTable.
  bool IsInPcTable(uintptr_t pc) const { return pc_index_map_.contains(pc); }

  // Returns a vector& containing all basic blocks (represented by their PCs)
  // reachable from `pc`. The reachability is computed once, lazily.
  // The method is const, under the hood it uses a mutable data member.
  // Thread-safe: can be called concurrently from multiple threads
  const std::vector<uintptr_t> &LazyGetReachabilityForPc(uintptr_t pc) const {
    CHECK_EQ(reachability_.size(), pc_index_map_.size());
    auto pc_index = GetPcIndex(pc);
    std::call_once(*(reachability_[pc_index].once), [this, &pc, &pc_index]() {
      reachability_[pc_index].reach = ComputeReachabilityForPc(pc);
    });
    return reachability_[pc_index].reach;
  }

 private:
  // Map from PC to the idx in pc_table.
  absl::flat_hash_map<uintptr_t, PCIndex> pc_index_map_;
  // A vector of size PCTable. func_entries[idx] is true iff means the PC at idx
  // is a function entry.
  std::vector<bool> func_entries_;
  // A map with PC as the keys and vector of PCs as value.
  absl::flat_hash_map<uintptr_t, std::vector<uintptr_t>> graph_;
  // A map from function PC to its calculated cyclomatic complexity. It is
  // to avoid unnecessary calls to ComputeFunctionCyclomaticComplexity.
  absl::flat_hash_map<uintptr_t, uint32_t> function_complexities_;

  // Returns a vector of PCs reachable from `pc`, not in any particular order.
  // The result always includes `pc`, since any block is reachable from itself.
  std::vector<uintptr_t> ComputeReachabilityForPc(uintptr_t pc) const;
  FRIEND_TEST(ControlFlowGraph, ComputeReachabilityForPc);

  // ReachInfo is a struct to store reachability information for each PC in
  // pc_table. The once flag is used to make sure the reach vector is populated
  // only once lazily in a thread-friendly manner.
  struct ReachInfo {
    mutable std::once_flag *once;
    mutable std::vector<uintptr_t> reach;
    ReachInfo() : once(new std::once_flag) {}
    ~ReachInfo() { delete once; }
  };
  // A vector of size PCTable. reachability_[idx] is reachability info for the
  // `idx`th pc. Conceptually it is constant, but we compute it lazily, hence
  // 'mutable'
  std::vector<ReachInfo> reachability_;
};

// Computes the Cyclomatic Complexity for the given function,
// https://en.wikipedia.org/wiki/Cyclomatic_complexity.
uint32_t ComputeFunctionCyclomaticComplexity(uintptr_t pc,
                                             const ControlFlowGraph &cfg);

}  // namespace fuzztest::internal
#endif  // THIRD_PARTY_CENTIPEDE_CONTROL_FLOW_H_
