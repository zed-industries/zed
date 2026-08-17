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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ARGS_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ARGS_PARSER_H_

#include "src/trace_processor/importers/common/args_tracker.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/proto_to_args_parser.h"

namespace perfetto::trace_processor {

// A ProtoToArgsParser::Delegate that writes the parsed proto data into
// TraceStorage after interning key strings.
class ArgsParser : public util::ProtoToArgsParser::Delegate {
 public:
  using Key = util::ProtoToArgsParser::Key;

  ArgsParser(int64_t packet_timestamp,
             ArgsTracker::BoundInserter& inserter,
             TraceStorage& storage,
             PacketSequenceStateGeneration* sequence_state = nullptr,
             bool support_json = false);
  ~ArgsParser() override;
  void AddInteger(const Key&, int64_t) override;
  void AddUnsignedInteger(const Key&, uint64_t) override;
  void AddString(const Key&, const protozero::ConstChars&) override;
  void AddString(const Key&, const std::string&) override;
  void AddDouble(const Key&, double) override;
  void AddPointer(const Key&, uint64_t) override;
  void AddBoolean(const Key&, bool) override;
  void AddBytes(const Key&, const protozero::ConstBytes&) override;
  bool AddJson(const Key&, const protozero::ConstChars&) override;
  void AddNull(const Key&) override;
  size_t GetArrayEntryIndex(const std::string& array_key) override;
  size_t IncrementArrayEntryIndex(const std::string& array_key) override;
  int64_t packet_timestamp() override;
  PacketSequenceStateGeneration* seq_state() override;

 protected:
  InternedMessageView* GetInternedMessageView(uint32_t field_id,
                                              uint64_t iid) override;

 private:
  const bool support_json_;
  const int64_t packet_timestamp_;
  PacketSequenceStateGeneration* const sequence_state_;
  ArgsTracker::BoundInserter& inserter_;
  TraceStorage& storage_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ARGS_PARSER_H_
