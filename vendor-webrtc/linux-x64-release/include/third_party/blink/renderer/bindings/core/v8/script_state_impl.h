// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_STATE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_STATE_IMPL_H_

#include "third_party/blink/renderer/platform/bindings/script_state.h"

namespace blink {

class DOMWrapperWorld;
class ExecutionContext;

// This is a helper class to resolve a layering violation. ScripState should
// hold a Member<ExecutionContext>, but ExecutionContext lives in core/, and
// ScriptState lives in platform/, which isn't allowed. Consumers can just use
// helpers like ToExecutionContext(ScriptState*) and the bindings will take care
// of the requisite casts.
class ScriptStateImpl final : public ScriptState {
 public:
  static void Init();

  ScriptStateImpl(v8::Local<v8::Context>, DOMWrapperWorld*, ExecutionContext*);
  ScriptStateImpl(const ScriptState&) = delete;
  ScriptStateImpl& operator=(const ScriptState&) = delete;
  ~ScriptStateImpl() override = default;
  void Trace(Visitor*) const override;

  ExecutionContext* GetExecutionContext() const {
    return execution_context_.Get();
  }

 private:
  static ScriptState* Create(v8::Local<v8::Context>,
                             DOMWrapperWorld*,
                             ExecutionContext*);

  WeakMember<ExecutionContext> execution_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_STATE_IMPL_H_
