// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_FETCH_RESPOND_WITH_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_FETCH_RESPOND_WITH_OBSERVER_H_

#include "base/task/single_thread_task_runner.h"
#include "services/network/public/cpp/cross_origin_embedder_policy.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/respond_with_observer.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class CrossOriginResourcePolicyChecker;
class ExecutionContext;
class FetchEvent;
class ReadableStream;
class Response;
class WaitUntilObserver;

namespace mojom {
namespace blink {
class FetchAPIRequest;
}  // namespace blink
}  // namespace mojom

// This class observes the service worker's handling of a FetchEvent and
// notifies the client.
class MODULES_EXPORT FetchRespondWithObserver : public RespondWithObserver {
 public:
  FetchRespondWithObserver(
      ExecutionContext*,
      int fetch_event_id,
      base::WeakPtr<CrossOriginResourcePolicyChecker> corp_checker,
      const mojom::blink::FetchAPIRequest&,
      WaitUntilObserver*);
  ~FetchRespondWithObserver() override = default;

  void OnResponseFulfilled(ScriptState*, Response*);
  void OnResponseRejected(mojom::ServiceWorkerResponseError) override;
  void OnNoResponse(ScriptState*) override;

  void SetEvent(FetchEvent* event);

  void Trace(Visitor*) const override;

 private:
  const KURL request_url_;
  const network::mojom::RequestMode request_mode_;
  const network::mojom::RedirectMode redirect_mode_;
  const mojom::RequestContextFrameType frame_type_;
  const network::mojom::RequestDestination request_destination_;
  Member<FetchEvent> event_;
  Member<ReadableStream> original_request_body_stream_;
  // https://fetch.spec.whatwg.org/#concept-body-source
  const bool request_body_has_source_;
  const bool range_request_;
  base::WeakPtr<CrossOriginResourcePolicyChecker> corp_checker_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_FETCH_RESPOND_WITH_OBSERVER_H_
