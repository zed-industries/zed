// Copyright 2023 The Centipede Authors.
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

#ifndef THIRD_PARTY_CENTIPEDE_RUNNER_SANCOV_OBJECT_H_
#define THIRD_PARTY_CENTIPEDE_RUNNER_SANCOV_OBJECT_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <vector>

#include "absl/base/nullability.h"
#include "./centipede/pc_info.h"
#include "./centipede/runner_dl_info.h"

// TODO(kcc): gradually replace the old code in runner_sancov.cc with this code.
// The difference is that the old code allows only one sancov-instrumented DSO,
// while this code allows multiple instrumented DSO.
// TODO(kcc): this code is not a full replacement for the old code yet.

namespace fuzztest::internal {

// Information about one sancov-instrumented object (DSO).
// See https://clang.llvm.org/docs/SanitizerCoverage.html.
// These structs are created as globals and are linker-initialized to zero.
struct SanCovObject {
  DlInfo dl_info;                       // Obtained via GetDlInfo.
  PCGuard *pc_guard_start;              // __sanitizer_cov_trace_pc_guard_init.
  PCGuard *pc_guard_stop;               // __sanitizer_cov_trace_pc_guard_init.
  const PCInfo *pcs_beg;                // __sanitizer_cov_pcs_init
  const PCInfo *pcs_end;                // __sanitizer_cov_pcs_init
  const uintptr_t *cfs_beg;             // __sanitizer_cov_cfs_init
  const uintptr_t *cfs_end;             // __sanitizer_cov_cfs_init
  uint8_t *inline_8bit_counters_start;  // __sanitizer_cov_8bit_counters_init
  uint8_t *inline_8bit_counters_stop;   // __sanitizer_cov_8bit_counters_init
};

// A fixed size array of SanCovObject structs.
// Also linker-initialized to zero.
class SanCovObjectArray {
 public:
  // To be called in __sanitizer_cov_trace_pc_guard_init.
  void PCGuardInit(PCGuard *absl_nullable start, PCGuard *stop);

  // To be called in __sanitizer_cov_pcs_init.
  void PCInfoInit(const PCInfo *absl_nullable pcs_beg, const PCInfo *pcs_end);

  // To be called in __sanitizer_cov_cfs_init.
  void CFSInit(const uintptr_t *cfs_beg, const uintptr_t *cfs_end);

  // To be called in __sanitizer_cov_8bit_counters_init.
  void Inline8BitCountersInit(uint8_t *inline_8bit_counters_start,
                              uint8_t *inline_8bit_counters_stop);

  // Sets all inline counters to zero.
  void ClearInlineCounters();

  // Calls `callback` for every non-zero inline counter of every object.
  // The `idx` passed to `callback` is the zero-based index of the counter
  // in the entire process, not just in the object.
  // `counter_value` is the non-zero value of the counter.
  void ForEachNonZeroInlineCounter(
      const std::function<void(size_t idx, uint8_t counter_value)> &callback)
      const;

  // Returns the number of sancov-instrumented objects observed so far.
  size_t size() const { return size_; }

  // Returns the number of sancov-instrumented PCs across all DSOs.
  size_t NumInstrumentedPCs() const { return num_instrumented_pcs_; }

  // Returns a vector of PCInfo for all instrumented DSOs.
  // Every PC in the vector has the object's ASLR base (dl_info.start_address)
  // subtracted. So, unless there is exactly one instrumented DSO, this vector
  // by itself is not sufficient to map PCs to DSOs or symbols.
  // This will require additional information. TODO(kcc) implement.
  std::vector<PCInfo> CreatePCTable() const;

  // Returns a vector of uintptr_t corresponding to a control flow table:
  // https://clang.llvm.org/docs/SanitizerCoverage.html#tracing-control-flow.
  // Similar to CreatePCTable(), subtracts the ASLR base from every PC before
  // returning.
  std::vector<uintptr_t> CreateCfTable() const;

  // Returns a DsoTable computed from all SanCovObjects.
  DsoTable CreateDsoTable() const;

 private:
  static constexpr size_t kMaxSize = 64 * 1024;
  // Set by `PCGuardInit`/`Inline8BitCountersInit` if the current DSO has an
  // empty PC guard/counter table, which should not be tracked in a
  // SanCovObject.
  //
  // TODO(b/326950832): Clean up the SanCov init handling to check assumptions
  // (e.g. callback ordering) in a cleaner way.
  bool skipping_no_code_dso_;
  size_t size_;
  SanCovObject objects_[kMaxSize];
  size_t num_instrumented_pcs_;  // Total number of instrumented PCs.
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_RUNNER_SANCOV_OBJECT_H_
