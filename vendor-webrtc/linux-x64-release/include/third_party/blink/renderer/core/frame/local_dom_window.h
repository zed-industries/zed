/*
 * Copyright (C) 2006, 2007, 2009, 2010 Apple Inc. All rights reserved.
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_DOM_WINDOW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_DOM_WINDOW_H_

#include <memory>

#include "base/task/single_thread_task_runner.h"
#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink.h"
#include "services/network/public/mojom/storage_access_api.mojom-blink.h"
#include "third_party/blink/public/common/frame/delegated_capability_request_token.h"
#include "third_party/blink/public/common/frame/history_user_activation_state.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/editing/spellcheck/spell_checker.h"
#include "third_party/blink/renderer/core/editing/suggestion/text_suggestion_controller.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/dom_window.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/frame/universal_global_scope.h"
#include "third_party/blink/renderer/core/frame/use_counter_impl.h"
#include "third_party/blink/renderer/core/frame/window_event_handlers.h"
#include "third_party/blink/renderer/core/frame/window_or_worker_global_scope.h"
#include "third_party/blink/renderer/core/html/closewatcher/close_watcher.h"
#include "third_party/blink/renderer/core/loader/frame_loader.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/storage/blink_storage_key.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/uuid.h"

namespace blink {

class BarProp;
class CSSStyleDeclaration;
class CustomElementRegistry;
class Document;
class DocumentInit;
class DOMSelection;
class DOMViewport;
class DOMVisualViewport;
class Element;
class ExceptionState;
class External;
class Fence;
class FrameConsole;
class History;
class InputMethodController;
class LocalFrame;
class MediaQueryList;
class MessageEvent;
class Modulator;
class NavigationApi;
class Navigator;
class Screen;
class ScriptController;
class ScriptState;
class ScrollToOptions;
class SecurityOrigin;
class SerializedScriptValue;
class SourceLocation;
class StyleMedia;
class TrustedTypePolicyFactory;
class V8FrameRequestCallback;
struct WebPictureInPictureWindowOptions;
class WindowAgent;

namespace scheduler {
class TaskAttributionInfo;
}

enum PageTransitionEventPersistence {
  kPageTransitionEventNotPersisted = 0,
  kPageTransitionEventPersisted = 1
};

// Note: if you're thinking of returning something DOM-related by reference,
// please ping dcheng@chromium.org first. You probably don't want to do that.
class CORE_EXPORT LocalDOMWindow final : public DOMWindow,
                                         public ExecutionContext,
                                         public WindowOrWorkerGlobalScope,
                                         public UniversalGlobalScope,
                                         public WindowEventHandlers,
                                         public Supplementable<LocalDOMWindow> {
  USING_PRE_FINALIZER(LocalDOMWindow, Dispose);

 public:
  class CORE_EXPORT EventListenerObserver : public GarbageCollectedMixin {
   public:
    virtual void DidAddEventListener(LocalDOMWindow*, const AtomicString&) = 0;
    virtual void DidRemoveEventListener(LocalDOMWindow*,
                                        const AtomicString&) = 0;
    virtual void DidRemoveAllEventListeners(LocalDOMWindow*) = 0;
  };

  static LocalDOMWindow* From(const ScriptState*);

  LocalDOMWindow(LocalFrame&, WindowAgent*);
  ~LocalDOMWindow() override;

  // Returns the token identifying the frame that this ExecutionContext was
  // associated with at the moment of its creation. This remains valid even
  // after the frame has been destroyed and the ExecutionContext is detached.
  // This is used as a stable and persistent identifier for attributing detached
  // context memory usage.
  const LocalFrameToken& GetLocalFrameToken() const { return token_; }
  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

  LocalFrame* GetFrame() const {
    // UnsafeTo<> is safe here because DOMWindow's frame can only change to
    // nullptr, and it was constructed with a LocalFrame in the constructor.
    return UnsafeTo<LocalFrame>(DOMWindow::GetFrame());
  }

  ScriptController& GetScriptController() const { return *script_controller_; }

  void Initialize();
  void ClearForReuse();

  void ResetWindowAgent(WindowAgent*);

  mojom::blink::V8CacheOptions GetV8CacheOptions() const override;

  // Bind Content Security Policy to this window. This will cause the
  // CSP to resolve the 'self' attribute and all policies will then be
  // applied to this document.
  void BindContentSecurityPolicy();

  void Trace(Visitor*) const override;

  // ExecutionContext overrides:
  bool IsWindow() const final { return true; }
  bool IsContextThread() const final;
  bool ShouldInstallV8Extensions() const final;
  ContentSecurityPolicy* GetContentSecurityPolicyForWorld(
      const DOMWrapperWorld* world) final;
  const KURL& Url() const final;
  const KURL& BaseURL() const final;
  KURL CompleteURL(const String&) const final;
  void DisableEval(const String& error_message) final;
  void SetWasmEvalErrorMessage(const String& error_message) final;
  String UserAgent() const final;
  UserAgentMetadata GetUserAgentMetadata() const final;
  HttpsState GetHttpsState() const final;
  ResourceFetcher* Fetcher() final;
  bool CanExecuteScripts(ReasonForCallingCanExecuteScripts) final;
  void ExceptionThrown(ErrorEvent*) final;
  void AddInspectorIssue(AuditsIssue) final;
  EventTarget* ErrorEventTarget() final { return this; }
  String OutgoingReferrer() const final;
  CoreProbeSink* GetProbeSink() final;
  const BrowserInterfaceBrokerProxy& GetBrowserInterfaceBroker() const final;
  FrameOrWorkerScheduler* GetScheduler() final;
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType) final;
  TrustedTypePolicyFactory* GetTrustedTypes() const final {
    return GetTrustedTypesForWorld(*GetCurrentWorld());
  }
  ScriptWrappable* ToScriptWrappable() final { return this; }
  void ReportPermissionsPolicyViolation(
      network::mojom::PermissionsPolicyFeature,
      mojom::blink::PolicyDisposition,
      const String& reporting_endpoint,
      const String& message = g_empty_string) const final;
  void ReportPotentialPermissionsPolicyViolation(
      network::mojom::PermissionsPolicyFeature,
      mojom::blink::PolicyDisposition,
      const String& reporting_endpoint,
      const String& message = g_empty_string,
      const String& allow_attribute = g_empty_string,
      const String& src_attribute = g_empty_string) const final;
  void ReportDocumentPolicyViolation(
      mojom::blink::DocumentPolicyFeature,
      mojom::blink::PolicyDisposition,
      const String& message = g_empty_string,
      // If source_file is set to empty string,
      // current JS file would be used as source_file instead.
      const String& source_file = g_empty_string) const final;
  void SetIsInBackForwardCache(bool) final;
  net::StorageAccessApiStatus GetStorageAccessApiStatus() const final;

  void AddConsoleMessageImpl(ConsoleMessage*, bool discard_duplicates) final;

  scoped_refptr<base::SingleThreadTaskRunner>
  GetAgentGroupSchedulerCompositorTaskRunner() final;

  // UseCounter orverrides:
  void CountUse(mojom::WebFeature feature) final;
  void CountWebDXFeature(mojom::blink::WebDXFeature feature) final;

  // Count |feature| only when this window is associated with a cross-origin
  // iframe.
  void CountUseOnlyInCrossOriginIframe(mojom::blink::WebFeature feature);

  // Count |feature| only when this window is associated with a same-origin
  // iframe with the outermost main frame.
  void CountUseOnlyInSameOriginIframe(mojom::blink::WebFeature feature);

  // Count |feature| only when this window is associated with a cross-site
  // iframe. A "site" is a scheme and registrable domain.
  void CountUseOnlyInCrossSiteIframe(mojom::blink::WebFeature feature) override;

  // Count permissions policy feature usage through use counter.
  void CountPermissionsPolicyUsage(
      network::mojom::PermissionsPolicyFeature feature,
      UseCounterImpl::PermissionsPolicyUsageType type);

  // Checks if navigation to Javascript URL is allowed. This check should run
  // before any action is taken (e.g. creating new window) for all
  // same-origin navigations.
  String CheckAndGetJavascriptUrl(
      const DOMWrapperWorld* world,
      const KURL& url,
      Element* element,
      network::mojom::CSPDisposition csp_disposition =
          network::mojom::CSPDisposition::CHECK);

  Document* InstallNewDocument(const DocumentInit&);

  // EventTarget overrides:
  ExecutionContext* GetExecutionContext() const override;
  const LocalDOMWindow* ToLocalDOMWindow() const override;
  LocalDOMWindow* ToLocalDOMWindow() override;

  // Same-origin DOM Level 0
  Screen* screen();
  History* history();
  BarProp* locationbar();
  BarProp* menubar();
  BarProp* personalbar();
  BarProp* scrollbars();
  BarProp* statusbar();
  BarProp* toolbar();
  Navigator* navigator();
  Navigator* clientInformation() { return navigator(); }

  bool offscreenBuffering() const;

  int outerHeight() const;
  int outerWidth() const;
  int innerHeight() const;
  int innerWidth() const;
  int screenX() const;
  int screenY() const;
  int screenLeft() const { return screenX(); }
  int screenTop() const { return screenY(); }
  double scrollX() const;
  double scrollY() const;
  double pageXOffset() const { return scrollX(); }
  double pageYOffset() const { return scrollY(); }

  DOMViewport* viewport();
  DOMVisualViewport* visualViewport();

  const AtomicString& name() const;
  void setName(const AtomicString&);

  String status() const;
  void setStatus(const String&);
  String defaultStatus() const;
  void setDefaultStatus(const String&);
  String origin() const;

  // DOM Level 2 AbstractView Interface
  Document* document() const;

  // CSSOM View Module
  StyleMedia* styleMedia();

  // WebKit extensions
  double devicePixelRatio() const;

  // This is the interface orientation in degrees. Some examples are:
  //  0 is straight up; -90 is when the device is rotated 90 clockwise;
  //  90 is when rotated counter clockwise.
  int orientation() const;

  DOMSelection* getSelection();

  void print(ScriptState*);
  void stop();

  void alert(ScriptState*, const String& message = String());
  bool confirm(ScriptState*, const String& message);
  String prompt(ScriptState*,
                const String& message,
                const String& default_value);

  bool find(const String&,
            bool case_sensitive,
            bool backwards,
            bool wrap,
            bool whole_word,
            bool search_in_frames,
            bool show_dialog) const;

  // FIXME: ScrollBehaviorSmooth is currently unsupported in VisualViewport.
  // crbug.com/434497
  void scrollBy(double x, double y) const;
  void scrollBy(const ScrollToOptions*) const;
  void scrollTo(double x, double y) const;
  void scrollTo(const ScrollToOptions*) const;
  void scroll(double x, double y) const { scrollTo(x, y); }
  void scroll(const ScrollToOptions* scroll_to_options) const {
    scrollTo(scroll_to_options);
  }
  void moveBy(int x, int y) const;
  void moveTo(int x, int y) const;

  void resizeBy(int x, int y, ExceptionState&) const;
  void resizeTo(int width, int height, ExceptionState&) const;

  MediaQueryList* matchMedia(const String&);

  // DOM Level 2 Style Interface
  CSSStyleDeclaration* getComputedStyle(
      Element*,
      const String& pseudo_elt = String()) const;

  // WebKit animation extensions
  int requestAnimationFrame(V8FrameRequestCallback*);
  int webkitRequestAnimationFrame(V8FrameRequestCallback*);
  void cancelAnimationFrame(int id);

  // https://html.spec.whatwg.org/C/#dom-originagentcluster
  bool originAgentCluster() const;

  // Custom elements
  CustomElementRegistry* customElements(ScriptState*) const;
  CustomElementRegistry* customElements() const;
  CustomElementRegistry* MaybeCustomElements() const;

  void SetModulator(Modulator*);

  // Obsolete APIs
  void captureEvents() {}
  void releaseEvents() {}
  External* external();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(search, kSearch)

  DEFINE_ATTRIBUTE_EVENT_LISTENER(orientationchange, kOrientationchange)

  DEFINE_ATTRIBUTE_EVENT_LISTENER(pageswap, kPageswap)

  DEFINE_ATTRIBUTE_EVENT_LISTENER(pagereveal, kPagereveal)

  void RegisterEventListenerObserver(EventListenerObserver*);

  void FrameDestroyed();
  void Reset();

  Element* frameElement() const;

  DOMWindow* open(v8::Isolate*,
                  const String& url_string,
                  const AtomicString& target,
                  const String& features,
                  ExceptionState&);

  DOMWindow* openPictureInPictureWindow(
      v8::Isolate*,
      const WebPictureInPictureWindowOptions&);

  FrameConsole* GetFrameConsole() const;

  void PrintErrorMessage(const String&) const;

  void DispatchPostMessage(
      MessageEvent* event,
      scoped_refptr<const SecurityOrigin> intended_target_origin,
      std::unique_ptr<SourceLocation> location,
      const base::UnguessableToken& source_agent_cluster_id);

  void DispatchMessageEventWithOriginCheck(
      const SecurityOrigin* intended_target_origin,
      MessageEvent*,
      std::unique_ptr<SourceLocation>,
      const base::UnguessableToken& source_agent_cluster_id);

  // Events
  // EventTarget API
  void RemoveAllEventListeners() override;

  using EventTarget::DispatchEvent;
  DispatchEventResult DispatchEvent(Event&, EventTarget*);

  void FinishedLoading(FrameLoader::NavigationFinishState);

  // Dispatch the (deprecated) orientationchange event to this DOMWindow and
  // recurse on its child frames.
  void SendOrientationChangeEvent();

  void EnqueueWindowEvent(Event&, TaskType);
  void EnqueueDocumentEvent(Event&, TaskType);
  void EnqueueNonPersistedPageshowEvent();
  void EnqueueHashchangeEvent(const String& old_url, const String& new_url);
  void DispatchPopstateEvent(scoped_refptr<SerializedScriptValue>,
                             scheduler::TaskAttributionInfo* parent_task,
                             bool has_ua_visual_transition);
  void DispatchWindowLoadEvent();
  void DocumentWasClosed();

  void AcceptLanguagesChanged();

  // https://dom.spec.whatwg.org/#dom-window-event
  ScriptValue event(ScriptState*);
  Event* CurrentEvent() const;
  void SetCurrentEvent(Event*);

  TrustedTypePolicyFactory* trustedTypes(ScriptState*) const;
  TrustedTypePolicyFactory* GetTrustedTypesForWorld(
      const DOMWrapperWorld&) const;

  // Returns true if this window is cross-site to the outermost main frame.
  // Defaults to false in a detached window. Note: This uses an outdated
  // definition of "site" which only includes the registrable domain and not the
  // scheme. IsCrossSiteSubframeIncludingScheme() uses HTML's definition of
  // "site" as a registrable domain and scheme.
  bool IsCrossSiteSubframe() const;

  bool IsCrossSiteSubframeIncludingScheme() const;

  void DispatchPersistedPageshowEvent(base::TimeTicks navigation_start);

  void DispatchPagehideEvent(PageTransitionEventPersistence persistence);

  InputMethodController& GetInputMethodController() const {
    return *input_method_controller_;
  }
  TextSuggestionController& GetTextSuggestionController() const {
    return *text_suggestion_controller_;
  }
  SpellChecker& GetSpellChecker() const { return *spell_checker_; }

  void ClearIsolatedWorldCSPForTesting(int32_t world_id);

  bool CrossOriginIsolatedCapability() const override;
  bool IsIsolatedContext() const override;

  // These delegate to the document_.
  ukm::UkmRecorder* UkmRecorder() override;
  ukm::SourceId UkmSourceID() const override;

  const BlinkStorageKey& GetStorageKey() const { return storage_key_; }
  void SetStorageKey(const BlinkStorageKey& storage_key);

  void DidReceiveUserActivation();

  // Returns the state of the |payment_request_token_| in this document.
  bool IsPaymentRequestTokenActive() const;

  // Consumes the |payment_request_token_| if it was active in this document.
  bool ConsumePaymentRequestToken();

  // Returns the state of the |fullscreen_request_token_| in this document.
  bool IsFullscreenRequestTokenActive() const;

  // Consumes the |fullscreen_request_token_| if it was active in this document.
  bool ConsumeFullscreenRequestToken();

  // Returns the state of the |display_capture_request_token_| in this document.
  bool IsDisplayCaptureRequestTokenActive() const;

  // Consumes the |display_capture_request_token_| if it was active in this
  // document.
  bool ConsumeDisplayCaptureRequestToken();

  // Called when a network request buffered an additional `num_bytes` while this
  // frame is in back-forward cache.
  void DidBufferLoadWhileInBackForwardCache(bool update_process_wide_count,
                                            size_t num_bytes);

  // Whether the window is credentialless or not.
  bool credentialless() const;

  bool IsInFencedFrame() const override;

  Fence* fence();

  CloseWatcher::WatcherStack* closewatcher_stack() {
    return closewatcher_stack_.Get();
  }

  void GenerateNewNavigationId();

  String GetNavigationId() const { return navigation_id_; }

  NavigationApi* navigation();

  // Is this a Document Picture in Picture window?
  bool IsPictureInPictureWindow() const;

  void set_is_picture_in_picture_window_for_testing(
      bool is_picture_in_picture) {
    is_picture_in_picture_window_ = is_picture_in_picture;
  }

  // Sets the StorageAccessApiStatus. Calls to this method must not downgrade
  // the status.
  void SetStorageAccessApiStatus(net::StorageAccessApiStatus status);

  // https://html.spec.whatwg.org/multipage/browsing-the-web.html#has-been-revealed
  bool HasBeenRevealed() const { return has_been_revealed_; }
  void SetHasBeenRevealed(bool revealed);

 protected:
  // EventTarget overrides.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;
  void RemovedEventListener(const AtomicString& event_type,
                            const RegisteredEventListener&) override;

  // Protected DOMWindow overrides.
  void SchedulePostMessage(PostedMessage*) override;

 private:
  class NetworkStateObserver;

  // Intentionally private to prevent redundant checks.
  bool IsLocalDOMWindow() const override { return true; }

  bool HasInsecureContextInAncestors() const override;

  Document& GetDocumentForWindowEventHandler() const override {
    return *document();
  }

  void Dispose();

  void DispatchLoadEvent();

  void SetIsPictureInPictureWindow();

  // Return the viewport size including scrollbars.
  gfx::Size GetViewportSize() const;

  void UpdateEventListenerCountsToDocumentForReuseIfNeeded();

  Member<ScriptController> script_controller_;

  Member<Document> document_;
  Member<DOMViewport> viewport_;
  Member<DOMVisualViewport> visualViewport_;

  bool should_print_when_finished_loading_;

  mutable Member<Screen> screen_;
  mutable Member<History> history_;
  mutable Member<BarProp> locationbar_;
  mutable Member<BarProp> menubar_;
  mutable Member<BarProp> personalbar_;
  mutable Member<BarProp> scrollbars_;
  mutable Member<BarProp> statusbar_;
  mutable Member<BarProp> toolbar_;
  mutable Member<Navigator> navigator_;
  mutable Member<StyleMedia> media_;
  mutable Member<CustomElementRegistry> custom_elements_;
  Member<External> external_;

  Member<NavigationApi> navigation_;

  String status_;
  String default_status_;

  HeapHashSet<WeakMember<EventListenerObserver>> event_listener_observers_;

  // Trackers for delegated payment, fullscreen, and display-capture requests.
  // These are related to |Frame::user_activation_state_|.
  DelegatedCapabilityRequestToken payment_request_token_;
  DelegatedCapabilityRequestToken fullscreen_request_token_;
  DelegatedCapabilityRequestToken display_capture_request_token_;

  // https://dom.spec.whatwg.org/#window-current-event
  // We represent the "undefined" value as nullptr.
  Member<Event> current_event_;

  // Store TrustedTypesPolicyFactory, per DOMWrapperWorld.
  mutable HeapHashMap<Member<const DOMWrapperWorld>,
                      Member<TrustedTypePolicyFactory>>
      trusted_types_map_;

  // A dummy scheduler to return when the window is detached.
  // All operations on it result in no-op, but due to this it's safe to
  // use the returned value of GetScheduler() without additional checks.
  // A task posted to a task runner obtained from one of its task runners
  // will be forwarded to the default task runner.
  // TODO(altimin): We should be able to remove it after we complete
  // frame:document lifetime refactoring.
  std::unique_ptr<FrameOrWorkerScheduler> detached_scheduler_;

  Member<InputMethodController> input_method_controller_;
  Member<SpellChecker> spell_checker_;
  Member<TextSuggestionController> text_suggestion_controller_;

  // Map from isolated world IDs to their ContentSecurityPolicy instances.
  Member<GCedHeapHashMap<int, Member<ContentSecurityPolicy>>>
      isolated_world_csp_map_;

  // Tracks which features have already been potentially violated in this
  // document. This helps to count them only once per page load.
  // We don't use std::bitset to avoid to include
  // permissions_policy.mojom-blink.h.
  mutable Vector<bool> potentially_violated_features_;

  // Token identifying the LocalFrame that this window was associated with at
  // creation. Remains valid even after the frame is destroyed and the context
  // is detached.
  const LocalFrameToken token_;

  // Tracks which document policy violation reports have already been sent in
  // this document, to avoid reporting duplicates. The value stored comes
  // from |DocumentPolicyViolationReport::MatchId()|.
  mutable HashSet<unsigned> document_policy_violation_reports_sent_;

  // The storage key for this LocalDomWindow.
  BlinkStorageKey storage_key_;

  // Fire "online" and "offline" events.
  Member<NetworkStateObserver> network_state_observer_;

  // The total bytes buffered by all network requests in this frame while frozen
  // due to back-forward cache. This number gets reset when the frame gets out
  // of the back-forward cache.
  size_t total_bytes_buffered_while_in_back_forward_cache_ = 0;

  // Collection of fenced frame APIs.
  // https://github.com/shivanigithub/fenced-frame/issues/14
  Member<Fence> fence_;

  Member<CloseWatcher::WatcherStack> closewatcher_stack_;

  // If set, this window is a Document Picture in Picture window.
  // https://wicg.github.io/document-picture-in-picture/
  bool is_picture_in_picture_window_ = false;

  // The navigation id of a document is to identify navigation of special types
  // like bfcache navigation or soft navigation. It changes when navigations
  // of these types occur.
  String navigation_id_;

  // Records this window's Storage Access API status. It cannot be downgraded.
  net::StorageAccessApiStatus storage_access_api_status_ =
      net::StorageAccessApiStatus::kNone;

  // Tracks whether this window has shown a payment request without a user
  // activation. It cannot be revoked once set to true.
  // TODO(crbug.com/1439565): Move this bit to a new payments-specific
  // per-LocalDOMWindow class in the payments module.
  bool had_activationless_payment_request_ = false;

  // https://html.spec.whatwg.org/multipage/browsing-the-web.html#has-been-revealed
  bool has_been_revealed_ = false;

  // Used to indicate if the DOM window is reused or not.
  bool is_dom_window_reused_ = false;
};

template <>
struct DowncastTraits<LocalDOMWindow> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsWindow();
  }
  static bool AllowFrom(const DOMWindow& window) {
    return window.IsLocalDOMWindow();
  }
};

inline String LocalDOMWindow::status() const {
  return status_;
}

inline String LocalDOMWindow::defaultStatus() const {
  DCHECK(RuntimeEnabledFeatures::WindowDefaultStatusEnabled());
  return default_status_;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_DOM_WINDOW_H_
