#ifndef MEDIAPIPE_UTIL_TFLITE_TFLITE_SIGNATURE_READER_H_
#define MEDIAPIPE_UTIL_TFLITE_TFLITE_SIGNATURE_READER_H_

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "tensorflow/lite/interpreter.h"

namespace mediapipe {
using SignatureName = std::string;

// Stores input and output tensor name vectors which are ordered in
// accordance to the default signature of the provided TfLite model.
struct SignatureInputOutputTensorNames {
  std::vector<std::string> input_tensor_names;
  std::vector<std::string> output_tensor_names;
};

class TfLiteSignatureReader {
 public:
  // Returns names of input and output tensors from TfLite signatures in the
  // order the TfLite model / InferenceCalculators expects them. The interpreter
  // argument must be initialized with a TfLite model. If the optional signature
  // key is provided, the model matching the signature will be queried. Returns
  // an error if the signature is not found. If signature_key is not provided, a
  // single TfLite signature is expected. Returns pair of input and output
  // tensor names.
  static absl::StatusOr<SignatureInputOutputTensorNames>
  GetInputOutputTensorNamesFromTfliteSignature(
      const tflite::Interpreter& interpreter,
      const std::string* signature_key = nullptr);

  // Returns a map of signature name to input and output tensor names from all
  // TfLite signatures in the order the TfLite model / InferenceCalculators
  // expects them. The interpreter argument must be initialized with a TfLite
  // model.
  static absl::StatusOr<
      absl::flat_hash_map<SignatureName, SignatureInputOutputTensorNames>>
  GetInputOutputTensorNamesFromAllTfliteSignatures(
      const tflite::Interpreter& interpreter);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TFLITE_TFLITE_SIGNATURE_READER_H_
