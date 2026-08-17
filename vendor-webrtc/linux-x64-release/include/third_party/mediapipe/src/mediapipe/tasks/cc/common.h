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

#ifndef MEDIAPIPE_TASKS_CC_COMMON_H_
#define MEDIAPIPE_TASKS_CC_COMMON_H_

#include "absl/status/status.h"
#include "absl/strings/string_view.h"

namespace mediapipe {
namespace tasks {

// Name (aka type URL key) of the `absl::Status` payload which contains a
// stringified `MediaPipeTasksStatus` code (see below).
inline constexpr absl::string_view kMediaPipeTasksPayload =
    "MediaPipeTasksStatus";

// Error codes for MediaPipe Tasks C++ APIs.
//
// At runtime, such codes are meant to be attached (where applicable) to a
// `absl::Status` in a key-value manner with `kMediaPipeTasksPayload` as key and
// stringified error code as value (aka payload). This logic is encapsulated in
// the `CreateStatusWithPayload` helper below for convenience.
//
// The returned status includes:
// 1. The canonical error code (INVALID_ARGUMENT)
// 2. The fine-grained error message ("Invalid metadata ...")
// 3. The specific status code as a payload (kMetadataInvalidSchemaVersionError)
enum class MediaPipeTasksStatus {
  // Generic error codes.

  // Success.
  kOk = 0,
  // Unspecified error.
  kError = 1,
  // Invalid argument specified.
  kInvalidArgumentError = 2,
  // Invalid FlatBuffer file or buffer specified.
  kInvalidFlatBufferError = 3,
  // Model contains a builtin op that isn't supported by the OpResolver or
  // delegates.
  kUnsupportedBuiltinOp = 4,
  // Model contains a custom op that isn't supported by the OpResolver or
  // delegates.
  kUnsupportedCustomOp = 5,

  // File I/O error codes.

  // No such file.
  kFileNotFoundError = 100,
  // Permission issue.
  kFilePermissionDeniedError,
  // I/O error when reading file.
  kFileReadError,
  // I/O error when mmap-ing file.
  kFileMmapError,
  // ZIP I/O error when unpacking the zip file.
  kFileZipError,

  // TensorFlow Lite metadata error codes.

  // Unexpected schema version (aka file_identifier) in the Metadata FlatBuffer.
  kMetadataInvalidSchemaVersionError = 200,
  // No such associated file within metadata, or file has not been packed.
  kMetadataAssociatedFileNotFoundError,
  // ZIP I/O error when unpacking an associated file.
  kMetadataAssociatedFileZipError,
  // Inconsistency error between the metadata and actual TF Lite model.
  // E.g.: number of labels and output tensor values differ.
  kMetadataInconsistencyError,
  // Invalid process units specified.
  // E.g.: multiple ProcessUnits with the same type for a given tensor.
  kMetadataInvalidProcessUnitsError,
  // Inconsistency error with the number of labels.
  // E.g.: label files for different locales have a different number of labels.
  kMetadataNumLabelsMismatchError,
  // Score calibration parameters parsing error.
  // E.g.: too many parameters provided in the corresponding associated file.
  kMetadataMalformedScoreCalibrationError,
  // Unexpected number of subgraphs for the current task.
  // E.g.: image classification expects a single subgraph.
  kMetadataInvalidNumSubgraphsError,
  // A given tensor requires NormalizationOptions but none were found.
  // E.g.: float input tensor requires normalization to preprocess input images.
  kMetadataMissingNormalizationOptionsError,
  // Invalid ContentProperties specified.
  // E.g. expected ImageProperties, got BoundingBoxProperties.
  kMetadataInvalidContentPropertiesError,
  // Metadata is mandatory but was not found.
  // E.g. current task requires TFLite Model Metadata but none was found.
  kMetadataNotFoundError,
  // Associated TENSOR_AXIS_LABELS or TENSOR_VALUE_LABELS file is mandatory but
  // none was found or it was empty.
  // E.g. current task requires labels but none were found.
  kMetadataMissingLabelsError,
  // The ProcessingUnit for tokenizer is not correctly configured.
  // E.g BertTokenizer doesn't have a valid vocab file associated.
  kMetadataInvalidTokenizerError,

  // Input tensor(s) error codes.

  // Unexpected number of input tensors for the current task.
  // E.g. current task expects a single input tensor.
  kInvalidNumInputTensorsError = 300,
  // Unexpected input tensor dimensions for the current task.
  // E.g.: only 4D input tensors supported.
  kInvalidInputTensorDimensionsError,
  // Unexpected input tensor type for the current task.
  // E.g.: current task expects a uint8 pixel image as input.
  kInvalidInputTensorTypeError,
  // Unexpected input tensor bytes size.
  // E.g.: size in bytes does not correspond to the expected number of pixels.
  kInvalidInputTensorSizeError,
  // No correct input tensor found for the model.
  // E.g.: input tensor name is not part of the text model's input tensors.
  kInputTensorNotFoundError,

  // Output tensor(s) error codes.

  // Unexpected output tensor dimensions for the current task.
  // E.g.: only a batch size of 1 is supported.
  kInvalidOutputTensorDimensionsError = 400,
  // Unexpected input tensor type for the current task.
  // E.g.: multi-head model with different output tensor types.
  kInvalidOutputTensorTypeError,
  // No correct output tensor found for the model.
  // E.g.: output tensor name is not part of the text model's output tensors.
  kOutputTensorNotFoundError,
  // Unexpected number of output tensors for the current task.
  // E.g.: current task expects a single output tensor.
  kInvalidNumOutputTensorsError,

  // Image processing error codes.

  // Unspecified image processing failures.
  kImageProcessingError = 500,
  // Unexpected input or output buffer metadata.
  // E.g.: rotate RGBA buffer to Grayscale buffer by 90 degrees.
  kImageProcessingInvalidArgumentError,
  // Image processing operation failures.
  // E.g. libyuv rotation failed for an unknown reason.
  kImageProcessingBackendError,

  // Task runner error codes.
  kRunnerError = 600,
  // Task runner is not initialized.
  kRunnerInitializationError,
  // Task runner is not started successfully.
  kRunnerFailsToStartError,
  // Task runner is not started.
  kRunnerNotStartedError,
  // Task runner API is called in the wrong processing mode.
  kRunnerApiCalledInWrongModeError,
  // Task runner receives/produces invalid MediaPipe packet timestamp.
  kRunnerInvalidTimestampError,
  // Task runner receives unexpected MediaPipe graph input packet.
  // E.g. The packet type doesn't match the graph input stream's data type.
  kRunnerUnexpectedInputError,
  // Task runner produces unexpected MediaPipe graph output packet.
  // E.g. The number of output packets is not equal to the number of graph
  // output streams.
  kRunnerUnexpectedOutputError,
  // Task runner is not closed successfully.
  kRunnerFailsToCloseError,
  // Task runner's model resources cache service is unavailable or the
  // targeting model resources bundle is not found.
  kRunnerModelResourcesCacheServiceError,

  // Task graph error codes.
  kGraphError = 700,
  // Task graph is not implemented.
  kTaskGraphNotImplementedError,
  // Task graph config is invalid.
  kInvalidTaskGraphConfigError,
};

// Convenience helper to create an `absl::Status` augmented with the
// fine-grained `mediapipe_tasks_code` attached as payload under the
// `kMediaPipeTasksPayload` type URL key.
//
// This should only be used for non-ok codes since otherwise it does nothing
// more than returning an object identical to an OK status. See `absl::Status`
// for more details.
absl::Status CreateStatusWithPayload(
    absl::StatusCode canonical_code, absl::string_view message,
    MediaPipeTasksStatus mediapipe_tasks_code = MediaPipeTasksStatus::kError);

// Attaches a new mediapipe tasks status payload to a non-ok status.
absl::Status AddPayload(
    absl::Status status, absl::string_view message,
    MediaPipeTasksStatus mediapipe_tasks_code = MediaPipeTasksStatus::kError);

}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_COMMON_H_
