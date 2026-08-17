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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_NL_CLASSIFIER_COMMON_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_NL_CLASSIFIER_COMMON_H_

// C API for the NLClassifier results, Catergory.

// TODO(b/197355311): deprecate this class and use the unified one with image
// and audio.

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct Category {
  char* text;
  double score;
} Category;

typedef struct Categories {
  int size;
  Category* categories;
} Categories;

void TfLiteNLClassifierCategoriesDelete(Categories* categories);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_TEXT_NL_CLASSIFIER_COMMON_H_
