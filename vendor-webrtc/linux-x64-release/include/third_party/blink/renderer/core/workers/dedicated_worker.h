// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_H_

#include <memory>

#include "base/functional/function_ref.h"
#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "third_party/blink/public/common/loader/worker_main_script_load_parameters.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/worker/dedicated_worker_host.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_dedicated_worker.h"
#include "third_party/blink/public/platform/web_dedicated_worker_host_factory_client.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_structured_serialize_options.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_worker_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/messaging/message_port.h"
#include "third_party/blink/renderer/core/workers/abstract_worker.h"
#include "third_party/blink/renderer/core/workers/custom_event_message.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_client_settings_object_snapshot.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "v8/include/v8-inspector.h"

namespace blink {

class DedicatedWorkerMessagingProxy;
class ExceptionState;
class ExecutionContext;
class PostMessageOptions;
class ScriptState;
class WebContentSettingsClient;
class WorkerClassicScriptLoader;
struct GlobalScopeCreationParams;

// Implementation of the Worker interface defined in the WebWorker HTML spec:
// https://html.spec.whatwg.org/C/#worker
//
// Confusingly, the Worker interface is for dedicated workers, so this class is
// called DedicatedWorker. This lives on the thread that created the worker (the
// thread that called `new Worker()`), i.e., the main thread if a document
// created the worker or a worker thread in the case of nested workers.
class CORE_EXPORT DedicatedWorker final
    : public AbstractWorker,
      public ActiveScriptWrappable<DedicatedWorker>,
      public WebDedicatedWorker {
  DEFINE_WRAPPERTYPEINFO();
  // Pre-finalization is needed to notify the parent object destruction of the
  // GC-managed messaging proxy and to initiate worker termination.
  USING_PRE_FINALIZER(DedicatedWorker, Dispose);

 public:
  static DedicatedWorker* Create(ExecutionContext*,
                                 const String& url,
                                 const WorkerOptions*,
                                 ExceptionState&);

  DedicatedWorker(ExecutionContext*,
                  const KURL& script_request_url,
                  const WorkerOptions*);
  // Exposed for testing.
  DedicatedWorker(
      ExecutionContext*,
      const KURL& script_request_url,
      const WorkerOptions*,
      base::FunctionRef<DedicatedWorkerMessagingProxy*(DedicatedWorker*)>
          context_proxy_factory);
  ~DedicatedWorker() override;

  void Dispose();

  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   HeapVector<ScriptObject> transfer,
                   ExceptionState&);
  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   const PostMessageOptions*,
                   ExceptionState&);

  void PostCustomEvent(
      TaskType,
      ScriptState*,
      CrossThreadFunction<Event*(ScriptState*, CustomEventMessage)>
          event_factory_callback,
      CrossThreadFunction<Event*(ScriptState*)> event_factory_error_callback,
      const ScriptValue& message,
      HeapVector<ScriptObject> transfer,
      ExceptionState&);

  void terminate();

  // Implements ExecutionContextLifecycleObserver (via AbstractWorker).
  void ContextDestroyed() override;

  // Implements ScriptWrappable
  // (via AbstractWorker -> EventTarget -> EventTarget).
  bool HasPendingActivity() const final;

  // Implements WebDedicatedWorker.
  void OnWorkerHostCreated(
      CrossVariantMojoRemote<mojom::blink::BrowserInterfaceBrokerInterfaceBase>
          browser_interface_broker,
      CrossVariantMojoRemote<mojom::blink::DedicatedWorkerHostInterfaceBase>
          dedicated_worker_host,
      const WebSecurityOrigin& origin) override;
  void OnScriptLoadStarted(
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      CrossVariantMojoRemote<
          mojom::blink::BackForwardCacheControllerHostInterfaceBase>
          back_forward_cache_controller_host,
      CrossVariantMojoReceiver<mojom::blink::ReportingObserverInterfaceBase>
          coep_reporting_observer,
      CrossVariantMojoReceiver<mojom::blink::ReportingObserverInterfaceBase>
          dip_reporting_observer) override;
  void OnScriptLoadStartFailed() override;

  void DispatchErrorEventForScriptFetchFailure();

  // Returns a unique identifier for this worker, shared between the browser
  // process and this renderer. This is generated in the renderer process when
  // the worker is created, and it is subsequently communicated to the browser
  // process.
  const blink::DedicatedWorkerToken& GetToken() const { return token_; }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(message, kMessage)

  void ContextLifecycleStateChanged(mojom::FrameLifecycleState state) override;
  void Trace(Visitor*) const override;

 private:
  FRIEND_TEST_ALL_PREFIXES(DedicatedWorkerTest, TopLevelFrameSecurityOrigin);

  // Starts the worker.
  void Start();
  void ContinueStart(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      network::mojom::ReferrerPolicy,
      Vector<network::mojom::blink::ContentSecurityPolicyPtr>
          response_content_security_policies,
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          coep_reporting_observer,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          dip_reporting_observer);
  void ContinueStartInternal(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      network::mojom::ReferrerPolicy,
      Vector<network::mojom::blink::ContentSecurityPolicyPtr>
          response_content_security_policies,
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          coep_reporting_observer,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          dip_reporting_observer);
  std::unique_ptr<GlobalScopeCreationParams> CreateGlobalScopeCreationParams(
      const KURL& script_url,
      network::mojom::ReferrerPolicy,
      Vector<network::mojom::blink::ContentSecurityPolicyPtr>
          response_content_security_policies,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          coep_reporting_observer,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          dip_reporting_observer);

  scoped_refptr<WebWorkerFetchContext> CreateWebWorkerFetchContext();
  // May return nullptr.
  std::unique_ptr<WebContentSettingsClient> CreateWebContentSettingsClient();

  // Callbacks for |classic_script_loader_|.
  // TODO(crbug.com/400455021): Investigate whether these can be removed now
  // that PlzDedicatedWorker has shipped.
  void OnResponse();
  void OnFinished(
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host);

  // Implements EventTarget (via AbstractWorker -> EventTarget).
  const AtomicString& InterfaceName() const final;

  // The unique identifier for this DedicatedWorker. This is created in the
  // renderer process, and passed to the browser. This must be initialized
  // before |context_proxy_|.
  blink::DedicatedWorkerToken token_;

  const KURL script_request_url_;
  Member<const WorkerOptions> options_;
  Member<const FetchClientSettingsObjectSnapshot>
      outside_fetch_client_settings_object_;
  const Member<DedicatedWorkerMessagingProxy> context_proxy_;

  Member<WorkerClassicScriptLoader> classic_script_loader_;

  std::unique_ptr<WebDedicatedWorkerHostFactoryClient> factory_client_;

  // Used for tracking cross-debugger calls.
  v8_inspector::V8StackTraceId v8_stack_trace_id_;

  mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>
      browser_interface_broker_;

  // Passed to DedicatedWorkerMessagingProxy on worker thread start.
  mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
      pending_dedicated_worker_host_;

  // The timestamp taken when Start() is called.
  base::TimeTicks start_time_;

  // Whether the worker is frozen due to a call from this context.
  bool requested_frozen_ = false;

  // The origin used by this dedicated worker on the renderer side, calculated
  // from the browser side.
  scoped_refptr<SecurityOrigin> origin_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_H_
