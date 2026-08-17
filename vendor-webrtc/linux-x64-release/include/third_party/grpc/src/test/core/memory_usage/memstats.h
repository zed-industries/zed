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

#ifndef GRPC_TEST_CORE_MEMORY_USAGE_MEMSTATS_H
#define GRPC_TEST_CORE_MEMORY_USAGE_MEMSTATS_H

#include <optional>

// IWYU pragma: no_include <bits/types/struct_rusage.h>

// Get the memory usage of either the calling process or another process using
// the pid
long GetMemUsage(std::optional<int> pid = std::nullopt);

struct MemStats {
  long rss;  // Resident set size, in kb
  static MemStats Snapshot() { return MemStats{GetMemUsage()}; }
};

#endif  // GRPC_TEST_CORE_MEMORY_USAGE_MEMSTATS_H
