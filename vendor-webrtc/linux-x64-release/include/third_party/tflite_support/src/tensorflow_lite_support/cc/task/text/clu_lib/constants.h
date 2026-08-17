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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_CONSTANTS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_CONSTANTS_H_

namespace tflite::task::text::clu {

// Whether to do lower case before Bert Tokenization.
inline constexpr bool kUseLowerCase = true;

// Tokens used in BERT model.
inline constexpr char kClsToken[] = "[CLS]";
inline constexpr char kSepToken[] = "[SEP]";
inline constexpr char kUnkToken[] = "[UNK]";
// The padding symbol for BERT WordpieceTokenizer.
inline constexpr char kWordpiecePadToken[] = "[PAD]";
// IOB tags
inline constexpr char kSlotOTag[] = "O";
inline constexpr char kSlotBTagPrefix[] = "B-";
inline constexpr char kSlotITagPrefix[] = "I-";

inline constexpr char kNamespaceDelim[] = "~~";

// CLU features
struct CLUFeature {
  static constexpr char kDialogId[] = "dialog_id";
  static constexpr char kTurnIndex[] = "turn_index";
  static constexpr char kUtteranceSeq[] = "utterance_seq";
  static constexpr char kUtteranceTokenIdSeq[] = "utterance_token_id_seq";
  static constexpr char kUtteranceFirstSubwordSeq[] =
      "utterance_first_subword_seq";
  static constexpr char kUtteranceSegmentIdSeq[] = "utterance_segment_id_seq";
  static constexpr char kUtteranceTurnIdSeq[] = "utterance_turn_id_seq";
  static constexpr char kUtteranceCharSeq[] = "utterance_char_seq";
  static constexpr char kUtteranceEngineeredSeq[] = "utterance_engineered_seq";
  static constexpr char kSeqLength[] = "seq_length";
  static constexpr char kWord[] = "word";
  static constexpr char kRawUtterance[] = "raw_utterance";
  static constexpr char kTokenAlignmentSeq[] = "token_alignment_seq";
  static constexpr char kSlotTagSeq[] = "slot_tag_seq";
  static constexpr char kIntents[] = "intents";
  static constexpr char kDomain[] = "domain";
  static constexpr char kPosTagSeq[] = "pos_tag_seq";
  // Indicator features denoting whether the domain/intent/slot annotations are
  // present in the example.
  static constexpr char kHasDomain[] = "has_domain";
  static constexpr char kHasIntents[] = "has_intents";
  static constexpr char kHasSlots[] = "has_slots";
  // Annotated Span features
  static constexpr char kAnnotatedSpanSeq[] = "annotated_span_seq";
  static constexpr char kAnnotatedSpanScoreSeq[] = "annotated_span_score_seq";
  static constexpr char kAnnotatedSpanTimeDurationSeq[] =
      "annotated_span_time_duration_seq";
  static constexpr char kAnnotatedSpanTimeOfDaySeq[] =
      "annotated_span_time_of_day_seq";
  static constexpr char kAnnotatedSpanDayOfWeekSeq[] =
      "annotated_span_day_of_week_seq";
  static constexpr char kAnnotatedSpanAppTypeSeq[] =
      "annotated_span_app_type_seq";
  static constexpr char kAnnotatedSpanAppLocaleSeq[] =
      "annotated_span_app_locale_seq";
  static constexpr char kAnnotatedSpanAppCategorySeq[] =
      "annotated_span_app_category_seq";
  // End of Annotated Span features
  static constexpr char kNumHistoryTurns[] = "num_history_turns";
  static constexpr char kHistorySeq[] = "history_seq_";
  static constexpr char kHistorySeqLength[] = "history_seq_length_";
  static constexpr char kSurfaceType[] = "surface_type";
};

}  // namespace tflite::task::text::clu

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_CONSTANTS_H_
