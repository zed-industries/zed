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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TASK_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TASK_UTILS_H_

#include <algorithm>
#include <cstring>
#include <numeric>
#include <vector>

#include "absl/memory/memory.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/str_cat.h"  // from @com_google_absl
#include "absl/strings/str_format.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "flatbuffers/flatbuffers.h"  // from @flatbuffers
#include "tensorflow/lite/kernels/internal/tensor_ctypes.h"
#include "tensorflow/lite/kernels/op_macros.h"
#include "tensorflow/lite/string_util.h"
#include "tensorflow/lite/type_to_tflitetype.h"
#include "tensorflow_lite_support/cc/common.h"
#include "tensorflow_lite_support/cc/port/status_macros.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/metadata/metadata_schema_generated.h"

namespace tflite {
namespace task {
namespace core {

// Checks and returns type of a tensor, fails if tensor type is not T.
template <typename T>
tflite::support::StatusOr<T*> AssertAndReturnTypedTensor(
    const TfLiteTensor* tensor) {
  if (!tensor->data.raw) {
    return tflite::support::CreateStatusWithPayload(
        absl::StatusCode::kInternal,
        absl::StrFormat("Tensor (%s) has no raw data.", tensor->name));
  }

  // Checks if data type of tensor is T and returns the pointer casted to T if
  // applicable, returns nullptr if tensor type is not T.
  // See type_to_tflitetype.h for a mapping from plain C++ type to TfLiteType.
  if (tensor->type == typeToTfLiteType<T>()) {
    return reinterpret_cast<T*>(tensor->data.raw);
  }
  return tflite::support::CreateStatusWithPayload(
      absl::StatusCode::kInternal,
      absl::StrFormat("Type mismatch for tensor %s. Required %d, got %d.",
                      tensor->name, typeToTfLiteType<T>(), tensor->bytes));
}

// Populates tensor with array of data, fails if data type doesn't match tensor
// type or has not the same number of elements.
// Note: std::negation is not used because it is from C++17, where the code will
// be compiled using C++14 in OSS.
template <typename T, typename = std::enable_if_t<
                          std::is_same<T, std::string>::value == false>>
inline absl::Status PopulateTensor(const T* data, int num_elements,
                                   TfLiteTensor* tensor) {
  T* v;
  TFLITE_ASSIGN_OR_RETURN(v, AssertAndReturnTypedTensor<T>(tensor));
  size_t bytes = num_elements * sizeof(T);
  if (tensor->bytes != bytes) {
    return tflite::support::CreateStatusWithPayload(
        absl::StatusCode::kInternal,
        absl::StrFormat("tensor->bytes (%d) != bytes (%d)", tensor->bytes,
                        bytes));
  }
  memcpy(v, data, bytes);
  return absl::OkStatus();
}

// Populates tensor with vector of data, fails if data type doesn't match tensor
// type or has not the same number of elements.
template <typename T>
inline absl::Status PopulateTensor(const std::vector<T>& data,
                                   TfLiteTensor* tensor) {
  return PopulateTensor<T>(data.data(), data.size(), tensor);
}

template <>
inline absl::Status PopulateTensor<std::string>(
    const std::vector<std::string>& data, TfLiteTensor* tensor) {
  if (tensor->type != kTfLiteString) {
    return tflite::support::CreateStatusWithPayload(
        absl::StatusCode::kInternal,
        absl::StrFormat("Type mismatch for tensor %s. Required STRING, got %d.",
                        tensor->name, tensor->bytes));
  }
  tflite::DynamicBuffer input_buf;
  for (const auto& value : data) {
    input_buf.AddString(value.data(), value.length());
  }
  input_buf.WriteToTensorAsVector(tensor);
  return absl::OkStatus();
}

// Populates tensor one data item, fails if data type doesn't match tensor
// type.
template <typename T>
inline absl::Status PopulateTensor(const T& data, TfLiteTensor* tensor) {
  T* v;
  TFLITE_ASSIGN_OR_RETURN(v, AssertAndReturnTypedTensor<T>(tensor));
  *v = data;
  return absl::OkStatus();
}

template <>
inline absl::Status PopulateTensor<std::string>(const std::string& data,
                                                TfLiteTensor* tensor) {
  tflite::DynamicBuffer input_buf;
  input_buf.AddString(data.data(), data.length());
  input_buf.WriteToTensorAsVector(tensor);
  return absl::OkStatus();
}

// Populates a vector from the tensor, fails if data type doesn't match tensor
// type.
template <typename T>
inline absl::Status PopulateVector(const TfLiteTensor* tensor,
                                   std::vector<T>* data) {
  const T* v;
  TFLITE_ASSIGN_OR_RETURN(v, AssertAndReturnTypedTensor<T>(tensor));
  size_t num = tensor->bytes / sizeof(tensor->type);
  data->reserve(num);
  for (size_t i = 0; i < num; i++) {
    data->emplace_back(v[i]);
  }
  return absl::OkStatus();
}

template <>
inline absl::Status PopulateVector<std::string>(
    const TfLiteTensor* tensor, std::vector<std::string>* data) {
  std::string* v;
  TFLITE_ASSIGN_OR_RETURN(v, AssertAndReturnTypedTensor<std::string>(tensor));
  (void)v;
  int num = GetStringCount(tensor);
  data->reserve(num);
  for (int i = 0; i < num; i++) {
    const auto& strref = tflite::GetString(tensor, i);
    data->emplace_back(strref.str, strref.len);
  }
  return absl::OkStatus();
}

// Populates vector to a repeated field.
// Note: std::negation is not used because it is from C++17, where the code will
// be compiled using C++14 in OSS.
template <
    class TRepeatedField, class T = float,
    typename = std::enable_if_t<std::is_same<T, std::string>::value == false>>
inline absl::Status PopulateVectorToRepeated(const TfLiteTensor* tensor,
                                             TRepeatedField* data) {
  const T* v;
  TFLITE_ASSIGN_OR_RETURN(v, AssertAndReturnTypedTensor<T>(tensor));
  size_t num = tensor->bytes / sizeof(tensor->type);
  data->Resize(num, T());
  T* pdata = data->mutable_data();
  for (size_t i = 0; i < num; i++) {
    pdata[i] = v[i];
  }
  return absl::OkStatus();
}

// Returns the reversely sorted indices of a vector.
template <typename T>
std::vector<size_t> ReverseSortIndices(const std::vector<T>& v) {
  std::vector<size_t> idx(v.size());
  std::iota(idx.begin(), idx.end(), 0);

  std::stable_sort(idx.begin(), idx.end(),
                   [&v](size_t i1, size_t i2) { return v[i2] < v[i1]; });

  return idx;
}

// Returns the original (dequantized) value of the 'index'-th element of
// 'tensor.
double Dequantize(const TfLiteTensor& tensor, int index);

// Returns the index-th string from the tensor.
std::string GetStringAtIndex(const TfLiteTensor* labels, int index);

// Loads binary content of a file into a string.
std::string LoadBinaryContent(const char* filename);

// Finds the tensor index of the specified tensor name from a vector of tensors
// by checking the metadata tensor name.
// The range of the return value should be [0, tensor_size). Return -1 if no
// tensor is found by name.
int FindTensorIndexByMetadataName(
    const flatbuffers::Vector<flatbuffers::Offset<TensorMetadata>>*
        tensor_metadata,
    absl::string_view name);

// Finds the tensor index of the specified tensor name from a vector of tensors
// by checking the model tensor name.
// The range of the return value should be [0, tensor_size). Return -1 if no
// tensor is found by name.
template <typename TensorType>
int FindTensorIndexByModelName(const std::vector<TensorType*>& tensors,
                               absl::string_view name) {
  for (int i = 0; i < tensors.size(); i++) {
    TensorType* tensor = tensors[i];
    if (tensor->name == name) {
      return i;
    }
  }
  return -1;
}

// Finds the tensor index of the specified tensor name from a vector of tensors
// by first checking the metadata tensor name, and then the model tensor name.
// The range of the return value should be [0, tensor_size). Return -1 if no
// tensor is found by name.
template <typename TensorType>
int FindTensorIndexByName(
    const std::vector<TensorType*>& tensors,
    const flatbuffers::Vector<flatbuffers::Offset<TensorMetadata>>*
        tensor_metadata,
    absl::string_view metadata_tensor_name,
    absl::string_view model_tensor_name) {
  if (tensor_metadata != nullptr && tensor_metadata->size() == tensors.size()) {
    int index =
        FindTensorIndexByMetadataName(tensor_metadata, metadata_tensor_name);
    if (index > -1) return index;
  }

  return FindTensorIndexByModelName(tensors, model_tensor_name);
}

// Finds the tensor from a vector of tensors with name specified inside
// metadata.
template <typename TensorType>
static TensorType* FindTensorByName(
    const std::vector<TensorType*>& tensors,
    const flatbuffers::Vector<flatbuffers::Offset<TensorMetadata>>*
        tensor_metadata,
    absl::string_view metadata_tensor_name) {
  int index = FindTensorIndexByName(tensors, tensor_metadata,
                                    metadata_tensor_name, absl::string_view());
  return index == -1 ? nullptr : tensors[index];
}

}  // namespace core
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_TASK_UTILS_H_
