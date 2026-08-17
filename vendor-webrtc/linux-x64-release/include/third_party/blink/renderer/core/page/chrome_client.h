/*
 * Copyright (C) 2006, 2007, 2008, 2009, 2010, 2011, 2012 Apple, Inc. All rights
 * reserved.
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
 * Copyright (C) 2012 Samsung Electronics. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_CHROME_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_CHROME_CLIENT_H_

#include <memory>

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "cc/input/event_listener_properties.h"
#include "cc/input/overscroll_behavior.h"
#include "cc/paint/draw_image.h"
#include "cc/trees/paint_holding_commit_trigger.h"
#include "cc/trees/paint_holding_reason.h"
#include "components/viz/common/surfaces/frame_sink_id.h"
#include "third_party/blink/public/common/dom_storage/session_storage_namespace_id.h"
#include "third_party/blink/public/common/input/web_gesture_event.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/page/drag_operation.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/external_date_time_chooser.h"
#include "third_party/blink/renderer/core/html/forms/popup_menu.h"
#include "third_party/blink/renderer/core/loader/frame_loader.h"
#include "third_party/blink/renderer/core/loader/navigation_policy.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "ui/gfx/delegated_ink_metadata.h"
#include "ui/gfx/geometry/transform.h"
#include "ui/gfx/geometry/vector2d_f.h"

// To avoid conflicts with the CreateWindow macro from the Windows SDK...
#undef CreateWindow

namespace cc {
class AnimationHost;
class AnimationTimeline;
struct ElementId;
class Layer;
struct OverscrollBehavior;
class ScopedPauseRendering;
}

namespace display {
struct ScreenInfo;
struct ScreenInfos;
}  // namespace display

namespace ui {
class Cursor;
}

namespace viz {
struct FrameTimingDetails;
}

namespace blink {

class ColorChooser;
class ColorChooserClient;
class DateTimeChooser;
class DateTimeChooserClient;
class Element;
class FileChooser;
class Frame;
class FullscreenOptions;
class HTMLFormControlElement;
class HTMLFormElement;
class HTMLInputElement;
class HTMLSelectElement;
class HitTestLocation;
class HitTestResult;
class KeyboardEvent;
class LocalFrame;
class LocalFrameView;
class Node;
class Page;
class PagePopup;
class PagePopupClient;
class PopupOpeningObserver;
class WebDragData;
class WebViewImpl;

enum class FullscreenRequestType;

struct DateTimeChooserParameters;
struct FrameLoadRequest;
struct ViewportDescription;
struct WebWindowFeatures;

namespace mojom {
namespace blink {
class TextAutosizerPageInfo;
}
}  // namespace mojom

using CompositorElementId = cc::ElementId;

class CORE_EXPORT ChromeClient : public GarbageCollected<ChromeClient> {
 public:
  ChromeClient(const ChromeClient&) = delete;
  ChromeClient& operator=(const ChromeClient&) = delete;
  virtual ~ChromeClient() = default;

  virtual WebViewImpl* GetWebView() const = 0;

  // Converts the scalar value from window coordinates to viewport scale.
  virtual float WindowToViewportScalar(LocalFrame*,
                                       const float value) const = 0;

  virtual bool IsPopup() { return false; }

  virtual Element* GetPopupClientOwnerElement() { return nullptr; }

  virtual void ChromeDestroyed() = 0;

  virtual void SetWindowRect(const gfx::Rect&, LocalFrame&) = 0;

  virtual void Minimize(LocalFrame&) = 0;
  virtual void Maximize(LocalFrame&) = 0;
  virtual void Restore(LocalFrame&) = 0;
  virtual void SetResizable(bool resizable, LocalFrame&) = 0;

  // For non-composited WebViews that exist to contribute to a "parent" WebView
  // painting. This informs the client of the area that needs to be redrawn.
  virtual void InvalidateContainer() = 0;

  // Converts the rect from local root coordinates (using the local root of the
  // given LocalFrameView) to screen coordinates. Performs the visual viewport
  // transform.
  virtual gfx::Rect LocalRootToScreenDIPs(const gfx::Rect&,
                                          const LocalFrameView*) const = 0;

  void ScheduleAnimation(const LocalFrameView* view) {
    ScheduleAnimation(view, base::TimeDelta(), /*urgent=*/false);
  }
  void ScheduleAnimation(const LocalFrameView* view, base::TimeDelta delay) {
    ScheduleAnimation(view, delay, /*urgent=*/false);
  }

  virtual void ScheduleAnimation(const LocalFrameView* local_frame_view,
                                 base::TimeDelta delay,
                                 bool urgent) = 0;

  // Tells the browser that another page has accessed the DOM of the initial
  // empty document of a main frame.
  virtual void DidAccessInitialMainDocument() = 0;

  // This gives the rect of the top level window that the given LocalFrame is a
  // part of.
  virtual gfx::Rect RootWindowRect(LocalFrame&) = 0;

  virtual void FocusPage() = 0;
  virtual void DidFocusPage() = 0;

  virtual bool CanTakeFocus(mojom::blink::FocusType) = 0;
  virtual void TakeFocus(mojom::blink::FocusType) = 0;

  virtual void SetKeyboardFocusURL(Element*) {}

  // Returns true if the page should support drag regions via the app-region
  // CSS property.
  virtual bool SupportsDraggableRegions() = 0;

  // Sends the draggable regions defined by the app-region CSS property to the
  // browser.
  virtual void DraggableRegionsChanged() = 0;

  // Allow document lifecycle updates to be run in order to produce composited
  // outputs. Updates are blocked from occurring during loading navigation in
  // order to prevent contention and allow Blink to proceed more quickly. This
  // signals that enough progress has been made and document lifecycle updates
  // are desirable. This will allow visual updates to occur unless the caller
  // also uses StartDeferringCommits().
  //
  // This may only be called for the main frame, and takes it as
  // reference to make it clear that callers may only call this while a local
  // main frame is present and the values does not persist between instances of
  // local main frames.
  virtual void BeginLifecycleUpdates(LocalFrame& main_frame) = 0;

  // Notifies clients immediately before a newly committed main frame is pushed
  // to the compositor thread.
  struct CORE_EXPORT CommitObserver : public GarbageCollectedMixin {
    virtual void WillCommitCompositorFrame() {}

   protected:
    virtual ~CommitObserver() = default;
  };

  virtual void RegisterForCommitObservation(CommitObserver*) = 0;
  virtual void UnregisterFromCommitObservation(CommitObserver*) = 0;

  virtual void WillCommitCompositorFrame() = 0;

  virtual bool StartDeferringCommits(LocalFrame& main_frame,
                                     base::TimeDelta timeout,
                                     cc::PaintHoldingReason reason) = 0;
  virtual void StopDeferringCommits(LocalFrame& main_frame,
                                    cc::PaintHoldingCommitTrigger) = 0;
  virtual void SetShouldThrottleFrameRate(bool flag,
                                          LocalFrame& main_frame) = 0;

  virtual std::unique_ptr<cc::ScopedPauseRendering> PauseRendering(
      LocalFrame& main_frame) = 0;

  // Returns the maximum bounds for buffers allocated for rasterization and
  // compositing.
  // Returns null if the compositing stack has not been initialized yet.
  // |frame| must be a local frame.
  virtual std::optional<int> GetMaxRenderBufferBounds(
      LocalFrame& frame) const = 0;

  // Start a system drag and drop operation.
  //
  // The `cursor_offset` is the offset of the drag-point from the top-left of
  // `drag_image`, which may not be the same as the top-left of
  // `drag_obj_rect`.  For details, see the function header comment for:
  // `blink::DragController::StartDrag()`.
  virtual void StartDragging(LocalFrame*,
                             const WebDragData&,
                             DragOperationsMask,
                             const SkBitmap& drag_image,
                             const gfx::Vector2d& cursor_offset,
                             const gfx::Rect& drag_obj_rect) = 0;
  virtual bool AcceptsLoadDrops() const = 0;

  // The LocalFrame pointer provides the ChromeClient with context about which
  // LocalFrame wants to create the new Page. Also, the newly created window
  // should not be shown to the user until the ChromeClient of the newly
  // created Page has its show method called.
  // The FrameLoadRequest parameter is only for ChromeClient to check if the
  // request could be fulfilled. The ChromeClient should not load the request.
  Page* CreateWindow(LocalFrame*,
                     const FrameLoadRequest&,
                     const AtomicString& frame_name,
                     const WebWindowFeatures&,
                     network::mojom::blink::WebSandboxFlags,
                     const SessionStorageNamespaceId&,
                     bool& consumed_user_gesture);

  // Show a previously created Page that was created via CreateWindow. This
  // should only be called once the newly created window when it is ready to be
  // shown. Under some circumstances CreateWindow's implementation may return a
  // previously shown page. Calling this method should still work and the
  // browser will discard the unnecessary show request.
  virtual void Show(LocalFrame& frame,
                    LocalFrame& opener_frame,
                    NavigationPolicy navigation_policy,
                    bool consumed_user_gesture) = 0;

  // For a scrollbar scroll action, injects a gesture event of |injected_type|
  // to be dispatched at a later point in time. |injected_type| is required to
  // be one of GestureScroll{Begin,Update,End}. If the main thread is currently
  // handling an input event, the gesture will be dispatched immediately after
  // the current event is finished being processed.
  // If there is no input event being handled, the gesture is queued up
  // on the main thread's input event queue.
  // The dispatched gesture will scroll the ScrollableArea identified by
  // |scrollable_area_element_id| by the given delta+granularity.
  // See also InputHandlerProxy::InjectScrollbarGestureScroll() which may
  // shortcut callers of this function for composited scrollbars.
  virtual void InjectScrollbarGestureScroll(
      LocalFrame& local_frame,
      const gfx::Vector2dF& delta,
      ui::ScrollGranularity granularity,
      CompositorElementId scrollable_area_element_id,
      WebInputEvent::Type injected_type) {}

  // Finishes a ScrollIntoView for a focused editable element by performing a
  // view-level reveal. That is, when an embedder requests to reveal a focused
  // editable, the editable is first ScrollIntoView'ed in the layout tree to
  // ensure it's visible in the outermost document but stops short of scrolling
  // the outermost frame. This method will then perform a platform-specific
  // reveal of the editable, e.g. by animating a scroll and zoom in to a
  // legible scale. This should only be called in a WebView where the main
  // frame is local and outermost.
  virtual void FinishScrollFocusedEditableIntoView(
      const gfx::RectF& caret_rect_in_root_frame,
      mojom::blink::ScrollIntoViewParamsPtr params) {}

  // Set the browser's behavior when overscroll happens, e.g. whether to glow
  // or navigate. This may only be called for the main frame, and takes it as
  // reference to make it clear that callers may only call this while a local
  // main frame is present and the values do not persist between instances of
  // local main frames.
  virtual void SetOverscrollBehavior(LocalFrame& main_frame,
                                     const cc::OverscrollBehavior&) = 0;

  virtual bool ShouldReportDetailedMessageForSourceAndSeverity(
      LocalFrame&,
      mojom::blink::ConsoleMessageLevel log_level,
      const String& source) = 0;
  virtual void AddMessageToConsole(LocalFrame*,
                                   mojom::ConsoleMessageSource,
                                   mojom::ConsoleMessageLevel,
                                   const String& message,
                                   unsigned line_number,
                                   const String& source_id,
                                   const String& stack_trace) = 0;

  virtual bool CanOpenBeforeUnloadConfirmPanel() = 0;
  bool OpenBeforeUnloadConfirmPanel(const String& message,
                                    LocalFrame*,
                                    bool is_reload);

  virtual void CloseWindow() = 0;

  bool OpenJavaScriptAlert(LocalFrame*, const String&);
  bool OpenJavaScriptConfirm(LocalFrame*, const String&);
  bool OpenJavaScriptPrompt(LocalFrame*,
                            const String& message,
                            const String& default_value,
                            String& result);
  virtual bool TabsToLinks() = 0;

  virtual const display::ScreenInfo& GetScreenInfo(LocalFrame& frame) const = 0;
  virtual const display::ScreenInfos& GetScreenInfos(
      LocalFrame& frame) const = 0;

  virtual void SetCursor(const ui::Cursor&, LocalFrame* local_root) = 0;
  virtual void SetCursorOverridden(bool) = 0;

  virtual void AutoscrollStart(const gfx::PointF& position, LocalFrame*) {}
  virtual void AutoscrollFling(const gfx::Vector2dF& velocity, LocalFrame*) {}
  virtual void AutoscrollEnd(LocalFrame*) {}

  virtual ui::Cursor LastSetCursorForTesting() const = 0;
  Node* LastSetTooltipNodeForTesting() const {
    return last_mouse_over_node_.Get();
  }

  virtual void SetCursorForPlugin(const ui::Cursor&, LocalFrame*) = 0;

  // Returns the scale used to convert incoming input events while emulating
  // device metics.
  virtual float InputEventsScaleForEmulation() const { return 1; }

  virtual void DispatchViewportPropertiesDidChange(
      const ViewportDescription&) const {}

  virtual bool DoubleTapToZoomEnabled() const { return false; }

  virtual void EnablePreferredSizeChangedMode() {}

  virtual void ZoomToFindInPageRect(const gfx::Rect&) {}

  virtual void ContentsSizeChanged(LocalFrame*, const gfx::Size&) const = 0;
  // Call during pinch gestures, or when page-scale changes on main-frame load.
  virtual void PageScaleFactorChanged() const {}
  virtual float ClampPageScaleFactorToLimits(float scale) const {
    return scale;
  }
  virtual void OutermostMainFrameScrollOffsetChanged() const = 0;
  virtual void ResizeAfterLayout() const {}
  virtual void MainFrameLayoutUpdated() const {}

  void MouseDidMoveOverElement(LocalFrame&,
                               const HitTestLocation&,
                               const HitTestResult&);
  virtual void UpdateTooltipUnderCursor(LocalFrame&,
                                        const String&,
                                        TextDirection) = 0;
  void ElementFocusedFromKeypress(LocalFrame&, const Element*);
  // This function allows us to trigger a tooltip to show from a keypress. The
  // tooltip will be positioned in the gfx::Rect passed by parameter. That rect
  // corresponds to the focused element's bounds, which are in viewport
  // coordinates at this point. They will be converted to enclosed DIPS before
  // being passed to the browser process.
  virtual void UpdateTooltipFromKeyboard(LocalFrame&,
                                         const String&,
                                         TextDirection,
                                         const gfx::Rect&) = 0;
  virtual void ClearKeyboardTriggeredTooltip(LocalFrame&) = 0;
  void ClearToolTip(LocalFrame&);
  String GetLastToolTipTextForTesting() {
    return current_tool_tip_text_for_test_;
  }

  bool Print(LocalFrame*);

  virtual ColorChooser* OpenColorChooser(LocalFrame*,
                                         ColorChooserClient*,
                                         const Color&) = 0;

  // This function is used for:
  //  - Mandatory date/time choosers if InputMultipleFieldsUI flag is not set
  //  - Date/time choosers for types for which
  //    LayoutTheme::SupportsCalendarPicker returns true, if
  //    InputMultipleFieldsUI flag is set
  //  - <datalist> UI for date/time input types regardless of
  //    InputMultipleFieldsUI flag
  // |LocalFrame| should not be null.
  virtual DateTimeChooser* OpenDateTimeChooser(
      LocalFrame*,
      DateTimeChooserClient*,
      const DateTimeChooserParameters&) = 0;
  virtual ExternalDateTimeChooser* GetExternalDateTimeChooserForTesting() {
    return nullptr;
  }

  virtual void OpenTextDataListChooser(HTMLInputElement&) = 0;

  virtual void OpenFileChooser(LocalFrame*, scoped_refptr<FileChooser>) = 0;

  // Pass nullptr as the cc::Layer to detach the root layer.
  // This sets the cc::Layer for the LocalFrame's WebWidget, if it has
  // one. Otherwise it sets it for the WebViewImpl.
  virtual void AttachRootLayer(scoped_refptr<cc::Layer>,
                               LocalFrame* local_root) = 0;

  virtual cc::AnimationHost* GetCompositorAnimationHost(LocalFrame&) const = 0;

  virtual cc::AnimationTimeline* GetScrollAnimationTimeline(
      LocalFrame&) const = 0;

  virtual void EnterFullscreen(LocalFrame&,
                               const FullscreenOptions*,
                               FullscreenRequestType) {}
  virtual void ExitFullscreen(LocalFrame&) {}
  virtual void FullscreenElementChanged(Element* old_element,
                                        Element* new_element,
                                        const FullscreenOptions* options,
                                        FullscreenRequestType) {}

  virtual void AnimateDoubleTapZoom(const gfx::Point& point,
                                    const gfx::Rect& rect) {}

  // The client keeps track of which touch/mousewheel event types have handlers,
  // and if they do, whether the handlers are passive and/or blocking. This
  // allows the client to know which optimizations can be used for the
  // associated event classes.
  virtual void SetEventListenerProperties(LocalFrame*,
                                          cc::EventListenerClass,
                                          cc::EventListenerProperties) = 0;

  virtual void SetHasScrollEventHandlers(LocalFrame*, bool) = 0;
  virtual void SetNeedsLowLatencyInput(LocalFrame*, bool) = 0;
  virtual void SetNeedsUnbufferedInputForDebugger(LocalFrame*, bool) = 0;
  virtual void RequestUnbufferedInputEvents(LocalFrame*) = 0;
  virtual void SetTouchAction(LocalFrame*, TouchAction) = 0;
  virtual void SetPanAction(LocalFrame*,
                            mojom::blink::PanAction pan_action) = 0;

  // Checks if there is an opened popup, called by LayoutMenuList::showPopUp().
  virtual bool HasOpenedPopup() const = 0;
  virtual PopupMenu* OpenPopupMenu(LocalFrame&, HTMLSelectElement&) = 0;
  virtual PagePopup* OpenPagePopup(PagePopupClient*) = 0;
  virtual void ClosePagePopup(PagePopup*) = 0;
  virtual DOMWindow* PagePopupWindowForTesting() const = 0;

  virtual void SetBrowserControlsState(float top_height,
                                       float bottom_height,
                                       bool shrinks_layout) {}
  virtual void SetBrowserControlsShownRatio(float top_ratio,
                                            float bottom_ratio) {}

  virtual String AcceptLanguages() = 0;

  enum class UIElementType {
    kAlertDialog = 0,
    kConfirmDialog = 1,
    kPromptDialog = 2,
    kPrintDialog = 3,
    kPopup = 4
  };
  virtual bool ShouldOpenUIElementDuringPageDismissal(
      LocalFrame&,
      UIElementType,
      const String&,
      Document::PageDismissalType) const {
    return false;
  }

  virtual bool IsIsolatedSVGChromeClient() const { return false; }

  virtual gfx::Size MinimumWindowSize() const { return gfx::Size(100, 100); }

  virtual bool IsChromeClientImpl() const { return false; }

  virtual void DidChangeFormRelatedElementDynamically(
      LocalFrame*,
      HTMLElement*,
      WebFormRelatedChangeType) {}
  virtual void DidChangeValueInTextField(HTMLFormControlElement&) {}
  virtual void DidClearValueInTextField(HTMLFormControlElement&) {}
  virtual void DidUserChangeContentEditableContent(Element&) {}
  virtual void DidEndEditingOnTextField(HTMLInputElement&) {}
  virtual void HandleKeyboardEventOnTextField(HTMLInputElement&,
                                              KeyboardEvent&) {}
  virtual void TextFieldDataListChanged(HTMLInputElement&) {}

  // Called when the selected option of a <select> control is changed as a
  // result of user activation - see
  // https://html.spec.whatwg.org/multipage/interaction.html#tracking-user-activation
  virtual void DidChangeSelectionInSelectControl(HTMLFormControlElement&) {}

  virtual void SelectFieldOptionsChanged(HTMLFormControlElement&) {}
  virtual void AjaxSucceeded(LocalFrame*) {}
  // Called when the value of `element` has been changed by JavaScript.
  // `old_value` contains the value before being changed.
  // `was_autofilled` is the state of the field prior to the JS change.
  // Only called if there is an observable change in the actual value, i.e.
  // JavaScript setting it to the current value will not trigger this.
  virtual void JavaScriptChangedValue(HTMLFormControlElement&,
                                      const String& old_value,
                                      bool was_autofilled) {}

  // Input method editor related functions.
  virtual void ShowVirtualKeyboardOnElementFocus(LocalFrame&) {}

  virtual gfx::Transform GetDeviceEmulationTransform() const {
    return gfx::Transform();
  }

  virtual void OnMouseDown(Node&) {}

  virtual void DidUpdateBrowserControls() const {}

  virtual void DidUpdateMaxSafeAreaInsets(
      const gfx::InsetsF& max_safe_area_insets) const {}

  virtual void RegisterPopupOpeningObserver(PopupOpeningObserver*) = 0;
  virtual void UnregisterPopupOpeningObserver(PopupOpeningObserver*) = 0;
  virtual void NotifyPopupOpeningObservers() const = 0;

  virtual gfx::Vector2dF ElasticOverscroll() const { return gfx::Vector2dF(); }

  virtual void InstallSupplements(LocalFrame&);

  virtual viz::FrameSinkId GetFrameSinkId(LocalFrame*) {
    return viz::FrameSinkId();
  }

  virtual void RequestDecode(LocalFrame*,
                             const cc::DrawImage& image,
                             base::OnceCallback<void(bool)> callback) {
    std::move(callback).Run(false);
  }

  // The `callback` will be fired when the corresponding renderer frame for the
  // `frame` is presented in the display compositor. If there is no update in
  // the frame to be presented, the `callback` will run with the time of the
  // failure.
  using ReportTimeCallback =
      base::OnceCallback<void(const viz::FrameTimingDetails&)>;
  virtual void NotifyPresentationTime(LocalFrame& frame,
                                      ReportTimeCallback callback) {}

  // Enable or disable BeginMainFrameNotExpected signals from the compositor of
  // the local root of |frame|. These signals would be consumed by the blink
  // scheduler.
  virtual void RequestBeginMainFrameNotExpected(LocalFrame& frame,
                                                bool request) = 0;

  // A stable numeric Id for |frame|'s local root's compositor. For
  // tracing/debugging purposes.
  virtual int GetLayerTreeId(LocalFrame& frame) = 0;

  virtual void Trace(Visitor*) const;

  virtual void DidUpdateTextAutosizerPageInfo(
      const mojom::blink::TextAutosizerPageInfo&) {}

  virtual void DocumentDetached(Document&) {}

  // Return the user's zoom factor which is different from the typical usage
  // of "zoom factor" in blink (e.g., |LocalFrame::LayoutZoomFactor()|) which
  // includes CSS zoom and the device scale factor (if use-zoom-for-dsf is
  // enabled). This only includes the zoom initiated by the user (ctrl +/-).
  virtual double UserZoomFactor(LocalFrame* frame) const { return 1; }

  virtual void SetDelegatedInkMetadata(
      LocalFrame* frame,
      std::unique_ptr<gfx::DelegatedInkMetadata> metadata) {}

  virtual void FormElementReset(HTMLFormElement& element) {}

  virtual void PasswordFieldReset(HTMLInputElement& element) {}

  virtual float ZoomFactorForViewportLayout() { return 1; }

  virtual void OnFirstContentfulPaint() {}

 protected:
  ChromeClient() = default;

  virtual void ShowMouseOverURL(const HitTestResult&) = 0;
  virtual bool OpenBeforeUnloadConfirmPanelDelegate(LocalFrame*,
                                                    bool is_reload) = 0;
  virtual bool OpenJavaScriptAlertDelegate(LocalFrame*, const String&) = 0;
  virtual bool OpenJavaScriptConfirmDelegate(LocalFrame*, const String&) = 0;
  virtual bool OpenJavaScriptPromptDelegate(LocalFrame*,
                                            const String& message,
                                            const String& default_value,
                                            String& result) = 0;
  virtual void PrintDelegate(LocalFrame*) = 0;
  virtual Page* CreateWindowDelegate(LocalFrame*,
                                     const FrameLoadRequest&,
                                     const AtomicString& frame_name,
                                     const WebWindowFeatures&,
                                     network::mojom::blink::WebSandboxFlags,
                                     const SessionStorageNamespaceId&,
                                     bool& consumed_user_gesture) = 0;

 private:
  bool CanOpenUIElementIfDuringPageDismissal(Frame& main_frame,
                                             UIElementType,
                                             const String& message);
  void UpdateTooltipUnderCursor(LocalFrame&,
                                const HitTestLocation&,
                                const HitTestResult&);

  WeakMember<Node> last_mouse_over_node_;
  PhysicalOffset last_tool_tip_point_;
  String last_tool_tip_text_;
  // |last_tool_tip_text_| is kept even if ClearToolTip is called. This is for
  // the tooltip text that is cleared when ClearToolTip is called.
  String current_tool_tip_text_for_test_;

  FRIEND_TEST_ALL_PREFIXES(ChromeClientTest, UpdateTooltipUnderCursorFlood);
  FRIEND_TEST_ALL_PREFIXES(ChromeClientTest,
                           UpdateTooltipUnderCursorEmptyString);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_CHROME_CLIENT_H_
