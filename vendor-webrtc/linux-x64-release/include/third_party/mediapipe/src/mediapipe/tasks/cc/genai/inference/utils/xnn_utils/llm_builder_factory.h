// Copyright 2024 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_BUILDER_FACTORY_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_BUILDER_FACTORY_H_

#include <memory>

#include "absl/status/statusor.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/graph_builder.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/llm.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/llm_weights.h"
#include "mediapipe/tasks/cc/genai/inference/utils/xnn_utils/sampling.h"

namespace mediapipe::tasks::genai::xnn_utils {

absl::StatusOr<std::unique_ptr<Llm>> CreateLlm(
    const LlmParams& llm_params,
    std::unique_ptr<RuntimeConfigs> runtime_configs,
    std::unique_ptr<LlmWeightsLoader> weight_loader,
    std::unique_ptr<Sampler> sampler,
    odml::infra::proto::LlmModelType model_type);

}  // namespace mediapipe::tasks::genai::xnn_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_XNN_UTILS_LLM_BUILDER_FACTORY_H_
