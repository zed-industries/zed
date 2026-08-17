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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_TAG_MAP_HELPER_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_TAG_MAP_HELPER_H_

#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {
namespace tool {

// Create a TagMap using a vector of TAG:<index>:name.
absl::StatusOr<std::shared_ptr<TagMap>> CreateTagMap(
    const std::vector<std::string>& tag_index_names);

// Create a TagMap using an integer number of entries (for tag "").
absl::StatusOr<std::shared_ptr<TagMap>> CreateTagMap(int num_entries);

// Create a TagMap using a vector of just tag names.
absl::StatusOr<std::shared_ptr<TagMap>> CreateTagMapFromTags(
    const std::vector<std::string>& tags);

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_TAG_MAP_HELPER_H_
