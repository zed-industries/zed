// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_TEST_STUB_LAYER_TREE_VIEW_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_TEST_STUB_LAYER_TREE_VIEW_DELEGATE_H_

#include "cc/paint/element_id.h"
#include "cc/trees/paint_holding_reason.h"
#include "third_party/blink/renderer/platform/widget/compositing/layer_tree_view_delegate.h"

namespace cc {
struct ApplyViewportChangesArgs;
}

namespace blink {

class StubLayerTreeViewDelegate : public LayerTreeViewDelegate {
 public:
  StubLayerTreeViewDelegate() = default;

  // LayerTreeViewDelegate overrides:
  void RequestNewLayerTreeFrameSink(
      LayerTreeFrameSinkCallback callback) override {}
  void ApplyViewportChanges(const cc::ApplyViewportChangesArgs& args) override {
  }
  void UpdateCompositorScrollState(
      const cc::CompositorCommitData& commit_data) override {}
  void BeginMainFrame(const viz::BeginFrameArgs& args) override {}
  void OnDeferMainFrameUpdatesChanged(bool) override {}
  void OnDeferCommitsChanged(
      bool defer_status,
      cc::PaintHoldingReason reason,
      std::optional<cc::PaintHoldingCommitTrigger> trigger) override {}
  void OnCommitRequested() override {}
  void DidBeginMainFrame() override {}
  void DidCommitAndDrawCompositorFrame() override {}
  void WillCommitCompositorFrame() override {}
  void DidCommitCompositorFrame(base::TimeTicks commit_start_time,
                                base::TimeTicks commit_finish_time) override {}
  void DidCompletePageScaleAnimation() override {}
  void DidObserveFirstScrollDelay(
      base::TimeDelta first_scroll_delay,
      base::TimeTicks first_scroll_timestamp) override {}
  void RecordStartOfFrameMetrics() override {}
  void RecordEndOfFrameMetrics(
      base::TimeTicks frame_begin_time,
      cc::ActiveFrameSequenceTrackers trackers) override {}
  std::unique_ptr<cc::BeginMainFrameMetrics> GetBeginMainFrameMetrics()
      override {
    return nullptr;
  }
  void BeginUpdateLayers() override {}
  void EndUpdateLayers() override {}
  void UpdateVisualState() override {}
  void WillBeginMainFrame() override {}
  void RunPaintBenchmark(int repeat_count,
                         cc::PaintBenchmarkResult& result) override {}
  void ScheduleAnimationForWebTests() override {}
  std::unique_ptr<cc::RenderFrameMetadataObserver> CreateRenderFrameObserver()
      override {
    return nullptr;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_TEST_STUB_LAYER_TREE_VIEW_DELEGATE_H_
