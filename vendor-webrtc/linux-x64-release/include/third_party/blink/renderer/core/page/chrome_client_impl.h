/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_CHROME_CLIENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_CHROME_CLIENT_IMPL_H_

#include <memory>

#include "base/gtest_prod_util.h"
#include "cc/input/overscroll_behavior.h"
#include "third_party/blink/public/common/widget/constants.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_navigation_policy.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/chrome_client.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "ui/base/cursor/cursor.h"

namespace ui {
class Cursor;
}

namespace blink {

class PagePopup;
class PagePopupClient;
class WebAutofillClient;
class WebViewImpl;

// Handles window-level notifications from core on behalf of a WebView.
class CORE_EXPORT ChromeClientImpl final : public ChromeClient {
 public:
  explicit ChromeClientImpl(WebViewImpl*);
  ~ChromeClientImpl() override;
  void Trace(Visitor* visitor) const override;

  // ChromeClient methods:
  WebViewImpl* GetWebView() const override;
  void ChromeDestroyed() override;
  void SetWindowRect(const gfx::Rect&, LocalFrame&) override;
  void Minimize(LocalFrame&) override;
  void Maximize(LocalFrame&) override;
  void Restore(LocalFrame&) override;
  void SetResizable(bool resizable, LocalFrame& frame) override;
  gfx::Rect RootWindowRect(LocalFrame&) override;
  void DidAccessInitialMainDocument() override;
  void FocusPage() override;
  void DidFocusPage() override;
  bool CanTakeFocus(mojom::blink::FocusType) override;
  void TakeFocus(mojom::blink::FocusType) override;
  void SetKeyboardFocusURL(Element* new_focus_element) override;
  bool SupportsDraggableRegions() override;
  void DraggableRegionsChanged() override;
  void BeginLifecycleUpdates(LocalFrame& main_frame) override;
  void RegisterForCommitObservation(CommitObserver*) override;
  void UnregisterFromCommitObservation(CommitObserver*) override;
  void WillCommitCompositorFrame() override;
  bool StartDeferringCommits(LocalFrame& main_frame,
                             base::TimeDelta timeout,
                             cc::PaintHoldingReason reason) override;
  void StopDeferringCommits(LocalFrame& main_frame,
                            cc::PaintHoldingCommitTrigger) override;
  void SetShouldThrottleFrameRate(bool flag, LocalFrame& main_frame) override;
  std::unique_ptr<cc::ScopedPauseRendering> PauseRendering(
      LocalFrame&) override;
  std::optional<int> GetMaxRenderBufferBounds(LocalFrame&) const override;
  void StartDragging(LocalFrame*,
                     const WebDragData&,
                     DragOperationsMask,
                     const SkBitmap& drag_image,
                     const gfx::Vector2d& cursor_offset,
                     const gfx::Rect& drag_obj_rect) override;
  bool AcceptsLoadDrops() const override;
  Page* CreateWindowDelegate(LocalFrame*,
                             const FrameLoadRequest&,
                             const AtomicString& name,
                             const WebWindowFeatures&,
                             network::mojom::blink::WebSandboxFlags,
                             const SessionStorageNamespaceId&,
                             bool& consumed_user_gesture) override;
  void Show(LocalFrame& frame,
            LocalFrame& opener_frame,
            NavigationPolicy navigation_policy,
            bool user_gesture) override;
  void SetOverscrollBehavior(LocalFrame& main_frame,
                             const cc::OverscrollBehavior&) override;
  void InjectScrollbarGestureScroll(
      LocalFrame& local_frame,
      const gfx::Vector2dF& delta,
      ui::ScrollGranularity granularity,
      CompositorElementId scrollable_area_element_id,
      WebInputEvent::Type injected_type) override;
  void FinishScrollFocusedEditableIntoView(
      const gfx::RectF& caret_rect_in_root_frame,
      mojom::blink::ScrollIntoViewParamsPtr params) override;
  bool ShouldReportDetailedMessageForSourceAndSeverity(
      LocalFrame&,
      mojom::blink::ConsoleMessageLevel log_level,
      const String&) override;
  void AddMessageToConsole(LocalFrame*,
                           mojom::ConsoleMessageSource,
                           mojom::ConsoleMessageLevel,
                           const String& message,
                           unsigned line_number,
                           const String& source_id,
                           const String& stack_trace) override;
  bool CanOpenBeforeUnloadConfirmPanel() override;
  bool OpenBeforeUnloadConfirmPanelDelegate(LocalFrame*,
                                            bool is_reload) override;
  // Used in tests to set a mock value for a before unload confirmation dialog
  // box. The value is cleared after being read.
  void SetBeforeUnloadConfirmPanelResultForTesting(bool result_success);

  void CloseWindow() override;
  bool OpenJavaScriptAlertDelegate(LocalFrame*, const String&) override;
  bool OpenJavaScriptConfirmDelegate(LocalFrame*, const String&) override;
  bool OpenJavaScriptPromptDelegate(LocalFrame*,
                                    const String& message,
                                    const String& default_value,
                                    String& result) override;
  bool TabsToLinks() override;
  void InvalidateContainer() override;
  void ScheduleAnimation(const LocalFrameView*,
                         base::TimeDelta delay,
                         bool urgent) override;
  gfx::Rect LocalRootToScreenDIPs(const gfx::Rect&,
                                  const LocalFrameView*) const override;
  float WindowToViewportScalar(LocalFrame*, const float) const override;
  const display::ScreenInfo& GetScreenInfo(LocalFrame&) const override;
  const display::ScreenInfos& GetScreenInfos(LocalFrame&) const override;
  float InputEventsScaleForEmulation() const override;
  void ContentsSizeChanged(LocalFrame*, const gfx::Size&) const override;
  bool DoubleTapToZoomEnabled() const override;
  void EnablePreferredSizeChangedMode() override;
  void ZoomToFindInPageRect(const gfx::Rect& rect_in_root_frame) override;
  void PageScaleFactorChanged() const override;
  float ClampPageScaleFactorToLimits(float scale) const override;
  void OutermostMainFrameScrollOffsetChanged() const override;
  void ResizeAfterLayout() const override;
  void MainFrameLayoutUpdated() const override;
  void ShowMouseOverURL(const HitTestResult&) override;
  void UpdateTooltipUnderCursor(LocalFrame&,
                                const String&,
                                TextDirection) override;
  void UpdateTooltipFromKeyboard(LocalFrame&,
                                 const String&,
                                 TextDirection,
                                 const gfx::Rect&) override;
  void ClearKeyboardTriggeredTooltip(LocalFrame&) override;
  void DispatchViewportPropertiesDidChange(
      const ViewportDescription&) const override;
  void PrintDelegate(LocalFrame*) override;
  ColorChooser* OpenColorChooser(LocalFrame*,
                                 ColorChooserClient*,
                                 const Color&) override;
  DateTimeChooser* OpenDateTimeChooser(
      LocalFrame* frame,
      DateTimeChooserClient*,
      const DateTimeChooserParameters&) override;
  ExternalDateTimeChooser* GetExternalDateTimeChooserForTesting() override;
  void OpenFileChooser(LocalFrame*, scoped_refptr<FileChooser>) override;
  void SetCursor(const ui::Cursor&, LocalFrame*) override;
  void SetCursorOverridden(bool) override;
  ui::Cursor LastSetCursorForTesting() const override;
  void SetEventListenerProperties(LocalFrame*,
                                  cc::EventListenerClass,
                                  cc::EventListenerProperties) override;
  // Informs client about the existence of handlers for scroll events so
  // appropriate scroll optimizations can be chosen.
  void SetHasScrollEventHandlers(LocalFrame*, bool has_event_handlers) override;
  void SetNeedsLowLatencyInput(LocalFrame*, bool needs_low_latency) override;
  void SetNeedsUnbufferedInputForDebugger(LocalFrame*, bool immediate) override;
  void RequestUnbufferedInputEvents(LocalFrame*) override;
  void SetTouchAction(LocalFrame*, TouchAction) override;
  void SetPanAction(LocalFrame*, mojom::blink::PanAction pan_action) override;

  void AttachRootLayer(scoped_refptr<cc::Layer>,
                       LocalFrame* local_root) override;

  cc::AnimationHost* GetCompositorAnimationHost(LocalFrame&) const override;
  cc::AnimationTimeline* GetScrollAnimationTimeline(LocalFrame&) const override;

  void EnterFullscreen(LocalFrame&,
                       const FullscreenOptions*,
                       FullscreenRequestType) override;
  void ExitFullscreen(LocalFrame&) override;
  void FullscreenElementChanged(Element* old_element,
                                Element* new_element,
                                const FullscreenOptions*,
                                FullscreenRequestType) override;

  void AnimateDoubleTapZoom(const gfx::Point& point,
                            const gfx::Rect& rect) override;

  // ChromeClient methods:
  String AcceptLanguages() override;
  void SetCursorForPlugin(const ui::Cursor&, LocalFrame*) override;
  void SetDelegatedInkMetadata(
      LocalFrame* frame,
      std::unique_ptr<gfx::DelegatedInkMetadata> metadata) override;

  // ChromeClientImpl:
  void SetNewWindowNavigationPolicy(WebNavigationPolicy);

  // FileChooser calls this function to kick pending file chooser
  // requests.
  void DidCompleteFileChooser(FileChooser& file_chooser);

  void AutoscrollStart(const gfx::PointF& viewport_point, LocalFrame*) override;
  void AutoscrollFling(const gfx::Vector2dF& velocity, LocalFrame*) override;
  void AutoscrollEnd(LocalFrame*) override;

  bool HasOpenedPopup() const override;
  PopupMenu* OpenPopupMenu(LocalFrame&, HTMLSelectElement&) override;
  PagePopup* OpenPagePopup(PagePopupClient*) override;
  void ClosePagePopup(PagePopup*) override;
  DOMWindow* PagePopupWindowForTesting() const override;

  void SetBrowserControlsState(float top_height,
                               float bottom_height,
                               bool shrinks_layout) override;
  void SetBrowserControlsShownRatio(float top_ratio,
                                    float bottom_ratio) override;

  bool ShouldOpenUIElementDuringPageDismissal(
      LocalFrame&,
      UIElementType,
      const String& dialog_message,
      Document::PageDismissalType) const override;

  // AutofillClient pass throughs:
  void DidChangeFormRelatedElementDynamically(
      LocalFrame*,
      HTMLElement*,
      WebFormRelatedChangeType) override;
  void HandleKeyboardEventOnTextField(HTMLInputElement&,
                                      KeyboardEvent&) override;
  void DidChangeValueInTextField(HTMLFormControlElement&) override;
  void DidClearValueInTextField(HTMLFormControlElement&) override;
  void DidUserChangeContentEditableContent(Element&) override;
  void DidEndEditingOnTextField(HTMLInputElement&) override;
  void OpenTextDataListChooser(HTMLInputElement&) override;
  void TextFieldDataListChanged(HTMLInputElement&) override;
  void DidChangeSelectionInSelectControl(HTMLFormControlElement&) override;
  void SelectFieldOptionsChanged(HTMLFormControlElement&) override;
  void AjaxSucceeded(LocalFrame*) override;
  void JavaScriptChangedValue(HTMLFormControlElement&,
                              const String& old_value,
                              bool was_autofilled) override;

  void ShowVirtualKeyboardOnElementFocus(LocalFrame&) override;

  gfx::Transform GetDeviceEmulationTransform() const override;

  void OnMouseDown(Node&) override;
  void DidUpdateBrowserControls() const override;

  void DidUpdateMaxSafeAreaInsets(
      const gfx::InsetsF& max_safe_area_insets) const override;

  gfx::Vector2dF ElasticOverscroll() const override;

  void RegisterPopupOpeningObserver(PopupOpeningObserver*) override;
  void UnregisterPopupOpeningObserver(PopupOpeningObserver*) override;
  void NotifyPopupOpeningObservers() const override;

  viz::FrameSinkId GetFrameSinkId(LocalFrame*) override;

  void RequestDecode(LocalFrame*,
                     const cc::DrawImage&,
                     base::OnceCallback<void(bool)>) override;

  void NotifyPresentationTime(LocalFrame& frame,
                              ReportTimeCallback callback) override;

  void RequestBeginMainFrameNotExpected(LocalFrame& frame,
                                        bool request) override;

  void DidUpdateTextAutosizerPageInfo(
      const mojom::blink::TextAutosizerPageInfo& page_info) override;

  int GetLayerTreeId(LocalFrame& frame) override;

  void DocumentDetached(Document&) override;

  double UserZoomFactor(LocalFrame* frame) const override;

  void FormElementReset(HTMLFormElement& element) override;

  void PasswordFieldReset(HTMLInputElement& element) override;

  float ZoomFactorForViewportLayout() override;

  void OnFirstContentfulPaint() override;

 private:
  bool IsChromeClientImpl() const override { return true; }

  void SetCursorInternal(const ui::Cursor&, LocalFrame*);

  // Returns WebAutofillClient associated with the WebLocalFrame. This takes and
  // returns nullable.
  WebAutofillClient* AutofillClientFromFrame(LocalFrame*);

  // Returns a copy of |pending_rect|, adjusted for available screen area
  // constraints. This is used to synchronously estimate, or preemptively apply,
  // anticipated browser- or OS-imposed constraints. Note: This applies legacy
  // same-screen constraints; use un-adjusted values if permission-gated
  // cross-screen window placement requests may be honored.
  // TODO(crbug.com/897300): Use permission state for better sync estimates or
  // store unadjusted pending window rects if that will not break many sites.
  gfx::Rect AdjustWindowRectForDisplay(
      const gfx::Rect& pending_rect,
      LocalFrame& frame,
      int minimum_size = blink::kMinimumWindowSize);

  WebViewImpl* web_view_;  // Weak pointer.
  HeapHashSet<WeakMember<PopupOpeningObserver>> popup_opening_observers_;
  Vector<scoped_refptr<FileChooser>> file_chooser_queue_;
  ui::Cursor last_set_mouse_cursor_for_testing_;
  bool cursor_overridden_;
  Member<ExternalDateTimeChooser> external_date_time_chooser_;
  bool did_request_non_empty_tool_tip_;
  std::optional<bool> before_unload_confirm_panel_result_for_testing_;
  HeapHashSet<WeakMember<CommitObserver>> commit_observers_;

  FRIEND_TEST_ALL_PREFIXES(FileChooserQueueTest, DerefQueuedChooser);
};

template <>
struct DowncastTraits<ChromeClientImpl> {
  static bool AllowFrom(const ChromeClient& client) {
    return client.IsChromeClientImpl();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_CHROME_CLIENT_IMPL_H_
