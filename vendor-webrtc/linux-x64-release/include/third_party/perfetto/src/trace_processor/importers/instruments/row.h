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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_ROW_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_ROW_H_

#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/util/build_id.h"

namespace perfetto::trace_processor::instruments_importer {

// TODO(leszeks): Would be nice if these were strong type aliases, to be
// type safe.
using ThreadId = uint32_t;
using ProcessId = uint32_t;
using BacktraceId = uint32_t;
using BacktraceFrameId = uint32_t;
using BinaryId = uint32_t;

constexpr uint32_t kNullId = 0u;

struct Binary {
  std::string path;
  BuildId uuid = BuildId::FromRaw(std::string(""));
  long long load_addr = 0;
  long long max_addr = 0;
};

struct Frame {
  long long addr = 0;
  std::string name;
  BinaryId binary = kNullId;
};

struct Process {
  int pid = 0;
  StringPool::Id fmt = StringPool::Id::Null();
};

struct Thread {
  int tid = 0;
  StringPool::Id fmt = StringPool::Id::Null();
  ProcessId process = kNullId;
};

struct Backtrace {
  std::vector<BacktraceFrameId> frames;
};

struct alignas(8) Row {
  int64_t timestamp_;
  uint32_t core_id;
  ThreadId thread = kNullId;
  BacktraceId backtrace = kNullId;
};

}  // namespace perfetto::trace_processor::instruments_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_ROW_H_
