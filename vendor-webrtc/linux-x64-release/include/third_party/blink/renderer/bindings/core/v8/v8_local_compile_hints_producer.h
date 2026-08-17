// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_LOCAL_COMPILE_HINTS_PRODUCER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_LOCAL_COMPILE_HINTS_PRODUCER_H_

#include "third_party/blink/renderer/bindings/buildflags.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class ClassicScript;
class ExecutionContext;
class LocalFrame;

namespace v8_compile_hints {

// Produces compile hints suitable for local caching.
class CORE_EXPORT V8LocalCompileHintsProducer
    : public GarbageCollected<V8LocalCompileHintsProducer> {
 public:
  explicit V8LocalCompileHintsProducer(LocalFrame* frame);

  V8LocalCompileHintsProducer(const V8LocalCompileHintsProducer&) = delete;
  V8LocalCompileHintsProducer& operator=(const V8LocalCompileHintsProducer&) =
      delete;

  ~V8LocalCompileHintsProducer() = default;
  void RecordScript(ExecutionContext* execution_context,
                    const v8::Local<v8::Script> script,
                    ClassicScript* classic_script);
  void GenerateData(bool final_data);

  void Trace(Visitor* visitor) const;

  static v8::ScriptCompiler::CachedData* CreateCompileHintsCachedDataForScript(
      std::vector<int>& compile_hints,
      uint64_t prefix);

 private:
  HeapVector<Member<CachedMetadataHandler>> cache_handlers_;
  HeapVector<v8::TracedReference<v8::CompileHintsCollector>>
      compile_hints_collectors_;
  bool should_generate_data_;
  Member<LocalFrame> frame_;
};

}  // namespace v8_compile_hints
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_LOCAL_COMPILE_HINTS_PRODUCER_H_
