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

#ifndef MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_MUX_INPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_MUX_INPUT_STREAM_HANDLER_H_

#include <memory>
#include <utility>

#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/input_stream_handler.h"

namespace mediapipe {

// Implementation of the input stream handler for the MuxCalculator.
//
// One of the input streams is the control stream; all the other input streams
// are data streams. To make MuxInputStreamHandler work properly, the tag of the
// input streams must obey the following rules:
// Let N be the number of input streams. Data streams must use tag "INPUT" with
// index 0, ..., N - 2; the control stream must use tag "SELECT".
//
// The control stream carries packets of type 'int'. The 'int' value in a
// control stream packet must be a valid index in the range 0, ..., N - 2 and
// select the data stream at that index. The selected data stream must have a
// packet with the same timestamp as the control stream packet.
//
// When the control stream is done, GetNodeReadiness() returns
// NodeReadiness::kReadyForClose.
//
// TODO: pass the input stream tags to the MuxInputStreamHandler
// constructor so that it can refer to input streams by tag. See b/30125118.
class MuxInputStreamHandler : public InputStreamHandler {
 public:
  MuxInputStreamHandler() = delete;
  MuxInputStreamHandler(std::shared_ptr<tool::TagMap> tag_map,
                        CalculatorContextManager* cc_manager,
                        const MediaPipeOptions& options,
                        bool calculator_run_in_parallel)
      : InputStreamHandler(std::move(tag_map), cc_manager, options,
                           calculator_run_in_parallel) {}

 private:
  CollectionItemId GetControlStreamId() const;
  void RemoveOutdatedDataPackets(Timestamp timestamp);

 protected:
  // In MuxInputStreamHandler, a node is "ready" if:
  // - the control stream is done (need to call Close() in this case), or
  // - we have received the packets on the control stream and the selected data
  //   stream at the next timestamp.
  NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) override;

  // Only invoked when associated GetNodeReadiness() returned kReadyForProcess.
  void FillInputSet(Timestamp input_timestamp,
                    InputStreamShardSet* input_set) override;

 private:
  // Must be acquired when manipulating the control and data streams to ensure
  // we have a consistent view of the two streams.
  absl::Mutex input_streams_mutex_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_MUX_INPUT_STREAM_HANDLER_H_
