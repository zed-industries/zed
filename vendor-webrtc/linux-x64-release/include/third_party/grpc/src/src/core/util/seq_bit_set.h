// Copyright 2025 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_UTIL_SEQ_BIT_SET_H
#define GRPC_SRC_CORE_UTIL_SEQ_BIT_SET_H

#include <cstdint>
#include <set>

#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"

namespace grpc_core {

// A bitset of flags for whether a sequence number has been
// seen or not.
// Assumes that the bits are turned on in roughly sequence
// order, and so early bits can be compacted once the sequence
// is full.
// Starts with all bits unset.
class SeqBitSet {
 public:
  // Returns true if seq was already set, false if not.
  bool Set(uint64_t seq);
  bool IsSet(uint64_t seq) const;

  template <typename Sink>
  friend void AbslStringify(Sink& out, SeqBitSet sbs) {
    std::set<uint64_t> rest = sbs.far_future_bits_;
    for (size_t i = 0; i < kNumFutureBitEntries; i++) {
      for (int j = 0; j < 64; j++) {
        if (sbs.future_bits_[i] & (1ull << j)) {
          rest.insert(sbs.epoch_ + i * 64 + j);
        }
      }
    }
    out.Append(absl::StrCat("epoch:", sbs.epoch_, " set:{",
                            absl::StrJoin(rest, ","), "}"));
  }

 private:
  static constexpr std::size_t kNumFutureBitEntries = 3;
  // All bits before seq epoch_ are set.
  uint64_t epoch_ = 0;
  uint64_t future_bits_[kNumFutureBitEntries] = {0, 0, 0};
  std::set<uint64_t> far_future_bits_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_SEQ_BIT_SET_H
