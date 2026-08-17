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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_BASE_VISION_TASK_API_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_BASE_VISION_TASK_API_H_

#include <array>
#include <memory>
#include <utility>
#include <vector>

#include "absl/memory/memory.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/time/clock.h"  // from @com_google_absl
#include "tensorflow/lite/c/common.h"
#include "tensorflow_lite_support/cc/common.h"
#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/core/task_utils.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/processor/image_preprocessor.h"
#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"
#include "tensorflow_lite_support/cc/task/vision/proto/bounding_box_proto_inc.h"
#include "tensorflow_lite_support/cc/task/vision/utils/frame_buffer_utils.h"
#include "tensorflow_lite_support/cc/task/vision/utils/image_tensor_specs.h"
#include "tensorflow_lite_support/metadata/metadata_schema_generated.h"

namespace tflite {
namespace task {
namespace vision {

// Base class providing common logic for vision models.
template <class OutputType>
class BaseVisionTaskApi
    : public tflite::task::core::BaseTaskApi<OutputType, const FrameBuffer&,
                                             const BoundingBox&> {
 public:
  explicit BaseVisionTaskApi(std::unique_ptr<core::TfLiteEngine> engine)
      : tflite::task::core::BaseTaskApi<OutputType, const FrameBuffer&,
                                        const BoundingBox&>(std::move(engine)) {
  }
  // BaseVisionTaskApi is neither copyable nor movable.
  BaseVisionTaskApi(const BaseVisionTaskApi&) = delete;
  BaseVisionTaskApi& operator=(const BaseVisionTaskApi&) = delete;

  // Sets the ProcessEngine used for image pre-processing. Must be called before
  // any inference is performed. Can be called between inferences to override
  // the current process engine.
  void SetProcessEngine(const FrameBufferUtils::ProcessEngine& process_engine) {
    process_engine_ = process_engine;
  }

 protected:
  FrameBufferUtils::ProcessEngine process_engine_;

  // Checks input tensor and metadata (if any) are valid, or return an error
  // otherwise. This must be called once at initialization time, before running
  // inference, as it is a prerequisite for `Preprocess`.
  // Note: the underlying interpreter and metadata extractor are assumed to be
  // already successfully initialized before calling this method.
  virtual absl::Status CheckAndSetInputs() {
    // BaseTaskApi always assume having a single input.
    TFLITE_ASSIGN_OR_RETURN(preprocessor_,
                     ::tflite::task::processor::ImagePreprocessor::Create(
                         this->GetTfLiteEngine(), {0}, process_engine_));
    return absl::OkStatus();
  }

  // Performs image preprocessing on the input frame buffer over the region of
  // interest so that it fits model requirements (e.g. upright 224x224 RGB) and
  // populate the corresponding input tensor. This is performed by (in this
  // order):
  // - cropping the frame buffer to the region of interest (which, in most
  //   cases, just covers the entire input image),
  // - resizing it (with bilinear interpolation, aspect-ratio *not* preserved)
  //   to the dimensions of the model input tensor,
  // - converting it to the colorspace of the input tensor (i.e. RGB, which is
  //   the only supported colorspace for now),
  // - rotating it according to its `Orientation` so that inference is performed
  //   on an "upright" image.
  //
  // IMPORTANT: as a consequence of cropping occurring first, the provided
  // region of interest is expressed in the unrotated frame of reference
  // coordinates system, i.e. in `[0, frame_buffer.width) x [0,
  // frame_buffer.height)`, which are the dimensions of the underlying
  // `frame_buffer` data before any `Orientation` flag gets applied. Also, the
  // region of interest is not clamped, so this method will return a non-ok
  // status if the region is out of these bounds.
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const FrameBuffer& frame_buffer,
                          const BoundingBox& roi) override {
    if (preprocessor_ == nullptr) {
      return tflite::support::CreateStatusWithPayload(
          absl::StatusCode::kInternal,
          "Uninitialized preprocessor: CheckAndSetInputs must be called "
          "at initialization time.");
    }
    if (GetInputSpecs().image_height == 0 && GetInputSpecs().image_width == 0) {
      return tflite::support::CreateStatusWithPayload(
          absl::StatusCode::kInternal,
          "Uninitialized input tensor specs: CheckAndSetInputs must be called "
          "at initialization time.");
    }
    return preprocessor_->Preprocess(frame_buffer, roi);
  }

  // Returns the spec for the input image.
  const vision::ImageTensorSpecs& GetInputSpecs() const {
    return preprocessor_->GetInputSpecs();
  }

 private:
  std::unique_ptr<processor::ImagePreprocessor> preprocessor_ = nullptr;
};

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_BASE_VISION_TASK_API_H_
