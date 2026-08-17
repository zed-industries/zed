// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BEGIN_FRAME_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BEGIN_FRAME_PROVIDER_H_

#include <string>

#include "base/notreached.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/viz/public/mojom/compositing/compositor_frame_sink.mojom-blink.h"
#include "third_party/blink/public/mojom/frame_sinks/embedded_frame_sink.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

struct BeginFrameProviderParams;

class PLATFORM_EXPORT BeginFrameProviderClient : public GarbageCollectedMixin {
 public:
  virtual void BeginFrame(const viz::BeginFrameArgs&) = 0;
  virtual scoped_refptr<base::SingleThreadTaskRunner>
  GetCompositorTaskRunner() = 0;
  virtual ~BeginFrameProviderClient() = default;
};

class PLATFORM_EXPORT BeginFrameProvider
    : public GarbageCollected<BeginFrameProvider>,
      public viz::mojom::blink::CompositorFrameSinkClient,
      public mojom::blink::EmbeddedFrameSinkClient {
 public:
  BeginFrameProvider(
      const BeginFrameProviderParams& begin_frame_provider_params,
      BeginFrameProviderClient* client,
      ContextLifecycleNotifier* context);

  void CreateCompositorFrameSinkIfNeeded();

  void RequestBeginFrame();
  void FinishBeginFrame(const viz::BeginFrameArgs&);

  // viz::mojom::blink::CompositorFrameSinkClient implementation.
  void DidReceiveCompositorFrameAck(
      WTF::Vector<viz::ReturnedResource> resources) final {
    NOTIMPLEMENTED();
  }
  void OnBeginFrame(const viz::BeginFrameArgs&,
                    const WTF::HashMap<uint32_t, viz::FrameTimingDetails>&,
                    WTF::Vector<viz::ReturnedResource> resources) final;
  void OnBeginFramePausedChanged(bool paused) final {}
  void ReclaimResources(WTF::Vector<viz::ReturnedResource> resources) final {
    NOTIMPLEMENTED();
  }
  void OnCompositorFrameTransitionDirectiveProcessed(
      uint32_t sequence_id) final {
    NOTIMPLEMENTED();
  }
  void OnSurfaceEvicted(const viz::LocalSurfaceId& local_surface_id) final {
    NOTIMPLEMENTED();
  }

  // viz::mojom::blink::EmbeddedFrameSinkClient implementation.
  void BindSurfaceEmbedder(
      mojo::PendingReceiver<mojom::blink::SurfaceEmbedder> receiver) override {
    NOTIMPLEMENTED();
  }

  void ResetCompositorFrameSink();

  bool IsValidFrameProvider();

  void Trace(Visitor*) const;

  ~BeginFrameProvider() override = default;

 private:
  void OnMojoConnectionError(uint32_t custom_reason,
                             const std::string& description);

  bool needs_begin_frame_;
  bool requested_needs_begin_frame_;

  HeapMojoReceiver<viz::mojom::blink::CompositorFrameSinkClient,
                   BeginFrameProvider>
      cfs_receiver_;

  HeapMojoReceiver<mojom::blink::EmbeddedFrameSinkClient, BeginFrameProvider>
      efs_receiver_;
  viz::FrameSinkId frame_sink_id_;
  viz::FrameSinkId parent_frame_sink_id_;
  HeapMojoRemote<viz::mojom::blink::CompositorFrameSink> compositor_frame_sink_;
  Member<BeginFrameProviderClient> begin_frame_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BEGIN_FRAME_PROVIDER_H_
