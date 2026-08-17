/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_IMAGE_SEGMENTER_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_IMAGE_SEGMENTER_H_

#include <stdint.h>

#include "tensorflow_lite_support/c/common.h"
#include "tensorflow_lite_support/c/task/core/base_options.h"
#include "tensorflow_lite_support/c/task/processor/segmentation_result.h"
#include "tensorflow_lite_support/c/task/vision/core/frame_buffer.h"

// --------------------------------------------------------------------------
/// C API for ImageSegmenter.
///
/// The API leans towards simplicity and uniformity instead of convenience, as
/// most usage will be by language-specific wrappers. It provides largely the
/// same set of functionality as that of the C++ TensorFlow Lite
/// `ImageSegmenter` API, but is useful for shared libraries where having
/// a stable ABI boundary is important.
///
/// Usage:
/// <pre><code>
/// // Create the model
/// Using the options initialized with default values returned by
/// TfLiteImageSegmenterOptionsCreate() makes sure that there will be no
/// undefined behaviour due to garbage values in unitialized members.
/// TfLiteImageSegmenterOptions options = TfLiteImageSegmenterOptionsCreate();
///
/// Set the model file path in options
/// options.base_options.model_file.file_path = "/path/to/model.tflite";
///
/// If need be, set values for any options to customize behaviour.
/// options.base_options.compute_settings.cpu_settings.num_threads = 3
///
/// Create TfLiteImageSegmenter using the options:
/// If error information is not needed in case of failure:
/// TfLiteImageSegmenter* image_segmenter =
///       TfLiteImageSegmenterFromOptions(&options, NULL);
///
/// If error information is needed in case of failure:
/// TfLiteSupportError* create_error = NULL;
/// TfLiteImageSegmenter* image_segmenter =
///       TTfLiteImageSegmenterFromOptions(&options, &create_error);
///
/// if (!image_segmenter) {
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
/// If error information is not needed in case of failure:
/// TfLiteSegmentationResult* segmentation_result =
///       TfLiteImageSegmenterSegment(image_segmenter, &frame_buffer, NULL);
///
/// If error information is needed in case of failure:
/// TfLiteSupportError* segment_error = NULL;
/// TfLiteSegmentationResult* segmentation_result =
///       TfLiteImageSegmenterSegment(image_segmenter, &frame_buffer,
///       &segment_error);
///
/// if (!segmentation_result) {
///   Handle failure.
///   Do something with `segment_error`, if requested as illustrated above.
/// }
///
/// Dispose of the segment_error object.
/// TfLiteSupportErrorDelete(segment_error);
///
/// Dispose of the API object.
/// TfLiteImageClassifierDelete(image_segmenter);

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct TfLiteImageSegmenter TfLiteImageSegmenter;

// Specifies the type of output segmentation mask to be returned
// as a result of the image segmentation operation.
// This allows specifying the type of post-processing to
// perform on the raw model results (see TfLiteSegmentationResult for more).
typedef enum TfLiteImageSegmenterOutputType {
  kUnspecified,

  // Gives a single output mask where each pixel represents the class which
  // the pixel in the original image was predicted to belong to.
  kCategoryMask,

  // Gives a list of output masks where, for each mask, each pixel represents
  // the prediction confidence, usually in the [0, 1] range.
  kConfidenceMask
} TfLiteImageSegmenterOutputType;

// Holds options for configuring the creation of TfLiteImageSegmenter.
typedef struct TfLiteImageSegmenterOptions {
  TfLiteBaseOptions base_options;

  // Specifies the type of output segmentation mask to be returned
  // as a result of the image segmentation operation. (See
  // TfLiteImageSegmenterOutputType for more).
  TfLiteImageSegmenterOutputType output_type;

  // The locale to use for display names specified through the TFLite Model
  // Metadata, if any. Defaults to English.
  char* display_names_locale;
} TfLiteImageSegmenterOptions;

// Creates and returns TfLiteImageSegmenterOptions initialized with default
// values. Default values are as follows:
// 1. .base_options.compute_settings.tflite_settings.cpu_settings.num_threads =
// -1, which makes the TFLite runtime choose the value.
// 2. .output_type = kCategoryMask
// 3. display_names_locale is NULL.
TfLiteImageSegmenterOptions TfLiteImageSegmenterOptionsCreate(void);

// Creates TfLiteImageSegmenter from options.
// .base_options.model_file.file_path in TfLiteImageSegmenterOptions should be
// set to the path of the tflite model you wish to create the
// TfLiteImageSegmenter with.
// Create TfLiteImageSegmenterOptions using
// TfLiteImageSegmenterOptionsCreate(). If need be, you can change the default
// values of options for customizing segmentation, If options are not created
// in the aforementioned way, you have to make sure that all members are
// initialized to respective default values and all pointer members are zero
// initialized to avoid any undefined behaviour.
//
// Returns the created image segmenter in case of success.
// Returns nullptr on failure which happens commonly due to one of the following
// issues:
// 1. file doesn't exist or is not a well formatted.
// 2. options is nullptr.
//
// The caller can check if an error was encountered by testing if the returned
// value of the function is null. If the caller doesn't want the reason for
// failure, they can simply pass a NULL for the address of the error pointer as
// shown below:
//
// TfLiteImageSegmenter* segmenter = TfLiteImageSegmenterFromOptions(options,
// NULL);
//
// If the caller wants to be informed of the reason for failure, they must pass
// the address of a pointer of type TfLiteSupportError to the `error` param as
// shown below:
//
// TfLiteSupport *error = NULL:
// TfLiteImageSegmenter* segmenter = TfLiteImageSegmenterFromOptions(options,
// &error);
//
// In case of unsuccessful execution, Once the function returns, the error
// pointer will point to a struct containing the error information. If error
// info is passed back to the caller, it is the responsibility of the caller to
// free the error struct by calling the following method defined in common.h:
//
// TfLiteSupportErrorDelete(error)
//
TfLiteImageSegmenter* TfLiteImageSegmenterFromOptions(
    const TfLiteImageSegmenterOptions* options, TfLiteSupportError** error);

// Invokes the encapsulated TFLite model and performs image segmentation on
// the frame_buffer.
// Returns a pointer to the created segmentation result in case of success or
// NULL in case of failure. The caller must test the return value to identify
// success or failure. If the caller doesn't want the reason for failure, they
// can simply pass a NULL for the address of the error pointer as shown below:
//
// TfLiteSegmentationResult* segmentation_result =
//      TfLiteImageSegmenterSegment(&options, NULL);
//
// If the caller wants to be informed of the reason for failure, they must pass
// the address of a pointer of type TfLiteSupportError to the `error` param as
// shown below:
//
// TfLiteSupport *error = NULL:
// TfLiteSegmentationResult* segmentation_result =
//      TfLiteImageSegmenterSegment(&options, &error);
//
// In case of unsuccessful execution, Once the function returns, the error
// pointer will point to a struct containing the error information. If error
// info is passed back to the caller, it is the responsibility of the caller to
// free the error struct by calling the following method defined in common.h:
//
// TfLiteSupportErrorDelete(error)
//
TfLiteSegmentationResult* TfLiteImageSegmenterSegment(
    const TfLiteImageSegmenter* segmenter,
    const TfLiteFrameBuffer* frame_buffer, TfLiteSupportError** error);

// Disposes of the image segmenter.
void TfLiteImageSegmenterDelete(TfLiteImageSegmenter* segmenter);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_IMAGE_SEGMENTER_H_
