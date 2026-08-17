/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_FRAME_WIDGET_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_FRAME_WIDGET_IMPL_H_

#include "base/functional/function_ref.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "base/types/optional_ref.h"
#include "base/types/pass_key.h"
#include "base/unguessable_token.h"
#include "build/build_config.h"
#include "cc/input/browser_controls_offset_tag_modifications.h"
#include "cc/input/event_listener_properties.h"
#include "cc/input/overscroll_behavior.h"
#include "cc/trees/layer_tree_host.h"
#include "cc/trees/paint_holding_reason.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/viz/public/mojom/hit_test/input_target_client.mojom-blink.h"
#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/public/common/input/web_gesture_device.h"
#include "third_party/blink/public/mojom/device_posture/device_posture_provider.mojom-blink.h"
#include "third_party/blink/public/mojom/drag/drag.mojom-blink.h"
#include "third_party/blink/public/mojom/input/ime_host.mojom-blink.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/input/stylus_writing_gesture.mojom-blink.h"
#include "third_party/blink/public/mojom/manifest/display_mode.mojom-blink.h"
#include "third_party/blink/public/mojom/page/widget.mojom-blink.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_drag_data.h"
#include "third_party/blink/public/web/web_frame_widget.h"
#include "third_party/blink/public/web/web_meaningful_layout.h"
#include "third_party/blink/renderer/core/clipboard/data_object.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/exported/web_page_popup_impl.h"
#include "third_party/blink/renderer/core/frame/animation_frame_timing_monitor.h"
#include "third_party/blink/renderer/core/frame/web_local_frame_impl.h"
#include "third_party/blink/renderer/core/page/drag_controller.h"
#include "third_party/blink/renderer/core/page/event_with_hit_test_results.h"
#include "third_party/blink/renderer/core/page/viewport_description.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/graphics/apply_viewport_changes.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_image.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/widget/frame_widget.h"
#include "third_party/blink/renderer/platform/widget/input/widget_base_input_handler.h"
#include "third_party/blink/renderer/platform/widget/widget_base_client.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/base/dragdrop/mojom/drag_drop_types.mojom-shared.h"
#include "ui/base/mojom/menu_source_type.mojom-blink-forward.h"
#include "ui/base/mojom/window_show_state.mojom-blink-forward.h"
#include "ui/gfx/ca_layer_result.h"

namespace gfx {
class Point;
class PointF;
}  // namespace gfx

namespace blink {
class AnimationWorkletMutatorDispatcherImpl;
class HitTestResult;
class HTMLPlugInElement;
class Page;
class PaintWorkletPaintDispatcher;
class RemoteFrame;
class WebLocalFrameImpl;
class WebPlugin;
class WebViewImpl;
class WidgetBase;
class WidgetEventHandler;
class ScreenMetricsEmulator;

// Implements WebFrameWidget for both main frames and child local root frame
// (OOPIF).
class CORE_EXPORT WebFrameWidgetImpl
    : public GarbageCollected<WebFrameWidgetImpl>,
      public WebFrameWidget,
      public WidgetBaseClient,
      public mojom::blink::FrameWidget,
      public viz::mojom::blink::InputTargetClient,
      public mojom::blink::FrameWidgetInputHandler,
      public FrameWidget,
      public AnimationFrameTimingMonitor::Client,
      public WidgetEventHandler {
 public:
  struct PromiseCallbacks {
    base::OnceCallback<void(base::TimeTicks)> swap_time_callback;
    base::OnceCallback<void(const viz::FrameTimingDetails&)>
        presentation_time_callback;
#if BUILDFLAG(IS_APPLE)
    base::OnceCallback<void(gfx::CALayerResult)>
        core_animation_error_code_callback;
#endif
    bool IsEmpty() {
      return (!swap_time_callback &&
#if BUILDFLAG(IS_APPLE)
              !core_animation_error_code_callback &&
#endif
              !presentation_time_callback);
    }
  };

  WebFrameWidgetImpl(
      base::PassKey<WebLocalFrame>,
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
      bool is_for_scalable_page);
  ~WebFrameWidgetImpl() override;

  virtual void Trace(Visitor*) const;

  // Shutdown the widget.
  void Close(DetachReason detach_reason);

  // Returns the WebFrame that this widget is attached to. It will be a local
  // root since only local roots have a widget attached.
  WebLocalFrameImpl* LocalRootImpl() const { return local_root_.Get(); }

  // Returns the bounding box of the block type node touched by the WebPoint.
  gfx::Rect ComputeBlockBound(const gfx::Point& point_in_root_frame,
                              bool ignore_clipping) const;

  virtual void BindLocalRoot(WebLocalFrame&);

  // If this widget is for the top most main frame. This is different than
  // |ForMainFrame| because |ForMainFrame| could return true but this method
  // returns false. If this widget is a MainFrame widget embedded in another
  // widget, for example embedding a <webview>.
  bool ForTopMostMainFrame() const;

  // Returns true if this widget is for a local root that is a child frame,
  // false otherwise.
  bool ForSubframe() const { return is_for_child_local_root_; }

  // Opposite of |ForSubframe|. If this widget is for the local main frame.
  bool ForMainFrame() const { return !ForSubframe(); }

  // Called when the intrinsic size of the owning container is changing its
  // size. This should only be called when `ForSubframe` is true.
  void IntrinsicSizingInfoChanged(mojom::blink::IntrinsicSizingInfoPtr);

  void AutoscrollStart(const gfx::PointF& position);
  void AutoscrollFling(const gfx::Vector2dF& position);
  void AutoscrollEnd();

  bool HandleCurrentKeyboardEvent();

  // Creates or returns cached mutator dispatcher. This usually requires a
  // round trip to the compositor. The returned WeakPtr must only be
  // dereferenced on the output |mutator_task_runner|.
  base::WeakPtr<AnimationWorkletMutatorDispatcherImpl>
  EnsureCompositorMutatorDispatcher(
      scoped_refptr<base::SingleThreadTaskRunner> mutator_task_runner);

  // TODO: consider merge the input and return value to be one parameter.
  // Creates or returns cached paint dispatcher. The returned WeakPtr must only
  // be dereferenced on the output |paint_task_runner|.
  base::WeakPtr<PaintWorkletPaintDispatcher> EnsureCompositorPaintDispatcher(
      scoped_refptr<base::SingleThreadTaskRunner>* paint_task_runner);

  HitTestResult CoreHitTestResultAt(const gfx::PointF&);

  // Registers callbacks for the corresponding renderer frame: `swap_callback`
  // is fired with the submission (aka swap) timestamp when the frame is
  // submitted to Viz; `presentation_callback` is fired with the presentation
  // timestamp after the frame is presented to the user.
  void NotifySwapAndPresentationTimeForTesting(PromiseCallbacks callbacks);

  // Process the input event, invoking the callback when complete. This
  // method will call the callback synchronously.
  void ProcessInputEventSynchronouslyForTesting(
      const WebCoalescedInputEvent&,
      WidgetBaseInputHandler::HandledEventCallback);

  // Overrides `browser_controls_params.top_controls_height` of the properties
  // passed to `UpdateVisualProperties()`. Passing `std::nullopt` disables the
  // override.
  void SetBrowserControlsTopHeightOverride(std::optional<float> height);

  // FrameWidget overrides.
  cc::AnimationHost* AnimationHost() const final;
  cc::AnimationTimeline* ScrollAnimationTimeline() const final;
  void SetOverscrollBehavior(
      const cc::OverscrollBehavior& overscroll_behavior) final;
  void RequestAnimationAfterDelay(const base::TimeDelta&, bool urgent) final;
  void SetRootLayer(scoped_refptr<cc::Layer>) override;
  void RequestDecode(const cc::DrawImage&,
                     base::OnceCallback<void(bool)>) override;
  void RequestBeginMainFrameNotExpected(bool request) final;
  int GetLayerTreeId() final;
  const cc::LayerTreeSettings* GetLayerTreeSettings() final;
  void UpdateBrowserControlsState(
      cc::BrowserControlsState constraints,
      cc::BrowserControlsState current,
      bool animate,
      base::optional_ref<const cc::BrowserControlsOffsetTagModifications>
          offset_tag_modifications) final;
  void SetEventListenerProperties(cc::EventListenerClass,
                                  cc::EventListenerProperties) final;
  cc::EventListenerProperties EventListenerProperties(
      cc::EventListenerClass) const final;
  mojom::blink::DisplayMode DisplayMode() const override;
  ui::mojom::blink::WindowShowState WindowShowState() const override;
  bool Resizable() const override;
  const std::vector<gfx::Rect>& ViewportSegments() const override;
  void SetDelegatedInkMetadata(
      std::unique_ptr<gfx::DelegatedInkMetadata> metadata) final;
  void InjectScrollbarGestureScroll(const gfx::Vector2dF& delta,
                                    ui::ScrollGranularity granularity,
                                    cc::ElementId scrollable_area_element_id,
                                    WebInputEvent::Type injected_type) override;
  void DidChangeCursor(const ui::Cursor&) override;
  void GetCompositionCharacterBoundsInWindow(
      Vector<gfx::Rect>* bounds_in_dips) override;
  // Return the last calculated cursor anchor info.
  mojom::blink::InputCursorAnchorInfoPtr& GetLastCursorAnchorInfoForTesting();
  bool HasImeRenderWidgetHost() const override {
    return !!ime_render_widget_host_;
  }
  void UpdateCursorAnchorInfo(bool update_requested) override;
  gfx::Range CompositionRange() override;
  WebTextInputInfo TextInputInfo() override;
  ui::mojom::VirtualKeyboardVisibilityRequest
  GetLastVirtualKeyboardVisibilityRequest() override;
  void GetEditContextBoundsInWindow(
      std::optional<gfx::Rect>* control_bounds,
      std::optional<gfx::Rect>* selection_bounds) override;
  int32_t ComputeWebTextInputNextPreviousFlags() override;
  void ResetVirtualKeyboardVisibilityRequest() override;
  bool GetSelectionBoundsInWindow(gfx::Rect* focus,
                                  gfx::Rect* anchor,
                                  gfx::Rect* bounding_box,
                                  base::i18n::TextDirection* focus_dir,
                                  base::i18n::TextDirection* anchor_dir,
                                  bool* is_anchor_first) override;
  void ClearTextInputState() override;

  bool SetComposition(const String& text,
                      const Vector<ui::ImeTextSpan>& ime_text_spans,
                      const gfx::Range& replacement_range,
                      int selection_start,
                      int selection_end) override;
  void CommitText(const String& text,
                  const Vector<ui::ImeTextSpan>& ime_text_spans,
                  const gfx::Range& replacement_range,
                  int relative_cursor_pos) override;
  void FinishComposingText(bool keep_selection) override;
  bool IsProvisional() override;
  cc::ElementId GetScrollableContainerIdAt(const gfx::PointF& point) override;
  bool ShouldHandleImeEvents() override;
  void SetEditCommandsForNextKeyEvent(
      Vector<mojom::blink::EditCommandPtr> edit_commands) override;
  Vector<ui::mojom::blink::ImeTextSpanInfoPtr> GetImeTextSpansInfo(
      const std::vector<ui::ImeTextSpan>& ime_text_spans) override;
  void RequestMouseLock(
      bool has_transient_user_activation,
      bool request_unadjusted_movement,
      mojom::blink::WidgetInputHandlerHost::RequestMouseLockCallback callback)
      override;
  gfx::RectF BlinkSpaceToDIPs(const gfx::RectF& rect) override;
  gfx::Rect BlinkSpaceToEnclosedDIPs(const gfx::Rect& rect) override;
  gfx::Size BlinkSpaceToFlooredDIPs(const gfx::Size& size) override;
  gfx::PointF DIPsToBlinkSpace(const gfx::PointF& point) override;
  gfx::Point DIPsToRoundedBlinkSpace(const gfx::Point& point) override;
  float DIPsToBlinkSpace(float scalar) override;
  void MouseCaptureLost() override;
  bool CanComposeInline() override;
  bool ShouldDispatchImeEventsToPlugin() override;
  void ImeSetCompositionForPlugin(const String& text,
                                  const Vector<ui::ImeTextSpan>& ime_text_spans,
                                  const gfx::Range& replacement_range,
                                  int selection_start,
                                  int selection_end) override;
  void ImeCommitTextForPlugin(const String& text,
                              const Vector<ui::ImeTextSpan>& ime_text_spans,
                              const gfx::Range& replacement_range,
                              int relative_cursor_pos) override;
  void ImeFinishComposingTextForPlugin(bool keep_selection) override;
  float GetCompositingScaleFactor() override;
  const cc::LayerTreeDebugState* GetLayerTreeDebugState() override;
  void SetLayerTreeDebugState(const cc::LayerTreeDebugState& state) override;
  void SetMayThrottleIfUndrawnFrames(
      bool may_throttle_if_undrawn_frames) override;
  int GetVirtualKeyboardResizeHeight() const override;

  void OnTaskCompletedForFrame(base::TimeTicks start_time,
                               base::TimeTicks end_time,
                               LocalFrame*) override;
  void SetVirtualKeyboardResizeHeightForTesting(int);
  bool GetMayThrottleIfUndrawnFramesForTesting();

  // AnimationFrameTimingMonitor::Client overrides
  bool ShouldReportLongAnimationFrameTiming() const override;
  void ReportLongTaskTiming(base::TimeTicks start_time,
                            base::TimeTicks end,
                            ExecutionContext* task_context) override;
  bool RequestedMainFramePending() override;
  AnimationFrameTimingInfo* RecordRenderingUpdateEndTime(
      base::TimeTicks) override;
  ukm::UkmRecorder* MainFrameUkmRecorder() override;
  ukm::SourceId MainFrameUkmSourceId() override;

  // WebFrameWidget overrides.
  void InitializeNonCompositing(WebNonCompositedWidgetClient* client) override;
  WebLocalFrame* LocalRoot() const override;
  void UpdateCompositorScrollState(
      const cc::CompositorCommitData& commit_data) override;
  WebInputMethodController* GetActiveWebInputMethodController() const override;
  void DisableDragAndDrop() override;
  WebLocalFrameImpl* FocusedWebLocalFrameInWidget() const override;
  bool ScrollFocusedEditableElementIntoView() override;
  void ApplyViewportChangesForTesting(
      const ApplyViewportChangesArgs& args) override;
  void ApplyViewportIntersectionForTesting(
      mojom::blink::ViewportIntersectionStatePtr intersection_state);
  void NotifyPresentationTime(
      base::OnceCallback<void(const viz::FrameTimingDetails&)>
          presentation_callback) override;
#if BUILDFLAG(IS_APPLE)
  void NotifyCoreAnimationErrorCode(
      base::OnceCallback<void(gfx::CALayerResult)> callback) override;
#endif
  void WaitForDebuggerWhenShown() override;
  void SetTextZoomFactor(float text_zoom_factor) override;
  float TextZoomFactor() override;
  void SetMainFrameOverlayColor(SkColor) override;
  void AddEditCommandForNextKeyEvent(const WebString& name,
                                     const WebString& value) override;
  void ClearEditCommands() override;
  bool IsPasting() override;
  bool HandlingSelectRange() override;
  void CalculateSelectionBounds(gfx::Rect& anchor_in_root_frame,
                                gfx::Rect& focus_in_root_frame) override;
  void ReleaseMouseLockAndPointerCaptureForTesting() override;
  const viz::FrameSinkId& GetFrameSinkId() override;
  WebHitTestResult HitTestResultAt(const gfx::PointF&) override;
  void SetZoomLevelForTesting(double zoom_level) override;
  void ResetZoomLevelForTesting() override;
  void SetDeviceScaleFactorForTesting(float factor) override;
  FrameWidgetTestHelper* GetFrameWidgetTestHelperForTesting() override;
  void PrepareForFinalLifecyclUpdateForTesting() override;
  void ApplyLocalSurfaceIdUpdate(const viz::LocalSurfaceId& id) override;

  // Called when a drag-n-drop operation should begin.
  virtual void StartDragging(LocalFrame* source_frame,
                             const WebDragData&,
                             DragOperationsMask,
                             const SkBitmap& drag_image,
                             const gfx::Vector2d& cursor_offset,
                             const gfx::Rect& drag_obj_rect);

  bool DoingDragAndDrop() { return doing_drag_and_drop_; }
  static void SetIgnoreInputEvents(
      const base::UnguessableToken& browsing_context_group_token,
      bool value);
  static bool IgnoreInputEvents(
      const base::UnguessableToken& browsing_context_group_token);

  // Resets the layout tracking steps for the main frame. When
  // `UpdateLifecycle()` is called it generates `WebMeaningfulLayout` events
  // only once. This resets the state back to the default so it will fire new
  // events.
  void ResetMeaningfulLayoutStateForMainFrame();

  // WebWidget overrides.
  void InitializeCompositing(const display::ScreenInfos& screen_infos,
                             const cc::LayerTreeSettings* settings) override;
  void InitializeCompositingFromPreviousWidget(
      const display::ScreenInfos& screen_infos,
      const cc::LayerTreeSettings* settings,
      WebFrameWidget& previous_widget) override;
  void SetCompositorVisible(bool visible) override;
  void WarmUpCompositor() override;
  gfx::Size Size() override;
  void Resize(const gfx::Size& size_with_dsf) override;
  void SetCursor(const ui::Cursor& cursor) override;
  bool HandlingInputEvent() override;
  void SetHandlingInputEvent(bool handling) override;
  void ProcessInputEventSynchronouslyForTesting(
      const WebCoalescedInputEvent&) override;
  void DispatchNonBlockingEventForTesting(
      std::unique_ptr<WebCoalescedInputEvent> event) override;
  WebInputEventResult DispatchBufferedTouchEvents() override;
  WebInputEventResult HandleInputEvent(const WebCoalescedInputEvent&) override;
  void UpdateTextInputState() override;
  void UpdateSelectionBounds() override;
  void ShowVirtualKeyboard() override;
  bool HasFocus() override;
  void SetFocus(bool focus) override;
  void FlushInputProcessedCallback() override;
  void CancelCompositionForPepper() override;
  void ApplyVisualProperties(
      const VisualProperties& visual_properties) override;
  bool PinchGestureActiveInMainFrame() override;
  float PageScaleInMainFrame() override;
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
  WebString GetLastToolTipTextForTesting() const override;
  float GetEmulatorScale() override;

  // WidgetBaseClient overrides:
  void OnCommitRequested() override;
  void BeginMainFrame(const viz::BeginFrameArgs& args) override;
  void UpdateLifecycle(WebLifecycleUpdate requested_update,
                       DocumentUpdateReason reason) override;
  void ShowContextMenu(ui::mojom::blink::MenuSourceType source_type,
                       const gfx::Point& location) override;
  void BindInputTargetClient(
      mojo::PendingReceiver<viz::mojom::blink::InputTargetClient> receiver)
      override;

  // mojom::blink::FrameWidget overrides:
  void SetViewportIntersection(
      mojom::blink::ViewportIntersectionStatePtr intersection_state,
      const std::optional<VisualProperties>& visual_properties) override;
  void DragSourceEndedAt(const gfx::PointF& point_in_viewport,
                         const gfx::PointF& screen_point,
                         ui::mojom::blink::DragOperation,
                         base::OnceClosure callback) override;
  void OnStartStylusWriting(
#if BUILDFLAG(IS_WIN)
      const gfx::Rect& focus_widget_rect_in_dips,
#endif  // BUILDFLAG(IS_WIN)
      OnStartStylusWritingCallback callback) override;
#if BUILDFLAG(IS_ANDROID)
  void PassImeRenderWidgetHost(
      mojo::PendingRemote<mojom::blink::ImeRenderWidgetHost>) override;
#endif
  void NotifyClearedDisplayedGraphics() override;

  // mojom::blink::FrameWidgetInputHandler overrides:
  void HandleStylusWritingGestureAction(
      mojom::blink::StylusWritingGestureDataPtr gesture_data,
      HandleStylusWritingGestureActionCallback callback) override;

  // Sets the display mode, which comes from the top-level browsing context and
  // is applied to all widgets.
  void SetDisplayMode(mojom::blink::DisplayMode);

  // Sets the window show state.
  void SetWindowShowState(ui::mojom::blink::WindowShowState);

  void SetResizable(bool);

  std::optional<gfx::Point> GetAndResetContextMenuLocation();

  void BindWidgetCompositor(
      mojo::PendingReceiver<mojom::blink::WidgetCompositor> receiver) override;

  // viz::mojom::blink::InputTargetClient:
  void FrameSinkIdAt(const gfx::PointF& point,
                     const uint64_t trace_id,
                     FrameSinkIdAtCallback callback) override;

  // Called when the FrameView for this Widget's local root is created.
  void DidCreateLocalRootView();

  double GetZoomLevel() override;
  void SetZoomLevel(double zoom_level) override;
  double GetCSSZoomFactor() const override;

  // Called when the View has auto resized.
  virtual void DidAutoResize(const gfx::Size& size);

  // This method returns the focused frame belonging to this WebWidget, that
  // is, a focused frame with the same local root as the one corresponding
  // to this widget. It will return nullptr if no frame is focused or, the
  // focused frame has a different local root.
  LocalFrame* FocusedLocalFrameInWidget() const;

  // For when the embedder itself change scales on the page (e.g. devtools)
  // and wants all of the content at the new scale to be crisp
  void SetNeedsRecalculateRasterScales();

  // Sets the background color to be filled in as gutter behind/around the
  // painted content. Non-composited WebViews need not implement this, as they
  // paint into another widget which has a background color of its own.
  void SetBackgroundColor(SkColor color);

  // Sets whether the prefers-reduced-motion hint has been enabled.
  void SetPrefersReducedMotion(bool prefers_reduced_motion);

  // Starts an animation of the page scale to a target scale factor and scroll
  // offset.
  // If use_anchor is true, destination is a point on the screen that will
  // remain fixed for the duration of the animation.
  // If use_anchor is false, destination is the final top-left scroll position.
  void StartPageScaleAnimation(const gfx::Point& destination,
                               bool use_anchor,
                               float new_page_scale,
                               base::TimeDelta duration);

  // Called to update if scroll events should be sent.
  void SetHaveScrollEventHandlers(bool);

  // Start deferring commits to the compositor, allowing document lifecycle
  // updates without committing the layer tree. Commits are deferred
  // until at most the given |timeout| has passed. If multiple calls are made
  // when deferral is active then the initial timeout applies.
  bool StartDeferringCommits(base::TimeDelta timeout,
                             cc::PaintHoldingReason reason);
  // Immediately stop deferring commits.
  void StopDeferringCommits(cc::PaintHoldingCommitTrigger);

  void SetShouldThrottleFrameRate(bool flag);

  // Pause all rendering (main and compositor thread) in the compositor.
  [[nodiscard]] std::unique_ptr<cc::ScopedPauseRendering> PauseRendering();

  // Returns the maximum bounds for buffers allocated for rasterization and
  // compositing. This is is max texture size for GPU compositing and a browser
  // chosen limit in software mode.
  // Returns null if the compositing stack has not been initialized yet.
  std::optional<int> GetMaxRenderBufferBounds() const;

  // Prevents any updates to the input for the layer tree, and the layer tree
  // itself, and the layer tree from becoming visible.
  std::unique_ptr<cc::ScopedDeferMainFrameUpdate> DeferMainFrameUpdate();

  // Sets the amount that the top and bottom browser controls are showing, from
  // 0 (hidden) to 1 (fully shown).
  void SetBrowserControlsShownRatio(float top_ratio, float bottom_ratio);

  // Set browser controls params. These params consist of top and bottom
  // heights, min-heights, browser_controls_shrink_blink_size, and
  // animate_browser_controls_height_changes. If
  // animate_browser_controls_height_changes is set to true, changes to the
  // browser controls height will be animated. If
  // browser_controls_shrink_blink_size is set to true, then Blink shrunk the
  // viewport clip layers by the top and bottom browser controls height. Top
  // controls will translate the web page down and do not immediately scroll
  // when hiding. The bottom controls scroll immediately and never translate the
  // content (only clip it).
  void SetBrowserControlsParams(cc::BrowserControlsParams params);

  void SetMaxSafeAreaInsets(const gfx::InsetsF& max_safe_area_insets);

  // This function provides zooming for find in page results when browsing with
  // page autosize.
  void ZoomToFindInPageRect(const gfx::Rect& rect_in_root_frame);

  // Return the compositor LayerTreeHost.
  cc::LayerTreeHost* LayerTreeHostForTesting() const;
  // Ask compositor to composite a frame for testing. This will generate a
  // BeginMainFrame, and update the document lifecycle.
  void SynchronouslyCompositeForTesting(base::TimeTicks frame_time);

  // Sets the device color space for testing.
  void SetDeviceColorSpaceForTesting(const gfx::ColorSpace& color_space);

  // Converts from DIPs to Blink coordinate space (ie. Viewport/Device
  // pixels).
  gfx::Size DIPsToCeiledBlinkSpace(const gfx::Size& size);

  void SetWindowRect(const gfx::Rect& requested_rect,
                     const gfx::Rect& adjusted_rect);
  void SetWindowRectSynchronouslyForTesting(const gfx::Rect& new_window_rect);

  void UpdateTooltipUnderCursor(const String& tooltip_text, TextDirection dir);
  void UpdateTooltipFromKeyboard(const String& tooltip_text,
                                 TextDirection dir,
                                 const gfx::Rect& bounds);
  void ClearKeyboardTriggeredTooltip();

  void ShowVirtualKeyboardOnElementFocus();
  void ProcessTouchAction(WebTouchAction touch_action);
  void SetPanAction(mojom::blink::PanAction pan_action);

  // Called to update whether low latency input mode is enabled or not.
  void SetNeedsLowLatencyInput(bool);

  // Requests unbuffered (ie. low latency) input until a pointerup
  // event occurs.
  void RequestUnbufferedInputEvents();

  // Requests unbuffered (ie. low latency) input due to debugger being
  // attached. Debugger needs to paint when stopped in the event handler.
  void SetNeedsUnbufferedInputForDebugger(bool);

  // Called when the main frame navigates.
  void DidNavigate();

  // Ensures all queued input in the widget has been processed and the queues
  // emptied.
  void FlushInputForTesting(base::OnceClosure done_callback);

  // Called when the widget should get targeting input.
  void SetMouseCapture(bool capture);

  // Called when the widget's main frame has ongoing autoscroll selection.
  void NotifyAutoscrollForSelectionInMainFrame(bool autoscroll_selection);

  // Sets the current page scale factor and minimum / maximum limits. Both
  // limits are initially 1 (no page scale allowed).
  void SetPageScaleStateAndLimits(float page_scale_factor,
                                  bool is_pinch_gesture_active,
                                  float minimum,
                                  float maximum);
  void UpdateViewportDescription(
      const ViewportDescription& viewport_description);

  const viz::LocalSurfaceId& LocalSurfaceIdFromParent();

  ScreenMetricsEmulator* DeviceEmulator();

  // Calculates the selection bounds in the root frame. Returns bounds unchanged
  // when there is no focused frame. Returns the caret bounds if the selection
  // range is empty.
  void CalculateSelectionBounds(gfx::Rect& anchor_in_root_frame,
                                gfx::Rect& focus_in_root_frame,
                                gfx::Rect* bounding_box_in_root_frame);

  // Returns if auto resize mode is enabled.
  bool AutoResizeMode();

  void SetScreenMetricsEmulationParameters(
      bool enabled,
      const blink::DeviceEmulationParams& params);
  void SetScreenInfoAndSize(const display::ScreenInfos& screen_infos,
                            const gfx::Size& widget_size_in_dips,
                            const gfx::Size& visible_viewport_size_in_dips);

  // Update the surface allocation information, compositor viewport rect and
  // screen info on the widget.
  void UpdateSurfaceAndScreenInfo(
      const viz::LocalSurfaceId& new_local_surface_id,
      const gfx::Rect& compositor_viewport_pixel_rect,
      const display::ScreenInfos& screen_infos);
  // Similar to UpdateSurfaceAndScreenInfo but the surface allocation
  // and compositor viewport rect remains the same.
  void UpdateScreenInfo(const display::ScreenInfos& screen_infos);
  void UpdateSurfaceAndCompositorRect(
      const viz::LocalSurfaceId& new_local_surface_id,
      const gfx::Rect& compositor_viewport_pixel_rect);
  void UpdateCompositorViewportRect(
      const gfx::Rect& compositor_viewport_pixel_rect);
  void OverrideDevicePostureForEmulation(
      mojom::blink::DevicePostureType device_posture_param);
  void DisableDevicePostureOverrideForEmulation();
  void SetViewportSegments(const std::vector<gfx::Rect>& viewport_segments);
  viz::FrameSinkId GetFrameSinkIdAtPoint(const gfx::PointF& point,
                                         gfx::PointF* local_point);

  // Set the pending window rect. For every SetPendingWindowRect
  // call there must be an AckPendingWindowRect call.
  void SetPendingWindowRect(const gfx::Rect& window_screen_rect);

  // Clear a previously set pending window rect. For every SetPendingWindowRect
  // call there must be an AckPendingWindowRect call.
  void AckPendingWindowRect();

  // Return the focused WebPlugin if there is one.
  WebPlugin* GetFocusedPluginContainer();

  // Return if there is a pending scale animation.
  bool HasPendingPageScaleAnimation();

  // Set the source URL (if the compositor is rendering the primary main frame).
  // Also propagates the history sequence number (equivalent to a
  // `PropagateHistorySequenceNumberToCompositor()` call).
  void UpdateNavigationStateForCompositor(ukm::SourceId source_id,
                                          const KURL& url);

  // Propagates the HistoryItem's ItemSequenceNumber to the compositor. This is
  // used as part of a RenderFrameMetadata which is sent to the browser when the
  // compositor submits a frame.
  void PropagateHistorySequenceNumberToCompositor();

  // Ask compositor to create the shared memory for smoothness ukm region.
  base::ReadOnlySharedMemoryRegion CreateSharedMemoryForSmoothnessUkm();

  // Ask compositor to create the shared memory for dropped frames ukm region.
  base::ReadOnlySharedMemoryRegion CreateSharedMemoryForDroppedFramesUkm();
#if BUILDFLAG(IS_ANDROID)
  // Calculate and cache the most up to date line bounding boxes in the document
  // coordinate space.
  Vector<gfx::Rect> CalculateVisibleLineBoundsOnScreen();
#endif  // BUILDFLAG(IS_ANDROID)

  // Returns true if this widget corresponds to a frame which is being replaced.
  // The compositor for the widget has been detached and passed to the new
  // widget.
  bool WillBeDestroyed() const;

  bool IsScrollGestureActive() const;

  // Request the compositor thread to tell the GPU process to generate a
  // screenshot of the current viewport. The screenshot is tagged with `token`.
  // The caller must have requested a new `viz::LocalSurfaceID` before making
  // this request.
  void RequestViewportScreenshot(const base::UnguessableToken& token);

  // Request a new `viz::LocalSurfaceId` on the compositor thread.
  void RequestNewLocalSurfaceId();

  void OnDevToolsSessionConnectionChanged(bool attached);

 protected:
  // WidgetBaseClient overrides:
  void ScheduleAnimation(bool urgent) override;
  void DidBeginMainFrame() override;
  std::unique_ptr<cc::LayerTreeFrameSink> AllocateNewLayerTreeFrameSink()
      override;
  void WasShown(bool was_evicted) override;

  // Whether compositing to LCD text should be auto determined. This can be
  // overridden by tests to disable this.
  virtual bool ShouldAutoDetermineCompositingToLCDTextSetting();

  WidgetBase* widget_base_for_testing() const { return widget_base_.get(); }

  // WebFrameWidget overrides.
  cc::LayerTreeHost* LayerTreeHost() override;

  // Determines whether the drag source is currently dragging.
  bool doing_drag_and_drop_ = false;

 private:
  friend class WebViewImpl;
  friend class ReportTimeSwapPromise;

  void NotifySwapAndPresentationTime(PromiseCallbacks callbacks);

  // WidgetBaseClient overrides.
  void BeginCommitCompositorFrame() override;
  void EndCommitCompositorFrame(base::TimeTicks commit_start_time,
                                base::TimeTicks commit_finish_time) override;
  void ApplyViewportChanges(const cc::ApplyViewportChangesArgs& args) override;
  void RecordDispatchRafAlignedInputTime(
      base::TimeTicks raf_aligned_input_start_time) override;
  void SetSuppressFrameRequestsWorkaroundFor704763Only(bool) override;
  void RecordStartOfFrameMetrics() override;
  void RecordEndOfFrameMetrics(
      base::TimeTicks,
      cc::ActiveFrameSequenceTrackers trackers) override;
  std::unique_ptr<cc::BeginMainFrameMetrics> GetBeginMainFrameMetrics()
      override;
  void BeginUpdateLayers() override;
  void EndUpdateLayers() override;
  void DidCommitAndDrawCompositorFrame() override;
  void DidObserveFirstScrollDelay(
      base::TimeDelta first_scroll_delay,
      base::TimeTicks first_scroll_timestamp) override;
  void DidCompletePageScaleAnimation() override;
  void FocusChangeComplete() override;
  void WillHandleGestureEvent(const WebGestureEvent& event,
                              bool* suppress) override;
  void WillHandleMouseEvent(const WebMouseEvent& event) override;
  void ObserveGestureEventAndResult(
      const WebGestureEvent& gesture_event,
      const gfx::Vector2dF& unused_delta,
      const cc::OverscrollBehavior& overscroll_behavior,
      bool event_processed) override;
  bool SupportsBufferedTouchEvents() override { return true; }
  void DidHandleKeyEvent() override;
  WebTextInputType GetTextInputType() override;
  void SetCursorVisibilityState(bool is_visible) override;
  blink::FrameWidget* FrameWidget() override { return this; }
  void FocusChanged(mojom::blink::FocusState focus_state) override;
  bool ShouldAckSyntheticInputImmediately() override;
  void UpdateVisualProperties(
      const VisualProperties& visual_properties) override;
  bool UpdateScreenRects(const gfx::Rect& widget_screen_rect,
                         const gfx::Rect& window_screen_rect) override;
  void OrientationChanged() override;
  void DidUpdateSurfaceAndScreen(
      const display::ScreenInfos& previous_original_screen_infos) override;
  gfx::Rect ViewportVisibleRect() override;
  std::optional<display::mojom::blink::ScreenOrientation>
  ScreenOrientationOverride() override;
  void WasHidden() override;
  void RunPaintBenchmark(int repeat_count,
                         cc::PaintBenchmarkResult& result) override;
  KURL GetURLForDebugTrace() override;
  float GetTestingDeviceScaleFactorOverride() override;
  void CountDroppedPointerDownForEventTiming(unsigned count) override;
  // mojom::blink::FrameWidget overrides.
  void DragTargetDragEnter(const WebDragData&,
                           const gfx::PointF& point_in_viewport,
                           const gfx::PointF& screen_point,
                           DragOperationsMask operations_allowed,
                           uint32_t key_modifiers,
                           DragTargetDragEnterCallback callback) override;
  void DragTargetDragOver(const gfx::PointF& point_in_viewport,
                          const gfx::PointF& screen_point,
                          DragOperationsMask operations_allowed,
                          uint32_t key_modifiers,
                          DragTargetDragOverCallback callback) override;
  void DragTargetDragLeave(const gfx::PointF& point_in_viewport,
                           const gfx::PointF& screen_point) override;
  void DragTargetDrop(const WebDragData&,
                      const gfx::PointF& point_in_viewport,
                      const gfx::PointF& screen_point,
                      uint32_t key_modifiers,
                      base::OnceClosure callback) override;
  void DragSourceSystemDragEnded() override;
  void SetBackgroundOpaque(bool opaque) override;
  void SetActive(bool active) override;
  // For both mainframe and childframe change the text direction of the
  // currently selected input field (if any).
  void SetTextDirection(base::i18n::TextDirection direction) override;
  // Sets the inherited effective touch action on an out-of-process iframe.
  void SetInheritedEffectiveTouchActionForSubFrame(
      WebTouchAction touch_action) override;
  // Toggles render throttling for an out-of-process iframe. Local frames are
  // throttled based on their visibility in the viewport, but remote frames
  // have to have throttling information propagated from parent to child
  // across processes.
  void UpdateRenderThrottlingStatusForSubFrame(bool is_throttled,
                                               bool subtree_throttled,
                                               bool display_locked) override;
  void EnableDeviceEmulation(const DeviceEmulationParams& parameters) override;
  void DisableDeviceEmulation() override;
  // Sets the inert bit on an out-of-process iframe, causing it to ignore
  // input.
  void SetIsInertForSubFrame(bool inert) override;
#if BUILDFLAG(IS_MAC)
  void GetStringAtPoint(const gfx::Point& point_in_local_root,
                        GetStringAtPointCallback callback) override;
#endif

  // mojom::blink::FrameWidgetInputHandler overrides.
  void AddImeTextSpansToExistingText(
      uint32_t start,
      uint32_t end,
      const Vector<ui::ImeTextSpan>& ime_text_spans) override;
  void ClearImeTextSpansByType(uint32_t start,
                               uint32_t end,
                               ui::ImeTextSpan::Type type) override;
  void SetCompositionFromExistingText(
      int32_t start,
      int32_t end,
      const Vector<ui::ImeTextSpan>& ime_text_spans) override;
  void ExtendSelectionAndDelete(int32_t before, int32_t after) override;
  void ExtendSelectionAndReplace(uint32_t before,
                                 uint32_t after,
                                 const String& replacement_text) override;
  void DeleteSurroundingText(int32_t before, int32_t after) override;
  void DeleteSurroundingTextInCodePoints(int32_t before,
                                         int32_t after) override;
  void SetEditableSelectionOffsets(int32_t start, int32_t end) override;
  void ExecuteEditCommand(const String& command, const String& value) override;
  void Undo() override;
  void Redo() override;
  void Cut() override;
  void Copy() override;
  void CopyToFindPboard() override;
  void CenterSelection() override;
  void Paste() override;
  void PasteAndMatchStyle() override;
  void Delete() override;
  void SelectAll() override;
  void CollapseSelection() override;
  void Replace(const String& word) override;
  void ReplaceMisspelling(const String& word) override;
  void SelectRange(const gfx::Point& base_in_dips,
                   const gfx::Point& extent_in_dips) override;
  void AdjustSelectionByCharacterOffset(
      int32_t start,
      int32_t end,
      mojom::blink::SelectionMenuBehavior behavior) override;
  void MoveRangeSelectionExtent(const gfx::Point& extent_in_dips) override;
  void ScrollFocusedEditableNodeIntoView() override;
  void WaitForPageScaleAnimationForTesting(
      WaitForPageScaleAnimationForTestingCallback callback) override;
  void MoveCaret(const gfx::Point& point_in_dips) override;

#if BUILDFLAG(IS_IOS)
  void StartAutoscrollForSelectionToPoint(
      const gfx::PointF& point_in_dips) override;
  void StopAutoscroll() override;

  void RectForEditFieldChars(const gfx::Range& range,
                             RectForEditFieldCharsCallback callback) override;
#endif  // BUILDFLAG(IS_IOS)

  void SelectAroundCaret(mojom::blink::SelectionGranularity granularity,
                         bool should_show_handle,
                         bool should_show_context_menu,
                         SelectAroundCaretCallback callback) override;

  // PageWidgetEventHandler overrides:
  WebInputEventResult HandleKeyEvent(const WebKeyboardEvent&) override;
  void HandleMouseDown(LocalFrame&, const WebMouseEvent&) override;
  void HandleMouseLeave(LocalFrame&, const WebMouseEvent&) override;
  WebInputEventResult HandleMouseUp(LocalFrame&, const WebMouseEvent&) override;
  WebInputEventResult HandleGestureEvent(const WebGestureEvent&) override;
  WebInputEventResult HandleMouseWheel(LocalFrame&,
                                       const WebMouseWheelEvent&) override;
  WebInputEventResult HandleCharEvent(const WebKeyboardEvent&) override;

  void SetZoomInternal(double zoom_level, double css_zoom_factor);

  WebInputEventResult HandleCapturedMouseEvent(const WebCoalescedInputEvent&);
  void MouseContextMenu(const WebMouseEvent&);
  void CancelDrag();
  void PresentationCallbackForMeaningfulLayout(
      const viz::FrameTimingDetails& first_paint_details);

  void ForEachRemoteFrameControlledByWidget(
      base::FunctionRef<void(RemoteFrame*)> callback);

  void SendOverscrollEventFromImplSide(const gfx::Vector2dF& overscroll_delta,
                                       cc::ElementId scroll_latched_element_id);
  // TODO(crbug.com/372627916): This function is not used when
  // MultipleImplOnlyScrollAnimations is enabled. It should be considered
  // deprecated and should be deleted when the MultipleImplOnlyScrollAnimations
  // code path is the only existing code path.
  void SendEndOfScrollEventsDeprecated(bool affects_outer_viewport,
                                       bool affects_inner_viewport,
                                       cc::ElementId scroll_latched_element_id);
  void SendEndOfScrollEvents(const cc::CompositorCommitData& commit_data);
  void SendScrollSnapChangingEventIfNeeded(
      const cc::CompositorCommitData& commit_data);
  void RecordManipulationTypeCounts(cc::ManipulationInfo info);

  enum DragAction { kDragEnter, kDragOver };
  // Consolidate some common code between starting a drag over a target and
  // updating a drag over a target. If we're starting a drag, |isEntering|
  // should be true.
  void DragTargetDragEnterOrOver(const gfx::PointF& point_in_viewport,
                                 const gfx::PointF& screen_point,
                                 DragAction,
                                 uint32_t key_modifiers);

  // Helper function to call VisualViewport::viewportToRootFrame().
  gfx::PointF ViewportToRootFrame(const gfx::PointF& point_in_viewport) const;

  WebViewImpl* View() const;

  // Returns the page object associated with this widget. This may be null when
  // the page is shutting down, but will be valid at all other times.
  Page* GetPage() const;

  mojom::blink::FrameWidgetHost* GetAssociatedFrameWidgetHost() const;

  void InitializeCompositingInternal(const display::ScreenInfos& screen_infos,
                                     const cc::LayerTreeSettings* settings,
                                     WebFrameWidget* previous_widget);

  // Notifies RenderWidgetHostImpl that the frame widget has painted something.
  void DidMeaningfulLayout(WebMeaningfulLayout layout_type);

  // Enable or disable auto-resize. This is part of
  // UpdateVisualProperties though tests may call to it more directly.
  void SetAutoResizeMode(bool auto_resize,
                         const gfx::Size& min_size_before_dsf,
                         const gfx::Size& max_size_before_dsf,
                         float device_scale_factor);

  void ApplyViewportIntersection(
      mojom::blink::ViewportIntersectionStatePtr intersection_state);

  // Called when a gesture event has been processed.
  void DidHandleGestureEvent(const WebGestureEvent& event);

  // Called to update if pointerrawupdate events should be sent.
  void SetHasPointerRawUpdateEventHandlers(bool);

  // Helper function to process events while pointer locked.
  void PointerLockMouseEvent(const WebCoalescedInputEvent&);
  bool IsPointerLocked();

  // The fullscreen granted status from the most recent VisualProperties update.
  bool IsFullscreenGranted();

  // Set the compositing scale factor for this widget and notify remote frames
  // to update their compositing scale factor.
  void NotifyCompositingScaleFactorChanged(float compositing_scale_factor);

  void NotifyPageScaleFactorChanged(float page_scale_factor,
                                    bool is_pinch_gesture_active);

  // Helper for notifying frame-level objects that care about input events.
  // TODO: With some effort, this could be folded into a common implementation
  // of WebViewImpl::HandleInputEvent and WebFrameWidgetImpl::HandleInputEvent.
  void NotifyInputObservers(const WebCoalescedInputEvent& coalesced_event);

  Frame* FocusedCoreFrame() const;

  // Returns the currently focused `Element` in any `LocalFrame` owned by the
  // associated `WebView`.
  Element* FocusedElement() const;

  gfx::Rect GetAbsoluteCaretBounds();

  // Perform a hit test for a point relative to the root frame of the page.
  HitTestResult HitTestResultForRootFramePos(
      const gfx::PointF& pos_in_root_frame);

  // Called during |UpdateVisualProperties| to apply the new size to the widget.
  void ApplyVisualPropertiesSizing(const VisualProperties& visual_properties);

  // Returns true iff the visual property state contains an update that will
  // change the fullscreen state (e.g. on/off or current display).
  bool DidChangeFullscreenState(
      const VisualProperties& visual_properties) const;

  // Satisfy the render blocking condition for cross-document view transitions.
  void NotifyViewTransitionRenderingHasBegun();

  // True when `this` should ignore input events.
  bool ShouldIgnoreInputEvents();

  // Triggers onmove event for window.
  void EnqueueMoveEvent();

  // Let latched scroller know to unpin its selected scroll-marker.
  void NotifyLatchedScrollMarkerGroup(
      const cc::CompositorCommitData& commit_data);

#if BUILDFLAG(IS_WIN)
  // Computes a contiguous range of character bounds within proximity of
  // `pivot_position` to enable gesture support for StylusHandwritingWin.
  mojom::blink::ProximateCharacterRangeBoundsPtr
  ComputeProximateCharacterBounds(
      const PositionWithAffinity& pivot_position) const;
#endif  // BUILDFLAG(IS_WIN)

  // Stores the last cursor anchor info calculated for the currently focused
  // editable element.
  mojom::blink::InputCursorAnchorInfoPtr last_cursor_anchor_info_;

  // A copy of the web drop data object we received from the browser.
  Member<DataObject> current_drag_data_;

  // The available drag operations (copy, move link...) allowed by the source.
  DragOperationsMask operations_allowed_ = kDragOperationNone;

  // The current drag operation as negotiated by the source and destination.
  // When not equal to DragOperationNone, the drag data can be dropped onto the
  // current drop target in this WebView (the drop target can accept the drop).
  DragController::Operation drag_operation_;

  // This field stores drag/drop related info for the event that is currently
  // being handled. If the current event results in starting a drag/drop
  // session, this info is sent to the browser along with other drag/drop info.
  mojom::blink::DragEventSourceInfo possible_drag_event_info_;

  // Base functionality all widgets have. This is a member as to avoid
  // complicated inheritance structures.
  std::unique_ptr<WidgetBase> widget_base_;

  // Compositing scale factor for all frames attached to this widget sent from
  // the remote parent frame.
  float compositing_scale_factor_ = 1.f;

  // The last seen page scale state, which comes from the main frame if we're
  // in a child frame. This state is propagated through the RenderWidget tree
  // passed to any new child RenderWidget.
  float page_scale_factor_in_mainframe_ = 1.f;
  bool is_pinch_gesture_active_in_mainframe_ = false;

  bool is_scroll_gesture_active_ = false;

  // If set, the (plugin) element which has mouse capture.
  Member<HTMLPlugInElement> mouse_capture_element_;

  // The size of the widget in viewport coordinates. This is slightly different
  // than the WebViewImpl::size_ since isn't set in auto resize mode.
  std::optional<gfx::Size> size_;

  // The amount the top-most widget has been resized by the virtual keyboard,
  // in device pixels.
  int virtual_keyboard_resize_height_device_px_ = 0;

  static bool ignore_input_events_;

  const viz::FrameSinkId frame_sink_id_;

  // WebFrameWidget is associated with a subtree of the frame tree,
  // corresponding to a maximal connected tree of LocalFrames. This member
  // points to the root of that subtree.
  Member<WebLocalFrameImpl> local_root_;

  mojom::blink::DisplayMode display_mode_;
  ui::mojom::blink::WindowShowState window_show_state_;
  bool resizable_;

  std::vector<gfx::Rect> viewport_segments_
      ALLOW_DISCOURAGED_TYPE("Will be passed to FrameVisualProperties");

  // This is owned by the LayerTreeHostImpl, and should only be used on the
  // compositor thread, so we keep the TaskRunner where you post tasks to
  // make that happen.
  base::WeakPtr<AnimationWorkletMutatorDispatcherImpl> mutator_dispatcher_;
  scoped_refptr<base::SingleThreadTaskRunner> mutator_task_runner_;

  // The |paint_dispatcher_| should only be dereferenced on the
  // |paint_task_runner_| (in practice this is the compositor thread). We keep a
  // copy of it here to provide to new PaintWorkletProxyClient objects (which
  // run on the worklet thread) so that they can talk to the
  // PaintWorkletPaintDispatcher on the compositor thread.
  base::WeakPtr<PaintWorkletPaintDispatcher> paint_dispatcher_;
  scoped_refptr<base::SingleThreadTaskRunner> paint_task_runner_;

  // WebFrameWidgetImpl is not tied to ExecutionContext
  HeapMojoAssociatedRemote<mojom::blink::FrameWidgetHost> frame_widget_host_{
      nullptr};
  // WebFrameWidgetImpl is not tied to ExecutionContext
  HeapMojoAssociatedReceiver<mojom::blink::FrameWidget, WebFrameWidgetImpl>
      receiver_{this, nullptr};
  HeapMojoReceiverSet<viz::mojom::blink::InputTargetClient, WebFrameWidgetImpl>
      input_target_receivers_;

  // WebFrameWidgetImpl is not tied to ExecutionContext
  HeapMojoRemote<mojom::blink::ImeRenderWidgetHost> ime_render_widget_host_{
      nullptr};

  // Different consumers in the browser process makes different assumptions, so
  // must always send the first IPC regardless of value.
  std::optional<bool> has_touch_handlers_;

  Vector<mojom::blink::EditCommandPtr> edit_commands_;

  std::optional<gfx::Point> host_context_menu_location_;
  uint32_t last_capture_sequence_number_ = 0u;

  // Indicates whether tab-initiated fullscreen was granted.
  bool is_fullscreen_granted_ = false;

  // Indicates whether we need to consume scroll gestures to move cursor.
  bool swipe_to_move_cursor_activated_ = false;

  // Set when a measurement begins, reset when the measurement is taken.
  std::optional<base::TimeTicks> update_layers_start_time_;

  // Metrics for gathering time for commit of compositor frame.
  std::optional<base::TimeTicks> commit_compositor_frame_start_time_;
  std::optional<base::TimeTicks> next_commit_compositor_frame_start_time_;

  // Present when emulation is enabled, only on a main frame's WebFrameWidget.
  // Used to override values given from the browser such as ScreenInfo,
  // WidgetScreenRect, WindowScreenRect, and the widget's size.
  Member<ScreenMetricsEmulator> device_emulator_;

  Member<AnimationFrameTimingMonitor> animation_frame_timing_monitor_;

  // keyPress events to be suppressed if the associated keyDown event was
  // handled.
  bool suppress_next_keypress_event_ = false;

  // Whether drag and drop is supported by this widget. When disabled
  // any drag operation that is started will be canceled immediately.
  bool drag_and_drop_disabled_ = false;

  // A callback client for non-composited frame widgets.
  WebNonCompositedWidgetClient* non_composited_client_ = nullptr;

  // This struct contains data that is only valid for child local root widgets.
  // You should use `child_data()` to access it.
  struct ChildLocalRootData {
    gfx::Rect compositor_visible_rect;
    bool did_suspend_parsing = false;
  } child_local_root_data_;

  ChildLocalRootData& child_data() {
    DCHECK(ForSubframe());
    return child_local_root_data_;
  }

  // Web tests override the zoom factor in the renderer with this. We store it
  // to keep the override if the browser passes along VisualProperties with the
  // real device scale factor. A value of -INFINITY means this is ignored.
  // It is always valid to read this variable but it can only be set for main
  // frame widgets.
  double zoom_level_for_testing_ = -INFINITY;

  // Web tests override the device scale factor in the renderer with this. We
  // store it to keep the override if the browser passes along VisualProperties
  // with the real device scale factor. A value of 0.f means this is ignored.
  // It is always valid to read this variable but it can only be set for main
  // frame widgets.
  float device_scale_factor_for_testing_ = 0;

  // When device_scale_factor_for_testing_ is set (i.e. nonzero), this
  // stores the device scale factor before the testing override was set.
  // Otherwise it is set to zero.
  float non_testing_device_scale_factor_ = 0;

  // This struct contains data that is only valid for main frame widgets.
  // You should use `main_data()` to access it.
  struct MainFrameData {
    // `UpdateLifecycle()` generates `WebMeaningfulLayout` events these
    // variables track what events should be generated. They are only applicable
    // for main frame widgets.
    bool should_dispatch_first_visually_non_empty_layout = false;
    bool should_dispatch_first_layout_after_finished_parsing = false;
    bool should_dispatch_first_layout_after_finished_loading = false;
    // Last background color sent to the browser. Only set for main frames.
    std::optional<SkColor> last_background_color;
    // This bit is used to tell if this is a nested widget (an "inner web
    // contents") like a <webview>. If false, the widget is the top level
    // widget.
    bool is_for_nested_main_frame = false;
  } main_frame_data_;

  MainFrameData& main_data() {
    DCHECK(ForMainFrame());
    return main_frame_data_;
  }

  const MainFrameData& main_data() const {
    DCHECK(ForMainFrame());
    return main_frame_data_;
  }

  // Whether this widget is for a child local root, or otherwise a main frame.
  // Prefer using |ForSubframe()| for distinguishing subframes and
  // |widget_base_.is_embedded()| for subframes and embedded main frames (i.e.,
  // all embedded scenarios).
  const bool is_for_child_local_root_;

  // Whether this widget is for a guest view, or top level frame.
  // These may have a page scale node, so it is important to plumb this
  // information through to avoid breaking assumptions.
  const bool is_for_scalable_page_;

  WaitForPageScaleAnimationForTestingCallback
      page_scale_animation_for_testing_callback_;

  // This stores the last hidden page popup. If a GestureTap attempts to open
  // the popup that is closed by its previous GestureTapDown, the popup remains
  // closed.
  scoped_refptr<WebPagePopupImpl> last_hidden_page_popup_;

  base::WeakPtrFactory<mojom::blink::FrameWidgetInputHandler>
      input_handler_weak_ptr_factory_{this};

  double zoom_level_ = 0;
  double css_zoom_factor_ = 1;

  std::optional<float> browser_controls_top_height_override_;

  bool throttling_frame_rate_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_FRAME_WIDGET_IMPL_H_
