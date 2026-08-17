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

#ifndef THIRD_PARTY_CENTIPEDE_REVERSE_PC_TABLE_H_
#define THIRD_PARTY_CENTIPEDE_REVERSE_PC_TABLE_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>

#include "./centipede/pc_info.h"

namespace fuzztest::internal {

// Maps PCs to PCGuard objects.
class ReversePCTable {
 public:
  ReversePCTable() = default;
  // Non copyable, non-movable.
  ReversePCTable(const ReversePCTable &) = delete;
  ReversePCTable &operator=(const ReversePCTable &) = delete;
  ReversePCTable(ReversePCTable &&) = default;
  ReversePCTable &operator=(ReversePCTable &&) = default;

  // Constructs the reverse PC table from `pc_table`.
  // The assumption is that all PCs are relatively small, such that the
  // implementation is allowed to create an array indexed by a PC.
  void SetFromPCs(const PCTable& pc_table) {
    num_pcs_ = pc_table.size();
    if (table_ != nullptr) delete[] table_;
    if (num_pcs_ == 0) {
      size_ = 0;
      table_ = nullptr;
      return;
    }
    // Compute max_pc.
    uintptr_t max_pc = 0;
    for (const auto& pc_info : pc_table) {
      max_pc = std::max(max_pc, pc_info.pc);
    }
    // Create an array of max_pc + 1 elements such that we can directly
    // index this array with any valid PC.
    size_ = max_pc + 1;
    table_ = new PCGuard[size_];
    std::fill(table_, table_ + size_, kInvalidPCGuard);
    // Make sure all PC indices fit into PCGuard::kMaxNumPCs.
    if (pc_table.size() >= PCGuard::kMaxNumPCs)
      __builtin_trap();  // no logging in runner. TODO(kcc): use RunnerCheck.
    // Fill in the table.
    for (size_t idx = 0; idx < pc_table.size(); ++idx) {
      const auto &pc_info = pc_table[idx];
      if (pc_info.pc >= size_) __builtin_trap();  // TODO(kcc): use RunnerCheck.
      table_[pc_info.pc] = {
          /*is_function_entry=*/pc_info.has_flag(PCInfo::kFuncEntry),
          /*pc_index=*/static_cast<uint32_t>(idx)};
    }
  }

  // Returns PCGuard that corresponds to `pc`. If `pc` was not present in
  // `pc_table` passed to SetFromPCs, returns kInvalidPCGuard. This is a hot
  // function and needs to be as simple and fast as possible.
  PCGuard GetPCGuard(uintptr_t pc) const {
    if (pc >= size_) return kInvalidPCGuard;
    return table_[pc];
  }

  // Returns the number of PCs that was passed to SetFromPCs().
  size_t NumPcs() const { return num_pcs_; }

 private:
  // A PCGuard object, such that IsValid() will return false.
  static constexpr PCGuard kInvalidPCGuard = {0, PCGuard::kInvalidPcIndex};

  // We use size_ and table_ pointer instead of std::vector<> because
  // (1) we need ReversePCTable object to be accessible even after the
  // destruction (in static storage duration); (2) size_ is cheaper to
  // compute inside GetPCIndex(). This would cause leakage if not
  // declared as static - one can explicitly call SetFromPCs({}) to
  // free the table.
  size_t size_ = 0;
  size_t num_pcs_ = 0;
  PCGuard *table_ = nullptr;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_REVERSE_PC_TABLE_H_
