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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_BERT_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_BERT_UTILS_H_

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/text/tokenizers/bert_tokenizer.h"

namespace tflite::task::text::clu {
// Processes input for BERT modeling.
//
// Given the current turn and the conversation history as list of utterances
// in the reverse chronological order, starting from the current turn,
// it does the following:
// * tokenizes the utterance of each turn,
// * concatenates all turns to form the input sequence for BERT,
// * truncates (if necessary) the input, and
// * adds CLS/SEP sentinels.
//
// If truncation is needed, it truncates the utterance based on natural word
// unit. That is, a natural word is either kept or truncated entirely.
//
// Inputs:
// - tokenizer: the tokenizer. TODO(xysong): Upgrade SentencePiece tokenizer to
//   return subword_indicator in order to be used here.
// - utterances_in_reverse_order: the pairs of utterance in the dialogue in the
//   reverse chronological order, starting from the
//   current turn.
// - max_seq_length: the max sequence length of BERT model.
// - max_history_turns: the max number of history turns that are encoded, in
//   addition to the current turn. So if 'max_history_turns=2', then there are 3
//   turns encoded in total.
//
// The result is stored in 'out_token_id_and_indicator_list',
// 'out_segment_id_list', 'out_turn_id_list', consisting of five vectors of the
// same length:
// - token_ids: [CLS] token ids of the current turn [SEP] token ids of the
//   previous turn [SEP] token ids of the turn before previous [SEP] ...
// - token_alignments: the span of the corresponding token in their original
//   utterance of the turn that it belongs to.
// - first_subword_indicators: 0 or 1s, the first subword indicator of the
//   corresponding token. 1 means the corresponding token is the first subword
//   of a natural word. 0 means the corresponding token is not the first subword
//   of a natural word. Note that for [CLS], [SEP], the first subword indicator
//   is always 0.
// - segment_ids: 0 or 1s. 0 means the corresponding token belongs to the first
//   sentence segment for BERT. 1 means that the corresponding token belongs to
//   the second sentence segment for BERT. This vector feeds to BERT as the
//   segment_ids (a.k.a. type_ids).
// - turn_ids: the turn index of the corresponding token, like 0 0 0 1 1 1 2
//   2 2. This can be used to determine which turn a token belongs to. [CLS]
//   belongs to turn 0. [SEP] belongs to the turn that it follows.
//
// An important case is that when truncation is necessary (i.e., if the combined
// length of all turns plus satinels exceeds 'max_seq_len'), the earliest turns
// get truncated first, then later turns, then finally the current turn. This is
// different from normal BERT practice where the longer sequence is always get
// truncated. The reason is that the more recent turns are often more important
// in dialogue.
//
// In addition, the utterances are truncated first before satinels (CLS/SEP) are
// added. There is always an [SEP] after each turn.
//
// This function does *not* do padding. The result contains the real tokens of
// the input. Padding is done later in TF graph and TFLite preprocessing. After
// modeling, we will do post-processing to extract slots from those real tokens.
absl::Status BertPreprocessing(
    const tflite::support::text::tokenizer::BertTokenizer* tokenizer,
    const std::vector<absl::string_view>& utterances_in_reverse_order,
    int max_seq_length, int max_history_turns, std::vector<int>* out_token_ids,
    std::vector<std::pair<int, int>>* out_token_alignments,
    std::vector<int>* out_token_first_subword_indicators,
    std::vector<int>* out_segment_id_list, std::vector<int>* out_turn_id_list);

}  // namespace tflite::task::text::clu

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_BERT_UTILS_H_
