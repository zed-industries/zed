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

#ifndef MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_IN_ORDER_OUTPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_IN_ORDER_OUTPUT_STREAM_HANDLER_H_

#include <memory>
#include <utility>

// TODO: Move protos in another CL after the C++ code migration.
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/output_stream_handler.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {

// InOrderOutputStreamHandler supports both sequential and parallel processing
// of input packets, and will deliver the output packets in increasing timestamp
// order.
class InOrderOutputStreamHandler : public OutputStreamHandler {
 public:
  InOrderOutputStreamHandler(
      std::shared_ptr<tool::TagMap> tag_map,
      CalculatorContextManager* calculator_context_manager,
      const MediaPipeOptions& options, bool calculator_run_in_parallel)
      : OutputStreamHandler(std::move(tag_map), calculator_context_manager,
                            options, calculator_run_in_parallel) {}

 private:
  void PropagationLoop() ABSL_EXCLUSIVE_LOCKS_REQUIRED(timestamp_mutex_) final;

  void PropagatePackets(CalculatorContext** calculator_context,
                        Timestamp* context_timestamp)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(timestamp_mutex_);

  void PropagationBound(CalculatorContext** calculator_context,
                        Timestamp* context_timestamp)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(timestamp_mutex_);
};
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_IN_ORDER_OUTPUT_STREAM_HANDLER_H_
