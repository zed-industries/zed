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

#ifndef MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_TIMESTAMP_ALIGN_INPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_TIMESTAMP_ALIGN_INPUT_STREAM_HANDLER_H_

#include <functional>
#include <memory>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/input_stream_handler.h"
#include "mediapipe/framework/stream_handler/timestamp_align_input_stream_handler.pb.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

// The input streams must have the same time unit but may have different time
// origins (also called epochs). The timestamp_base_tag_index option
// designates an input stream as the timestamp base.
//
// TimestampAlignInputStreamHandler operates in two phases:
//
// 1. Pre-initialization: In this phase, the input stream handler passes
// through input packets in the timestamp base input stream, but buffers the
// input packets in all other input streams. This phase ends when the input
// stream handler has an input packet in every input stream. It uses the
// the timestamps of these input packets to calculate the timestamp offset of
// each input stream with respect to the timestamp base input stream. The
// timestamp offsets are saved for use in the next phase.
//
// 2. Post-initialization: In this phase, the input stream handler behaves
// like the DefaultInputStreamHandler, except that timestamp offsets are
// applied to the packet timestamps.
class TimestampAlignInputStreamHandler : public InputStreamHandler {
 public:
  TimestampAlignInputStreamHandler() = delete;
  TimestampAlignInputStreamHandler(std::shared_ptr<tool::TagMap> tag_map,
                                   CalculatorContextManager* cc_manager,
                                   const mediapipe::MediaPipeOptions& options,
                                   bool calculator_run_in_parallel);

  void PrepareForRun(std::function<void()> headers_ready_callback,
                     std::function<void()> notification_callback,
                     std::function<void(CalculatorContext*)> schedule_callback,
                     std::function<void(absl::Status)> error_callback) override;

 protected:
  // In TimestampAlignInputStreamHandler, a node is "ready" if:
  // - before the timestamp offsets are initialized: we have received a packet
  //   in the timestamp base input stream, or
  // - after the timestamp offsets are initialized: the minimum bound (over
  //   all empty streams) is greater than the smallest timestamp of any
  //   stream, which means we have received all the packets that will be
  //   available at the next timestamp, or
  // - all streams are done (need to call Close() in this case).
  // Note that all packet timestamps and timestamp bounds are aligned with the
  // timestamp base.
  NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) override;

  // Only invoked when associated GetNodeReadiness() returned kReadyForProcess.
  void FillInputSet(Timestamp input_timestamp,
                    InputStreamShardSet* input_set) override;

 private:
  CollectionItemId timestamp_base_stream_id_;

  absl::Mutex mutex_;
  bool offsets_initialized_ ABSL_GUARDED_BY(mutex_) = false;
  std::vector<TimestampDiff> timestamp_offsets_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_TIMESTAMP_ALIGN_INPUT_STREAM_HANDLER_H_
