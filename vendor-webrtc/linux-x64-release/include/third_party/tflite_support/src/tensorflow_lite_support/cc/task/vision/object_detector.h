/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_OBJECT_DETECTOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_OBJECT_DETECTOR_H_

#include <memory>

#include "absl/container/flat_hash_set.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/external_file_handler.h"
#include "tensorflow_lite_support/cc/task/vision/core/base_vision_task_api.h"
#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"
#include "tensorflow_lite_support/cc/task/vision/core/label_map_item.h"
#include "tensorflow_lite_support/cc/task/vision/proto/detections_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/proto/object_detector_options_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/utils/score_calibration.h"

namespace tflite {
namespace task {
namespace vision {

// Performs object detection on images.
//
// The API expects a TFLite model with mandatory TFLite Model Metadata.
//
// Input tensor:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - image input of size `[batch x height x width x channels]`.
//    - batch inference is not supported (`batch` is required to be 1).
//    - only RGB inputs are supported (`channels` is required to be 3).
//    - if type is kTfLiteFloat32, NormalizationOptions are required to be
//      attached to the metadata for input normalization.
// Output tensors must be the 4 outputs of a `DetectionPostProcess` op, i.e:
//  (kTfLiteFloat32)
//   - locations tensor of size `[num_results x 4]`, the inner array
//     representing bounding boxes in the form [top, left, right, bottom].
//   - BoundingBoxProperties are required to be attached to the metadata
//     and must specify type=BOUNDARIES and coordinate_type=RATIO.
//  (kTfLiteFloat32)
//   - classes tensor of size `[num_results]`, each value representing the
//     integer index of a class.
//    - optional (but recommended) label map(s) can be attached as
//      AssociatedFile-s with type TENSOR_VALUE_LABELS, containing one label per
//      line. The first such AssociatedFile (if any) is used to fill the
//      `class_name` field of the results. The `display_name` field is filled
//      from the AssociatedFile (if any) whose locale matches the
//      `display_names_locale` field of the `ObjectDetectorOptions` used at
//      creation time ("en" by default, i.e. English). If none of these are
//      available, only the `index` field of the results will be filled.
//  (kTfLiteFloat32)
//   - scores tensor of size `[num_results]`, each value representing the score
//     of the detected object.
//  (kTfLiteFloat32)
//   - integer num_results as a tensor of size `[1]`
//
// An example of such model can be found at:
// https://tfhub.dev/google/lite-model/object_detection/mobile_object_localizer_v1/1/metadata/1
//
// A CLI demo tool is available for easily trying out this API, and provides
// example usage. See:
// examples/task/vision/desktop/object_detector_demo.cc
class ObjectDetector : public BaseVisionTaskApi<DetectionResult> {
 public:
  using BaseVisionTaskApi::BaseVisionTaskApi;

  // Creates an ObjectDetector from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<ObjectDetector>>
  CreateFromOptions(
      const ObjectDetectorOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs actual detection on the provided FrameBuffer.
  //
  // The FrameBuffer can be of any size and any of the supported formats, i.e.
  // RGBA, RGB, NV12, NV21, YV12, YV21. It is automatically pre-processed
  // before inference in order to (and in this order):
  // - resize it (with bilinear interpolation, aspect-ratio *not* preserved) to
  //   the dimensions of the model input tensor,
  // - convert it to the colorspace of the input tensor (i.e. RGB, which is the
  //   only supported colorspace for now),
  // - rotate it according to its `Orientation` so that inference is performed
  //   on an "upright" image.
  //
  // IMPORTANT: the returned bounding boxes are expressed in the unrotated input
  // frame of reference coordinates system, i.e. in `[0, frame_buffer.width) x
  // [0, frame_buffer.height)`, which are the dimensions of the underlying
  // `frame_buffer` data before any `Orientation` flag gets applied.
  //
  // In particular, this implies that the returned bounding boxes may not be
  // directly suitable for display if the input image is displayed *with* the
  // `Orientation` flag taken into account according to the EXIF specification
  // (http://jpegclub.org/exif_orientation.html): it may first need to be
  // rotated. This is typically true when consuming camera frames on Android or
  // iOS.
  //
  // For example, if the input `frame_buffer` has its `Orientation` flag set to
  // `kLeftBottom` (i.e. the image will be rotated 90° clockwise during
  // preprocessing to make it "upright"), then the same 90° clockwise rotation
  // needs to be applied to the bounding box for display.
  tflite::support::StatusOr<DetectionResult> Detect(
      const FrameBuffer& frame_buffer);

 protected:
  // Post-processing to transform the raw model outputs into detection results.
  tflite::support::StatusOr<DetectionResult> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const FrameBuffer& frame_buffer, const BoundingBox& roi) override;

  // Performs sanity checks on the provided ObjectDetectorOptions.
  static absl::Status SanityCheckOptions(const ObjectDetectorOptions& options);

  // Initializes the ObjectDetector from the provided ObjectDetectorOptions,
  // whose ownership is transferred to this object.
  absl::Status Init(std::unique_ptr<ObjectDetectorOptions>);

  // Performs pre-initialization actions.
  virtual absl::Status PreInit();

  // Performs post-initialization actions.
  virtual absl::Status PostInit();

 private:
  // Performs sanity checks on the model outputs and extracts their metadata.
  absl::Status CheckAndSetOutputs();

  // Performs sanity checks on the class whitelist/blacklist and forms the class
  // index set.
  absl::Status CheckAndSetClassIndexSet();

  // Checks if the class at the provided index is allowed, i.e. whitelisted in
  // case a whitelist is provided or not blacklisted if a blacklist is provided.
  // Always returns true if no whitelist or blacklist were provided.
  bool IsClassIndexAllowed(int class_index);

  // Initializes the score calibration parameters based on corresponding TFLite
  // Model Metadata, if any.
  absl::Status InitScoreCalibrations();

  // Given a DetectionResult object containing class indices, fills the name and
  // display name from the label map.
  absl::Status FillResultsFromLabelMap(DetectionResult* result);

  // The options used to build this ObjectDetector.
  std::unique_ptr<ObjectDetectorOptions> options_;

  // This is populated by reading the label files from the TFLite Model
  // Metadata: if no such files are available, this is left empty and the
  // ObjectDetector will only be able to populate the `index` field of the
  // detection results `classes` field.
  std::vector<LabelMapItem> label_map_;

  // For each pack of 4 coordinates returned by the model, this denotes the
  // order in which to get the left, top, right and bottom coordinates.
  std::vector<unsigned int> bounding_box_corners_order_;

  // Set of whitelisted or blacklisted class indices.
  struct ClassIndexSet {
    absl::flat_hash_set<int> values;
    bool is_whitelist;
  };
  // Whitelisted or blacklisted class indices based on provided options at
  // construction time. These are used to filter out results during
  // post-processing.
  ClassIndexSet class_index_set_;

  // Score threshold. Detections with a confidence below this value are
  // discarded. If none is provided via metadata or options, -FLT_MAX is set as
  // default value.
  float score_threshold_;

  // List of score calibration parameters, if any. Built from TFLite Model
  // Metadata.
  std::unique_ptr<ScoreCalibration> score_calibration_;

  // Indices of the output tensors to match the output tensors to the correct
  // index order of the output tensors: [location, categories, scores,
  // num_detections].
  std::vector<int> output_indices_;
};

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_OBJECT_DETECTOR_H_
