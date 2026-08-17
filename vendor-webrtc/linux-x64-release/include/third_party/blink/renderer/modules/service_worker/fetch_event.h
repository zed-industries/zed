// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_FETCH_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_FETCH_EVENT_H_

#include <memory>

#include "third_party/blink/public/mojom/timing/performance_mark_or_measure.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_fetch_event_init.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/fetch/request.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/modules/service_worker/wait_until_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/data_pipe_bytes_consumer.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class ExceptionState;
class FetchRespondWithObserver;
class PerformanceMark;
class PerformanceMeasure;
class Request;
class Response;
class ScriptState;
struct WebServiceWorkerError;
class WebURLResponse;
class WorkerGlobalScope;

// A fetch event is dispatched by the client to a service worker's script
// context. FetchRespondWithObserver can be used to notify the client about the
// service worker's response.
class MODULES_EXPORT FetchEvent final
    : public ExtendableEvent,
      public ActiveScriptWrappable<FetchEvent>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using PreloadResponseProperty = ScriptPromiseProperty<IDLAny, DOMException>;
  static FetchEvent* Create(ScriptState*,
                            const AtomicString& type,
                            const FetchEventInit*);

  FetchEvent(ScriptState*,
             const AtomicString& type,
             const FetchEventInit*,
             FetchRespondWithObserver*,
             WaitUntilObserver*,
             bool navigation_preload_sent);
  ~FetchEvent() override;

  Request* request() const;
  String clientId() const;
  String resultingClientId() const;
  bool isReload() const;

  void respondWith(ScriptState*, ScriptPromise<Response>, ExceptionState&);
  ScriptPromise<IDLAny> preloadResponse(ScriptState*);
  ScriptPromise<IDLUndefined> handled(ScriptState*);

  void ResolveHandledPromise();
  void RejectHandledPromise(const String& error_message);

  void addPerformanceEntry(PerformanceMark*);
  void addPerformanceEntry(PerformanceMeasure*);

  void OnNavigationPreloadResponse(ScriptState*,
                                   std::unique_ptr<WebURLResponse>,
                                   mojo::ScopedDataPipeConsumerHandle);
  void OnNavigationPreloadError(ScriptState*,
                                std::unique_ptr<WebServiceWorkerError>);
  void OnNavigationPreloadComplete(WorkerGlobalScope*,
                                   base::TimeTicks completion_time,
                                   int64_t encoded_data_length,
                                   int64_t encoded_body_length,
                                   int64_t decoded_body_length);

  const AtomicString& InterfaceName() const override;

  // ScriptWrappable
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

 private:
  Member<FetchRespondWithObserver> observer_;
  Member<Request> request_;
  Member<PreloadResponseProperty> preload_response_property_;
  std::unique_ptr<WebURLResponse> preload_response_;
  Member<DataPipeBytesConsumer::CompletionNotifier> body_completion_notifier_;
  Member<ScriptPromiseProperty<IDLUndefined, DOMException>> handled_property_;
  String client_id_;
  String resulting_client_id_;
  bool is_reload_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_FETCH_EVENT_H_
