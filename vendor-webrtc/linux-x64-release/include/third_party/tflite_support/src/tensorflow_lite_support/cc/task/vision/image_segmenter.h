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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_SEGMENTER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_SEGMENTER_H_

#include <memory>
#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/external_file_handler.h"
#include "tensorflow_lite_support/cc/task/vision/core/base_vision_task_api.h"
#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"
#include "tensorflow_lite_support/cc/task/vision/core/label_map_item.h"
#include "tensorflow_lite_support/cc/task/vision/proto/bounding_box_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/proto/image_segmenter_options_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/proto/segmentations_proto_inc.h"

namespace tflite {
namespace task {
namespace vision {

// Performs segmentation on images.
//
// The API expects a TFLite model with optional, but strongly recommended,
// TFLite Model Metadata.
//
// Input tensor:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - image input of size `[batch x height x width x channels]`.
//    - batch inference is not supported (`batch` is required to be 1).
//    - only RGB inputs are supported (`channels` is required to be 3).
//    - if type is kTfLiteFloat32, NormalizationOptions are required to be
//      attached to the metadata for input normalization.
// Output tensor:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - tensor of size `[batch x mask_height x mask_width x num_classes]`, where
//      `batch` is required to be 1, `mask_width` and `mask_height` are the
//      dimensions of the segmentation masks produced by the model, and
//      `num_classes` is the number of classes supported by the model.
//    - optional (but recommended) label map(s) can be attached as
//      AssociatedFile-s with type TENSOR_AXIS_LABELS, containing one label per
//      line. The first such AssociatedFile (if any) is used to fill the
//      `class_name` field of the results. The `display_name` field is filled
//      from the AssociatedFile (if any) whose locale matches the
//      `display_names_locale` field of the `ImageSegmenterOptions` used at
//      creation time ("en" by default, i.e. English). If none of these are
//      available, only the `index` field of the results will be filled.
//
// An example of such model can be found at:
// https://tfhub.dev/tensorflow/lite-model/deeplabv3/1/metadata/1
//
// A CLI demo tool is available for easily trying out this API, and provides
// example usage. See:
// examples/task/vision/desktop/image_segmenter_demo.cc
class ImageSegmenter : public BaseVisionTaskApi<SegmentationResult> {
 public:
  using BaseVisionTaskApi::BaseVisionTaskApi;

  // Creates an ImageSegmenter from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<ImageSegmenter>>
  CreateFromOptions(
      const ImageSegmenterOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs actual segmentation on the provided FrameBuffer.
  //
  // The FrameBuffer can be of any size and any of the supported formats, i.e.
  // RGBA, RGB, NV12, NV21, YV12, YV21. It is automatically pre-processed before
  // inference in order to (and in this order):
  // - resize it (with bilinear interpolation, aspect-ratio *not* preserved) to
  //   the dimensions of the model input tensor,
  // - convert it to the colorspace of the input tensor (i.e. RGB, which is the
  //   only supported colorspace for now),
  // - rotate it according to its `Orientation` so that inference is performed
  //   on an "upright" image.
  //
  // IMPORTANT: the returned segmentation masks are not direcly suited for
  // display, in particular:
  // * they are relative to the unrotated input frame, i.e. *not* taking into
  //   account the `Orientation` flag of the input FrameBuffer,
  // * their dimensions are intrinsic to the model, i.e. *not* dependent on the
  //   input FrameBuffer dimensions.
  //
  // Example of such post-processing, assuming:
  // * an input FrameBuffer with width=640, height=480, orientation=kLeftBottom
  //   (i.e. the image will be rotated 90° clockwise during preprocessing to
  //   make it "upright"),
  // * a model outputting masks of size 224x224.
  // In order to be directly displayable on top of the input image assumed to
  // be displayed *with* the `Orientation` flag taken into account according to
  // the EXIF specification (http://jpegclub.org/exif_orientation.html), the
  // masks need to be:
  // * re-scaled to 640 x 480,
  // * then rotated 90° clockwise.
  tflite::support::StatusOr<SegmentationResult> Segment(
      const FrameBuffer& frame_buffer);

 protected:
  // Post-processing to transform the raw model outputs into segmentation
  // results.
  tflite::support::StatusOr<SegmentationResult> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const FrameBuffer& frame_buffer, const BoundingBox& roi) override;

  // Performs sanity checks on the provided ImageSegmenterOptions.
  static absl::Status SanityCheckOptions(const ImageSegmenterOptions& options);

  // Initializes the Segmenter from the provided ImageSegmenterOptions, whose
  // ownership is transferred to this object.
  absl::Status Init(std::unique_ptr<ImageSegmenterOptions> options);

  // Performs pre-initialization actions.
  virtual absl::Status PreInit();

  // The options used for building this image segmenter.
  std::unique_ptr<ImageSegmenterOptions> options_;

  // The label map, extracted from the TFLite Model Metadata.
  std::vector<LabelMapItem> label_map_;

 private:
  // Performs sanity checks on the model outputs and extracts their metadata.
  absl::Status CheckAndSetOutputs();

  // Initializes the colored labels list from `label_map_` and stores it in
  // `colored_labels_`.
  absl::Status InitColoredLabels();

  // Returns the output confidence at coordinates {x, y, depth}, dequantizing
  // on-the-fly if needed (i.e. if `has_uint8_outputs_` is true).
  tflite::support::StatusOr<float> GetOutputConfidence(
      const TfLiteTensor& output_tensor, int x, int y, int depth);

  // Prebuilt list of ColoredLabel attached to each Segmentation result. The
  // i-th item in this list corresponds to the i-th label map item.
  std::vector<Segmentation::ColoredLabel> colored_labels_;

  // Whether the model features quantized inference type (QUANTIZED_UINT8). This
  // is currently detected by checking if all output tensors data type is uint8.
  bool has_uint8_outputs_;

  // Expected output width.
  int output_width_;
  // Expected output height.
  int output_height_;
  // Expected output depth. This corresponds to the number of supported classes.
  int output_depth_;
};

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_SEGMENTER_H_
