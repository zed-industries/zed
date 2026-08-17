// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_STATE_H_

#include "base/memory/raw_ptr.h"
#include "gin/public/context_holder.h"
#include "gin/public/gin_embedders.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/platform/bindings/scoped_persistent.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/assertions.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class DOMWrapperWorld;
class ExecutionContext;
class ScriptValue;
class V8PerContextData;

// ScriptState is an abstraction class that holds all information about script
// execution (e.g., v8::Isolate, v8::Context, DOMWrapperWorld, ExecutionContext
// etc). If you need any info about the script execution, you're expected to
// pass around ScriptState in the code base. ScriptState is in a 1:1
// relationship with v8::Context.
//
// When you need ScriptState, you can add [CallWith=ScriptState] to IDL files
// and pass around ScriptState into a place where you need ScriptState.
//
// In some cases, you need ScriptState in code that doesn't have any JavaScript
// on the stack. Then you can store ScriptState on a C++ object using
// Member<ScriptState> or Persistent<ScriptState>.
//
// class SomeObject : public GarbageCollected<SomeObject> {
//   void someMethod(ScriptState* scriptState) {
//     script_state_ = scriptState; // Record the ScriptState.
//     ...;
//   }
//
//   void asynchronousMethod() {
//     if (!script_state_->contextIsValid()) {
//       // It's possible that the context is already gone.
//       return;
//     }
//     // Enter the ScriptState.
//     ScriptState::Scope scope(script_state_);
//     // Do V8 related things.
//     ToV8(...);
//   }
//
//   virtual void Trace(Visitor* visitor) const {
//     visitor->Trace(script_state_);  // ScriptState also needs to be traced.
//   }
//
//   Member<ScriptState> script_state_;
// };
//
// You should not store ScriptState on a C++ object that can be accessed
// by multiple worlds. For example, you can store ScriptState on
// ScriptPromiseResolverBase, ScriptValue etc because they can be accessed from
// one world. However, you cannot store ScriptState on a DOM object that has an
// IDL interface because the DOM object can be accessed from multiple worlds. If
// ScriptState of one world "leak"s to another world, you will end up with
// leaking any JavaScript objects from one Chrome extension to another Chrome
// extension, which is a severe security bug.
//
// Lifetime:
// ScriptState is created when v8::Context is created.
// ScriptState is destroyed when v8::Context is garbage-collected and
// all V8 proxy objects that have references to the ScriptState are destructed.
class PLATFORM_EXPORT ScriptState : public GarbageCollected<ScriptState> {
 public:
  class Scope final {
    STACK_ALLOCATED();

   public:
    // You need to make sure that scriptState->context() is not empty before
    // creating a Scope.
    explicit Scope(ScriptState* script_state)
        : handle_scope_(script_state->GetIsolate()),
          context_(script_state->GetContext()) {
      DCHECK(script_state->ContextIsValid());
      context_->Enter();
    }

    ~Scope() { context_->Exit(); }

   private:
    v8::HandleScope handle_scope_;
    v8::Local<v8::Context> context_;
  };

  // Use EscapableScope if you have to return a v8::Local to an outer scope.
  // See v8::EscapableHandleScope.
  class EscapableScope final {
    STACK_ALLOCATED();

   public:
    // You need to make sure that scriptState->context() is not empty before
    // creating a Scope.
    explicit EscapableScope(ScriptState* script_state)
        : handle_scope_(script_state->GetIsolate()),
          context_(script_state->GetContext()) {
      DCHECK(script_state->ContextIsValid());
      context_->Enter();
    }

    ~EscapableScope() { context_->Exit(); }

    v8::Local<v8::Value> Escape(v8::Local<v8::Value> value) {
      return handle_scope_.Escape(value);
    }

   private:
    v8::EscapableHandleScope handle_scope_;
    v8::Local<v8::Context> context_;
  };

  static ScriptState* Create(v8::Local<v8::Context>,
                             DOMWrapperWorld*,
                             ExecutionContext*);

  ScriptState(const ScriptState&) = delete;
  ScriptState& operator=(const ScriptState&) = delete;
  virtual ~ScriptState();

  virtual void Trace(Visitor*) const;

  static ScriptState* ForCurrentRealm(v8::Isolate* isolate) {
    DCHECK(isolate->InContext());
    return From(isolate, isolate->GetCurrentContext());
  }

  static ScriptState* ForCurrentRealm(
      const v8::FunctionCallbackInfo<v8::Value>& info) {
    return ForCurrentRealm(info.GetIsolate());
  }

  static ScriptState* ForCurrentRealm(
      const v8::PropertyCallbackInfo<v8::Value>& info) {
    return ForCurrentRealm(info.GetIsolate());
  }

  static ScriptState* ForRelevantRealm(v8::Isolate* isolate,
                                       v8::Local<v8::Object> object) {
    DCHECK(!object.IsEmpty());
    ScriptState* script_state = static_cast<ScriptState*>(
        object->GetAlignedPointerFromEmbedderDataInCreationContext(
            isolate, kV8ContextPerContextDataIndex));
    // ScriptState::ForRelevantRealm() must be called only for objects having a
    // creation context while the context must have a valid embedder data in
    // the embedder field.
    DCHECK(script_state);
    return script_state;
  }

  static ScriptState* From(v8::Isolate* isolate,
                           v8::Local<v8::Context> context) {
    DCHECK(!context.IsEmpty());
    ScriptState* script_state =
        static_cast<ScriptState*>(context->GetAlignedPointerFromEmbedderData(
            isolate, kV8ContextPerContextDataIndex));
    // ScriptState::From() must not be called for a context that does not have
    // valid embedder data in the embedder field.
    DCHECK(script_state);
    SECURITY_CHECK(script_state->context_ == context);
    return script_state;
  }

  // For use when it is not absolutely certain that the v8::Context is
  // associated with a ScriptState. This is necessary in unit tests when a
  // v8::Context is created directly on the v8 API without going through the
  // usual blink codepaths.
  // This is also called in some situations where DissociateContext() has
  // already been called and therefore the ScriptState pointer on the
  // v8::Context has already been nulled.
  static ScriptState* MaybeFrom(v8::Isolate* isolate,
                                v8::Local<v8::Context> context) {
    DCHECK(!context.IsEmpty());
    if (context->GetNumberOfEmbedderDataFields() <=
        kV8ContextPerContextDataIndex) {
      return nullptr;
    }
    ScriptState* script_state =
        static_cast<ScriptState*>(context->GetAlignedPointerFromEmbedderData(
            isolate, kV8ContextPerContextDataIndex));
    SECURITY_CHECK(!script_state || script_state->context_ == context);
    return script_state;
  }

  v8::Isolate* GetIsolate() const { return isolate_; }
  DOMWrapperWorld& World() const { return *world_; }
  const V8ContextToken& GetToken() const { return token_; }

  // This can return an empty handle if the v8::Context is gone.
  v8::Local<v8::Context> GetContext() const {
    return context_.NewLocal(isolate_);
  }
  bool ContextIsValid() const {
    return !context_.IsEmpty() && per_context_data_;
  }
  void DetachGlobalObject();

  V8PerContextData* PerContextData() const { return per_context_data_.Get(); }
  void DisposePerContextData();

  // This method is expected to be called only from
  // WorkerOrWorkletScriptController to run operations that should have been
  // invoked by a weak callback if a V8 GC were run, in a worker thread
  // termination.
  void DissociateContext();

  void RecordScriptCompilation(String file, bool used_code_cache) {
    last_compiled_script_file_name_ = file;
    last_compiled_script_used_code_cache_ = used_code_cache;
  }
  String last_compiled_script_file_name() const {
    return last_compiled_script_file_name_;
  }
  bool last_compiled_script_used_code_cache() const {
    return last_compiled_script_used_code_cache_;
  }

 protected:
  ScriptState(v8::Local<v8::Context>, DOMWrapperWorld*, ExecutionContext*);

 private:
  static void OnV8ContextCollectedCallback(
      const v8::WeakCallbackInfo<ScriptState>&);

  raw_ptr<v8::Isolate, DanglingUntriaged> isolate_;
  // This persistent handle is weak.
  ScopedPersistent<v8::Context> context_;

  // This refptr doesn't cause a cycle because all persistent handles that
  // DOMWrapperWorld holds are weak.
  Member<DOMWrapperWorld> world_;

  Member<V8PerContextData> per_context_data_;

  // v8::Context has an internal field to this ScriptState* as a raw pointer,
  // which is out of scope of Blink GC, but it must be a strong reference.  We
  // use |reference_from_v8_context_| to represent this strong reference.  The
  // lifetime of |reference_from_v8_context_| and the internal field must match
  // exactly.
  SelfKeepAlive<ScriptState> reference_from_v8_context_{this};

  // Serves as a unique ID for this context, which can be used to name the
  // context in browser/renderer communications.
  V8ContextToken token_;

  using CreateCallback = ScriptState* (*)(v8::Local<v8::Context>,
                                          DOMWrapperWorld*,
                                          ExecutionContext*);
  static CreateCallback s_create_callback_;
  static void SetCreateCallback(CreateCallback);
  friend class ScriptStateImpl;

  static constexpr int kV8ContextPerContextDataIndex =
      static_cast<int>(gin::kPerContextDataStartIndex) +
      static_cast<int>(gin::kEmbedderBlink);

  // For accessing information about the last script compilation via
  // internals.idl.
  String last_compiled_script_file_name_;
  bool last_compiled_script_used_code_cache_ = false;
};

// ScriptStateProtectingContext keeps the context associated with the
// ScriptState alive.  You need to call Clear() once you no longer need the
// context. Otherwise, the context will leak.
class ScriptStateProtectingContext final
    : public GarbageCollected<ScriptStateProtectingContext> {
 public:
  explicit ScriptStateProtectingContext(ScriptState* script_state)
      : script_state_(script_state) {
    if (script_state_) {
      context_.Set(script_state_->GetIsolate(), script_state_->GetContext());
      context_.Get().AnnotateStrongRetainer(
          "blink::ScriptStateProtectingContext::context_");
    }
  }
  ScriptStateProtectingContext(const ScriptStateProtectingContext&) = delete;
  ScriptStateProtectingContext& operator=(const ScriptStateProtectingContext&) =
      delete;

  void Trace(Visitor* visitor) const { visitor->Trace(script_state_); }

  ScriptState* Get() const { return script_state_.Get(); }
  void Reset() {
    script_state_ = nullptr;
    context_.Clear();
  }

  // ScriptState like interface
  bool ContextIsValid() const { return script_state_->ContextIsValid(); }
  v8::Isolate* GetIsolate() const { return script_state_->GetIsolate(); }
  v8::Local<v8::Context> GetContext() const {
    return script_state_->GetContext();
  }

 private:
  Member<ScriptState> script_state_;
  ScopedPersistent<v8::Context> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_STATE_H_
