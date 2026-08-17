/*
 * Copyright (C) 2011, 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_CLIENT_H_

#include <memory>
#include <optional>
#include <utility>

#include "base/functional/callback_helpers.h"
#include "base/functional/function_ref.h"
#include "base/i18n/rtl.h"
#include "base/notreached.h"
#include "base/unguessable_token.h"
#include "media/base/audio_processing.h"
#include "media/base/output_device_info.h"
#include "media/base/speech_recognition_client.h"
#include "media/mojo/mojom/audio_processing.mojom-shared.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-shared.h"
#include "third_party/blink/public/common/dom_storage/session_storage_namespace_id.h"
#include "third_party/blink/public/common/frame/frame_owner_element_type.h"
#include "third_party/blink/public/common/loader/loading_behavior_flag.h"
#include "third_party/blink/public/common/loader/url_loader_factory_bundle.h"
#include "third_party/blink/public/common/performance/performance_timeline_constants.h"
#include "third_party/blink/public/common/subresource_load_metrics.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/common/use_counter/use_counter_feature.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-forward.h"
#include "third_party/blink/public/mojom/fenced_frame/fenced_frame.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/blocked_navigation_types.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/triggering_event_info.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/user_activation_update_types.mojom-shared.h"
#include "third_party/blink/public/mojom/loader/same_document_navigation_type.mojom-shared.h"
#include "third_party/blink/public/mojom/media/renderer_audio_input_stream_factory.mojom-shared.h"
#include "third_party/blink/public/platform/child_url_loader_factory_bundle.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_provider.h"
#include "third_party/blink/public/platform/resource_load_info_notifier_wrapper.h"
#include "third_party/blink/public/platform/url_loader_throttle_provider.h"
#include "third_party/blink/public/platform/web_background_resource_fetch_assets.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_security_policy_struct.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_effective_connection_type.h"
#include "third_party/blink/public/platform/web_file_system_type.h"
#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/public/platform/web_prescient_networking.h"
#include "third_party/blink/public/platform/web_set_sink_id_callbacks.h"
#include "third_party/blink/public/platform/web_source_location.h"
#include "third_party/blink/public/platform/web_url_error.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/public/platform/web_worker_fetch_context.h"
#include "third_party/blink/public/web/web_ax_object.h"
#include "third_party/blink/public/web/web_document_loader.h"
#include "third_party/blink/public/web/web_frame.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/public/web/web_frame_owner_properties.h"
#include "third_party/blink/public/web/web_history_commit_type.h"
#include "third_party/blink/public/web/web_history_item.h"
#include "third_party/blink/public/web/web_meaningful_layout.h"
#include "third_party/blink/public/web/web_media_inspector.h"
#include "third_party/blink/public/web/web_navigation_params.h"
#include "third_party/blink/public/web/web_navigation_policy.h"
#include "third_party/blink/public/web/web_navigation_type.h"
#include "ui/accessibility/ax_enums.mojom-shared.h"
#include "ui/accessibility/ax_event.h"
#include "ui/accessibility/ax_location_and_scroll_updates.h"
#include "ui/accessibility/ax_tree_update.h"
#include "ui/events/types/scroll_types.h"
#include "v8/include/v8.h"

namespace cc {
class LayerTreeSettings;
}  // namespace cc

namespace gfx {
class Rect;
}  // namespace gfx

namespace net {
class SiteForCookies;
}  // namespace net

namespace network {
class SharedURLLoaderFactory;
}  // namespace network

namespace blink {
namespace mojom {
enum class TreeScopeType;
}  // namespace mojom

class AssociatedInterfaceProvider;
class WebBackgroundResourceFetchAssets;
class WebContentDecryptionModule;
class WebDedicatedWorkerHostFactoryClient;
class WebDocumentLoader;
class WebEncryptedMediaClient;
class WebLocalFrame;
class WebMediaPlayer;
class WebMediaPlayerClient;
class WebMediaPlayerEncryptedMediaClient;
class WebMediaPlayerSource;
class WebMediaStreamDeviceObserver;
class WebNavigationControl;
class WebPlugin;
class WebPrescientNetworking;
class WebRelatedAppsFetcher;
class WebServiceWorkerProvider;
class WebSocketHandshakeThrottle;
class WebString;
class WebURL;
class URLLoader;
class WebURLRequest;
class WebURLResponse;
class WebView;
class WebLinkPreviewTriggerer;
struct FramePolicy;
struct Impression;
struct JavaScriptFrameworkDetectionResult;
struct WebConsoleMessage;
struct ContextMenuData;
struct WebPictureInPictureWindowOptions;
struct WebPluginParams;
struct WebWindowFeatures;

enum class SyncCondition {
  kNotForced,  // Sync only if the value has changed since the last call.
  kForced,     // Force a sync even if the value is unchanged.
};

// The reason a WebLocalFrame is being detached. See
// `WebLocalFrameClient::WillDetach()` for more details.
enum class DetachReason {
  // The WebLocalFrame is detached because the browsing context that contains it
  // is getting deleted (e.g. <iframe> element getting detached)
  kFrameDeletion,
  // The WebLocalFrame is detached because of a navigation, which will create a
  // new WebLocalFrame (possibly in a different renderer process) that will
  // replace the current one and takes its place in the same browsing context.
  kNavigation,
};

class BLINK_EXPORT WebLocalFrameClient {
 public:
  virtual ~WebLocalFrameClient() = default;

  // Initialization ------------------------------------------------------
  // Called exactly once during construction to notify the client about the
  // created WebLocalFrame. Guaranteed to be invoked before any other
  // WebLocalFrameClient callbacks. Note this takes WebNavigationControl
  // to give the client full control over frame's navigation.
  virtual void BindToFrame(WebNavigationControl*) {}

  // Factory methods -----------------------------------------------------

  // May return null.
  virtual WebPlugin* CreatePlugin(const WebPluginParams&) { return nullptr; }

  // May return null.
  // WebContentDecryptionModule* may be null if one has not yet been set.
  virtual std::unique_ptr<WebMediaPlayer> CreateMediaPlayer(
      const WebMediaPlayerSource&,
      WebMediaPlayerClient*,
      blink::MediaInspectorContext*,
      WebMediaPlayerEncryptedMediaClient*,
      WebContentDecryptionModule*,
      const WebString& sink_id,
      const cc::LayerTreeSettings* settings,
      scoped_refptr<base::TaskRunner> compositor_worker_task_runner) {
    return nullptr;
  }

  // May return null.
  virtual std::unique_ptr<WebServiceWorkerProvider>
  CreateServiceWorkerProvider() {
    return nullptr;
  }

  // May return null.
  virtual std::unique_ptr<WebContentSettingsClient>
  CreateWorkerContentSettingsClient() {
    return nullptr;
  }

  // May return null if speech recognition is not supported.
  virtual std::unique_ptr<media::SpeechRecognitionClient>
  CreateSpeechRecognitionClient() {
    return nullptr;
  }

  // Returns a new WebWorkerFetchContext for worklet.
  virtual scoped_refptr<WebWorkerFetchContext> CreateWorkletFetchContext() {
    return nullptr;
  }

  // Returns a new WebWorkerFetchContext for dedicated workers.
  virtual scoped_refptr<WebWorkerFetchContext> CreateWorkerFetchContext(
      WebDedicatedWorkerHostFactoryClient*) {
    return nullptr;
  }

  // May return null.
  virtual std::unique_ptr<WebPrescientNetworking> CreatePrescientNetworking() {
    return nullptr;
  }

  // Create a notifier used to notify loading stats for this frame.
  virtual std::unique_ptr<blink::ResourceLoadInfoNotifierWrapper>
  CreateResourceLoadInfoNotifierWrapper() {
    return std::make_unique<blink::ResourceLoadInfoNotifierWrapper>(
        /*resource_load_info_notifier=*/nullptr);
  }

  // Services ------------------------------------------------------------

  // Returns an AssociatedInterfaceProvider the frame can use to request
  // navigation-associated interfaces from the browser. See also
  // LocalFrame::GetRemoteNavigationAssociatedInterfaces().
  virtual AssociatedInterfaceProvider*
  GetRemoteNavigationAssociatedInterfaces();

  // General notifications -----------------------------------------------

  // Request the creation of a new child frame. Embedders may return nullptr
  // to prevent the new child frame from being attached. Otherwise, embedders
  // should create a new WebLocalFrame, insert it into the frame tree, call
  // `complete_creation()`, and return the created frame.
  //
  // `complete_creation` takes the newly-created `WebLocalFrame` as well as the
  // `DocumentToken` and `BrowserInterfaceBroker` to use for the initial empty
  // document.
  //
  // `document_ukm_source_id` is the UKM source id to be used for the new
  // document in the frame. If `ukm::kInvalidSourceId` is passed, a new UKM
  // source id will be generated.
  using FinishChildFrameCreationFn = base::FunctionRef<void(
      WebLocalFrame*,
      const DocumentToken&,
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>)>;
  virtual WebLocalFrame* CreateChildFrame(
      mojom::TreeScopeType,
      const WebString& name,
      const WebString& fallback_name,
      const FramePolicy&,
      const WebFrameOwnerProperties&,
      FrameOwnerElementType,
      WebPolicyContainerBindParams policy_container_bind_params,
      ukm::SourceId document_ukm_source_id,
      FinishChildFrameCreationFn complete_creation) {
    return nullptr;
  }

  // Notification a new fenced frame was created.
  virtual void DidCreateFencedFrame(const blink::RemoteFrameToken& token) {}

  // Called when Blink cannot find a frame with the given name in the frame's
  // browsing instance.  This gives the embedder a chance to return a frame
  // from outside of the browsing instance.
  virtual WebFrame* FindFrame(const WebString& name) { return nullptr; }

  // Notification that the frame is being detached and sends the current frame's
  // navigation state to the browser. Note that WebLocalFrame lifetime is not
  // identical to the lifetime of a "browsing context" in the HTML standard.
  // A WebLocalFrame can be detached for two reasons:
  // - a cross-document navigation, in which case, it will be replaced by a new
  // local or remote frame. In this case, the "browsing context" itself remains.
  // - destruction, e.g. the frame owner element is removed from the DOM, or
  // the window is closed. In this case, the "browsing context" itself is
  // gone.
  virtual void WillDetach(DetachReason detach_reason) {}

  // This frame has been detached. Embedders should release any resources
  // associated with this frame.
  virtual void FrameDetached(DetachReason detach_reason) {}

  // This frame's name has changed.
  virtual void DidChangeName(const WebString& name) {}

  // Called when a watched CSS selector matches or stops matching.
  virtual void DidMatchCSS(
      const std::vector<WebString>& newly_matching_selectors,
      const std::vector<WebString>& stopped_matching_selectors) {}

  // Console messages ----------------------------------------------------

  // Whether or not we should report a detailed message for the given source and
  // severity level.
  virtual bool ShouldReportDetailedMessageForSourceAndSeverity(
      mojom::ConsoleMessageLevel log_level,
      const WebString& source) {
    return false;
  }

  // A new message was added to the console.
  virtual void DidAddMessageToConsole(const WebConsoleMessage&,
                                      const WebString& source_name,
                                      unsigned source_line,
                                      const WebString& stack_trace) {}

  // Navigational queries ------------------------------------------------

  // Requests the client to begin a navigation for this frame.
  //
  // The client can just call CommitNavigation() on this frame's
  // WebNavigationControl in response. This will effectively commit a navigation
  // the frame has asked about. This usually happens for navigations which
  // do not require a network request, e.g. about:blank or mhtml archive.
  //
  // In the case of a navigation which requires network request and goes
  // to the browser process, client calls CreatePlaceholderDocumentLoader
  // (see WebNavigationControl for more details) and commits/cancels
  // the navigation later.
  //
  // It is also totally valid to ignore the request and abandon the
  // navigation entirely.
  //
  // Note that ignoring this method effectively disables any navigations
  // initiated by Blink in this frame.
  virtual void BeginNavigation(std::unique_ptr<blink::WebNavigationInfo> info) {
  }

  // Asks the embedder whether the frame is allowed to navigate the main frame
  // to a data URL.
  // TODO(crbug.com/713259): Move renderer side checks to
  //                         RenderFrameImpl::DecidePolicyForNavigation().
  virtual bool AllowContentInitiatedDataUrlNavigations(const WebURL&) {
    return false;
  }

  // Navigational notifications ------------------------------------------

  // These notifications bracket any loading that occurs in the WebFrame.
  virtual void DidStartLoading() {}
  virtual void DidStopLoading() {}

  // A datasource has been created for a new navigation.  The given
  // datasource will become the provisional datasource for the frame.
  virtual void DidCreateDocumentLoader(WebDocumentLoader*) {}

  // A navigation is about to commit in a new frame that is still provisional
  // (i.e. not swapped into the frame tree). Implementations should perform any
  // bookkeeping work to sync the state of the previous frame and the new frame
  // and use `WebFrame::Swap()` to swap in the new frame.
  //
  // The return value should be the return value of `WebFrame::Swap()`, which
  // returns false if the navigation should not proceed due to the frame being
  // removed from the frame tree by JS while swapping it in, or true otherwise.
  virtual bool SwapIn(WebFrame* previous_frame) { return false; }

  // The navigation has been committed, as a result of
  // WebNavigationControl::CommitNavigation call. The newly created document
  // is committed to the frame, the encoding of the response body is known,
  // but no content has been loaded or parsed yet.
  //
  // When a load commits and a new Document is created, WebLocalFrameClient
  // creates a new BrowserInterfaceBroker endpoint to ensure that interface
  // receivers in the newly committed Document are associated with the correct
  // origin (even if the origin of the old and the new Document are the same).
  // The one exception is if the Window object is reused; in that case, blink
  // passes |should_reset_browser_interface_broker| = false, and the old
  // BrowserInterfaceBroker connection will be reused.
  virtual void DidCommitNavigation(
      WebHistoryCommitType commit_type,
      bool should_reset_browser_interface_broker,
      const network::ParsedPermissionsPolicy& permissions_policy_header,
      const DocumentPolicyFeatureState& document_policy_header) {}

  // A new document has just been committed as a result of evaluating
  // javascript url or XSLT. This document inherited everything from the
  // previous document (url, origin, global object, etc.).
  // DidCommitNavigation is not called in this case.
  virtual void DidCommitDocumentReplacementNavigation(WebDocumentLoader*) {}

  // The window object for the frame has been cleared of any extra properties
  // that may have been set by script from the previously loaded document. This
  // will get invoked multiple times when navigating from an initial empty
  // document to the actual document.
  virtual void DidClearWindowObject() {}

  // The document element has been created.
  // This method may not invalidate the frame, nor execute JavaScript code.
  virtual void DidCreateDocumentElement() {}

  // Like |didCreateDocumentElement|, except this method may run JavaScript
  // code (and possibly invalidate the frame).
  virtual void RunScriptsAtDocumentElementAvailable() {}

  // The page title is available.
  virtual void DidReceiveTitle(const WebString& title) {}

  // The DOMContentLoaded event was dispatched for the frame's document.
  // This method may not execute JavaScript code.
  virtual void DidDispatchDOMContentLoadedEvent() {}

  // Like |DidDispatchDOMContentLoadedEvent|, except this method may run
  // JavaScript code (and possibly invalidate the frame).
  virtual void RunScriptsAtDocumentReady() {}

  // The frame's window.onload event is ready to fire. This method may delay
  // window.onload by incrementing LoadEventDelayCount.
  virtual void RunScriptsAtDocumentIdle() {}

  // The 'load' event was dispatched.
  virtual void DidHandleOnloadEvents() {}

  // The frame's document and all of its subresources succeeded to load.
  virtual void DidFinishLoad() {}

  // The frame's document and all of its subresources succeeded to load for
  // printing.
  virtual void DidFinishLoadForPrinting() {}

  // The navigation resulted in no change to the documents within the page.
  // For example, the navigation may have just resulted in scrolling to a named
  // anchor or a PopState event may have been dispatched.
  // |is_synchronously_committed| is true if the navigation is synchronously
  // committed from within Blink, as opposed to being driven by the browser's
  // navigation stack.
  // `screenshot_destination`, if non-empty, tags the destination of the
  // viewport screenshot.
  virtual void DidFinishSameDocumentNavigation(
      WebHistoryCommitType,
      bool is_synchronously_committed,
      mojom::SameDocumentNavigationType,
      bool is_client_redirect,
      const std::optional<blink::SameDocNavigationScreenshotDestinationToken>&
          screenshot_destination) {}

  // Called when an async same-document navigation fails before commit. This is
  // used in the case where a same-document navigation was instructed to commit
  // asynchronously by the navigate event, then subsequently failed before
  // commit.
  virtual void DidFailAsyncSameDocumentCommit() {}

  // Called before a frame's page is frozen.
  virtual void WillFreezePage() {}

  // The frame's document changed its URL due to document.open().
  virtual void DidOpenDocumentInputStream(const WebURL&) {}

  // Called when a frame's page lifecycle state gets updated.
  virtual void DidSetPageLifecycleState(bool restoring_from_bfcache) {}

  // Immediately notifies the browser of a change in the current HistoryItem.
  // Prefer DidUpdateCurrentHistoryItem().
  virtual void NotifyCurrentHistoryItemChanged() {}

  // Called upon update to scroll position, document state, and other
  // non-navigational events related to the data held by WebHistoryItem.
  // WARNING: This method may be called very frequently.
  virtual void DidUpdateCurrentHistoryItem() {}

  // Returns the effective connection type when the frame was fetched.
  virtual WebEffectiveConnectionType GetEffectiveConnectionType() {
    return WebEffectiveConnectionType::kTypeUnknown;
  }

  // Overrides the effective connection type for testing.
  virtual void SetEffectiveConnectionTypeForTesting(
      WebEffectiveConnectionType) {}

  // Returns token to be used as a frame id in the devtools protocol.
  // It is derived from the content's devtools_frame_token, is
  // defined by the browser and passed into Blink upon frame creation.
  virtual base::UnguessableToken GetDevToolsFrameToken() {
    return base::UnguessableToken::Create();
  }

  // PlzNavigate
  // Called to abort a navigation that is being handled by the browser process.
  // `for_new_navigation` is true iff the abort is the result of beginning a
  // new navigation, since a document should only have one navigation in-flight
  // at a time.
  virtual void AbortClientNavigation(bool for_new_navigation) {}

  // InstalledApp API ----------------------------------------------------

  // Used to access the embedder for the InstalledApp API.
  virtual WebRelatedAppsFetcher* GetRelatedAppsFetcher() { return nullptr; }

  // Editing -------------------------------------------------------------

  // These methods allow the client to intercept editing operations.

  // Called when the selection may have changed (Note, that due to
  // http://crbug.com/632920 the selection may not have changed). Additionally,
  // in some circumstances the browser selection may be known to not match the
  // last synced value, in which case SyncCondition::kForced is passed to force
  // an update even if the selection appears unchanged since the last call.
  virtual void DidChangeSelection(bool is_selection_empty,
                                  SyncCondition force_sync) {}
  virtual void DidChangeContents() {}

  // UI ------------------------------------------------------------------

  // Update a context menu data for testing.
  virtual void UpdateContextMenuDataForTesting(
      const ContextMenuData&,
      const std::optional<gfx::Point>&) {}

  // Called when a new element gets focused. |from_element| is the previously
  // focused element, |to_element| is the newly focused one. Either can be null.
  virtual void FocusedElementChanged(const WebElement& element) {}

  // For the main frame, called when the main frame's dimensions have changed,
  // e.g. resizing a tab causes the document width to change; loading additional
  // content causes the document height to increase; explicitly changing the
  // height of the body element.
  //
  // For a subframe, called when the intersection rect between the main frame
  // and the subframe has changed, e.g. the subframe is initially added; the
  // subframe's position is updated explicitly or inherently (e.g. sticky
  // position while the page is being scrolled).
  virtual void OnMainFrameIntersectionChanged(
      const gfx::Rect& main_frame_intersection_rect) {}

  // Called when the main frame's viewport rectangle (the viewport dimensions
  // and the scroll position) changed, e.g. the user scrolled the main frame or
  // the viewport dimensions themselves changed. Only invoked on the main frame.
  virtual void OnMainFrameViewportRectangleChanged(
      const gfx::Rect& main_frame_viewport_rect) {}

  // Called when an image ad rectangle changed. An empty `image_ad_rect` is used
  // to signal the removal of the rectangle. Only invoked on the main frame.
  virtual void OnMainFrameImageAdRectangleChanged(
      int element_id,
      const gfx::Rect& image_ad_rect) {}

  // Called when an overlay interstitial pop up ad is detected.
  virtual void OnOverlayPopupAdDetected() {}

  // Called when a large sticky ad is detected.
  virtual void OnLargeStickyAdDetected() {}

  // Low-level resource notifications ------------------------------------

  using ForRedirect = base::StrongAlias<class ForRedirectTag, bool>;

  // Called before a request is looked up from the cache. Allows the client
  // to override the url.
  virtual std::optional<WebURL> WillSendRequest(
      const WebURL& target,
      const WebSecurityOrigin& security_origin,
      const net::SiteForCookies& site_for_cookies,
      ForRedirect for_redirect,
      const WebURL& upstream_url) {
    return std::nullopt;
  }

  // A request is about to be sent out, and the client may modify it. The
  // client should not modify the url in this method, instead use
  // OverrideRequestUrl(). Other changes are allowed to the request.
  virtual void FinalizeRequest(WebURLRequest&) {}

  // The specified request was satified from WebCore's memory cache.
  virtual void DidLoadResourceFromMemoryCache(const WebURLRequest&,
                                              const WebURLResponse&) {}

  // A PingLoader was created, and a request dispatched to a URL.
  virtual void DidDispatchPingLoader(const WebURL&) {}

  // A performance timing event (e.g. first paint) occurred
  virtual void DidChangePerformanceTiming() {}

  // A user interaction is observed. A user interaction can be built up from
  // multiple input events (e.g. keydown then keyup). Each of these events has
  // an input to next frame latency. This reports the timings of the max
  // input-to-frame latency for each interaction. `max_event_start` is when
  // input was received, `max_event_end` is when the next frame was
  // presented, `max_event_queued_main_thread` is when the input was queued and
  // `max_event_commit_finish` is when the next commit finished after event has
  // been processed. See https://web.dev/inp/#whats-in-an-interaction for more
  // detailed motivation and explanation.
  virtual void DidObserveUserInteraction(
      base::TimeTicks max_event_start,
      base::TimeTicks max_event_queued_main_thread,
      base::TimeTicks max_event_commit_finish,
      base::TimeTicks max_event_end,
      uint64_t interaction_offset) {}

  // The first scroll delay, which measures the time between the user's first
  // scrolling and the resultant display update, has been observed.
  // The first scroll timestamp is when the first scroll event was created which
  // is the hardware timestamp provided by the host OS.
  virtual void DidObserveFirstScrollDelay(
      base::TimeDelta first_scroll_delay,
      base::TimeTicks first_scroll_timestamp) {}

  // A cpu task or tasks completed.  Triggered when at least 100ms of wall time
  // was spent in tasks on the frame.
  virtual void DidChangeCpuTiming(base::TimeDelta time) {}

  // UseCounter ----------------------------------------------------------
  // Blink exhibited a certain loading behavior that the browser process will
  // use for segregated histograms.
  virtual void DidObserveLoadingBehavior(LoadingBehaviorFlag) {}

  // Blink detected a JavaScript framework that the browser process will use for
  // UKM.
  virtual void DidObserveJavaScriptFrameworks(
      const JavaScriptFrameworkDetectionResult&) {}

  // A subresource load is observed.
  // It is called when there is a subresouce load. The reported values via
  // arguments are cumulative. They are NOT a difference from the previous call.
  virtual void DidObserveSubresourceLoad(
      const SubresourceLoadMetrics& subresource_load_metrics) {}

  // Blink hits the code path for a certain UseCounterFeature for the first time
  // on this frame. As a performance optimization, features already hit on other
  // frames associated with the same page in the renderer are not currently
  // reported.
  virtual void DidObserveNewFeatureUsage(const UseCounterFeature&) {}

  // A new soft navigation was observed.
  virtual void DidObserveSoftNavigation(blink::SoftNavigationMetrics metrics) {}

  // Reports that visible elements in the frame shifted (bit.ly/lsm-explainer).
  virtual void DidObserveLayoutShift(double score, bool after_input_or_scroll) {
  }

  // Script notifications ------------------------------------------------

  // Notifies that a new script context has been created for this frame.
  // This is similar to didClearWindowObject but only called once per
  // frame context.
  virtual void DidCreateScriptContext(v8::Local<v8::Context>,
                                      int32_t world_id) {}

  // WebKit is about to release its reference to a v8 context for a frame.
  virtual void WillReleaseScriptContext(v8::Local<v8::Context>,
                                        int32_t world_id) {}

  // Geometry notifications ----------------------------------------------

  // The main frame scrolled.
  virtual void DidChangeScrollOffset() {}

  // MediaStream -----------------------------------------------------

  virtual WebMediaStreamDeviceObserver* MediaStreamDeviceObserver() {
    return nullptr;
  }

  // Encrypted Media -------------------------------------------------

  virtual WebEncryptedMediaClient* EncryptedMediaClient() { return nullptr; }

  // User agent ------------------------------------------------------

  // Asks the embedder if a specific user agent should be used. Non-empty
  // strings indicate an override should be used. Otherwise,
  // Platform::current()->UserAgent() will be called to provide one.
  virtual WebString UserAgentOverride() { return WebString(); }

  // Asks the embedder what values to send for User Agent client hints
  // (or nullopt if none).  Used only when UserAgentOverride() is non-empty;
  // Platform::current()->UserAgentMetadata() is used otherwise.
  virtual std::optional<UserAgentMetadata> UserAgentMetadataOverride() {
    return std::nullopt;
  }

  //
  // Accessibility -------------------------------------------------------
  //

  // Notifies the embedder about an accessibility event on a WebAXObject.
  virtual void PostAccessibilityEvent(const ui::AXEvent& event) {}

  // Called when accessibility is ready to serialize.
  // Returns true if a serialization occurs.
  virtual bool SendAccessibilitySerialization(
      std::vector<ui::AXTreeUpdate> updates,
      std::vector<ui::AXEvent> events,
      ui::AXLocationAndScrollUpdates location_and_scroll_updates,
      bool had_load_complete_messages) {
    return false;
  }

  virtual bool IsAccessibilityEnabled() const { return false; }

  // Audio Output Devices API --------------------------------------------

  // Checks that the given audio sink exists and is authorized. This is mainly
  // used as a testing hook, if std::nullopt is returned it will fall back
  // checking that a sink exists.
  virtual std::optional<media::OutputDeviceStatus>
  CheckIfAudioSinkExistsAndIsAuthorized(const WebString& sink_id) {
    return std::nullopt;
  }

  // Visibility ----------------------------------------------------------

  // Overwrites the given URL to use an HTML5 embed if possible.
  // An empty URL is returned if the URL is not overriden.
  virtual WebURL OverrideFlashEmbedWithHTML(const WebURL& url) {
    return WebURL();
  }

  // Loading --------------------------------------------------------------

  virtual scoped_refptr<network::SharedURLLoaderFactory> GetURLLoaderFactory() {
    NOTREACHED();
  }

  virtual blink::ChildURLLoaderFactoryBundle* GetLoaderFactoryBundle() {
    NOTREACHED();
  }

  virtual URLLoaderThrottleProvider* GetURLLoaderThrottleProvider() {
    return nullptr;
  }

  virtual scoped_refptr<WebBackgroundResourceFetchAssets>
  MaybeGetBackgroundResourceFetchAssets() {
    return nullptr;
  }

  virtual std::unique_ptr<URLLoader> CreateURLLoaderForTesting();

  virtual void OnStopLoading() {}

  // WebSocket -----------------------------------------------------------
  virtual std::unique_ptr<WebSocketHandshakeThrottle>
  CreateWebSocketHandshakeThrottle() {
    return nullptr;
  }

  // Misc ----------------------------------------------------------------

  // Returns true when the contents of plugin are handled externally. This means
  // the plugin element will own a content frame but the frame is than used
  // externally to load the required handlers.
  virtual bool IsPluginHandledExternally(const WebElement& plugin_element,
                                         const WebURL& url,
                                         const WebString& suggested_mime_type) {
    return false;
  }

  // Specifies whether to disable DOM storage interfaces such as localStorage
  // and sessionStorage.
  virtual bool IsDomStorageDisabled() const { return false; }

  // Returns a scriptable object for the given plugin element. This is used for
  // having an external handler implement certain customized APIs for the
  // plugin element (e.g., to expose postMessage).
  virtual v8::Local<v8::Object> GetScriptableObject(const WebElement&,
                                                    v8::Isolate*) {
    return v8::Local<v8::Object>();
  }

  // Returns true if it has a focused plugin. |rect| is an output parameter to
  // get a caret bounds from the focused plugin.
  virtual bool GetCaretBoundsFromFocusedPlugin(gfx::Rect& rect) {
    return false;
  }

  // Update the current frame selection to the browser if it has changed since
  // the last call. Note that this only synchronizes the selection, if the
  // TextInputState may have changed call DidChangeSelection instead.
  // If the browser selection may not match the last synced
  // value, SyncCondition::kForced can be passed to force a sync.
  virtual void SyncSelectionIfRequired(SyncCondition force_sync) {}

  // TODO(https://crbug.com/787252): Remove the methods below and use the
  // Supplement mechanism.
  virtual void CreateAudioInputStream(
      CrossVariantMojoRemote<
          blink::mojom::RendererAudioInputStreamFactoryClientInterfaceBase>
          client,
      const base::UnguessableToken& session_id,
      const media::AudioParameters& params,
      bool automatic_gain_control,
      uint32_t shared_memory_count,
      CrossVariantMojoReceiver<
          media::mojom::AudioProcessorControlsInterfaceBase> controls_receiver,
      const media::AudioProcessingSettings* settings) {}
  virtual void AssociateInputAndOutputForAec(
      const base::UnguessableToken& input_stream_id,
      const std::string& output_device_id) {}

  // Called immediately following the first compositor-driven (frame-generating)
  // layout that happened after an interesting document lifecycle change (see
  // WebMeaningfulLayout for details.)
  virtual void DidMeaningfulLayout(WebMeaningfulLayout) {}

  // Notification that the BeginMainFrame completed, was committed into the
  // compositor (thread) and submitted to the display compositor.
  virtual void DidCommitAndDrawCompositorFrame() {}

  // Inform the widget that it was hidden.
  virtual void WasHidden() {}

  // Inform the widget that it was shown.
  virtual void WasShown() {}

  virtual void OnFrameVisibilityChanged(mojom::FrameVisibility render_status) {}

  // Called after a navigation which set the shared memory region for
  // tracking smoothness and dropped frames UKMs.
  virtual void SetUpSharedMemoryForUkms(
      base::ReadOnlySharedMemoryRegion smoothness_memory,
      base::ReadOnlySharedMemoryRegion dropped_frames_memory) {}

  // Returns the last commited URL used for UKM. This is slightly different
  // than the document's URL because it will contain a data URL if a base URL
  // was used for its load or if an unreachable URL was used.
  virtual WebURL LastCommittedUrlForUKM() { return WebURL(); }

  // Called when script in the frame (and it subframes) wishes to be printed via
  // a window.print() call.
  virtual void ScriptedPrint() {}

  // This method is ONLY for web tests and is not supposed to be overridden in
  // classes other than web_frame_test_proxy. It's called from accessibility and
  // is used as a way to notify that an accessibility object has been destroyed.
  virtual void HandleAXObjectDetachedForTest(unsigned axid) {}

  // This method is ONLY for web tests and is not supposed to be overridden in
  // classes other than web_frame_test_proxy. It's called from accessibility and
  // is used as a way to tunnel events to the accessibility_controller in web
  // tests.
  virtual void HandleWebAccessibilityEventForTest(
      const blink::WebAXObject& object,
      const char* event_name,
      const std::vector<ui::AXEventIntent>& event_intents) {}

  // This method is ONLY for web tests and is not supposed to be overridden in
  // classes other than web_frame_test_proxy. It's called from accessibility and
  // is used as a way to tunnel events to the accessibility_controller in web
  // tests.
  virtual void HandleWebAccessibilityEventForTest(const ui::AXEvent& event) {}

  // Create a new related WebView.  This method must clone its session storage
  // so any subsequent calls to createSessionStorageNamespace conform to the
  // WebStorage specification.
  // The request parameter is only for the client to check if the request
  // could be fulfilled.  The client should not load the request.
  // The policy parameter indicates how the new view will be displayed in
  // LocalMainFrameHost::ShowCreatedWidget.
  virtual WebView* CreateNewWindow(
      const WebURLRequest& request,
      const WebWindowFeatures& features,
      const WebString& name,
      const gfx::Rect& requested_screen_rect,
      WebNavigationPolicy policy,
      network::mojom::WebSandboxFlags,
      const SessionStorageNamespaceId& session_storage_namespace_id,
      bool& consumed_user_gesture,
      const std::optional<Impression>&,
      const std::optional<WebPictureInPictureWindowOptions>& pip_options,
      const WebURL& base_url) {
    return nullptr;
  }

  virtual std::unique_ptr<WebLinkPreviewTriggerer> CreateLinkPreviewTriggerer();

  virtual void SetLinkPreviewTriggererForTesting(
      std::unique_ptr<WebLinkPreviewTriggerer> trigger);

  virtual base::ScopedClosureRunner CreateScopedClientNavigationThrottler() {
    return {};
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_CLIENT_H_
