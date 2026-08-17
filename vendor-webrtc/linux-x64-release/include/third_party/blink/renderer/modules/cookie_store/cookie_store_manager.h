// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_COOKIE_STORE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_COOKIE_STORE_MANAGER_H_

#include "third_party/blink/public/mojom/cookie_store/cookie_store.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CookieStoreGetOptions;
class ExceptionState;
class ScriptState;

class CookieStoreManager final : public ScriptWrappable,
                                 public Supplement<ServiceWorkerRegistration> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  // Web Exposed as registration.cookies
  static CookieStoreManager* cookies(ServiceWorkerRegistration& registration);

  explicit CookieStoreManager(ServiceWorkerRegistration& registration);

  ~CookieStoreManager() override = default;

  ScriptPromise<IDLUndefined> subscribe(
      ScriptState* script_state,
      const HeapVector<Member<CookieStoreGetOptions>>& subscriptions,
      ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> unsubscribe(
      ScriptState* script_state,
      const HeapVector<Member<CookieStoreGetOptions>>& subscription,
      ExceptionState& exception_state);
  ScriptPromise<IDLSequence<CookieStoreGetOptions>> getSubscriptions(
      ScriptState* script_state,
      ExceptionState& exception_state);

  // GarbageCollected
  void Trace(Visitor* visitor) const override;

 private:
  // The non-static callbacks keep CookieStoreManager alive during mojo calls.
  //
  // The browser-side implementation of the mojo calls assumes the SW
  // registration is live. When CookieStoreManager is used from a Window global,
  // the CookieStoreManager needs to live through the mojo call, so it can keep
  // its ServiceWorkerRegistration alive.
  void OnSubscribeResult(ScriptPromiseResolver<IDLUndefined>* resolver,
                         bool backend_result);
  void OnGetSubscriptionsResult(
      ScriptPromiseResolver<IDLSequence<CookieStoreGetOptions>>* resolver,
      Vector<mojom::blink::CookieChangeSubscriptionPtr> backend_result,
      bool backend_success);

  // SW registration whose cookie change subscriptions are managed by this.
  Member<ServiceWorkerRegistration> registration_;

  // Wraps a Mojo pipe for managing service worker cookie change subscriptions.
  HeapMojoRemote<mojom::blink::CookieStore> backend_;

  // Default for cookie_url in CookieStoreGetOptions.
  //
  // This is the Service Worker registration's scope.
  const KURL default_cookie_url_;

  // The context in which cookies are accessed.
  const scoped_refptr<SecurityOrigin> default_top_frame_origin_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_STORE_COOKIE_STORE_MANAGER_H_
