// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_MOJO_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_MOJO_HANDLER_H_

#include "build/build_config.h"
#include "cc/input/browser_controls_offset_tag_modifications.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/confidence_level.mojom-blink.h"
#include "third_party/blink/public/mojom/device_posture/device_posture_provider.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/fullscreen.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/media/fullscreen_video_element.mojom-blink.h"
#include "third_party/blink/public/mojom/reporting/reporting.mojom-blink.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

#if BUILDFLAG(IS_MAC)
#include "third_party/blink/public/mojom/input/text_input_host.mojom-blink.h"
#endif

namespace blink {

class Document;
class LocalDOMWindow;
class LocalFrame;
class Page;

// LocalFrameMojoHandler is a part of LocalFrame, and is responsible for having
// Mojo-related stuff in order to avoid including full mojom headers from
// local_frame.h.
//
// This class should have:
//  - Mojo receivers
//  - Mojo remotes
//  - Data members of which types are defined by mojom.
//
// A single LocalFrame instance owns a single LocalFrameMojoHandler instance.
class LocalFrameMojoHandler
    : public GarbageCollected<LocalFrameMojoHandler>,
      public mojom::blink::LocalFrame,
      public mojom::blink::LocalMainFrame,
      public mojom::blink::FullscreenVideoElementHandler,
      public mojom::blink::DevicePostureClient {
 public:
  explicit LocalFrameMojoHandler(blink::LocalFrame& frame);
  void Trace(Visitor* visitor) const;

  void WasAttachedAsLocalMainFrame();
  void DidDetachFrame();

  void ClosePageForTesting();

  mojom::blink::LocalFrameHost& LocalFrameHostRemote() {
    return *local_frame_host_remote_.get();
  }

  mojom::blink::NonAssociatedLocalFrameHost&
  NonAssociatedLocalFrameHostRemote() {
    return *non_associated_local_frame_host_remote_.get();
  }

  mojom::blink::ReportingServiceProxy* ReportingService();
  mojom::blink::DevicePostureProvider* DevicePostureProvider();
  mojom::blink::BackForwardCacheControllerHost&
  BackForwardCacheControllerHostRemote();

#if BUILDFLAG(IS_MAC)
  mojom::blink::TextInputHost& TextInputHost();
  void ResetTextInputHostForTesting();
  void RebindTextInputHostForTesting();
#endif

  mojom::blink::DevicePostureType GetDevicePosture();
  void OverrideDevicePostureForEmulation(
      mojom::blink::DevicePostureType device_posture_param);
  void DisableDevicePostureOverrideForEmulation();

 private:
  Page* GetPage() const;
  LocalDOMWindow* DomWindow() const;
  Document* GetDocument() const;

  void BindToLocalFrameReceiver(
      mojo::PendingAssociatedReceiver<mojom::blink::LocalFrame> receiver);
  void BindToMainFrameReceiver(
      mojo::PendingAssociatedReceiver<mojom::blink::LocalMainFrame> receiver);
  void BindFullscreenVideoElementReceiver(
      mojo::PendingAssociatedReceiver<
          mojom::blink::FullscreenVideoElementHandler> receiver);

  // blink::mojom::LocalFrame overrides:
  void GetTextSurroundingSelection(
      uint32_t max_length,
      GetTextSurroundingSelectionCallback callback) final;
  void SendInterventionReport(const String& id, const String& message) final;
  void SetFrameOwnerProperties(
      mojom::blink::FrameOwnerPropertiesPtr properties) final;
  void NotifyUserActivation(
      mojom::blink::UserActivationNotificationType notification_type) final;
  void NotifyVirtualKeyboardOverlayRect(const gfx::Rect& keyboard_rect) final;
  void NotifyContextMenuInsetsObservers(const gfx::Rect&) final;
  void AddMessageToConsole(mojom::blink::ConsoleMessageLevel level,
                           const WTF::String& message,
                           bool discard_duplicates) final;
  void SwapInImmediately() final;
  void CheckCompleted() final;
  void StopLoading() final;
  void Collapse(bool collapsed) final;
  void EnableViewSourceMode() final;
  void Focus() final;
  void ClearFocusedElement() final;
  void CopyImageAt(const gfx::Point& window_point) final;
  void SaveImageAt(const gfx::Point& window_point) final;
  void ReportBlinkFeatureUsage(const Vector<mojom::blink::WebFeature>&) final;
  void RenderFallbackContent() final;
  void BeforeUnload(bool is_reload, BeforeUnloadCallback callback) final;
  void MediaPlayerActionAt(const gfx::Point& window_point,
                           mojom::blink::MediaPlayerActionPtr action) final;
  void RequestVideoFrameAtWithBoundsHint(
      const gfx::Point& window_point,
      const gfx::Size& max_size,
      int max_area,
      RequestVideoFrameAtWithBoundsHintCallback callback) final;
  void AdvanceFocusInFrame(
      mojom::blink::FocusType focus_type,
      const std::optional<RemoteFrameToken>& source_frame_token) final;
  void AdvanceFocusForIME(mojom::blink::FocusType focus_type) final;
  void ReportContentSecurityPolicyViolation(
      network::mojom::blink::CSPViolationPtr csp_violation) final;
  // Updates the snapshot policy attributes (sandbox flags and permissions
  // policy container policy) in the frame's FrameOwner. This is used when this
  // frame's parent is in another process and it dynamically updates this
  // frame's sandbox flags or container policy. The new policy won't take effect
  // until the next navigation.
  void DidUpdateFramePolicy(const FramePolicy& frame_policy) final;
  void OnFrameVisibilityChanged(mojom::blink::FrameVisibility visibility) final;
  void PostMessageEvent(
      const std::optional<RemoteFrameToken>& source_frame_token,
      const String& source_origin,
      const String& target_origin,
      BlinkTransferableMessage message) final;
  void JavaScriptMethodExecuteRequest(
      const String& object_name,
      const String& method_name,
      base::Value::List arguments,
      bool wants_result,
      JavaScriptMethodExecuteRequestCallback callback) final;
  void JavaScriptExecuteRequest(
      const String& javascript,
      bool wants_result,
      JavaScriptExecuteRequestCallback callback) final;
  void JavaScriptExecuteRequestForTests(
      const String& javascript,
      bool has_user_gesture,
      bool resolve_promises,
      bool honor_js_content_settings,
      int32_t world_id,
      JavaScriptExecuteRequestForTestsCallback callback) final;
  void JavaScriptExecuteRequestInIsolatedWorld(
      const String& javascript,
      bool wants_result,
      int32_t world_id,
      JavaScriptExecuteRequestInIsolatedWorldCallback callback) final;
#if BUILDFLAG(IS_MAC)
  void GetCharacterIndexAtPoint(const gfx::Point& point) final;
  void GetFirstRectForRange(const gfx::Range& range) final;
  void GetStringForRange(const gfx::Range& range,
                         GetStringForRangeCallback callback) final;
#endif
  void BindReportingObserver(
      mojo::PendingReceiver<mojom::blink::ReportingObserver> receiver) final;
  void UpdateOpener(
      const std::optional<blink::FrameToken>& opener_routing_id) final;
  void GetSavableResourceLinks(GetSavableResourceLinksCallback callback) final;
  void MixedContentFound(
      const KURL& main_resource_url,
      const KURL& mixed_content_url,
      mojom::blink::RequestContextType request_context,
      bool was_allowed,
      const KURL& url_before_redirects,
      bool had_redirect,
      network::mojom::blink::SourceLocationPtr source_location) final;
  void BindDevToolsAgent(
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsAgentHost> host,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsAgent> receiver)
      final;
#if BUILDFLAG(IS_ANDROID)
  void ExtractSmartClipData(const gfx::Rect& rect,
                            ExtractSmartClipDataCallback callback) final;
#endif
  void HandleRendererDebugURL(const KURL& url) final;
  void GetCanonicalUrlForSharing(
      GetCanonicalUrlForSharingCallback callback) final;
  void GetOpenGraphMetadata(GetOpenGraphMetadataCallback callback) final;

  void SetNavigationApiHistoryEntriesForRestore(
      mojom::blink::NavigationApiHistoryEntryArraysPtr,
      mojom::blink::NavigationApiEntryRestoreReason) final;
  void UpdatePrerenderURL(const KURL& matched_url,
                          UpdatePrerenderURLCallback callback) final;
  void NotifyNavigationApiOfDisposedEntries(
      const WTF::Vector<WTF::String>&) final;
  void TraverseCancelled(const String& navigation_api_key,
                         mojom::blink::TraverseCancelledReason reason) final;
  void DispatchNavigateEventForCrossDocumentTraversal(
      const KURL&,
      const std::string& page_state,
      bool is_browser_initiated) final;
  void SnapshotDocumentForViewTransition(
      const blink::ViewTransitionToken& navigation_id,
      mojom::blink::PageSwapEventParamsPtr,
      SnapshotDocumentForViewTransitionCallback callback) final;
  void NotifyViewTransitionAbortedToOldDocument() final;
  void DispatchPageSwap(mojom::blink::PageSwapEventParamsPtr) final;

  void AddResourceTimingEntryForFailedSubframeNavigation(
      const FrameToken& subframe_token,
      const KURL& initial_url,
      base::TimeTicks start_time,
      base::TimeTicks redirect_time,
      base::TimeTicks request_start,
      base::TimeTicks response_start,
      uint32_t response_code,
      const WTF::String& mime_type,
      network::mojom::blink::LoadTimingInfoPtr load_timing_info,
      net::HttpConnectionInfo connection_info,
      const WTF::String& alpn_negotiated_protocol,
      bool is_secure_transport,
      bool is_validated,
      const WTF::String& normalized_server_timing,
      const ::network::URLLoaderCompletionStatus& completion_status) final;
  void GetScrollPosition(GetScrollPositionCallback callback) final;

  // blink::mojom::LocalMainFrame overrides:
  void AnimateDoubleTapZoom(const gfx::Point& point,
                            const gfx::Rect& rect) override;
  void SetScaleFactor(float scale) override;
  void ClosePage(
      mojom::blink::LocalMainFrame::ClosePageCallback callback) override;
  void GetFullPageSize(
      mojom::blink::LocalMainFrame::GetFullPageSizeCallback callback) override;
  void PluginActionAt(const gfx::Point& location,
                      mojom::blink::PluginActionType action) override;
  void SetInitialFocus(bool reverse) override;
  void EnablePreferredSizeChangedMode() override;
  void ZoomToFindInPageRect(const gfx::Rect& rect_in_root_frame) override;
  void InstallCoopAccessMonitor(
      const FrameToken& accessed_window,
      network::mojom::blink::CrossOriginOpenerPolicyReporterParamsPtr
          coop_reporter_params,
      bool is_in_same_virtual_coop_related_group) final;
  void UpdateBrowserControlsState(
      cc::BrowserControlsState constraints,
      cc::BrowserControlsState current,
      bool animate,
      const std::optional<cc::BrowserControlsOffsetTagModifications>&
          offset_tag_modifications) override;
  void Discard() final;
  void FinalizeNavigationConfidence(
      double randomized_trigger_rate,
      mojom::blink::ConfidenceLevel confidence) final;
  void SetV8CompileHints(base::ReadOnlySharedMemoryRegion data) override;

  // mojom::FullscreenVideoElementHandler implementation:
  void RequestFullscreenVideoElement() final;

  // DevicePostureClient implementation:
  void OnPostureChanged(mojom::blink::DevicePostureType posture) final;

  Member<blink::LocalFrame> frame_;

  HeapMojoAssociatedRemote<mojom::blink::BackForwardCacheControllerHost>
      back_forward_cache_controller_host_remote_{nullptr};

#if BUILDFLAG(IS_MAC)
  HeapMojoRemote<mojom::blink::TextInputHost> text_input_host_{nullptr};
#endif

  HeapMojoRemote<mojom::blink::ReportingServiceProxy> reporting_service_{
      nullptr};

  // Device posture fields should only be used, e.g. non-null, on local roots.
  HeapMojoRemote<mojom::blink::DevicePostureProvider>
      device_posture_provider_service_{nullptr};
  // LocalFrameMojoHandler can be reused by multiple ExecutionContext.
  HeapMojoReceiver<mojom::blink::DevicePostureClient, LocalFrameMojoHandler>
      device_posture_receiver_{this, nullptr};
  mojom::blink::DevicePostureType current_device_posture_ =
      mojom::blink::DevicePostureType::kContinuous;

  HeapMojoAssociatedRemote<mojom::blink::LocalFrameHost>
      local_frame_host_remote_{nullptr};

  HeapMojoRemote<mojom::blink::NonAssociatedLocalFrameHost>
      non_associated_local_frame_host_remote_{nullptr};

  // LocalFrameMojoHandler can be reused by multiple ExecutionContext.
  HeapMojoAssociatedReceiver<mojom::blink::LocalFrame, LocalFrameMojoHandler>
      local_frame_receiver_{this, nullptr};
  // LocalFrameMojoHandler can be reused by multiple ExecutionContext.
  HeapMojoAssociatedReceiver<mojom::blink::LocalMainFrame,
                             LocalFrameMojoHandler>
      main_frame_receiver_{this, nullptr};
  // LocalFrameMojoHandler can be reused by multiple ExecutionContext.
  HeapMojoAssociatedReceiver<mojom::blink::FullscreenVideoElementHandler,
                             LocalFrameMojoHandler>
      fullscreen_video_receiver_{this, nullptr};
};

class ActiveURLMessageFilter : public mojo::MessageFilter {
 public:
  explicit ActiveURLMessageFilter(LocalFrame* local_frame)
      : local_frame_(local_frame) {}

  ~ActiveURLMessageFilter() override;

  // mojo::MessageFilter overrides.
  bool WillDispatch(mojo::Message* message) override;
  void DidDispatchOrReject(mojo::Message* message, bool accepted) override;

 private:
  WeakPersistent<LocalFrame> local_frame_;
  bool debug_url_set_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LOCAL_FRAME_MOJO_HANDLER_H_
