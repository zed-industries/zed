// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTEXT_MANAGER_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTEXT_MANAGER_H_

#include <deque>
#include <functional>
#include <map>
#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/log/absl_check.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_state.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {

// Calculator context manager owns and manages all calculator context objects of
// a calculator node.
class CalculatorContextManager {
 public:
  CalculatorContextManager() {}

  void Initialize(CalculatorState* calculator_state,
                  std::shared_ptr<tool::TagMap> input_tag_map,
                  std::shared_ptr<tool::TagMap> output_tag_map,
                  bool calculator_run_in_parallel);

  // Sets the callback that can setup the input and output stream shards in a
  // newly constructed calculator context. Then, initializes the default
  // calculator context.
  absl::Status PrepareForRun(
      std::function<absl::Status(CalculatorContext*)> setup_shards_callback);

  // Invoked by CalculatorNode::CleanupAfterRun().
  void CleanupAfterRun() ABSL_LOCKS_EXCLUDED(contexts_mutex_);

  // Returns true if the default calculator context has been initialized.
  bool HasDefaultCalculatorContext() const {
    return default_context_ != nullptr;
  }

  // Returns a pointer to the default calculator context that is used for
  // sequential execution. A source node should always reuse its default
  // calculator context.
  CalculatorContext* GetDefaultCalculatorContext() const;

  // Returns the context with the smallest input timestamp in active_contexts_.
  // The input timestamp of the calculator context is returned in
  // *context_input_timestamp.
  CalculatorContext* GetFrontCalculatorContext(
      Timestamp* context_input_timestamp) ABSL_LOCKS_EXCLUDED(contexts_mutex_);

  // For sequential execution, returns a pointer to the default calculator
  // context. For parallel execution, creates or reuses a calculator context,
  // and inserts the calculator context with the given input timestamp into
  // active_contexts_. Returns a pointer to the prepared calculator context.
  // The ownership of the calculator context object isn't tranferred to the
  // caller.
  CalculatorContext* PrepareCalculatorContext(Timestamp input_timestamp)
      ABSL_LOCKS_EXCLUDED(contexts_mutex_);

  // Removes the context with the smallest input timestamp from active_contexts_
  // and moves the calculator context to idle_contexts_. The caller must
  // guarantee that the output shards in the calculator context have been
  // propagated before calling this function.
  void RecycleCalculatorContext() ABSL_LOCKS_EXCLUDED(contexts_mutex_);

  // Returns true if active_contexts_ is non-empty.
  bool HasActiveContexts() ABSL_LOCKS_EXCLUDED(contexts_mutex_);

  int NumberOfContextTimestamps(
      const CalculatorContext& calculator_context) const {
    return calculator_context.NumberOfTimestamps();
  }

  bool ContextHasInputTimestamp(
      const CalculatorContext& calculator_context) const {
    return calculator_context.HasInputTimestamp();
  }

  void PushInputTimestampToContext(CalculatorContext* calculator_context,
                                   Timestamp input_timestamp) {
    ABSL_CHECK(calculator_context);
    calculator_context->PushInputTimestamp(input_timestamp);
  }

  void PopInputTimestampFromContext(CalculatorContext* calculator_context) {
    ABSL_CHECK(calculator_context);
    calculator_context->PopInputTimestamp();
  }

  void SetGraphStatusInContext(CalculatorContext* calculator_context,
                               const absl::Status& status) {
    ABSL_CHECK(calculator_context);
    calculator_context->SetGraphStatus(status);
  }

 private:
  CalculatorState* calculator_state_;
  std::shared_ptr<tool::TagMap> input_tag_map_;
  std::shared_ptr<tool::TagMap> output_tag_map_;
  bool calculator_run_in_parallel_;

  // The callback to setup the input and output stream shards in a newly
  // constructed calculator context.
  // NOTE: This callback invokes input/output stream handler methods.
  // The callback is used to break the circular dependency between
  // calculator context manager and input/output stream handlers.
  std::function<absl::Status(CalculatorContext*)> setup_shards_callback_;

  // The default calculator context that is always reused for sequential
  // execution. It is also used by Open() and Close() method of a parallel
  // calculator.
  std::unique_ptr<CalculatorContext> default_context_;
  // The mutex for synchronizing the operations on active_contexts_ and
  // idle_contexts_ during parallel execution.
  absl::Mutex contexts_mutex_;
  // A map from input timestamps to calculator contexts.
  std::map<Timestamp, std::unique_ptr<CalculatorContext>> active_contexts_
      ABSL_GUARDED_BY(contexts_mutex_);
  // Idle calculator contexts that are ready for reuse.
  std::deque<std::unique_ptr<CalculatorContext>> idle_contexts_
      ABSL_GUARDED_BY(contexts_mutex_);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTEXT_MANAGER_H_
