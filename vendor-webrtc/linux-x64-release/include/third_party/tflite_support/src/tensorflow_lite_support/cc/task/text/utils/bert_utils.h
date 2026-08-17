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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_UTILS_BERT_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_UTILS_BERT_UTILS_H_

#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"

namespace tflite {
namespace task {
namespace text {

// Returns the input tensor indices for a Bert model in this order: ids, segment
// ids, mask.
//
// The model is expected to contain input tensors with names:
//
// Tensor           | Metadata Name
// ---------------- | --------------
// IDs              | "ids"
// Segment IDs      | "segment_ids"
// Mask             | "mask"
//
// If no matching tensor is found, the first three input tensors will be used
// for ids, segment ids and mask respectively.
tflite::support::StatusOr<std::vector<int>> GetBertInputTensorIndices(
    tflite::task::core::TfLiteEngine* engine);

}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_UTILS_BERT_UTILS_H_
