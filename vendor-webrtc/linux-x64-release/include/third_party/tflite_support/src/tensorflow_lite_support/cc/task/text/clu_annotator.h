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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_CLU_ANNOTATOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_CLU_ANNOTATOR_H_

#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/text/proto/clu_proto_inc.h"

namespace tflite {
namespace task {
namespace text {
namespace clu {

// Interface for a CLU-Annotator API.
class CluAnnotator : public core::BaseTaskApi<CluResponse, const CluRequest&> {
 public:
  explicit CluAnnotator(std::unique_ptr<core::TfLiteEngine> engine)
      : BaseTaskApi(std::move(engine)) {}

  virtual absl::StatusOr<CluResponse> Annotate(const CluRequest& request) = 0;
};

}  // namespace clu
}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_CLU_ANNOTATOR_H_
