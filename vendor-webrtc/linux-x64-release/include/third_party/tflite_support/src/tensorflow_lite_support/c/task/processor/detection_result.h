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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_DETECTION_RESULT_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_DETECTION_RESULT_H_

#include "tensorflow_lite_support/c/task/processor/bounding_box.h"
#include "tensorflow_lite_support/c/task/processor/category.h"

// Defines C structure for Object Detection Results and associated helper
// methods.

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// Bounding box and list of predicted classes (aka labels) for a detected
// object.
typedef struct TfLiteDetection {
  // The bounding box of the detected object.
  TfLiteBoundingBox bounding_box;

  // The array of predicted classes for the object detection represented by an
  // instance of TfLiteDetection, usually sorted by descending scores (e.g. from
  // high to low probability). Since this array is dynamically allocated, use
  // size to traverse through the array.
  TfLiteCategory* categories;

  // Number of detectd objects be used to traverse the array of the detected
  // objects.
  int size;
} TfLiteDetection;

// Holds Object Detection results.
// Contains one set of results per detected object.
typedef struct TfLiteDetectionResult {
  // Number of detectd objects be used to traverse the array of the detected
  // objects.
  int size;

  // Array of results per detected object. This array can
  // have any number of results. size holds the size of this array. size should
  // be used to traverse this array.
  TfLiteDetection* detections;
} TfLiteDetectionResult;

// Frees up the DetectionResult Structure.
void TfLiteDetectionResultDelete(TfLiteDetectionResult* detection_result);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_DETECTION_H_
