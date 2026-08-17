/*
 * Copyright (C) 2009, 2010, 2011, 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_VIEW_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_VIEW_H_

#include <optional>

#include "third_party/blink/public/common/dom_storage/session_storage_namespace_id.h"
#include "third_party/blink/public/common/fenced_frame/redacted_fenced_frame_config.h"
#include "third_party/blink/public/common/page/browsing_context_group_info.h"
#include "third_party/blink/public/common/renderer_preferences/renderer_preferences.h"
#include "third_party/blink/public/common/web_preferences/web_preferences.h"
#include "third_party/blink/public/mojom/fenced_frame/fenced_frame.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-shared.h"
#include "third_party/blink/public/mojom/page/page.mojom-shared.h"
#include "third_party/blink/public/mojom/page/page_visibility_state.mojom-shared.h"
#include "third_party/blink/public/mojom/page/prerender_page_param.mojom-forward.h"
#include "third_party/blink/public/mojom/partitioned_popins/partitioned_popin_params.mojom-forward.h"
#include "third_party/blink/public/mojom/renderer_preference_watcher.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/scheduler/web_agent_group_scheduler.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/skia/include/core/SkColor.h"
#include "ui/display/mojom/screen_orientation.mojom-shared.h"

namespace base {
class TimeDelta;
}

namespace cc {
class PaintCanvas;
}

namespace gfx {
class ColorSpace;
class Point;
class PointF;
class Rect;
class Size;
class SizeF;
}  // namespace gfx

namespace blink {
struct ColorProviderColorMaps;
class PageScheduler;
class WebFrame;
class WebFrameWidget;
class WebHitTestResult;
class WebLocalFrame;
class WebNoStatePrefetchClient;
class WebPagePopup;
class WebRemoteFrame;
class WebSettings;
class WebString;
class WebViewClient;
class WebWidget;
struct DeviceEmulationParams;
struct WebWindowFeatures;

class BLINK_EXPORT WebView {
 public:
  static const double kTextSizeMultiplierRatio;
  static const double kMinTextSizeMultiplier;
  static const double kMaxTextSizeMultiplier;

  enum StyleInjectionTarget {
    kInjectStyleInAllFrames,
    kInjectStyleInTopFrameOnly
  };

  // Initialization ------------------------------------------------------

  // Creates a WebView that is NOT yet initialized. To complete initialization,
  // call WebLocalFrame::CreateMainFrame() or WebRemoteFrame::CreateMainFrame()
  // as appropriate. It is legal to modify settings before completing
  // initialization.
  //
  // The WebView is kept alive as long as the `page_handle` mojo interface
  // is alive. The WebView will be destroyed when that interface closes, if
  // a client wishes to close the WebView synchronously it can call `Close`
  // directly.
  //
  // clients may be null, but should both be null or not together.
  // |is_hidden| defines the initial visibility of the page.
  // |prerender_param| defines a set of parameters for prerendering views. It is
  // set iff the view is created for a prerendering page. (see
  // content/browser/preloading/prerender/README.md).
  // [is_fenced_frame] defines whether the page is for a fenced frame.
  // |compositing_enabled| dictates whether accelerated compositing should be
  // enabled for the page. It must be false if no clients are provided, or if a
  // LayerTreeView will not be set for the WebWidget.
  // TODO(danakj): This field should go away as WebWidgets always composite
  // their output.
  // |page_handle| is only set for views that are part of a WebContents' frame
  // tree.
  // |widgets_never_composited| is an indication that all WebWidgets associated
  // with this WebView will never be user-visible and thus never need to produce
  // pixels for display. This is separate from page visibility, as background
  // pages can be marked visible in blink even though they are not user-visible.
  // Page visibility controls blink behaviour for javascript, timers, and such
  // to inform blink it is in the foreground or background. Whereas this bit
  // refers to user-visibility and whether the tab needs to produce pixels to
  // put on the screen at some point or not.
  // |page_base_background_color| initial base background color used by the main
  // frame. Set on create to avoid races. Passing in nullopt indicates the
  // default base background color should be used.
  // TODO(yuzus): Remove |is_hidden| and start using |PageVisibilityState|.
  // |color_provider_colors| is used to create color providers that live in the
  // Page. Passing in nullptr indicates the default color maps should be used.
  // `partitioned_popin_params` are set if this window was opened as a
  // partitioned popin. The entire frame tree of a partitioned popin is
  // partitioned as though it was an iframe in the opener.
  // See https://explainers-by-googlers.github.io/partitioned-popins/
  static WebView* Create(
      WebViewClient*,
      bool is_hidden,
      blink::mojom::PrerenderParamPtr prerender_param,
      std::optional<blink::FencedFrame::DeprecatedFencedFrameMode>
          fenced_frame_mode,
      bool compositing_enabled,
      bool widgets_never_composited,
      WebView* opener,
      CrossVariantMojoAssociatedReceiver<mojom::PageBroadcastInterfaceBase>
          page_handle,
      scheduler::WebAgentGroupScheduler& agent_group_scheduler,
      const SessionStorageNamespaceId& session_storage_namespace_id,
      std::optional<SkColor> page_base_background_color,
      const BrowsingContextGroupInfo& browsing_context_group_info,
      const ColorProviderColorMaps* color_provider_colors,
      blink::mojom::PartitionedPopinParamsPtr partitioned_popin_params);

  // Destroys the WebView synchronously.
  virtual void Close() = 0;

  // Called to inform WebViewImpl that a local main frame has been attached.
  // After this call MainFrameImpl() will return a valid frame until it is
  // detached.
  virtual void DidAttachLocalMainFrame() = 0;

  // Called while the main LocalFrame is being detached. The MainFrameImpl() is
  // still valid until after this method is called.
  virtual void DidDetachLocalMainFrame() = 0;

  // Called to inform WebViewImpl that a remote main frame has been attached.
  // Associated channels should be passed and bound.
  virtual void DidAttachRemoteMainFrame(
      CrossVariantMojoAssociatedRemote<mojom::RemoteMainFrameHostInterfaceBase>
          main_frame_host,
      CrossVariantMojoAssociatedReceiver<mojom::RemoteMainFrameInterfaceBase>
          main_frame) = 0;

  // Called to inform WebViewImpl that a remote main frame has been detached.
  virtual void DidDetachRemoteMainFrame() = 0;

  virtual void SetNoStatePrefetchClient(WebNoStatePrefetchClient*) = 0;

  // Returns the session storage namespace id associated with this WebView.
  virtual const SessionStorageNamespaceId& GetSessionStorageNamespaceId() = 0;

  // Options -------------------------------------------------------------

  // The returned pointer is valid for the lifetime of the WebView.
  virtual WebSettings* GetSettings() = 0;

  // Corresponds to the encoding of the main frame.  Setting the page
  // encoding may cause the main frame to reload.
  virtual WebString PageEncoding() const = 0;

  // Method that controls whether pressing Tab key cycles through page
  // elements or inserts a '\t' char in the focused text area.
  virtual void SetTabKeyCyclesThroughElements(bool) = 0;

  // Controls the WebView's active state, which may affect the rendering
  // of elements on the page (i.e., tinting of input elements).
  virtual bool IsActive() const = 0;
  virtual void SetIsActive(bool) = 0;

  // Allows setting the state of the various bars exposed via BarProp
  // properties on the window object. The size related fields of
  // WebWindowFeatures are ignored.
  virtual void SetWindowFeatures(const WebWindowFeatures&) = 0;

  // Marks the WebView as being opened by a DOM call. This is relevant
  // for whether window.close() may be called.
  virtual void SetOpenedByDOM() = 0;

  // Frames --------------------------------------------------------------

  virtual WebFrame* MainFrame() = 0;
  virtual const WebFrame* MainFrame() const = 0;

  // Focus ---------------------------------------------------------------

  virtual WebLocalFrame* FocusedFrame() = 0;
  virtual void SetFocusedFrame(WebFrame*) = 0;

  // Smooth scroll the root layer to |targetX|, |targetY| in |duration|.
  virtual void SmoothScroll(int target_x,
                            int target_y,
                            base::TimeDelta duration) {}

  // Advance the focus of the WebView forward to the next element or to the
  // previous element in the tab sequence (if reverse is true).
  virtual void AdvanceFocus(bool reverse) {}

  // Gets the scale factor of the page, where 1.0 is the normal size, > 1.0
  // is scaled up, < 1.0 is scaled down.
  virtual float PageScaleFactor() const = 0;

  // Scales the page without affecting layout by using the visual viewport.
  virtual void SetPageScaleFactor(float) = 0;

  // Minimum and Maximum as computed as a combination of default, page defined,
  // UA, etc. constraints.
  virtual float MinimumPageScaleFactor() const = 0;
  virtual float MaximumPageScaleFactor() const = 0;

  // Sets the offset of the visual viewport within the main frame, in
  // fractional CSS pixels. The offset will be clamped so the visual viewport
  // stays within the frame's bounds.
  virtual void SetVisualViewportOffset(const gfx::PointF&) = 0;

  // Gets the visual viewport's current offset within the page's main frame,
  // in fractional CSS pixels.
  virtual gfx::PointF VisualViewportOffset() const = 0;

  // Get the visual viewport's size in CSS pixels.
  virtual gfx::SizeF VisualViewportSize() const = 0;

  // Sets the default minimum, and maximum page scale. These will be overridden
  // by the page or by the overrides below if they are set.
  virtual void SetDefaultPageScaleLimits(float min_scale, float max_scale) = 0;

  // Sets the initial page scale to the given factor. This scale setting
  // overrides
  // page scale set in the page's viewport meta tag.
  virtual void SetInitialPageScaleOverride(float) = 0;

  // Reset any saved values for the scroll and scale state.
  virtual void ResetScrollAndScaleState() = 0;

  // Returns the "preferred" contents size, defined as the preferred minimum
  // width of the main document's contents and the minimum height required to
  // display the main document without scrollbars. If the document is in quirks
  // mode (does not have <!doctype html>), the height will stretch to fill the
  // viewport. The returned size has the page zoom factor applied. The lifecycle
  // must be updated to at least layout before calling (see: |UpdateLifecycle|).
  //
  // This may only be called when there is a local main frame attached to this
  // WebView.
  virtual gfx::Size ContentsPreferredMinimumSize() = 0;

  // Check whether the preferred size has changed. This should only be called
  // with up-to-date layout.
  virtual void UpdatePreferredSize() = 0;

  // Indicates that view's preferred size changes will be sent to the browser.
  virtual void EnablePreferredSizeChangedMode() = 0;

  // Sets the additional zoom factor used for device scale factor. This is used
  // to scale the content by the device scale factor, without affecting zoom
  // level.
  virtual void SetZoomFactorForDeviceScaleFactor(float) = 0;

  // Gets the device scale zoom that will be factored into the viewport layout
  // width.
  virtual float ZoomFactorForViewportLayout() = 0;

  // Override the screen orientation override.
  virtual void SetScreenOrientationOverrideForTesting(
      std::optional<display::mojom::ScreenOrientation> orientation) = 0;

  // Set the window rect synchronously for testing. The normal flow is an
  // asynchronous request to the browser.
  virtual void SetWindowRectSynchronouslyForTesting(
      const gfx::Rect& new_window_rect) = 0;

  // Auto-Resize -----------------------------------------------------------

  // Return the state of the auto resize mode.
  virtual bool AutoResizeMode() = 0;

  // Enable auto resize.
  virtual void EnableAutoResizeForTesting(const gfx::Size& min_size,
                                          const gfx::Size& max_size) = 0;

  // Disable auto resize.
  virtual void DisableAutoResizeForTesting(const gfx::Size& new_size) = 0;

  // Data exchange -------------------------------------------------------

  // Do a hit test equivalent to what would be done for a GestureTap event
  // that has width/height corresponding to the supplied |tapArea|.
  //
  // TODO(crbug.com/376493204): This method is only called by Blink unit tests,
  // so it should be removed from this API.
  virtual WebHitTestResult HitTestResultForTap(const gfx::Point& tap_point,
                                               const gfx::Size& tap_area) = 0;

  // Developer tools -----------------------------------------------------

  // Enables device emulation as specified in params.
  virtual void EnableDeviceEmulation(const DeviceEmulationParams&) = 0;

  // Cancel emulation started via |enableDeviceEmulation| call.
  virtual void DisableDeviceEmulation() = 0;

  // Context menu --------------------------------------------------------

  virtual void PerformCustomContextMenuAction(unsigned action) = 0;

  // Notify that context menu has been closed.
  virtual void DidCloseContextMenu() = 0;

  // Popup menu ----------------------------------------------------------

  // Sets whether select popup menus should be rendered by the browser.
  static void SetUseExternalPopupMenus(bool);

  // Cancels and hides the current popup (datetime, select...) if any.
  virtual void CancelPagePopup() = 0;

  // Returns the current popup if any.
  virtual WebPagePopup* GetPagePopup() const = 0;

  // Visited link state --------------------------------------------------

  // Tells all WebView instances to update the visited link state for the
  // specified hash.
  static void UpdateVisitedLinkState(uint64_t hash);

  // Tells all WebView instances to update the visited state for all
  // their links. Use invalidateVisitedLinkHashes to inform that the visitedlink
  // table was changed and the salt was changed too. And all cached visitedlink
  // hashes need to be recalculated.
  static void ResetVisitedLinkState(bool invalidate_visited_link_hashes);

  // Custom colors -------------------------------------------------------

  virtual void SetDeviceColorSpaceForTesting(
      const gfx::ColorSpace& color_space) = 0;

  // Scheduling -----------------------------------------------------------

  virtual PageScheduler* Scheduler() const = 0;

  // Visibility -----------------------------------------------------------

  // Sets the visibility of the WebView.
  virtual void SetVisibilityState(mojom::PageVisibilityState visibility_state,
                                  bool is_initial_state) = 0;
  virtual mojom::PageVisibilityState GetVisibilityState() = 0;

  // PageLifecycleState ----------------------------------------------------

  // Sets the |visibility| and |pagehide_dispatch| properties for the
  // PageLifecycleState of this page from a new page's commit. Should only be
  // called from a main-frame same-site navigation where we did a proactive
  // BrowsingInstance swap and we're reusing the old page's process.
  // Note that unlike SetPageLifecycleState in PageBroadcast/WebViewImpl, we
  // don't need to pass a callback here to notify the browser site that the
  // PageLifecycleState has been successfully updated.
  // TODO(rakina): When it's possible to pass PageLifecycleState here, pass
  // PageLifecycleState instead.
  virtual void SetPageLifecycleStateFromNewPageCommit(
      mojom::PageVisibilityState visibility,
      mojom::PagehideDispatch pagehide_dispatch) = 0;

  // Lifecycle state ------------------------------------------------------

  // Freezes or unfreezes the page and all the local frames.
  virtual void SetPageFrozen(bool frozen) = 0;

  // Autoplay configuration -----------------------------------------------

  // Sets the autoplay flags for this webview's page.
  // The valid flags are defined in
  // third_party/blink/public/platform/autoplay.mojom
  virtual void AddAutoplayFlags(int32_t flags) = 0;
  virtual void ClearAutoplayFlags() = 0;
  virtual int32_t AutoplayFlagsForTest() const = 0;
  virtual gfx::Size GetPreferredSizeForTest() = 0;

  // Non-composited support -----------------------------------------------

  // Called to paint the rectangular region within the WebView's main frame
  // onto the specified canvas at (viewport.x, viewport.y). This is to provide
  // support for non-composited WebViews, and is used to paint into a
  // PaintCanvas being supplied by another (composited) WebView.
  //
  // Before calling PaintContent(), the caller must ensure the lifecycle of the
  // widget's frame is clean by calling
  // UpdateLifecycle(WebLifecycleUpdate::All). It is okay to call paint multiple
  // times once the lifecycle is clean, assuming no other changes are made to
  // the WebWidget (e.g., once events are processed, it should be assumed that
  // another call to UpdateLifecycle is warranted before painting again). Paints
  // starting from the main LayoutView's property tree state, thus ignoring any
  // transient transormations (e.g. pinch-zoom, dev tools emulation, etc.).
  //
  // The painting will be performed without applying the DevicePixelRatio as
  // scaling is expected to already be applied to the PaintCanvas by the
  // composited WebView which supplied the PaintCanvas. The canvas state may
  // be modified and should be saved before calling this method and restored
  // after.
  virtual void PaintContent(cc::PaintCanvas*, const gfx::Rect& viewport) = 0;

  // Renderer preferences ---------------------------------------------------

  virtual void RegisterRendererPreferenceWatcher(
      CrossVariantMojoRemote<mojom::RendererPreferenceWatcherInterfaceBase>
          watcher) = 0;

  virtual void SetRendererPreferences(
      const RendererPreferences& preferences) = 0;
  virtual const RendererPreferences& GetRendererPreferences() const = 0;

  // Web preferences ---------------------------------------------------

  // Applies blink related preferences to this view.
  static void ApplyWebPreferences(const web_pref::WebPreferences& prefs,
                                  WebView* web_view);

  virtual void SetWebPreferences(
      const web_pref::WebPreferences& preferences) = 0;
  virtual const web_pref::WebPreferences& GetWebPreferences() = 0;

  // TODO(lfg): Remove this once the refactor of WebView/WebWidget is
  // completed.
  virtual WebFrameWidget* MainFrameWidget() = 0;

  // History list ---------------------------------------------------------
  virtual void SetHistoryListFromNavigation(
      int32_t history_index,
      std::optional<int32_t> history_length) = 0;
  virtual void IncreaseHistoryListFromNavigation() = 0;

  // Session history -----------------------------------------------------
  // Returns the number of history items before/after the current
  // history item.
  virtual int32_t HistoryBackListCount() const = 0;
  virtual int32_t HistoryForwardListCount() const = 0;

  // Returns whether this WebView represents a fenced frame root or not.
  virtual bool IsFencedFrameRoot() const = 0;

  // Draggable Regions ---------------------------------------------------
  // Indicates that this WebView should collect draggable regions set using the
  // app-region CSS property.
  virtual void SetSupportsDraggableRegions(bool supports_draggable_regions) = 0;

  // Misc -------------------------------------------------------------

  // Returns the number of live WebView instances in this process.
  static size_t GetWebViewCount();

  // Sets whether web or OS-level Attribution Reporting is supported. See
  // https://github.com/WICG/attribution-reporting-api/blob/main/app_to_web.md
  virtual void SetPageAttributionSupport(
      network::mojom::AttributionSupport support) = 0;

 protected:
  ~WebView() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_VIEW_H_
