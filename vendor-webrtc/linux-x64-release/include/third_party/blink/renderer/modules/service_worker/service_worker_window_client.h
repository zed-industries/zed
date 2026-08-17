// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_WINDOW_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_WINDOW_CLIENT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ScriptState;
class V8VisibilityState;

class MODULES_EXPORT ServiceWorkerWindowClient final
    : public ServiceWorkerClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using ResolveWindowClientCallback = base::OnceCallback<
      void(bool, mojom::blink::ServiceWorkerClientInfoPtr, const String&)>;

  static ResolveWindowClientCallback CreateResolveWindowClientCallback(
      ScriptPromiseResolver<IDLNullable<ServiceWorkerWindowClient>>*);

  explicit ServiceWorkerWindowClient(
      const mojom::blink::ServiceWorkerClientInfo&);
  ~ServiceWorkerWindowClient() override;

  // WindowClient.idl
  V8VisibilityState visibilityState() const;
  bool focused() const { return is_focused_; }
  ScriptPromise<ServiceWorkerWindowClient> focus(ScriptState*);
  ScriptPromise<IDLNullable<ServiceWorkerWindowClient>> navigate(
      ScriptState*,
      const String& url);

  void Trace(Visitor*) const override;

 private:
  bool page_hidden_;
  bool is_focused_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_WINDOW_CLIENT_H_
