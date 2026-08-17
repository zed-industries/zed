/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_TFLITE_TEST_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_TFLITE_TEST_UTILS_H_

#include <initializer_list>
#include <string>

#include "tensorflow/lite/c/common.h"
#include "tensorflow/lite/type_to_tflitetype.h"

namespace tflite::task::text::clu {

// A wrapper around TfLiteTensor which frees it in its dtor.
class UniqueTfLiteTensor {
 public:
  explicit UniqueTfLiteTensor(TfLiteTensor* tensor) : tensor_(tensor) {}

  UniqueTfLiteTensor() = default;

  // Returns the underlying pointer
  TfLiteTensor* get();

  TfLiteTensor& operator*() { return *tensor_; }

  TfLiteTensor* operator->() { return tensor_; }

  const TfLiteTensor* operator->() const { return tensor_; }

  // Resets the underlying pointer
  void reset(TfLiteTensor* tensor) { tensor_ = tensor; }

  // Deallocates the tensor as well
  ~UniqueTfLiteTensor();

 private:
  TfLiteTensor* tensor_ = nullptr;
};

template <typename T>
void PopulateTfLiteTensorValue(const std::initializer_list<T> values,
                               TfLiteTensor* tensor) {
  T* buffer = reinterpret_cast<T*>(tensor->data.raw);
  int i = 0;
  for (const auto v : values) {
    buffer[i++] = v;
  }
}

size_t NumTotalFromShape(const std::initializer_list<int>& shape);

template <>
void PopulateTfLiteTensorValue<std::string>(
    const std::initializer_list<std::string> values, TfLiteTensor* tensor);

template <typename T>
TfLiteType TypeToTfLiteType() {
  return tflite::typeToTfLiteType<T>();
}

template <>
TfLiteType TypeToTfLiteType<std::string>();

template <typename T>
void ReallocDynamicTensor(const std::initializer_list<int> shape,
                          TfLiteTensor* tensor) {
  TfLiteTensorFree(tensor);
  tensor->allocation_type = kTfLiteDynamic;
  tensor->type = TypeToTfLiteType<T>();
  // Populate Shape
  TfLiteIntArray* shape_arr = TfLiteIntArrayCreate(shape.size());
  int i = 0;
  const size_t num_total = NumTotalFromShape(shape);
  for (const int dim : shape) shape_arr->data[i++] = dim;
  tensor->dims = shape_arr;
  if (tensor->type != kTfLiteString) {
    TfLiteTensorRealloc(num_total * sizeof(T), tensor);
  }
}

template <typename T>
void PopulateTfLiteTensor(const std::initializer_list<T> values,
                          const std::initializer_list<int> shape,
                          TfLiteTensor* tensor) {
  const size_t num_total = NumTotalFromShape(shape);
  CHECK_EQ(num_total, values.size());
  // Populate Shape
  ReallocDynamicTensor<T>(shape, tensor);
  // Value allocation
  PopulateTfLiteTensorValue<T>(values, tensor);
}

}  // namespace tflite::task::text::clu

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_TFLITE_TEST_UTILS_H_
