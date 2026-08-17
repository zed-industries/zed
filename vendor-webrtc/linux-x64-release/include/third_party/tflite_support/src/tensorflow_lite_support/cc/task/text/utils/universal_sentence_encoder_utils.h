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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_UTILS_UNIVERSAL_SENTENCE_ENCODER_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_UTILS_UNIVERSAL_SENTENCE_ENCODER_UTILS_H_

#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"

namespace tflite {
namespace task {
namespace text {

// Returns the input tensor indices for a Universal Sentence Encoder QA model in
// this order: query text, response context, response text.
//
// The model is expected to contain input tensors with names:
//
// Tensor           | Metadata Name      | Tensor Name
// ---------------- | ------------------ | -------------------------------
// Query text       | "inp_text"         | "ParseExample/ParseExampleV2:1"
// Response context | "res_context"      | "ParseExample/ParseExampleV2:2"
// Response text    | "res_text"         | "ParseExample/ParseExampleV2:3"
//
// Tensors will be matched by first checking the metadata tensor name and then
// the Model tensor name. If no matching tensor name is found, the first three
// input tensors will be used for query text, response context, response text,
// respectively. Other input tensors will be ignored.
tflite::support::StatusOr<std::vector<int>>
GetUniversalSentenceEncoderInputTensorIndices(
    tflite::task::core::TfLiteEngine* engine);

// Returns the output tensor indices for a Universal Sentence Encoder QA model
// in this order: query encoding, response encoding.
//
// The model is expected to contain output tensors with names:
//
//   - Query encoding     "query_encoding"   | "Final/EncodeQuery/mul"
//   - Response encoding  "response_encoding"| "Final/EncodeResult/mul"
//
// Tensors will be matched by first checking the metadata tensor name and then
// the Model tensor name. If no matching tensor name is found, the first two
// output tensors will be used for query encoding and response encoding,
// respectively. Other output tensors will be ignored.
tflite::support::StatusOr<std::vector<int>>
GetUniversalSentenceEncoderOutputTensorIndices(
    tflite::task::core::TfLiteEngine* engine);

}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_UTILS_UNIVERSAL_SENTENCE_ENCODER_UTILS_H_
