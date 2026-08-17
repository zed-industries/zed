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

#ifndef MEDIAPIPE_ANDROID_FILE_BASE_HELPERS_H_
#define MEDIAPIPE_ANDROID_FILE_BASE_HELPERS_H_

#include <string>

#include "absl/strings/string_view.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/util/android/file/base/file.h"

namespace mediapipe {
namespace file {

// Read contents of a file to a string.
absl::Status GetContents(absl::string_view file_name, std::string* output,
                         const file::Options& options);

// Read contents of a file to a string with default file options.
absl::Status GetContents(absl::string_view file_name, std::string* output);

// Read contents of a file to a string from an open file descriptor.
absl::Status GetContents(int fd, std::string* output);

// Write string to file.
absl::Status SetContents(absl::string_view file_name, absl::string_view content,
                         const file::Options& options);

// Write string to file with default file options.
absl::Status SetContents(absl::string_view file_name,
                         absl::string_view content);

}  // namespace file
}  // namespace mediapipe

#endif  // MEDIAPIPE_ANDROID_FILE_BASE_HELPERS_H_
