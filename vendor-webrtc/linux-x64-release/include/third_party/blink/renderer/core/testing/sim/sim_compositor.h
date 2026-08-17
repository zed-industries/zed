// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_COMPOSITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_COMPOSITOR_H_

#include "base/time/time.h"
#include "cc/trees/layer_tree_host.h"
#include "third_party/blink/renderer/core/frame/frame_test_helpers.h"
#include "third_party/blink/renderer/core/testing/sim/sim_canvas.h"
#include "third_party/blink/renderer/platform/graphics/apply_viewport_changes.h"

namespace blink {

class WebViewImpl;

// Simulated very basic compositor that's capable of running the BeginMainFrame
// processing steps on WebView: beginFrame, layout, paint.
//
// The painting capabilities are very limited in that only the main layer of
// every CompositedLayerMapping will be painted, squashed layers
// are not supported and the entirety of every layer is always repainted even if
// only part of the layer was invalid.
//
// Note: This also does not support compositor driven animations.
class SimCompositor final {
 public:
  SimCompositor();
  ~SimCompositor();

  // When the compositor asks for a main frame, this WebViewImpl will have its
  // lifecycle updated and be painted.
  void SetWebView(WebViewImpl&);

  // Set the LayerTreeHost that the compositor is associated with.
  void SetLayerTreeHost(cc::LayerTreeHost*);

  // Executes the BeginMainFrame processing steps, an approximation of what
  // cc::ThreadProxy::BeginMainFrame would do.
  // If time is not specified a 60Hz frame rate time progression is used.
  // Returns all drawing commands that were issued during painting the frame
  // (including cached ones).
  // TODO(dcheng): This should take a base::TimeDelta.
  // Rasterization of tiles is only performed when |raster| is true.
  SimCanvas::Commands BeginFrame(double time_delta_in_seconds = 0.016,
                                 bool raster = false);

  // Helpers to query the state of the compositor from tests.
  //
  // Returns true if a main frame has been requested from blink, until the
  // BeginFrame() step occurs.
  bool NeedsBeginFrame() const {
    return LayerTreeHost()->RequestedMainFramePending();
  }
  // Returns true if commits are deferred in the compositor. Since these tests
  // use synchronous compositing through BeginFrame(), the deferred state has no
  // real effect.
  bool DeferMainFrameUpdate() const {
    return LayerTreeHost()->defer_main_frame_update();
  }
  // Returns true if a selection is set on the compositor.
  bool HasSelection() const {
    return LayerTreeHost()->selection() != cc::LayerSelection();
  }
  // Returns the background color set on the compositor.
  SkColor background_color() const {
    // TODO(crbug/1308932): Remove toSkColor and make all SkColor4f.
    return LayerTreeHost()->background_color().toSkColor();
  }

  base::TimeTicks LastFrameTime() const { return last_frame_time_; }

  // Sets last_frame_time_ to now, to sync with external time.
  void ResetLastFrameTime() { last_frame_time_ = base::TimeTicks::Now(); }

  cc::LayerTreeHost* LayerTreeHost() const;

 private:
  WebViewImpl* web_view_ = nullptr;
  cc::LayerTreeHost* layer_tree_host_ = nullptr;

  base::TimeTicks last_frame_time_;

  std::unique_ptr<cc::ScopedDeferMainFrameUpdate>
      scoped_defer_main_frame_update_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_COMPOSITOR_H_
