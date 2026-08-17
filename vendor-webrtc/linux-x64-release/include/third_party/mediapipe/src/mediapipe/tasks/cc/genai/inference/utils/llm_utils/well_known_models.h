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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_WELL_KNOWN_MODELS_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_WELL_KNOWN_MODELS_H_

#include "mediapipe/tasks/cc/genai/inference/proto/llm_params.pb.h"

namespace mediapipe::tasks::genai::llm_utils {

odml::infra::proto::LlmParameters GetFalconRW1BParams();
odml::infra::proto::LlmParameters GetGemma2BParams();
odml::infra::proto::LlmParameters GetGemma7BParams();
odml::infra::proto::LlmParameters GetGemma2_2BParams();
odml::infra::proto::LlmParameters GetStablelm4E1T3BParams();
odml::infra::proto::LlmParameters GetPhi2Params();

}  // namespace mediapipe::tasks::genai::llm_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_WELL_KNOWN_MODELS_H_
