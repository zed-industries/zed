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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_SLOT_TAGGING_OUTPUT_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_SLOT_TAGGING_OUTPUT_H_

#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/task/text/proto/clu_proto_inc.h"

namespace tflite::task::text::clu {

// Given the input IOB tags and corresponding confidences and token alignments
// this will populate the slots in CluResponse. For BERT models with history,
// 'token_alignments' is concatenation of all turns, and the turn id is given by
// 'token_turn_ids'.
//   Inputs:
//     tags: IOB tags
//     confidences: array of confidence scores score for each tag.
//     token_alignments: A list of (start, exclusive_end) offsets into the
//       original text.
//     token_turn_ids: A list of int. The turn id of each token.
//     first_subword_indicators: A list of int. Whether the corresponding
//       subword token is the first subword of a natural word or not.
//     threshold: the threshold for slot extraction.
//     reverse_utterance_list_to_encode: the utterance list in the reverse
//       chronological order.
//   Outputs:
//     response
absl::Status SlotModulePopulateResponse(
    const std::vector<absl::string_view>& tags, const float* confidences,
    const std::vector<std::pair<int, int>>& token_alignments,
    const std::vector<int>& token_turn_ids,
    const std::vector<int>& first_subword_indicators, float threshold,
    const std::vector<absl::string_view>& reverse_utterance_list_to_encode,
    CluResponse* response);

}  // namespace tflite::task::text::clu

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_SLOT_TAGGING_OUTPUT_H_
