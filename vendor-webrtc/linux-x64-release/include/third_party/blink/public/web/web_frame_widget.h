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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_WIDGET_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_WIDGET_H_

#include <stdint.h>

#include <vector>

#include "base/functional/callback_forward.h"
#include "base/types/pass_key.h"
#include "components/viz/common/surfaces/frame_sink_id.h"
#include "third_party/blink/public/common/page/drag_operation.h"
#include "third_party/blink/public/mojom/page/widget.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_touch_action.h"
#include "third_party/blink/public/web/web_widget.h"
#include "third_party/skia/include/core/SkColor.h"
#include "ui/base/dragdrop/mojom/drag_drop_types.mojom-shared.h"
#include "ui/gfx/ca_layer_result.h"
#include "ui/gfx/geometry/rect.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace cc {
struct ApplyViewportChangesArgs;
class LayerTreeHost;
}  // namespace cc

namespace gfx {
class PointF;
class RectF;
}  // namespace gfx

namespace viz {
struct FrameTimingDetails;
class LocalSurfaceId;
}  // namespace viz

namespace blink {

class FrameWidgetTestHelper;
class WebDragData;
class WebInputMethodController;
class WebLocalFrame;
class WebNonCompositedWidgetClient;

class WebFrameWidget : public WebWidget {
 public:
  // Similar to `InitializeCompositing()` but for non-compositing widgets.
  // Exactly one of either `InitializeCompositing()` or this method must
  // be called before using the widget.
  virtual void InitializeNonCompositing(
      WebNonCompositedWidgetClient* client) = 0;

  // Similar to `WebWidget::InitializeCompositing()` but for cases where there
  // is a `previous_widget` whose compositing setup should be reused instead of
  // initializing a new compositor.
  virtual void InitializeCompositingFromPreviousWidget(
      const display::ScreenInfos& screen_info,
      const cc::LayerTreeSettings* settings,
      WebFrameWidget& previous_widget) = 0;

  // Returns the local root of this WebFrameWidget.
  virtual WebLocalFrame* LocalRoot() const = 0;

  // Converts from Blink coordinate (ie. Viewport/Physical pixels) space to
  // DIPs.
  virtual gfx::RectF BlinkSpaceToDIPs(const gfx::RectF& rect) = 0;
  virtual gfx::Rect BlinkSpaceToEnclosedDIPs(const gfx::Rect& rect) = 0;

  // Current instance of the active WebInputMethodController, that is, the
  // WebInputMethodController corresponding to (and owned by) the focused
  // WebLocalFrameImpl. It will return nullptr when there are no focused
  // frames inside this WebFrameWidget.
  virtual WebInputMethodController* GetActiveWebInputMethodController()
      const = 0;

  // Callback methods when a drag-and-drop operation is trying to drop something
  // on the WebFrameWidget.
  virtual void DragTargetDragEnter(
      const WebDragData&,
      const gfx::PointF& point_in_viewport,
      const gfx::PointF& screen_point,
      DragOperationsMask operations_allowed,
      uint32_t key_modifiers,
      base::OnceCallback<void(ui::mojom::DragOperation, bool)> callback) = 0;
  virtual void DragTargetDragOver(
      const gfx::PointF& point_in_viewport,
      const gfx::PointF& screen_point,
      DragOperationsMask operations_allowed,
      uint32_t key_modifiers,
      base::OnceCallback<void(ui::mojom::DragOperation, bool)> callback) = 0;
  virtual void DragTargetDragLeave(const gfx::PointF& point_in_viewport,
                                   const gfx::PointF& screen_point) = 0;
  virtual void DragTargetDrop(const WebDragData&,
                              const gfx::PointF& point_in_viewport,
                              const gfx::PointF& screen_point,
                              uint32_t key_modifiers,
                              base::OnceClosure callback) = 0;

  // Notifies the WebFrameWidget that a drag has terminated.
  virtual void DragSourceEndedAt(const gfx::PointF& point_in_viewport,
                                 const gfx::PointF& screen_point,
                                 ui::mojom::DragOperation,
                                 base::OnceClosure callback) = 0;

  // Notifies the WebFrameWidget that the system drag and drop operation has
  // ended.
  virtual void DragSourceSystemDragEnded() = 0;

  // Disables Drag and drop on this widget. Any drag activity will be
  // immediately canceled.
  virtual void DisableDragAndDrop() = 0;

  // Sets the inherited effective touch action on an out-of-process iframe.
  virtual void SetInheritedEffectiveTouchAction(WebTouchAction) {}

  // Returns the currently focused WebLocalFrame (if any) inside this
  // WebFrameWidget. That is a WebLocalFrame which is focused and shares the
  // same LocalRoot() as this WebFrameWidget's LocalRoot().
  virtual WebLocalFrame* FocusedWebLocalFrameInWidget() const = 0;

  // Scrolls the editable element which is currently focused in (a focused frame
  // inside) this widget into view. The scrolling might end with a final zooming
  // into the editable region which is performed in the main frame process.
  virtual bool ScrollFocusedEditableElementIntoView() = 0;

  // Applies viewport related properties that are normally provided by the
  // compositor. Useful for tests that don't use a compositor.
  virtual void ApplyViewportChangesForTesting(
      const cc::ApplyViewportChangesArgs& args) = 0;

  // The |callback| will be fired when the corresponding renderer frame is
  // presented to the user. If the presentation is successful, the argument
  // passed to the callback is the presentation timestamp; otherwise, it would
  // be timestamp of when the failure is detected.
  virtual void NotifyPresentationTime(
      base::OnceCallback<void(const viz::FrameTimingDetails&)> callback) = 0;

#if BUILDFLAG(IS_APPLE)
  virtual void NotifyCoreAnimationErrorCode(
      base::OnceCallback<void(gfx::CALayerResult)> callback) = 0;
#endif

  // Instructs devtools to pause loading of the frame as soon as it's shown
  // until explicit command from the devtools client.
  virtual void WaitForDebuggerWhenShown() = 0;

  // Scales the text in the frame by a factor of text_zoom_factor.
  virtual void SetTextZoomFactor(float text_zoom_factor) = 0;
  // Returns the current text zoom factor, where 1.0 is the normal size, > 1.0
  // is scaled up and < 1.0 is scaled down.
  virtual float TextZoomFactor() = 0;

  // Overlay this frame with a solid color. Only valid for the main frame's
  // widget.
  virtual void SetMainFrameOverlayColor(SkColor) = 0;

  // Add an edit command to be processed as the default action if the next
  // keyboard event is unhandled.
  virtual void AddEditCommandForNextKeyEvent(const WebString& name,
                                             const WebString& value) = 0;

  // Clear any active edit commands that are pending.
  virtual void ClearEditCommands() = 0;

  // If the widget is currently handling a paste.
  virtual bool IsPasting() = 0;

  // If the widget is currently selecting a range.
  virtual bool HandlingSelectRange() = 0;

  // Calculates the selection bounds in the root frame. Returns bounds unchanged
  // when there is no focused frame. Returns the caret bounds if the selection
  // range is empty.
  virtual void CalculateSelectionBounds(gfx::Rect& anchor_in_root_frame,
                                        gfx::Rect& focus_in_root_frame) = 0;

  // Returns true if a pinch gesture is currently active in main frame.
  virtual bool PinchGestureActiveInMainFrame() = 0;

  // Returns page scale in main frame..
  virtual float PageScaleInMainFrame() = 0;

  // Override the zoom level for testing.
  virtual void SetZoomLevelForTesting(double zoom_level) = 0;

  // Remove the override for zoom level.
  virtual void ResetZoomLevelForTesting() = 0;

  // Override the device scale factor for testing.
  virtual void SetDeviceScaleFactorForTesting(float factor) = 0;

  // Get the viewport segments for this widget.
  // See
  // https://github.com/WICG/visual-viewport/blob/gh-pages/segments-explainer/SEGMENTS-EXPLAINER.md
  virtual const std::vector<gfx::Rect>& ViewportSegments() const = 0;

  // Release any mouse lock or pointer capture held. This is used to reset
  // state between WebTest runs.
  virtual void ReleaseMouseLockAndPointerCaptureForTesting() = 0;

  // Returns the FrameSinkId for this widget which is used for identifying
  // frames submitted from the compositor.
  virtual const viz::FrameSinkId& GetFrameSinkId() = 0;

  // Returns a FrameWidgetTestHelper if this widget was created using
  // `FrameWidgetTestHelper::CreateTestWebFrameWidget()`.
  virtual FrameWidgetTestHelper* GetFrameWidgetTestHelperForTesting() = 0;

  // This should be called for the local root frame before calling the final
  // UpdateAllLifecyclePhases() just before dumping pixels.
  virtual void PrepareForFinalLifecyclUpdateForTesting() = 0;

  // Returns the current zoom level.  0 is "original size", and each increment
  // above or below represents zooming 20% larger or smaller to default limits
  // of 300% and 50% of original size, respectively.  Only plugins use
  // non whole-numbers, since they might choose to have specific zoom level so
  // that fixed-width content is fit-to-page-width, for example.
  virtual double GetZoomLevel() = 0;
  // Changes the zoom level to the specified level, clamping at the limits
  // defined by the associated `webView`.
  virtual void SetZoomLevel(double zoom_level) = 0;

  // Returns the cumulative effect of the CSS "zoom" property on the embedding
  // element of this widget (if any) and all of its WebFrame ancestors.
  virtual double GetCSSZoomFactor() const = 0;

  // Update the LocalSurfaceId used for frames produced by this widget.
  virtual void ApplyLocalSurfaceIdUpdate(const viz::LocalSurfaceId& id) = 0;

 private:
  // This is a private virtual method so we don't expose cc::LayerTreeHost
  // outside of this class. Friend classes may be added in order to access it.
  virtual cc::LayerTreeHost* LayerTreeHost() = 0;

  // GPU benchmarking extension needs access to the LayerTreeHost
  friend class GpuBenchmarkingContext;

  // This private constructor and the class/friend declaration ensures that
  // WebFrameWidgetImpl is the only concrete subclass that implements
  // WebFrameWidget, so that it is safe to downcast to WebFrameWidgetImpl.
  friend class WebFrameWidgetImpl;
  WebFrameWidget() = default;
};

// Convenience type for creation method taken by
// InstallCreateWebFrameWidgetHook(). The method signature matches the
// WebFrameWidgetImpl constructor and must return a subclass of
// WebFrameWidgetImpl, though we do not expose that type here.
using CreateWebFrameWidgetCallback = base::RepeatingCallback<WebFrameWidget*(
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
// Allows tests to inject their own type of WebFrameWidget in order to
// override methods of the WebFrameWidgetImpl.
void BLINK_EXPORT
InstallCreateWebFrameWidgetHook(CreateWebFrameWidgetCallback* create_widget);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_FRAME_WIDGET_H_
