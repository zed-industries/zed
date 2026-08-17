// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_CALLBACKS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_CALLBACKS_H_

#include "base/gtest_prod_util.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ControllerPresentationConnection;
class PresentationConnection;
class PresentationRequest;

// PresentationConnectionCallbacks resolves or rejects the provided resolver's
// underlying promise depending on the result passed to the callback. On
// success, the promise will be resolved with a newly created
// ControllerPresentationConnection. In the case of reconnect, the callback may
// take an existing connection object with which the promise will be resolved
// on success.
class MODULES_EXPORT PresentationConnectionCallbacks final {
  USING_FAST_MALLOC(PresentationConnectionCallbacks);

 public:
  PresentationConnectionCallbacks(
      ScriptPromiseResolver<PresentationConnection>*,
      PresentationRequest*);
  PresentationConnectionCallbacks(
      ScriptPromiseResolver<PresentationConnection>*,
      ControllerPresentationConnection*);

  PresentationConnectionCallbacks(const PresentationConnectionCallbacks&) =
      delete;
  PresentationConnectionCallbacks& operator=(
      const PresentationConnectionCallbacks&) = delete;

  ~PresentationConnectionCallbacks() = default;

  void HandlePresentationResponse(mojom::blink::PresentationConnectionResultPtr,
                                  mojom::blink::PresentationErrorPtr);

 private:
  FRIEND_TEST_ALL_PREFIXES(PresentationConnectionCallbacksTest, HandleError);
  FRIEND_TEST_ALL_PREFIXES(PresentationConnectionCallbacksTest, HandleSuccess);
  FRIEND_TEST_ALL_PREFIXES(PresentationConnectionCallbacksTest,
                           HandleReconnect);

  void OnSuccess(const mojom::blink::PresentationInfo&,
                 mojo::PendingRemote<mojom::blink::PresentationConnection>
                     connection_remote,
                 mojo::PendingReceiver<mojom::blink::PresentationConnection>
                     connection_receiver);
  void OnError(const mojom::blink::PresentationError&);

  Persistent<ScriptPromiseResolver<PresentationConnection>> resolver_;
  Persistent<PresentationRequest> request_;
  WeakPersistent<ControllerPresentationConnection> connection_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_CALLBACKS_H_
