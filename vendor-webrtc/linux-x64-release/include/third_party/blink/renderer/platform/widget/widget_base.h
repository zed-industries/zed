// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_WIDGET_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_WIDGET_BASE_H_

#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "cc/animation/animation_timeline.h"
#include "cc/mojo_embedder/async_layer_tree_frame_sink.h"
#include "cc/paint/element_id.h"
#include "cc/trees/browser_controls_params.h"
#include "cc/trees/paint_holding_reason.h"
#include "components/viz/common/surfaces/local_surface_id.h"
#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "third_party/blink/public/common/metrics/document_update_reason.h"
#include "third_party/blink/public/common/page/content_to_visible_time_reporter.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"
#include "third_party/blink/public/mojom/widget/platform_widget.mojom-blink.h"
#include "third_party/blink/public/mojom/widget/record_content_to_visible_time_request.mojom-blink-forward.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_text_input_info.h"
#include "third_party/blink/renderer/platform/graphics/lcd_text_preference.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/widget/compositing/layer_tree_view_delegate.h"
#include "third_party/blink/renderer/platform/widget/compositing/render_frame_metadata_observer_impl.h"
#include "third_party/blink/renderer/platform/widget/input/widget_base_input_handler.h"
#include "ui/base/ime/text_input_mode.h"
#include "ui/base/ime/text_input_type.h"
#include "ui/base/mojom/menu_source_type.mojom-blink-forward.h"
#include "ui/gfx/ca_layer_result.h"

namespace cc {
class AnimationHost;
class AnimationTimeline;
class LayerTreeHost;
class LayerTreeSettings;
}  // namespace cc

namespace gpu {
class GpuChannelHost;
}

namespace ui {
class Cursor;
}

namespace display {
struct ScreenInfos;
}

namespace blink {
class ImeEventGuard;
class LayerTreeView;
class PageScheduler;
class WidgetBaseClient;
class WidgetInputHandlerManager;
class WidgetCompositor;

// This class is the foundational class for all widgets that blink creates.
// (WebPagePopupImpl, WebFrameWidgetImpl) will contain an instance of this
// class. For simplicity purposes this class will be a member of those classes.
//
// Co-orindates handled in this class can be in the "blink coordinate space"
// which is scaled DSF baked in.
class PLATFORM_EXPORT WidgetBase : public mojom::blink::Widget,
                                   public LayerTreeViewDelegate,
                                   public mojom::blink::RenderInputRouterClient,
                                   public scheduler::WidgetScheduler::Delegate {
 public:
  WidgetBase(
      WidgetBaseClient* client,
      CrossVariantMojoAssociatedRemote<mojom::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::WidgetInterfaceBase> widget,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      bool hidden,
      bool never_composited,
      bool is_embedded,
      bool is_for_scalable_page);
  ~WidgetBase() override;

  // Initialize the compositor. |settings| is typically null. When |settings| is
  // null the default settings will be used, tests may provide a |settings|
  // object to override the defaults.
  //
  // TODO(dtapuska): The WebFrameWidgetImpl should be responsible for making
  // the FrameWidgetInputHandlerImpl, but currently it is done in the general
  // widget input handler classes directly, so we have to plumb through the
  // main-thread mojom implementation.
  // The `frame_widget_input_handler` must be invalidated when the WidgetBase is
  // destroyed/invalidated.
  void InitializeCompositing(
      PageScheduler& page_scheduler,
      const display::ScreenInfos& screen_infos,
      const cc::LayerTreeSettings* settings,
      base::WeakPtr<mojom::blink::FrameWidgetInputHandler>
          frame_widget_input_handler,
      WidgetBase* previous_widget);

  // Similar to `InitializeCompositing()` but for non-compositing widgets.
  // Exactly one of either `InitializeCompositing()` or this method must
  // be called before using the widget.
  void InitializeNonCompositing();

  // Shutdown the compositor. When `delay_release` is true, this detaches
  // `layer_tree_view_` but delays its actual deletion, so that calling this
  // function won't block on doing the release in the compositor thread.
  void Shutdown(bool delay_release);

  void DidFirstVisuallyNonEmptyPaint(base::TimeTicks&);

  // Set the compositor as visible. If |visible| is true, then the compositor
  // will request a new layer frame sink, begin producing frames from the
  // compositor scheduler, and in turn will update the document lifecycle.
  void SetCompositorVisible(bool visible);

  void AddPresentationCallback(
      uint32_t frame_token,
      base::OnceCallback<void(const viz::FrameTimingDetails&)> callback);

  void WarmUpCompositor();

#if BUILDFLAG(IS_APPLE)
  void AddCoreAnimationErrorCodeCallback(
      uint32_t frame_token,
      base::OnceCallback<void(gfx::CALayerResult)> callback);
#endif

  // mojom::blink::RenderInputRouterClient overrides;
  void GetWidgetInputHandler(
      mojo::PendingReceiver<mojom::blink::WidgetInputHandler> request,
      mojo::PendingRemote<mojom::blink::WidgetInputHandlerHost> host) override;
  void GetWidgetInputHandlerForInputOnViz(
      mojo::PendingReceiver<mojom::blink::WidgetInputHandler> request) override;
  void ShowContextMenu(ui::mojom::blink::MenuSourceType source_type,
                       const gfx::Point& location) override;
  void BindInputTargetClient(
      mojo::PendingReceiver<viz::mojom::blink::InputTargetClient> host)
      override;

  // mojom::blink::Widget overrides:
  void ForceRedraw(mojom::blink::Widget::ForceRedrawCallback callback) override;
  void UpdateVisualProperties(
      const VisualProperties& visual_properties) override;
  void UpdateScreenRects(const gfx::Rect& widget_screen_rect,
                         const gfx::Rect& window_screen_rect,
                         UpdateScreenRectsCallback callback) override;
  void WasHidden() override;
  void WasShown(bool was_evicted,
                mojom::blink::RecordContentToVisibleTimeRequestPtr
                    record_tab_switch_time_request) override;
  void RequestSuccessfulPresentationTimeForNextFrame(
      mojom::blink::RecordContentToVisibleTimeRequestPtr visible_time_request)
      override;
  void CancelSuccessfulPresentationTimeRequest() override;
  void SetupBrowserRenderInputRouterConnections(
      mojo::PendingReceiver<mojom::blink::RenderInputRouterClient>
          browser_request) override;

  // LayerTreeViewDelegate overrides:
  // Applies viewport related properties during a commit from the compositor
  // thread.
  void ApplyViewportChanges(const cc::ApplyViewportChangesArgs& args) override;
  void UpdateCompositorScrollState(
      const cc::CompositorCommitData& commit_data) override;
  void BeginMainFrame(const viz::BeginFrameArgs& args) override;
  void OnDeferMainFrameUpdatesChanged(bool) override;
  void OnDeferCommitsChanged(
      bool defer_status,
      cc::PaintHoldingReason reason,
      std::optional<cc::PaintHoldingCommitTrigger> trigger) override;
  void OnCommitRequested() override;
  void DidBeginMainFrame() override;
  void RequestNewLayerTreeFrameSink(
      LayerTreeFrameSinkCallback callback) override;
  void DidCommitAndDrawCompositorFrame() override;
  void DidObserveFirstScrollDelay(
      base::TimeDelta first_scroll_delay,
      base::TimeTicks first_scroll_timestamp) override;
  void WillCommitCompositorFrame() override;
  void DidCommitCompositorFrame(base::TimeTicks commit_start_time,
                                base::TimeTicks commit_finish_time) override;
  void DidCompletePageScaleAnimation() override;
  void RecordStartOfFrameMetrics() override;
  void RecordEndOfFrameMetrics(
      base::TimeTicks frame_begin_time,
      cc::ActiveFrameSequenceTrackers trackers) override;
  std::unique_ptr<cc::BeginMainFrameMetrics> GetBeginMainFrameMetrics()
      override;
  void BeginUpdateLayers() override;
  void EndUpdateLayers() override;
  void UpdateVisualState() override;
  void WillBeginMainFrame() override;
  void RunPaintBenchmark(int repeat_count,
                         cc::PaintBenchmarkResult& result) override;
  void ScheduleAnimationForWebTests() override;
  std::unique_ptr<cc::RenderFrameMetadataObserver> CreateRenderFrameObserver()
      override;

  // scheduler::WidgetScheduler::Delegate overrides:
  void RequestBeginMainFrameNotExpected(bool) override;

  cc::AnimationHost* AnimationHost() const;
  cc::AnimationTimeline* ScrollAnimationTimeline() const;
  cc::LayerTreeHost* LayerTreeHost() const;
  scheduler::WidgetScheduler* WidgetScheduler();

  // Returns if we should gather begin main frame metrics. If there is no
  // compositor thread this returns false.
  static bool ShouldRecordBeginMainFrameMetrics();

  // Set the current cursor relay to browser if necessary.
  void SetCursor(const ui::Cursor& cursor);

  // Dispatch the virtual keyboard and update text input state.
  void ShowVirtualKeyboardOnElementFocus();

  // Process the touch action.
  void ProcessTouchAction(cc::TouchAction touch_action);

  WidgetBaseInputHandler& input_handler() { return input_handler_; }

  WidgetInputHandlerManager* widget_input_handler_manager() {
    return widget_input_handler_manager_.get();
  }

  gfx::Rect CompositorViewportRect() const;

  WidgetBaseClient* client() { return client_; }

  void UpdateTooltipUnderCursor(const String& tooltip_text, TextDirection dir);
  // This function allows us to trigger a tooltip to show from a keypress. The
  // tooltip will be positioned relative to the gfx::Rect. That rect corresponds
  // to the focused element's bounds in widget-relative DIPS.
  void UpdateTooltipFromKeyboard(const String& tooltip_text,
                                 TextDirection dir,
                                 const gfx::Rect& bounds);
  void ClearKeyboardTriggeredTooltip();

  // Posts a task with the given delay, then calls ScheduleAnimation() on the
  // WidgetBaseClient.
  void RequestAnimationAfterDelay(const base::TimeDelta& delay,
                                  bool urgent = false);

  void ShowVirtualKeyboard();
  void UpdateSelectionBounds();
  void UpdateTextInputState();
  void ClearTextInputState();
  void ForceTextInputStateUpdate();
  void RequestCompositionUpdates(bool immediate_request, bool monitor_updates);
  void UpdateCompositionInfo(bool immediate_request);
  void SetFocus(mojom::blink::FocusState focus_state);
  bool has_focus() const { return has_focus_; }
  void MouseCaptureLost();
  void CursorVisibilityChange(bool is_visible);
  void QueueSyntheticEvent(
      std::unique_ptr<blink::WebCoalescedInputEvent> event);
  void SetEditCommandsForNextKeyEvent(
      Vector<mojom::blink::EditCommandPtr> edit_commands);
  void ImeSetComposition(const String& text,
                         const Vector<ui::ImeTextSpan>& ime_text_spans,
                         const gfx::Range& replacement_range,
                         int selection_start,
                         int selection_end);
  void ImeCommitText(const String& text,
                     const Vector<ui::ImeTextSpan>& ime_text_spans,
                     const gfx::Range& replacement_range,
                     int relative_cursor_pos);
  void ImeFinishComposingText(bool keep_selection);
  bool IsForProvisionalFrame();
  void FlushInputProcessedCallback();
  void CancelCompositionForPepper();

  void RequestPresentationAfterScrollAnimationEnd(
      mojom::blink::Widget::ForceRedrawCallback callback);

  void OnImeEventGuardStart(ImeEventGuard* guard);
  void OnImeEventGuardFinish(ImeEventGuard* guard);

  bool is_hidden() const { return is_hidden_; }
  void set_is_pasting(bool value) { is_pasting_ = value; }
  bool is_pasting() const { return is_pasting_; }
  void set_handling_select_range(bool value) { handling_select_range_ = value; }
  bool handling_select_range() const { return handling_select_range_; }

  LCDTextPreference ComputeLCDTextPreference() const;

  const viz::LocalSurfaceId& local_surface_id_from_parent() const {
    return local_surface_id_from_parent_;
  }

  // Called to get the position of the widget's window in screen
  // coordinates. Note, the window includes any decorations such as borders,
  // scrollbars, URL bar, tab strip, etc. if they exist.
  gfx::Rect WindowRect();

  // Called to get the view rect in screen coordinates. This is the actual
  // content view area, i.e. doesn't include any window decorations.
  gfx::Rect ViewRect();

  // Sets the pending window rects (in screen coordinates). This is used because
  // the window rect is delivered asynchronously to the browser. Pass in nullptr
  // to clear the pending window rect once the browser has acknowledged the
  // request.
  void SetPendingWindowRect(const gfx::Rect& rect);

  // Must correspond with a previous call to SetPendingWindowRect.
  void AckPendingWindowRect();

  // Returns the location/bounds of the widget (in screen coordinates).
  const gfx::Rect& WidgetScreenRect() const { return widget_screen_rect_; }

  // Returns the bounds of the screen the widget is contained in (in screen
  // coordinates).
  const gfx::Rect& WindowScreenRect() const { return window_screen_rect_; }

  // Sets the screen rects (in screen coordinates).
  void SetScreenRects(const gfx::Rect& widget_screen_rect,
                      const gfx::Rect& window_screen_rect);

  // Returns the visible viewport size.
  const gfx::Size& VisibleViewportSize() const {
    return visible_viewport_size_device_px_;
  }

  // Set the visible viewport size.
  void SetVisibleViewportSize(const gfx::Size& size_device_px) {
    visible_viewport_size_device_px_ = size_device_px;
  }

  // Some touch start which can trigger pointerdown will not be sent to the main
  // thread. And following touchend can't be dispatched. We want to count those
  // touchstart, touchend and pointerdown for EventTiming.
  void CountDroppedPointerDownForEventTiming(unsigned count);

  // Converts from DIPs to Blink coordinate space (ie. Viewport/Physical
  // pixels).
  gfx::PointF DIPsToBlinkSpace(const gfx::PointF& point);
  gfx::Point DIPsToRoundedBlinkSpace(const gfx::Point& point);
  gfx::Size DIPsToCeiledBlinkSpace(const gfx::Size& size);
  gfx::RectF DIPsToBlinkSpace(const gfx::RectF& rect);
  float DIPsToBlinkSpace(float scalar);

  // Converts from Blink coordinate (ie. Viewport/Physical pixels) space to
  // DIPs.
  gfx::PointF BlinkSpaceToDIPs(const gfx::PointF& point);
  gfx::Point BlinkSpaceToFlooredDIPs(const gfx::Point& point);
  gfx::Size BlinkSpaceToFlooredDIPs(const gfx::Size& size);
  gfx::Rect BlinkSpaceToEnclosedDIPs(const gfx::Rect& rect);
  gfx::RectF BlinkSpaceToDIPs(const gfx::RectF& rectF);

  void BindWidgetCompositor(
      mojo::PendingReceiver<mojom::blink::WidgetCompositor> receiver);

  base::WeakPtr<WidgetBase> GetWeakPtr() {
    return weak_ptr_factory_.GetWeakPtr();
  }

  // Update the surface allocation information, compositor viewport rect and
  // screen info on the widget.
  void UpdateSurfaceAndScreenInfo(
      const viz::LocalSurfaceId& new_local_surface_id,
      const gfx::Rect& compositor_viewport_pixel_rect,
      const display::ScreenInfos& new_screen_infos);
  // Similar to UpdateSurfaceAndScreenInfo but the screen info remains the same.
  void UpdateSurfaceAndCompositorRect(
      const viz::LocalSurfaceId& new_local_surface_id,
      const gfx::Rect& compositor_viewport_pixel_rect);
  // Similar to UpdateSurfaceAndScreenInfo but the surface allocation
  // and compositor viewport rect remains the same.
  void UpdateScreenInfo(const display::ScreenInfos& new_screen_infos);
  // Similar to UpdateSurfaceAndScreenInfo but the surface allocation
  // remains the same.
  void UpdateCompositorViewportAndScreenInfo(
      const gfx::Rect& compositor_viewport_pixel_rect,
      const display::ScreenInfos& new_screen_infos);
  // Similar to UpdateSurfaceAndScreenInfo but the surface allocation and screen
  // info remains the same.
  void UpdateCompositorViewportRect(
      const gfx::Rect& compositor_viewport_pixel_rect);
  const display::ScreenInfo& GetScreenInfo();

  // Accessors for information about available screens and the current screen.
  const display::ScreenInfos& screen_infos() const { return screen_infos_; }

  bool is_embedded() const { return is_embedded_; }

  // Returns the maximum bounds for buffers allocated for rasterization and
  // compositing.
  // Returns null if the compositing stack has not been initialized yet.
  std::optional<int> GetMaxRenderBufferBounds() const;

  bool WillBeDestroyed() const { return will_be_destroyed_; }

  void OnDevToolsSessionConnectionChanged(bool attached);

  // Helper to get the non-emulated device scale factor.
  float GetOriginalDeviceScaleFactor() const;

 private:
  static void AssertAreCompatible(const WidgetBase& a, const WidgetBase& b);

  bool CanComposeInline();
  void UpdateTextInputStateInternal(bool show_virtual_keyboard,
                                    bool immediate_request);
  bool ShouldHandleImeEvents();
  // Returns the range of the text that is being composed or the selection if
  // the composition does not exist.
  void GetCompositionRange(gfx::Range* range);
  void GetCompositionCharacterBounds(Vector<gfx::Rect>* bounds);
  ui::TextInputType GetTextInputType();

  // Returns true if the composition range or composition character bounds
  // should be sent to the browser process.
  bool ShouldUpdateCompositionInfo(const gfx::Range& range,
                                   const Vector<gfx::Rect>& bounds);

  // Sets the "hidden" state of this widget.  All modification of is_hidden_
  // should use this method so that we can properly inform the RenderThread of
  // our state.
  void SetHidden(bool hidden);

  // Called after the delay given in `RequestAnimationAfterDelay()`.
  void RequestAnimationAfterDelayTimerFired(TimerBase*);

  // Finishes the call to RequestNewLayerTreeFrameSink() once the
  // |gpu_channel_host| is available.
  // TODO(crbug.com/1278147): Clean up these parameters using either a struct or
  // saving on WidgetBase if kEstablishGpuChannelAsync launches.
  void FinishRequestNewLayerTreeFrameSink(
      const KURL& url,
      mojo::PendingReceiver<viz::mojom::blink::CompositorFrameSink>
          compositor_frame_sink_receiver,
      mojo::PendingRemote<viz::mojom::blink::CompositorFrameSinkClient>
          compositor_frame_sink_client,
      mojo::PendingReceiver<cc::mojom::blink::RenderFrameMetadataObserverClient>
          render_frame_metadata_observer_client_receiver,
      mojo::PendingRemote<cc::mojom::blink::RenderFrameMetadataObserver>
          render_frame_metadata_observer_remote,
      std::unique_ptr<RenderFrameMetadataObserverImpl>
          render_frame_metadata_observer,
      std::unique_ptr<cc::mojo_embedder::AsyncLayerTreeFrameSink::InitParams>
          params,
      LayerTreeFrameSinkCallback callback,
      scoped_refptr<gpu::GpuChannelHost> gpu_channel_host);

  // This will do exactly one of these, depending on the params:
  // - Detaches the LayerTreeView and attaches it to `new_widget`, if set
  // - Detaches the LayerTreeView without releasing its resources (instead
  //   this will be done asynchronously after a delay), if `delay_release` is
  //   true
  // - Disconnects and releases the LayerTreeView's resources synchronously,
  //   if `delay_release` is false
  void DisconnectLayerTreeView(WidgetBase* new_widget, bool delay_release);

  // Indicates that we are never visible, so never produce graphical output.
  const bool never_composited_;
  // Indicates this is for a child local root or a nested main frame.
  const bool is_embedded_ = false;
  // Indicates that this widget is for a top level frame, or a GuestView.
  const bool is_for_scalable_page_ = false;
  // Set true by initialize functions, used to check that only one is called.
  bool initialized_ = false;

  // The client which handles behaviour specific to the type of widget.
  // It's the owner of the widget and will outlive this class.
  const raw_ptr<WidgetBaseClient> client_;

  mojo::AssociatedRemote<mojom::blink::WidgetHost> widget_host_;
  mojo::AssociatedReceiver<mojom::blink::Widget> receiver_;

  mojo::Receiver<mojom::blink::RenderInputRouterClient> browser_input_receiver_{
      this};
  mojo::Receiver<mojom::blink::RenderInputRouterClient> viz_input_receiver_{
      this};

  std::unique_ptr<LayerTreeView> layer_tree_view_;
  scoped_refptr<WidgetInputHandlerManager> widget_input_handler_manager_;
  scoped_refptr<scheduler::WidgetScheduler> widget_scheduler_;
  bool has_focus_ = false;
  WidgetBaseInputHandler input_handler_{this};
  scoped_refptr<WidgetCompositor> widget_compositor_;

  // Stores the current selection bounds.
  gfx::Rect selection_focus_rect_;
  gfx::Rect selection_anchor_rect_;
  gfx::Rect selection_bounding_box_;

  // Stores the current composition character bounds.
  Vector<gfx::Rect> composition_character_bounds_;

  // Stores the current composition range.
  gfx::Range composition_range_ = gfx::Range::InvalidRange();

  // True if the IME requests updated composition info.
  bool monitor_composition_info_ = false;

  // Stores information about the current text input.
  blink::WebTextInputInfo text_input_info_;

  // Stores the current text input type of |webwidget_|.
  ui::TextInputType text_input_type_ = ui::TEXT_INPUT_TYPE_NONE;

  // Stores the current text input mode of |webwidget_|.
  ui::TextInputMode text_input_mode_ = ui::TEXT_INPUT_MODE_DEFAULT;

  // Stores the current virtualkeyboardpolicy of |webwidget_|.
  ui::mojom::VirtualKeyboardPolicy vk_policy_ =
      ui::mojom::VirtualKeyboardPolicy::AUTO;

  // Stores the current control and selection bounds of |webwidget_|
  // that are used to position the candidate window during IME composition.
  // These are stored physical pixels and are relative to the root frame.
  gfx::Rect frame_control_bounds_;
  gfx::Rect frame_selection_bounds_;

  // Stores the current text input flags of |webwidget_|.
  int text_input_flags_ = 0;

  // Indicates whether currently focused input field has next/previous focusable
  // form input field.
  int next_previous_flags_;

  // Stores the current type of composition text rendering of |webwidget_|.
  bool can_compose_inline_ = true;

  // Used to inform didChangeSelection() when it is called in the context
  // of handling a FrameInputHandler::SelectRange IPC.
  bool handling_select_range_ = false;

  // Whether or not this RenderWidget is currently pasting.
  bool is_pasting_ = false;

  // Object to record tab switch time into this RenderWidget
  ContentToVisibleTimeReporter tab_switch_time_recorder_;

  // Info about available screens and which is currently showing the WidgetBase.
  // Rects in these structures do not include any scaling by device scale
  // factor, so are in DIPs, not blink coordinate space.
  display::ScreenInfos screen_infos_;

  viz::LocalSurfaceId local_surface_id_from_parent_;

  // It is possible that one ImeEventGuard is nested inside another
  // ImeEventGuard. We keep track of the outermost one, and update it as needed.
  raw_ptr<ImeEventGuard> ime_event_guard_ = nullptr;

  // The screen rects of the view and the window that contains it. These do not
  // include any scaling by device scale factor, so are logical pixels not
  // physical device pixels.
  gfx::Rect widget_screen_rect_;
  gfx::Rect window_screen_rect_;

  // While we are waiting for the browser to update window sizes, we track the
  // pending size temporarily.
  int pending_window_rect_count_ = 0;

  // A pending window rect that is inflight and hasn't been acknowledged by the
  // browser yet. This should only be set if |pending_window_rect_count_| is
  // non-zero.
  std::optional<gfx::Rect> pending_window_rect_;

  // The size of the visible viewport (in device pixels).
  gfx::Size visible_viewport_size_device_px_;

  // The AnimationTimeline for smooth scrolls in this widget.
  scoped_refptr<cc::AnimationTimeline> scroll_animation_timeline_;

  // Indicates that we shouldn't bother generated paint events.
  bool is_hidden_;

  // The task_runner on which Widget mojo interfaces are bound.
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // Delayed callback to ensure we have only one delayed ScheduleAnimation()
  // call going at a time.
  TaskRunnerTimer<WidgetBase> request_animation_after_delay_timer_;

  // The task runner on the main thread used for compositor tasks.
  scoped_refptr<base::SingleThreadTaskRunner>
      main_thread_compositor_task_runner_;
  base::PlatformThreadId main_thread_id_ = base::kInvalidThreadId;

  // The maximum bounds for buffers allocated for rasterization and compositing.
  // Set when the compositor is initialized.
  std::optional<int> max_render_buffer_bounds_gpu_;
  std::optional<int> max_render_buffer_bounds_sw_;

  // Tracks when the compositing setup for this widget has been torn down or
  // disconnected in preparation to destroy this widget.
  bool will_be_destroyed_ = false;

  // To store Viz side `WidgetInputHandler` receiver in case it arrives before
  // Browser side. We do not want to start processing messages on this interface
  // until a WidgetInputHandlerHost is bound which only happens after Browser
  // side `WidgetInputHandler` call is received.
  std::optional<mojo::PendingReceiver<mojom::blink::WidgetInputHandler>>
      pending_widget_input_handler_ = std::nullopt;

  base::WeakPtrFactory<WidgetBase> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_WIDGET_BASE_H_
