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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_BERT_NL_CLASSIFIER_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_BERT_NL_CLASSIFIER_H_

#include "tensorflow_lite_support/c/task/text/nl_classifier_common.h"
// --------------------------------------------------------------------------
// C API for BertNLClassifier.
//
// Usage:
// // Create the model and interpreter options.
// TfLiteBertNLClassifier* classifier =
//     TfLiteBertNLClassifierCreate("/path/to/model.tflite");
//
// // Classification.
// Categories* categories = TfLiteBertNLClassifierClassify(classifier,
//     question);
//
// // Dispose of the API object.
// TfLiteBertNLClassifierDelete(classifier);

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct TfLiteBertNLClassifier TfLiteBertNLClassifier;

typedef struct TfLiteBertNLClassifierOptions {
  // Max number of tokens to pass to the model.
  //
  // Deprecated: max_seq_len is now read from the model (i.e. input tensor size)
  // automatically.
  int max_seq_len;
} TfLiteBertNLClassifierOptions;

// Creates TfLiteBertNLClassifier from model path and options, returns nullptr
// if the file doesn't exist or is not a well formatted TFLite model path.
TfLiteBertNLClassifier* TfLiteBertNLClassifierCreateFromOptions(
    const char* model_path, const TfLiteBertNLClassifierOptions* options);

// Creates TfLiteBertNLClassifier from model path and default options, returns
// nullptr if the file doesn't exist or is not a well formatted TFLite model
// path.
TfLiteBertNLClassifier* TfLiteBertNLClassifierCreate(const char* model_path);

// Invokes the encapsulated TFLite model and classifies the input text.
Categories* TfLiteBertNLClassifierClassify(
    const TfLiteBertNLClassifier* classifier, const char* text);

void TfLiteBertNLClassifierDelete(TfLiteBertNLClassifier* classifier);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_BERT_NL_CLASSIFIER_H_
