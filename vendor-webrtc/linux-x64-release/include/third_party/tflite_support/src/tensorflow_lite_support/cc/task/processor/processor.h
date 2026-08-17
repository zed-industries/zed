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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_PROCESSOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_PROCESSOR_H_

#include <initializer_list>
#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/str_format.h"  // from @com_google_absl
#include "tensorflow/lite/c/common.h"
#include "tensorflow_lite_support/cc/common.h"
#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"

namespace tflite {
namespace task {
namespace processor {

// Abstract base class for all Processors.
// Shares the common logics to handle tflite_engine and metadata.
class Processor {
 public:
  Processor() = default;
  virtual ~Processor() = default;

  // Processor is neither copyable nor movable.
  Processor(const Processor&) = delete;
  Processor& operator=(const Processor&) = delete;

  template <typename T>
  using EnableIfProcessorSubclass =
      typename std::enable_if<std::is_base_of<Processor, T>::value>::type*;

  // Factory method to create a subclass of Processor.
  //
  // Example usage:
  // auto processor = Processor::Create<MyPreprocessor>(
  //   num_expected_tensors, engine, tensor_indices);
  template <typename T, EnableIfProcessorSubclass<T> = nullptr>
  static tflite::support::StatusOr<std::unique_ptr<T>> Create(
      int num_expected_tensors, tflite::task::core::TfLiteEngine* engine,
      const std::initializer_list<int> tensor_indices,
      bool requires_metadata = true) {
    auto processor = absl::make_unique<T>(engine, tensor_indices);
    TFLITE_RETURN_IF_ERROR(
        processor->SanityCheck(num_expected_tensors, requires_metadata));
    return processor;
  }

  // `tensor_indices` is the indices of the input tensors or output tensors
  // that the processor should process. For example, a model may have 4 input
  // tensors, and a preprocessor can process the first and third tensor,
  // then `tensor_indices` should be {0, 2}.
  explicit Processor(core::TfLiteEngine* engine,
                     const std::initializer_list<int> tensor_indices)
      : engine_(engine), tensor_indices_(tensor_indices) {}

  // Checks if tensor counts and metadata of the model matches what required
  // by the processor in general.
  absl::Status SanityCheck(int num_expected_tensors,
                           bool requires_metadata = true);

 protected:
  // Gets the associated tensor.
  // `i` refers to the element index in `tensor_indices`. For example,
  // assume `tensor_indices` is {3, 6, 8}, to access second tensor in
  // `tensor_indices`, which is the 6th tensor of the model inputs or ourputs,
  // `i` should be 1.
  virtual TfLiteTensor* GetTensor(int i) const = 0;

  // Gets the associated tensor metadata.
  // `i` refers to the element index in `tensor_indices`. For example,
  // assume `tensor_indices` is {3, 6, 8}, to access second tensor in
  // `tensor_indices`, which is the 6th tensor of the model inputs or ourputs,
  // `i` should be 1.
  virtual const tflite::TensorMetadata* GetTensorMetadata(int i = 0) const = 0;

  inline const tflite::metadata::ModelMetadataExtractor* GetMetadataExtractor()
      const {
    return engine_->metadata_extractor();
  }

  // Gets the tesnor indices in string format.
  std::string GetTensorIndexString();

  core::TfLiteEngine* engine_;
  const std::vector<int> tensor_indices_;

 private:
  // Gets the number of input or ourput tensors of the TfLiteEngine that this
  // processor holds.
  virtual int GetModelTensorCount() const = 0;

  // Either "input" or "output".
  virtual const char* GetTensorTypeName() const = 0;
};

// Abstract base class for all Preprocessors.
// Preprocessor is a helper class that converts input structured data (such as
// image) to raw bytes and populates the associated tensors in the
// interpreter.
//
// As a convention, child class needs to implement a factory `Create` method
// to initialize and bind tensors.
//
// Example usage:
// auto processor = MyPreprocessor::Create(
//   /* input_tensors */ {0}, engine, option);
// // Populate the associate tensors.
// processor->Preprocess(...);
class Preprocessor : public Processor {
 protected:
  using Processor::Processor;

  // Get the associated input tensor.
  // Note: Caller is responsible for passing in a valid `i`.
  inline TfLiteTensor* GetTensor(int i = 0) const override {
    return engine_->GetInput(engine_->interpreter(), tensor_indices_.at(i));
  }

  // Get the associated input metadata.
  // Note: Caller is responsible for passing in a valid `i`.
  inline const tflite::TensorMetadata* GetTensorMetadata(
      int i = 0) const override {
    return GetMetadataExtractor()->GetInputTensorMetadata(
        tensor_indices_.at(i));
  }

 private:
  static constexpr char kInputTypeName[] = "input";

  inline int GetModelTensorCount() const override {
    return engine_->InputCount(engine_->interpreter());
  }

  inline const char* GetTensorTypeName() const override {
    return kInputTypeName;
  }
};

// Abstract base class for all Postprocessors.
// Postprocessor is a helper class to convert tensor value to structured
// data.
// As a convention, child class needs to implement a factory `Create` method
// to initialize and bind tensors.
//
// Example usage:
// auto processor = MyPostprocessor::Create(
//   /* output_tensors */ {0}, engine, option);
// // Populate the associate tensors.
// auto value = processor->Postprocess();
class Postprocessor : public Processor {
 protected:
  using Processor::Processor;

  // Get the associated output tensor.
  // Note: Caller is responsible for passing in a valid `i`.
  inline TfLiteTensor* GetTensor(int i = 0) const override {
    return engine_->GetOutput(engine_->interpreter(), tensor_indices_.at(i));
  }

  // Get the associated output metadata.
  // Note: Caller is responsible for passing in a valid `i`.
  inline const tflite::TensorMetadata* GetTensorMetadata(
      int i = 0) const override {
    return GetMetadataExtractor()->GetOutputTensorMetadata(
        tensor_indices_.at(i));
  }

 private:
  static constexpr char kOutputTypeName[] = "output";

  inline int GetModelTensorCount() const override {
    return engine_->OutputCount(engine_->interpreter());
  }

  inline const char* GetTensorTypeName() const override {
    return kOutputTypeName;
  }
};

}  // namespace processor
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_PROCESSOR_PROCESSOR_H_
