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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_SEGMENTATION_RESULT_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_PROCESSOR_SEGMENTATION_RESULT_H_

#include <stdint.h>

// Defines C structure for Image Segmentation Results and associated helper
// methods.

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// Holds a label associated with an RGB color, for display purposes.
typedef struct TfLiteColoredLabel {
  // The RGB color components for the label, in the [0, 255] range.
  // Note uint32_t to keep it consistent with underlying C++ segmentations
  // proto.
  uint32_t r;
  uint32_t g;
  uint32_t b;

  // The class name, as provided in the label map packed in the TFLite Model
  // Metadata.
  char* label;

  // The display name, as provided in the label map (if available) packed in
  // the TFLite Model Metadata. See `display_names_locale` field in
  // ImageSegmenterOptions.
  char* display_name;
} TfLiteColoredLabel;

// Holds a resulting segmentation mask and associated metadata.
typedef struct TfLiteSegmentation {
  // The width of the mask. This is an intrinsic parameter of the model being
  // used, and does not depend on the input image dimensions.
  int width;

  // The height of the mask. This is an intrinsic parameter of the model being
  // used, and does not depend on the input image dimensions.
  int height;

  // IMPORTANT: A TfLiteSegmentation can either have `confidence_masks`
  // or `category_mask` based on the output type selected in
  // `TfLiteSegmentationOptions`i.e, they are mutually exclusive.
  // Whichever field amongst the two is not applicable based on the selected
  // output type will be null.

  // IMPORTANT: segmentation masks are not direcly suited for display, in
  // particular:
  // * they are relative to the unrotated input frame, i.e. *not* taking into
  //   account the `Orientation` flag of the input FrameBuffer,
  // * their dimensions are intrinsic to the model, i.e. *not* dependent on the
  //   input FrameBuffer dimensions.

  // One confidence masks of size `width` x `height` for each of the supported
  // classes. The value of each pixel in these masks represents the confidence
  // score for this particular class.
  float** confidence_masks;

  // Flattened 2D-array of size `width` x `height`,
  // in row major order. The value of each pixel in this mask represents the
  // class to which the pixel belongs.
  uint8_t* category_mask;

  // Number of colored labels which is equivalent to number of classes
  // supported by the model.
  int colored_labels_size;

  // The list of colored labels for all the supported categories (classes).
  // Depending on which is present, this list is in 1:1 correspondence with:
  // * `category_mask` pixel values, i.e. a pixel with value `i` is
  //   associated with `colored_labels[i]`,
  // * `confidence_masks` indices, i.e. `confidence_masks[i]` is associated with
  //   `colored_labels[i]`.
  TfLiteColoredLabel* colored_labels;
} TfLiteSegmentation;

// Holds Image Segmentation Results.
// Contains one set of results per detected object.
typedef struct TfLiteSegmentationResult {
  // Number of segmentations be used to traverse the array of segmentations.
  int size;

  // Array of seegmentations returned after inference by model.
  // Note that at the time, this array is expected to have a single
  // `TfLiteSegmentation`; the field is made an array for later extension to
  // e.g. instance segmentation models, which may return one segmentation per
  // object.
  TfLiteSegmentation* segmentations;
} TfLiteSegmentationResult;

// Frees up the TfLiteSegmentationResult structure.
void TfLiteSegmentationResultDelete(
    TfLiteSegmentationResult* segmentation_result);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_SEGMENTATION_RESULT_H_
