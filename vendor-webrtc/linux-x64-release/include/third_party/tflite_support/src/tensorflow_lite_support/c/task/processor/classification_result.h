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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_CLASSIFICATION_RESULT_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_CLASSIFICATION_RESULT_H_

#include "tensorflow_lite_support/c/task/processor/category.h"

// Defines C structure for Classification Results and associated helper methods.

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// List of predicted classes (aka labels) for a given image classifier head.
typedef struct TfLiteClassifications {
  // The index of the image classifier head these classes refer to. This is
  // useful for multi-head models.
  int head_index;

  // The name of the classifier head, which is the corresponding tensor metadata
  // name. See
  // https://github.com/tensorflow/tflite-support/blob/710e323265bfb71fdbdd72b3516e00cff15c0326/tensorflow_lite_support/metadata/metadata_schema.fbs#L545
  // This will always be NULL for vision APIs.
  char* head_name;

  // Number of predicted classes which can be used to traverse the array of
  // predicted classes.
  int size;

  // The array of predicted classes, usually sorted by descending scores (e.g.
  // from high to low probability). Since this array is dynamically allocated,
  // use size to traverse through the array.
  TfLiteCategory* categories;
} TfLiteClassifications;

// Holds Image Classification results.
// Contains one set of results per image classifier head.
typedef struct TfLiteClassificationResult {
  // Number of predicted classes which can be used to traverse the array of
  // predicted classes.
  int size;

  // Array of image classifier results per image classifier head. This array can
  // have any number of results. size holds the size of this array. size should
  // be used to traverse this array.
  TfLiteClassifications* classifications;
} TfLiteClassificationResult;

// Frees up the ClassificationResult Structure.
void TfLiteClassificationResultDelete(
    TfLiteClassificationResult* classification_result);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_CLASSIFICATION_RESULT_H_
