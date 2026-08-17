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

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_PROFILER_RESOURCE_UTIL_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_PROFILER_RESOURCE_UTIL_H_

#include <string>

#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/statusor.h"

namespace mediapipe {

// Returns the path to the directory where trace logs will be stored by default.
// If the function is unable to find an appropriate directory, it returns an
// error.
StatusOr<std::string> GetDefaultTraceLogDirectory();

// Given a log file path, this function provides an absolute path with which
// it can be accessed as a file.  Enclosing directories are created as needed.
StatusOr<std::string> PathToLogFile(const std::string& path);

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_PROFILER_RESOURCE_UTIL_H_
