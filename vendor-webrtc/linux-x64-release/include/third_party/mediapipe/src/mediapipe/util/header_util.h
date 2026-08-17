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

#ifndef MEDIAPIPE_UTIL_HEADER_UTIL_H_
#define MEDIAPIPE_UTIL_HEADER_UTIL_H_

#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// Copies headers from |inputs| into |outputs| respectively. The size of
// |inputs| and |outputs| must be equal.
absl::Status CopyInputHeadersToOutputs(const InputStreamSet& inputs,
                                       const OutputStreamSet& outputs);

absl::Status CopyInputHeadersToOutputs(const InputStreamShardSet& inputs,
                                       OutputStreamShardSet* outputs);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_HEADER_UTIL_H_
