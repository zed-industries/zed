// Copyright 2023 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_IMMEDIATE_INPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_IMMEDIATE_INPUT_STREAM_HANDLER_H_

#include <functional>
#include <memory>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/input_stream_handler.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {

// An input stream handler that delivers input packets to the Calculator
// immediately, with no dependency between input streams.  It also invokes
// Calculator::Process when any input stream becomes done.
//
// NOTE: If packets arrive successively on different input streams with
// identical or decreasing timestamps, this input stream handler will
// invoke its Calculator with a sequence of InputTimestamps that is
// non-increasing.  Its Calculator is responsible for accumulating packets
// with the required timestamps before processing and delivering output.
class ImmediateInputStreamHandler : public InputStreamHandler {
 public:
  ImmediateInputStreamHandler() = delete;
  ImmediateInputStreamHandler(
      std::shared_ptr<tool::TagMap> tag_map,
      CalculatorContextManager* calculator_context_manager,
      const MediaPipeOptions& options, bool calculator_run_in_parallel);

 protected:
  // Reinitializes this InputStreamHandler before each CalculatorGraph run.
  void PrepareForRun(std::function<void()> headers_ready_callback,
                     std::function<void()> notification_callback,
                     std::function<void(CalculatorContext*)> schedule_callback,
                     std::function<void(absl::Status)> error_callback) override;

  // Returns kReadyForProcess whenever a Packet is available at any of
  // the input streams, or any input stream becomes done.
  NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) override;

  // Selects a packet on each stream with an available packet with the
  // specified timestamp, leaving other input streams unaffected.
  void FillInputSet(Timestamp input_timestamp,
                    InputStreamShardSet* input_set) override;

  // Returns the number of sync-sets maintained by this input-handler.
  int SyncSetCount() override;

  absl::Mutex mutex_;
  // The packet-set builder for each input stream.
  std::vector<SyncSet> sync_sets_ ABSL_GUARDED_BY(mutex_);
  // The input timestamp for each kReadyForProcess input stream.
  std::vector<Timestamp> ready_timestamps_ ABSL_GUARDED_BY(mutex_);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_IMMEDIATE_INPUT_STREAM_HANDLER_H_
