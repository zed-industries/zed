/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_PAGE_POPUP_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_PAGE_POPUP_IMPL_H_

#include "base/time/time.h"
#include "build/build_config.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "third_party/blink/public/mojom/input/pointer_lock_context.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/input/pointer_lock_result.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/page/widget.mojom-blink.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/web/web_hit_test_result.h"
#include "third_party/blink/public/web/web_page_popup.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/core/input/widget_event_handler.h"
#include "third_party/blink/renderer/core/page/page_popup.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/widget/widget_base_client.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace cc {
class Layer;
}

namespace blink {
class AgentGroupScheduler;
class Element;
class EmptyLocalFrameClient;
class Node;
class Page;
class PagePopupChromeClient;
class PagePopupClient;
class WebViewImpl;
class LocalDOMWindow;
class WidgetBase;
class DOMRect;

class CORE_EXPORT WebPagePopupImpl final : public WebPagePopup,
                                           public WidgetEventHandler,
                                           public PagePopup,
                                           public RefCounted<WebPagePopupImpl>,
                                           public WidgetBaseClient {
  USING_FAST_MALLOC(WebPagePopupImpl);

 public:
  WebPagePopupImpl(const WebPagePopupImpl&) = delete;
  WebPagePopupImpl& operator=(const WebPagePopupImpl&) = delete;
  ~WebPagePopupImpl() override;

  // Cancel informs the PopupClient that it should initiate shutdown of this
  // popup via ClosePopup(). It is called to indicate the popup was closed due
  // to a user gesture outside the popup or other such reasons, where a default
  // cancelled response can be made.
  //
  // When the user chooses a value in the popup and thus it is closed, or if the
  // origin in the DOM disppears, then the Cancel() step would be skipped and go
  // directly to ClosePopup().
  void Cancel();
  // Once ClosePopup() has been called, the WebPagePopupImpl should be disowned
  // by any clients, and will be reaped when then browser closes its
  // RenderWidget which closes this object. This will call back to the
  // PopupClient to say DidClosePopup(), and to the WebViewImpl to cleanup
  // its reference to the popup.
  //
  // Only HasSamePopupClient() may still be called after ClosePopup() runs.
  void ClosePopup();

  // Returns whether another WebPagePopupImpl has the same PopupClient as this
  // instance. May be called after ClosePopup() has run still, in order to
  // determine if a popup sharing the same client was created immediately after
  // closing one.
  bool HasSamePopupClient(WebPagePopupImpl* other) {
    return other && popup_client_ == other->popup_client_;
  }

  LocalDOMWindow* Window();

  // WebPagePopup implementation.
  WebDocument GetDocument() override;

  // PagePopup implementation.
  void PostMessageToPopup(const String& message) override;
  void Update() override;

  // WidgetEventHandler implementation.
  WebInputEventResult HandleKeyEvent(const WebKeyboardEvent&) override;

  // Return the LayerTreeHost backing this popup widget.
  cc::LayerTreeHost* LayerTreeHostForTesting();

  // Called when the browser has shown the popup.
  void DidShowPopup();

  static WebPagePopupImpl* Create(
      CrossVariantMojoAssociatedRemote<
          mojom::blink::PopupWidgetHostInterfaceBase> popup_widget_host,
      CrossVariantMojoAssociatedRemote<mojom::blink::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::WidgetInterfaceBase>
          widget,
      WebViewImpl* opener_impl,
      AgentGroupScheduler& agent_group_scheduler,
      const display::ScreenInfos& screen_infos,
      PagePopupClient*);

 private:
  // WidgetBaseClient overrides:
  void OnCommitRequested() override;
  void BeginMainFrame(const viz::BeginFrameArgs& args) override;
  void SetSuppressFrameRequestsWorkaroundFor704763Only(bool) final;
  WebInputEventResult DispatchBufferedTouchEvents() override;
  void WillHandleGestureEvent(const WebGestureEvent& event,
                              bool* suppress) override;
  void WillHandleMouseEvent(const WebMouseEvent& event) override;
  void ObserveGestureEventAndResult(
      const WebGestureEvent& gesture_event,
      const gfx::Vector2dF& unused_delta,
      const cc::OverscrollBehavior& overscroll_behavior,
      bool event_processed) override;
  bool SupportsBufferedTouchEvents() override { return true; }
  void FocusChanged(mojom::blink::FocusState focus_state) override;
  void ScheduleAnimation(bool urgent) override;
  void UpdateVisualProperties(
      const VisualProperties& visual_properties) override;
  gfx::Rect ViewportVisibleRect() override;
  void ScreenRectToEmulated(gfx::Rect& screen_rect) override;
  void EmulatedToScreenRect(gfx::Rect& screen_rect) override;
  KURL GetURLForDebugTrace() override;
  std::unique_ptr<cc::LayerTreeFrameSink> AllocateNewLayerTreeFrameSink()
      override;

  // WebWidget implementation.
  // NOTE: The WebWidget may still be used after requesting the popup to be
  // closed and destroyed. But the Page and the MainFrame are destroyed
  // immediately. So all methods (outside of initialization) that are part
  // of the WebWidget need to check if close has already been initiated (they
  // can do so by checking |page_|) and not crash! https://crbug.com/906340
  void SetCompositorVisible(bool visible) override;
  void WarmUpCompositor() override;
  void UpdateLifecycle(WebLifecycleUpdate requested_update,
                       DocumentUpdateReason reason) override;
  void Resize(const gfx::Size&) override;
  WebInputEventResult HandleInputEvent(const WebCoalescedInputEvent&) override;
  void SetFocus(bool) override;
  bool HasFocus() override;
  WebHitTestResult HitTestResultAt(const gfx::PointF&) override { return {}; }
  void InitializeCompositing(const display::ScreenInfos& screen_infos,
                             const cc::LayerTreeSettings* settings) override;
  void SetCursor(const ui::Cursor& cursor) override;
  bool HandlingInputEvent() override;
  void SetHandlingInputEvent(bool handling) override;
  void ProcessInputEventSynchronouslyForTesting(
      const WebCoalescedInputEvent&) override;
  void DispatchNonBlockingEventForTesting(
      std::unique_ptr<WebCoalescedInputEvent> event) override;
  void UpdateTextInputState() override;
  void UpdateSelectionBounds() override;
  void ShowVirtualKeyboard() override;
  void FlushInputProcessedCallback() override;
  void CancelCompositionForPepper() override;
  void ApplyVisualProperties(
      const VisualProperties& visual_properties) override;
  const display::ScreenInfo& GetScreenInfo() override;
  const display::ScreenInfos& GetScreenInfos() override;
  const display::ScreenInfo& GetOriginalScreenInfo() override;
  const display::ScreenInfos& GetOriginalScreenInfos() override;
  gfx::Rect WindowRect() override;
  gfx::Rect ViewRect() override;
  void SetScreenRects(const gfx::Rect& widget_screen_rect,
                      const gfx::Rect& window_screen_rect) override;
  gfx::Size VisibleViewportSize() override;
  bool IsHidden() const override;

  // WidgetEventHandler functions
  WebInputEventResult HandleCharEvent(const WebKeyboardEvent&) override;
  WebInputEventResult HandleGestureEvent(const WebGestureEvent&) override;
  void HandleMouseDown(LocalFrame& main_frame, const WebMouseEvent&) override;
  WebInputEventResult HandleMouseWheel(LocalFrame& main_frame,
                                       const WebMouseWheelEvent&) override;

  // This may only be called if page_ is non-null.
  LocalFrame& MainFrame() const;

  void Close();

  Element* FocusedElement() const;

  bool IsViewportPointInWindow(int x, int y);
  bool ShouldCheckPopupPositionForTelemetry() const;
  void CheckScreenPointInOwnerWindowAndCount(const gfx::PointF& point_in_screen,
                                             WebFeature feature) const;
  gfx::Rect OwnerWindowRectInScreen() const;
  // Returns anchor rect in screen coordinates for this popup.
  gfx::Rect GetAnchorRectInScreen() const;

  // PagePopup function
  AXObject* RootAXObject(Element* popover_owner) override;
  void SetWindowRect(const gfx::Rect&) override;

  WebPagePopupImpl(
      CrossVariantMojoAssociatedRemote<
          mojom::blink::PopupWidgetHostInterfaceBase> popup_widget_host,
      CrossVariantMojoAssociatedRemote<mojom::blink::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::WidgetInterfaceBase>
          widget,
      WebViewImpl* opener_impl,
      AgentGroupScheduler& agent_group_scheduler,
      const display::ScreenInfos& screen_infos,
      PagePopupClient*);

  void DestroyPage();
  void MainFrameDetached();
  void SetRootLayer(scoped_refptr<cc::Layer>);
  void SetWebView(WebViewImpl* web_view);

  gfx::Rect WindowRectInScreen() const;

  void InjectScrollbarGestureScroll(const gfx::Vector2dF& delta,
                                    ui::ScrollGranularity granularity,
                                    cc::ElementId scrollable_area_element_id,
                                    WebInputEvent::Type injected_type);

  void WidgetHostDisconnected();
  void DidSetBounds();

  // This is the WebView that opened the popup.
  WebViewImpl* opener_web_view_ = nullptr;
  Persistent<PagePopupChromeClient> chrome_client_;
  Persistent<EmptyLocalFrameClient> local_frame_client_;
  // WebPagePopupImpl wraps its own Page that renders the content in the popup.
  // This member is non-null between the call to Initialize() and the call to
  // ClosePopup(). If page_ is non-null, it is guaranteed to have an attached
  // main LocalFrame with a corresponding non-null LocalFrameView and non-null
  // Document.
  Persistent<Page> page_;
  PagePopupClient* popup_client_;
  bool closing_ = false;

  scoped_refptr<cc::Layer> root_layer_;
  base::TimeTicks raf_aligned_input_start_time_;

  bool suppress_next_keypress_event_ = false;
  Persistent<DOMRect> popup_owner_client_rect_;

  // When emulation is enabled, and a popup widget is opened, the popup widget
  // needs these values to move between the popup's (non-emulated) coordinates
  // and the opener widget's (emulated) coordinates. They are only valid when
  // the |opener_emulator_scale_| is non-zero.
  gfx::Point opener_widget_screen_origin_;
  gfx::Point opener_original_widget_screen_origin_;
  float opener_emulator_scale_ = 0;

  // The channel associated with the browser. When this is closed the popup will
  // be destroyed.
  mojo::AssociatedRemote<mojom::blink::PopupWidgetHost> popup_widget_host_;

  // The rect before the widget is shown.
  gfx::Rect initial_rect_;

  // Defer setting the window rect until the widget is shown.
  bool should_defer_setting_window_rect_ = true;

  // Base functionality all widgets have. This is a member as to avoid
  // complicated inheritance structures.
  std::unique_ptr<WidgetBase> widget_base_;

  // Only used for Scroll Unification.
  // Will be set in GestureScrollBegin
  WeakPersistent<Node> scrollable_node_;

  friend class WebPagePopup;
  friend class PagePopupChromeClient;
  friend class EmptyLocalFrameClient;
};

// WebPagePopupImpl is the only implementation of WebPagePopup and PagePopup, so
// no further checking required.
template <>
struct DowncastTraits<WebPagePopupImpl> {
  static bool AllowFrom(const WebPagePopup& widget) { return true; }
  static bool AllowFrom(const PagePopup& popup) { return true; }
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_PAGE_POPUP_IMPL_H_
