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

#ifndef MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_EARLY_CLOSE_INPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_EARLY_CLOSE_INPUT_STREAM_HANDLER_H_

#include <memory>
#include <utility>

#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/input_stream_handler.h"
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {

// Implementation of an input stream handler that considers a node as ready for
// Close() if any input stream is done.
class EarlyCloseInputStreamHandler : public InputStreamHandler {
 public:
  EarlyCloseInputStreamHandler() = delete;
  EarlyCloseInputStreamHandler(std::shared_ptr<tool::TagMap> tag_map,
                               CalculatorContextManager* cc_manager,
                               const mediapipe::MediaPipeOptions& options,
                               bool calculator_run_in_parallel)
      : InputStreamHandler(std::move(tag_map), cc_manager, options,
                           calculator_run_in_parallel) {}

 protected:
  // In EarlyCloseInputStreamHandler, a node is "ready" if:
  // - any stream is done (need to call Close() in this case), or
  // - the minimum bound (over all empty streams) is greater than the smallest
  //   timestamp of any stream, which means we have received all the packets
  //   that will be available at the next timestamp.
  NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) override;

  // Only invoked when associated GetNodeReadiness() returned kReadyForProcess.
  void FillInputSet(Timestamp input_timestamp,
                    InputStreamShardSet* input_set) override;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_EARLY_CLOSE_INPUT_STREAM_HANDLER_H_
