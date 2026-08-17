/* Copyright 2022 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_CC_AUDIO_UTILS_AUDIO_TENSOR_SPECS_H_
#define MEDIAPIPE_TASKS_CC_AUDIO_UTILS_AUDIO_TENSOR_SPECS_H_

#include <array>

#include "absl/status/statusor.h"
#include "absl/types/optional.h"
#include "mediapipe/tasks/cc/metadata/metadata_extractor.h"
#include "mediapipe/tasks/metadata/metadata_schema_generated.h"

namespace mediapipe {
namespace tasks {
namespace audio {

// Parameters related to the expected tensor specifications when the tensor
// represents an audio buffer.
//
// E.g. Before running inference with the TF Lite interpreter, the caller must
// use these values and perform audio preprocessing so as to fill the actual
// input tensor appropriately.
struct AudioTensorSpecs {
  // Expected audio dimensions.
  // Expected number of channels of the input audio buffer, e.g.,
  // num_channels=1,
  int num_channels;
  //  Expected number of samples per channel of the input audio buffer, e.g.,
  //  num_samples=15600.
  int num_samples;
  // Expected sample rate, e.g., sample_rate=16000 for 16kHz.
  int sample_rate;
  // Expected input tensor type, e.g., tensor_type=TensorType_FLOAT32.
  tflite::TensorType tensor_type;
  // The number of the overlapping samples per channel between adjacent input
  // tensors.
  int num_overlapping_samples;
};

// Gets the audio tensor metadata from the metadata extractor by tensor index.
absl::StatusOr<const tflite::TensorMetadata*> GetAudioTensorMetadataIfAny(
    const metadata::ModelMetadataExtractor& metadata_extractor,
    int tensor_index);

// Performs sanity checks on the expected input tensor including consistency
// checks against model metadata, if any. For now, a 1D or 2D audio tesnor,
// is expected. Returns the corresponding input specifications if they pass, or
// an error otherwise (too many input tensors, etc).
// Note: both model and metadata extractor *must* be successfully
// initialized before calling this function by means of (respectively):
// - `tflite::GetModel`,
// - `mediapipe::metadata::ModelMetadataExtractor::CreateFromModelBuffer`.
absl::StatusOr<AudioTensorSpecs> BuildInputAudioTensorSpecs(
    const tflite::Tensor& audio_tensor,
    const tflite::TensorMetadata* audio_tensor_metadata);

}  // namespace audio
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_AUDIO_UTILS_AUDIO_TENSOR_SPECS_H_
