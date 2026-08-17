// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CLIENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CLIENTS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_client_query_options.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ScriptState;
class ServiceWorkerClient;
class ServiceWorkerWindowClient;

class ServiceWorkerClients final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ServiceWorkerClients* Create();

  ServiceWorkerClients();

  // Clients.idl
  ScriptPromise<ServiceWorkerClient> get(ScriptState*, const String& id);
  ScriptPromise<IDLSequence<ServiceWorkerClient>> matchAll(
      ScriptState*,
      const ClientQueryOptions*);
  ScriptPromise<IDLNullable<ServiceWorkerWindowClient>> openWindow(
      ScriptState*,
      const String& url);
  ScriptPromise<IDLUndefined> claim(ScriptState*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CLIENTS_H_
