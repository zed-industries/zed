/* Copyright 2022 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/
#ifndef MEDIAPIPE_TASKS_CC_METADATA_METADATA_VERSION_H_
#define MEDIAPIPE_TASKS_CC_METADATA_METADATA_VERSION_H_

#include <stddef.h>
#include <stdint.h>

#include <string>

#include "tensorflow/lite/c/common.h"

namespace mediapipe {
namespace tasks {
namespace metadata {

// Gets the minimum metadata parser version that can fully understand all fields
// in a given metadata flatbuffer. TFLite Metadata follows Semantic Versioning
// 2.0. Each release version has the form MAJOR.MINOR.PATCH.
TfLiteStatus GetMinimumMetadataParserVersion(const uint8_t* buffer_data,
                                             size_t buffer_size,
                                             std::string* min_version);

}  // namespace metadata
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_METADATA_METADATA_VERSION_H_
