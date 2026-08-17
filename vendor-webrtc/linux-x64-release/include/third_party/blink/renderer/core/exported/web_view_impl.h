/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_VIEW_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_VIEW_IMPL_H_

#include <memory>
#include <vector>

#include "base/debug/stack_trace.h"
#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/observer_list.h"
#include "build/build_config.h"
#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "mojo/public/cpp/bindings/remote_set.h"
#include "third_party/blink/public/common/input/web_gesture_event.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/page/browsing_context_group_info.h"
#include "third_party/blink/public/common/renderer_preferences/renderer_preferences.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/remote_frame.mojom-blink.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/page/page.mojom-blink.h"
#include "third_party/blink/public/mojom/page/page_visibility_state.mojom-blink.h"
#include "third_party/blink/public/mojom/page/prerender_page_param.mojom-forward.h"
#include "third_party/blink/public/mojom/partitioned_popins/partitioned_popin_params.mojom-forward.h"
#include "third_party/blink/public/mojom/renderer_preference_watcher.mojom-blink.h"
#include "third_party/blink/public/platform/scheduler/web_agent_group_scheduler.h"
#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/web/web_frame_widget.h"
#include "third_party/blink/public/web/web_navigation_policy.h"
#include "third_party/blink/public/web/web_view.h"
#include "third_party/blink/public/web/web_view_observer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/exported/web_page_popup_impl.h"
#include "third_party/blink/renderer/core/frame/resize_viewport_anchor.h"
#include "third_party/blink/renderer/core/loader/navigation_policy.h"
#include "third_party/blink/renderer/core/page/chrome_client.h"
#include "third_party/blink/renderer/core/page/context_menu_provider.h"
#include "third_party/blink/renderer/core/page/event_with_hit_test_results.h"
#include "third_party/blink/renderer/core/page/scoped_page_pauser.h"
#include "third_party/blink/renderer/platform/graphics/apply_viewport_changes.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/base/ime/mojom/virtual_keyboard_types.mojom-blink.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace cc {
class ScopedDeferMainFrameUpdate;
}

namespace blink {

namespace frame_test_helpers {
class WebViewHelper;
}

class BrowserControls;
struct ColorProviderColorMaps;
class DevToolsEmulator;
class Frame;
class FullscreenController;
class PageScaleConstraintsSet;
class WebLocalFrame;
class WebLocalFrameImpl;
class WebSettingsImpl;
class WebViewClient;
class WebFrameWidgetImpl;

enum class FullscreenRequestType;

namespace mojom {
namespace blink {
class TextAutosizerPageInfo;
}
}  // namespace mojom

using PaintHoldingCommitTrigger = cc::PaintHoldingCommitTrigger;

class CORE_EXPORT WebViewImpl final : public WebView,
                                      public mojom::blink::PageBroadcast {
 public:
  static WebViewImpl* Create(
      WebViewClient*,
      mojom::blink::PageVisibilityState visibility,
      blink::mojom::PrerenderParamPtr prerender_param,
      std::optional<blink::FencedFrame::DeprecatedFencedFrameMode>
          fenced_frame_mode,
      bool compositing_enabled,
      bool widgets_never_composited,
      WebViewImpl* opener,
      mojo::PendingAssociatedReceiver<mojom::blink::PageBroadcast> page_handle,
      scheduler::WebAgentGroupScheduler& agent_group_scheduler,
      const SessionStorageNamespaceId& session_storage_namespace_id,
      std::optional<SkColor> page_base_background_color,
      const BrowsingContextGroupInfo& browsing_context_group_info,
      const ColorProviderColorMaps* color_provider_colors,
      blink::mojom::PartitionedPopinParamsPtr partitioned_popin_params);

  // All calls to Create() should be balanced with a call to Close(). This
  // synchronously destroys the WebViewImpl.
  void Close() override;
  static HashSet<WebViewImpl*>& AllInstances();
  // Returns true if popup menus should be rendered by the browser, false if
  // they should be rendered by WebKit (which is the default).
  static bool UseExternalPopupMenus();

  // Returns whether frames under this WebView are backed by a compositor.
  bool does_composite() const { return does_composite_; }

  // WebView methods:
  void DidAttachLocalMainFrame() override;
  void DidDetachLocalMainFrame() override;
  void DidAttachRemoteMainFrame(
      CrossVariantMojoAssociatedRemote<
          mojom::blink::RemoteMainFrameHostInterfaceBase>,
      CrossVariantMojoAssociatedReceiver<
          mojom::blink::RemoteMainFrameInterfaceBase>) override;
  void DidDetachRemoteMainFrame() override;
  void SetNoStatePrefetchClient(WebNoStatePrefetchClient*) override;
  WebSettings* GetSettings() override;
  WebString PageEncoding() const override;
  void SetTabKeyCyclesThroughElements(bool value) override;
  bool IsActive() const override;
  void SetIsActive(bool value) override;
  void SetWindowFeatures(const WebWindowFeatures&) override;
  void SetOpenedByDOM() override;
  WebFrame* MainFrame() override;
  const WebFrame* MainFrame() const override;
  WebLocalFrame* FocusedFrame() override;
  void SetFocusedFrame(WebFrame*) override;
  void SmoothScroll(int target_x,
                    int target_y,
                    base::TimeDelta duration) override;
  void AdvanceFocus(bool reverse) override;
  float PageScaleFactor() const override;
  float MinimumPageScaleFactor() const override;
  float MaximumPageScaleFactor() const override;
  void SetDefaultPageScaleLimits(float min_scale, float max_scale) override;
  void SetInitialPageScaleOverride(float) override;
  void SetPageScaleFactor(float) override;
  void SetVisualViewportOffset(const gfx::PointF&) override;
  gfx::PointF VisualViewportOffset() const override;
  gfx::SizeF VisualViewportSize() const override;
  void SetScreenOrientationOverrideForTesting(
      std::optional<display::mojom::blink::ScreenOrientation> orientation)
      override;
  void SetWindowRectSynchronouslyForTesting(
      const gfx::Rect& new_window_rect) override;
  void ResetScrollAndScaleState() override;
  gfx::Size ContentsPreferredMinimumSize() override;
  void UpdatePreferredSize() override;
  void EnablePreferredSizeChangedMode() override;
  void SetZoomFactorForDeviceScaleFactor(float) override;
  float ZoomFactorForViewportLayout() override {
    // This returns the zoom factor to use when determining the layout width
    // while processing the viewport meta tag. We use only the device scale
    // factor, rather than the full zoom factor which includes browser zoom,
    // since changing browser zoom should cause a page to reflow into a static
    // initial containing block. i.e. The device's (potentially simulated by the
    // meta tag) screen size, as measured in physical pixels, does not change
    // with browser zoom.
    //
    // compositor_device_scale_factor_override_ is set by dev tools emulation to
    // simulate a device scale factor of a particular device. If this is set we
    // should use it rather than the host's device scale factor.
    return compositor_device_scale_factor_override_
               ? compositor_device_scale_factor_override_
               : zoom_factor_for_device_scale_factor_;
  }
  bool AutoResizeMode() override;
  void EnableAutoResizeForTesting(const gfx::Size& min_window_size,
                                  const gfx::Size& max_window_size) override;
  void DisableAutoResizeForTesting(const gfx::Size& new_window_size) override;
  WebHitTestResult HitTestResultForTap(const gfx::Point&,
                                       const gfx::Size&) override;
  void EnableDeviceEmulation(const DeviceEmulationParams&) override;
  void DisableDeviceEmulation() override;
  void PerformCustomContextMenuAction(unsigned action) override;
  void DidCloseContextMenu() override;
  void CancelPagePopup() override;
  WebPagePopupImpl* GetPagePopup() const override { return page_popup_.get(); }
  void SetPageFrozen(bool frozen) override;
  WebFrameWidget* MainFrameWidget() override;
  void SetDeviceColorSpaceForTesting(
      const gfx::ColorSpace& color_space) override;
  void PaintContent(cc::PaintCanvas*, const gfx::Rect&) override;
  void RegisterRendererPreferenceWatcher(
      CrossVariantMojoRemote<mojom::RendererPreferenceWatcherInterfaceBase>
          watcher) override;
  void SetRendererPreferences(const RendererPreferences& preferences) override;
  const RendererPreferences& GetRendererPreferences() const override;
  void SetWebPreferences(const web_pref::WebPreferences& preferences) override;
  const web_pref::WebPreferences& GetWebPreferences() override;
  void SetHistoryListFromNavigation(
      int32_t history_index,
      std::optional<int32_t> history_length) override;
  void IncreaseHistoryListFromNavigation() override;
  int32_t HistoryBackListCount() const override;
  int32_t HistoryForwardListCount() const override;
  int32_t HistoryListLength() const { return history_list_length_; }
  const SessionStorageNamespaceId& GetSessionStorageNamespaceId() override;
  bool IsFencedFrameRoot() const override;
  void SetSupportsDraggableRegions(bool supports_draggable_regions) override;

  // Functions to add and remove observers for this object.
  void AddObserver(WebViewObserver* observer);
  void RemoveObserver(WebViewObserver* observer);

  // `BaseBackgroundColor()` affects how the document is rendered.
  // `BackgroundColor()` is what the document computes as its background color
  // (with `BaseBackgroundColor()` as an input), or `BaseBackgroundColor()` if
  // there is no local main frame; it's used as the background color of the
  // compositor.
  //
  // These methods override `BaseBackgroundColor()` or `BackgroundColor()` for
  // specific use cases. You can use this to set a transparent background,
  // which is useful if you want to have some custom background rendered behind
  // the widget.
  //
  // These settings only have an effect for composited WebViews
  // These overrides are listed in order of precedence.
  // - Overriding the background color for fullscreen ignores all other inputs,
  //   including `BaseBackgroundColor()`. Note, however, that
  //   `BaseBackgroundColor()` is passed directly into
  //   `LocalFrameView::SetBaseBackgroundColor`.
  // - Overriding base background color transparency takes precedences over
  //   any base background color that might be specified for inspector--the
  //   transparency specified in the inspector base background color is
  //   ignored.
  //
  // Add new methods with clear precedence for new use cases.
  void SetBackgroundColorOverrideForFullscreenController(
      std::optional<SkColor>);
  void SetBaseBackgroundColorOverrideTransparent(bool override_to_transparent);
  void SetBaseBackgroundColorOverrideForInspector(std::optional<SkColor>);

  // Resize the WebView. You likely should be using
  // MainFrameWidget()->Resize instead.
  void Resize(const gfx::Size&);

  // This method is used for testing.
  // Resize the view at the same time as changing the state of the top
  // controls. If |browser_controls_shrink_layout| is true, the embedder shrunk
  // the WebView size by the browser controls height.
  void ResizeWithBrowserControls(const gfx::Size& main_frame_widget_size,
                                 float top_controls_height,
                                 float bottom_controls_height,
                                 bool browser_controls_shrink_layout);
  // Same as ResizeWithBrowserControls(const gfx::Size&,float,float,bool), but
  // includes all browser controls params such as the min heights.
  void ResizeWithBrowserControls(const gfx::Size& main_frame_widget_size,
                                 const gfx::Size& visible_viewport_size,
                                 cc::BrowserControlsParams);

  // Requests a page-scale animation based on the specified point/rect.
  void AnimateDoubleTapZoom(const gfx::Point&, const gfx::Rect& block_bounds);

  // mojom::blink::PageBroadcast method:
  void SetPageLifecycleState(
      mojom::blink::PageLifecycleStatePtr state,
      mojom::blink::PageRestoreParamsPtr page_restore_params,
      SetPageLifecycleStateCallback callback) override;
  void AudioStateChanged(bool is_audio_playing) override;
  void ActivatePrerenderedPage(
      mojom::blink::PrerenderPageActivationParamsPtr
          prerender_page_activation_params,
      ActivatePrerenderedPageCallback callback) override;
  void UpdateWebPreferences(
      const blink::web_pref::WebPreferences& preferences) override;
  void UpdateRendererPreferences(
      const RendererPreferences& preferences) override;
  void SetHistoryIndexAndLength(int32_t history_index,
                                int32_t history_length) override;
  void SetPageBaseBackgroundColor(std::optional<SkColor> color) override;
  void CreateRemoteMainFrame(
      const RemoteFrameToken& frame_token,
      const std::optional<FrameToken>& opener_frame_token,
      mojom::blink::FrameReplicationStatePtr replicated_state,
      bool is_loading,
      const base::UnguessableToken& devtools_frame_token,
      mojom::blink::RemoteFrameInterfacesFromBrowserPtr remote_frame_interfaces,
      mojom::blink::RemoteMainFrameInterfacesPtr remote_main_frame_interfaces)
      override;
  void UpdatePageBrowsingContextGroup(
      const BrowsingContextGroupInfo& browsing_context_group_info) override;
  void SetPageAttributionSupport(
      network::mojom::AttributionSupport support) override;
  void UpdateColorProviders(
      const ColorProviderColorMaps& color_provider_colors) override;

  void DispatchPersistedPageshow(base::TimeTicks navigation_start);
  void DispatchPagehide(mojom::blink::PagehideDispatch pagehide_dispatch);
  void HookBackForwardCacheEviction(bool hook);

  float DefaultMinimumPageScaleFactor() const;
  float DefaultMaximumPageScaleFactor() const;
  float ClampPageScaleFactorToLimits(float) const;
  void ResetScaleStateImmediately();
  std::optional<display::mojom::blink::ScreenOrientation>
  ScreenOrientationOverride();

  // This is only for non-composited WebViewPlugin.
  void InvalidateContainer();

  void SetZoomFactorOverride(float);
  void SetCompositorDeviceScaleFactorOverride(float);
  gfx::Transform GetDeviceEmulationTransform() const;
  void EnableAutoResizeMode(const gfx::Size& min_viewport_size,
                            const gfx::Size& max_viewport_size);
  void DisableAutoResizeMode();
  void ActivateDevToolsTransform(const DeviceEmulationParams&);
  void DeactivateDevToolsTransform();

  SkColor BackgroundColor() const;
  Color BaseBackgroundColor() const;

  Frame* FocusedCoreFrame() const;

  // Returns the currently focused Element or null if no element has focus.
  Element* FocusedElement() const;

  // Returns the page object associated with this view. This may be null when
  // the page is shutting down, but will be valid at all other times.
  Page* GetPage() const { return page_.Get(); }

  WebViewClient* Client() { return web_view_client_; }

  DevToolsEmulator* GetDevToolsEmulator() const {
    return dev_tools_emulator_.Get();
  }

  // When true, a hint to all WebWidgets that they will never be
  // user-visible and thus never need to produce pixels for display. This is
  // separate from page visibility, as background pages can be marked visible in
  // blink even though they are not user-visible. Page visibility controls blink
  // behaviour for javascript, timers, and such to inform blink it is in the
  // foreground or background. Whereas this bit refers to user-visibility and
  // whether the tab needs to produce pixels to put on the screen at some point
  // or not.
  bool widgets_never_composited() const { return widgets_never_composited_; }

  // Returns the main frame associated with this view. This will be null when
  // the main frame is remote.
  // Internally during startup/shutdown this can be null when no main frame
  // (local or remote) is attached, but this should not generally matter to code
  // outside this class.
  WebLocalFrameImpl* MainFrameImpl() const;

  // TODO(https://crbug.com/1139104): Remove this.
  std::string GetNullFrameReasonForBug1139104() const;

  // Finishes a ScrollIntoView for a focused editable element by performing a
  // view-level reveal. That is, when an embedder requests to reveal a focused
  // editable, the editable is first ScrollIntoView'ed in the layout tree to
  // ensure it's visible in the outermost document but stops short of scrolling
  // the outermost frame. This method will then perform a platform-specific
  // reveal of the editable, e.g. by animating a scroll and zoom in to a
  // legible scale. This should only be called in a WebView where the main
  // frame is local and outermost.
  void FinishScrollFocusedEditableIntoView(
      const gfx::RectF& caret_rect_in_root_frame,
      mojom::blink::ScrollIntoViewParamsPtr params);

  // Handles context menu events orignated via the the keyboard. These
  // include the VK_APPS virtual key and the Shift+F10 combine. Code is
  // based on the Webkit function bool WebView::handleContextMenuEvent(WPARAM
  // wParam, LPARAM lParam) in webkit\webkit\win\WebView.cpp. The only
  // significant change in this function is the code to convert from a
  // Keyboard event to the Right Mouse button down event.
  WebInputEventResult SendContextMenuEvent();

  // Notifies the WebView that a load has been committed. isNewNavigation
  // will be true if a new session history item should be created for that
  // load. isNavigationWithinPage will be true if the navigation does
  // not take the user away from the current page.
  void DidCommitLoad(bool is_new_navigation, bool is_navigation_within_page);

  // Indicates two things:
  //   1) This view may have a new layout now.
  //   2) Layout is up-to-date.
  // After calling WebWidget::updateAllLifecyclePhases(), expect to get this
  // notification unless the view did not need a layout.
  void MainFrameLayoutUpdated();
  void ResizeAfterLayout();
  void DidCommitCompositorFrameForLocalMainFrame();
  void DidChangeContentsSize();
  void PageScaleFactorChanged();
  void OutermostMainFrameScrollOffsetChanged();
  void TextAutosizerPageInfoChanged(
      const mojom::blink::TextAutosizerPageInfo& page_info);

  bool ShouldAutoResize() const { return should_auto_resize_; }

  gfx::Size MinAutoSize() const { return min_auto_size_; }

  gfx::Size MaxAutoSize() const { return max_auto_size_; }

  void UpdateMainFrameLayoutSize();
  void UpdatePageDefinedViewportConstraints(const ViewportDescription&);

  WebPagePopupImpl* OpenPagePopup(PagePopupClient*);
  bool HasOpenedPopup() const { return page_popup_.get(); }
  void ClosePagePopup(PagePopup*);
  // Callback from PagePopup when it is closed, which it can be done directly
  // without coming through WebViewImpl.
  void CleanupPagePopup();
  // Ensure popup's size and position is correct based on its owner element's
  // dimensions.
  void UpdatePagePopup();
  LocalDOMWindow* PagePopupWindow() const;

  PageScheduler* Scheduler() const override;
  void SetVisibilityState(mojom::blink::PageVisibilityState visibility_state,
                          bool is_initial_state) override;
  mojom::blink::PageVisibilityState GetVisibilityState() override;

  void SetPageLifecycleStateFromNewPageCommit(
      mojom::blink::PageVisibilityState visibility,
      mojom::blink::PagehideDispatch pagehide_dispatch) override;

  // Called by a full frame plugin inside this view to inform it that its
  // zoom level has been updated.  The plugin should only call this function
  // if the zoom change was triggered by the browser, it's only needed in case
  // a plugin can update its own zoom, say because of its own UI.
  void FullFramePluginZoomLevelChanged(double zoom_level);

  // Requests a page-scale animation based on the specified rect.
  void ZoomToFindInPageRect(const gfx::Rect&);

  void ComputeScaleAndScrollForBlockRect(
      const gfx::Point& hit_point,
      const gfx::Rect& block_rect,
      float padding,
      float default_scale_when_already_legible,
      float& scale,
      gfx::Point& scroll);
  Node* BestTapNode(const GestureEventWithHitTestResults& targeted_tap_event);
  void EnableTapHighlightAtPoint(
      const GestureEventWithHitTestResults& targeted_tap_event);

  void EnableFakePageScaleAnimationForTesting(bool);
  bool FakeDoubleTapAnimationPendingForTesting() const {
    return double_tap_zoom_pending_;
  }
  gfx::Point FakePageScaleAnimationTargetPositionForTesting() const {
    return fake_page_scale_animation_target_position_;
  }
  float FakePageScaleAnimationPageScaleForTesting() const {
    return fake_page_scale_animation_page_scale_factor_;
  }
  bool FakePageScaleAnimationUseAnchorForTesting() const {
    return fake_page_scale_animation_use_anchor_;
  }
  ui::mojom::blink::VirtualKeyboardMode VirtualKeyboardModeForTesting() {
    return virtual_keyboard_mode_;
  }

  void EnterFullscreen(LocalFrame&,
                       const FullscreenOptions*,
                       FullscreenRequestType);
  void ExitFullscreen(LocalFrame&);
  void FullscreenElementChanged(Element* old_element,
                                Element* new_element,
                                const FullscreenOptions* options,
                                FullscreenRequestType);

  // Sends a request to the main frame's view to resize, and updates the page
  // scale limits if needed.
  void SendResizeEventForMainFrame();

  // Exposed for testing purposes.
  bool HasHorizontalScrollbar();
  bool HasVerticalScrollbar();

  WebSettingsImpl* SettingsImpl();

  BrowserControls& GetBrowserControls();
  // Called anytime browser controls layout height or content offset have
  // changed.
  void DidUpdateBrowserControls();

  void DidUpdateMaxSafeAreaInsets(const gfx::InsetsF& max_safe_area_insets);

  void AddAutoplayFlags(int32_t) override;
  void ClearAutoplayFlags() override;
  int32_t AutoplayFlagsForTest() const override;
  gfx::Size GetPreferredSizeForTest() override;

  gfx::Size Size();
  gfx::Size MainFrameSize();

  PageScaleConstraintsSet& GetPageScaleConstraintsSet() const;

  gfx::Vector2dF ElasticOverscroll() const { return elastic_overscroll_; }

  class ChromeClient& GetChromeClient() const { return *chrome_client_.Get(); }

  // Allows main frame updates to occur if they were previously blocked. They
  // are blocked during loading a navigation, to allow Blink to proceed without
  // being interrupted by useless work until enough progress is made that it
  // desires composited output to be generated.
  void StopDeferringMainFrameUpdate();

  // This function checks the element ids of ScrollableAreas only and returns
  // the equivalent DOM Node if such exists.
  Node* FindNodeFromScrollableCompositorElementId(
      cc::ElementId element_id) const;

  void DidEnterFullscreen();
  void DidExitFullscreen();

  // Called when some JS code has instructed the window associated to the main
  // frame to close, which will result in a request to the browser to close the
  // Widget associated to it.
  void CloseWindow();

  // Controls whether pressing Tab key advances focus to links.
  bool TabsToLinks() const;
  void SetTabsToLinks(bool);

  // Prevent the web page from setting min/max scale via the viewport meta
  // tag. This is an accessibility feature that lets folks zoom in to web
  // pages even if the web page tries to block scaling.
  void SetIgnoreViewportTagScaleLimits(bool);

  // Sets the maximum page scale considered to be legible. Automatic zooms (e.g,
  // double-tap or find in page) will have the page scale limited to this value
  // times the font scale factor. Manual pinch zoom will not be affected by this
  // limit.
  void SetMaximumLegibleScale(float);

  void SetMainFrameViewWidget(WebFrameWidgetImpl* widget);
  WebFrameWidgetImpl* MainFrameViewWidget();

  // Called when hovering over an anchor with the given URL.
  void SetMouseOverURL(const KURL&);

  // Called when keyboard focus switches to an anchor with the given URL.
  void SetKeyboardFocusURL(const KURL&);

  // Asks the browser process to activate this web view.
  void Focus();

  // Asks the browser process to take focus away from the WebView by focusing an
  // adjacent UI element in the containing window.
  void TakeFocus(bool reverse);

  // Shows a previously created WebView (via window.open()).
  void Show(const LocalFrameToken& opener_frame_token,
            NavigationPolicy policy,
            const gfx::Rect& requested_rect,
            const gfx::Rect& adjusted_rect,
            bool opened_by_user_gesture);

  // Send the window rect to the browser and call `ack_callback` when the
  // browser has processed it.
  void SendWindowRectToMainFrameHost(const gfx::Rect& bounds,
                                     base::OnceClosure ack_callback);

  // Tells the browser that another page has accessed the DOM of the initial
  // empty document of a main frame.
  void DidAccessInitialMainDocument();

  // Sends window.minimize() requests to the browser window.
  void Minimize();
  // Sends window.maximize() requests to the browser window.
  void Maximize();
  // Sends window.restore() requests to the browser window.
  void Restore();
  // Sends window.setResizable() requests to the browser window.
  void SetResizable(bool resizable);

  // TODO(crbug.com/1149992): This is called from the associated widget and this
  // code should eventually move out of WebView into somewhere else.
  void ApplyViewportChanges(const ApplyViewportChangesArgs& args);

  // Indication that the root layer for the main frame widget has changed.
  void DidChangeRootLayer(bool root_layer_exists);

  // Sets the page focus.
  void SetPageFocus(bool enable);

  // This method is used for testing.
  // Resizes the unscaled (page scale = 1.0) visual viewport. Normally the
  // unscaled visual viewport is the same size as the main frame. The passed
  // size becomes the size of the viewport when page scale = 1. This
  // is used to shrink the visible viewport to allow things like the ChromeOS
  // virtual keyboard to overlay over content but allow scrolling it into view.
  void ResizeVisualViewport(const gfx::Size&);

  // Called once a paint happens after the first non empty layout. In other
  // words, after the frame has painted something.
  void DidFirstVisuallyNonEmptyPaint();

  // Caleld once the first contentful paint happens on the main frame.
  void OnFirstContentfulPaint();

  scheduler::WebAgentGroupScheduler& GetWebAgentGroupScheduler();

  // Returns true if the page supports app-region: drag/no-drag.
  bool SupportsDraggableRegions();
  // Called when draggable regions in the page change.
  void DraggableRegionsChanged();

  double ClampZoomLevel(double zoom_level) const;
  double ZoomLevelToZoomFactor(double zoom_level) const;

 private:
  FRIEND_TEST_ALL_PREFIXES(WebFrameTest, DivScrollIntoEditableTest);
  FRIEND_TEST_ALL_PREFIXES(WebFrameTest,
                           DivScrollIntoEditablePreservePageScaleTest);
  FRIEND_TEST_ALL_PREFIXES(WebFrameTest,
                           DivScrollIntoEditableTestZoomToLegibleScaleDisabled);
  FRIEND_TEST_ALL_PREFIXES(WebFrameTest,
                           DivScrollIntoEditableTestWithDeviceScaleFactor);
  FRIEND_TEST_ALL_PREFIXES(WebViewTest, SetBaseBackgroundColorBeforeMainFrame);
  FRIEND_TEST_ALL_PREFIXES(WebViewTest, LongPressImage);
  FRIEND_TEST_ALL_PREFIXES(WebViewTest, LongPressImageAndThenLongTapImage);
  FRIEND_TEST_ALL_PREFIXES(WebViewTest, UpdateTargetURLWithInvalidURL);
  FRIEND_TEST_ALL_PREFIXES(WebViewTest, TouchDragContextMenu);
  FRIEND_TEST_ALL_PREFIXES(WebViewTest, ContextMenuAndDrag);

  friend class frame_test_helpers::WebViewHelper;
  friend class SimCompositor;
  friend class WebView;  // So WebView::Create can call our constructor

  void AcceptLanguagesChanged();
  void ThemeChanged();

  // Update the target url locally and tell the browser that the target URL has
  // changed. If |url| is empty, show |fallback_url|.
  void UpdateTargetURL(const WebURL& url, const WebURL& fallback_url);

  // Helper functions to send the updated target URL to the right render frame
  // in the browser process, and to handle its associated reply message.
  void SendUpdatedTargetURLToBrowser(const KURL& target_url);
  void TargetURLUpdatedInBrowser();

  void SetPageScaleFactorAndLocation(float scale,
                                     bool is_pinch_gesture_active,
                                     const gfx::PointF&);
  void PropagateZoomFactorToLocalFrameRoots(Frame* frame, float zoom_factor);
  void SetPageLifecycleStateInternal(
      mojom::blink::PageLifecycleStatePtr new_state,
      mojom::blink::PageRestoreParamsPtr page_restore_params);

  // Updates the main frame's view transition state.
  void UpdateViewTransitionState(
      bool restoring_from_bfcache,
      bool storing_in_bfcache,
      const mojom::blink::PageRestoreParamsPtr& page_restore_params);

  float MaximumLegiblePageScale() const;
  void RefreshPageScaleFactor();
  gfx::Size ContentsSize() const;

  void UpdateBrowserControlsConstraint(cc::BrowserControlsState constraint);
  void UpdateICBAndResizeViewport(const gfx::Size& visible_viewport_size);
  void ResizeViewWhileAnchored(cc::BrowserControlsParams params,
                               const gfx::Size& visible_viewport_size);

  void UpdateBaseBackgroundColor();
  void UpdateFontRenderingFromRendererPrefs();

  // Request the window to close from the renderer by sending the request to the
  // browser.
  void DoDeferredCloseWindowSoon();

#if BUILDFLAG(IS_CHROMEOS)
  void UpdateUseOverlayScrollbar(bool use_overlay_scrollbar);
#endif

  WebViewImpl(
      WebViewClient*,
      mojom::blink::PageVisibilityState visibility,
      blink::mojom::PrerenderParamPtr prerender_param,
      std::optional<blink::FencedFrame::DeprecatedFencedFrameMode>
          fenced_frame_mode,
      bool does_composite,
      bool widgets_never_composite,
      WebViewImpl* opener,
      mojo::PendingAssociatedReceiver<mojom::blink::PageBroadcast> page_handle,
      scheduler::WebAgentGroupScheduler& agent_group_scheduler,
      const SessionStorageNamespaceId& session_storage_namespace_id,
      std::optional<SkColor> page_base_background_color,
      const BrowsingContextGroupInfo& browsing_context_group_info,
      const ColorProviderColorMaps* color_provider_colors,
      blink::mojom::PartitionedPopinParamsPtr partitioned_popin_params);
  ~WebViewImpl() override;

  void ConfigureAutoResizeMode();

  void DoComposite();
  void ReallocateRenderer();

  void SetDeviceEmulationTransform(const gfx::Transform&);
  void UpdateDeviceEmulationTransform();

  // Helper function: Widens the width of |source| by the specified margins
  // while keeping it smaller than page width.
  //
  // This method can only be called if the main frame is local.
  gfx::Rect WidenRectWithinPageBounds(const gfx::Rect& source,
                                      int target_margin,
                                      int minimum_margin);

  void EnablePopupMouseWheelEventListener(WebLocalFrameImpl* local_root);
  void DisablePopupMouseWheelEventListener();

  LocalFrame* FocusedLocalFrameInWidget() const;

  // Clear focus and text input state of the page. If there was a focused
  // element, this will trigger updates to observers and send focus, selection,
  // and text input-related events.
  void RemoveFocusAndTextInputState();

  // Finds the zoom and scroll parameters for zooming into an editable element
  // with bounds |element_bounds_in_root_frame| and caret bounds
  // |caret_bounds_in_root_frame|.
  void ComputeScaleAndScrollForEditableElementRects(
      const gfx::Rect& element_bounds_in_root_frame,
      const gfx::Rect& caret_bounds_in_root_frame,
      bool zoom_into_legible_scale,
      float& new_scale,
      gfx::Point& new_scroll_position,
      bool& need_animation);

  // Starts a page scale (and scroll!) animation on the compositor thread that
  // will smoothly animate the viewport position and zoom. Returns true if an
  // animation was queued, false if an animation was not needed.
  //
  // * target_position - A position in root document coordinates ("position"
  // meaning it's relative to the top-left of the document).  This is the
  // location that scroll will animate to.
  // * use_anchor: If true, the animation `target_position` is used as an
  // "anchor". `target_position` must already be visible in the viewport and
  // scroll/zoom will be animated such that the anchor will be kept fixed in
  // the viewport (if possible).
  // * new_scale: The target page scale factor to animate to.
  // * duration: The animation's duration.
  bool StartPageScaleAnimation(const gfx::Point& target_position,
                               bool use_anchor,
                               float new_scale,
                               base::TimeDelta duration);

  // Sends any outstanding TrackedFeaturesUpdate messages to the browser.
  void ReportActiveSchedulerTrackedFeatures();

  // Callback when this widget window has been displayed by the browser.
  // Corresponds to a Show method call.
  void DidShowCreatedWindow();

  // Called when mojo is disconnected.
  void MojoDisconnected();

  // Called when any input to zoom factor calculation changes on the WebView, to
  // trigger recalculation of zoom factor for all affected widgets.
  void UpdateWidgetZoomFactors();
  void UpdateInspectorDeviceScaleFactorOverride();

  // A value provided by the browser to state that all Widgets in this
  // WebView's frame tree will never be user-visible and thus never need to
  // produce pixels for display. This is separate from Page visibility, as
  // non-user-visible pages can still be marked visible for blink. Page
  // visibility controls blink behaviour for javascript, timers, and such to
  // inform blink it is in the foreground or background. Whereas this bit refers
  // to user-visibility and whether the tab needs to produce pixels to put on
  // the screen at some point or not.
  const bool widgets_never_composited_;

  // Can be null (e.g. unittests, shared workers, etc).
  WebViewClient* web_view_client_;
  Persistent<ChromeClient> chrome_client_;
  Persistent<Page> page_;

  // This is the size of the page that the web contents will render into. This
  // is usually, but not necessarily the same as the VisualViewport size. The
  // VisualViewport is the 'inner' viewport, and can be smaller than the size of
  // the page. This allows the browser to shrink the size of the displayed
  // contents [e.g. to accomodate a keyboard] without forcing the web page to
  // relayout. For more details, see the header for the VisualViewport class.
  gfx::Size size_;
  // If true, automatically resize the layout view around its content.
  bool should_auto_resize_ = false;
  // The lower bound on the size when auto-resizing.
  gfx::Size min_auto_size_;
  // The upper bound on the size when auto-resizing.
  gfx::Size max_auto_size_;

  // An object that can be used to manipulate m_page->settings() without linking
  // against WebCore. This is lazily allocated the first time GetWebSettings()
  // is called.
  std::unique_ptr<WebSettingsImpl> web_settings_;

  // The state of our target_url transmissions. When we receive a request to
  // send a URL to the browser, we set this to TARGET_INFLIGHT until an ACK
  // comes back - if a new request comes in before the ACK, we store the new
  // URL in pending_target_url_ and set the status to TARGET_PENDING. If an
  // ACK comes back and we are in TARGET_PENDING, we send the stored URL and
  // revert to TARGET_INFLIGHT.
  //
  // We don't need a queue of URLs to send, as only the latest is useful.
  enum {
    TARGET_NONE,
    TARGET_INFLIGHT,  // We have a request in-flight, waiting for an ACK
    TARGET_PENDING    // INFLIGHT + we have a URL waiting to be sent
  } target_url_status_ = TARGET_NONE;

  // The URL we show the user in the status bar. We use this to determine if we
  // want to send a new one (we do not need to send duplicates). It will be
  // equal to either |mouse_over_url_| or |focus_url_|, depending on which was
  // updated last.
  KURL target_url_;

  // The next target URL we want to send to the browser.
  KURL pending_target_url_;

  // The URL the user's mouse is hovering over.
  KURL mouse_over_url_;

  // The URL that has keyboard focus.
  KURL focus_url_;

  // while zoom level is stored for each frame, the maximum zoom level and
  // minimum zoom level are a webView Property.
  const double minimum_zoom_level_;
  const double maximum_zoom_level_;

  // Additional zoom factor used to scale the content by device scale factor.
  double zoom_factor_for_device_scale_factor_ = 1.;

  // This value, when multiplied by the font scale factor, gives the maximum
  // page scale that can result from automatic zooms.
  float maximum_legible_scale_ = 1.f;

  // The scale moved to by the latest double tap zoom, if any.
  float double_tap_zoom_page_scale_factor_ = 0.f;
  // Have we sent a double-tap zoom and not yet heard back the scale?
  bool double_tap_zoom_pending_ = false;

  // Used for testing purposes.
  bool enable_fake_page_scale_animation_for_testing_ = false;
  gfx::Point fake_page_scale_animation_target_position_;
  float fake_page_scale_animation_page_scale_factor_ = 0.f;
  bool fake_page_scale_animation_use_anchor_ = false;

  float compositor_device_scale_factor_override_ = 0.f;
  gfx::Transform device_emulation_transform_;

  // The index of the current item in the history list.
  int32_t history_list_index_ = -1;

  // The RenderView's current impression of the history length.  This includes
  // any items that have committed in this process, but because of cross-process
  // navigations, the history may have some entries that were committed in other
  // processes.  We won't know about them until the next navigation in this
  // process.
  int32_t history_list_length_ = 0;

  // The popup associated with an input/select element. The popup is owned via
  // closership (self-owned-but-deleted-via-close) by RenderWidget. We also hold
  // a reference here because we can extend the lifetime of the popup while
  // handling input events in order to compare its popup client after it was
  // closed.
  scoped_refptr<WebPagePopupImpl> page_popup_;

  Persistent<DevToolsEmulator> dev_tools_emulator_;

  // Whether the user can press tab to focus links.
  bool tabs_to_links_ = false;

  // WebViews, and WebWidgets, are used to host a Page. The WidgetClient()
  // provides compositing support for the WebView.
  // In some cases, a WidgetClient() is not provided, or it informs us that
  // it won't be presenting content via a compositor.
  //
  // TODO(dcheng): All WebViewImpls should have an associated LayerTreeView,
  // but for various reasons, that's not the case... WebView plugin, printing,
  // workers, and tests don't use a compositor in their WebViews. Sometimes
  // they avoid the compositor by using a null client, and sometimes by having
  // the client return a null compositor. We should make things more consistent
  // and clear.
  const bool does_composite_;

  bool matches_heuristics_for_gpu_rasterization_ = false;

  std::unique_ptr<FullscreenController> fullscreen_controller_;

  std::optional<SkColor> background_color_override_for_fullscreen_controller_;
  bool override_base_background_color_to_transparent_ = false;
  std::optional<SkColor> base_background_color_override_for_inspector_;
  SkColor page_base_background_color_;  // Only applies to main frame.

  float zoom_factor_override_ = 0.f;

  gfx::Vector2dF elastic_overscroll_;

  // If true, we send IPC messages when |preferred_size_| changes.
  bool send_preferred_size_changes_ = false;

  // Whether the preferred size may have changed and |UpdatePreferredSize| needs
  // to be called.
  bool needs_preferred_size_update_ = true;

  // Cache the preferred size of the page in order to prevent sending the IPC
  // when layout() recomputes but doesn't actually change sizes.
  gfx::Size preferred_size_in_dips_;

  Persistent<EventListener> popup_mouse_wheel_event_listener_;

  web_pref::WebPreferences web_preferences_;

  blink::RendererPreferences renderer_preferences_;

  // The local root whose document has |popup_mouse_wheel_event_listener_|
  // registered.
  WeakPersistent<WebLocalFrameImpl> local_root_with_empty_mouse_wheel_listener_;

  // The WebWidget for the main frame. This is expected to be unset when the
  // WebWidget destroys itself. This will be null if the main frame is remote.
  WeakPersistent<WebFrameWidgetImpl> web_widget_;

  // We defer commits when transitioning to a new page. ChromeClientImpl calls
  // StopDeferringCommits() to release this when a new page is loaded.
  std::unique_ptr<cc::ScopedDeferMainFrameUpdate>
      scoped_defer_main_frame_update_;

  Persistent<ResizeViewportAnchor> resize_viewport_anchor_;

  // Handle to the local main frame host. Only valid when the MainFrame is
  // local. It is ok to use WTF::Unretained(this) for callbacks made on this
  // interface because the callbacks will be associated with the lifecycle
  // of this AssociatedRemote and the lifetime of the main LocalFrame.
  mojo::AssociatedRemote<mojom::blink::LocalMainFrameHost>
      local_main_frame_host_remote_;

  // Handle to the remote main frame host. Only valid when the MainFrame is
  // remote.  It is ok to use WTF::Unretained(this) for callbacks made on this
  // interface because the callbacks will be associated with the lifecycle
  // of this AssociatedRemote and the lifetime of the main RemoteFrame.
  mojo::AssociatedRemote<mojom::blink::RemoteMainFrameHost>
      remote_main_frame_host_remote_;

  std::optional<display::mojom::blink::ScreenOrientation>
      screen_orientation_override_;

  mojo::AssociatedReceiver<mojom::blink::PageBroadcast> receiver_;

  // These are observing changes in |renderer_preferences_|. This is used for
  // keeping WorkerFetchContext in sync.
  mojo::RemoteSet<mojom::blink::RendererPreferenceWatcher>
      renderer_preference_watchers_;

  // The SessionStorage namespace that we're assigned to has an ID, and that ID
  // is passed to us upon creation.  WebKit asks for this ID upon first use and
  // uses it whenever asking the browser process to allocate new storage areas.
  SessionStorageNamespaceId session_storage_namespace_id_;

  // The mode that the virtual keyboard is in, with respect to how it will
  // affect the Blink viewport and layout. This can be set by the page using
  // the viewport meta tag.
  ui::mojom::blink::VirtualKeyboardMode virtual_keyboard_mode_ =
      ui::mojom::blink::VirtualKeyboardMode::kUnset;

  scheduler::WebAgentGroupScheduler& web_agent_group_scheduler_;

  // TODO(crbug.com/1499519): Remove this temporary debugging.
  std::optional<base::debug::StackTrace> close_task_posted_stack_trace_;
  std::optional<base::debug::StackTrace> close_called_stack_trace_;
  std::optional<base::debug::StackTrace> close_window_called_stack_trace_;

  // Indicates whether the page supports draggable regions via the app-region
  // CSS property.
  bool supports_draggable_regions_ = false;

  // All the registered observers.
  base::ObserverList<WebViewObserver> observers_;
};

// WebView is always implemented by WebViewImpl, so explicitly allow the
// downcast.
template <>
struct DowncastTraits<WebViewImpl> {
  static bool AllowFrom(const WebView& web_view) { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_VIEW_IMPL_H_
