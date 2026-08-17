// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_FRAME_WIDGET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_FRAME_WIDGET_H_

#include <optional>
#include <vector>

#include "base/time/time.h"
#include "base/types/optional_ref.h"
#include "cc/input/browser_controls_offset_tag_modifications.h"
#include "mojo/public/mojom/base/text_direction.mojom-blink.h"
#include "services/viz/public/mojom/compositing/frame_sink_id.mojom-blink.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"
#include "third_party/blink/public/mojom/manifest/display_mode.mojom-blink.h"
#include "third_party/blink/public/platform/web_text_input_info.h"
#include "third_party/blink/public/platform/web_text_input_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/base/ime/mojom/text_input_state.mojom-blink.h"
#include "ui/base/ime/mojom/virtual_keyboard_types.mojom-blink.h"
#include "ui/base/mojom/window_show_state.mojom-blink-forward.h"
#include "ui/gfx/mojom/delegated_ink_metadata.mojom-blink.h"

namespace cc {
class AnimationHost;
class AnimationTimeline;
class DrawImage;
enum class EventListenerClass;
enum class EventListenerProperties;
class Layer;
class LayerTreeSettings;
class LayerTreeDebugState;
struct ElementId;
}  // namespace cc

namespace display {
struct ScreenInfo;
struct ScreenInfos;
}  // namespace display

namespace ui {
class Cursor;
}  // namespace ui

namespace viz {
struct FrameTimingDetails;
}  // namespace viz

namespace blink {

class AnimationFrameTimingInfo;
class LocalFrame;
// In interface exposed within Blink from local root frames that provides
// local-root specific things related to compositing and input. This
// class extends the FrameWidgetInputHandler implementation. All API calls
// on this class occur on the main thread. input/FrameWidgetInputHandlerImpl
// which also implements the FrameWidgetInputHandler interface runs on the
// compositor thread and proxies calls to this class.
class PLATFORM_EXPORT FrameWidget {
 public:
  virtual ~FrameWidget();

  // Returns the compositors's AnimationHost for the widget.
  virtual cc::AnimationHost* AnimationHost() const = 0;

  // Returns the compositors's AnimationTimeline for the widget.
  virtual cc::AnimationTimeline* ScrollAnimationTimeline() const = 0;

  // Set the browser's behavior when overscroll happens, e.g. whether to glow
  // or navigate.
  virtual void SetOverscrollBehavior(
      const cc::OverscrollBehavior& overscroll_behavior) = 0;

  // Posts a task with the given delay, then requests an animation frame from
  // the compositor (ie LayerTreeHost::SetNeedsAnimate()).
  virtual void RequestAnimationAfterDelay(const base::TimeDelta&,
                                          bool urgent) = 0;

  // Sets the root layer. The |layer| can be null when detaching the root layer.
  virtual void SetRootLayer(scoped_refptr<cc::Layer> layer) = 0;

  // Image decode functionality.
  virtual void RequestDecode(const cc::DrawImage&,
                             base::OnceCallback<void(bool)>) = 0;

  // Forwards to `WebFrameWidget::NotifyPresentationTime()`.
  // `presentation_callback` will be fired when the corresponding renderer frame
  // is presented to the user. If the presentation is successful, the argument
  // passed to the callback is the presentation timestamp; otherwise, it would
  // be timestamp of when the failure is detected.
  virtual void NotifyPresentationTime(
      base::OnceCallback<void(const viz::FrameTimingDetails&)>
          presentation_callback) = 0;

  // Enable or disable BeginMainFrameNotExpected signals from the compositor,
  // which are consumed by the blink scheduler.
  virtual void RequestBeginMainFrameNotExpected(bool request) = 0;

  // A stable numeric Id for the local root's compositor. For tracing/debugging
  // purposes.
  virtual int GetLayerTreeId() = 0;

  // Return the LayerTreeSettings from the compositor. These are constant from
  // the time the compositor is created. This may return null if the widget
  // does not composite.
  virtual const cc::LayerTreeSettings* GetLayerTreeSettings() = 0;

  // Sets the state of the browser controls. (Used for URL bar animations.)
  virtual void UpdateBrowserControlsState(
      cc::BrowserControlsState constraints,
      cc::BrowserControlsState current,
      bool animate,
      base::optional_ref<const cc::BrowserControlsOffsetTagModifications>
          offset_tag_modifications) = 0;

  // Set or get what event handlers exist in the document contained in the
  // WebWidget in order to inform the compositor thread if it is able to handle
  // an input event, or it needs to pass it to the main thread to be handled.
  // The class is the type of input event, and for each class there is a
  // properties defining if the compositor thread can handle the event.
  virtual void SetEventListenerProperties(cc::EventListenerClass,
                                          cc::EventListenerProperties) = 0;
  virtual cc::EventListenerProperties EventListenerProperties(
      cc::EventListenerClass) const = 0;

  // Returns the DisplayMode in use for the widget.
  virtual mojom::blink::DisplayMode DisplayMode() const = 0;

  // Returns the WindowShowState in use for the widget.
  virtual ui::mojom::blink::WindowShowState WindowShowState() const = 0;

  // Returns the CanResize value of the widget.
  virtual bool Resizable() const = 0;

  // Returns the viewport segments for the widget.
  virtual const std::vector<gfx::Rect>& ViewportSegments() const = 0;

  // Sets the ink metadata on the layer tree host
  virtual void SetDelegatedInkMetadata(
      std::unique_ptr<gfx::DelegatedInkMetadata> metadata) = 0;

  // For a scrollbar scroll action, requests that a gesture of |injected_type|
  // be reissued at a later point in time. |injected_type| is required to be one
  // of GestureScroll{Begin,Update,End}. The dispatched gesture will scroll the
  // ScrollableArea identified by |scrollable_area_element_id| by the given
  // delta + granularity.
  // See also InputHandlerProxy::InjectScrollbarGestureScroll() which may
  // shortcut callers of this function for composited scrollbars.
  virtual void InjectScrollbarGestureScroll(
      const gfx::Vector2dF& delta,
      ui::ScrollGranularity granularity,
      cc::ElementId scrollable_area_element_id,
      WebInputEvent::Type injected_type) = 0;

  // Called when the cursor for the widget changes.
  virtual void DidChangeCursor(const ui::Cursor&) = 0;

  // Return the composition character in window coordinates.
  virtual void GetCompositionCharacterBoundsInWindow(
      Vector<gfx::Rect>* bounds_in_dips) = 0;

  virtual bool HasImeRenderWidgetHost() const { return false; }

  // Called to send new cursor anchor info data to the browser.
  virtual void UpdateCursorAnchorInfo(bool update_requested) = 0;

  virtual gfx::Range CompositionRange() = 0;
  // Returns ime_text_spans and corresponding window coordinates for the list
  // of given spans.
  virtual Vector<ui::mojom::blink::ImeTextSpanInfoPtr> GetImeTextSpansInfo(
      const std::vector<ui::ImeTextSpan>& ime_text_spans) = 0;
  virtual WebTextInputInfo TextInputInfo() = 0;
  virtual ui::mojom::blink::VirtualKeyboardVisibilityRequest
  GetLastVirtualKeyboardVisibilityRequest() = 0;

  // Return the edit context bounds in window coordinates.
  virtual void GetEditContextBoundsInWindow(
      std::optional<gfx::Rect>* control_bounds,
      std::optional<gfx::Rect>* selection_bounds) = 0;

  virtual int32_t ComputeWebTextInputNextPreviousFlags() = 0;
  virtual void ResetVirtualKeyboardVisibilityRequest() = 0;

  // Return the selection bounds in window coordinates. Returns true if the
  // bounds returned were different than the passed in focus and anchor bounds.
  virtual bool GetSelectionBoundsInWindow(gfx::Rect* focus,
                                          gfx::Rect* anchor,
                                          gfx::Rect* bounding_box,
                                          base::i18n::TextDirection* focus_dir,
                                          base::i18n::TextDirection* anchor_dir,
                                          bool* is_anchor_first) = 0;

  // Clear any cached text input state.
  virtual void ClearTextInputState() = 0;

  // This message sends a string being composed with an input method.
  virtual bool SetComposition(const String& text,
                              const Vector<ui::ImeTextSpan>& ime_text_spans,
                              const gfx::Range& replacement_range,
                              int selection_start,
                              int selection_end) = 0;

  // This message deletes the current composition, inserts specified text, and
  // moves the cursor.
  virtual void CommitText(const String& text,
                          const Vector<ui::ImeTextSpan>& ime_text_spans,
                          const gfx::Range& replacement_range,
                          int relative_cursor_pos) = 0;

  // This message inserts the ongoing composition.
  virtual void FinishComposingText(bool keep_selection) = 0;

  virtual bool IsProvisional() = 0;
  virtual cc::ElementId GetScrollableContainerIdAt(
      const gfx::PointF& point) = 0;

  virtual bool ShouldHandleImeEvents() { return false; }

  virtual void SetEditCommandsForNextKeyEvent(
      Vector<mojom::blink::EditCommandPtr> edit_commands) = 0;

  // Returns information about the screen currently showing the widget.
  virtual const display::ScreenInfo& GetScreenInfo() = 0;

  // Returns information about available screens and the current screen.
  virtual const display::ScreenInfos& GetScreenInfos() = 0;

  // Called to get the position of the widget's window in screen
  // coordinates. Note, the window includes any decorations such as borders,
  // scrollbars, URL bar, tab strip, etc. if they exist.
  virtual gfx::Rect WindowRect() = 0;

  // Called to get the view rect in screen coordinates. This is the actual
  // content view area, i.e. doesn't include any window decorations.
  virtual gfx::Rect ViewRect() = 0;

  // Converts from Blink coordinate (ie. Viewport/Physical pixels) space to
  // DIPs.
  virtual gfx::RectF BlinkSpaceToDIPs(const gfx::RectF&) = 0;
  virtual gfx::Rect BlinkSpaceToEnclosedDIPs(const gfx::Rect&) = 0;
  virtual gfx::Size BlinkSpaceToFlooredDIPs(const gfx::Size& size) = 0;

  // Converts from DIPs to Blink coordinate space (ie. Viewport/Physical
  // pixels).
  virtual gfx::PointF DIPsToBlinkSpace(const gfx::PointF& point) = 0;
  virtual gfx::Point DIPsToRoundedBlinkSpace(const gfx::Point& point) = 0;
  virtual float DIPsToBlinkSpace(float scalar) = 0;

  virtual void RequestMouseLock(
      bool has_transient_user_activation,
      bool request_unadjusted_movement,
      mojom::blink::WidgetInputHandlerHost::RequestMouseLockCallback
          callback) = 0;

  // Mouse capture has been lost.
  virtual void MouseCaptureLost() = 0;

  // Determines whether composition can happen inline.
  virtual bool CanComposeInline() = 0;

  // Determines if IME events should be sent to Plugin instead of processed to
  // the currently focused frame.
  virtual bool ShouldDispatchImeEventsToPlugin() = 0;

  // Set the composition in plugin.
  virtual void ImeSetCompositionForPlugin(
      const String& text,
      const Vector<ui::ImeTextSpan>& ime_text_spans,
      const gfx::Range& replacement_range,
      int selection_start,
      int selection_end) = 0;

  // Commit the text to plugin.
  virtual void ImeCommitTextForPlugin(
      const String& text,
      const Vector<ui::ImeTextSpan>& ime_text_spans,
      const gfx::Range& replacement_range,
      int relative_cursor_pos) = 0;

  // Indicate composition is complete to plugin.
  virtual void ImeFinishComposingTextForPlugin(bool keep_selection) = 0;

  // Returns the FrameSinkId for this widget which is used for identifying
  // frames submitted from the compositor.
  virtual const viz::FrameSinkId& GetFrameSinkId() = 0;

  // Returns the raster scale factor for the local root frame associated with
  // this widget, taking into account its transform to main frame space.
  virtual float GetCompositingScaleFactor() = 0;

  // Get and set the configuration for the debugging overlay managed by the
  // underlying LayerTreeHost. This may return null if the widget does not
  // composite.
  virtual const cc::LayerTreeDebugState* GetLayerTreeDebugState() = 0;
  virtual void SetLayerTreeDebugState(const cc::LayerTreeDebugState& state) = 0;

  // Set whether or not this widget should be throttled if it sends
  // CompositorFrames while widget is hidden.  By default, it should throttle,
  // since we should be smart enough not to send them.  However,
  // PictureInPicture requires that we are allowed to continue to produce
  // CompositorFrames even if they're discarded by viz, since those frames are a
  // by-product of producing the content that does make it to the picture-in-
  // picture window.  Ideally, we would know not to send the extra
  // CompositorFrames.  See https://crbug.com/1232173 for more details.
  virtual void SetMayThrottleIfUndrawnFrames(
      bool may_throttle_if_undrawn_frames) = 0;

  // Returns, in physical pixels, the amount that the widget has been resized
  // by the virtual keyboard. The virtual keyboard always insets a widget from
  // the bottom so only the height can be affected. Only the outermost main
  // frame's widget returns a non-zero value.
  virtual int GetVirtualKeyboardResizeHeight() const = 0;

  virtual void OnTaskCompletedForFrame(base::TimeTicks start_time,
                                       base::TimeTicks end_time,
                                       LocalFrame*) = 0;

  // Implementation of
  // https://w3c.github.io/long-animation-frames/#record-rendering-time (the
  // other parameters are recorded earlier).
  virtual AnimationFrameTimingInfo* RecordRenderingUpdateEndTime(
      base::TimeTicks) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_FRAME_WIDGET_H_
