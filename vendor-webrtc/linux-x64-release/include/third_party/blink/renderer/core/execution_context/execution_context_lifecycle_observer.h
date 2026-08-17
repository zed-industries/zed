/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_LIFECYCLE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_LIFECYCLE_OBSERVER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"

namespace blink {

class ExecutionContext;
class LocalDOMWindow;

// ExecutionContextClient and ExecutionContextLifecycleObserver are helpers to
// associate an object with an ExecutionContext. It is unsafe to access an
// associated GC object, *including via GetExecutionContext()*, in the
// destructor of a GC object.
//
// To discourage incorrect usage, these objects both start returning null
// once the execution context shuts down. For a window, this occurs when
// the frame navigates to another window or is detached. For a worker global
// scope, this occurs when it shuts down.
//
// * If an object only needs to refer to a valid ExecutionContext but does not
//   need to stop or suspend any activity, it should be an
//   ExecutionContextClient.
// * If an object associated with an ExecutionContext has shutdown logic to
//   perform, such as halting activity or disconnecting from longer-lived
//   objects, it should be a ExecutionContextLifecycleObserver.
// * If an object additionally must suspend its activity during pause (see
//   execution_context_lifecycle_state_observer.h), it should be a
//   ExecutionContextLifecycleStateObserver (and thus, transitively, also a
//   ExecutionContextLifecycleObserver).
//
// If your object has activity which requires that it be kept alive, even if no
// other object has a reference to it, consider whether your object should also
// derive from ActiveScriptWrappable.

// ExecutionContextClient provides access to the associated execution context
// until it is shut down (e.g. for a window, at navigation or frame detach).
class CORE_EXPORT ExecutionContextClient : public GarbageCollectedMixin {
 public:
  // Returns the execution context until it is detached.
  // From then on, returns null instead.
  ExecutionContext* GetExecutionContext() const;

  // If the execution context is a window, returns it. Returns nullptr if the
  // execution context is not a window or if it has been detached.
  LocalDOMWindow* DomWindow() const;

  void Trace(Visitor*) const override;

 protected:
  explicit ExecutionContextClient(ExecutionContext*);

 private:
  WeakMember<ExecutionContext> execution_context_;
};

// ExecutionContextLifecycleObserver provides an additional ContextDestroyed()
// hook to execute cleanup code when a context is shut down (e.g. for a
// document, at navigation or frame detach -- not when its destructor runs).
//
// Execution context associated objects which have ongoing activity,
// registration with objects which outlive the context, or resources which
// should be promptly released, should consider deriving from
// ExecutionContextLifecycleObserver. As a rule of thumb: if the destructor
// contains non-trivial logic, that logic may belong in ContextDestroyed()
// instead.
//
// If there is ongoing activity associated with the object, consider whether it
// needs to be paused when execution is suspended (see
// ExecutionContextLifecycleStateObserver).
//
// If none of the above applies, prefer the simpler ExecutionContextClient.
class CORE_EXPORT ExecutionContextLifecycleObserver
    : public ContextLifecycleObserver {
 public:
  // Returns the execution context until it is detached.
  // From then on, returns null instead.
  ExecutionContext* GetExecutionContext() const;
  virtual void SetExecutionContext(ExecutionContext*);

  // If the execution context is a window, returns it. Returns nullptr if the
  // execution context is not a window or if it has been detached.
  LocalDOMWindow* DomWindow() const;

  enum Type {
    kGenericType,
    kStateObjectType,
  };

  Type ObserverType() const { return observer_type_; }

  bool IsExecutionContextLifecycleObserver() const override { return true; }

  void Trace(Visitor*) const override;

  // This will be triggered when the ExecutionContext is moved into BFCache.
  virtual void ContextEnteredBackForwardCache() {}

 protected:
  explicit ExecutionContextLifecycleObserver(
      ExecutionContext* execution_context,
      Type type = kGenericType);

 private:
  Type observer_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_LIFECYCLE_OBSERVER_H_
