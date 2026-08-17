/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_VIEWCAPTURE_ARGS_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_VIEWCAPTURE_ARGS_PARSER_H_

#include <optional>

#include "src/trace_processor/importers/proto/args_parser.h"

namespace perfetto {
namespace trace_processor {

// Specialized args parser to de-intern ViewCapture strings
class ViewCaptureArgsParser : public ArgsParser {
 public:
  using Key = ArgsParser::Key;
  using IidToStringMap = base::FlatHashMap<uint64_t, StringId>;

  ViewCaptureArgsParser(int64_t packet_timestamp,
                        ArgsTracker::BoundInserter& inserter,
                        TraceStorage& storage,
                        PacketSequenceStateGeneration* sequence_state);
  void AddInteger(const Key&, int64_t) override;
  void AddUnsignedInteger(const Key&, uint64_t) override;

  base::FlatHashMap<StringId, IidToStringMap> flat_key_to_iid_args;

 private:
  bool TryAddDeinternedString(const Key&, uint64_t);
  std::optional<protozero::ConstChars> TryDeinternString(const Key&, uint64_t);

  const base::StringView ERROR_MSG{"STRING DE-INTERNING ERROR"};
  TraceStorage& storage_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_VIEWCAPTURE_ARGS_PARSER_H_
