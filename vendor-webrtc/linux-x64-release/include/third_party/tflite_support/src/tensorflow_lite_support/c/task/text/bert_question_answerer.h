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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_BERT_QUESTION_ANSWERER_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_BERT_QUESTION_ANSWERER_H_

// --------------------------------------------------------------------------
// C API for BertQuestionAnswerer.
//
// Usage:
// <pre><code>
// // Create the model and interpreter options.
// TfLiteBertQuestionAnswerer* qa_answerer =
//     TfLiteBertQuestionAnswererCreate("/path/to/model.tflite");
//
// // Answer a question.
// TfLiteQaAnswers* answers = TfLiteBertQuestionAnswererAnswer(qa_answerer,
//     question);
//
// // Dispose of the API and QaAnswers objects.
// TfLiteBertQuestionAnswererDelete(qa_answerer);
// TfLiteQaAnswersDelete(answers);

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct TfLiteBertQuestionAnswerer TfLiteBertQuestionAnswerer;

typedef struct TfLiteQaAnswer {
  int start;
  int end;
  float logit;
  char* text;
} TfLiteQaAnswer;

typedef struct TfLiteQaAnswers {
  int size;
  TfLiteQaAnswer* answers;
} TfLiteQaAnswers;

// Creates TfLiteBertQuestionAnswerer from model path, returns nullptr if the
// file doesn't exist or is not a well formatted TFLite model path.
TfLiteBertQuestionAnswerer* TfLiteBertQuestionAnswererCreate(
    const char* model_path);

// Invokes the encapsulated TFLite model and answers a question based on
// context.
TfLiteQaAnswers* TfLiteBertQuestionAnswererAnswer(
    const TfLiteBertQuestionAnswerer* question_answerer, const char* context,
    const char* question);

void TfLiteBertQuestionAnswererDelete(
    TfLiteBertQuestionAnswerer* bert_question_answerer);

void TfLiteQaAnswersDelete(TfLiteQaAnswers* qa_answers);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_BERT_QUESTION_ANSWERER_H_
