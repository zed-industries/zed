// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_OPERATION_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_OPERATION_DEFINITION_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class V8NoArgumentConstructor;
class V8RunFunctionForSharedStorageSelectURLOperation;
class V8RunFunctionForSharedStorageRunOperation;

// The definition of the shared storage operation registered via
// `register(name, operation)`.
class MODULES_EXPORT SharedStorageOperationDefinition final
    : public GarbageCollected<SharedStorageOperationDefinition>,
      public NameClient {
 public:
  SharedStorageOperationDefinition(ScriptState* script_state,
                                   const String& name,
                                   V8NoArgumentConstructor* constructor,
                                   v8::Local<v8::Function> v8_run);

  ~SharedStorageOperationDefinition() override;

  void Trace(Visitor* visitor) const;

  const char* NameInHeapSnapshot() const override {
    return "SharedStorageOperationDefinition";
  }

  ScriptState* GetScriptState() { return script_state_.Get(); }

  V8NoArgumentConstructor* GetConstructorFunction() {
    return constructor_.Get();
  }

  V8RunFunctionForSharedStorageSelectURLOperation*
  GetRunFunctionForSharedStorageSelectURLOperation() {
    return run_function_for_select_url_.Get();
  }

  V8RunFunctionForSharedStorageRunOperation*
  GetRunFunctionForSharedStorageRunOperation() {
    return run_function_for_run_.Get();
  }

  TraceWrapperV8Reference<v8::Value> GetInstance();

 private:
  Member<ScriptState> script_state_;

  const String name_;

  Member<V8NoArgumentConstructor> constructor_;

  // The operation can potentially be used by both sharedStorage.selectURL() and
  // sharedStorage.run(). Thus, we will store both variants of the run()
  // function, and later on when selectURL() or run() is called, the
  // corresponding variant will be invoked.
  Member<V8RunFunctionForSharedStorageSelectURLOperation>
      run_function_for_select_url_;
  Member<V8RunFunctionForSharedStorageRunOperation> run_function_for_run_;

  bool did_call_constructor_ = false;
  TraceWrapperV8Reference<v8::Value> instance_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_OPERATION_DEFINITION_H_
