/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_TEST_HELPERS_H_

#include <gmock/gmock.h>
#include <gtest/gtest.h>

#include <memory>
#include <string>

#include "base/functional/callback_helpers.h"
#include "base/functional/function_ref.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "cc/test/fake_layer_tree_frame_sink.h"
#include "cc/trees/layer_tree_host.h"
#include "components/viz/common/surfaces/frame_sink_id.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/common/input/web_mouse_event.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/tree_scope_type.mojom-blink.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"
#include "third_party/blink/public/mojom/page/widget.mojom-blink.h"
#include "third_party/blink/public/mojom/widget/platform_widget.mojom-blink.h"
#include "third_party/blink/public/platform/platform.h"
#include "third_party/blink/public/platform/scheduler/web_agent_group_scheduler.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/public/web/web_frame_owner_properties.h"
#include "third_party/blink/public/web/web_history_item.h"
#include "third_party/blink/public/web/web_local_frame_client.h"
#include "third_party/blink/public/web/web_settings.h"
#include "third_party/blink/public/web/web_view_client.h"
#include "third_party/blink/renderer/core/exported/web_view_impl.h"  // IWYU pragma: export
#include "third_party/blink/renderer/core/frame/settings.h"
#include "third_party/blink/renderer/core/frame/web_frame_widget_impl.h"
#include "third_party/blink/renderer/core/scroll/scrollbar_theme.h"
#include "third_party/blink/renderer/core/testing/scoped_mock_overlay_scrollbars.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/display/screen_info.h"

namespace base {
class TickClock;
}

namespace blink {
class AgentGroupScheduler;
class WebFrame;
class WebLocalFrameImpl;
struct WebNavigationParams;
class WebRemoteFrameImpl;
class WebSettings;
class WidgetInputHandlerManager;

namespace frame_test_helpers {
class TestWebFrameClient;
class TestWidgetInputHandlerHost;
class WebViewHelper;

cc::LayerTreeSettings GetSynchronousSingleThreadLayerTreeSettings();

// Loads a url into the specified WebLocalFrame for testing purposes.
void LoadFrameDontWait(WebLocalFrame*, const WebURL& url);
// Same as above, but also pumps any pending resource requests,
// as well as waiting for the threaded parser to finish, before returning.
void LoadFrame(WebLocalFrame*, const std::string& url);
// Same as above, but for WebLocalFrame::LoadHTMLString().
void LoadHTMLString(WebLocalFrame*,
                    const std::string& html,
                    const WebURL& base_url,
                    const base::TickClock* clock = nullptr);
// Same as above, but for WebLocalFrame::RequestFromHistoryItem/Load.
void LoadHistoryItem(WebLocalFrame*,
                     const WebHistoryItem&,
                     mojom::FetchCacheMode);
// Same as above, but for WebLocalFrame::Reload().
void ReloadFrame(WebLocalFrame*);
void ReloadFrameBypassingCache(WebLocalFrame*);

// Fills navigation params if needed. Params should have the proper url set up.
void FillNavigationParamsResponse(WebNavigationParams*);

// Pumps pending resource requests while waiting for a frame to load. Consider
// using one of the above helper methods whenever possible.
void PumpPendingRequestsForFrameToLoad(WebLocalFrame*);

WebMouseEvent CreateMouseEvent(WebInputEvent::Type,
                               WebMouseEvent::Button,
                               const gfx::Point&,
                               int modifiers);

// Helpers for creating frames for test purposes. All methods that accept raw
// pointer client arguments allow nullptr as a valid argument; if a client
// pointer is null, the test framework will automatically create and manage the
// lifetime of that client interface. Otherwise, the caller is responsible for
// ensuring that non-null clients outlive the created frame.

// Helper for creating a local child frame of a local parent frame.
WebLocalFrameImpl* CreateLocalChild(
    WebLocalFrame& parent,
    mojom::blink::TreeScopeType,
    TestWebFrameClient*,
    WebPolicyContainerBindParams policy_container_bind_params,
    WebLocalFrameClient::FinishChildFrameCreationFn finish_creation);

// Similar, but unlike the overload which takes the client as a raw pointer,
// ownership of the TestWebFrameClient is transferred to the test framework.
// TestWebFrameClient may not be null.
WebLocalFrameImpl* CreateLocalChild(
    WebLocalFrame& parent,
    blink::mojom::blink::TreeScopeType,
    std::unique_ptr<TestWebFrameClient>,
    WebPolicyContainerBindParams policy_container_bind_params,
    WebLocalFrameClient::FinishChildFrameCreationFn finish_creation);

// Helper for creating a remote frame. Generally used when creating a remote
// frame to swap into the frame tree.
// TODO(dcheng): Consider allowing security origin to be passed here once the
// frame tree moves back to core.
WebRemoteFrameImpl* CreateRemote();

// Helper for creating a remote child frame of a remote parent frame.
WebRemoteFrameImpl* CreateRemoteChild(WebRemoteFrame& parent,
                                      const WebString& name = WebString(),
                                      scoped_refptr<SecurityOrigin> = nullptr);

// Call Swap with a `new_remote_frame` stubbing out the mojo channels if
// necessary.
void SwapRemoteFrame(
    WebFrame* old_frame,
    WebRemoteFrame* new_remote_frame,
    mojo::PendingAssociatedRemote<mojom::blink::RemoteFrameHost> =
        mojo::NullAssociatedRemote());

class TestWebFrameWidgetHost : public mojom::blink::WidgetHost,
                               public mojom::blink::FrameWidgetHost {
 public:
  size_t CursorSetCount() const { return cursor_set_count_; }
  size_t VirtualKeyboardRequestCount() const {
    return virtual_keyboard_request_count_;
  }

  // mojom::blink::WidgetHost overrides:
  void SetCursor(const ui::Cursor& cursor) override;
  void UpdateTooltipUnderCursor(
      const String& tooltip_text,
      base::i18n::TextDirection text_direction_hint) override;
  void UpdateTooltipFromKeyboard(const String& tooltip_text,
                                 base::i18n::TextDirection text_direction_hint,
                                 const gfx::Rect& bounds) override;
  void ClearKeyboardTriggeredTooltip() override;
  void TextInputStateChanged(
      ui::mojom::blink::TextInputStatePtr state) override;
  void SelectionBoundsChanged(const gfx::Rect& anchor_rect,
                              base::i18n::TextDirection anchor_dir,
                              const gfx::Rect& focus_rect,
                              base::i18n::TextDirection focus_dir,
                              const gfx::Rect& bounding_box,
                              bool is_anchor_first) override;
  void CreateFrameSink(
      mojo::PendingReceiver<viz::mojom::blink::CompositorFrameSink>
          compositor_frame_sink_receiver,
      mojo::PendingRemote<viz::mojom::blink::CompositorFrameSinkClient>
          compositor_frame_sink_client,
      mojo::PendingRemote<blink::mojom::blink::RenderInputRouterClient>
          viz_rir_client_remote) override;
  void RegisterRenderFrameMetadataObserver(
      mojo::PendingReceiver<cc::mojom::blink::RenderFrameMetadataObserverClient>
          render_frame_metadata_observer_client_receiver,
      mojo::PendingRemote<cc::mojom::blink::RenderFrameMetadataObserver>
          render_frame_metadata_observer) override;

  // blink::mojom::FrameWidgetHost overrides.
  void AnimateDoubleTapZoomInMainFrame(const gfx::Point& tap_point,
                                       const gfx::Rect& rect_to_zoom) override;
  void ZoomToFindInPageRectInMainFrame(const gfx::Rect& rect_to_zoom) override;
  void SetHasTouchEventConsumers(
      mojom::blink::TouchEventConsumersPtr consumers) override;
  void IntrinsicSizingInfoChanged(
      mojom::blink::IntrinsicSizingInfoPtr sizing_info) override;
  void AutoscrollStart(const gfx::PointF& position) override;
  void AutoscrollFling(const gfx::Vector2dF& position) override;
  void AutoscrollEnd() override;

  void BindWidgetHost(
      mojo::PendingAssociatedReceiver<mojom::blink::WidgetHost>,
      mojo::PendingAssociatedReceiver<mojom::blink::FrameWidgetHost>);
  void BindRenderInputRouterInterfaces(
      mojo::PendingRemote<mojom::blink::RenderInputRouterClient> remote);
  void GetWidgetInputHandler(
      mojo::PendingReceiver<mojom::blink::WidgetInputHandler> request,
      mojo::PendingRemote<mojom::blink::WidgetInputHandlerHost> host);

 private:
  size_t cursor_set_count_ = 0;
  size_t virtual_keyboard_request_count_ = 0;
  mojo::Remote<mojom::blink::RenderInputRouterClient> client_remote_;
  mojo::AssociatedReceiver<mojom::blink::WidgetHost> receiver_{this};
  mojo::AssociatedReceiver<mojom::blink::FrameWidgetHost> frame_receiver_{this};
};

class TestWebFrameWidget : public WebFrameWidgetImpl {
 public:
  template <typename... Args>
  explicit TestWebFrameWidget(Args&&... args)
      : WebFrameWidgetImpl(std::forward<Args>(args)...) {}
  ~TestWebFrameWidget() override = default;

  TestWebFrameWidgetHost& WidgetHost() { return *widget_host_; }

  bool HaveScrollEventHandlers() const;
  const Vector<std::unique_ptr<blink::WebCoalescedInputEvent>>&
  GetInjectedScrollEvents() const {
    return injected_scroll_events_;
  }

  // The returned pointer is valid after AllocateNewLayerTreeFrameSink() occurs,
  // until another call to AllocateNewLayerTreeFrameSink() happens. This
  // pointer is valid to use from the main thread for tests that use a single
  // threaded compositor, such as SimCompositor tests.
  cc::FakeLayerTreeFrameSink* LastCreatedFrameSink();

  virtual display::ScreenInfo GetInitialScreenInfo();
  void SetInitialScreenInfo(const display::ScreenInfo&);

  virtual std::unique_ptr<TestWebFrameWidgetHost> CreateWidgetHost();

  void BindWidgetChannels(
      mojo::AssociatedRemote<mojom::blink::Widget>,
      mojo::PendingAssociatedReceiver<mojom::blink::WidgetHost>,
      mojo::PendingAssociatedReceiver<mojom::blink::FrameWidgetHost>);

  WidgetInputHandlerManager* GetWidgetInputHandlerManager() const;
  void FlushInputHandlerTasks();

  // Simulates an input event arriving at the WidgetInputHandlerManager from the
  // browser process.  The event will run synchronously through the compositor's
  // real input handling code (InputHandlerProxy and ThreadedInputHandler).
  //
  // Note that with scroll unification, tests should send gesture scroll events
  // using this method, and not through WebFrameWidgetImpl::HandleInputEvent or
  // EventHandler::HandleGestureEvent.  Tests that use this method for scrolling
  // should also use SimTest::ResizeView or WebViewHelper::Resize (not directly
  // WebFrameWidgetImpl::Resize) to set the initial size of the viewport.
  //
  void DispatchThroughCcInputHandler(const WebInputEvent& event);
  const mojom::blink::DidOverscrollParamsPtr& last_overscroll() const {
    return last_overscroll_;
  }

  void RequestDecode(const cc::DrawImage&,
                     base::OnceCallback<void(bool)>) override;

  using WebFrameWidgetImpl::GetOriginalScreenInfo;

 protected:
  // Allow subclasses to provide their own input handler host.
  virtual TestWidgetInputHandlerHost* GetInputHandlerHost();

  // WidgetBaseClient overrides.
  std::unique_ptr<cc::LayerTreeFrameSink> AllocateNewLayerTreeFrameSink()
      override;
  void WillQueueSyntheticEvent(const WebCoalescedInputEvent& event) override;
  bool ShouldAutoDetermineCompositingToLCDTextSetting() override {
    return false;
  }
  bool AllowsScrollResampling() override { return false; }

 private:
  cc::FakeLayerTreeFrameSink* last_created_frame_sink_ = nullptr;
  Vector<std::unique_ptr<blink::WebCoalescedInputEvent>>
      injected_scroll_events_;
  std::unique_ptr<TestWidgetInputHandlerHost> widget_input_handler_host_;
  viz::FrameSinkId frame_sink_id_;
  std::unique_ptr<TestWebFrameWidgetHost> widget_host_;
  mojom::blink::DidOverscrollParamsPtr last_overscroll_;
  display::ScreenInfo initial_screen_info_;
};

using CreateTestWebFrameWidgetCallback =
    base::RepeatingCallback<TestWebFrameWidget*(
        base::PassKey<WebLocalFrame>,
        CrossVariantMojoAssociatedRemote<mojom::FrameWidgetHostInterfaceBase>
            frame_widget_host,
        CrossVariantMojoAssociatedReceiver<mojom::FrameWidgetInterfaceBase>
            frame_widget,
        CrossVariantMojoAssociatedRemote<mojom::WidgetHostInterfaceBase>
            widget_host,
        CrossVariantMojoAssociatedReceiver<mojom::WidgetInterfaceBase> widget,
        scoped_refptr<base::SingleThreadTaskRunner> task_runner,
        const viz::FrameSinkId& frame_sink_id,
        bool hidden,
        bool never_composited,
        bool is_for_child_local_root,
        bool is_for_nested_main_frame,
        bool is_for_scalable_page)>;

// Convenience class for handling the lifetime of a WebView and its associated
// mainframe in tests.
class WebViewHelper : public ScopedMockOverlayScrollbars {
  USING_FAST_MALLOC(WebViewHelper);

 public:
  explicit WebViewHelper(CreateTestWebFrameWidgetCallback
                             create_web_frame_callback = base::NullCallback());
  WebViewHelper(const WebViewHelper&) = delete;
  WebViewHelper& operator=(const WebViewHelper&) = delete;
  ~WebViewHelper();

  // Helpers for creating the main frame. All methods that accept raw
  // pointer client arguments allow nullptr as a valid argument; if a client
  // pointer is null, the test framework will automatically create and manage
  // the lifetime of that client interface. Otherwise, the caller is responsible
  // for ensuring that non-null clients outlive the created frame.

  // Creates and initializes the WebView with a main WebLocalFrame.
  WebViewImpl* InitializeWithOpener(
      WebFrame* opener,
      TestWebFrameClient* = nullptr,
      WebViewClient* = nullptr,
      void (*update_settings_func)(WebSettings*) = nullptr,
      std::optional<blink::FencedFrame::DeprecatedFencedFrameMode>
          fenced_frame_mode = std::nullopt,
      bool is_prerendering = false);

  // Same as InitializeWithOpener(), but always sets the opener to null.
  WebViewImpl* Initialize(TestWebFrameClient* = nullptr,
                          WebViewClient* = nullptr,
                          void (*update_settings_func)(WebSettings*) = nullptr);

  // Same as InitializeWithOpener(), but passes null for everything but the
  // settings function.
  WebViewImpl* InitializeWithSettings(
      void (*update_settings_func)(WebSettings*));

  WebViewImpl* InitializeWithAndroidSettings() {
    return InitializeWithSettings(&UpdateAndroidCompositingSettings);
  }
  static void UpdateAndroidCompositingSettings(WebSettings*);

  // Same as Initialize() but also performs the initial load of the url. Only
  // returns once the load is complete.
  WebViewImpl* InitializeAndLoad(
      const std::string& url,
      TestWebFrameClient* = nullptr,
      WebViewClient* = nullptr,
      void (*update_settings_func)(WebSettings*) = nullptr);

  // Creates and initializes the WebView with a main WebRemoteFrame, but doesn't
  // bind the RemoteFrame to anything, to simulate the "placeholder RemoteFrame"
  // case, where a RemoteFrame is created explicitly without a browser-side
  // counterpart. See the comments in `AgentSchedulingGroup::CreateWebView()`
  // for more details.
  WebViewImpl* InitializePlaceholderRemote();

  // Same as InitializeRemoteWithOpener(), but always sets the opener to null.
  WebViewImpl* InitializeRemote(scoped_refptr<SecurityOrigin> = nullptr,
                                WebViewClient* = nullptr);

  // Same as InitializeRemoteWithOpenerAndAssociatedRemoteAndReceivers(), but
  // always sets the opener to null and sets the associated remote & receivers
  // to a stub implementation (instead of not binding anything like
  // `InitializePlaceholderRemote()`).
  WebViewImpl* InitializeRemoteWithOpener(
      WebFrame* opener,
      scoped_refptr<SecurityOrigin> = nullptr,
      WebViewClient* = nullptr);

  // Creates and initializes the WebView with a main WebRemoteFrame. Passing
  // nullptr as the SecurityOrigin results in a frame with a unique security
  // origin.
  WebViewImpl* InitializeRemoteWithOpenerAndAssociatedRemoteAndReceivers(
      WebFrame* opener,
      scoped_refptr<SecurityOrigin>,
      WebViewClient*,
      mojo::PendingAssociatedRemote<mojom::blink::RemoteFrameHost>
          remote_frame_host,
      mojo::PendingAssociatedReceiver<mojom::blink::RemoteFrame> receiver);

  // Creates a new uninitialized WebView.
  WebViewImpl* CreateWebView(WebViewClient*, bool compositing_enabled);

  // Helper for creating a local child frame of a remote parent frame.
  WebLocalFrameImpl* CreateLocalChild(
      WebRemoteFrame& parent,
      const WebString& name = WebString(),
      const WebFrameOwnerProperties& = WebFrameOwnerProperties(),
      WebFrame* previous_sibling = nullptr,
      TestWebFrameClient* = nullptr);

  // Helper for creating a provisional local frame that can replace a local or
  // remote frame.
  WebLocalFrameImpl* CreateProvisional(WebFrame& old_frame,
                                       TestWebFrameClient* = nullptr);

  // Creates a frame widget but does not initialize compositing.
  TestWebFrameWidget* CreateFrameWidget(WebLocalFrame* frame);

  // Creates a frame widget and initializes compositing.
  TestWebFrameWidget* CreateFrameWidgetAndInitializeCompositing(
      WebLocalFrame* frame);

  // Load the 'Ahem' font to this WebView.
  // The 'Ahem' font is the only font whose font metrics is consistent across
  // platforms, but it's not guaranteed to be available.
  // See external/wpt/css/fonts/ahem/README for more about the 'Ahem' font.
  void LoadAhem();

  void Resize(const gfx::Size&);

  void Reset();

  WebViewImpl* GetWebView() const { return web_view_; }
  cc::LayerTreeHost* GetLayerTreeHost() const;
  WebLocalFrameImpl* LocalMainFrame() const;
  WebRemoteFrameImpl* RemoteMainFrame() const;
  TestWebFrameWidget* GetMainFrameWidget() const;
  WidgetInputHandlerManager* GetWidgetInputHandlerManager() const;

  void set_viewport_enabled(bool viewport) {
    DCHECK(!web_view_)
        << "set_viewport_enabled() should be called before Initialize.";
    viewport_enabled_ = viewport;
  }

  template <class C = TestWebFrameWidget>
  static TestWebFrameWidget* CreateTestWebFrameWidget(
      base::PassKey<WebLocalFrame> pass_key,
      CrossVariantMojoAssociatedRemote<
          mojom::blink::FrameWidgetHostInterfaceBase> frame_widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::FrameWidgetInterfaceBase>
          frame_widget,
      CrossVariantMojoAssociatedRemote<mojom::blink::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::WidgetInterfaceBase>
          widget,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      const viz::FrameSinkId& frame_sink_id,
      bool hidden,
      bool never_composited,
      bool is_for_child_local_root,
      bool is_for_nested_main_frame,
      bool is_for_scalable_page) {
    return MakeGarbageCollected<C>(
        std::move(pass_key), std::move(frame_widget_host),
        std::move(frame_widget), std::move(widget_host), std::move(widget),
        std::move(task_runner), frame_sink_id, hidden, never_composited,
        is_for_child_local_root, is_for_nested_main_frame,
        is_for_scalable_page);
  }

  AgentGroupScheduler& GetAgentGroupScheduler() {
    return agent_group_scheduler_->GetAgentGroupScheduler();
  }

 private:
  void InitializeWebView(
      WebViewClient*,
      class WebView* opener,
      std::optional<blink::FencedFrame::DeprecatedFencedFrameMode>
          fenced_frame_mode,
      bool is_prerendering);
  void CheckFrameIsAssociatedWithWebView(WebFrame* frame);

  bool viewport_enabled_ = false;

  WebViewImpl* web_view_;

  std::unique_ptr<WebViewClient> owned_web_view_client_;

  std::unique_ptr<blink::scheduler::WebAgentGroupScheduler>
      agent_group_scheduler_;
  CreateWebFrameWidgetCallback create_widget_callback_wrapper_;

  // The Platform should not change during the lifetime of the test!
  Platform* const platform_;
};

// Minimal implementation of WebLocalFrameClient needed for unit tests that load
// frames. Tests that load frames and need further specialization of
// WebLocalFrameClient behavior should subclass this.
class TestWebFrameClient : public WebLocalFrameClient {
 public:
  TestWebFrameClient();
  ~TestWebFrameClient() override;

  static bool IsLoading() { return loads_in_progress_ > 0; }
  Vector<String>& ConsoleMessages() { return console_messages_; }

  WebNavigationControl* Frame() const { return frame_; }
  // Pass ownership of the TestWebFrameClient to |self_owned| here if the
  // TestWebFrameClient should delete itself on frame detach.
  void Bind(WebLocalFrame*,
            std::unique_ptr<TestWebFrameClient> self_owned = nullptr);

  // WebLocalFrameClient:
  void FrameDetached(DetachReason detach_reason) override;
  WebLocalFrame* CreateChildFrame(
      blink::mojom::blink::TreeScopeType,
      const WebString& name,
      const WebString& fallback_name,
      const FramePolicy&,
      const WebFrameOwnerProperties&,
      FrameOwnerElementType,
      WebPolicyContainerBindParams policy_container_bind_params,
      ukm::SourceId document_ukm_source_id,
      FinishChildFrameCreationFn finish_creation) override;
  void DidStartLoading() override;
  void DidStopLoading() override;
  bool SwapIn(WebFrame* previous_frame) override;
  std::unique_ptr<URLLoader> CreateURLLoaderForTesting() override;
  void BeginNavigation(std::unique_ptr<WebNavigationInfo> info) override;
  WebEffectiveConnectionType GetEffectiveConnectionType() override;
  void SetEffectiveConnectionTypeForTesting(
      WebEffectiveConnectionType) override;
  void DidAddMessageToConsole(const WebConsoleMessage&,
                              const WebString& source_name,
                              unsigned source_line,
                              const WebString& stack_trace) override;
  WebPlugin* CreatePlugin(const WebPluginParams& params) override;
  AssociatedInterfaceProvider* GetRemoteNavigationAssociatedInterfaces()
      override;
  void DidMeaningfulLayout(WebMeaningfulLayout) override;
  WebView* CreateNewWindow(
      const WebURLRequest&,
      const WebWindowFeatures&,
      const WebString& name,
      const gfx::Rect& requested_screen_rect,
      WebNavigationPolicy,
      network::mojom::blink::WebSandboxFlags,
      const SessionStorageNamespaceId&,
      bool& consumed_user_gesture,
      const std::optional<Impression>&,
      const std::optional<WebPictureInPictureWindowOptions>&,
      const WebURL& base_url) override;

  int VisuallyNonEmptyLayoutCount() const {
    return visually_non_empty_layout_count_;
  }
  int FinishedParsingLayoutCount() const {
    return finished_parsing_layout_count_;
  }
  int FinishedLoadingLayoutCount() const {
    return finished_loading_layout_count_;
  }
  network::mojom::WebSandboxFlags sandbox_flags() const {
    return sandbox_flags_;
  }

  void DestroyChildViews();

  void SetFrameDetachedCallback(base::OnceClosure callback);

 private:
  void CommitNavigation(std::unique_ptr<WebNavigationInfo>);

  static int loads_in_progress_;

  // If set to a non-null value, self-deletes on frame detach.
  std::unique_ptr<TestWebFrameClient> self_owned_;

  std::unique_ptr<AssociatedInterfaceProvider> associated_interface_provider_;

  // This is null from when the client is created until it is initialized with
  // Bind().
  WebNavigationControl* frame_ = nullptr;

  base::CancelableOnceCallback<void()> navigation_callback_;
  WebEffectiveConnectionType effective_connection_type_;
  Vector<String> console_messages_;
  int visually_non_empty_layout_count_ = 0;
  int finished_parsing_layout_count_ = 0;
  int finished_loading_layout_count_ = 0;

  // The sandbox flags to use when committing navigations.
  network::mojom::WebSandboxFlags sandbox_flags_ =
      network::mojom::WebSandboxFlags::kNone;

  // Callback to run when |FrameDetached| is called.
  base::OnceClosure frame_detached_callback_ = base::DoNothing();

  WTF::Vector<std::unique_ptr<WebViewHelper>> child_web_views_;
  base::WeakPtrFactory<TestWebFrameClient> weak_factory_{this};
};

class TestWidgetInputHandlerHost : public mojom::blink::WidgetInputHandlerHost {
 public:
  mojo::PendingRemote<mojom::blink::WidgetInputHandlerHost> BindNewRemote();

  void SetTouchActionFromMain(cc::TouchAction touch_action) override;
  void SetPanAction(mojom::blink::PanAction pan_action) override;
  void DidOverscroll(mojom::blink::DidOverscrollParamsPtr params) override;
  void DidStartScrollingViewport() override;
  void ImeCancelComposition() override;
  void ImeCompositionRangeChanged(
      const gfx::Range& range,
      const std::optional<WTF::Vector<gfx::Rect>>& character_bounds) override;
  void SetMouseCapture(bool capture) override;
  void SetAutoscrollSelectionActiveInMainFrame(
      bool autoscroll_selection) override;
  void RequestMouseLock(bool from_user_gesture,
                        bool unadjusted_movement,
                        RequestMouseLockCallback callback) override;

 private:
  mojo::Receiver<mojom::blink::WidgetInputHandlerHost> receiver_{this};
};

}  // namespace frame_test_helpers
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_TEST_HELPERS_H_
