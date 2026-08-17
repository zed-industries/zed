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

#ifndef SRC_BIGTRACE_WORKER_REPOSITORY_POLICIES_LOCAL_TRACE_PROCESSOR_LOADER_H_
#define SRC_BIGTRACE_WORKER_REPOSITORY_POLICIES_LOCAL_TRACE_PROCESSOR_LOADER_H_

#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/trace_processor.h"
#include "src/bigtrace/worker/repository_policies/trace_processor_loader.h"

namespace perfetto::bigtrace {

class LocalTraceProcessorLoader : public TraceProcessorLoader {
 public:
  base::StatusOr<std::unique_ptr<trace_processor::TraceProcessor>>
  LoadTraceProcessor(const std::string& path) override;
};

}  // namespace perfetto::bigtrace

#endif  // SRC_BIGTRACE_WORKER_REPOSITORY_POLICIES_LOCAL_TRACE_PROCESSOR_LOADER_H_
