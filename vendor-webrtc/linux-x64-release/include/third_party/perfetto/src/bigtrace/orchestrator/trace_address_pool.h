/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_BIGTRACE_ORCHESTRATOR_TRACE_ADDRESS_POOL_H_
#define SRC_BIGTRACE_ORCHESTRATOR_TRACE_ADDRESS_POOL_H_

#include <mutex>
#include <optional>
#include <vector>

namespace perfetto::bigtrace {

// This pool contains all trace addresses of a given query and facilitates a
// thread safe way of popping traces and returning them to the pool if the query
// is cancelled
class TraceAddressPool {
 public:
  explicit TraceAddressPool(const std::vector<std::string>& trace_addresses);
  std::optional<std::string> Pop();
  void MarkCancelled(std::string trace_address);
  uint32_t RemainingCount();

 private:
  std::vector<std::string> trace_addresses_;
  std::mutex trace_addresses_lock_;
  uint32_t running_queries_ = 0;
};

}  // namespace perfetto::bigtrace

#endif  // SRC_BIGTRACE_ORCHESTRATOR_TRACE_ADDRESS_POOL_H_
