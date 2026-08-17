// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_CALLBACK_INTERFACE_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_CALLBACK_INTERFACE_BASE_H_

#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// CallbackInterfaceBase is the common base class of all the callback interface
// classes. Most importantly this class provides a way of type dispatching (e.g.
// overload resolutions, SFINAE technique, etc.) so that it's possible to
// distinguish callback interfaces from anything else. Also it provides a common
// implementation of callback interfaces.
//
// As the signatures of callback interface's operations vary, this class does
// not implement any operation. Subclasses will implement it.
class PLATFORM_EXPORT CallbackInterfaceBase
    : public GarbageCollected<CallbackInterfaceBase>,
      public NameClient {
 public:
  // Whether the callback interface is a "single operation callback interface"
  // or not.
  // https://webidl.spec.whatwg.org/#dfn-single-operation-callback-interface
  enum SingleOperationOrNot {
    kNotSingleOperation,
    kSingleOperation,
  };

  ~CallbackInterfaceBase() override = default;

  virtual void Trace(Visitor*) const;

  // Check the identity of |callback_object_|. There can be multiple
  // CallbackInterfaceBase objects that have the same |callback_object_| but
  // have different |incumbent_script_state_|s.
  bool HasTheSameCallbackObject(const CallbackInterfaceBase& other) const {
    return callback_object_ == other.callback_object_;
  }

  v8::Local<v8::Object> CallbackObject() {
    return callback_object_.Get(GetIsolate());
  }

  // Returns true iff the callback interface is a single operation callback
  // interface and the callback interface type value is callable.
  bool IsCallbackObjectCallable() const { return is_callback_object_callable_; }

  v8::Isolate* GetIsolate() { return incumbent_script_state_->GetIsolate(); }

  // Returns the ScriptState of the relevant realm of the callback object.
  //
  // NOTE: This function must be used only when it's pretty sure that the
  // callback object is the same origin-domain. Otherwise,
  // |CallbackRelevantScriptStateOrReportError| or
  // |CallbackRelevantScriptStateOrThrowException| must be used instead.
  ScriptState* CallbackRelevantScriptState() {
    DCHECK(callback_relevant_script_state_);
    return callback_relevant_script_state_.Get();
  }

  // Returns the ScriptState of the relevant realm of the callback object iff
  // the callback is the same origin-domain. Otherwise, reports an error and
  // returns nullptr.
  ScriptState* CallbackRelevantScriptStateOrReportError(
      const char* interface_name,
      const char* operation_name);

  // Returns the ScriptState of the relevant realm of the callback object iff
  // the callback is the same origin-domain. Otherwise, throws an exception and
  // returns nullptr.
  ScriptState* CallbackRelevantScriptStateOrThrowException(
      const char* interface_name,
      const char* operation_name);

  ScriptState* IncumbentScriptState() { return incumbent_script_state_.Get(); }

  DOMWrapperWorld& GetWorld() const { return incumbent_script_state_->World(); }

 protected:
  explicit CallbackInterfaceBase(v8::Local<v8::Object> callback_object,
                                 SingleOperationOrNot);

 private:
  // The "callback interface type" value.
  TraceWrapperV8Reference<v8::Object> callback_object_;
  bool is_callback_object_callable_ = false;
  // The associated Realm of the callback interface type value. Note that the
  // callback interface type value can be different from the function object
  // to be invoked.
  Member<ScriptState> callback_relevant_script_state_;
  // The callback context, i.e. the incumbent Realm when an ECMAScript value is
  // converted to an IDL value.
  // https://webidl.spec.whatwg.org/#dfn-callback-context
  Member<ScriptState> incumbent_script_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_CALLBACK_INTERFACE_BASE_H_
