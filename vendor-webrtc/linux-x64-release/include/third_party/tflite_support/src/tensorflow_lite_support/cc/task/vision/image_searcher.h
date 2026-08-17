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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_SEARCHER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_SEARCHER_H_

#include <memory>
#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/processor/proto/search_result.pb.h"
#include "tensorflow_lite_support/cc/task/processor/search_postprocessor.h"
#include "tensorflow_lite_support/cc/task/vision/core/base_vision_task_api.h"
#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"
#include "tensorflow_lite_support/cc/task/vision/proto/bounding_box_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/proto/image_searcher_options.pb.h"

namespace tflite {
namespace task {
namespace vision {

// Performs embedding extraction on images, followed by nearest-neighbor search
// in an index of embeddings through ScaNN.
// TODO(b/223535177): add pointer to README in the scann folder once available.
//
// The API expects a TFLite embedder model with optional, but strongly
// recommended, TFLite Model Metadata.
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
//    - `N` components corresponding to the `N` dimensions of the returned
//      feature vector for this output layer.
//    - Either 2 or 4 dimensions, i.e. `[1 x N]` or `[1 x 1 x 1 x N]`.
//
// TODO(b/180502532): add pointer to example model.
//
// A CLI demo tool is available for easily trying out this API, and provides
// example usage. See:
// examples/task/vision/desktop/image_searcher_demo.cc
class ImageSearcher
    : public BaseVisionTaskApi<tflite::task::processor::SearchResult> {
 public:
  using BaseVisionTaskApi::BaseVisionTaskApi;

  // Creates an ImageSearcher from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<ImageSearcher>>
  CreateFromOptions(
      const ImageSearcherOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs embedding extraction on the provided FrameBuffer, followed by
  // nearest-neighbor search in the index.
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
  tflite::support::StatusOr<tflite::task::processor::SearchResult> Search(
      const FrameBuffer& frame_buffer);

  // Same as above, except the inference is performed only on the provided
  // region of interest. Note that the region of interest is not clamped, so
  // this method will fail if the region is out of bounds of the input image.
  tflite::support::StatusOr<tflite::task::processor::SearchResult> Search(
      const FrameBuffer& frame_buffer, const BoundingBox& roi);

  // Provides access to the opaque user info stored in the index file (if any),
  // in raw binary form. Returns an empty string if the index doesn't contain
  // user info.
  tflite::support::StatusOr<absl::string_view> GetUserInfo();

 protected:
  // The options used to build this ImageSearcher.
  std::unique_ptr<ImageSearcherOptions> options_;

  // Post-processing to transform the raw model outputs into embeddings, then
  // perform the nearest-neighbor search in the index.
  tflite::support::StatusOr<tflite::task::processor::SearchResult> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const FrameBuffer& frame_buffer, const BoundingBox& roi) override;

  // Initializes the ImageSearcher.
  absl::Status Init(std::unique_ptr<ImageSearcherOptions> options);

  // Performs pre-initialization actions.
  virtual absl::Status PreInit();

 private:
  std::unique_ptr<tflite::task::processor::SearchPostprocessor> postprocessor_;
};

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_SEARCHER_H_
