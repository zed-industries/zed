#ifndef MEDIAPIPE_CALCULATORS_TENSOR_Inference_feedback_manager_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_Inference_feedback_manager_H_

#include <vector>

#include "absl/container/flat_hash_set.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/calculators/tensor/inference_calculator.pb.h"
#include "mediapipe/calculators/tensor/inference_io_mapper.h"
#include "tensorflow/lite/interpreter.h"

namespace mediapipe {

// Feedback tensors are pairs of model output / input tensors where the
// model output is used as model input in the next model invocation. This allows
// to manage a notion of temporal state by continuously feeding from the model's
// output to the model's input during each inference step. The
// InferenceFeedbackManager initializes the feedback input tensors with zeros
// and efficiently swaps them from output to input with zero copies.
class InferenceFeedbackManager {
 public:
  // Initializes the feedback tensors with zeros and generates
  // feedback_tensor_indices_links_. The provided interpreter must outlive the
  // InferenceFeedbackManager instance.
  absl::Status Init(
      const mediapipe::InferenceCalculatorOptions::InputOutputConfig& io_config,
      const InputOutputTensorNames& input_output_tensor_names_map,
      tflite::Interpreter* interpreter);

  // Swaps the feedback tensors from output to input.
  void SwapFeedbackTensors();

  // Returns the number of expected non-feedback tensors. This can be used to
  // confirm the number of input tensors to the InferenceRunner implementation.
  int GetNumberOfNonFeedbackInputTensors() const;

  //  Returns the number of feedback tensor pairs.
  int GetNumberOfFeedbackTensors() const;

  // Since feedback tensors are excluded from InferenceRunner input, This method
  // maps the tensor index from the InferenceRunner input to the TfLite model
  // tensor input.
  absl::StatusOr<int> MapInputTensorToModelIndex(int input_idx) const;

  // Returns true if the tensor at the given index is a feedback input tensor.
  bool IsFeedbackInputTensorAtIndex(int idx) const;

  // Returns true if the tensor at the given index is a feedback output tensor.
  bool IsFeedbackOutputTensorAtIndex(int idx) const;

 private:
  // Links between feedback tensors defined by model tensor indices.
  struct TensorFeedbackIndicesLink {
    int from_idx;
    int to_idx;
  };

  // Translates the tensor names from the input/output config into the
  // corresponding TfLite tensor indices.
  static absl::StatusOr<std::vector<TensorFeedbackIndicesLink>>
  ConvertSignatureTensorNamesToModelIndices(
      const mediapipe::InferenceCalculatorOptions::InputOutputConfig& io_config,
      const InputOutputTensorNames& input_output_tensor_names_map);

  // Non-owning reference to the TfLite interpreter.
  tflite::Interpreter* interpreter_ = nullptr;

  // List of tensor feedback pairs defined by model tensor indices.
  std::vector<TensorFeedbackIndicesLink> feedback_tensor_indices_links_;

  // Maps InferenceRunner input indices to TfLiteModel input indices.
  std::vector<int> input_tensor_to_model_indices_;

  // Set of feedback input model tensor indices.
  absl::flat_hash_set<int> feedback_input_indices_;

  // Set of feedback output model tensor indices.
  absl::flat_hash_set<int> feedback_output_indices_;
};
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_Inference_feedback_manager_H_
