// Copyright 2022 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_UTILS_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_UTILS_H_

#include <cstddef>
#include <cstdint>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/calculators/tensor/inference_calculator.pb.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/memory_manager.h"
#include "mediapipe/framework/port/ret_check.h"
#include "tensorflow/lite/interpreter.h"
#include "tensorflow/lite/util.h"

namespace mediapipe {

// Returns number of threads to configure XNNPACK delegate with.
// Returns user provided value if specified. Otherwise, tries to choose optimal
// number of threads depending on the device. The default can be overridden by
// setting the --xnnpack_default_num_threads flag.
int GetXnnpackNumThreads(
    bool opts_has_delegate,
    const mediapipe::InferenceCalculatorOptions::Delegate& opts_delegate);

absl::Status CopyCpuInputIntoTfLiteTensor(const Tensor& input_tensor,
                                          TfLiteTensor& tflite_tensor);

absl::Status CopyTfLiteTensorIntoCpuOutput(const TfLiteTensor& tflite_tensor,
                                           Tensor& output_tensor);

// Converts TfLiteTensor to mediapipe::Tensor, returns InvalidArgumentError if
// the type is not supported.
absl::StatusOr<Tensor> ConvertTfLiteTensorToTensor(
    const TfLiteTensor& tflite_tensor);

// Returns true if the input tensor is aligned with the default alignment
// used by TFLite.
template <typename T>
bool IsAlignedWithTFLiteDefaultAlignment(T* data_ptr) {
  return (reinterpret_cast<const uintptr_t>(data_ptr) %
          tflite::kDefaultTensorAlignment) == 0;
}

// Uses TfLite's CustomAllocation to directly set the input tensor's data.
template <typename T>
absl::Status SetTfLiteCustomAllocation(tflite::Interpreter& interpreter,
                                       T* data_ptr, size_t size_bytes,
                                       int tensor_index) {
  RET_CHECK(IsAlignedWithTFLiteDefaultAlignment(data_ptr))
      << "data_ptr must be aligned to " << tflite::kDefaultTensorAlignment
      << " bytes.";
  TfLiteCustomAllocation allocation = {
      /*data=*/const_cast<void*>(reinterpret_cast<const void*>(data_ptr)),
      /*bytes=*/size_bytes};
  RET_CHECK_EQ(
      interpreter.SetCustomAllocationForTensor(tensor_index, allocation),
      kTfLiteOk);
  return absl::OkStatus();
}

// Creates a new MP Tensor instance that matches the size and type of the
// specified TfLite tensor. If optional 'alignment' is specified, the returned
// tensor will be byte aligned to that value.
absl::StatusOr<Tensor> CreateTensorWithTfLiteTensorSpecs(
    const TfLiteTensor& reference_tflite_tensor,
    MemoryManager* memory_manager = nullptr, int alignment = 0);

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_INFERENCE_CALCULATOR_UTILS_H_
