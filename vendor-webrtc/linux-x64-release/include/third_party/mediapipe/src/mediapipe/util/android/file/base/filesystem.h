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

#ifndef MEDIAPIPE_ANDROID_FILE_BASE_FILESYSTEM_H_
#define MEDIAPIPE_ANDROID_FILE_BASE_FILESYSTEM_H_

#include "absl/strings/string_view.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/util/android/file/base/file.h"

namespace mediapipe {
namespace file {

absl::Status RecursivelyCreateDir(absl::string_view path,
                                  const file::Options& options);

absl::Status Exists(absl::string_view path, const file::Options& options);

absl::Status IsDirectory(absl::string_view path, const file::Options& options);

}  // namespace file.
}  // namespace mediapipe

#endif  // MEDIAPIPE_ANDROID_FILE_BASE_FILESYSTEM_H_
