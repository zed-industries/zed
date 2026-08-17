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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_EXECUTOR_UTIL_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_EXECUTOR_UTIL_H_

#include <cstdint>

#include "mediapipe/framework/calculator.pb.h"

namespace mediapipe {

namespace tool {
// Ensures the default executor's stack size is at least min_stack_size.
//
// Note that this will also initialize the default executor; any configuration
// changes, such as num_threads, should be done to the config before calling
// this.
void EnsureMinimumDefaultExecutorStackSize(int32_t min_stack_size,
                                           CalculatorGraphConfig* config);
}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_EXECUTOR_UTIL_H_
