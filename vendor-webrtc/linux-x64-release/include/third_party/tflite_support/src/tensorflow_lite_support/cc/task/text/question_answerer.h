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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_QA_QUESTION_ANSWERER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_QA_QUESTION_ANSWERER_H_

#include <string>
#include <utility>
#include <vector>

#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"

namespace tflite {
namespace task {
namespace text {

// Struct for the Answer to QuestionAnswerer.
struct QaAnswer {
  // struct to represent the logit and offset of the answer related to context.
  struct Pos {
    Pos(int arg_start, int arg_end, float arg_logit)
        : start(arg_start), end(arg_end), logit(arg_logit) {}
    int start, end;
    float logit;
    bool operator<(const Pos& rhs) const { return rhs.logit < logit; }
  };

  QaAnswer(std::string arg_text, Pos arg_pos)
      : text(std::move(arg_text)), pos(arg_pos) {}
  std::string text;
  Pos pos;
};

// Interface for an Question-Answer API.
class QuestionAnswerer
    : public core::BaseTaskApi<std::vector<QaAnswer>, const std::string&,
                               const std::string&> {
 public:
  explicit QuestionAnswerer(std::unique_ptr<core::TfLiteEngine> engine)
      : BaseTaskApi(std::move(engine)) {}

  virtual std::vector<QaAnswer> Answer(const std::string& context,
                                       const std::string& question) = 0;
};

}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_QA_QUESTION_ANSWERER_H_
