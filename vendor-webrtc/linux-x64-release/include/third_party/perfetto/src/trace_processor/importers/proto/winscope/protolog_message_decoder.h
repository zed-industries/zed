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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_PROTOLOG_MESSAGE_DECODER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_PROTOLOG_MESSAGE_DECODER_H_

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/winscope_tables_py.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

enum ProtoLogLevel : int32_t {
  DEBUG = 1,
  VERBOSE = 2,
  INFO = 3,
  WARN = 4,
  ERROR = 5,
  WTF = 6,
};

struct DecodedMessage {
  ProtoLogLevel log_level;
  std::string group_tag;
  std::string message;
  std::optional<std::string> location;
};

struct TrackedGroup {
  std::string tag;
};

struct TrackedMessage {
  ProtoLogLevel level;
  uint32_t group_id;
  std::string message;
  std::optional<std::string> location;
};

class ProtoLogMessageDecoder : public Destructible {
 public:
  explicit ProtoLogMessageDecoder(TraceProcessorContext* context);
  virtual ~ProtoLogMessageDecoder() override;

  static ProtoLogMessageDecoder* GetOrCreate(TraceProcessorContext* context) {
    if (!context->protolog_message_decoder) {
      context->protolog_message_decoder.reset(
          new ProtoLogMessageDecoder(context));
    }
    return static_cast<ProtoLogMessageDecoder*>(
        context->protolog_message_decoder.get());
  }

  std::optional<DecodedMessage> Decode(
      uint64_t message_id,
      const std::vector<int64_t>& sint64_params,
      const std::vector<double>& double_params,
      const std::vector<bool>& boolean_params,
      const std::vector<std::string>& string_params);

  void TrackGroup(uint32_t id, const std::string& tag);

  void TrackMessage(uint64_t message_id,
                    ProtoLogLevel level,
                    uint32_t group_id,
                    const std::string& message,
                    const std::optional<std::string>& location);

 private:
  TraceProcessorContext* const context_;
  base::FlatHashMap<uint64_t, TrackedGroup> tracked_groups_;
  base::FlatHashMap<uint64_t, TrackedMessage> tracked_messages_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_PROTOLOG_MESSAGE_DECODER_H_
