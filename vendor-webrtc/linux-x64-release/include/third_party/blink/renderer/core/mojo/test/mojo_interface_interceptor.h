// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_TEST_MOJO_INTERFACE_INTERCEPTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_TEST_MOJO_INTERFACE_INTERCEPTOR_H_

#include "base/types/strong_alias.h"
#include "mojo/public/cpp/system/message_pipe.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_mojo_interface_interceptor_scope.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ExecutionContext;

// A MojoInterfaceInterceptor can be constructed by test scripts in order to
// intercept all outgoing requests for a specific named interface from the
// owning document, whether the requests come from other script or from native
// code (e.g. native API implementation code.) In production, such requests are
// normally routed to the browser to be bound to real interface implementations,
// but in test environments it's often useful to mock them out locally.
class MojoInterfaceInterceptor final
    : public EventTarget,
      public ActiveScriptWrappable<MojoInterfaceInterceptor>,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using Scope = V8MojoInterfaceInterceptorScope;
  static MojoInterfaceInterceptor* Create(ExecutionContext*,
                                          const String& interface_name,
                                          const Scope& scope,
                                          ExceptionState&);

  MojoInterfaceInterceptor(ExecutionContext*,
                           const String& interface_name,
                           Scope::Enum scope);
  ~MojoInterfaceInterceptor() override;

  void start(ExceptionState&);
  void stop();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(interfacerequest, kInterfacerequest)

  void Trace(Visitor*) const override;

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ActiveScriptWrappable
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() final;

 private:
  void OnInterfaceRequest(mojo::ScopedMessagePipeHandle);
  void DispatchInterfaceRequestEvent(mojo::ScopedMessagePipeHandle);

  const String interface_name_;
  bool started_ = false;
  Scope::Enum scope_ = Scope::Enum::kContext;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_TEST_MOJO_INTERFACE_INTERCEPTOR_H_
