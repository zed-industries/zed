// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_LOCAL_FRAME_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_LOCAL_FRAME_HOST_H_

#include "mojo/public/cpp/bindings/associated_receiver_set.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "third_party/blink/public/common/associated_interfaces/associated_interface_provider.h"
#include "third_party/blink/public/common/context_menu_data/untrustworthy_context_menu_params.h"
#include "third_party/blink/public/mojom/context_menu/context_menu.mojom-blink.h"
#include "third_party/blink/public/mojom/favicon/favicon_url.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/policy_container.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/remote_frame.mojom-blink.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink.h"
#include "ui/base/ime/mojom/virtual_keyboard_types.mojom-blink.h"

namespace blink {

// This class implements a LocalFrameHost that can be attached to the
// AssociatedInterfaceProvider so that it will be called when the renderer
// normally sends a request to the browser process. But for a unittest
// setup it can be intercepted by this class.
class FakeLocalFrameHost : public mojom::blink::LocalFrameHost {
 public:
  FakeLocalFrameHost() = default;

  void Init(blink::AssociatedInterfaceProvider* provider);
  void EnterFullscreen(mojom::blink::FullscreenOptionsPtr options,
                       EnterFullscreenCallback callback) override;
  void ExitFullscreen() override;
  void FullscreenStateChanged(
      bool is_fullscreen,
      mojom::blink::FullscreenOptionsPtr options) override;
  void RegisterProtocolHandler(const WTF::String& scheme,
                               const ::blink::KURL& url,
                               bool user_gesture) override;
  void UnregisterProtocolHandler(const WTF::String& scheme,
                                 const ::blink::KURL& url,
                                 bool user_gesture) override;
  void DidDisplayInsecureContent() override;
  void DidContainInsecureFormAction() override;
  void MainDocumentElementAvailable(bool uses_temporary_zoom_level) override;
  void SetNeedsOcclusionTracking(bool needs_tracking) override;
  void SetVirtualKeyboardMode(
      ui::mojom::blink::VirtualKeyboardMode mode) override;
  void VisibilityChanged(mojom::blink::FrameVisibility visibility) override;
  void DidChangeThemeColor(std::optional<::SkColor> theme_color) override;
  void DidChangeBackgroundColor(const SkColor4f& background_color,
                                bool color_adjust) override;
  void DidFailLoadWithError(const ::blink::KURL& url,
                            int32_t error_code) override;
  void DidFocusFrame() override;
  void DidCallFocus() override;
  void EnforceInsecureRequestPolicy(
      mojom::InsecureRequestPolicy policy_bitmap) override;
  void EnforceInsecureNavigationsSet(const WTF::Vector<uint32_t>& set) override;
  void SuddenTerminationDisablerChanged(
      bool present,
      blink::mojom::SuddenTerminationDisablerType disabler_type) override;
  void HadStickyUserActivationBeforeNavigationChanged(bool value) override;
  void ScrollRectToVisibleInParentFrame(
      const gfx::RectF& rect_to_scroll,
      blink::mojom::blink::ScrollIntoViewParamsPtr params) override;
  void BubbleLogicalScrollInParentFrame(
      blink::mojom::blink::ScrollDirection direction,
      ui::ScrollGranularity granularity) override;
  void StartLoadingForAsyncNavigationApiCommit() override {}
  void DidBlockNavigation(const KURL& blocked_url,
                          mojom::NavigationBlockedReason reason) override;
  void DidChangeLoadProgress(double load_progress) override;
  void DidFinishLoad(const KURL& validated_url) override;
  void DispatchLoad() override;
  void GoToEntryAtOffset(
      int32_t offset,
      bool has_user_gesture,
      std::optional<blink::scheduler::TaskAttributionId>) override;
  void NavigateToNavigationApiKey(
      const WTF::String& key,
      bool has_user_gesture,
      std::optional<blink::scheduler::TaskAttributionId> task_id) override {}
  void NavigateEventHandlerPresenceChanged(bool present) override {}
  void UpdateTitle(const WTF::String& title,
                   base::i18n::TextDirection title_direction) override;
  void UpdateApplicationTitle(const WTF::String& application_title) override;
  void UpdateUserActivationState(
      mojom::blink::UserActivationUpdateType update_type,
      mojom::UserActivationNotificationType notification_type) override;
  void DidConsumeHistoryUserActivation() override {}
  void HandleAccessibilityFindInPageResult(
      mojom::blink::FindInPageResultAXParamsPtr params) override;
  void HandleAccessibilityFindInPageTermination() override;
  void DocumentOnLoadCompleted() override;
  void ForwardResourceTimingToParent(
      mojom::blink::ResourceTimingInfoPtr timing) override;
  void DidDispatchDOMContentLoadedEvent() override;
  void RunModalAlertDialog(const WTF::String& alert_message,
                           bool disable_third_party_subframe_suppresion,
                           RunModalAlertDialogCallback callback) override;
  void RunModalConfirmDialog(const WTF::String& alert_message,
                             bool disable_third_party_subframe_suppresion,
                             RunModalConfirmDialogCallback callback) override;
  void RunModalPromptDialog(const WTF::String& alert_message,
                            const WTF::String& default_value,
                            bool disable_third_party_subframe_suppresion,
                            RunModalPromptDialogCallback callback) override;
  void RunBeforeUnloadConfirm(bool is_reload,
                              RunBeforeUnloadConfirmCallback callback) override;
  void UpdateFaviconURL(
      WTF::Vector<blink::mojom::blink::FaviconURLPtr> favicon_urls) override;
  void DownloadURL(mojom::blink::DownloadURLParamsPtr params) override;
  void FocusedElementChanged(bool is_editable_element,
                             bool is_richly_editable_element,
                             const gfx::Rect& bounds_in_frame_widget,
                             blink::mojom::FocusType focus_type) override;
  void TextSelectionChanged(const WTF::String& text,
                            uint32_t offset,
                            const gfx::Range& range) override;
  void ShowPopupMenu(
      mojo::PendingRemote<mojom::blink::PopupMenuClient> popup_client,
      const gfx::Rect& bounds,
      double font_size,
      int32_t selected_item,
      Vector<mojom::blink::MenuItemPtr> menu_items,
      bool right_aligned,
      bool allow_multiple_selection) override;
  void CreateNewPopupWidget(
      mojo::PendingAssociatedReceiver<mojom::blink::PopupWidgetHost>
          popup_widget_host,
      mojo::PendingAssociatedReceiver<mojom::blink::WidgetHost> widget_host,
      mojo::PendingAssociatedRemote<mojom::blink::Widget> widget) override;
  void ShowContextMenu(
      mojo::PendingAssociatedRemote<mojom::blink::ContextMenuClient>
          context_menu_client,
      const blink::UntrustworthyContextMenuParams& params) override;
  void DidLoadResourceFromMemoryCache(
      const KURL& url,
      const WTF::String& http_method,
      const WTF::String& mime_type,
      network::mojom::blink::RequestDestination request_destination,
      bool include_credentials) override;
  void DidChangeFrameOwnerProperties(
      const blink::FrameToken& child_frame_token,
      mojom::blink::FrameOwnerPropertiesPtr frame_owner_properties) override;
  void DidChangeOpener(
      const std::optional<LocalFrameToken>& opener_frame) override;
  void DidChangeIframeAttributes(const blink::FrameToken& child_frame_token,
                                 mojom::blink::IframeAttributesPtr) override;
  void DidChangeFramePolicy(const blink::FrameToken& child_frame_token,
                            const FramePolicy& frame_policy) override;
  void CapturePaintPreviewOfSubframe(
      const gfx::Rect& clip_rect,
      const base::UnguessableToken& guid) override;
  void SetCloseListener(
      mojo::PendingRemote<mojom::blink::CloseListener>) override;
  void Detach() override;
  void GetKeepAliveHandleFactory(
      mojo::PendingReceiver<mojom::blink::KeepAliveHandleFactory> receiver)
      override;
  void DidAddMessageToConsole(
      mojom::blink::ConsoleMessageLevel log_level,
      const WTF::String& message,
      uint32_t line_no,
      const WTF::String& source_id,
      const WTF::String& untrusted_stack_trace) override;
  void FrameSizeChanged(const gfx::Size& frame_size) override;
  void DidInferColorScheme(
      blink::mojom::PreferredColorScheme preferred_color_scheme) override;
  void DidChangeSrcDoc(const blink::FrameToken& child_frame_token,
                       const WTF::String& srcdoc_value) override;
  void ReceivedDelegatedCapability(
      blink::mojom::DelegatedCapability delegated_capability) override;
  void SendFencedFrameReportingBeacon(
      const WTF::String& event_data,
      const WTF::String& event_type,
      const WTF::Vector<blink::FencedFrame::ReportingDestination>& destinations,
      bool cross_origin_exposed) override;
  void SendFencedFrameReportingBeaconToCustomURL(
      const blink::KURL& destination_url,
      bool cross_origin_exposed) override;
  void SetFencedFrameAutomaticBeaconReportEventData(
      blink::mojom::AutomaticBeaconType event_type,
      const WTF::String& event_data,
      const WTF::Vector<blink::FencedFrame::ReportingDestination>& destinations,
      bool once,
      bool cross_origin_exposed) override;
  void DisableUntrustedNetworkInFencedFrame(
      DisableUntrustedNetworkInFencedFrameCallback callback) override;
  void ExemptUrlFromNetworkRevocationForTesting(
      const blink::KURL& exempted_url,
      ExemptUrlFromNetworkRevocationForTestingCallback callback) override;
  void SendLegacyTechEvent(
      const WTF::String& type,
      mojom::blink::LegacyTechEventCodeLocationPtr code_location) override;
  void SendPrivateAggregationRequestsForFencedFrameEvent(
      const WTF::String& event_type) override;
  void CreateFencedFrame(
      mojo::PendingAssociatedReceiver<mojom::blink::FencedFrameOwnerHost>,
      mojom::blink::RemoteFrameInterfacesFromRendererPtr
          remote_frame_interfaces,
      const RemoteFrameToken& frame_token,
      const base::UnguessableToken& devtools_frame_token) override;
  void ForwardFencedFrameEventAndUserActivationToEmbedder(
      const WTF::String& event_type) override;
  void OnViewTransitionOptInChanged(
      mojom::blink::ViewTransitionSameOriginOptIn) override {}
  void StartDragging(const blink::WebDragData& drag_data,
                     blink::DragOperationsMask operations_allowed,
                     const SkBitmap& bitmap,
                     const gfx::Vector2d& cursor_offset_in_dip,
                     const gfx::Rect& drag_obj_rect_in_dip,
                     mojom::blink::DragEventSourceInfoPtr event_info) override;
  void IssueKeepAliveHandle(
      mojo::PendingReceiver<mojom::blink::NavigationStateKeepAliveHandle>
          receiver) override;
  void NotifyStorageAccessed(blink::mojom::StorageTypeAccessed storageType,
                             bool blocked) override;
  void RecordWindowProxyUsageMetrics(
      const blink::FrameToken& target_frame_token,
      blink::mojom::WindowProxyAccessType access_type) override;
  void NotifyDocumentInteractive() override;

 private:
  void BindFrameHostReceiver(mojo::ScopedInterfaceEndpointHandle handle);

  mojo::AssociatedReceiver<mojom::blink::LocalFrameHost> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_LOCAL_FRAME_HOST_H_
