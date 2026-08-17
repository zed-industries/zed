// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_LAYER_TREE_VIEW_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_LAYER_TREE_VIEW_DELEGATE_H_

#include <memory>

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "cc/trees/layer_tree_host_client.h"
#include "cc/trees/paint_holding_reason.h"

namespace cc {
class LayerTreeFrameSink;
struct BeginMainFrameMetrics;
class RenderFrameMetadataObserver;
}  // namespace cc

namespace blink {

// Consumers of LayerTreeView implement this delegate in order to
// transport compositing information across processes.
class LayerTreeViewDelegate {
 public:
  using LayerTreeFrameSinkCallback = base::OnceCallback<void(
      std::unique_ptr<cc::LayerTreeFrameSink>,
      std::unique_ptr<cc::RenderFrameMetadataObserver>)>;

  // Report viewport related properties during a commit from the compositor
  // thread.
  virtual void ApplyViewportChanges(
      const cc::ApplyViewportChangesArgs& args) = 0;

  virtual void UpdateCompositorScrollState(
      const cc::CompositorCommitData& commit_data) = 0;

  // Notifies that the compositor has issued a BeginMainFrame.
  virtual void BeginMainFrame(const viz::BeginFrameArgs& args) = 0;

  virtual void OnDeferMainFrameUpdatesChanged(bool) = 0;
  virtual void OnDeferCommitsChanged(
      bool defer_status,
      cc::PaintHoldingReason reason,
      std::optional<cc::PaintHoldingCommitTrigger> trigger) = 0;
  virtual void OnCommitRequested() = 0;

  // Notifies that the layer tree host has completed a call to
  // RequestMainFrameUpdate in response to a BeginMainFrame.
  virtual void DidBeginMainFrame() = 0;

  // Requests a LayerTreeFrameSink to submit CompositorFrames to.
  virtual void RequestNewLayerTreeFrameSink(
      LayerTreeFrameSinkCallback callback) = 0;

  // Notifies that the draw commands for a committed frame have been issued.
  virtual void DidCommitAndDrawCompositorFrame() = 0;

  virtual void DidObserveFirstScrollDelay(
      base::TimeDelta first_scroll_delay,
      base::TimeTicks first_scroll_timestamp) = 0;

  // Notifies that a compositor frame commit operation is about to start.
  virtual void WillCommitCompositorFrame() = 0;

  // Notifies about a compositor frame commit operation having finished.
  // The commit_start_time is the time that the impl thread started processing
  // the commit.
  virtual void DidCommitCompositorFrame(base::TimeTicks commit_start_time,
                                        base::TimeTicks commit_finish_time) = 0;

  // Called by the compositor when page scale animation completed.
  virtual void DidCompletePageScaleAnimation() = 0;

  // Requests that a UMA and UKM metrics be recorded for the total frame time
  // and the portion of frame time spent in various sub-systems.
  // Call RecordStartOfFrameMetrics when a main frame is starting, and call
  // RecordEndOfFrameMetrics as soon as the total frame time becomes known for
  // a given frame. For example, ProxyMain::BeginMainFrame calls
  // RecordStartOfFrameMetrics just be WillBeginMainFrame() and
  // RecordEndOfFrameMetrics immediately before aborting or completing the
  // BeginMainFrame method.
  virtual void RecordStartOfFrameMetrics() = 0;
  virtual void RecordEndOfFrameMetrics(
      base::TimeTicks frame_begin_time,
      cc::ActiveFrameSequenceTrackers trackers) = 0;
  // Return metrics information for the stages of BeginMainFrame. This is
  // ultimately implemented by Blink's LocalFrameUKMAggregator. It must be a
  // distinct call from the FrameMetrics above because the BeginMainFrameMetrics
  // for compositor latency must be gathered before the layer tree is
  // committed to the compositor, which is before the call to
  // RecordEndOfFrameMetrics.
  virtual std::unique_ptr<cc::BeginMainFrameMetrics>
  GetBeginMainFrameMetrics() = 0;

  // Notification of the beginning and end of LayerTreeHost::UpdateLayers, for
  // metrics collection.
  virtual void BeginUpdateLayers() = 0;
  virtual void EndUpdateLayers() = 0;

  // Requests a visual frame-based update to the state of the delegate if there
  // is an update available.
  virtual void UpdateVisualState() = 0;

  // Indicates that the compositor is about to begin a frame. This is primarily
  // to signal to flow control mechanisms that a frame is beginning, not to
  // perform actual painting work.
  virtual void WillBeginMainFrame() = 0;

  virtual void RunPaintBenchmark(int repeat_count,
                                 cc::PaintBenchmarkResult& result) = 0;

  // Used in web tests without threaded compositing, to indicate that a new
  // commit needs to be scheduled. Has no effect in any other mode.
  virtual void ScheduleAnimationForWebTests() = 0;

  // Creates a RenderFrameMetadataObserver to track frame production in the
  // compositor. Generally this is supplied with the LayerTreeFrameSink. This
  // API is used if the compositor attaches to a new delegate, which requires a
  // new observer bound to the new delegate.
  virtual std::unique_ptr<cc::RenderFrameMetadataObserver>
  CreateRenderFrameObserver() = 0;

 protected:
  virtual ~LayerTreeViewDelegate() {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_LAYER_TREE_VIEW_DELEGATE_H_
