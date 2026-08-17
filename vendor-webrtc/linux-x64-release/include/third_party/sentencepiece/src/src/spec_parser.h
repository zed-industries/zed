// Copyright 2016 Google LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.!

#ifndef SPEC_PARSER_H_
#define SPEC_PARSER_H_

#include <string>
#include <vector>

#include "absl/strings/ascii.h"
#include "absl/strings/str_split.h"
#include "sentencepiece_processor.h"
#include "util.h"

namespace sentencepiece {

#define PARSE_STRING(param_name)                   \
  if (name == #param_name) {                       \
    message->set_##param_name(std::string(value)); \
    return util::OkStatus();                       \
  }

#define PARSE_REPEATED_STRING(param_name)                       \
  if (name == #param_name) {                                    \
    for (const std::string& val : util::StrSplitAsCSV(value)) { \
      message->add_##param_name(val);                           \
    }                                                           \
    return util::OkStatus();                                    \
  }

#define PARSE_BYTE(param_name)                             \
  if (name == #param_name) {                               \
    message->set_##param_name(value.data(), value.size()); \
    return util::OkStatus();                               \
  }

#define PARSE_INT32(param_name)                                               \
  if (name == #param_name) {                                                  \
    int32 v;                                                                  \
    if (!string_util::lexical_cast(value, &v))                                \
      return util::StatusBuilder(util::StatusCode::kInvalidArgument, GTL_LOC) \
             << "cannot parse \"" << value << "\" as int.";                   \
    message->set_##param_name(v);                                             \
    return util::OkStatus();                                                  \
  }

#define PARSE_UINT64(param_name)                                              \
  if (name == #param_name) {                                                  \
    uint64 v;                                                                 \
    if (!string_util::lexical_cast(value, &v))                                \
      return util::StatusBuilder(util::StatusCode::kInvalidArgument, GTL_LOC) \
             << "cannot parse \"" << value << "\" as int.";                   \
    message->set_##param_name(v);                                             \
    return util::OkStatus();                                                  \
  }

#define PARSE_DOUBLE(param_name)                                              \
  if (name == #param_name) {                                                  \
    double v;                                                                 \
    if (!string_util::lexical_cast(value, &v))                                \
      return util::StatusBuilder(util::StatusCode::kInvalidArgument, GTL_LOC) \
             << "cannot parse \"" << value << "\" as int.";                   \
    message->set_##param_name(v);                                             \
    return util::OkStatus();                                                  \
  }

#define PARSE_BOOL(param_name)                                                \
  if (name == #param_name) {                                                  \
    bool v;                                                                   \
    if (!string_util::lexical_cast(value.empty() ? "true" : value, &v))       \
      return util::StatusBuilder(util::StatusCode::kInvalidArgument, GTL_LOC) \
             << "cannot parse \"" << value << "\" as bool.";                  \
    message->set_##param_name(v);                                             \
    return util::OkStatus();                                                  \
  }

#define PARSE_ENUM(param_name, map_name)                                      \
  if (name == #param_name) {                                                  \
    const auto it = map_name.find(absl::AsciiStrToUpper(value));              \
    if (it == map_name.end())                                                 \
      return util::StatusBuilder(util::StatusCode::kInvalidArgument, GTL_LOC) \
             << "unknown enumeration value of \"" << value << "\" as "        \
             << #map_name;                                                    \
    message->set_##param_name(it->second);                                    \
    return util::OkStatus();                                                  \
  }

#define PRINT_PARAM(param_name) \
  os << "  " << #param_name << ": " << message.param_name() << "\n";

#define PRINT_REPEATED_STRING(param_name)    \
  for (const auto &v : message.param_name()) \
    os << "  " << #param_name << ": " << v << "\n";

#define PRINT_ENUM(param_name, map_name)               \
  const auto it = map_name.find(message.param_name()); \
  if (it == map_name.end())                            \
    os << "  " << #param_name << ": unknown\n";        \
  else                                                 \
    os << "  " << #param_name << ": " << it->second << "\n";

inline std::string PrintProto(const TrainerSpec& message,
                              absl::string_view name) {
  std::ostringstream os;

  os << name << " {\n";

  PRINT_REPEATED_STRING(input);
  PRINT_PARAM(input_format);
  PRINT_PARAM(model_prefix);

  static const std::map<TrainerSpec::ModelType, std::string> kModelType_Map = {
      {TrainerSpec::UNIGRAM, "UNIGRAM"},
      {TrainerSpec::BPE, "BPE"},
      {TrainerSpec::WORD, "WORD"},
      {TrainerSpec::CHAR, "CHAR"},
  };

  PRINT_ENUM(model_type, kModelType_Map);
  PRINT_PARAM(vocab_size);
  PRINT_REPEATED_STRING(accept_language);
  PRINT_PARAM(self_test_sample_size);
  PRINT_PARAM(character_coverage);
  PRINT_PARAM(input_sentence_size);
  PRINT_PARAM(shuffle_input_sentence);
  PRINT_PARAM(seed_sentencepiece_size);
  PRINT_PARAM(shrinking_factor);
  PRINT_PARAM(max_sentence_length);
  PRINT_PARAM(num_threads);
  PRINT_PARAM(num_sub_iterations);
  PRINT_PARAM(max_sentencepiece_length);
  PRINT_PARAM(split_by_unicode_script);
  PRINT_PARAM(split_by_number);
  PRINT_PARAM(split_by_whitespace);
  PRINT_PARAM(split_digits);
  PRINT_PARAM(pretokenization_delimiter);
  PRINT_PARAM(treat_whitespace_as_suffix);
  PRINT_PARAM(allow_whitespace_only_pieces);
  PRINT_REPEATED_STRING(control_symbols);
  PRINT_REPEATED_STRING(user_defined_symbols);
  PRINT_PARAM(required_chars);
  PRINT_PARAM(byte_fallback);
  PRINT_PARAM(vocabulary_output_piece_score);
  PRINT_PARAM(train_extremely_large_corpus);
  PRINT_PARAM(hard_vocab_limit);
  PRINT_PARAM(use_all_vocab);
  PRINT_PARAM(unk_id);
  PRINT_PARAM(bos_id);
  PRINT_PARAM(eos_id);
  PRINT_PARAM(pad_id);
  PRINT_PARAM(unk_piece);
  PRINT_PARAM(bos_piece);
  PRINT_PARAM(eos_piece);
  PRINT_PARAM(pad_piece);
  PRINT_PARAM(unk_surface);
  PRINT_PARAM(enable_differential_privacy);
  PRINT_PARAM(differential_privacy_noise_level);
  PRINT_PARAM(differential_privacy_clipping_threshold);

  os << "}\n";

  return os.str();
}

inline std::string PrintProto(const NormalizerSpec& message,
                              absl::string_view name) {
  std::ostringstream os;

  os << name << " {\n";

  PRINT_PARAM(name);
  PRINT_PARAM(add_dummy_prefix);
  PRINT_PARAM(remove_extra_whitespaces);
  PRINT_PARAM(escape_whitespaces);
  PRINT_PARAM(normalization_rule_tsv);

  os << "}\n";

  return os.str();
}

util::Status SentencePieceTrainer::SetProtoField(absl::string_view name,
                                                 absl::string_view value,
                                                 TrainerSpec* message) {
  CHECK_OR_RETURN(message);

  PARSE_REPEATED_STRING(input);
  PARSE_STRING(input_format);
  PARSE_STRING(model_prefix);

  static const std::map<std::string, TrainerSpec::ModelType> kModelType_Map = {
      {"UNIGRAM", TrainerSpec::UNIGRAM},
      {"BPE", TrainerSpec::BPE},
      {"WORD", TrainerSpec::WORD},
      {"CHAR", TrainerSpec::CHAR},
  };

  PARSE_ENUM(model_type, kModelType_Map);
  PARSE_INT32(vocab_size);
  PARSE_REPEATED_STRING(accept_language);
  PARSE_INT32(self_test_sample_size);
  PARSE_DOUBLE(character_coverage);
  PARSE_UINT64(input_sentence_size);
  PARSE_BOOL(shuffle_input_sentence);
  PARSE_INT32(seed_sentencepiece_size);
  PARSE_DOUBLE(shrinking_factor);
  PARSE_INT32(max_sentence_length);
  PARSE_INT32(num_threads);
  PARSE_INT32(num_sub_iterations);
  PARSE_INT32(max_sentencepiece_length);
  PARSE_BOOL(split_by_unicode_script);
  PARSE_BOOL(split_by_number);
  PARSE_BOOL(split_by_whitespace);
  PARSE_BOOL(split_digits);
  PARSE_STRING(pretokenization_delimiter);
  PARSE_BOOL(treat_whitespace_as_suffix);
  PARSE_BOOL(allow_whitespace_only_pieces);
  PARSE_REPEATED_STRING(control_symbols);
  PARSE_REPEATED_STRING(user_defined_symbols);
  PARSE_STRING(required_chars);
  PARSE_BOOL(byte_fallback);
  PARSE_BOOL(hard_vocab_limit);
  PARSE_BOOL(vocabulary_output_piece_score);
  PARSE_BOOL(train_extremely_large_corpus);
  PARSE_BOOL(use_all_vocab);
  PARSE_INT32(unk_id);
  PARSE_INT32(bos_id);
  PARSE_INT32(eos_id);
  PARSE_INT32(pad_id);
  PARSE_STRING(unk_piece);
  PARSE_STRING(bos_piece);
  PARSE_STRING(eos_piece);
  PARSE_STRING(pad_piece);
  PARSE_STRING(unk_surface);
  PARSE_BOOL(enable_differential_privacy);
  PARSE_DOUBLE(differential_privacy_noise_level);
  PARSE_UINT64(differential_privacy_clipping_threshold);

  return util::StatusBuilder(util::StatusCode::kNotFound, GTL_LOC)
         << "unknown field name \"" << name << "\" in TrainerSpec.";
}

util::Status SentencePieceTrainer::SetProtoField(absl::string_view name,
                                                 absl::string_view value,
                                                 NormalizerSpec* message) {
  CHECK_OR_RETURN(message);

  PARSE_STRING(name);
  PARSE_BYTE(precompiled_charsmap);
  PARSE_BOOL(add_dummy_prefix);
  PARSE_BOOL(remove_extra_whitespaces);
  PARSE_BOOL(escape_whitespaces);
  PARSE_STRING(normalization_rule_tsv);

  return util::StatusBuilder(util::StatusCode::kNotFound, GTL_LOC)
         << "unknown field name \"" << name << "\" in NormalizerSpec.";
}

#undef PARSE_STRING
#undef PARSE_REPEATED_STRING
#undef PARSE_BOOL
#undef PARSE_BYTE
#undef PARSE_INT32
#undef PARSE_DUOBLE
#undef PARSE_ENUM
#undef PRINT_MAP
#undef PRINT_REPEATED_STRING
#undef PRINT_ENUM
}  // namespace sentencepiece

#endif  // SPEC_PARSER_H_
