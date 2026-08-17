// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SURFACE_LAYER_BRIDGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SURFACE_LAYER_BRIDGE_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "components/viz/common/surfaces/parent_local_surface_id_allocator.h"
#include "components/viz/common/surfaces/surface_id.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/frame_sinks/embedded_frame_sink.mojom-blink.h"
#include "third_party/blink/public/platform/web_surface_layer_bridge.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace cc {
class SolidColorLayer;
class SurfaceLayer;
}  // namespace cc

namespace blink {

// The SurfaceLayerBridge facilitates communication about changes to a Surface
// between the Render and Browser processes.
class PLATFORM_EXPORT SurfaceLayerBridge
    : public blink::mojom::blink::EmbeddedFrameSinkClient,
      public blink::mojom::blink::SurfaceEmbedder,
      public WebSurfaceLayerBridge {
 public:
  SurfaceLayerBridge(
      viz::FrameSinkId parent_frame_sink_id,
      ContainsVideo contains_video,
      WebSurfaceLayerBridgeObserver*,
      cc::UpdateSubmissionStateCB update_submission_state_callback);
  ~SurfaceLayerBridge() override;

  void CreateSolidColorLayer();

  // Implementation of blink::mojom::blink::EmbeddedFrameSinkClient
  void BindSurfaceEmbedder(
      mojo::PendingReceiver<mojom::blink::SurfaceEmbedder> receiver) override;

  void EmbedSurface(const viz::SurfaceId& surface_id);

  // Implementation of blink::mojom::blink::SurfaceEmbedder
  void SetLocalSurfaceId(const viz::LocalSurfaceId& local_surface_id) override;
  void OnOpacityChanged(bool is_opaque) override;

  // Implementation of WebSurfaceLayerBridge.
  cc::Layer* GetCcLayer() const override;
  const viz::FrameSinkId& GetFrameSinkId() const override;
  void SetContentsOpaque(bool) override;
  void CreateSurfaceLayer() override;
  void ClearObserver() override;

  const viz::SurfaceId& GetSurfaceId() const override {
    return current_surface_id_;
  }

  void RegisterFrameSinkHierarchy() override;
  void UnregisterFrameSinkHierarchy() override;

  // Update the opacity of `surface_layer_` based on what the embedder expects
  // and what the embeddee has actually sent to the frame sink.  The idea is
  // that it's always safe to say "not opaque", since cc will emit quads under
  // the surface layer which will be overdrawn by the surface layer as needed.
  // The bad case is if we mark the surface layer as not opaque, when it is not
  // really opaque; this can cause all sorts of drawing badness since there
  // might not be anything drawn under it.  Bad cases include:
  //
  //  - No frames have been submitted to the viz.  In this case, the surface
  //    quad will fail during aggregation, causing viz to use the background
  //    color of the surface quad.  The default is transparent.  While we could
  //    change this to something opaque, we'd still have to handle the other
  //    cases so it doesn't buy us much.
  //  - Frames have been submitted to viz, but they are not opaque.  In this
  //    case, whichever pixels are not fully opaque may result in undefined
  //    values in the frame buffer.
  //
  // We try to avoid these by letting the embedder tell us when it expects
  // frames to be opaque via `SetContentsOpaque()`, and also let the embeddee
  // tell us what it has submitted via `OnOpacityChange()`.  Assuming that the
  // embedder tells us soon enough that it expects non-opaque content before the
  // embeddee actually submits a frame that is not opaque, we will have time to
  // tell the surface layer that it is not opaque and have it emit a new quad.
  //
  // This is all heuristic, and has several potential failures.  However, these
  // failures all require changes from the (default) non-opaque => opaque =>
  // non-opaque, which should be very rare.
  //
  // It is okay to call this without a surface layer.
  void UpdateSurfaceLayerOpacity();

 private:
  scoped_refptr<cc::SurfaceLayer> surface_layer_;
  scoped_refptr<cc::SolidColorLayer> solid_color_layer_;

  // The |observer_| handles unregistering the contents layer on its own.
  raw_ptr<WebSurfaceLayerBridgeObserver> observer_;
  cc::UpdateSubmissionStateCB update_submission_state_callback_;
  viz::ParentLocalSurfaceIdAllocator parent_local_surface_id_allocator_;
  mojo::Receiver<blink::mojom::blink::EmbeddedFrameSinkClient> receiver_{this};
  mojo::Receiver<blink::mojom::blink::SurfaceEmbedder>
      surface_embedder_receiver_{this};
  mojo::Remote<mojom::blink::EmbeddedFrameSinkProvider>
      embedded_frame_sink_provider_;

  const viz::FrameSinkId frame_sink_id_;
  const ContainsVideo contains_video_;
  viz::SurfaceId current_surface_id_;
  const viz::FrameSinkId parent_frame_sink_id_;
  // Does the embedder expect our content to be fully opaque?  This is presumed
  // to lead the frames that are sent by the embedee.
  bool embedder_expects_opaque_ = false;
  // Has the embedee submitted opaque frames without later non-opaque ones?
  bool frames_are_opaque_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SURFACE_LAYER_BRIDGE_H_
