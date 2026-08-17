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

#ifndef MEDIAPIPE_TENSORFLOW_CALCULATORS_TENSORFLOW_SESSION_H_
#define MEDIAPIPE_TENSORFLOW_CALCULATORS_TENSORFLOW_SESSION_H_

#include <memory>

#include "tensorflow/core/public/session.h"

namespace mediapipe {
struct TensorFlowSession {
  // TensorFlow session wrapper to get around the RTTI issue.
  std::unique_ptr<tensorflow::Session> session;

  // Store an optional mapping to the between MediaPipe tags and TensorFlow
  // tensor names. Creating this mapping when the session is loaded allows more
  // flexible definition of mapping tags to tensors across platforms.
  std::map<std::string, std::string> tag_to_tensor_map;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_TENSORFLOW_CALCULATORS_TENSORFLOW_SESSION_H_
