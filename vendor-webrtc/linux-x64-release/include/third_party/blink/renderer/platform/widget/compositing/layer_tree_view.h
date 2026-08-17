// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_LAYER_TREE_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_LAYER_TREE_VIEW_H_

#include <stdint.h>

#include "base/containers/circular_deque.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "cc/input/browser_controls_state.h"
#include "cc/trees/layer_tree_host_client.h"
#include "cc/trees/layer_tree_host_single_thread_client.h"
#include "cc/trees/paint_holding_reason.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/widget/compositing/layer_tree_view_delegate.h"
#include "ui/gfx/ca_layer_result.h"
#include "ui/gfx/geometry/rect.h"

namespace cc {
class AnimationHost;
class LayerTreeFrameSink;
class LayerTreeHost;
class LayerTreeSettings;
class RenderFrameMetadataObserver;
class TaskGraphRunner;
}  // namespace cc

namespace blink {

namespace scheduler {
class WidgetScheduler;
}  // namespace scheduler

class PLATFORM_EXPORT LayerTreeView
    : public cc::LayerTreeHostClient,
      public cc::LayerTreeHostSingleThreadClient,
      public cc::LayerTreeHostSchedulingClient {
 public:
  LayerTreeView(LayerTreeViewDelegate* delegate,
                scoped_refptr<scheduler::WidgetScheduler> scheduler);
  LayerTreeView(const LayerTreeView&) = delete;
  LayerTreeView& operator=(const LayerTreeView&) = delete;
  ~LayerTreeView() override;

  // The |main_thread| is the task runner that the compositor will use for the
  // main thread (where it is constructed). The |compositor_thread| is the task
  // runner for the compositor thread, but is null if the compositor will run in
  // single-threaded mode (in tests only).
  // The |ukm_recorder_factory| may be null to disable recording (in tests
  // only).
  void Initialize(const cc::LayerTreeSettings& settings,
                  scoped_refptr<base::SingleThreadTaskRunner> main_thread,
                  scoped_refptr<base::SingleThreadTaskRunner> compositor_thread,
                  cc::TaskGraphRunner* task_graph_runner);

  // Drops any references back to the delegate in preparation for being
  // destroyed.
  void Disconnect();

  // Drops any references back to the current delegate and attaches to
  // `delegate` if non-null.
  void ClearPreviousDelegateAndReattachIfNeeded(
      LayerTreeViewDelegate* delegate,
      scoped_refptr<scheduler::WidgetScheduler> scheduler);

  cc::AnimationHost* animation_host() { return animation_host_.get(); }

  void SetVisible(bool visible);
  void SetShouldWarmUp();

  // cc::LayerTreeHostClient implementation.
  // NOTE: LayerTreeView allows re-attaching itself to a different delegate.
  // Since the compositor is threaded, we could receive callbacks from the host
  // which are tied to content committed by the previous delegate.
  //
  // Ensure such callbacks have a `source_frame_number` to ensure callbacks
  // associated with the previous delegate are safely discarded.
  void WillBeginMainFrame() override;
  void DidBeginMainFrame() override;
  void WillUpdateLayers() override;
  void DidUpdateLayers() override;
  void BeginMainFrame(const viz::BeginFrameArgs& args) override;
  void OnDeferMainFrameUpdatesChanged(bool) override;
  void OnDeferCommitsChanged(
      bool defer_status,
      cc::PaintHoldingReason reason,
      std::optional<cc::PaintHoldingCommitTrigger> trigger) override;
  void OnCommitRequested() override;
  void BeginMainFrameNotExpectedSoon() override;
  void BeginMainFrameNotExpectedUntil(base::TimeTicks time) override;
  void UpdateLayerTreeHost() override;
  void ApplyViewportChanges(const cc::ApplyViewportChangesArgs& args) override;
  void UpdateCompositorScrollState(
      const cc::CompositorCommitData& commit_data) override;
  void RequestNewLayerTreeFrameSink() override;
  void DidInitializeLayerTreeFrameSink() override;
  void DidFailToInitializeLayerTreeFrameSink() override;
  void WillCommit(const cc::CommitState&) override;
  void DidCommit(int source_frame_number,
                 base::TimeTicks commit_start_time,
                 base::TimeTicks commit_finish_time) override;
  void DidCommitAndDrawFrame(int source_frame_number) override;
  void DidCompletePageScaleAnimation(int source_frame_number) override;
  void DidPresentCompositorFrame(
      uint32_t frame_token,
      const viz::FrameTimingDetails& frame_timing_details) override;
  void RecordStartOfFrameMetrics() override;
  void RecordEndOfFrameMetrics(
      base::TimeTicks frame_begin_time,
      cc::ActiveFrameSequenceTrackers trackers) override;
  std::unique_ptr<cc::BeginMainFrameMetrics> GetBeginMainFrameMetrics()
      override;
  void NotifyCompositorMetricsTrackerResults(
      cc::CustomTrackerResults results) override;
  void DidObserveFirstScrollDelay(
      int source_frame_number,
      base::TimeDelta first_scroll_delay,
      base::TimeTicks first_scroll_timestamp) override;
  void RunPaintBenchmark(int repeat_count,
                         cc::PaintBenchmarkResult& result) override;
  std::string GetPausedDebuggerLocalizedMessage() override;

  // cc::LayerTreeHostSingleThreadClient implementation.
  void DidSubmitCompositorFrame() override;
  void DidLoseLayerTreeFrameSink() override;
  void ScheduleAnimationForWebTests() override;

  // cc::LayerTreeHostSchedulingClient implementation.
  void DidRunBeginMainFrame() override;

  // Registers a callback that will be run on the first successful presentation
  // for `frame_token` or a following frame.
  void AddPresentationCallback(
      uint32_t frame_token,
      base::OnceCallback<void(const viz::FrameTimingDetails&)> callback);

#if BUILDFLAG(IS_APPLE)
  void AddCoreAnimationErrorCodeCallback(
      uint32_t frame_token,
      base::OnceCallback<void(gfx::CALayerResult)> callback);
#endif

  cc::LayerTreeHost* layer_tree_host() { return layer_tree_host_.get(); }
  const cc::LayerTreeHost* layer_tree_host() const {
    return layer_tree_host_.get();
  }

 protected:
  friend class RenderViewImplScaleFactorTest;

 private:
  void SetLayerTreeFrameSink(
      std::unique_ptr<cc::LayerTreeFrameSink> layer_tree_frame_sink,
      std::unique_ptr<cc::RenderFrameMetadataObserver>
          render_frame_metadata_observer);

  template <typename Callback>
  void AddCallback(
      uint32_t frame_token,
      Callback callback,
      base::circular_deque<std::pair<uint32_t, std::vector<Callback>>>&
          callbacks);

  scoped_refptr<scheduler::WidgetScheduler> widget_scheduler_;
  const std::unique_ptr<cc::AnimationHost> animation_host_;

  // The delegate_ becomes null when Disconnect() is called. After that, the
  // class should do nothing in calls from the LayerTreeHost, and just wait to
  // be destroyed. It is not expected to be used at all after Disconnect()
  // outside of handling/dropping LayerTreeHost client calls.
  raw_ptr<LayerTreeViewDelegate> delegate_;
  std::unique_ptr<cc::LayerTreeHost> layer_tree_host_;

  enum class FrameSinkState {
    kNoFrameSink,
    kRequestBufferedInvisible,
    kRequestPending,
    kInitializing,
    kInitialized
  };
  FrameSinkState frame_sink_state_ = FrameSinkState::kNoFrameSink;

  base::circular_deque<std::pair<
      uint32_t,
      std::vector<base::OnceCallback<void(const viz::FrameTimingDetails&)>>>>
      presentation_callbacks_;

#if BUILDFLAG(IS_APPLE)
  base::circular_deque<std::pair<
      uint32_t,
      std::vector<base::OnceCallback<void(gfx::CALayerResult error_code)>>>>
      core_animation_error_code_callbacks_;
#endif

  // Tracks the source frame number for the first main frame when a new
  // delegate is bound to this view. This is used to safely ignore redundant
  // callbacks which are tied to content produced by the previous delegate.
  int first_source_frame_for_current_delegate_ = 0;

  base::WeakPtrFactory<LayerTreeView> weak_factory_{this};
  base::WeakPtrFactory<LayerTreeView> weak_factory_for_delegate_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_LAYER_TREE_VIEW_H_
