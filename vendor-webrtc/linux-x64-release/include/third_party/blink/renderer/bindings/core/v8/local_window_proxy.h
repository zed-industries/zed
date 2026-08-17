/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_LOCAL_WINDOW_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_LOCAL_WINDOW_PROXY_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/window_proxy.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "v8/include/v8.h"

namespace blink {

class HTMLDocument;
class ScriptState;
class SecurityOrigin;

// Subclass of WindowProxy that only handles LocalFrame.
class LocalWindowProxy final : public WindowProxy {
 public:
  LocalWindowProxy(v8::Isolate*, LocalFrame&, DOMWrapperWorld*);
  void Trace(Visitor*) const override;

  v8::Local<v8::Context> ContextIfInitialized() const {
    return script_state_ ? script_state_->GetContext()
                         : v8::Local<v8::Context>();
  }

  // Update document object of the frame.
  void UpdateDocument();

  void NamedItemAdded(HTMLDocument*, const AtomicString&);
  void NamedItemRemoved(HTMLDocument*, const AtomicString&);

  // Update the security origin of a document
  // (e.g., after setting docoument.domain).
  void UpdateSecurityOrigin(const SecurityOrigin*);

  void SetAbortScriptExecution(
      v8::Context::AbortScriptExecutionCallback callback);

 private:
  // LocalWindowProxy overrides:
  bool IsLocal() const override { return true; }
  void Initialize() override;
  void DisposeContext(Lifecycle next_status, FrameReuseStatus) override;

  // Creates a new v8::Context with the window wrapper object as the global
  // object (aka the inner global).  Note that the window wrapper and its
  // prototype chain do not get fully initialized yet, e.g. the window
  // wrapper is not yet associated with the native DOMWindow object.
  void CreateContext();

  // Installs conditionally enabled features, if necessary.
  void InstallConditionalFeatures();

  // Associates the window wrapper and its prototype chain with the native
  // DOMWindow object. Also does some more Window-specific initialization.
  void SetupWindowPrototypeChain();

  void SetSecurityToken(const SecurityOrigin*);

  // Triggers updates of objects that are associated with a Document:
  // - the activity logger
  // - the document DOM wrapper (performance optimization for accessing
  //   window.document in the main world)
  // - the security origin
  void UpdateDocumentForMainWorld();

  // The JavaScript wrapper for the document object is cached on the global
  // object for fast access. UpdateDocumentProperty sets the wrapper
  // for the current document on the global object.
  void UpdateDocumentProperty();

  // Updates Activity Logger for the current context.
  void UpdateActivityLogger();

  LocalFrame* GetFrame() const {
    return To<LocalFrame>(WindowProxy::GetFrame());
  }

  Member<ScriptState> script_state_;
  bool context_was_created_from_snapshot_ = false;
};

template <>
struct DowncastTraits<LocalWindowProxy> {
  static bool AllowFrom(const WindowProxy& windowProxy) {
    return windowProxy.IsLocal();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_LOCAL_WINDOW_PROXY_H_
