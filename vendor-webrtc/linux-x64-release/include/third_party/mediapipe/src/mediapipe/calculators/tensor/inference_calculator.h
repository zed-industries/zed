// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_H_

#include <algorithm>
#include <functional>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/calculators/tensor/inference_calculator.pb.h"
#include "mediapipe/calculators/tensor/inference_io_mapper.h"
#include "mediapipe/calculators/tensor/tensor_span.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/util/tflite/tflite_model_loader.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"

namespace mediapipe {
namespace api2 {

// Runs inference on the provided input Tensors and TFLite model.
//
// Creates an interpreter with given model and calls invoke().
// Optionally run inference on CPU/GPU.
//
// This calculator can be used with TensorConverterCalculator to get the
// appropriate inputs.
//
// When the input tensors are on CPU, gpu inference is optional and can be
// specified in the calculator options.
// When the input tensors are on GPU, inference is GPU and output can be CPU or
// GPU.
//
// Input:
//  TENSORS - Vector of Tensors
//
// Output:
//  TENSORS - Vector of Tensors
//
// Input side packet:
//  CUSTOM_OP_RESOLVER (optional)
//    DEPRECATED: prefer to use the "OP_RESOLVER" input side packet instead.
//    Use a custom op resolver, instead of the builtin one.
//  OP_RESOLVER (optional)
//    Use to provide tflite op resolver (tflite::OpResolver)
//  MODEL (optional)
//    Use to specify TfLite model.
//    (std::unique_ptr<tflite::FlatBufferModel,
//       std::function<void(tflite::FlatBufferModel*)>>)
//  DELEGATE (optional)
//    Use to specify special values per a particular delegate.
//    (InferenceCalculatorOptions::Delegate)
//  IO_CONFIG (optional)
//    Use to specify input/output remapping.
//    (InferenceCalculatorOptions::InputOutputConfig)
//
//    NOTE: InferenceCalculator, being a subgraph which is replaced by concrete
//      implementations/calculators during the graph expansion, cannot access
//      side packets, and DELEGATE side packet rarely (only if concrete
//      implementations/calculators allow that) can be used to switch between
//      delegates.
//
// Example use:
// node {
//   calculator: "InferenceCalculator"
//   input_stream: "TENSORS:tensor_image"
//   output_stream: "TENSORS:tensors"
//   options: {
//     [mediapipe.InferenceCalculatorOptions.ext] {
//       model_path: "modelname.tflite"
//     }
//   }
// }
//
// or
//
// node {
//   calculator: "InferenceCalculator"
//   input_stream: "TENSORS:tensor_image"
//   input_side_packet: "MODEL:model"
//   output_stream: "TENSORS:tensors"
//   options: {
//     [mediapipe.InferenceCalculatorOptions.ext] {
//       model_path: "modelname.tflite"
//       delegate { gpu {} }
//     }
//   }
// }
//
// IMPORTANT Notes:
//  Tensors are assumed to be ordered correctly (sequentially added to model).
//  Input tensors are assumed to be of the correct size and already normalized.

class InferenceCalculator : public NodeIntf {
 public:
  // Default API: inputs and outputs will be passed as a single vector.
  static constexpr Input<std::vector<Tensor>>::Optional kInTensors{"TENSORS"};
  static constexpr Output<std::vector<Tensor>>::Optional kOutTensors{"TENSORS"};

  // New API (not yet supported by all subclasses): inputs and outputs will be
  // passed as multiple (ordered) Tensor streams. Only one of the two APIs can
  // be used, so TENSORS and TENSOR are mutually exclusive.
  static constexpr Input<Tensor>::Multiple kInTensor{"TENSOR"};
  static constexpr Output<Tensor>::Multiple kOutTensor{"TENSOR"};

  // Deprecated. Prefers to use "OP_RESOLVER" input side packet instead.
  // TODO: Removes the "CUSTOM_OP_RESOLVER" side input after the
  // migration.
  static constexpr SideInput<tflite::ops::builtin::BuiltinOpResolver>::Optional
      kSideInCustomOpResolver{"CUSTOM_OP_RESOLVER"};
  static constexpr SideInput<tflite::OpResolver>::Optional kSideInOpResolver{
      "OP_RESOLVER"};
  static constexpr SideInput<TfLiteModelPtr>::Optional kSideInModel{"MODEL"};
  static constexpr SideInput<
      mediapipe::InferenceCalculatorOptions::Delegate>::Optional kDelegate{
      "DELEGATE"};
  static constexpr SideInput<
      mediapipe::InferenceCalculatorOptions::InputOutputConfig>::Optional
      kSideInIoMap{"IO_CONFIG"};
  MEDIAPIPE_NODE_CONTRACT(kInTensors, kInTensor, kSideInCustomOpResolver,
                          kSideInOpResolver, kSideInModel, kOutTensors,
                          kOutTensor, kDelegate, kSideInIoMap);

 protected:
  using TfLiteDelegatePtr =
      std::unique_ptr<TfLiteOpaqueDelegate,
                      std::function<void(TfLiteOpaqueDelegate*)>>;

  // Helper to be used in subclass UpdateContract calls to enforce constraints
  // when TENSORS and TENSOR are both available.
  static absl::Status TensorContractCheck(CalculatorContract* cc);

  static absl::StatusOr<Packet<TfLiteModelPtr>> GetModelAsPacket(
      CalculatorContext* cc);

  static absl::StatusOr<Packet<tflite::OpResolver>> GetOpResolverAsPacket(
      CalculatorContext* cc);

  // Checks if feedback tensor support is available and warns otherwise.
  static void WarnFeedbackTensorsUnsupported(CalculatorContract* cc);
};

struct InferenceCalculatorSelector : public InferenceCalculator {
  static constexpr char kCalculatorName[] = "InferenceCalculator";
};

struct InferenceCalculatorGl : public InferenceCalculator {
  static constexpr char kCalculatorName[] = "InferenceCalculatorGl";
};

struct InferenceCalculatorGlAdvanced : public InferenceCalculator {
  static constexpr char kCalculatorName[] = "InferenceCalculatorGlAdvanced";
};

struct InferenceCalculatorMetal : public InferenceCalculator {
  static constexpr char kCalculatorName[] = "InferenceCalculatorMetal";
};

struct InferenceCalculatorCpu : public InferenceCalculator {
  static constexpr char kCalculatorName[] = "InferenceCalculatorCpu";
};

struct InferenceCalculatorXnnpack : public InferenceCalculator {
  static constexpr char kCalculatorName[] = "InferenceCalculatorXnnpack";
};

// For Process overriding, we subclass Impl rather than Intf
// Subclasses must implement InferenceCalculatorNodeImpl's `Process` method.
template <class Intf, class Impl = void>
class InferenceCalculatorNodeImpl : public NodeImpl<Intf, Impl> {
 public:
  virtual ~InferenceCalculatorNodeImpl() = default;

  // Override Process to handle common Tensor I/O functionality.
  absl::Status Process(CalculatorContext* cc) final {
    if (InferenceCalculator::kInTensors(cc).IsConnected()) {
      // Using old vector<Tensor> inputs; skip if empty input stream, but error
      // if the input vector is empty.
      if (InferenceCalculator::kInTensors(cc).IsEmpty()) {
        return absl::OkStatus();
      }
      const auto& input_tensors = *InferenceCalculator::kInTensors(cc);
      RET_CHECK(!input_tensors.empty());
      MP_ASSIGN_OR_RETURN(
          auto output_tensors,
          RemapAndProcessTensors(cc, MakeTensorSpan(input_tensors)));
      return SendOutputTensors(cc, std::move(output_tensors));
    }
    // Using new direct Tensor inputs; return early if any empty streams.
    for (int i = 0; i < InferenceCalculator::kInTensor(cc).Count(); ++i) {
      if (InferenceCalculator::kInTensor(cc)[i].IsEmpty()) {
        return absl::OkStatus();
      }
    }

    MP_ASSIGN_OR_RETURN(
        auto output_tensors,
        RemapAndProcessTensors(
            cc, MakeTensorSpan(InferenceCalculator::kInTensor(cc))));
    return SendOutputTensors(cc, std::move(output_tensors));
  }

 protected:
  // Updates IoMapper with input/output tensor names from the TfLite model.
  absl::Status UpdateIoMapping(CalculatorContext* cc,
                               const InputOutputTensorNames& tensor_names) {
    if (io_mapper_ == nullptr) {
      io_mapper_ = std::make_unique<InferenceIoMapper>();
    }
    return io_mapper_->UpdateIoMap(GetInputOutputConfig(cc), tensor_names);
  }

  // Process call providing TensorSpan input.
  virtual absl::StatusOr<std::vector<Tensor>> Process(
      CalculatorContext* cc, const TensorSpan& tensor_span) = 0;

 private:
  // Remaps input tensors according to the IO map, runs inference, and remaps
  // output tensors.
  absl::StatusOr<std::vector<Tensor>> RemapAndProcessTensors(
      CalculatorContext* cc, const TensorSpan& input_tensors) {
    RET_CHECK(io_mapper_ != nullptr)
        << "IO mapper is not initialized. MaybeUpdateIoMapping must be called "
           "prior to Process.";
    MP_ASSIGN_OR_RETURN(const TensorSpan input_tensors_remapped,
                        io_mapper_->RemapInputTensors(input_tensors));
    MP_ASSIGN_OR_RETURN(std::vector<Tensor> output_tensors,
                        Process(cc, input_tensors_remapped));
    return io_mapper_->RemapOutputTensors(std::move(output_tensors));
  }

  // Send output tensors into the proper output streams, regardless of how
  // those Tensors are expected to be sent. We take an rvalue-reference to
  // ensure we can destroy/move the tensors.
  static absl::Status SendOutputTensors(CalculatorContext* cc,
                                        std::vector<Tensor>&& output_tensors) {
    if (InferenceCalculator::kOutTensors(cc).IsConnected()) {
      InferenceCalculator::kOutTensors(cc).Send(std::move(output_tensors));
    } else {
      const int output_count =
          std::min(InferenceCalculator::kOutTensor(cc).Count(),
                   static_cast<int>(output_tensors.size()));
      for (int i = 0; i < output_count; ++i) {
        InferenceCalculator::kOutTensor(cc)[i].Send(
            std::move(output_tensors[i]));
      }
    }
    return absl::OkStatus();
  }

  // Looks up InputOutputConfig from side-packet or options. Returns an
  // empty map in case of missing configuration.
  static mediapipe::InferenceCalculatorOptions::InputOutputConfig
  GetInputOutputConfig(CalculatorContext* cc) {
    if (InferenceCalculator::kSideInIoMap(cc).IsConnected()) {
      return *InferenceCalculator::kSideInIoMap(cc)
                  .As<mediapipe::InferenceCalculatorOptions::
                          InputOutputConfig>();
    }
    const auto& options = cc->Options<mediapipe::InferenceCalculatorOptions>();
    if (options.has_input_output_config()) {
      return options.input_output_config();
    }
    // In case of missing configuration, return empty config.
    return mediapipe::InferenceCalculatorOptions::InputOutputConfig();
  }

  std::unique_ptr<InferenceIoMapper> io_mapper_;
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_H_
