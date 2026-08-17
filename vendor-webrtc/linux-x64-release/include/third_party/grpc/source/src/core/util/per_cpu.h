// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_PER_CPU_H
#define GRPC_SRC_CORE_UTIL_PER_CPU_H

#include <grpc/support/cpu.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <algorithm>
#include <cstddef>
#include <limits>
#include <memory>

// Sharded collections of objects
// This used to be per-cpu, now it's much less so - but still a way to limit
// contention.

namespace grpc_core {

class PerCpuOptions {
 public:
  // Set the number of cpus that colocate on the same shard
  PerCpuOptions SetCpusPerShard(size_t cpus_per_shard) {
    cpus_per_shard_ = std::max<size_t>(1, cpus_per_shard);
    return *this;
  }

  // Set the maximum number of allowable shards
  PerCpuOptions SetMaxShards(size_t max_shards) {
    max_shards_ = std::max<size_t>(1, max_shards);
    return *this;
  }

  size_t cpus_per_shard() const { return cpus_per_shard_; }
  size_t max_shards() const { return max_shards_; }

  size_t Shards();
  size_t ShardsForCpuCount(size_t cpu_count);

 private:
  size_t cpus_per_shard_ = 1;
  size_t max_shards_ = std::numeric_limits<size_t>::max();
};

class PerCpuShardingHelper {
 public:
  size_t GetShardingBits() {
    // We periodically refresh the last seen cpu to try to ensure that we spread
    // load evenly over all shards of a per-cpu data structure, even in the
    // event of shifting thread distributions, load patterns.
    // Ideally we'd just call gpr_cpu_current_cpu() every call of this function
    // to get perfect distribution, but that function is currently quite slow on
    // some platforms and so we need to cache it somewhat.
    if (GPR_UNLIKELY(state_.uses_until_refresh == 0)) state_ = State();
    --state_.uses_until_refresh;
    return state_.last_seen_cpu;
  }

 private:
  struct State {
    uint16_t last_seen_cpu = gpr_cpu_current_cpu();
    uint16_t uses_until_refresh = 65535;
  };
  static thread_local State state_;
};

template <typename T>
class PerCpu {
 public:
  // Options are not defaulted to try and force consideration of what the
  // options specify.
  explicit PerCpu(PerCpuOptions options) : shards_(options.Shards()) {}

  T& this_cpu() { return data_[sharding_helper_.GetShardingBits() % shards_]; }

  T* begin() { return data_.get(); }
  T* end() { return data_.get() + shards_; }
  const T* begin() const { return data_.get(); }
  const T* end() const { return data_.get() + shards_; }

 private:
  PerCpuShardingHelper sharding_helper_;
  const size_t shards_;
  std::unique_ptr<T[]> data_{new T[shards_]};
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_PER_CPU_H
