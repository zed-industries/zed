/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_SHELL_TRANSITIONS_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_SHELL_TRANSITIONS_TRACKER_H_

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/importers/common/args_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/winscope_proto_mapping.h"

namespace perfetto {
namespace trace_processor {

// Tracks information in the transition table.
class ShellTransitionsTracker : public Destructible {
 public:
  explicit ShellTransitionsTracker(TraceProcessorContext*);
  virtual ~ShellTransitionsTracker() override;

  static ShellTransitionsTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->shell_transitions_tracker) {
      context->shell_transitions_tracker.reset(
          new ShellTransitionsTracker(context));
    }
    return static_cast<ShellTransitionsTracker*>(
        context->shell_transitions_tracker.get());
  }

  ArgsTracker::BoundInserter AddArgsTo(int32_t transition_id);

  void SetTimestamp(int32_t transition_id, int64_t timestamp_ns);

  void Flush();

 private:
  struct TransitionInfo {
    tables::WindowManagerShellTransitionsTable::Id row_id;
    ArgsTracker args_tracker;
  };

  TransitionInfo* GetOrInsertTransition(int32_t transition_id);

  TraceProcessorContext* context_;
  std::unordered_map<int32_t, TransitionInfo> transitions_infos_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_SHELL_TRANSITIONS_TRACKER_H_
