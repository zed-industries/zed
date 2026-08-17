// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_H_

#include <memory>

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/inspector/console_message.h"
#include "third_party/blink/renderer/core/workers/worker_or_worklet_global_scope.h"
#include "third_party/blink/renderer/core/workers/worklet_module_responses_map.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/code_cache_host.h"
#include "third_party/blink/renderer/platform/mojo/browser_interface_broker_proxy_impl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class FetchClientSettingsObjectSnapshot;
class WorkletPendingTasks;
class WorkerReportingProxy;
struct GlobalScopeCreationParams;

// This is an implementation of the web-exposed WorkletGlobalScope interface
// defined in the Worklets spec:
// https://drafts.css-houdini.org/worklets/#workletglobalscope
//
// This instance lives either on the main thread (main thread worklet) or a
// worker thread (threaded worklet). It's determined by constructors. See
// comments on the constructors.
class CORE_EXPORT WorkletGlobalScope
    : public WorkerOrWorkletGlobalScope,
      public ActiveScriptWrappable<WorkletGlobalScope> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~WorkletGlobalScope() override;

  bool IsMainThreadWorkletGlobalScope() const final;
  bool IsThreadedWorkletGlobalScope() const final;
  bool IsWorkletGlobalScope() const final { return true; }

  // Always returns false here as PaintWorkletGlobalScope and
  // AnimationWorkletGlobalScope don't have a #close() method on the global.
  // Note that AudioWorkletGlobal overrides this behavior.
  bool IsClosing() const override { return false; }

  ExecutionContext* GetExecutionContext() const override;

  // ExecutionContext
  const KURL& Url() const final { return url_; }
  const KURL& BaseURL() const final { return url_; }
  KURL CompleteURL(const String&) const final;
  String UserAgent() const final { return user_agent_; }
  bool IsContextThread() const final;
  void AddConsoleMessageImpl(ConsoleMessage*, bool discard_duplicates) final;
  void AddInspectorIssue(AuditsIssue) final;
  void ExceptionThrown(ErrorEvent*) final;
  CoreProbeSink* GetProbeSink() final;
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType) final;
  FrameOrWorkerScheduler* GetScheduler() final;
  bool CrossOriginIsolatedCapability() const final;
  bool IsIsolatedContext() const final;
  ukm::UkmRecorder* UkmRecorder() final;
  ukm::SourceId UkmSourceID() const final;

  // WorkerOrWorkletGlobalScope
  void Dispose() override;
  WorkerThread* GetThread() const final;
  const base::UnguessableToken& GetDevToolsToken() const override;
  CodeCacheHost* GetCodeCacheHost() override;
  std::optional<mojo::PendingRemote<network::mojom::blink::URLLoaderFactory>>
  FindRaceNetworkRequestURLLoaderFactory(
      const base::UnguessableToken& token) override {
    return std::nullopt;
  }

  // Returns `blob_url_store_pending_remote_` for use when instantiating the
  // PublicURLManager in threaded worklet contexts. This method should only be
  // called once. See `blob_url_store_pending_remote_` for more details.
  mojo::PendingRemote<mojom::blink::BlobURLStore>
  TakeBlobUrlStorePendingRemote();

  virtual LocalFrame* GetFrame() const;

  // Implementation of the "fetch and invoke a worklet script" algorithm:
  // https://drafts.css-houdini.org/worklets/#fetch-and-invoke-a-worklet-script
  // When script evaluation is done or any exception happens, it's notified to
  // the given WorkletPendingTasks via |outside_settings_task_runner| (i.e., the
  // parent frame's task runner).
  void FetchAndInvokeScript(
      const KURL& module_url_record,
      network::mojom::CredentialsMode,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      scoped_refptr<base::SingleThreadTaskRunner> outside_settings_task_runner,
      WorkletPendingTasks*);

  WorkletModuleResponsesMap* GetModuleResponsesMap() const {
    return module_responses_map_.Get();
  }

  const SecurityOrigin* DocumentSecurityOrigin() const {
    return document_security_origin_.get();
  }

  // Customize the security context used for origin trials.
  // Origin trials are only enabled in secure contexts, but WorkletGlobalScopes
  // are defined to have a unique, opaque origin, so are not secure:
  // https://drafts.css-houdini.org/worklets/#script-settings-for-worklets
  // For origin trials, instead consider the context of the document which
  // created the worklet, since the origin trial tokens are inherited from the
  // document.
  bool DocumentSecureContext() const { return IsCreatorSecureContext(); }

  void Trace(Visitor*) const override;

  // ActiveScriptWrappable.
  bool HasPendingActivity() const override;

  HttpsState GetHttpsState() const override { return https_state_; }

  // Constructs an instance as a main thread worklet. Must be called on the main
  // thread.
  WorkletGlobalScope(std::unique_ptr<GlobalScopeCreationParams>,
                     WorkerReportingProxy&,
                     LocalFrame*);

  // Constructs an instance as a threaded worklet. Must be called on a worker
  // thread.
  WorkletGlobalScope(std::unique_ptr<GlobalScopeCreationParams>,
                     WorkerReportingProxy&,
                     WorkerThread*);

  const BrowserInterfaceBrokerProxy& GetBrowserInterfaceBroker() const override;

  // Returns the WorkletToken that uniquely identifies this worklet.
  virtual WorkletToken GetWorkletToken() const = 0;

  // Returns the ExecutionContextToken that uniquely identifies the parent
  // context that created this worklet. Note that this will always be a
  // LocalFrameToken.
  std::optional<ExecutionContextToken> GetParentExecutionContextToken()
      const final {
    return frame_token_;
  }

 private:
  enum class ThreadType {
    // Indicates this global scope lives on the main thread.
    kMainThread,
    // Indicates this global scope lives on a worker thread.
    kOffMainThread
  };

  // The base constructor delegated from other public constructors. This
  // partially implements the "set up a worklet environment settings object"
  // algorithm defined in the Worklets spec:
  // https://drafts.css-houdini.org/worklets/#script-settings-for-worklets
  WorkletGlobalScope(std::unique_ptr<GlobalScopeCreationParams>,
                     WorkerReportingProxy&,
                     v8::Isolate*,
                     ThreadType,
                     LocalFrame*,
                     WorkerThread*);

  // Returns a destination used for fetching worklet scripts.
  // https://html.spec.whatwg.org/C/#worklet-destination-type
  virtual network::mojom::RequestDestination GetDestination() const = 0;

  EventTarget* ErrorEventTarget() final { return nullptr; }

  // The |url_| and |user_agent_| are inherited from the parent Document.
  const KURL url_;
  const String user_agent_;

  // Used for module fetch and origin trials, inherited from the parent
  // Document.
  const scoped_refptr<const SecurityOrigin> document_security_origin_;

  CrossThreadPersistent<WorkletModuleResponsesMap> module_responses_map_;

  const HttpsState https_state_;

  const ThreadType thread_type_;
  // |frame_| is available only when |thread_type_| is kMainThread.
  Member<LocalFrame> frame_;
  // |worker_thread_| is available only when |thread_type_| is kOffMainThread.
  WorkerThread* worker_thread_;

  // The token identifying the LocalFrame that caused this scope to be created.
  const LocalFrameToken frame_token_;

  std::unique_ptr<ukm::UkmRecorder> ukm_recorder_;

  // This is inherited at construction to make sure it is possible to used
  // restricted API between the document and the worklet (e.g.
  // SharedArrayBuffer passing via postMessage).
  const bool parent_cross_origin_isolated_capability_;

  // This is inherited at construction to ensure it's possible to use APIs
  // like Direct Sockets if they're made available in Worklets.
  //
  // TODO(crbug.com/1206150): We need a spec for this capability.
  const bool parent_is_isolated_context_;

  // This is the interface that handles generated code cache
  // requests both to fetch code cache when loading resources
  // and to store generated code cache to disk.
  std::unique_ptr<CodeCacheHost> code_cache_host_;

  // The interface through which the worklet may request interfaces from the
  // browser. It's currently only set for SharedStorageWorklet.
  blink::BrowserInterfaceBrokerProxyImpl browser_interface_broker_proxy_;

  // A PendingRemote for use in threaded worklets that gets created from the
  // parent frame's BrowserInterfaceBroker and used when instantiating the
  // worklet's PublicURLManager. This remote is used for Blob URL related
  // functionality such as registering, revoking, and navigating to Blob URLs.
  mojo::PendingRemote<mojom::blink::BlobURLStore>
      blob_url_store_pending_remote_;
};

template <>
struct DowncastTraits<WorkletGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsWorkletGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_H_
