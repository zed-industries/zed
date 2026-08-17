// Copyright 2024 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_SPAN_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_SPAN_H_

#include <utility>
#include <vector>

#include "mediapipe/framework/formats/tensor.h"

namespace mediapipe {

// Utility class to allow for iterating over various containers of Tensors
// *without* making any deep-copies or keeping any memory alive. Essentially
// this is like absl::Span<T> except that we do not care about the type of T as
// long as it can be used to extract references or pointers to Tensors.
// TODO: Extend to have both const and mutable variants.
class TensorSpan {
 public:
  TensorSpan() = default;
  explicit TensorSpan(std::vector<const Tensor*>&& tensor_refs);

  // Accessors
  // We pattern this after std::vector so that the syntax is familiar to users.
  // TODO: Would be nice to extend this to have an iterator too.
  int size() const;
  const Tensor& operator[](int index) const;

 private:
  std::vector<const Tensor*> tensor_refs_;
};

// Supported factory functions:
// Makes a TensorSpan from a memory-owning vector of Tensors
TensorSpan MakeTensorSpan(const std::vector<Tensor>& tensors);

// Makes a TensorSpan from a collection of input streams of Tensors, using the
// api2 framework. Example usage:
// ```
//   static constexpr Input<Tensor>::Multiple kInTensor{"TENSOR"};
//   ... check for any empty input Tensors and handle accordingly ...
//   MakeTensorSpan(kInTensor(cc));
// ```
template <typename TensorInputStreamT>
TensorSpan MakeTensorSpan(const TensorInputStreamT& tensor_streams) {
  std::vector<const Tensor*> refs;
  const int num_tensors = tensor_streams.Count();
  refs.reserve(num_tensors);
  for (int i = 0; i < num_tensors; ++i) {
    refs.push_back(&(*tensor_streams[i]));
  }
  return TensorSpan(std::move(refs));
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_SPAN_H_
