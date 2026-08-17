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

#ifndef MEDIAPIPE_TASKS_C_TEXT_LANGUAGE_DETECTOR_LANGUAGE_DETECTOR_H_
#define MEDIAPIPE_TASKS_C_TEXT_LANGUAGE_DETECTOR_LANGUAGE_DETECTOR_H_

#include "mediapipe/tasks/c/components/processors/classifier_options.h"
#include "mediapipe/tasks/c/core/base_options.h"

#ifndef MP_EXPORT
#define MP_EXPORT __attribute__((visibility("default")))
#endif  // MP_EXPORT

#ifdef __cplusplus
extern "C" {
#endif

// A language code and its probability.
struct LanguageDetectorPrediction {
  // An i18n language / locale code, e.g. "en" for English, "uz" for Uzbek,
  // "ja"-Latn for Japanese (romaji).
  char* language_code;

  float probability;
};

// Task output.
struct LanguageDetectorResult {
  struct LanguageDetectorPrediction* predictions;

  // The count of predictions.
  uint32_t predictions_count;
};

// The options for configuring a MediaPipe language detector task.
struct LanguageDetectorOptions {
  // Base options for configuring MediaPipe Tasks, such as specifying the model
  // file with metadata, accelerator options, op resolver, etc.
  struct BaseOptions base_options;

  // Options for configuring the detector behavior, such as score threshold,
  // number of results, etc.
  struct ClassifierOptions classifier_options;
};

// Creates a LanguageDetector from the provided `options`.
// Returns a pointer to the language detector on success.
// If an error occurs, returns `nullptr` and sets the error parameter to an
// an error message (if `error_msg` is not `nullptr`). You must free the memory
// allocated for the error message.
MP_EXPORT void* language_detector_create(
    struct LanguageDetectorOptions* options, char** error_msg);

// Performs language detection on the input `text`. Returns `0` on success.
// If an error occurs, returns an error code and sets the error parameter to an
// an error message (if `error_msg` is not `nullptr`). You must free the memory
// allocated for the error message.
MP_EXPORT int language_detector_detect(void* detector, const char* utf8_str,
                                       struct LanguageDetectorResult* result,
                                       char** error_msg);

// Frees the memory allocated inside a LanguageDetectorResult result. Does not
// free the result pointer itself.
MP_EXPORT void language_detector_close_result(
    struct LanguageDetectorResult* result);

// Shuts down the LanguageDetector when all the work is done. Frees all memory.
// If an error occurs, returns an error code and sets the error parameter to an
// an error message (if `error_msg` is not `nullptr`). You must free the memory
// allocated for the error message.
MP_EXPORT int language_detector_close(void* detector, char** error_msg);

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_TEXT_LANGUAGE_DETECTOR_LANGUAGE_DETECTOR_H_
