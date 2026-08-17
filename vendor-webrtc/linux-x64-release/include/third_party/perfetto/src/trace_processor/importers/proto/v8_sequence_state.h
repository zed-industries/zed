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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_SEQUENCE_STATE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_SEQUENCE_STATE_H_

#include <cstdint>
#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/v8_tables_py.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;
class V8Tracker;

// Helper class to deal with V8 related interned data.
class V8SequenceState final
    : public PacketSequenceStateGeneration::CustomState {
 public:
  explicit V8SequenceState(TraceProcessorContext* context);

  ~V8SequenceState() override;

  std::optional<tables::V8IsolateTable::Id> GetOrInsertIsolate(uint64_t iid);
  std::optional<tables::V8JsFunctionTable::Id> GetOrInsertJsFunction(
      uint64_t iid,
      tables::V8IsolateTable::Id isolate_id);
  std::optional<tables::V8WasmScriptTable::Id> GetOrInsertWasmScript(
      uint64_t iid,
      tables::V8IsolateTable::Id isolate_id);

 private:
  std::optional<tables::V8JsScriptTable::Id> GetOrInsertJsScript(
      uint64_t iid,
      tables::V8IsolateTable::Id isolate_id);
  std::optional<StringId> GetOrInsertJsFunctionName(uint64_t iid);

  TraceProcessorContext* const context_;
  V8Tracker* const v8_tracker_;

  using InterningId = uint64_t;
  base::FlatHashMap<InterningId, std::optional<tables::V8IsolateTable::Id>>
      isolates_;
  base::FlatHashMap<InterningId, tables::V8JsScriptTable::Id> js_scripts_;
  base::FlatHashMap<InterningId, tables::V8WasmScriptTable::Id> wasm_scripts_;
  base::FlatHashMap<InterningId, tables::V8JsFunctionTable::Id> js_functions_;
  base::FlatHashMap<InterningId, StringId> js_function_names_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_SEQUENCE_STATE_H_
