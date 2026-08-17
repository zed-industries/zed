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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_IMAGE_PREPROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_IMAGE_PREPROCESSOR_H_

#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/task/processor/processor.h"
#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"
#include "tensorflow_lite_support/cc/task/vision/proto/bounding_box_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/utils/frame_buffer_utils.h"
#include "tensorflow_lite_support/cc/task/vision/utils/image_tensor_specs.h"

namespace tflite {
namespace task {
namespace processor {

// Process input image and populate the associate input tensor.
// Requirement for the input tensor:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - image input of size `[batch x height x width x channels]`.
//    - batch inference is not supported (`batch` is required to be 1).
//    - only RGB inputs are supported (`channels` is required to be 3).
//    - if type is kTfLiteFloat32, NormalizationOptions are required to be
//      attached to the metadata for input normalization.
class ImagePreprocessor : public Preprocessor {
 public:
  static tflite::support::StatusOr<std::unique_ptr<ImagePreprocessor>> Create(
      core::TfLiteEngine* engine,
      const std::initializer_list<int> input_indices,
      const vision::FrameBufferUtils::ProcessEngine& process_engine =
          vision::FrameBufferUtils::ProcessEngine::kLibyuv);

  // Processes the provided FrameBuffer and populate tensor values.
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
  // NOTE: In case the model has dynamic input shape, the method would re-dim
  // the entire graph based on the dimensions of the image.
  absl::Status Preprocess(const vision::FrameBuffer& frame_buffer);

  // Same as above, except based on the input region of interest.
  //
  // IMPORTANT: as a consequence of cropping occurring first, the provided
  // region of interest is expressed in the unrotated frame of reference
  // coordinates system, i.e. in `[0, frame_buffer.width) x [0,
  // frame_buffer.height)`, which are the dimensions of the underlying
  // `frame_buffer` data before any `Orientation` flag gets applied. Also, the
  // region of interest is not clamped, so this method will return a non-ok
  // status if the region is out of these bounds.
  absl::Status Preprocess(const vision::FrameBuffer& frame_buffer,
                          const vision::BoundingBox& roi);

  // Returns the spec of model. Passing in an image with this spec will speed up
  // the inference as it bypasses image cropping and resizing.
  const vision::ImageTensorSpecs& GetInputSpecs() const { return input_specs_; }

 private:
  using Preprocessor::Preprocessor;

  // Returns false if image preprocessing could be skipped, true otherwise.
  bool IsImagePreprocessingNeeded(const vision::FrameBuffer& frame_buffer,
                                  const vision::BoundingBox& roi);

  absl::Status Init(
      const vision::FrameBufferUtils::ProcessEngine& process_engine);

  // Parameters related to the input tensor which represents an image.
  vision::ImageTensorSpecs input_specs_;

  // Utils for input image preprocessing (resizing, colorspace conversion, etc).
  std::unique_ptr<vision::FrameBufferUtils> frame_buffer_utils_;

  // Is true if the model expects dynamic image shape, false otherwise.
  bool is_height_mutable_ = false;
  bool is_width_mutable_ = false;
};

}  // namespace processor
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_IMAGE_PREPROCESSOR_H_
