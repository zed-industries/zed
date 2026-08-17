// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_ANDROID_WEBVIEW_SYNCHRONOUS_LAYER_TREE_FRAME_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_ANDROID_WEBVIEW_SYNCHRONOUS_LAYER_TREE_FRAME_SINK_H_

#include <stddef.h>

#include <memory>

#include "base/cancelable_callback.h"
#include "base/compiler_specific.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/read_only_shared_memory_region.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "cc/trees/layer_tree_frame_sink.h"
#include "cc/trees/managed_memory_policy.h"
#include "components/viz/common/display/renderer_settings.h"
#include "components/viz/common/frame_sinks/begin_frame_source.h"
#include "components/viz/common/frame_timing_details_map.h"
#include "components/viz/common/quads/compositor_frame.h"
#include "components/viz/common/quads/compositor_frame_metadata.h"
#include "components/viz/common/surfaces/local_surface_id.h"
#include "components/viz/common/surfaces/parent_local_surface_id_allocator.h"
#include "components/viz/service/display/display_client.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/viz/public/mojom/compositing/compositor_frame_sink.mojom-blink.h"
#include "services/viz/public/mojom/compositing/compositor_frame_sink.mojom-forward.h"
#include "third_party/blink/renderer/platform/widget/compositing/android_webview/synchronous_compositor_registry.h"
#include "ui/gfx/geometry/transform.h"

class SkCanvas;

namespace viz {
class BeginFrameSource;
class CompositorFrameSinkSupport;
class Display;
class FrameSinkManagerImpl;
class ParentLocalSurfaceIdAllocator;
class RasterContextProvider;
}  // namespace viz

namespace blink {

// This class represents the client interface for the frame sink
// created for the synchronous compositor.
class SynchronousLayerTreeFrameSinkClient {
 public:
  virtual void DidActivatePendingTree() = 0;
  virtual void Invalidate(bool needs_draw) = 0;
  virtual void SubmitCompositorFrame(
      uint32_t layer_tree_frame_sink_id,
      const viz::LocalSurfaceId& local_surface_id,
      std::optional<viz::CompositorFrame> frame,
      std::optional<viz::HitTestRegionList> hit_test_region_list) = 0;
  virtual void SetNeedsBeginFrames(bool needs_begin_frames) = 0;
  virtual void SinkDestroyed() = 0;

 protected:
  virtual ~SynchronousLayerTreeFrameSinkClient() {}
};

// Specialization of the output surface that adapts it to implement the
// blink::mojom::SynchronousCompositor public API. This class effects an
// "inversion of control" - enabling drawing to be  orchestrated by the
// embedding layer, instead of driven by the compositor internals - hence it
// holds two 'client' pointers (|client_| in the LayerTreeFrameSink baseclass
// and |delegate_|) which represent the consumers of the two roles in plays.
// This class can be created only on the main thread, but then becomes pinned
// to a fixed thread when BindToClient is called.
class SynchronousLayerTreeFrameSink
    : public cc::LayerTreeFrameSink,
      public viz::mojom::blink::CompositorFrameSinkClient,
      public viz::ExternalBeginFrameSourceClient {
 public:
  SynchronousLayerTreeFrameSink(
      scoped_refptr<viz::RasterContextProvider> context_provider,
      scoped_refptr<cc::RasterContextProviderWrapper>
          worker_context_provider_wrapper,
      scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner,
      uint32_t layer_tree_frame_sink_id,
      std::unique_ptr<viz::BeginFrameSource> begin_frame_source,
      SynchronousCompositorRegistry* registry,
      mojo::PendingRemote<viz::mojom::blink::CompositorFrameSink>
          compositor_frame_sink_remote,
      mojo::PendingReceiver<viz::mojom::blink::CompositorFrameSinkClient>
          client_receiver);
  SynchronousLayerTreeFrameSink(const SynchronousLayerTreeFrameSink&) = delete;
  SynchronousLayerTreeFrameSink& operator=(
      const SynchronousLayerTreeFrameSink&) = delete;
  ~SynchronousLayerTreeFrameSink() override;

  // cc::LayerTreeFrameSink implementation.
  bool BindToClient(cc::LayerTreeFrameSinkClient* sink_client) override;
  void DetachFromClient() override;
  void SetLocalSurfaceId(const viz::LocalSurfaceId& local_surface_id) override;
  void SubmitCompositorFrame(viz::CompositorFrame frame,
                             bool hit_test_data_changed) override;
  void DidNotProduceFrame(const viz::BeginFrameAck& ack,
                          cc::FrameSkippedReason reason) override;
  void Invalidate(bool needs_draw) override;

  // viz::mojom::CompositorFrameSinkClient implementation.
  void DidReceiveCompositorFrameAck(
      Vector<viz::ReturnedResource> resources) override;
  void OnBeginFrame(
      const viz::BeginFrameArgs& args,
      const HashMap<uint32_t, viz::FrameTimingDetails>& timing_details,
      Vector<viz::ReturnedResource> resources) override;
  void ReclaimResources(Vector<viz::ReturnedResource> resources) override;
  void OnBeginFramePausedChanged(bool paused) override;
  void OnCompositorFrameTransitionDirectiveProcessed(
      uint32_t sequence_id) override {}
  void OnSurfaceEvicted(const viz::LocalSurfaceId& local_surface_id) override {}

  // viz::ExternalBeginFrameSourceClient overrides.
  void OnNeedsBeginFrames(bool needs_begin_frames) override;

  void SetSyncClient(SynchronousLayerTreeFrameSinkClient* compositor);
  void DidPresentCompositorFrame(
      const viz::FrameTimingDetailsMap& timing_details);
  void BeginFrame(const viz::BeginFrameArgs& args);
  void SetBeginFrameSourcePaused(bool paused);
  void SetMemoryPolicy(size_t bytes_limit);
  void ReclaimResources(uint32_t layer_tree_frame_sink_id,
                        Vector<viz::ReturnedResource> resources);
  void OnCompositorFrameTransitionDirectiveProcessed(
      uint32_t layer_tree_frame_sink_id,
      uint32_t sequence_id);
  void DemandDrawHw(const gfx::Size& viewport_size,
                    const gfx::Rect& viewport_rect_for_tile_priority,
                    const gfx::Transform& transform_for_tile_priority,
                    bool need_new_local_surface_id);
  void DemandDrawSw(SkCanvas* canvas);
  void DemandDrawSwZeroCopy();
  void WillSkipDraw();
  bool UseZeroCopySoftwareDraw();

 private:
  class SoftwareOutputSurface;

  void InvokeComposite(const gfx::Transform& transform,
                       const gfx::Rect& viewport);
  void DidActivatePendingTree();
  void DeliverMessages();

  const uint32_t layer_tree_frame_sink_id_;
  const raw_ptr<SynchronousCompositorRegistry> registry_;  // Not owned.

  // Not owned.
  raw_ptr<SynchronousLayerTreeFrameSinkClient> sync_client_ = nullptr;

  // Only valid (non-null) during a DemandDrawSw() call.
  raw_ptr<SkCanvas> current_sw_canvas_ = nullptr;

  cc::ManagedMemoryPolicy memory_policy_;
  bool in_software_draw_ = false;
  bool did_submit_frame_ = false;

  mojo::PendingRemote<viz::mojom::blink::CompositorFrameSink>
      unbound_compositor_frame_sink_;
  mojo::PendingReceiver<viz::mojom::blink::CompositorFrameSinkClient>
      unbound_client_;
  mojo::Remote<viz::mojom::blink::CompositorFrameSink> compositor_frame_sink_;
  mojo::Receiver<viz::mojom::blink::CompositorFrameSinkClient> client_receiver_{
      this};
  viz::LocalSurfaceId local_surface_id_;

  class StubDisplayClient : public viz::DisplayClient {
    void DisplayOutputSurfaceLost() override {}
    void DisplayWillDrawAndSwap(
        bool will_draw_and_swap,
        viz::AggregatedRenderPassList* render_passes) override {}
    void DisplayDidDrawAndSwap() override {}
    void DisplayDidReceiveCALayerParams(
        const gfx::CALayerParams& ca_layer_params) override {}
    void DisplayDidCompleteSwapWithSize(const gfx::Size& pixel_size) override {}
    void DisplayAddChildWindowToBrowser(
        gpu::SurfaceHandle child_window) override {}
    void SetWideColorEnabled(bool enabled) override {}
    void SetPreferredFrameInterval(base::TimeDelta interval) override {}
    base::TimeDelta GetPreferredFrameIntervalForFrameSinkId(
        const viz::FrameSinkId& id,
        viz::mojom::CompositorFrameSinkType* type) override;
  };

  viz::DebugRendererSettings debug_settings_;

  // TODO(danakj): These don't to be stored in unique_ptrs when OutputSurface
  // is owned/destroyed on the compositor thread.
  std::unique_ptr<viz::FrameSinkManagerImpl> frame_sink_manager_;
  viz::ParentLocalSurfaceIdAllocator root_local_surface_id_allocator_;
  viz::ParentLocalSurfaceIdAllocator child_local_surface_id_allocator_;
  viz::LocalSurfaceId child_local_surface_id_;
  viz::LocalSurfaceId root_local_surface_id_;
  gfx::Size child_size_;
  gfx::Size display_size_;
  float device_scale_factor_ = 0;
  float root_device_scale_factor_ = 0;
  viz::FrameTokenGenerator root_next_frame_token_;
  std::unique_ptr<viz::mojom::CompositorFrameSinkClient>
      software_frame_sink_client_;
  // Uses frame_sink_manager_.
  std::unique_ptr<viz::CompositorFrameSinkSupport> root_support_;
  // Uses frame_sink_manager_.
  std::unique_ptr<viz::CompositorFrameSinkSupport> child_support_;
  StubDisplayClient display_client_;
  // Uses frame_sink_manager_.
  std::unique_ptr<viz::Display> display_;
  // Owned by |display_|.
  raw_ptr<SoftwareOutputSurface> software_output_surface_ = nullptr;
  std::unique_ptr<viz::BeginFrameSource> synthetic_begin_frame_source_;
  std::unique_ptr<viz::ExternalBeginFrameSource> external_begin_frame_source_;

  gfx::Rect sw_viewport_for_current_draw_;

  THREAD_CHECKER(thread_checker_);

  // Indicates that webview using viz
  const bool viz_frame_submission_enabled_;
  bool begin_frames_paused_ = false;
  bool needs_begin_frames_ = false;
  const bool use_zero_copy_sw_draw_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_ANDROID_WEBVIEW_SYNCHRONOUS_LAYER_TREE_FRAME_SINK_H_
