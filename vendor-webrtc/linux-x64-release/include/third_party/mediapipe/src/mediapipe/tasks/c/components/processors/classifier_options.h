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

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_PROCESSORS_CLASSIFIER_OPTIONS_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_PROCESSORS_CLASSIFIER_OPTIONS_H_

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Classifier options for MediaPipe C classification Tasks.
struct ClassifierOptions {
  // The locale to use for display names specified through the TFLite Model
  // Metadata, if any. Defaults to English.
  const char* display_names_locale;

  // The maximum number of top-scored classification results to return. If < 0,
  // all available results will be returned. If 0, an invalid argument error is
  // returned.
  int max_results;

  // Score threshold to override the one provided in the model metadata (if
  // any). Results below this value are rejected.
  float score_threshold;

  // The allowlist of category names. If non-empty, detection results whose
  // category name is not in this set will be filtered out. Duplicate or unknown
  // category names are ignored. Mutually exclusive with category_denylist.
  const char** category_allowlist;
  // The number of elements in the category allowlist.
  uint32_t category_allowlist_count;

  // The denylist of category names. If non-empty, detection results whose
  // category name is in this set will be filtered out. Duplicate or unknown
  // category names are ignored. Mutually exclusive with category_allowlist.
  const char** category_denylist;
  // The number of elements in the category denylist.
  uint32_t category_denylist_count;
};

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_PROCESSORS_CLASSIFIER_OPTIONS_H_
