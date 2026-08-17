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

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_CLASSIFICATION_RESULT_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_CLASSIFICATION_RESULT_H_

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Defines classification results for a given classifier head.
struct Classifications {
  // The array of predicted categories, usually sorted by descending scores,
  // e.g. from high to low probability.
  struct Category* categories;
  // The number of elements in the categories array.
  uint32_t categories_count;

  // The index of the classifier head (i.e. output tensor) these categories
  // refer to. This is useful for multi-head models.
  int head_index;

  // The optional name of the classifier head, as provided in the TFLite Model
  // Metadata [1] if present. This is useful for multi-head models.
  //
  // [1]: https://www.tensorflow.org/lite/convert/metadata
  char* head_name;
};

// Defines classification results of a model.
struct ClassificationResult {
  // The classification results for each head of the model.
  struct Classifications* classifications;
  // The number of classifications in the classifications array.
  uint32_t classifications_count;

  // The optional timestamp (in milliseconds) of the start of the chunk of data
  // corresponding to these results.
  //
  // This is only used for classification on time series (e.g. audio
  // classification). In these use cases, the amount of data to process might
  // exceed the maximum size that the model can process: to solve this, the
  // input data is split into multiple chunks starting at different timestamps.
  int64_t timestamp_ms;
  // Specifies whether the timestamp contains a valid value.
  bool has_timestamp_ms;
};

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_CLASSIFICATION_RESULT_H_
