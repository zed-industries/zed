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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_NL_CLASSIFIER_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_NL_CLASSIFIER_H_

#include "tensorflow_lite_support/c/task/text/nl_classifier_common.h"
// --------------------------------------------------------------------------
// C API for NLClassifier.
//
// Usage:
// // Create the model and interpreter options.
// TfLiteNLClassifier* classifier = TfLiteNLClassifierCreate(
//     "/path/to/model.tflite");
//
// // Classification.
// Categories* categories = TfLiteNLClassifierClassify(classifier, question);
//
// // Dispose of the API object.
// TfLiteNLClassifierDelete(classifier);

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct TfLiteNLClassifier TfLiteNLClassifier;

typedef struct TfLiteNLClassifierOptions {
  int input_tensor_index;
  int output_score_tensor_index;
  int output_label_tensor_index;
  const char* input_tensor_name;
  const char* output_score_tensor_name;
  const char* output_label_tensor_name;
} TfLiteNLClassifierOptions;

// Creates TfLiteNLClassifier from model path and options, returns nullptr if
// the file doesn't exist or is not a well formatted TFLite model path.
TfLiteNLClassifier* TfLiteNLClassifierCreateFromOptions(
    const char* model_path, const TfLiteNLClassifierOptions* options);

// Invokes the encapsulated TFLite model and classifies the input text.
Categories* TfLiteNLClassifierClassify(const TfLiteNLClassifier* classifier,
                                       const char* text);

void TfLiteNLClassifierDelete(TfLiteNLClassifier* classifier);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_NL_CLASSIFIER_H_
