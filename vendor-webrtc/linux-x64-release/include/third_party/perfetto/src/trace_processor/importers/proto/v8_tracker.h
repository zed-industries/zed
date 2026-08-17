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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_TRACKER_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/protozero/field.h"
#include "protos/perfetto/trace/chrome/v8.pbzero.h"
#include "src/trace_processor/importers/common/address_range.h"
#include "src/trace_processor/importers/proto/jit_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/v8_tables_py.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceStorage;
class UserMemoryMapping;

using IsolateId = tables::V8IsolateTable::Id;

// Keeps track of V8 related objects.
class V8Tracker : public Destructible {
 public:
  static V8Tracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->v8_tracker) {
      context->v8_tracker.reset(new V8Tracker(context));
    }
    return static_cast<V8Tracker*>(context->v8_tracker.get());
  }

  ~V8Tracker() override;

  // Might return `std::nullopt` if we can not create an isolate because it has
  // no code range (we do not support this yet).
  std::optional<IsolateId> InternIsolate(protozero::ConstBytes bytes);
  tables::V8JsScriptTable::Id InternJsScript(protozero::ConstBytes bytes,
                                             IsolateId isolate_id);
  tables::V8WasmScriptTable::Id InternWasmScript(protozero::ConstBytes bytes,
                                                 IsolateId isolate_id);
  tables::V8JsFunctionTable::Id InternJsFunction(
      protozero::ConstBytes bytes,
      StringId name,
      tables::V8JsScriptTable::Id script_id);

  void AddJsCode(int64_t timestamp,
                 UniqueTid utid,
                 IsolateId isolate_id,
                 tables::V8JsFunctionTable::Id function_id,
                 const protos::pbzero::V8JsCode::Decoder& code);

  void AddInternalCode(int64_t timestamp,
                       UniqueTid utid,
                       IsolateId v8_isolate_id,
                       const protos::pbzero::V8InternalCode::Decoder& code);

  void AddWasmCode(int64_t timestamp,
                   UniqueTid utid,
                   IsolateId isolate_id,
                   tables::V8WasmScriptTable::Id script_id,
                   const protos::pbzero::V8WasmCode::Decoder& code);

  void AddRegExpCode(int64_t timestamp,
                     UniqueTid utid,
                     IsolateId v8_isolate_id,
                     const protos::pbzero::V8RegExpCode::Decoder& code);

  void MoveCode(int64_t timestamp,
                UniqueTid utid,
                IsolateId v8_isolate_id,
                const protos::pbzero::V8CodeMove::Decoder& code_move);

 private:
  struct JsFunctionHash {
    size_t operator()(const tables::V8JsFunctionTable::Row& v) const {
      return static_cast<size_t>(base::Hasher::Combine(
          v.name.raw_id(), v.v8_js_script_id.value, v.is_toplevel,
          v.kind.raw_id(), v.line.value_or(0), v.col.value_or(0)));
    }
  };

  struct IsolateCodeRanges {
    AddressSet heap_code;
    std::optional<AddressRange> embedded_blob;

    bool operator==(const IsolateCodeRanges& o) const {
      return heap_code == o.heap_code && embedded_blob == o.embedded_blob;
    }
  };

  struct SharedCodeRanges {
    IsolateCodeRanges code_ranges;
    AddressRangeMap<JitCache*> jit_caches;
  };

  // V8 internal isolate_id and upid uniquely identify an isolate in a trace.
  struct IsolateKey {
    struct Hasher {
      size_t operator()(const IsolateKey& v) const {
        return base::Hasher::Combine(v.upid, v.isolate_id);
      }
    };

    bool operator==(const IsolateKey& other) const {
      return upid == other.upid && isolate_id == other.isolate_id;
    }

    bool operator!=(const IsolateKey& other) const { return !(*this == other); }
    UniquePid upid;
    int32_t isolate_id;
  };

  struct ScriptIndexHash {
    size_t operator()(const std::pair<IsolateId, int32_t>& v) const {
      return static_cast<size_t>(
          base::Hasher::Combine(v.first.value, v.second));
    }
  };

  explicit V8Tracker(TraceProcessorContext* context);

  StringId InternV8String(const protos::pbzero::V8String::Decoder& v8_string);

  tables::V8IsolateTable::ConstRowReference InsertIsolate(
      const protos::pbzero::InternedV8Isolate::Decoder& isolate);

  IsolateId CreateIsolate(
      const protos::pbzero::InternedV8Isolate::Decoder& isolate);

  // Find JitCache that fully contains the given range. Returns null if not
  // found and updates error counter.
  JitCache* FindJitCache(IsolateId isolate_id, AddressRange code_range) const;
  // Same as `FindJitCache` but error counter is not updated if no cache is
  // found.
  JitCache* MaybeFindJitCache(IsolateId isolate_id,
                              AddressRange code_range) const;

  UserMemoryMapping* FindEmbeddedBlobMapping(
      UniquePid upid,
      AddressRange embedded_blob_code) const;

  std::pair<IsolateCodeRanges, bool> GetIsolateCodeRanges(
      UniquePid upid,
      const protos::pbzero::InternedV8Isolate::Decoder& isolate);
  AddressRangeMap<JitCache*> GetOrCreateSharedJitCaches(
      UniquePid upid,
      const IsolateCodeRanges& code_ranges);
  AddressRangeMap<JitCache*> CreateJitCaches(
      UniquePid upid,
      const IsolateCodeRanges& code_ranges);

  TraceProcessorContext* const context_;

  base::FlatHashMap<IsolateId, AddressRangeMap<JitCache*>> isolates_;

  // Multiple isolates in the same process might share the code. Keep track of
  // those here.
  base::FlatHashMap<UniquePid, SharedCodeRanges> shared_code_ranges_;

  base::FlatHashMap<IsolateKey, std::optional<IsolateId>, IsolateKey::Hasher>
      isolate_index_;
  base::FlatHashMap<std::pair<IsolateId, int32_t>,
                    tables::V8JsScriptTable::Id,
                    ScriptIndexHash>
      js_script_index_;
  base::FlatHashMap<std::pair<IsolateId, int32_t>,
                    tables::V8WasmScriptTable::Id,
                    ScriptIndexHash>
      wasm_script_index_;
  base::FlatHashMap<tables::V8JsFunctionTable::Row,
                    tables::V8JsFunctionTable::Id,
                    JsFunctionHash>
      js_function_index_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_TRACKER_H_
