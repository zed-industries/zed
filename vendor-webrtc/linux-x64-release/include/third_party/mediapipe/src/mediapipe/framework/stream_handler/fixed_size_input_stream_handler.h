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

#ifndef MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_FIXED_SIZE_INPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_FIXED_SIZE_INPUT_STREAM_HANDLER_H_

#include <cstdint>
#include <list>
#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/input_stream_handler.h"
#include "mediapipe/framework/stream_handler/default_input_stream_handler.h"

namespace mediapipe {

// Input stream handler that limits each input queue to a maximum of
// target_queue_size packets, discarding older packets as needed.  When a
// timestamp is dropped from a stream, it is dropped from all others as well.
//
// For example, a calculator node with one input stream and the following input
// stream handler specs:
//
// node {
//   calculator: "CalculatorRunningAtOneFps"
//   input_stream: "packets_streaming_in_at_ten_fps"
//   input_stream_handler {
//     input_stream_handler: "FixedSizeInputStreamHandler"
//   }
// }
//
// will always try to keep the newest packet in the input stream.
//
// A few details: FixedSizeInputStreamHandler takes action when any stream grows
// to trigger_queue_size or larger.  It then keeps at most target_queue_size
// packets in every InputStreamImpl.  Every stream is truncated at the same
// timestamp, so that each included timestamp delivers the same packets as
// DefaultInputStreamHandler includes.
class FixedSizeInputStreamHandler : public DefaultInputStreamHandler {
 public:
  FixedSizeInputStreamHandler() = delete;
  FixedSizeInputStreamHandler(std::shared_ptr<tool::TagMap> tag_map,
                              CalculatorContextManager* cc_manager,
                              const MediaPipeOptions& options,
                              bool calculator_run_in_parallel);

 private:
  // Drops packets if all input streams exceed trigger_queue_size.
  void EraseAllSurplus() ABSL_EXCLUSIVE_LOCKS_REQUIRED(erase_mutex_);

  // Returns the latest timestamp allowed before a bound.
  Timestamp PreviousAllowedInStream(Timestamp bound);

  // Returns the lowest timestamp at which a packet may arrive at any stream.
  Timestamp MinStreamBound();

  // Returns the lowest timestamp of a packet ready to process.
  Timestamp MinTimestampToProcess();

  // Keeps only the most recent target_queue_size packets in each stream
  // exceeding trigger_queue_size.  Also, discards all packets older than the
  // first kept timestamp on any stream.
  void EraseAnySurplus(bool keep_one)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(erase_mutex_);

  void EraseSurplusPackets(bool keep_one)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(erase_mutex_);

  NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) override;

  void AddPackets(CollectionItemId id,
                  const std::list<Packet>& packets) override;

  void MovePackets(CollectionItemId id, std::list<Packet>* packets) override;

  void FillInputSet(Timestamp input_timestamp,
                    InputStreamShardSet* input_set) override;

 private:
  int32_t trigger_queue_size_;
  int32_t target_queue_size_;
  bool fixed_min_size_;
  // Indicates that GetNodeReadiness has returned kReadyForProcess once, and
  // the corresponding call to FillInputSet has not yet completed.
  bool pending_ ABSL_GUARDED_BY(erase_mutex_);
  // The timestamp used to truncate all input streams.
  Timestamp kept_timestamp_ ABSL_GUARDED_BY(erase_mutex_);
  absl::Mutex erase_mutex_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_FIXED_SIZE_INPUT_STREAM_HANDLER_H_
