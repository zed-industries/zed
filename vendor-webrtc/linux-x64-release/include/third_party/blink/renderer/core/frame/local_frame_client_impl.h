/*
 * Copyright (C) 2009, 2012 Google Inc. All rights reserved.
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_CLIENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_CLIENT_IMPL_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/receiver_set.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/public/common/performance/performance_timeline_constants.h"
#include "third_party/blink/public/common/subresource_load_metrics.h"
#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/local_frame_client.h"
#include "third_party/blink/renderer/core/frame/web_local_frame_impl.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class WebDevToolsAgentImpl;
class WebLocalFrameImpl;
class WebSpellCheckPanelHostClient;

class CORE_EXPORT LocalFrameClientImpl final : public LocalFrameClient {
 public:
  explicit LocalFrameClientImpl(WebLocalFrameImpl*);
  ~LocalFrameClientImpl() override;

  void Trace(Visitor*) const override;

  WebLocalFrameImpl* GetWebFrame() const override;

  // LocalFrameClient ----------------------------------------------
  WebContentCaptureClient* GetWebContentCaptureClient() const override;
  void DidCommitDocumentReplacementNavigation(DocumentLoader*) override;
  // Notifies the WebView delegate that the JS window object has been cleared,
  // giving it a chance to bind native objects to the window before script
  // parsing begins.
  void DispatchDidClearWindowObjectInMainWorld(
      v8::Isolate* isolate,
      v8::MicrotaskQueue* microtask_queue) override;
  void DocumentElementAvailable() override;
  void RunScriptsAtDocumentElementAvailable() override;
  void RunScriptsAtDocumentReady(bool document_is_empty) override;
  void RunScriptsAtDocumentIdle() override;

  void DidCreateScriptContext(v8::Local<v8::Context>,
                              int32_t world_id) override;
  void WillReleaseScriptContext(v8::Local<v8::Context>,
                                int32_t world_id) override;

  // Returns true if we should allow register V8 extensions to be added.
  bool AllowScriptExtensions() override;

  bool HasWebView() const override;
  bool InShadowTree() const override;
  void WillBeDetached() override;
  void Detached(FrameDetachType) override;
  void DispatchFinalizeRequest(ResourceRequest&) override;
  std::optional<KURL> DispatchWillSendRequest(
      const KURL& requested_url,
      const scoped_refptr<const SecurityOrigin>& requestor_origin,
      const net::SiteForCookies& site_for_cookies,
      bool has_redirect_info,
      const KURL& upstream_url) override;
  void DispatchDidLoadResourceFromMemoryCache(const ResourceRequest&,
                                              const ResourceResponse&) override;
  void DispatchDidHandleOnloadEvents() override;
  void DidFinishSameDocumentNavigation(WebHistoryCommitType,
                                       bool is_handled_within_agent,
                                       mojom::blink::SameDocumentNavigationType,
                                       bool is_client_redirect,
                                       bool is_browser_initiated,
                                       bool should_skip_screenshot) override;
  void DidFailAsyncSameDocumentCommit() override;
  void DispatchDidOpenDocumentInputStream(const KURL& url) override;
  void DispatchDidReceiveTitle(const String&) override;
  void DispatchDidCommitLoad(
      HistoryItem*,
      WebHistoryCommitType,
      bool should_reset_browser_interface_broker,
      const network::ParsedPermissionsPolicy& permissions_policy_header,
      const blink::DocumentPolicyFeatureState& document_policy_header) override;
  void DispatchDidFailLoad(const ResourceError&, WebHistoryCommitType) override;
  void DispatchDidDispatchDOMContentLoadedEvent() override;
  void DispatchDidFinishLoad() override;
  void DispatchDidFinishLoadForPrinting() override;

  void BeginNavigation(
      const ResourceRequest&,
      const KURL& requestor_base_url,
      mojom::RequestContextFrameType,
      LocalDOMWindow* origin_window,
      DocumentLoader*,
      WebNavigationType,
      NavigationPolicy,
      WebFrameLoadType,
      mojom::blink::ForceHistoryPush,
      bool is_client_redirect,
      // TODO(crbug.com/1315802): Refactor _unfencedTop handling.
      bool is_unfenced_top_navigation,
      mojom::blink::TriggeringEventInfo,
      HTMLFormElement*,
      network::mojom::CSPDisposition should_bypass_main_world_csp,
      mojo::PendingRemote<mojom::blink::BlobURLToken>,
      base::TimeTicks input_start_time,
      base::TimeTicks actual_navigation_start,
      const String& href_translate,
      const std::optional<Impression>& impression,
      const LocalFrameToken* initiator_frame_token,
      std::unique_ptr<SourceLocation> source_location,
      mojo::PendingRemote<mojom::blink::NavigationStateKeepAliveHandle>
          initiator_navigation_state_keep_alive_handle,
      bool is_container_initiated,
      bool has_rel_opener) override;
  void DispatchWillSendSubmitEvent(HTMLFormElement*) override;
  void DidStartLoading() override;
  void DidStopLoading() override;
  bool NavigateBackForward(
      int offset,
      std::optional<scheduler::TaskAttributionId>
          soft_navigation_heuristics_task_id) const override;
  void DidDispatchPingLoader(const KURL&) override;
  void DidChangePerformanceTiming() override;
  void DidObserveUserInteraction(base::TimeTicks max_event_start,
                                 base::TimeTicks max_event_queued_main_thread,
                                 base::TimeTicks max_event_commit_finish,
                                 base::TimeTicks max_event_end,
                                 uint64_t interaction_offset) override;
  void DidChangeCpuTiming(base::TimeDelta) override;
  void DidObserveLoadingBehavior(LoadingBehaviorFlag) override;
  void DidObserveJavaScriptFrameworks(
      const JavaScriptFrameworkDetectionResult&) override;
  void DidObserveSubresourceLoad(
      const SubresourceLoadMetrics& subresource_load_metrics) override;
  void DidObserveNewFeatureUsage(const UseCounterFeature&) override;
  void DidObserveSoftNavigation(SoftNavigationMetrics metrics) override;
  void DidObserveLayoutShift(double score, bool after_input_or_scroll) override;
  void SelectorMatchChanged(const Vector<String>& added_selectors,
                            const Vector<String>& removed_selectors) override;

  void DidCreateDocumentLoader(DocumentLoader*) override;

  String UserAgentOverride() override;
  WTF::String UserAgent() override;
  std::optional<blink::UserAgentMetadata> UserAgentMetadata() override;
  WTF::String DoNotTrackValue() override;
  void TransitionToCommittedForNewPage() override;
  LocalFrame* CreateFrame(const WTF::AtomicString& name,
                          HTMLFrameOwnerElement*) override;

  RemoteFrame* CreateFencedFrame(
      HTMLFencedFrameElement*,
      mojo::PendingAssociatedReceiver<mojom::blink::FencedFrameOwnerHost>)
      override;

  WebPluginContainerImpl* CreatePlugin(HTMLPlugInElement&,
                                       const KURL&,
                                       const Vector<WTF::String>&,
                                       const Vector<WTF::String>&,
                                       const WTF::String&,
                                       bool load_manually) override;
  std::unique_ptr<WebMediaPlayer> CreateWebMediaPlayer(
      HTMLMediaElement&,
      const WebMediaPlayerSource&,
      WebMediaPlayerClient*) override;
  RemotePlaybackClient* CreateRemotePlaybackClient(HTMLMediaElement&) override;
  void DidChangeScrollOffset() override;
  void NotifyCurrentHistoryItemChanged() override;
  void DidUpdateCurrentHistoryItem() override;

  bool AllowContentInitiatedDataUrlNavigations(const KURL&) override;

  void DidChangeName(const String&) override;

  std::unique_ptr<WebServiceWorkerProvider> CreateServiceWorkerProvider()
      override;
  WebContentSettingsClient* GetContentSettingsClient() override;

  void DispatchDidChangeManifest() override;

  unsigned BackForwardLength() override;

  KURL OverrideFlashEmbedWithHTML(const KURL&) override;

  void NotifyUserActivation() override;

  void AbortClientNavigation(bool for_new_navigation) override;

  WebSpellCheckPanelHostClient* SpellCheckPanelHostClient() const override;

  WebTextCheckClient* GetTextCheckerClient() const override;

  scoped_refptr<network::SharedURLLoaderFactory> GetURLLoaderFactory() override;
  std::unique_ptr<URLLoader> CreateURLLoaderForTesting() override;

  blink::ChildURLLoaderFactoryBundle* GetLoaderFactoryBundle() override;

  scoped_refptr<WebBackgroundResourceFetchAssets>
  MaybeGetBackgroundResourceFetchAssets() override;

  AssociatedInterfaceProvider* GetRemoteNavigationAssociatedInterfaces()
      override;

  base::UnguessableToken GetDevToolsFrameToken() const override;

  String evaluateInInspectorOverlayForTesting(const String& script) override;

  bool HandleCurrentKeyboardEvent() override;

  void DidChangeSelection(bool is_selection_empty,
                          blink::SyncCondition force_sync) override;

  void DidChangeContents() override;

  Frame* FindFrame(const AtomicString& name) const override;

  void FocusedElementChanged(Element* element) override;

  void OnMainFrameIntersectionChanged(
      const gfx::Rect& main_frame_intersection_rect) override;

  void OnMainFrameViewportRectangleChanged(
      const gfx::Rect& main_frame_viewport_rect) override;

  void OnMainFrameImageAdRectangleChanged(
      DOMNodeId element_id,
      const gfx::Rect& image_ad_rect) override;

  void OnOverlayPopupAdDetected() override;

  void OnLargeStickyAdDetected() override;

  bool IsPluginHandledExternally(HTMLPlugInElement&,
                                 const KURL&,
                                 const String&) override;
  v8::Local<v8::Object> GetScriptableObject(HTMLPlugInElement&,
                                            v8::Isolate*) override;

  scoped_refptr<WebWorkerFetchContext> CreateWorkletFetchContext() override;
  scoped_refptr<WebWorkerFetchContext> CreateWorkerFetchContext(
      WebDedicatedWorkerHostFactoryClient*) override;
  std::unique_ptr<WebContentSettingsClient> CreateWorkerContentSettingsClient()
      override;

  void SetMouseCapture(bool capture) override;

  void NotifyAutoscrollForSelectionInMainFrame(
      bool autoscroll_selection) override;

  std::unique_ptr<blink::ResourceLoadInfoNotifierWrapper>
  CreateResourceLoadInfoNotifierWrapper() override;

  void BindDevToolsAgent(
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsAgentHost> host,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsAgent> receiver)
      override;

  bool IsDomStorageDisabled() const override;

 private:
  bool IsLocalFrameClientImpl() const override { return true; }
  WebDevToolsAgentImpl* DevToolsAgent(bool create_if_necessary);

  // The WebFrame that owns this object and manages its lifetime. Therefore,
  // the web frame object is guaranteed to exist.
  Member<WebLocalFrameImpl> web_frame_;

  String user_agent_;
};

template <>
struct DowncastTraits<LocalFrameClientImpl> {
  static bool AllowFrom(const LocalFrameClient& client) {
    return client.IsLocalFrameClientImpl();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_CLIENT_IMPL_H_
