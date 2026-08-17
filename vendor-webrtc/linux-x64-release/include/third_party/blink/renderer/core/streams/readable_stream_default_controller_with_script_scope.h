// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_CONTROLLER_WITH_SCRIPT_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_CONTROLLER_WITH_SCRIPT_SCOPE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/to_v8_traits.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ReadableStreamDefaultController;
class ScriptState;

class CORE_EXPORT ReadableStreamDefaultControllerWithScriptScope
    : public GarbageCollected<ReadableStreamDefaultControllerWithScriptScope> {
 public:
  ReadableStreamDefaultControllerWithScriptScope(
      ScriptState* script_state,
      ReadableStreamDefaultController* controller);

  // After calling this the other methods will no longer do anything.
  void Deactivate();

  void Close();
  double DesiredSize() const;
  void Enqueue(v8::Local<v8::Value> js_chunk) const;
  void Error(v8::Local<v8::Value> js_error);

  // Helper methods
  template <typename ChunkType>
    requires std::derived_from<ChunkType, bindings::DictionaryBase> ||
             std::derived_from<ChunkType, ScriptWrappable>
  void Enqueue(ChunkType* chunk) const {
    ScriptState::Scope scope(script_state_);
    Enqueue(chunk->ToV8(script_state_));
  }

  template <typename ErrorType>
    requires std::derived_from<ErrorType, bindings::DictionaryBase> ||
             std::derived_from<ErrorType, ScriptWrappable>
  void Error(ErrorType* error) {
    ScriptState::Scope scope(script_state_);
    Error(error->ToV8(script_state_));
  }

  ReadableStreamDefaultController* GetOriginalController() {
    return controller_.Get();
  }

  void Trace(Visitor*) const;

 private:
  const Member<ScriptState> script_state_;
  Member<ReadableStreamDefaultController> controller_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_CONTROLLER_WITH_SCRIPT_SCOPE_H_
