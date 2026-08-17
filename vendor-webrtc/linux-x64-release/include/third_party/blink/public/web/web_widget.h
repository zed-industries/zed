/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_WIDGET_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_WIDGET_H_

#include "third_party/blink/public/common/metrics/document_update_reason.h"
#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/web/web_lifecycle_update.h"
#include "ui/gfx/geometry/size.h"

namespace cc {
class LayerTreeHost;
class LayerTreeSettings;
}

namespace display {
struct ScreenInfo;
struct ScreenInfos;
}  // namespace display

namespace gfx {
class PointF;
class Rect;
class Vector2dF;
}  // namespace gfx

namespace ui {
class Cursor;
}

namespace blink {
struct VisualProperties;
class WebCoalescedInputEvent;
class WebHitTestResult;

class WebWidget {
 public:
  // Initialize compositing. This will create a LayerTreeHost but will not
  // allocate a frame sink or begin producing frames until SetCompositorVisible
  // is called. |settings| is typically null. When |settings| is null
  // the default settings will be used, tests may provide a |settings| object to
  // override the defaults.
  virtual void InitializeCompositing(const display::ScreenInfos& screen_info,
                                     const cc::LayerTreeSettings* settings) = 0;

  // Set the compositor as visible. If |visible| is true, then the compositor
  // will request a new layer frame sink and begin producing frames from the
  // compositor.
  virtual void SetCompositorVisible(bool visible) = 0;

  // Asks the compositor to request warming up and request a new frame sink
  // speculatively. This is an experimental function and only used if
  // `kWarmUpCompositor` is enabled. Please see crbug.com/41496019
  // for more details.
  virtual void WarmUpCompositor() = 0;

  // Returns the current size of the WebWidget.
  virtual gfx::Size Size() { return gfx::Size(); }

  // Called to resize the WebWidget.
  virtual void Resize(const gfx::Size&) {}

  // Called to run through the entire set of document lifecycle phases needed
  // to render a frame of the web widget. This MUST be called before Paint,
  // and it may result in calls to WebViewClient::DidInvalidateRect (for
  // non-composited WebViews).
  // |reason| must be used to indicate the source of the
  // update for the purposes of metrics gathering.
  virtual void UpdateAllLifecyclePhases(DocumentUpdateReason reason) {
    UpdateLifecycle(WebLifecycleUpdate::kAll, reason);
  }

  // UpdateLifecycle is used to update to a specific lifestyle phase, as given
  // by |LifecycleUpdate|. To update all lifecycle phases, use
  // UpdateAllLifecyclePhases.
  // |reason| must be used to indicate the source of the
  // update for the purposes of metrics gathering.
  virtual void UpdateLifecycle(WebLifecycleUpdate requested_update,
                               DocumentUpdateReason reason) {}

  // Do a hit test at given point and return the WebHitTestResult.
  virtual WebHitTestResult HitTestResultAt(const gfx::PointF&) = 0;

  // Called to inform the WebWidget of an input event.
  virtual WebInputEventResult HandleInputEvent(const WebCoalescedInputEvent&) {
    return WebInputEventResult::kNotHandled;
  }

  // Send any outstanding touch events. Touch events need to be grouped together
  // and any changes since the last time a touch event is going to be sent in
  // the new touch event.
  virtual WebInputEventResult DispatchBufferedTouchEvents() {
    return WebInputEventResult::kNotHandled;
  }

  // Called to inform the WebWidget of the mouse cursor's visibility.
  virtual void SetCursorVisibilityState(bool is_visible) {}

  // Called to inform the WebWidget that it has gained or lost keyboard focus.
  virtual void SetFocus(bool) {}

  // Returns the state of focus for the WebWidget.
  virtual bool HasFocus() { return false; }

  virtual void SetCursor(const ui::Cursor& cursor) = 0;

  // Get the current tooltip text.
  virtual WebString GetLastToolTipTextForTesting() const { return WebString(); }

  // Whether or not the widget is in the process of handling input events.
  virtual bool HandlingInputEvent() = 0;

  // Set state that the widget is in the process of handling input events.
  virtual void SetHandlingInputEvent(bool handling) = 0;

  // Process the input event, blocking until complete.
  virtual void ProcessInputEventSynchronouslyForTesting(
      const WebCoalescedInputEvent&) = 0;

  // Dispatches the input event asynchronously, without blocking.
  virtual void DispatchNonBlockingEventForTesting(
      std::unique_ptr<WebCoalescedInputEvent> event) = 0;

  virtual void DidOverscrollForTesting(
      const gfx::Vector2dF& overscroll_delta,
      const gfx::Vector2dF& accumulated_overscroll,
      const gfx::PointF& position_in_viewport,
      const gfx::Vector2dF& velocity_in_viewport) {}

  // Requests the text input state be updated. If anything has changed the
  // updated state will be sent to the browser.
  virtual void UpdateTextInputState() = 0;

  // Flush any pending input.
  virtual void FlushInputProcessedCallback() = 0;

  // Cancel the current composition.
  virtual void CancelCompositionForPepper() = 0;

  // Requests the selection bounds be updated.
  virtual void UpdateSelectionBounds() = 0;

  // Request the virtual keyboard be shown.
  virtual void ShowVirtualKeyboard() = 0;

  // Apply the visual properties to the widget.
  virtual void ApplyVisualProperties(
      const VisualProperties& visual_properties) = 0;

  // Returns information about the screen where this view's widgets are being
  // displayed.
  virtual const display::ScreenInfo& GetScreenInfo() = 0;

  // Returns information about all available screens.
  virtual const display::ScreenInfos& GetScreenInfos() = 0;

  // Returns original (non-emulated) information about the screen where this
  // view's widgets are being displayed.
  virtual const display::ScreenInfo& GetOriginalScreenInfo() = 0;

  // Returns original (non-emulated) information about all available screens.
  virtual const display::ScreenInfos& GetOriginalScreenInfos() = 0;

  // Called to get the position of the widget's window in screen
  // coordinates. Note, the window includes any decorations such as borders,
  // scrollbars, URL bar, tab strip, etc. if they exist.
  virtual gfx::Rect WindowRect() = 0;

  // Called to get the view rect in screen coordinates. This is the actual
  // content view area, i.e. doesn't include any window decorations.
  virtual gfx::Rect ViewRect() = 0;

  // Sets the screen rects (in screen coordinates).
  virtual void SetScreenRects(const gfx::Rect& widget_screen_rect,
                              const gfx::Rect& window_screen_rect) = 0;

  // Returns the visible viewport size (in device pixel screen coordinates).
  virtual gfx::Size VisibleViewportSize() = 0;

  // Returns the emulator scale.
  virtual float GetEmulatorScale() { return 1.0f; }

  virtual bool IsHidden() const = 0;

 protected:
  ~WebWidget() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_WIDGET_H_
