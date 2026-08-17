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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_LANGUAGE_DETECTOR_H_
#define MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_LANGUAGE_DETECTOR_H_

#include <memory>
#include <string>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/components/processors/classifier_options.h"
#include "mediapipe/tasks/cc/core/base_options.h"
#include "mediapipe/tasks/cc/core/base_task_api.h"

namespace mediapipe::tasks::text::language_detector {

// A language code and its probability.
struct LanguageDetectorPrediction {
  // An i18n language / locale code, e.g. "en" for English, "uz" for Uzbek,
  // "ja"-Latn for Japanese (romaji).
  std::string language_code;

  float probability;
};

// Task output.
using LanguageDetectorResult = std::vector<LanguageDetectorPrediction>;

// The options for configuring a MediaPipe LanguageDetector task.
struct LanguageDetectorOptions {
  // Base options for configuring MediaPipe Tasks, such as specifying the model
  // file with metadata, accelerator options, op resolver, etc.
  tasks::core::BaseOptions base_options;

  // Options for configuring the classifier behavior, such as score threshold,
  // number of results, etc.
  components::processors::ClassifierOptions classifier_options;
};

// Predicts the language of an input text.
//
// This API expects a TFLite model with TFLite Model Metadata that
// contains the mandatory (described below) input tensors, output tensor,
// and the language codes in an AssociatedFile.
//
// Input tensors:
//   (kTfLiteString)
//    - 1 input tensor that is scalar or has shape [1] containing the input
//      string.
// Output tensor:
//   (kTfLiteFloat32)
//    - 1 output tensor of shape`[1 x N]` where `N` is the number of languages.
class LanguageDetector : core::BaseTaskApi {
 public:
  using BaseTaskApi::BaseTaskApi;

  // Creates a LanguageDetector instance from the provided `options`.
  static absl::StatusOr<std::unique_ptr<LanguageDetector>> Create(
      std::unique_ptr<LanguageDetectorOptions> options);

  // Predicts the language of the input `text`.
  absl::StatusOr<LanguageDetectorResult> Detect(absl::string_view text);

  // Shuts down the LanguageDetector instance when all the work is done.
  absl::Status Close() { return runner_->Close(); }
};

}  // namespace mediapipe::tasks::text::language_detector

#endif  // MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_LANGUAGE_DETECTOR_H_
