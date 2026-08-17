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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_OBJECT_DETECTOR_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_OBJECT_DETECTOR_H_

#include <stdint.h>

#include "tensorflow_lite_support/c/common.h"
#include "tensorflow_lite_support/c/task/core/base_options.h"
#include "tensorflow_lite_support/c/task/processor/classification_options.h"
#include "tensorflow_lite_support/c/task/processor/detection_result.h"
#include "tensorflow_lite_support/c/task/vision/core/frame_buffer.h"

// --------------------------------------------------------------------------
/// C API for Object Detector.
///
/// The API leans towards simplicity and uniformity instead of convenience, as
/// most usage will be by language-specific wrappers. It provides largely the
/// same set of functionality as that of the C++ TensorFlow Lite
/// `ObjectDetector` API, but is useful for shared libraries where having
/// a stable ABI boundary is important.
///
/// Usage:
/// <pre><code>
/// // Create the model
/// Using the options initialized with default values returned by
/// TfLiteObjectDetectorOptionsCreate() makes sure that there will be no
/// undefined behaviour due to garbage values in unitialized members.
/// TfLiteObjectDetectorOptions options = TfLiteObjectDetectorOptionsCreate();
///
/// Set the model file path in options
///   options.base_options.model_file.file_path = "/path/to/model.tflite";
///
/// If need be, set values for any options to customize behaviour.
/// options.base_options.compute_settings.cpu_settings.num_threads = 3
///
/// Create TfLiteObjectDetector using the options:
/// If error information is not nedded in case of failure:
/// TfLiteObjectDetector* object_detector =
///       TfLiteObjectDetectorFromOptions(&options, NULL);
///
/// If error information is nedded in case of failure:
/// TfLiteSupportError* create_error = NULL;
/// TfLiteObjectDetector* object_detector =
///       TfLiteObjectDetectorFromOptions(&options, &create_error);
///
/// if (!object_detector) {
///   Handle failure.
///   Do something with `create_error`, if requested as illustrated above.
/// }
///
/// Dispose of the create_error object.
/// TfLiteSupportErrorDelete(create_error);
///
/// Classify an image
/// TfLiteFrameBuffer frame_buffer = { Initialize with image data }
///
/// If error information is not nedded in case of failure:
/// TfLiteDetectionResult* detection_result =
///       TfLiteObjectDetectorClassify(object_detector, &frame_buffer, NULL);
///
/// If error information is needed in case of failure:
/// TfLiteSupportError* detect_error = NULL;
/// TfLiteDetectionResult* detection_result =
///       TfLiteObjectDetectorDetect(object_detector, &frame_buffer,
///       &detect_error);
///
/// if (!detection_result) {
///   Handle failure.
///   Do something with `detection_error`, if requested as illustrated above.
/// }
///
/// Dispose of the detection_error object.
/// TfLiteSupportErrorDelete(detection_error);
///
/// Dispose of the API object.
/// TfLiteObjectDetectorOptionsDelete(object_detector);

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct TfLiteObjectDetector TfLiteObjectDetector;

typedef struct TfLiteObjectDetectorOptions {
  TfLiteClassificationOptions classification_options;
  TfLiteBaseOptions base_options;
} TfLiteObjectDetectorOptions;

// Creates and returns TfLiteObjectDetectorOptions initialized with default
// values. Default values are as follows:
// 1. .classification_options.max_results = -1, which returns all classification
// categories by default.
// 2. .base_options.compute_settings.tflite_settings.cpu_settings.num_threads =
// -1, which makes the TFLite runtime choose the value.
// 3. .classification_options.score_threshold = 0
// 4. All pointers like .base_options.model_file.file_path,
// .base_options.classification_options.display_names_local,
// .classification_options.label_allowlist.list,
// options.classification_options.label_denylist.list are NULL.
// 5. All other integer values are initialized to 0.
TfLiteObjectDetectorOptions TfLiteObjectDetectorOptionsCreate(void);

// Creates TfLiteObjectDetector from options.
// .base_options.model_file.file_path in TfLiteObjectDetectorOptions should be
// set to the path of the tflite model you wish to create the
// TfLiteObjectDetector with.
// Create TfLiteObjectDetectorOptions using
// TfLiteObjectDetectorOptionsCreate(). If need be, you can change the default
// values of options for customizing classification, If options are not created
// in the aforementioned way, you have to make sure that all members are
// initialized to respective default values and all pointer members are zero
// initialized to avoid any undefined behaviour.
//
// Returns the created object detector in case of success.
// Returns nullptr on failure which happens commonly due to one of the following
// issues:
// 1. file doesn't exist or is not a well formatted.
// 2. options is nullptr.
// 3. Both options.classification_options.label_denylist and
// options.classification_options.label_allowlist are non empty. These
// fields are mutually exclusive.
//
// The caller can check if an error was encountered by testing if the returned
// value of the function is null. If the caller doesn't want the reason for
// failure, they can simply pass a NULL for the address of the error pointer as
// shown below:
//
// TfLiteObjectDetector* detector = TfLiteObjectDetectorFromOptions(options,
// NULL);
//
// If the caller wants to be informed of the reason for failure, they must pass
// the adress of a pointer of type TfLiteSupportError to the `error` param as
// shown below:
//
// TfLiteSupport *error = NULL:
// TfLiteObjectDetector* classifier = TfLiteObjectDetectorFromOptions(options,
// &error);
//
// In case of unsuccessful execution, Once the function returns, the error
// pointer will point to a struct containing the error information. If error
// info is passed back to the caller, it is the responsibility of the caller to
// free the error struct by calling the following method defined in common.h:
//
// TfLiteSupportErrorDelete(error)
//
TfLiteObjectDetector* TfLiteObjectDetectorFromOptions(
    const TfLiteObjectDetectorOptions* options, TfLiteSupportError** error);

// Invokes the encapsulated TFLite model and performs object detection on the
// frame_buffer. Returns a pointer to the created object detection result result
// in case of success or NULL in case of failure. The caller must test the
// return value to identify success or failure. If the caller doesn't want the
// reason for failure, they can simply pass a NULL for the address of the error
// pointer as shown below:
//
// TfLiteDetectionResult* detection_result =
// TfLiteObjectDetectorDetect(&options, NULL);
//
// If the caller wants to be informed of the reason for failure, they must pass
// the adress of a pointer of type TfLiteSupportError to the `error` param as
// shown below:
//
// TfLiteSupport *error = NULL:
// TfLiteObjectDetector* detector = TfLiteObjectDetectorFromOptions(options,
// &error);
//
// In case of unsuccessful execution, Once the function returns, the error
// pointer will point to a struct containing the error information. If error
// info is passed back to the caller, it is the responsibility of the caller to
// free the error struct by calling the following method defined in common.h:
//
// TfLiteSupportErrorDelete(error)
//
TfLiteDetectionResult* TfLiteObjectDetectorDetect(
    const TfLiteObjectDetector* detector, const TfLiteFrameBuffer* frame_buffer,
    TfLiteSupportError** error);

// Disposes off the object detector.
void TfLiteObjectDetectorDelete(TfLiteObjectDetector* detector);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_OBJECT_DETECTOR_H_
