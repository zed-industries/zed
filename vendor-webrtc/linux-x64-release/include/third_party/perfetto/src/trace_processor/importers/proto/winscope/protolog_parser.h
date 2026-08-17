/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_PROTOLOG_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_PROTOLOG_PARSER_H_

#include <cstdint>
#include <string>
#include <vector>

#include "protos/perfetto/trace/android/protolog.pbzero.h"
#include "src/trace_processor/importers/proto/winscope/protolog_message_decoder.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/util/descriptors.h"
#include "src/trace_processor/util/proto_to_args_parser.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class ProtoLogParser {
 public:
  explicit ProtoLogParser(TraceProcessorContext*);
  void ParseProtoLogMessage(PacketSequenceStateGeneration* sequence_state,
                            protozero::ConstBytes,
                            int64_t timestamp);
  void ParseAndAddViewerConfigToMessageDecoder(protozero::ConstBytes);

 private:
  void PopulateReservedRowWithMessage(tables::ProtoLogTable::Id table_row_id,
                                      ProtoLogLevel level,
                                      std::string& group_tag,
                                      std::string& formatted_message,
                                      std::optional<StringId> stacktrace,
                                      std::optional<std::string>& location);

  TraceProcessorContext* const context_;
  util::ProtoToArgsParser args_parser_;

  const StringId log_level_debug_string_id_;
  const StringId log_level_verbose_string_id_;
  const StringId log_level_info_string_id_;
  const StringId log_level_warn_string_id_;
  const StringId log_level_error_string_id_;
  const StringId log_level_wtf_string_id_;
  const StringId log_level_unknown_string_id_;
};
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_PROTOLOG_PARSER_H_
