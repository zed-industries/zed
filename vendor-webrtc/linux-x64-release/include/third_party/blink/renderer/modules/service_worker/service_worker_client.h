// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CLIENT_H_

#include "third_party/blink/public/mojom/service_worker/service_worker_client.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class PostMessageOptions;
class ScriptState;
class V8ClientLifecycleState;
class V8ClientType;
class V8ContextFrameType;

class MODULES_EXPORT ServiceWorkerClient : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ServiceWorkerClient(const mojom::blink::ServiceWorkerClientInfo&);
  ~ServiceWorkerClient() override;

  // Client.idl
  String url() const { return url_; }
  V8ClientType type() const;
  V8ContextFrameType frameType(ScriptState*) const;
  String id() const { return uuid_; }
  V8ClientLifecycleState lifecycleState() const;
  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   HeapVector<ScriptObject> transfer,
                   ExceptionState&);
  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   const PostMessageOptions*,
                   ExceptionState&);

 protected:
  String Uuid() const { return uuid_; }

 private:
  const String uuid_;
  const String url_;
  const mojom::ServiceWorkerClientType type_;
  const mojom::RequestContextFrameType frame_type_;
  const mojom::ServiceWorkerClientLifecycleState lifecycle_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CLIENT_H_
