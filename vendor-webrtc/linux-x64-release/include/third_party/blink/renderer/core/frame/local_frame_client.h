/*
 * Copyright (C) 2006, 2007, 2008, 2009, 2010, 2011, 2012 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_CLIENT_H_

#include <memory>
#include <optional>

#include "base/time/time.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-blink-forward.h"
#include "third_party/blink/public/common/loader/loading_behavior_flag.h"
#include "third_party/blink/public/common/loader/url_loader_factory_bundle.h"
#include "third_party/blink/public/common/performance/performance_timeline_constants.h"
#include "third_party/blink/public/common/subresource_load_metrics.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/common/use_counter/use_counter_feature.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fenced_frame/fenced_frame.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/remote_frame.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/triggering_event_info.mojom-blink-forward.h"
#include "third_party/blink/public/platform/child_url_loader_factory_bundle.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/public/platform/web_background_resource_fetch_assets.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_effective_connection_type.h"
#include "third_party/blink/public/platform/web_worker_fetch_context.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/public/web/web_history_commit_type.h"
#include "third_party/blink/public/web/web_manifest_manager.h"
#include "third_party/blink/public/web/web_navigation_params.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/icon_url.h"
#include "third_party/blink/renderer/core/frame/frame_client.h"
#include "third_party/blink/renderer/core/frame/frame_types.h"
#include "third_party/blink/renderer/core/html/link_resource.h"
#include "third_party/blink/renderer/core/loader/document_loader.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/loader/navigation_policy.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/network/content_security_policy_parsers.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace network {
class SharedURLLoaderFactory;
}  // namespace network

namespace blink {

class AssociatedInterfaceProvider;
class DocumentLoader;
class HTMLFencedFrameElement;
class HTMLFormElement;
class HTMLFrameOwnerElement;
class HTMLMediaElement;
class HTMLPlugInElement;
class HistoryItem;
class KURL;
class LocalDOMWindow;
class LocalFrame;
class RemoteFrame;
class RemotePlaybackClient;
class ResourceError;
class ResourceRequest;
class ResourceResponse;
class SourceLocation;
class WebContentCaptureClient;
class WebDedicatedWorkerHostFactoryClient;
class WebLocalFrame;
class WebMediaPlayer;
class WebMediaPlayerClient;
class WebMediaPlayerSource;
class WebPluginContainerImpl;
class WebServiceWorkerProvider;
class WebSpellCheckPanelHostClient;
class WebTextCheckClient;
class URLLoader;
class ResourceLoadInfoNotifierWrapper;
enum class SyncCondition;
struct Impression;
struct JavaScriptFrameworkDetectionResult;

namespace scheduler {
class TaskAttributionId;
}  // namespace scheduler

class CORE_EXPORT LocalFrameClient : public FrameClient {
 public:
  ~LocalFrameClient() override = default;

  virtual WebContentCaptureClient* GetWebContentCaptureClient() const {
    return nullptr;
  }

  virtual WebLocalFrame* GetWebFrame() const { return nullptr; }

  virtual bool HasWebView() const = 0;  // mainly for assertions

  virtual base::UnguessableToken GetDevToolsFrameToken() const = 0;

  virtual void WillBeDetached() = 0;
  virtual void DispatchFinalizeRequest(ResourceRequest&) = 0;
  virtual std::optional<KURL> DispatchWillSendRequest(
      const KURL& requested_url,
      const scoped_refptr<const SecurityOrigin>& requestor_origin,
      const net::SiteForCookies& site_for_cookies,
      bool has_redirect_info,
      const KURL& upstream_url) = 0;
  virtual void DispatchDidLoadResourceFromMemoryCache(
      const ResourceRequest&,
      const ResourceResponse&) = 0;

  virtual void DispatchDidHandleOnloadEvents() = 0;
  virtual void DidFinishSameDocumentNavigation(
      WebHistoryCommitType,
      bool is_synchronously_committed,
      mojom::blink::SameDocumentNavigationType,
      bool is_client_redirect,
      bool is_browser_initiated,
      bool should_skip_screenshot) {}
  virtual void DidFailAsyncSameDocumentCommit() {}
  virtual void DispatchDidOpenDocumentInputStream(const KURL&) {}
  virtual void DispatchDidReceiveTitle(const String&) = 0;
  virtual void DispatchDidCommitLoad(
      HistoryItem* item,
      WebHistoryCommitType commit_type,
      bool should_reset_browser_interface_broker,
      const network::ParsedPermissionsPolicy& permissions_policy_header,
      const blink::DocumentPolicyFeatureState& document_policy_header) = 0;
  virtual void DispatchDidFailLoad(const ResourceError&,
                                   WebHistoryCommitType) = 0;
  virtual void DispatchDidDispatchDOMContentLoadedEvent() = 0;
  virtual void DispatchDidFinishLoad() = 0;
  virtual void DispatchDidFinishLoadForPrinting() {}

  virtual void BeginNavigation(
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
      network::mojom::CSPDisposition
          should_check_main_world_content_security_policy,
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
      bool has_rel_opener) = 0;

  virtual void DispatchWillSendSubmitEvent(HTMLFormElement*) = 0;

  virtual void DidStartLoading() = 0;
  virtual void DidStopLoading() = 0;

  virtual bool NavigateBackForward(
      int offset,
      std::optional<scheduler::TaskAttributionId>
          soft_navigation_heuristics_task_id) const = 0;

  virtual void DidDispatchPingLoader(const KURL&) = 0;

  // Will be called when |PerformanceTiming| events are updated
  virtual void DidChangePerformanceTiming() {}

  // Will be called when a user interaction is observed.
  virtual void DidObserveUserInteraction(
      base::TimeTicks max_event_start,
      base::TimeTicks max_event_queued_main_thread,
      base::TimeTicks max_event_commit_finish,
      base::TimeTicks max_event_end,
      uint64_t interaction_offset) {}

  // Will be called when |CpuTiming| events are updated
  virtual void DidChangeCpuTiming(base::TimeDelta time) {}

  // Will be called when a particular loading code path has been used. This
  // propogates renderer loading behavior to the browser process for histograms.
  virtual void DidObserveLoadingBehavior(LoadingBehaviorFlag) {}

  // propagates framework detection info to the browser process for histograms.
  virtual void DidObserveJavaScriptFrameworks(
      const JavaScriptFrameworkDetectionResult&) {}

  // Will be called when a sub resource load happens.
  virtual void DidObserveSubresourceLoad(
      const SubresourceLoadMetrics& subresource_load_metrics) {}

  // Will be called when a new UseCounterFeature has been observed in a frame.
  // This propagates feature usage to the browser process for histograms.
  virtual void DidObserveNewFeatureUsage(const UseCounterFeature&) {}

  // A new soft navigation was observed.
  virtual void DidObserveSoftNavigation(SoftNavigationMetrics metrics) {}

  // Reports that visible elements in the frame shifted (bit.ly/lsm-explainer).
  virtual void DidObserveLayoutShift(double score, bool after_input_or_scroll) {
  }

  // Transmits the change in the set of watched CSS selectors property that
  // match any element on the frame.
  virtual void SelectorMatchChanged(
      const Vector<String>& added_selectors,
      const Vector<String>& removed_selectors) = 0;

  virtual void DidCreateDocumentLoader(DocumentLoader*) = 0;

  virtual String UserAgentOverride() = 0;
  virtual String UserAgent() = 0;
  virtual std::optional<blink::UserAgentMetadata> UserAgentMetadata() = 0;

  virtual String DoNotTrackValue() = 0;

  virtual void TransitionToCommittedForNewPage() = 0;

  virtual LocalFrame* CreateFrame(const AtomicString& name,
                                  HTMLFrameOwnerElement*) = 0;

  // Creates a remote fenced frame hosted by an MPArch frame tree for the
  // |HTMLFencedFrameElement|.
  virtual RemoteFrame* CreateFencedFrame(
      HTMLFencedFrameElement*,
      mojo::PendingAssociatedReceiver<mojom::blink::FencedFrameOwnerHost>) = 0;

  // TODO(crbug.com/40511450): Remove `load_manually` once PPAPI is gone.
  virtual WebPluginContainerImpl* CreatePlugin(HTMLPlugInElement&,
                                               const KURL&,
                                               const Vector<String>&,
                                               const Vector<String>&,
                                               const String&,
                                               bool load_manually) = 0;

  virtual std::unique_ptr<WebMediaPlayer> CreateWebMediaPlayer(
      HTMLMediaElement&,
      const WebMediaPlayerSource&,
      WebMediaPlayerClient*) = 0;
  virtual RemotePlaybackClient* CreateRemotePlaybackClient(
      HTMLMediaElement&) = 0;

  virtual void DidCommitDocumentReplacementNavigation(DocumentLoader*) = 0;
  virtual void DispatchDidClearWindowObjectInMainWorld(
      v8::Isolate* isolate,
      v8::MicrotaskQueue* microtask_queue) = 0;
  virtual void DocumentElementAvailable() = 0;
  virtual void RunScriptsAtDocumentElementAvailable() = 0;
  virtual void RunScriptsAtDocumentReady(bool document_is_empty) = 0;
  virtual void RunScriptsAtDocumentIdle() = 0;

  virtual void DidCreateScriptContext(v8::Local<v8::Context>,
                                      int32_t world_id) = 0;
  virtual void WillReleaseScriptContext(v8::Local<v8::Context>,
                                        int32_t world_id) = 0;
  virtual bool AllowScriptExtensions() = 0;

  virtual void DidChangeScrollOffset() {}

  // Immediately notifies the browser of a change in the current HistoryItem.
  // Prefer DidUpdateCurrentHistoryItem().
  virtual void NotifyCurrentHistoryItemChanged() {}
  // Notifies the browser of a change in the current HistoryItem on a timer,
  // allowing batching of updates.
  virtual void DidUpdateCurrentHistoryItem() {}

  // Called when a content-initiated, main frame navigation to a data URL is
  // about to occur.
  virtual bool AllowContentInitiatedDataUrlNavigations(const KURL&) {
    return false;
  }

  virtual void DidChangeName(const String&) {}

  virtual std::unique_ptr<WebServiceWorkerProvider>
  CreateServiceWorkerProvider() = 0;

  virtual WebContentSettingsClient* GetContentSettingsClient() = 0;

  virtual void DispatchDidChangeManifest() {}

  unsigned BackForwardLength() override { return 0; }

  virtual bool IsLocalFrameClientImpl() const { return false; }

  // Overwrites the given URL to use an HTML5 embed if possible. An empty URL is
  // returned if the URL is not overriden.
  virtual KURL OverrideFlashEmbedWithHTML(const KURL&) { return KURL(); }

  virtual AssociatedInterfaceProvider*
  GetRemoteNavigationAssociatedInterfaces() = 0;

  virtual void NotifyUserActivation() {}

  virtual void AbortClientNavigation(bool for_new_navigation) {}

  virtual WebSpellCheckPanelHostClient* SpellCheckPanelHostClient() const = 0;

  virtual WebTextCheckClient* GetTextCheckerClient() const = 0;

  virtual scoped_refptr<network::SharedURLLoaderFactory>
  GetURLLoaderFactory() = 0;
  virtual std::unique_ptr<URLLoader> CreateURLLoaderForTesting() = 0;
  virtual blink::ChildURLLoaderFactoryBundle* GetLoaderFactoryBundle() = 0;

  virtual scoped_refptr<WebBackgroundResourceFetchAssets>
  MaybeGetBackgroundResourceFetchAssets() = 0;

  virtual void SetVirtualTimePauser(
      WebScopedVirtualTimePauser virtual_time_pauser) {}

  virtual String evaluateInInspectorOverlayForTesting(const String& script) = 0;

  virtual bool HandleCurrentKeyboardEvent() { return false; }

  // Called when the selection may have changed (Note, that due to
  // http://crbug.com/632920 the selection may not have changed). Additionally,
  // in some circumstances the browser selection may be known to not match the
  // last synced value, in which case SyncCondition::kForced is passed to force
  // an update even if the selection appears unchanged since the last call.
  virtual void DidChangeSelection(bool is_selection_empty,
                                  blink::SyncCondition force_sync) {}

  virtual void DidChangeContents() {}

  virtual Frame* FindFrame(const AtomicString& name) const = 0;

  virtual void OnOverlayPopupAdDetected() {}

  virtual void OnLargeStickyAdDetected() {}

  virtual void FocusedElementChanged(Element* element) {}

  // Returns true when the contents of plugin are handled externally. This means
  // the plugin element will own a content frame but the frame is than used
  // externally to load the required handelrs.
  virtual bool IsPluginHandledExternally(HTMLPlugInElement&,
                                         const KURL&,
                                         const String&) {
    return false;
  }

  // When a plugin element is handled externally, this method is used to obtain
  // a scriptable object which exposes custom API such as postMessage.
  virtual v8::Local<v8::Object> GetScriptableObject(HTMLPlugInElement&,
                                                    v8::Isolate*) {
    return v8::Local<v8::Object>();
  }

  // Returns a new WebWorkerFetchContext for worklets.
  virtual scoped_refptr<WebWorkerFetchContext> CreateWorkletFetchContext() {
    return nullptr;
  }

  // Returns a new WebWorkerFetchContext for dedicated workers.
  virtual scoped_refptr<WebWorkerFetchContext> CreateWorkerFetchContext(
      WebDedicatedWorkerHostFactoryClient*) {
    return nullptr;
  }

  virtual std::unique_ptr<WebContentSettingsClient>
  CreateWorkerContentSettingsClient() {
    return nullptr;
  }

  virtual void SetMouseCapture(bool) {}

  virtual void NotifyAutoscrollForSelectionInMainFrame(bool) {}

  virtual std::unique_ptr<ResourceLoadInfoNotifierWrapper>
  CreateResourceLoadInfoNotifierWrapper() {
    return nullptr;
  }

  // Specifies whether to disable DOM storage interfaces such as localStorage
  // and sessionStorage.
  virtual bool IsDomStorageDisabled() const { return false; }

  // Debugging -----------------------------------------------------------
  virtual void BindDevToolsAgent(
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsAgentHost> host,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsAgent> receiver) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_CLIENT_H_
