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

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_REMOTE_WINDOW_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_REMOTE_WINDOW_PROXY_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/window_proxy.h"
#include "third_party/blink/renderer/core/frame/remote_frame.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "v8/include/v8.h"

namespace blink {

// Subclass of WindowProxy that only handles RemoteFrame.
class RemoteWindowProxy final : public WindowProxy {
 public:
  RemoteWindowProxy(v8::Isolate*, RemoteFrame&, DOMWrapperWorld*);

 private:
  // WindowProxy overrides:
  void Initialize() override;
  void DisposeContext(Lifecycle next_status, FrameReuseStatus) override;

  // Creates a new v8::Context with the window wrapper object as the global
  // object (aka the inner global).  Note that the window wrapper and its
  // prototype chain do not get fully initialized yet, e.g. the window
  // wrapper is not yet associated with the native DOMWindow object.
  void CreateContext();

  // Associates the window wrapper and its prototype chain with the native
  // DOMWindow object. Also does some more Window-specific initialization.
  void SetupWindowPrototypeChain();
};

template <>
struct DowncastTraits<RemoteWindowProxy> {
  static bool AllowFrom(const WindowProxy& proxy) { return !proxy.IsLocal(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_REMOTE_WINDOW_PROXY_H_
