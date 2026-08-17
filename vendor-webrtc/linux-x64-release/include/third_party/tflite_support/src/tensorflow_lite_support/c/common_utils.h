/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_C_COMMON_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_C_COMMON_UTILS_H_

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/c/common.h"

// Utils for Conversion of absl::Status to TfLiteError
// -----------------------------------------------------------------
// Meant to be used with task C apis.

namespace tflite {
namespace support {

// Creates a TfLiteSupportError with a TfLiteSupportErrorCode and message.
void CreateTfLiteSupportError(enum TfLiteSupportErrorCode code,
                              const char* message, TfLiteSupportError** error);

// Creates a TfLiteSupportError from absl::Status and passes it back as a
// parameter which is a pointer to the error pointer.
//
// Example Usage With Image Classifier
//
// APIs: TfLiteImageClassifier* TfLiteImageClassifierFromOptions(
//     const TfLiteImageClassifierOptions* options,
//     TfLiteSupportError **error) {
// // Necessary checks
// tflite::support::StatusOr<std::unique_ptr<ImageClassifier>> classifier_status
// = // Call to create Cpp Image Classifier.
// if (classifier_status.ok()) {
//     Code to return classifier
// } else {
//     ::tflite::support::CreateTfLiteSupportErrorWithStatus(classifier_status.status(),
//     error);
//     return nullptr;
//  }
//}
void CreateTfLiteSupportErrorWithStatus(const absl::Status& status,
                                        TfLiteSupportError** error);

}  // namespace support
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_C_COMMON_UTILS_H_
