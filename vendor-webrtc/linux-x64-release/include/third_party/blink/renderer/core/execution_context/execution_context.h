/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 * Copyright (C) 2012 Google Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_H_

#include <memory>

#include "base/notreached.h"
#include "base/task/single_thread_task_runner.h"
#include "net/storage_access_api/status.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-blink-forward.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/devtools/inspector_issue.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/permissions_policy/policy_disposition.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/v8_cache_options.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/sanitize_script_errors.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/security_context.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/platform/feature_context.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap_observer_list.h"
#include "third_party/blink/renderer/platform/loader/fetch/https_state.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/use_counter_and_console_logger.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"
#include "v8/include/v8-callbacks.h"
#include "v8/include/v8-forward.h"

namespace base {
class UnguessableToken;
}  // namespace base

namespace ukm {
class UkmRecorder;
}  // namespace ukm

namespace v8 {
class MicrotaskQueue;
}  // namespace v8

namespace perfetto::protos::pbzero {
class BlinkExecutionContext;
}  // namespace perfetto::protos::pbzero

namespace blink {

class Agent;
class AuditsIssue;
class CodeCacheHost;
class ConsoleMessage;
class ContentSecurityPolicy;
class ContentSecurityPolicyDelegate;
class ContextLifecycleObserver;
class CoreProbeSink;
class DOMWrapperWorld;
class ErrorEvent;
class EventTarget;
class FrameOrWorkerScheduler;
class KURL;
class LocalDOMWindow;
class OriginTrialContext;
class RuntimeFeatureStateOverrideContext;
class PolicyContainer;
class PublicURLManager;
class ResourceFetcher;
class SecurityOrigin;
class ScriptState;
class ScriptWrappable;
class TrustedTypePolicyFactory;

enum ReasonForCallingCanExecuteScripts {
  kAboutToExecuteScript,
  kNotAboutToExecuteScript
};

enum ReferrerPolicySource { kPolicySourceHttpHeader, kPolicySourceMetaTag };

// An environment in which script can execute. This class exposes the common
// properties of script execution environments on the web (i.e, common between
// script executing in a window and script executing in a worker), such as:
//
// - a base URL for the resolution of relative URLs
// - a security context that defines the privileges associated with the
//   environment (note, however, that specific isolated script contexts may
//   still enjoy elevated privileges)
// - affordances for the activity (including script and active DOM objects) to
//   be paused or terminated, e.g. because a frame has entered the background or
//   been closed permanently
// - a console logging facility for debugging
//
// Typically, the ExecutionContext is an instance of LocalDOMWindow or of
// WorkerOrWorkletGlobalScope.
//
// Note that this is distinct from the notion of a ScriptState or v8::Context,
// which are associated with a single script context (with a single global
// object). For example, there are separate JavaScript globals for "main world"
// script written by a web author and an "isolated world" content script written
// by an extension developer, but these share an ExecutionContext (the window)
// in common.
class CORE_EXPORT ExecutionContext : public Supplementable<ExecutionContext>,
                                     public MojoBindingContext,
                                     public UseCounterAndConsoleLogger,
                                     public FeatureContext {
 public:
  ExecutionContext(const ExecutionContext&) = delete;
  ExecutionContext& operator=(const ExecutionContext&) = delete;

  void Trace(Visitor*) const override;

  static ExecutionContext* From(const ScriptState*);
  static ExecutionContext* From(v8::Local<v8::Context>);

  // Returns the CodeCacheHost interface associated with the execution
  // context. This could return nullptr if there is no CodeCacheHost associated
  // with the current execution context.
  static CodeCacheHost* GetCodeCacheHostFromContext(ExecutionContext*);

  virtual bool IsWindow() const { return false; }
  virtual bool IsWorkerOrWorkletGlobalScope() const { return false; }
  virtual bool IsWorkerGlobalScope() const { return false; }
  virtual bool IsWorkletGlobalScope() const { return false; }
  virtual bool IsMainThreadWorkletGlobalScope() const { return false; }
  virtual bool IsDedicatedWorkerGlobalScope() const { return false; }
  virtual bool IsSharedWorkerGlobalScope() const { return false; }
  virtual bool IsServiceWorkerGlobalScope() const { return false; }
  virtual bool IsAnimationWorkletGlobalScope() const { return false; }
  virtual bool IsAudioWorkletGlobalScope() const { return false; }
  virtual bool IsLayoutWorkletGlobalScope() const { return false; }
  virtual bool IsPaintWorkletGlobalScope() const { return false; }
  virtual bool IsThreadedWorkletGlobalScope() const { return false; }
  virtual bool IsShadowRealmGlobalScope() const { return false; }
  virtual bool IsSharedStorageWorkletGlobalScope() const { return false; }
  virtual bool IsJSExecutionForbidden() const { return false; }

  // Notifies the execution context that new web socket activity (such as
  // sending or receiving a message) has happened.
  virtual void NotifyWebSocketActivity() {}

  // TODO(crbug.com/1335924) Change this method to be pure-virtual and each
  // derivative explicitly override it.
  virtual bool IsInFencedFrame() const { return false; }

  virtual bool IsContextThread() const { return true; }

  virtual bool ShouldInstallV8Extensions() const { return false; }

  virtual void CountUseOnlyInCrossSiteIframe(mojom::blink::WebFeature feature) {
  }

  // Return the associated AgentGroupScheduler's compositor tasl runner.
  virtual scoped_refptr<base::SingleThreadTaskRunner>
  GetAgentGroupSchedulerCompositorTaskRunner() {
    return nullptr;
  }

  const SecurityOrigin* GetSecurityOrigin() const;
  SecurityOrigin* GetMutableSecurityOrigin();

  ContentSecurityPolicy* GetContentSecurityPolicy() const;
  void SetContentSecurityPolicy(ContentSecurityPolicy* content_security_policy);
  void SetRequireTrustedTypes();
  void SetRequireTrustedTypesForTesting();

  network::mojom::blink::WebSandboxFlags GetSandboxFlags() const;
  bool IsSandboxed(network::mojom::blink::WebSandboxFlags mask) const;

  // Returns a reference to the current world we are in. If the current v8
  // context is empty, returns null.
  const DOMWrapperWorld* GetCurrentWorld() const;

  // Returns the content security policy to be used based on the current
  // JavaScript world we are in.
  ContentSecurityPolicy* GetContentSecurityPolicyForCurrentWorld();

  // Returns the content security policy to be used for the given |world|.
  virtual ContentSecurityPolicy* GetContentSecurityPolicyForWorld(
      const DOMWrapperWorld* world);

  virtual const KURL& Url() const = 0;
  virtual const KURL& BaseURL() const = 0;
  virtual KURL CompleteURL(const String& url) const = 0;
  virtual void DisableEval(const String& error_message) = 0;
  virtual void SetWasmEvalErrorMessage(const String& error_message) = 0;
  virtual String UserAgent() const = 0;
  virtual UserAgentMetadata GetUserAgentMetadata() const {
    return UserAgentMetadata();
  }

  virtual HttpsState GetHttpsState() const = 0;

  virtual ResourceFetcher* Fetcher() = 0;

  SecurityContext& GetSecurityContext() { return security_context_; }
  const SecurityContext& GetSecurityContext() const {
    return security_context_;
  }

  // https://tc39.github.io/ecma262/#sec-agent-clusters
  const base::UnguessableToken& GetAgentClusterID() const;

  bool IsSameAgentCluster(const base::UnguessableToken&) const;

  virtual bool CanExecuteScripts(ReasonForCallingCanExecuteScripts) {
    return false;
  }
  virtual mojom::blink::V8CacheOptions GetV8CacheOptions() const;

  void DispatchErrorEvent(ErrorEvent*, SanitizeScriptErrors);

  virtual void ExceptionThrown(ErrorEvent*) = 0;

  PublicURLManager& GetPublicURLManager();

  ContentSecurityPolicyDelegate& GetContentSecurityPolicyDelegate();

  virtual void RemoveURLFromMemoryCache(const KURL&);

  virtual void SetIsInBackForwardCache(bool);
  bool is_in_back_forward_cache() const { return is_in_back_forward_cache_; }

  void SetLifecycleState(mojom::FrameLifecycleState);
  virtual void NotifyContextDestroyed();

  using ConsoleLogger::AddConsoleMessage;

  void AddConsoleMessage(ConsoleMessage* message,
                         bool discard_duplicates = false) {
    AddConsoleMessageImpl(message, discard_duplicates);
  }
  virtual void AddInspectorIssue(AuditsIssue) = 0;

  void CountDeprecation(WebFeature feature) override;

  bool IsContextPaused() const;
  LoaderFreezeMode GetLoaderFreezeMode() const;
  mojom::FrameLifecycleState ContextPauseState() const {
    return lifecycle_state_;
  }
  bool IsContextFrozenOrPaused() const;

  // Gets the next id in a circular sequence from 1 to 2^31-1.
  int CircularSequentialID();

  virtual EventTarget* ErrorEventTarget() = 0;

  // Methods related to window interaction. It should be used to manage window
  // focusing and window creation permission for an ExecutionContext.
  void AllowWindowInteraction();
  void ConsumeWindowInteraction();
  bool IsWindowInteractionAllowed() const;

  // Decides whether this context is privileged, as described in
  // https://w3c.github.io/webappsec-secure-contexts/#is-settings-object-contextually-secure.
  SecureContextMode GetSecureContextMode() const {
    return security_context_.GetSecureContextMode();
  }
  virtual bool IsSecureContext() const {
    return GetSecureContextMode() == SecureContextMode::kSecureContext;
  }
  bool IsSecureContext(String& error_message) const;

  virtual bool HasInsecureContextInAncestors() const { return false; }

  // Returns a referrer to be used in the "Determine request's Referrer"
  // algorithm defined in the Referrer Policy spec.
  // https://w3c.github.io/webappsec-referrer-policy/#determine-requests-referrer
  virtual String OutgoingReferrer() const;

  // Parses a referrer policy directive using either Header or Meta rules and
  // sets the context to use that policy. If the supplied policy is invalid,
  // the context's policy is unchanged and a message is logged to the console.
  //
  // For a header-set policy, parses a comma-delimited list of tokens, and sets
  // the context's policy to the last one that is a valid policy. For a meta-set
  // policy, accepts only a single token, and allows the legacy tokens defined
  // in the HTML specification.
  void ParseAndSetReferrerPolicy(const String& policy,
                                 ReferrerPolicySource source);
  void SetReferrerPolicy(network::mojom::ReferrerPolicy);
  network::mojom::ReferrerPolicy GetReferrerPolicy() const;

  PolicyContainer* GetPolicyContainer() { return policy_container_.get(); }
  const PolicyContainer* GetPolicyContainer() const {
    return policy_container_.get();
  }
  void SetPolicyContainer(std::unique_ptr<PolicyContainer> container);
  std::unique_ptr<PolicyContainer> TakePolicyContainer();

  virtual CoreProbeSink* GetProbeSink() { return nullptr; }

  virtual FrameOrWorkerScheduler* GetScheduler() = 0;

  v8::Isolate* GetIsolate() const { return isolate_; }
  Agent* GetAgent() const { return agent_.Get(); }

  v8::MicrotaskQueue* GetMicrotaskQueue() const;

  OriginTrialContext* GetOriginTrialContext() const {
    return origin_trial_context_.Get();
  }

  RuntimeFeatureStateOverrideContext* GetRuntimeFeatureStateOverrideContext()
      const override {
    return runtime_feature_state_override_context_.Get();
  }

  virtual TrustedTypePolicyFactory* GetTrustedTypes() const { return nullptr; }
  virtual bool RequireTrustedTypes() const;

  // FeatureContext override
  bool FeatureEnabled(mojom::blink::OriginTrialFeature) const override;

  // Tests whether the policy-controlled feature is enabled in this frame.
  // Optionally sends a report to any registered reporting observers or
  // Report-To endpoints, via ReportPermissionsPolicyViolation(), if the feature
  // is disabled. The optional ConsoleMessage will be sent to the console if
  // present, or else a default message will be used instead.
  bool IsFeatureEnabled(network::mojom::PermissionsPolicyFeature) const;
  bool IsFeatureEnabled(
      network::mojom::PermissionsPolicyFeature,
      ReportOptions report_option = ReportOptions::kDoNotReport,
      const String& message = g_empty_string);

  bool IsFeatureEnabled(mojom::blink::DocumentPolicyFeature) const;
  bool IsFeatureEnabled(mojom::blink::DocumentPolicyFeature,
                        PolicyValue threshold_value) const;
  bool IsFeatureEnabled(
      mojom::blink::DocumentPolicyFeature,
      ReportOptions report_option = ReportOptions::kDoNotReport,
      const String& message = g_empty_string,
      const String& source_file = g_empty_string);
  bool IsFeatureEnabled(
      mojom::blink::DocumentPolicyFeature,
      PolicyValue threshold_value,
      ReportOptions report_option = ReportOptions::kDoNotReport,
      const String& message = g_empty_string,
      const String& source_file = g_empty_string);

  // Report policy violations is delegated to Document because in order
  // to both remain const qualified and output console message, needs
  // to call |frame_->Console().AddMessage()| directly.
  virtual void ReportPermissionsPolicyViolation(
      network::mojom::PermissionsPolicyFeature,
      mojom::blink::PolicyDisposition,
      const String& reporting_endpoint,
      const String& message = g_empty_string) const {}
  virtual void ReportPotentialPermissionsPolicyViolation(
      network::mojom::PermissionsPolicyFeature,
      mojom::blink::PolicyDisposition,
      const String& reporting_endpoint,
      const String& message = g_empty_string,
      const String& allow_attribute = g_empty_string,
      const String& src_attribute = g_empty_string) const {}
  virtual void ReportDocumentPolicyViolation(
      mojom::blink::DocumentPolicyFeature,
      mojom::blink::PolicyDisposition,
      const String& message = g_empty_string,
      const String& source_file = g_empty_string) const {}

  HeapObserverList<ContextLifecycleObserver>& ContextLifecycleObserverSet();
  unsigned ContextLifecycleStateObserverCountForTesting() const;

  // Implementation of WindowOrWorkerGlobalScope.crossOriginIsolated.
  // https://html.spec.whatwg.org/C/webappapis.html#concept-settings-object-cross-origin-isolated-capability
  virtual bool CrossOriginIsolatedCapability() const = 0;

  // Allows --disable-web-security (via `Agent::IsWebSecurityDisabled()`) to
  // override `CrossOriginIsolatedCapability()` .
  bool CrossOriginIsolatedCapabilityOrDisabledWebSecurity() const;

  // Returns true if scripts within this ExecutionContext are allowed to use
  // Trusted Context APIs (i.e. annotated with [IsolatedContext] IDL attribute).
  //
  // TODO(mkwst): We need a specification for the necessary restrictions.
  virtual bool IsIsolatedContext() const = 0;

  // Returns true if scripts within this ExecutionContext are considered
  // sufficiently protected from injection attacks (e.g. by enforcing a strict
  // CSP, a la https://csp.withgoogle.com/docs/strict-csp.html.
  bool IsInjectionMitigatedContext() const;

  // Returns true if SharedArrayBuffers can be transferred via PostMessage,
  // false otherwise. SharedArrayBuffer allows pages to craft high-precision
  // timers useful for Spectre-style side channel attacks, so are restricted
  // to cross-origin isolated contexts.
  bool SharedArrayBufferTransferAllowed() const;
  // Returns SharedArrayBufferTransferAllowed() but potentially reports an
  // inspector issue if the transfer was disallowed, or will be disallowed in
  // the future.
  bool CheckSharedArrayBufferTransferAllowedAndReport();

  virtual ukm::UkmRecorder* UkmRecorder() = 0;
  virtual ukm::SourceId UkmSourceID() const = 0;

  // Returns the token that uniquely identifies this ExecutionContext.
  virtual ExecutionContextToken GetExecutionContextToken() const = 0;

  // Returns the token that uniquely identifies the parent ExecutionContext of
  // this context. If an ExecutionContext has a parent context, it means that it
  // was created from that context, and the lifetime of this context is tied to
  // the lifetime of its parent. This is used for resource usage attribution,
  // where the resource usage of a child context will be charged to its parent
  // (and so on up the tree).
  virtual std::optional<ExecutionContextToken> GetParentExecutionContextToken()
      const {
    return std::nullopt;
  }

  // ExecutionContext subclasses are usually the V8 global object, which means
  // they are also a ScriptWrappable. This casts the ExecutionContext to a
  // ScriptWrappable if possible.
  virtual ScriptWrappable* ToScriptWrappable() { NOTREACHED(); }

  bool has_filed_shared_array_buffer_creation_issue() const {
    return has_filed_shared_array_buffer_creation_issue_;
  }

  void FileSharedArrayBufferCreationIssue();

  bool IsInRequestAnimationFrame() const {
    return is_in_request_animation_frame_;
  }

  // Write a representation of this object into a trace.
  using Proto = perfetto::protos::pbzero::BlinkExecutionContext;
  void WriteIntoTrace(perfetto::TracedProto<Proto> proto) const;

  // For use by FrameRequestCallbackCollection::ExecuteFrameCallbacks();
  // IsInRequestAnimationFrame() for the corresponding ExecutionContext will
  // return true while this instance exists.
  class ScopedRequestAnimationFrameStatus {
    STACK_ALLOCATED();

   public:
    explicit ScopedRequestAnimationFrameStatus(ExecutionContext* context)
        : context_(context) {
      DCHECK(!context_->is_in_request_animation_frame_);
      context_->is_in_request_animation_frame_ = true;
    }
    ~ScopedRequestAnimationFrameStatus() {
      context_->is_in_request_animation_frame_ = false;
    }

   private:
    ExecutionContext* context_;
  };

  // Returns the context's Storage Access API status.
  virtual net::StorageAccessApiStatus GetStorageAccessApiStatus() const {
    return net::StorageAccessApiStatus::kNone;
  }

 protected:
  ExecutionContext(v8::Isolate* isolate, Agent* agent, bool is_window = false);
  ~ExecutionContext() override;

  // Resetting the Agent is only necessary for a special case related to the
  // GetShouldReuseGlobalForUnownedMainFrame() Setting.
  void ResetAgent(Agent* agent) { agent_ = agent; }

 private:
  // ConsoleLogger implementation.
  void AddConsoleMessageImpl(
      mojom::blink::ConsoleMessageSource,
      mojom::blink::ConsoleMessageLevel,
      const String& message,
      bool discard_duplicates,
      std::optional<mojom::ConsoleMessageCategory> category) override;
  void AddConsoleMessageImpl(ConsoleMessage*,
                             bool discard_duplicates) override = 0;

  v8::Isolate* const isolate_;

  SecurityContext security_context_;

  Member<Agent> agent_;

  bool DispatchErrorEventInternal(ErrorEvent*, SanitizeScriptErrors);

  unsigned circular_sequential_id_;

  bool in_dispatch_error_event_;
  HeapVector<Member<ErrorEvent>> pending_exceptions_;

  mojom::FrameLifecycleState lifecycle_state_;

  bool is_in_back_forward_cache_ = false;

  bool has_filed_shared_array_buffer_transfer_issue_ = false;
  bool has_filed_shared_array_buffer_creation_issue_ = false;

  bool is_in_request_animation_frame_ = false;

  Member<PublicURLManager> public_url_manager_;

  const Member<ContentSecurityPolicyDelegate> csp_delegate_;

  // Counter that keeps track of how many window interaction calls are allowed
  // for this ExecutionContext. Callers are expected to call
  // |allowWindowInteraction()| and |consumeWindowInteraction()| in order to
  // increment and decrement the counter.
  int window_interaction_tokens_;

  // The |policy_container_| contains security policies for this
  // ExecutionContext.
  std::unique_ptr<PolicyContainer> policy_container_;

  Member<OriginTrialContext> origin_trial_context_;

  Member<ContentSecurityPolicy> content_security_policy_;

  Member<RuntimeFeatureStateOverrideContext>
      runtime_feature_state_override_context_;

  bool require_trusted_types_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_H_
