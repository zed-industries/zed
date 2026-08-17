/*
 * Copyright (C) 2006, 2007, 2008, 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_DOCUMENT_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_DOCUMENT_LOADER_H_

#include <memory>
#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "base/uuid.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/bindings/shared_remote.h"
#include "net/storage_access_api/status.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/public/common/fenced_frame/redacted_fenced_frame_config.h"
#include "third_party/blink/public/common/frame/view_transition_state.h"
#include "third_party/blink/public/common/loader/loading_behavior_flag.h"
#include "third_party/blink/public/common/permissions_policy/document_policy.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/public/common/subresource_load_metrics.h"
#include "third_party/blink/public/mojom/fenced_frame/fenced_frame.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/triggering_event_info.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/content_security_notifier.mojom-blink.h"
#include "third_party/blink/public/mojom/loader/mhtml_load_result.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/same_document_navigation_type.mojom-blink.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-shared.h"
#include "third_party/blink/public/mojom/page/page.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/page_state/page_state.mojom-blink.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/public/mojom/runtime_feature_state/runtime_feature.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/service_worker/controller_service_worker_mode.mojom-blink.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/public/platform/web_navigation_body_loader.h"
#include "third_party/blink/public/web/web_document_loader.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/public/web/web_history_commit_type.h"
#include "third_party/blink/public/web/web_navigation_params.h"
#include "third_party/blink/public/web/web_navigation_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/weak_identifier_map.h"
#include "third_party/blink/renderer/core/frame/dactyloscoper.h"
#include "third_party/blink/renderer/core/frame/frame_types.h"
#include "third_party/blink/renderer/core/frame/policy_container.h"
#include "third_party/blink/renderer/core/frame/use_counter_impl.h"
#include "third_party/blink/renderer/core/html/parser/parser_synchronization_policy.h"
#include "third_party/blink/renderer/core/loader/document_load_timing.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/loader/navigation_policy.h"
#include "third_party/blink/renderer/core/loader/preload_helper.h"
#include "third_party/blink/renderer/core/page/viewport_description.h"
#include "third_party/blink/renderer/core/permissions_policy/policy_helper.h"
#include "third_party/blink/renderer/core/speculation_rules/speculation_rule_set.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/exported/wrapped_resource_response.h"
#include "third_party/blink/renderer/platform/instrumentation/use_counter.h"
#include "third_party/blink/renderer/platform/loader/fetch/client_hints_preferences.h"
#include "third_party/blink/renderer/platform/loader/fetch/code_cache_host.h"
#include "third_party/blink/renderer/platform/loader/fetch/early_hints_preload_entry.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_error.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/storage/blink_storage_key.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace base {
class TickClock;
}

namespace blink {

class BackgroundCodeCacheHost;
class ContentSecurityPolicy;
class CodeCacheHost;
class Document;
class DocumentParser;
class Element;
class Frame;
class FrameLoader;
class HistoryItem;
class LocalDOMWindow;
class LocalFrame;
class LocalFrameClient;
class MHTMLArchive;
class PrefetchedSignedExchangeManager;
class SerializedScriptValue;
class SubresourceFilter;
class WebServiceWorkerNetworkProvider;
struct JavaScriptFrameworkDetectionResult;

namespace mojom {
enum class CommitResult : int32_t;
}  // namespace mojom

enum class FirePopstate { kYes, kNo };

// The DocumentLoader fetches a main resource and handles the result.
// TODO(https://crbug.com/855189). This was originally structured to have a
// provisional load, then commit but that is no longer necessary and this class
// can be simplified.
class CORE_EXPORT DocumentLoader : public GarbageCollected<DocumentLoader>,
                                   public WebDocumentLoader,
                                   public UseCounter,
                                   public WebNavigationBodyLoader::Client {
 public:
  // Limit used for policies optimizing for LCP, e.g. font loading. Based on the
  // "good" LCP value of 2500ms. See `RemainingTimeToLCPLimit()`.
  static constexpr base::TimeDelta kLCPLimit = base::Milliseconds(2000);

  DocumentLoader(LocalFrame*,
                 WebNavigationType navigation_type,
                 std::unique_ptr<WebNavigationParams> navigation_params,
                 std::unique_ptr<PolicyContainer> policy_container,
                 std::unique_ptr<ExtraData> extra_data);
  ~DocumentLoader() override;

  // Returns WebNavigationParams that can be used to clone DocumentLoader. Used
  // for javascript: URL and XSLT commits, where we want to create a new
  // Document but keep most of the property of the current DocumentLoader.
  std::unique_ptr<WebNavigationParams>
  CreateWebNavigationParamsToCloneDocument();

  static bool WillLoadUrlAsEmpty(const KURL&);

  LocalFrame* GetFrame() const { return frame_.Get(); }

  void DetachFromFrame(bool flush_microtask_queue);

  uint64_t MainResourceIdentifier() const;

  const AtomicString& MimeType() const;

  // WebDocumentLoader overrides:
  WebString OriginalReferrer() const override;
  WebURL GetUrl() const override { return Url(); }
  WebString HttpMethod() const override;
  WebString Referrer() const override { return GetReferrer(); }
  bool HasUnreachableURL() const override {
    return !UnreachableURL().IsEmpty();
  }
  WebURL UnreachableWebURL() const override { return UnreachableURL(); }
  const WebURLResponse& GetWebResponse() const override {
    return response_wrapper_;
  }
  bool IsClientRedirect() const override { return is_client_redirect_; }
  bool ReplacesCurrentHistoryItem() const override {
    return replaces_current_history_item_;
  }
  WebNavigationType GetNavigationType() const override {
    return navigation_type_;
  }
  ExtraData* GetExtraData() const override;
  std::unique_ptr<ExtraData> CloneExtraData() override;
  void SetExtraData(std::unique_ptr<ExtraData>) override;
  void SetSubresourceFilter(WebDocumentSubresourceFilter*) override;
  void SetServiceWorkerNetworkProvider(
      std::unique_ptr<WebServiceWorkerNetworkProvider>) override;
  // May return null before the first HTML tag is inserted by the
  // parser (before didCreateDataSource is called), after the document
  // is detached from frame, or in tests.
  WebServiceWorkerNetworkProvider* GetServiceWorkerNetworkProvider() override {
    return service_worker_network_provider_.get();
  }
  // Can be used to temporarily suspend feeding the parser with new data. The
  // parser will be allowed to read new data when ResumeParser() is called the
  // same number of time than BlockParser().
  void BlockParser() override;
  void ResumeParser() override;
  bool HasBeenLoadedAsWebArchive() const override {
    return archive_ != nullptr;
  }
  WebArchiveInfo GetArchiveInfo() const override;
  bool LastNavigationHadTransientUserActivation() const override {
    return last_navigation_had_transient_user_activation_;
  }
  void SetCodeCacheHost(
      CrossVariantMojoRemote<mojom::blink::CodeCacheHostInterfaceBase>
          code_cache_host,
      CrossVariantMojoRemote<mojom::blink::CodeCacheHostInterfaceBase>
          code_cache_host_for_background) override;
  WebString OriginCalculationDebugInfo() const override {
    return origin_calculation_debug_info_;
  }
  bool HasLoadedNonInitialEmptyDocument() const override;
  bool IsForDiscard() const override;

  MHTMLArchive* Archive() const { return archive_.Get(); }

  SubresourceFilter* GetSubresourceFilter() const {
    return subresource_filter_.Get();
  }

  // TODO(dcheng, japhet): Some day, Document::Url() will always match
  // DocumentLoader::Url(), and one of them will be removed. Today is not that
  // day though.
  const KURL& Url() const;

  const KURL& UrlForHistory() const;
  const AtomicString& GetReferrer() const;
  const SecurityOrigin* GetRequestorOrigin() const;
  const KURL& UnreachableURL() const;
  const std::optional<blink::mojom::FetchCacheMode>& ForceFetchCacheMode()
      const;

  void DidChangePerformanceTiming();
  void DidObserveLoadingBehavior(LoadingBehaviorFlag);
  void DidObserveJavaScriptFrameworks(
      const JavaScriptFrameworkDetectionResult&);

  // https://html.spec.whatwg.org/multipage/history.html#url-and-history-update-steps
  void RunURLAndHistoryUpdateSteps(const KURL&,
                                   HistoryItem*,
                                   mojom::blink::SameDocumentNavigationType,
                                   scoped_refptr<SerializedScriptValue>,
                                   WebFrameLoadType,
                                   FirePopstate,
                                   bool should_skip_screenshot,
                                   bool is_browser_initiated = false,
                                   bool is_synchronously_committed = true,
                                   std::optional<scheduler::TaskAttributionId>
                                       soft_navigation_heuristics_task_id =
                                           std::nullopt);

  // |is_synchronously_committed| is described in comment for
  // CommitSameDocumentNavigation.
  void UpdateForSameDocumentNavigation(
      const KURL&,
      HistoryItem*,
      mojom::blink::SameDocumentNavigationType,
      scoped_refptr<SerializedScriptValue>,
      WebFrameLoadType,
      FirePopstate,
      const SecurityOrigin* initiator_origin,
      bool is_browser_initiated,
      bool is_synchronously_committed,
      std::optional<scheduler::TaskAttributionId>
          soft_navigation_heuristics_task_id,
      bool has_transient_user_activation,
      bool has_ua_visual_transition,
      bool should_skip_screenshot);

  const ResourceResponse& GetResponse() const { return response_; }

  bool IsCommittedButEmpty() const {
    return state_ >= kCommitted && !data_received_;
  }

  void SetSentDidFinishLoad() { state_ = kSentDidFinishLoad; }
  bool SentDidFinishLoad() const { return state_ == kSentDidFinishLoad; }

  WebFrameLoadType LoadType() const { return load_type_; }
  void SetLoadType(WebFrameLoadType load_type) { load_type_ = load_type; }

  void SetNavigationType(WebNavigationType navigation_type) {
    navigation_type_ = navigation_type;
  }

  HistoryItem* GetHistoryItem() const { return history_item_.Get(); }

  void StartLoading();
  void StopLoading();

  // CommitNavigation() does the work of creating a Document and
  // DocumentParser, as well as creating a new LocalDOMWindow if needed. It also
  // initializes a bunch of state on the Document (e.g., the state based on
  // response headers).
  void CommitNavigation();

  // Called when the browser process has asked this renderer process to commit a
  // same document navigation in that frame. Returns false if the navigation
  // cannot commit, true otherwise.
  // |initiator_origin| is the origin of the document or script that initiated
  // the navigation or nullptr if the navigation is initiated via browser UI
  // (e.g. typed in omnibox), or a history traversal to a previous navigation
  // via browser UI.
  // |is_synchronously_committed| is true if the navigation is synchronously
  // committed from within Blink, rather than being driven by the browser's
  // navigation stack.
  mojom::CommitResult CommitSameDocumentNavigation(
      const KURL&,
      WebFrameLoadType,
      HistoryItem*,
      ClientRedirectPolicy,
      bool has_transient_user_activation,
      const SecurityOrigin* initiator_origin,
      bool is_synchronously_committed,
      Element* source_element,
      mojom::blink::TriggeringEventInfo,
      bool is_browser_initiated,
      bool has_ua_visual_transition,
      std::optional<scheduler::TaskAttributionId>
          soft_navigation_heuristics_task_id,
      bool should_skip_screenshot);

  void SetDefersLoading(LoaderFreezeMode);

  DocumentLoadTiming& GetTiming() { return document_load_timing_; }

  struct InitialScrollState {
    DISALLOW_NEW();
    InitialScrollState()
        : was_scrolled_by_user(false), did_restore_from_history(false) {}

    bool was_scrolled_by_user;
    bool did_restore_from_history;
  };
  InitialScrollState& GetInitialScrollState() { return initial_scroll_state_; }

  enum State { kNotStarted, kProvisional, kCommitted, kSentDidFinishLoad };

  void DispatchLinkHeaderPreloads(const ViewportDescription*,
                                  PreloadHelper::LoadLinksFromHeaderMode);
  void DispatchLcppFontPreloads(const ViewportDescription*,
                                PreloadHelper::LoadLinksFromHeaderMode);

  void LoadFailed(const ResourceError&);

  void Trace(Visitor*) const override;

  // For automation driver-initiated navigations over the devtools protocol,
  // |devtools_navigation_token_| is used to tag the navigation. This navigation
  // token is then sent into the renderer and lands on the DocumentLoader. That
  // way subsequent Blink-level frame lifecycle events can be associated with
  // the concrete navigation.
  // - The value should not be sent back to the browser.
  // - The value on DocumentLoader may be generated in the renderer in some
  // cases, and thus shouldn't be trusted.
  // TODO(crbug.com/783506): Replace devtools navigation token with the generic
  // navigation token that can be passed from renderer to the browser.
  const base::UnguessableToken& GetDevToolsNavigationToken() {
    return devtools_navigation_token_;
  }

  UseCounterImpl& GetUseCounter() { return use_counter_; }

  PrefetchedSignedExchangeManager* GetPrefetchedSignedExchangeManager() const;

  // UseCounter
  void CountUse(mojom::WebFeature) override;
  void CountDeprecation(mojom::WebFeature) override;
  void CountWebDXFeature(mojom::blink::WebDXFeature) override;

  void SetCommitReason(CommitReason reason) { commit_reason_ = reason; }

  // Whether the navigation originated from the browser process. Note: history
  // navigation is always considered to be browser initiated, even if the
  // navigation was started using the history API in the renderer.
  bool IsBrowserInitiated() const { return is_browser_initiated_; }

  bool LastNavigationHadTrustedInitiator() const {
    return last_navigation_had_trusted_initiator_;
  }

  // Called when the URL needs to be updated due to a document.open() call.
  void DidOpenDocumentInputStream(const KURL& url);

  enum class HistoryNavigationType {
    kDifferentDocument,
    kFragment,
    kHistoryApi
  };

  void SetHistoryItemStateForCommit(HistoryItem* old_item,
                                    WebFrameLoadType,
                                    HistoryNavigationType,
                                    CommitReason commit_reason);

  bool NavigationScrollAllowed() const { return navigation_scroll_allowed_; }

  // We want to make sure that the largest content is painted before the "LCP
  // limit", so that we get a good LCP value. This returns the remaining time to
  // the LCP limit. See crbug.com/1065508 for details.
  base::TimeDelta RemainingTimeToLCPLimit() const;

  // We are experimenting the idea of making preloaded fonts render-blocking up
  // to a certain amount of time after navigation starts. This returns the
  // remaining time to that time limit. See crbug.com/1412861 for details.
  base::TimeDelta RemainingTimeToRenderBlockingFontMaxBlockingTime() const;

  mojom::blink::ContentSecurityNotifier& GetContentSecurityNotifier();

  // Returns the value of the text fragment token and then resets it to false
  // to ensure the token can only be used to invoke a single text fragment.
  bool ConsumeTextFragmentToken();

  // Notifies that the prerendering document this loader is working for is
  // activated.
  void NotifyPrerenderingDocumentActivated(
      const mojom::blink::PrerenderPageActivationParams& params);

  CodeCacheHost* GetCodeCacheHost();
  scoped_refptr<BackgroundCodeCacheHost> CreateBackgroundCodeCacheHost();
  static void DisableCodeCacheForTesting();

  mojo::PendingRemote<mojom::blink::CodeCacheHost> CreateWorkerCodeCacheHost();

  HashMap<KURL, EarlyHintsPreloadEntry> GetEarlyHintsPreloadedResources();

  const std::optional<Vector<KURL>>& AdAuctionComponents() const {
    return ad_auction_components_;
  }

  const std::optional<FencedFrame::RedactedFencedFrameProperties>&
  FencedFrameProperties() const {
    return fenced_frame_properties_;
  }

  // Detect if the page is reloaded or after form submitted. This method is
  // called in order to disable some interventions or optimizations based on the
  // heuristic that the user might reload the page when interventions cause
  // problems. Also, the user is likely to avoid reloading the page when they
  // submit forms. So this method is useful to skip interventions in the
  // following conditions.
  // - Reload a page.
  // - Submit a form.
  // - Resubmit a form.
  // The reason why we use DocumentLoader::GetNavigationType() instead of
  // DocumentLoader::LoadType() is that DocumentLoader::LoadType() is reset to
  // WebFrameLoadType::kStandard on DidFinishNavigation(). When JavaScript adds
  // iframes after navigation, DocumentLoader::LoadType() always returns
  // WebFrameLoadType::kStandard. DocumentLoader::GetNavigationType() doesn't
  // have this problem.
  bool IsReloadedOrFormSubmitted() const;

  // (crbug.com/1371756) Record the page if the main resource is not fetched via
  // ServiceWorker and at least one subresource is fetched via ServiceWorker.
  // This method won't record the page if the main resource was not controlled
  // by ServiceWorker at the time of the initial navigation. This helps us to
  // understand the potential impact of the fetch fast-path effort.
  void MaybeRecordServiceWorkerFallbackMainResource(
      bool was_subresource_fetched_via_service_worker);

  // Starts loading the navigation body in a background thread.
  static void MaybeStartLoadingBodyInBackground(
      WebNavigationBodyLoader* body_loader,
      LocalFrame* frame,
      const KURL& url,
      const ResourceResponse& response);

  // This needs to be kept as public to be accessible from
  // SameSizeAsDocumentLoader as GCC will fail to allow access
  // even if it is friend of DocumentLoader
  class DecodedBodyData;

  network::mojom::NavigationDeliveryType GetNavigationDeliveryType() const {
    return navigation_delivery_type_;
  }

  void UpdateSubresourceLoadMetrics(
      const SubresourceLoadMetrics& subresource_load_metrics);

  const AtomicString& GetCookieDeprecationLabel() const {
    return cookie_deprecation_label_;
  }

  // Gets the content settings for the current {frame, navigation commit} tuple.
  const mojom::RendererContentSettingsPtr& GetContentSettings();

 protected:
  // Based on its MIME type, if the main document's response corresponds to an
  // MHTML archive, then every resources will be loaded from this archive.
  //
  // This includes:
  // - The main document.
  // - Every nested document.
  // - Every subresource.
  //
  // This excludes:
  // - data-URLs documents and subresources.
  // - about:srcdoc documents.
  // - Error pages.
  //
  // Whether about:blank and derivative should be loaded from the archive is
  // weird edge case: Please refer to the tests:
  // - NavigationMhtmlBrowserTest.IframeAboutBlankNotFound
  // - NavigationMhtmlBrowserTest.IframeAboutBlankFound
  //
  // Nested documents are loaded in the same process and grab a reference to the
  // same `archive_` as their parent.
  //
  // Resources:
  // - https://tools.ietf.org/html/rfc822
  // - https://tools.ietf.org/html/rfc2387
  Member<MHTMLArchive> archive_;

 private:
  class BodyData;
  class EncodedBodyData;

  Frame* CalculateOwnerFrame();
  scoped_refptr<SecurityOrigin> CalculateOrigin(Document* owner_document);
  void InitializeWindow(Document* owner_document);
  void DidInstallNewDocument(Document*);
  void WillCommitNavigation();
  void DidCommitNavigation();
  void RecordUseCountersForCommit();
  void RecordConsoleMessagesForCommit();

  // Use to record UMA metrics on the matches between the Content-Language
  // response header value and the Accept-Language request header values.
  void RecordAcceptLanguageAndContentLanguageMetric();

  // Use to record UMA metrics on the matches between the parent frame's
  // Content-Language request header value and child frame's Content-Language
  // request header values.
  void RecordParentAndChildContentLanguageMetric();

  void CreateParserPostCommit();

  void CommitSameDocumentNavigationInternal(
      const KURL&,
      WebFrameLoadType,
      HistoryItem*,
      mojom::blink::SameDocumentNavigationType,
      ClientRedirectPolicy,
      bool has_transient_user_activation,
      const SecurityOrigin* initiator_origin,
      bool is_browser_initiated,
      bool is_synchronously_committed,
      mojom::blink::TriggeringEventInfo,
      std::optional<scheduler::TaskAttributionId>
          soft_navigation_heuristics_task_id,
      bool has_ua_visual_transition,
      bool should_skip_screenshot);

  // Use these method only where it's guaranteed that |m_frame| hasn't been
  // cleared.
  FrameLoader& GetFrameLoader() const;
  LocalFrameClient& GetLocalFrameClient() const;

  void ConsoleError(const String& message);

  // Replace the current document with a empty one and the URL with a unique
  // opaque origin.
  void ReplaceWithEmptyDocument();

  DocumentPolicy::ParsedDocumentPolicy CreateDocumentPolicy();

  void StartLoadingInternal();
  void StartLoadingResponse();
  void FinishedLoading(base::TimeTicks finish_time);

  // Process a redirect to update the redirect chain, current URL, referrer,
  // etc.
  void HandleRedirect(const WebNavigationParams::RedirectInfo& redirect);
  void HandleResponse();

  void InitializeEmptyResponse();

  void CommitData(BodyData& data);
  // Processes the data stored in |data_buffer_| or |decoded_data_buffer_|, used
  // to avoid appending data to the parser in a nested message loop.
  void ProcessDataBuffer(BodyData* data = nullptr);
  void BodyDataReceivedImpl(BodyData& data);

  // WebNavigationBodyLoader::Client
  void BodyDataReceived(base::span<const char> data) override;
  void DecodedBodyDataReceived(
      const WebString& data,
      const WebEncodingData& encoding_data,
      base::SpanOrSize<const char> encoded_data) override;
  void BodyLoadingFinished(base::TimeTicks completion_time,
                           int64_t total_encoded_data_length,
                           int64_t total_encoded_body_length,
                           int64_t total_decoded_body_length,
                           const std::optional<WebURLError>& error) override;
  ProcessBackgroundDataCallback TakeProcessBackgroundDataCallback() override;

  void ApplyClientHintsConfig(
      const std::vector<network::mojom::WebClientHintsType>&
          enabled_client_hints);

  // If the page was loaded from a signed exchange which has "allowed-alt-sxg"
  // link headers in the inner response and PrefetchedSignedExchanges were
  // passed from the previous page, initializes a
  // PrefetchedSignedExchangeManager which will hold the subresource signed
  // exchange related headers ("alternate" link header in the outer response and
  // "allowed-alt-sxg" link header in the inner response of the page's signed
  // exchange), and the passed PrefetchedSignedExchanges. The created
  // PrefetchedSignedExchangeManager will be used to load the prefetched signed
  // exchanges for matching requests.
  void InitializePrefetchedSignedExchangeManager();

  bool IsJavaScriptURLOrXSLTCommitOrDiscard() const {
    return commit_reason_ == CommitReason::kJavascriptUrl ||
           commit_reason_ == CommitReason::kXSLT ||
           commit_reason_ == CommitReason::kDiscard;
  }

  // Computes and creates CSP for this document.
  ContentSecurityPolicy* CreateCSP();

  bool IsSameOriginInitiator() const;

  // This initiates a view transition if the `view_transition_state_` has been
  // specified.
  void StartViewTransitionIfNeeded(Document& document);

  // Injects speculation rules automatically for some pages based on their
  // contents (currently only detected JavaScript frameworks). Configured by the
  // AutoSpeculationRules feature.
  void InjectAutoSpeculationRules(const JavaScriptFrameworkDetectionResult&);
  void InjectSpeculationRulesFromString(const String&,
                                        BrowserInjectedSpeculationRuleOptOut);

  // Params are saved in constructor and are cleared after StartLoading().
  // TODO(dgozman): remove once StartLoading is merged with constructor.
  std::unique_ptr<WebNavigationParams> params_;

  // The policy container to be moved into the window at initialization time. We
  // need this and cannot use params_->policy_container because the latter has
  // type WebPolicyContainer, and we want to avoid a back-and-forth type
  // conversion.
  std::unique_ptr<PolicyContainer> policy_container_;

  // The permissions policy to be applied to the window at initialization time.
  const std::optional<network::ParsedPermissionsPolicy>
      initial_permissions_policy_;

  // These fields are copied from WebNavigationParams, see there for definition.
  DocumentToken token_;
  KURL url_;
  KURL original_url_;
  AtomicString http_method_;
  // The referrer on the final request for this document.
  AtomicString referrer_;
  scoped_refptr<EncodedFormData> http_body_;
  AtomicString http_content_type_;
  const scoped_refptr<const SecurityOrigin> requestor_origin_;
  const KURL unreachable_url_;
  const KURL pre_redirect_url_for_failed_navigations_;
  std::unique_ptr<WebNavigationBodyLoader> body_loader_;
  const bool grant_load_local_resources_ = false;
  const std::optional<blink::mojom::FetchCacheMode> force_fetch_cache_mode_;
  const FramePolicy frame_policy_;
  std::optional<uint64_t> visited_link_salt_;

  Member<LocalFrame> frame_;

  Member<HistoryItem> history_item_;

  // The parser that was created when the current Document was installed.
  // document.open() may create a new parser at a later point, but this
  // will not be updated.
  Member<DocumentParser> parser_;

  Member<SubresourceFilter> subresource_filter_;

  const AtomicString original_referrer_;

  ResourceResponse response_;
  // Mutable because the const getters will magically sync these to the
  // latest version of |response_|.
  mutable WrappedResourceResponse response_wrapper_;

  WebFrameLoadType load_type_;

  bool is_client_redirect_;
  bool replaces_current_history_item_;
  bool data_received_;
  const bool is_error_page_for_failed_navigation_;

  HeapMojoRemote<mojom::blink::ContentSecurityNotifier>
      content_security_notifier_;

  const scoped_refptr<SecurityOrigin> origin_to_commit_;

  // Information about how `origin_to_commit_` was calculated, to help debug if
  // it differs from the origin calculated on the browser side.
  // TODO(https://crbug.com/1220238): Remove this.
  AtomicString origin_calculation_debug_info_;

  blink::BlinkStorageKey storage_key_;

  WebNavigationType navigation_type_;

  DocumentLoadTiming document_load_timing_;

  base::TimeTicks time_of_last_data_received_;

  mojom::blink::ControllerServiceWorkerMode
      service_worker_initial_controller_mode_ =
          mojom::blink::ControllerServiceWorkerMode::kNoController;

  std::unique_ptr<WebServiceWorkerNetworkProvider>
      service_worker_network_provider_;

  DocumentPolicy::ParsedDocumentPolicy document_policy_;
  bool was_blocked_by_document_policy_;
  Vector<PolicyParserMessageBuffer::Message> document_policy_parsing_messages_;

  ClientHintsPreferences client_hints_preferences_;
  InitialScrollState initial_scroll_state_;

  State state_;

  // Used to block the parser.
  int parser_blocked_count_ = 0;
  bool finish_loading_when_parser_resumed_ = false;

  // Used to protect against reentrancy into CommitData().
  bool in_commit_data_;

  // Either |data_buffer_| or |decoded_data_buffer_| will be used depending on
  // whether BodyDataReceived() or DecodedBodyDataReceived() is called.
  scoped_refptr<SharedBuffer> data_buffer_;
  Vector<DecodedBodyData> decoded_data_buffer_;

  const base::UnguessableToken devtools_navigation_token_;

  const base::Uuid base_auction_nonce_;

  LoaderFreezeMode freeze_mode_ = LoaderFreezeMode::kNone;

  // Whether the last navigation (cross-document or same-document) that
  // committed in this DocumentLoader had transient activation.
  bool last_navigation_had_transient_user_activation_ = false;

  // Whether the last navigation (cross-document or same-document) that
  // committed in this DocumentLoader was initiated from the same-origin as the
  // current document or was browser-initiated.
  bool last_navigation_had_trusted_initiator_ = false;

  // Whether this load request comes with a sticky user activation. For
  // prerendered pages, this is initially false but could be updated on
  // prerender page activation.
  bool had_sticky_activation_ = false;

  // Whether this load request was initiated by the browser.
  const bool is_browser_initiated_ = false;

  // Whether this loader committed a document in a prerendered page that has not
  // yet been activated. This is only set after commit.
  bool is_prerendering_ = false;

  // If true, the navigation loading this document should allow a text fragment
  // to invoke. This token may be instead consumed to pass this permission
  // through a redirect.
  bool has_text_fragment_token_ = false;

  // See WebNavigationParams for definition.
  const bool was_discarded_ = false;

  // True when loading the main document from the MHTML archive. It implies an
  // |archive_| to be created. Nested documents will also inherit from the same
  // |archive_|, but won't have |loading_main_document_from_mhtml_archive_| set.
  bool loading_main_document_from_mhtml_archive_ = false;
  const bool loading_srcdoc_ = false;
  const KURL fallback_base_url_;
  const bool loading_url_as_empty_document_ = false;
  const bool is_static_data_ = false;
  CommitReason commit_reason_ = CommitReason::kRegular;
  uint64_t main_resource_identifier_ = 0;
  mojom::blink::ResourceTimingInfoPtr resource_timing_info_for_parent_;
  WebScopedVirtualTimePauser virtual_time_pauser_;
  Member<PrefetchedSignedExchangeManager> prefetched_signed_exchange_manager_;
  ukm::SourceId ukm_source_id_;

  // This UseCounter tracks feature usage associated with the lifetime of
  // the document load. Features recorded prior to commit will be recorded
  // locally. Once committed, feature usage will be piped to the browser side
  // page load metrics that aggregates usage from frames to one page load and
  // report feature usage to UMA histograms per page load.
  UseCounterImpl use_counter_;

  const base::TickClock* clock_;

  const Vector<mojom::blink::OriginTrialFeature>
      initiator_origin_trial_features_;

  const Vector<String> force_enabled_origin_trials_;

  // Whether the document can be scrolled on load
  bool navigation_scroll_allowed_ = true;

  bool origin_agent_cluster_ = false;
  bool origin_agent_cluster_left_as_default_ = true;

  // Whether this load request is from a cross-site navigation that swaps
  // BrowsingContextGroup.
  bool is_cross_site_cross_browsing_context_group_ = false;

  // Whether the new document should start with sticky user activation, because
  // the previously committed document did, and the navigation was same-site.
  bool should_have_sticky_user_activation_ = false;

  std::vector<WebHistoryItem> navigation_api_back_entries_
      ALLOW_DISCOURAGED_TYPE("Matches WebNavigationParams");
  std::vector<WebHistoryItem> navigation_api_forward_entries_
      ALLOW_DISCOURAGED_TYPE("Matches WebNavigationParams");
  Member<HistoryItem> navigation_api_previous_entry_;

  // This is the interface that handles generated code cache
  // requests to fetch code cache when loading resources.
  std::unique_ptr<CodeCacheHost> code_cache_host_;
  mojo::PendingRemote<mojom::blink::CodeCacheHost>
      pending_code_cache_host_for_background_;

  HashMap<KURL, EarlyHintsPreloadEntry> early_hints_preloaded_resources_;

  // If this is a navigation to fenced frame from an interest group auction,
  // contains URNs to the ad components returned by the winning bid. Null,
  // otherwise.
  std::optional<Vector<KURL>> ad_auction_components_;

  std::unique_ptr<ExtraData> extra_data_;

  // Reduced accept language for top-level frame.
  const AtomicString reduced_accept_language_;

  const network::mojom::NavigationDeliveryType navigation_delivery_type_;

  // Provides state from the previous Document that will be replaced by this
  // navigation for a ViewTransition.
  std::optional<ViewTransitionState> view_transition_state_;

  std::optional<FencedFrame::RedactedFencedFrameProperties>
      fenced_frame_properties_;

  // The StorageAccessApiStatus that the document should be loaded with.
  const net::StorageAccessApiStatus storage_access_api_status_;

  // Only container-initiated navigations (e.g. iframe change src) report
  // their resource timing to the parent.
  mojom::blink::ParentResourceTimingAccess parent_resource_timing_access_;

  // Indicates which browsing context group this frame belongs to. It is only
  // set for a main frame committing in another browsing context group.
  const std::optional<BrowsingContextGroupInfo> browsing_context_group_info_;

  // Runtime feature state override is applied to the document. They are applied
  // before JavaScript context creation (i.e. CreateParserPostCommit).
  const base::flat_map<mojom::blink::RuntimeFeature, bool>
      modified_runtime_features_;

  // The cookie deprecation label for cookie deprecation facilitated testing.
  // Will be used in
  // //third_party/blink/renderer/modules/cookie_deprecation_label.
  const AtomicString cookie_deprecation_label_;

  // Renderer-enforced content settings are stored on a per-document basis.
  mojom::RendererContentSettingsPtr content_settings_;

  // When document is fetched from service worker, we keep track of the body
  // size for reporting in Navigation Timing encodedBodySize/decodedBodySize.
  int64_t total_body_size_from_service_worker_ = 0;

  // Map of permission statuses, snapshotting and propagated from browser before
  // committing a navigation.
  // Note: the permission statues will be only used as initial states of
  // `CachedPermissionStatus`
  std::optional<
      HashMap<mojom::blink::PermissionName, mojom::blink::PermissionStatus>>
      initial_permission_statuses_;

  // When this is set to true, the navigation must create a new document
  // sequence number to avoid appearing as a same-document navigation, even if
  // the URL seems like a match. This matters for cross-origin navigations
  // (apart from error pages with the same precursor origin).
  bool force_new_document_sequence_number_ = false;
};

DECLARE_WEAK_IDENTIFIER_MAP(DocumentLoader);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_DOCUMENT_LOADER_H_
