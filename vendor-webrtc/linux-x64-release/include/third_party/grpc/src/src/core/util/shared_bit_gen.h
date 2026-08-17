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

#ifndef GRPC_SRC_CORE_UTIL_SHARED_BIT_GEN_H
#define GRPC_SRC_CORE_UTIL_SHARED_BIT_GEN_H

#include "absl/random/random.h"

namespace grpc_core {

class SharedBitGen {
 public:
  SharedBitGen() = default;
  SharedBitGen(const SharedBitGen&) = delete;
  SharedBitGen& operator=(const SharedBitGen&) = delete;
  SharedBitGen(SharedBitGen&&) = default;
  SharedBitGen& operator=(SharedBitGen&&) = default;

  using result_type = absl::BitGen::result_type;
  result_type operator()() { return bit_gen_(); }

  static constexpr auto min() { return absl::BitGen::min(); }
  static constexpr auto max() { return absl::BitGen::max(); }

 private:
  // TODO(ctiller): Perhaps use per-cpu storage? Would add additional overhead
  // for the mutex acquisition.
  static thread_local absl::BitGen bit_gen_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_SHARED_BIT_GEN_H
