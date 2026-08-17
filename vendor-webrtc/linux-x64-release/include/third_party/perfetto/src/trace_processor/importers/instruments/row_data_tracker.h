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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_ROW_DATA_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_ROW_DATA_TRACKER_H_

#include "src/trace_processor/importers/instruments/row.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor::instruments_importer {

template <typename T>
struct IdPtr {
  uint32_t id;
  T* ptr;
};

// Keeps track of row data.
class RowDataTracker : public Destructible {
 public:
  static RowDataTracker& GetOrCreate(TraceProcessorContext* context) {
    if (!context->instruments_row_data_tracker) {
      context->instruments_row_data_tracker.reset(new RowDataTracker());
    }
    return static_cast<RowDataTracker&>(*context->instruments_row_data_tracker);
  }
  ~RowDataTracker() override;

  IdPtr<Thread> NewThread();
  Thread* GetThread(ThreadId id);

  IdPtr<Process> NewProcess();
  Process* GetProcess(ProcessId id);

  IdPtr<Frame> NewFrame();
  Frame* GetFrame(BacktraceFrameId id);

  IdPtr<Backtrace> NewBacktrace();
  Backtrace* GetBacktrace(BacktraceId id);

  IdPtr<Binary> NewBinary();
  Binary* GetBinary(BinaryId id);

 private:
  explicit RowDataTracker();

  std::vector<Thread> threads_;
  std::vector<Process> processes_;
  std::vector<Frame> frames_;
  std::vector<Backtrace> backtraces_;
  std::vector<Binary> binaries_;
};

}  // namespace perfetto::trace_processor::instruments_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_ROW_DATA_TRACKER_H_
