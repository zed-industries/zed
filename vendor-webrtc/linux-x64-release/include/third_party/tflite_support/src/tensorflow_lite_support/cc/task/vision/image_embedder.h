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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_EMBEDDER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_EMBEDDER_H_

#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/external_file_handler.h"
#include "tensorflow_lite_support/cc/task/processor/embedding_postprocessor.h"
#include "tensorflow_lite_support/cc/task/vision/core/base_vision_task_api.h"
#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"
#include "tensorflow_lite_support/cc/task/vision/proto/bounding_box_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/proto/embeddings_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/proto/image_embedder_options_proto_inc.h"

namespace tflite {
namespace task {
namespace vision {

// Performs dense feature vector extraction on images.
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
// At least one output tensor with:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - `N` components corresponding to the `N` dimensions of the returned
//      feature vector for this output layer.
//    - Either 2 or 4 dimensions, i.e. `[1 x N]` or `[1 x 1 x 1 x N]`.
//
// TODO(b/180502532): add pointer to example model.
//
// A CLI demo tool is available for easily trying out this API, and provides
// example usage. See:
// examples/task/vision/desktop/image_embedder_demo.cc
class ImageEmbedder
    : public tflite::task::vision::BaseVisionTaskApi<EmbeddingResult> {
 public:
  using BaseVisionTaskApi::BaseVisionTaskApi;

  // Creates an ImageEmbedder from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<ImageEmbedder>>
  CreateFromOptions(
      const ImageEmbedderOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs actual feature vector extraction on the provided FrameBuffer.
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
  tflite::support::StatusOr<EmbeddingResult> Embed(
      const FrameBuffer& frame_buffer);

  // Same as above, except the inference is performed only on the provided
  // region of interest. Note that the region of interest is not clamped, so
  // this method will fail if the region is out of bounds of the input image.
  tflite::support::StatusOr<EmbeddingResult> Embed(
      const FrameBuffer& frame_buffer, const BoundingBox& roi);

  // Returns the Embedding output by the output_index'th layer. In (the most
  // common) case where a single embedding is produced, you can just call
  // GetEmbeddingByIndex(result, 0).
  // Returns an empty Embedding if `output_index` is out of bounds.
  Embedding GetEmbeddingByIndex(const EmbeddingResult& result,
                                int output_index);

  // Returns the dimensionality of the embedding output by the output_index'th
  // output layer. Returns -1 if `output_index` is out of bounds.
  int GetEmbeddingDimension(int output_index) const;

  // Returns the number of output layers of the model.
  int GetNumberOfOutputLayers() const;

  // Utility function to compute cosine similarity [1] between two feature
  // vectors. May return an InvalidArgumentError if e.g. the feature vectors are
  // of different types (quantized vs. float), have different sizes, or have a
  // an L2-norm of 0.
  //
  // [1]: https://en.wikipedia.org/wiki/Cosine_similarity
  static tflite::support::StatusOr<double> CosineSimilarity(
      const FeatureVector& u, const FeatureVector& v);

 protected:
  // The options used to build this ImageEmbedder.
  std::unique_ptr<ImageEmbedderOptions> options_;

  // Post-processing to transform the raw model outputs into embedding results.
  tflite::support::StatusOr<EmbeddingResult> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const FrameBuffer& frame_buffer, const BoundingBox& roi) override;

  // Performs pre-initialization actions.
  virtual absl::Status PreInit();
  // Performs post-initialization actions.
  virtual absl::Status PostInit();

  // Initializes the ImageEmbedder.
  absl::Status Init(std::unique_ptr<ImageEmbedderOptions> options);

  // Performs scalar quantization on a feature vector whose elements are
  // assumed to lie in the range [-1.0, 1.0] (values outside this range will be
  // clamped to -128 or 127).
  void QuantizeFeatureVector(FeatureVector* feature_vector) const;

 private:
  std::vector<std::unique_ptr<processor::EmbeddingPostprocessor>>
      postprocessors_;
};

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_IMAGE_EMBEDDER_H_
