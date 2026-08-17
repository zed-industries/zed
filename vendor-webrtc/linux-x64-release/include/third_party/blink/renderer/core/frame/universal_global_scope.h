// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_UNIVERSAL_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_UNIVERSAL_GLOBAL_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExceptionState;
class V8VoidFunction;
class StructuredSerializeOptions;
class ScriptState;
class ScriptValue;

class CORE_EXPORT UniversalGlobalScope
    : public Supplementable<UniversalGlobalScope> {
 public:
  String btoa(const String& string_to_encode, ExceptionState&);
  String atob(const String& encoded_string, ExceptionState&);

  void queueMicrotask(V8VoidFunction*);

  ScriptValue structuredClone(ScriptState*,
                              const ScriptValue& message,
                              const StructuredSerializeOptions*,
                              ExceptionState&);

  void reportError(ScriptState*, const ScriptValue&);

  bool isSecureContextForBindings(ScriptState* script_state) const;

  void Trace(Visitor*) const override;

 protected:
  virtual ExecutionContext* GetExecutionContext() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_UNIVERSAL_GLOBAL_SCOPE_H_
