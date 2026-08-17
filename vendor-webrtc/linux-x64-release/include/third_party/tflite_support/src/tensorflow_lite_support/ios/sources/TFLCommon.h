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
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/**
 * @enum TFLSupportErrorCode
 * This enum specifies  error codes for TensorFlow Lite Task Library.
 * It maintains a 1:1 mapping to TfLiteSupportErrorCode of C libray.
 */
typedef NS_ENUM(NSUInteger, TFLSupportErrorCode) {

  /** Unspecified error. */
  TFLSupportErrorCodeUnspecifiedError = 1,

  /** Invalid argument specified. */
  TFLSupportErrorCodeInvalidArgumentError = 2,

  /** Invalid FlatBuffer file or buffer specified. */
  TFLSupportErrorCodeInvalidFlatBufferError = 3,

  /** Model contains a builtin op that isn't supported by the OpResolver or
   delegates.  */
  TFLSupportErrorCodeUnsupportedBuiltinOpError = 4,

  /** Model contains a custom op that isn't supported by the OpResolver or
   * delegates. */
  TFLSupportErrorCodeUnsupportedCustomOpError = 5,

  /** File I/O error codes. */

  /** No such file.  */
  TFLSupportErrorCodeFileNotFoundError = 100,

  /** Permission issue. */
  TFLSupportErrorCodeFilePermissionDeniedError,

  /** I/O error when reading file. */
  TFLSupportErrorCodeFileReadError,

  /** I/O error when mmap-ing file. */
  TFLSupportErrorCodeFileMmapError,

  /** TensorFlow Lite metadata error codes. */

  /** Unexpected schema version (aka file_identifier) in the Metadata FlatBuffer. */
  TFLSupportErrorCodeMetadataInvalidSchemaVersionError = 200,

  /** No such associated file within metadata, or file has not been packed. */
  TFLSupportErrorCodeMetadataAssociatedFileNotFoundError,

  /** ZIP I/O error when unpacking an associated file. */
  TFLSupportErrorCodeMetadataAssociatedFileZipError,

  /**
   * Inconsistency error between the metadata and actual TF Lite model.
   * E.g.: number of labels and output tensor values differ.
   */
  TFLSupportErrorCodeMetadataInconsistencyError,

  /**
   * Invalid process units specified.
   * E.g.: multiple ProcessUnits with the same type for a given tensor.
   */
  TFLSupportErrorCodeMetadataInvalidProcessUnitsError,

  /**
   * Inconsistency error with the number of labels.
   * E.g.: label files for different locales have a different number of labels.
   */
  TFLSupportErrorCodeMetadataNumLabelsMismatchError,

  /**
   * Score calibration parameters parsing error.
   * E.g.: too many parameters provided in the corresponding associated file.
   */
  TFLSupportErrorCodeMetadataMalformedScoreCalibrationError,

  /** Unexpected number of subgraphs for the current task.
   * E.g.: image classification expects a single subgraph.
   */
  TFLSupportErrorCodeMetadataInvalidNumSubgraphsError,
  /**
   * A given tensor requires NormalizationOptions but none were found.
   * E.g.: float input tensor requires normalization to preprocess input images.
   */
  TFLSupportErrorCodeMetadataMissingNormalizationOptionsError,

  /**
   * Invalid ContentProperties specified.
   * E.g. expected ImageProperties, got BoundingBoxProperties.
   */
  TFLSupportErrorCodeMetadataInvalidContentPropertiesError,

  /**
   * Metadata is mandatory but was not found.
   * E.g. current task requires TFLite Model Metadata but none was found.
   */
  TFLSupportErrorCodeMetadataNotFoundError,

  /**
   * Associated TENSOR_AXIS_LABELS or TENSOR_VALUE_LABELS file is mandatory but
   * none was found or it was empty.
   * E.g. current task requires labels but none were found.
   */
  TFLSupportErrorCodeMetadataMissingLabelsError,

  /**
   * The ProcessingUnit for tokenizer is not correctly configured.
   * E.g BertTokenizer doesn't have a valid vocab file associated.
   */
  TFLSupportErrorCodeMetadataInvalidTokenizerError,

  /** Input tensor(s) error codes. */

  /**
   * Unexpected number of input tensors for the current task.
   * E.g. current task expects a single input tensor.
   */
  TFLSupportErrorCodeInvalidNumInputTensorsError = 300,

  /**
   * Unexpected input tensor dimensions for the current task.
   * E.g.: only 4D input tensors supported.
   */
  TFLSupportErrorCodeInvalidInputTensorDimensionsError,

  /**
   * Unexpected input tensor type for the current task.
   * E.g.: current task expects a uint8 pixel image as input.
   */
  TFLSupportErrorCodeInvalidInputTensorTypeError,

  /**
   * Unexpected input tensor bytes size.
   * E.g.: size in bytes does not correspond to the expected number of pixels.
   */
  TFLSupportErrorCodeInvalidInputTensorSizeError,

  /**
   * No correct input tensor found for the model.
   * E.g.: input tensor name is not part of the text model's input tensors.
   */
  TFLSupportErrorCodeInputTensorNotFoundError,

  /** Output tensor(s) error codes. */

  /**
   * Unexpected output tensor dimensions for the current task.
   * E.g.: only a batch size of 1 is supported.
   */
  TFLSupportErrorCodeInvalidOutputTensorDimensionsError = 400,

  /**
   * Unexpected input tensor type for the current task.
   * E.g.: multi-head model with different output tensor types.
   */
  TFLSupportErrorCodeInvalidOutputTensorTypeError,

  /**
   * No correct output tensor found for the model.
   * E.g.: output tensor name is not part of the text model's output tensors.
   */
  TFLSupportErrorCodeOutputTensorNotFoundError,

  /**
   * Unexpected number of output tensors for the current task.
   * E.g.: current task expects a single output tensor.
   */
  TFLSupportErrorCodeInvalidNumOutputTensorsError,

  /** Image processing error codes. **/

  /**  Unspecified image processing failures. */
  TFLSupportErrorCodeImageProcessingError = 500,

  /**
   * Unexpected input or output buffer metadata.
   * E.g.: rotate RGBA buffer to Grayscale buffer by 90 degrees.
   */
  TFLSupportErrorCodeImageProcessingInvalidArgumentError,

  /**
   * Image processing operation failures.
   * E.g. libyuv rotation failed for an unknown reason.
   */
  TFLSupportErrorCodeImageProcessingBackendError,

  /**
   * The first error code in TFLSupportErrorCode (for internal use only).
   */
  TFLErrorCodeFirst = TFLSupportErrorCodeUnspecifiedError,

  /**
   * The last error code in TFLSupportErrorCode (for internal use only).
   */
  TFLErrorCodeLast = TFLSupportErrorCodeImageProcessingBackendError,

  /** kNotFound indicates some requested entity (such as a file or directory) was not found. */
  TFLSupportErrorCodeNotFoundError = 900,

  /** kInternal indicates an internal error has occurred and some invariants expected by the
   * underlying system have not been satisfied. This error code is reserved for serious errors.
   */
  TFLSupportErrorCodeInternalError,
} NS_SWIFT_NAME(SupportErrorCode);

NS_ASSUME_NONNULL_END
