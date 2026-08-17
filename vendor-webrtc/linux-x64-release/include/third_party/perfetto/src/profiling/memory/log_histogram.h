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

#ifndef SRC_PROFILING_MEMORY_LOG_HISTOGRAM_H_
#define SRC_PROFILING_MEMORY_LOG_HISTOGRAM_H_

#include <stddef.h>

#include <array>
#include <cinttypes>
#include <utility>
#include <vector>

namespace perfetto {
namespace profiling {

class LogHistogram {
 public:
  static const uint64_t kMaxBucket;
  static constexpr size_t kBuckets = 20;

  void Add(uint64_t value) { values_[GetBucket(value)]++; }
  std::vector<std::pair<uint64_t, uint64_t>> GetData() const;

 private:
  size_t GetBucket(uint64_t value);

  std::array<uint64_t, kBuckets> values_ = {};
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_MEMORY_LOG_HISTOGRAM_H_
