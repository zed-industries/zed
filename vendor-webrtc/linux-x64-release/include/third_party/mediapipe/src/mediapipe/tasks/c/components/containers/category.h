/* Copyright 2023 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_CATEGORY_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_CATEGORY_H_

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Defines a single classification result.
//
// The label maps packed into the TFLite Model Metadata [1] are used to populate
// the 'category_name' and 'display_name' fields.
//
// [1]: https://www.tensorflow.org/lite/convert/metadata
struct Category {
  // The index of the category in the classification model output.
  int index;

  // The score for this category, e.g. (but not necessarily) a probability in
  // [0,1].
  float score;

  // The optional ID for the category, read from the label map packed in the
  // TFLite Model Metadata if present. Not necessarily human-readable.
  char* category_name;

  // The optional human-readable name for the category, read from the label map
  // packed in the TFLite Model Metadata if present.
  char* display_name;
};

// A list of categories.
struct Categories {
  struct Category* categories;
  uint32_t categories_count;
};

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_CATEGORY_H_
