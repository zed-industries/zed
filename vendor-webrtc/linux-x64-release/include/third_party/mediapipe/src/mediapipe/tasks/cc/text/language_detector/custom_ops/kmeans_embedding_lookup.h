/* Copyright 2023 The MediaPipe Authors.

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

// This op was originally written by the Learn2Compress team.
// It takes in a list of indices, an encoding table which consists of
// integer indices into a codebook with floating point vectors.
// For each index, it looks up the corresponding row in the encoding table and
// for each entry in the row of the encoding table, it looks up the
// corresponding row in the codebook and populates it in an output embedding.
// The average of the output embeddings for each of the input indices is the
// output of this op.

#ifndef MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_KMEANS_EMBEDDING_LOOKUP_H_
#define MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_KMEANS_EMBEDDING_LOOKUP_H_

#include "tensorflow/lite/kernels/register.h"

namespace mediapipe::tflite_operations {

TfLiteRegistration* Register_KmeansEmbeddingLookup();

}  // namespace mediapipe::tflite_operations

#endif  // MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_KMEANS_EMBEDDING_LOOKUP_H_
