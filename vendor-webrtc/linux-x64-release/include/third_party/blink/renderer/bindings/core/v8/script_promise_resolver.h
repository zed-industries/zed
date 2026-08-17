// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_RESOLVER_H_

#include "base/dcheck_is_on.h"
#include "base/hash/hash.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/to_v8_traits.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/dictionary_base.h"
#include "third_party/blink/renderer/platform/bindings/exception_context.h"
#include "third_party/blink/renderer/platform/bindings/scoped_persistent.h"
#include "third_party/blink/renderer/platform/bindings/script_forbidden_scope.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/union_base.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "v8/include/v8.h"

#if DCHECK_IS_ON()
#include "base/debug/stack_trace.h"
#endif

namespace blink {

template <typename IDLResolvedType>
class ScriptPromiseResolver;
class SourceLocation;

// This class wraps v8::Promise::Resolver for easier use in blink.
//
// A ScriptPromiseResolver must be templated with its IDLResolveType. This
// is the type of the promise specified in the relevant IDL file. For types
// that are part of the IDL language, find the corresponding type in idl_types.h
// (`bool` becomes `IDLBoolean`, `any` becomes `IDLAny`, etc). For anything
// defined in an IDL file (interfaces, dictionaries, unions, enums),  template
// with the C++ object type. If the type is nullable (indicated by a "?" suffix
// in the IDL file), wrap the type in IDLNullable<>. The given idl type will
// determine what values may be passed to `Resolve()` - the IDLResolveType and
// the passed-in type will be forwarded to ToV8Traits<>, so the types must be
// compatible for that purpose. For example, a
// `ScriptPromiseResolver<IDLAny>` may `Resolve()` with  a `ScriptValue` or
// a `v8::Local<v8::Value>`, but a `ScriptPromiseResolver<IDLBoolean>` must
// `Resolve()` with a `bool`.
//
// ScriptPromiseResolverBase is an untyped base class and may not be created
// directly. It should only be used for logic that needs to handle resolvers
// of multiple different IDL types. A ScriptPromiseResolverBase can be rejected
// without being downcasted, because reject types are not specified in the IDL.
// However, it must be downcasted via `DowncastTo<IDLResolveType>()` in order
// to `Resolve()`.
//
// `DowncastTo` will DCHECK in the case of a bad cast. It's not a security issue
// because all ScriptPromiseResolverBase/ScriptPromiseResolver classes have the
// same memory layout and vtable, but it is a correctness issue because the
// promise will be resolved with an incorrect type.
//
// This class retains a ScriptState, so a caller can call resolve or reject from
// outside of a V8 context. When the ScriptState's associated ExecutionContext
// is destroyed, resolve or reject  will be ignored.
//
// There are cases where promises cannot work (e.g., where the thread is being
// terminated). In such cases operations will silently fail.
class CORE_EXPORT ScriptPromiseResolverBase
    : public GarbageCollected<ScriptPromiseResolverBase> {
#if DCHECK_IS_ON()
  USING_PRE_FINALIZER(ScriptPromiseResolverBase, Dispose);
#endif

 public:
  ScriptPromiseResolverBase(const ScriptPromiseResolverBase&) = delete;
  ScriptPromiseResolverBase& operator=(const ScriptPromiseResolverBase&) =
      delete;
  virtual ~ScriptPromiseResolverBase();

#if DCHECK_IS_ON()
  void Dispose();
#endif

  // Anything that can be passed to ToV8Traits can be passed to this function.
  template <typename IDLType, typename BlinkType>
  void Reject(BlinkType value) {
    if (!PrepareToResolveOrReject<kRejecting>()) {
      return;
    }
    ResolveOrReject<IDLType, BlinkType>(std::move(value));
  }

  // These are shorthand helpers for rejecting the promise with a common type.
  // Use the templated Reject<IDLType>() variant for uncommon types.
  void Reject(DOMException*);
  void Reject(v8::Local<v8::Value>);
  void Reject(const ScriptValue&);
  void Reject(const char*);
  void Reject(bool);
  void Reject() { Reject<IDLUndefined>(ToV8UndefinedGenerator()); }

  // The following functions create an exception of the given type.
  // They require ScriptPromiseResolver to be created with ExceptionContext in
  // order to have context information added to the message.

  // Reject with DOMException with given exception code.
  void RejectWithDOMException(DOMExceptionCode exception_code,
                              const String& message);
  // Reject with DOMException with SECURITY_ERR.
  void RejectWithSecurityError(const String& sanitized_message,
                               const String& unsanitized_message);
  // Reject with ECMAScript Error object.
  void RejectWithTypeError(const String& message);
  void RejectWithRangeError(const String& message);

  // Reject with WebAssembly Error object.
  void RejectWithWasmCompileError(const String& message);

  ScriptState* GetScriptState() const { return script_state_.Get(); }

  template <typename IDLResolvedType>
  ScriptPromiseResolver<IDLResolvedType>* DowncastTo() {
#if DCHECK_IS_ON()
    DCHECK_EQ(runtime_type_id_,
              GetTypeId<ScriptPromiseResolver<IDLResolvedType>>());
#endif
    return static_cast<ScriptPromiseResolver<IDLResolvedType>*>(this);
  }

  // Calling this function makes the resolver release its internal resources.
  // That means the associated promise will never be resolved or rejected
  // unless it's already been resolved or rejected.
  // Do not call this function unless you truly need the behavior.
  void Detach();

  // Suppresses the check in Dispose. Do not use this function unless you truly
  // need the behavior. Also consider using Detach().
  void SuppressDetachCheck() {
#if DCHECK_IS_ON()
    suppress_detach_check_ = true;
#endif
  }

  virtual void Trace(Visitor*) const;

  ExecutionContext* GetExecutionContext();

 private:
  template <typename IDLResolvedType>
  friend class ScriptPromiseResolver;

  ScriptPromiseResolverBase(ScriptState*, const ExceptionContext&);

#if DCHECK_IS_ON()
  template <typename T>
  inline size_t GetTypeId() {
    return base::FastHash(__PRETTY_FUNCTION__);
  }

  // True if promise() is called.
  bool is_promise_called_ = false;
  size_t runtime_type_id_ = 0;
#endif

  class ExceptionStateScope;
  enum ResolutionState {
    kPending,
    kResolving,
    kRejecting,
    kDone,
  };

  template <typename IDLType, typename BlinkType>
  void ResolveOrReject(BlinkType value) {
    ScriptState::Scope scope(script_state_.Get());
    // Calling ToV8 in a ScriptForbiddenScope will trigger a CHECK and
    // cause a crash. ToV8 just invokes a constructor for wrapper creation,
    // which is safe (no author script can be run). Adding AllowUserAgentScript
    // directly inside createWrapper could cause a perf impact (calling
    // isMainThread() every time a wrapper is created is expensive). Ideally,
    // resolveOrReject shouldn't be called inside a ScriptForbiddenScope.
    {
      ScriptForbiddenScope::AllowUserAgentScript allow_script;
      v8::Isolate* isolate = script_state_->GetIsolate();
      v8::MicrotasksScope microtasks_scope(
          isolate, ToMicrotaskQueue(script_state_.Get()),
          v8::MicrotasksScope::kDoNotRunMicrotasks);
      value_.Reset(isolate,
                   ToV8Traits<IDLType>::ToV8(script_state_, std::move(value)));
    }
    NotifyResolveOrReject();
  }

  template <ResolutionState new_state>
  bool PrepareToResolveOrReject() {
    static_assert(new_state == kResolving || new_state == kRejecting);
    if (state_ != kPending || !GetExecutionContext()) {
      return false;
    }
    state_ = new_state;
    return true;
  }

  void OverrideScriptStateToCurrentContext() {
    v8::Isolate* isolate = script_state_->GetIsolate();
    CHECK(isolate->InContext());
    script_state_ = ScriptState::ForCurrentRealm(isolate);
  }

  void NotifyResolveOrReject();
  void ResolveOrRejectImmediately();
  void ScheduleResolveOrReject();
  void ResolveOrRejectDeferred();

  TraceWrapperV8Reference<v8::Promise::Resolver> resolver_;
  ResolutionState state_;
  Member<ScriptState> script_state_;
  TraceWrapperV8Reference<v8::Value> value_;
  const ExceptionContext exception_context_;
  std::unique_ptr<SourceLocation> source_location_;

#if DCHECK_IS_ON()
  bool suppress_detach_check_ = false;

  base::debug::StackTrace create_stack_trace_{8};
#endif
};

template <typename IDLResolvedType>
class ScriptPromiseResolver final : public ScriptPromiseResolverBase {
 public:
  explicit ScriptPromiseResolver(ScriptState* script_state)
      : ScriptPromiseResolver(script_state,
                              ExceptionContext(v8::ExceptionContext::kUnknown,
                                               nullptr,
                                               nullptr)) {}

  ScriptPromiseResolver(ScriptState* script_state,
                        const ExceptionContext& context)
      : ScriptPromiseResolverBase(script_state, context) {
#if DCHECK_IS_ON()
    runtime_type_id_ = GetTypeId<ScriptPromiseResolver<IDLResolvedType>>();
#endif
  }

  // Anything that can be passed to ToV8Traits<IDLResolvedType> can be passed to
  // this function.
  template <typename BlinkType>
  void Resolve(BlinkType value) {
    if (!PrepareToResolveOrReject<kResolving>()) {
      return;
    }
    ResolveOrReject<IDLResolvedType, BlinkType>(std::move(value));
  }

  // This Resolve() variant completely ignores the ScriptState given in the
  // constructor and resolves in the current context. This is not the default
  // behavior and should only be used if a WPT needs it.
  template <typename BlinkType>
  void ResolveOverridingToCurrentContext(BlinkType value) {
    OverrideScriptStateToCurrentContext();
    if (!PrepareToResolveOrReject<kResolving>()) {
      return;
    }
    ResolveOrReject<IDLResolvedType, BlinkType>(std::move(value));
  }

  // This Resolve() method allows a Promise expecting to be resolved with a
  // union type to be resolved with any type of that union without the caller
  // needing to explicitly construct a union object.
  template <typename BlinkType>
    requires std::derived_from<IDLResolvedType, bindings::UnionBase>
  void Resolve(BlinkType value) {
    if (!PrepareToResolveOrReject<kResolving>()) {
      return;
    }
    ResolveOrReject<IDLResolvedType, IDLResolvedType*>(
        MakeGarbageCollected<IDLResolvedType>(value));
  }

  // This Resolve() method allows a Promise expecting to be resolved with an
  // enum type to be resolved with an enum value of that type rather than having
  // to explicitly construct the enum type.
  template <typename T = IDLResolvedType>
    requires std::derived_from<IDLResolvedType, bindings::EnumerationBase>
  void Resolve(T::Enum value) {
    if (!PrepareToResolveOrReject<kResolving>()) {
      return;
    }
    ResolveOrReject<IDLResolvedType>(T(value));
  }

  // A promise may be resolved with another promise if they are the same type.
  void Resolve(ScriptPromise<IDLResolvedType> promise) {
    if (!PrepareToResolveOrReject<kResolving>()) {
      return;
    }
    value_.Reset(script_state_->GetIsolate(), promise.V8Promise());
    NotifyResolveOrReject();
  }

  // Many IDL-exposed promises with a type other than undefined nevertheless
  // resolve with undefined in certain circumstances. Do we need to support this
  // behavior?
  void Resolve() {
    if (!PrepareToResolveOrReject<kResolving>()) {
      return;
    }
    ResolveOrReject<IDLUndefined, ToV8UndefinedGenerator>(
        ToV8UndefinedGenerator());
  }

  // TODO(japhet): Exposing the underlying v8::Promise is a perf workaround
  // for internal usage only. Ideally we'd use ScriptPromise everywhere.
  v8::Local<v8::Promise> V8Promise() {
#if DCHECK_IS_ON()
    is_promise_called_ = true;
#endif
    // `resolver_` should only be empty if `Detach()` was invoked. The promise
    // should not be accessed after 'Detach()`.
    CHECK(!resolver_.IsEmpty());
    return resolver_.Get(script_state_->GetIsolate())->GetPromise();
  }

  ScriptPromise<IDLResolvedType> Promise() {
    return ScriptPromise<IDLResolvedType>(script_state_->GetIsolate(),
                                          V8Promise());
  }

  // Returns a callback that will run |callback| with the Entry realm
  // and the Current realm set to the resolver's ScriptState. Note |callback|
  // will only be run if the execution context and V8 context are capable
  // to run. This situation occurs when the resolver's execution context
  // or V8 context have started their destruction. See
  // `IsInParallelAlgorithmRunnable` for details.
  template <typename... Args>
  base::OnceCallback<void(Args...)> WrapCallbackInScriptScope(
      base::OnceCallback<void(ScriptPromiseResolver<IDLResolvedType>*, Args...)>
          callback) {
    return WTF::BindOnce(
        [](ScriptPromiseResolver<IDLResolvedType>* resolver,
           base::OnceCallback<void(ScriptPromiseResolver<IDLResolvedType>*,
                                   Args...)> callback,
           Args... args) {
          ScriptState* script_state = resolver->GetScriptState();
          if (!IsInParallelAlgorithmRunnable(resolver->GetExecutionContext(),
                                             script_state)) {
            return;
          }
          ScriptState::Scope script_state_scope(script_state);
          std::move(callback).Run(resolver, std::move(args)...);
        },
        WrapPersistent(this), std::move(callback));
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_RESOLVER_H_
