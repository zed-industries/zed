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

#ifndef SRC_BIGTRACE_ORCHESTRATOR_ORCHESTRATOR_IMPL_H_
#define SRC_BIGTRACE_ORCHESTRATOR_ORCHESTRATOR_IMPL_H_

#include <grpcpp/client_context.h>
#include <memory>
#include <mutex>
#include <optional>
#include "perfetto/ext/base/threading/thread_pool.h"
#include "protos/perfetto/bigtrace/orchestrator.grpc.pb.h"
#include "protos/perfetto/bigtrace/worker.grpc.pb.h"

namespace perfetto::bigtrace {
namespace {
const uint64_t kDefaultMaxQueryConcurrency = 8;
}  // namespace

class OrchestratorImpl final : public protos::BigtraceOrchestrator::Service {
 public:
  explicit OrchestratorImpl(std::unique_ptr<protos::BigtraceWorker::Stub> stub,
                            uint32_t max_query_concurrency);

  grpc::Status Query(
      grpc::ServerContext*,
      const protos::BigtraceQueryArgs* args,
      grpc::ServerWriter<protos::BigtraceQueryResponse>* writer) override;

 private:
  std::unique_ptr<protos::BigtraceWorker::Stub> stub_;
  std::unique_ptr<base::ThreadPool> pool_;
  uint32_t max_query_concurrency_ = kDefaultMaxQueryConcurrency;
  uint32_t query_count_ = 0;
  std::mutex query_count_mutex_;
};

}  // namespace perfetto::bigtrace

#endif  // SRC_BIGTRACE_ORCHESTRATOR_ORCHESTRATOR_IMPL_H_
