// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_SCOPED_PROMISE_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_SCOPED_PROMISE_RESOLVER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Off-heap wrapper that holds a strong reference to a
// ScriptPromiseResolverBase.
class ScopedPromiseResolver {
  USING_FAST_MALLOC(ScopedPromiseResolver);

 public:
  explicit ScopedPromiseResolver(ScriptPromiseResolverBase* resolver);

  ScopedPromiseResolver(const ScopedPromiseResolver&) = delete;
  ScopedPromiseResolver& operator=(const ScopedPromiseResolver&) = delete;

  ~ScopedPromiseResolver();

  // Releases the owned |resolver_|. This is to be called by the Mojo response
  // callback responsible for resolving the corresponding ScriptPromise
  //
  // If this method is not called before |this| goes of scope, it is assumed
  // that a Mojo connection error has occurred, and the response callback was
  // never invoked. The Promise will be rejected with an appropriate exception.
  ScriptPromiseResolverBase* Release();

 private:
  void OnConnectionError();

  Persistent<ScriptPromiseResolverBase> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_SCOPED_PROMISE_RESOLVER_H_
