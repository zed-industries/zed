/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_CLASSIFICATION_OPTIONS_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_CLASSIFICATION_OPTIONS_H_

#include <stdint.h>

// Defines C Struct for Classification Options Shared by All Classification
// Tasks.

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// Holds pointer to array of C strings and length for looping through the array.
typedef struct TfLiteStringArrayOption {
  // Length of list. length can be used to loop through list.
  int length;

  // Array of C strings.
  char** list;
} TfLiteStringArrayOption;

// Holds settings for any single classification task.
typedef struct TfLiteClassificationOptions {
  // Optional denylist of class labels. If non NULL, classifications whose
  // class label is in this set will be filtered out. Duplicate or unknown
  // class labels are ignored. Mutually exclusive with label_allowlist.
  TfLiteStringArrayOption label_denylist;

  // Optional allowlist of class labels. If non-empty, classifications whose
  // class label is not in this set will be filtered out. Duplicate or unknown
  // class labels are ignored. Mutually exclusive with label_denylist.
  TfLiteStringArrayOption label_allowlist;

  // The locale to use for display names specified through the TFLite Model
  // Metadata, if any. Defaults to English.
  char* display_names_local;

  // The maximum number of top-scored classification results to return. If < 0,
  // all available results will be returned. If 0, an invalid argument error is
  // returned. Defaults to -1.
  int max_results;

  // Score threshold, overrides the ones provided in the model metadata
  // (if any). Results below this value are rejected.
  float score_threshold;
} TfLiteClassificationOptions;

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_CLASSIFICATION_OPTIONS_H_
