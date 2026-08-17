// Copyright 2022 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_UTIL_LABEL_MAP_UTIL_H_
#define MEDIAPIPE_UTIL_LABEL_MAP_UTIL_H_

#include <cstdint>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/util/label_map.pb.h"

namespace mediapipe {

// Builds a label map from labels and (optional) display names file contents,
// both expected to contain one label per line.
// Returns an error e.g. if there's a mismatch between the number of labels and
// display names.
absl::StatusOr<mediapipe::proto_ns::Map<int64_t, mediapipe::LabelMapItem>>
BuildLabelMapFromFiles(absl::string_view labels_file_contents,
                       absl::string_view display_names_file_contents);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_LABEL_MAP_UTIL_H_
