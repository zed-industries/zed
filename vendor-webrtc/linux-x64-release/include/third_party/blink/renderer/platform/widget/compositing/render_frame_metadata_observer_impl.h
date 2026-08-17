// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_RENDER_FRAME_METADATA_OBSERVER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_RENDER_FRAME_METADATA_OBSERVER_IMPL_H_

#include "build/build_config.h"
#include "cc/mojom/render_frame_metadata.mojom-blink.h"
#include "cc/trees/render_frame_metadata.h"
#include "cc/trees/render_frame_metadata_observer.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Implementation of cc::RenderFrameMetadataObserver which exists in the
// renderer and observers frame submission. It then notifies the
// cc::mojom::RenderFrameMetadataObserverClient, which is expected to be in the
// browser process, of the metadata associated with the frame.
//
// BindToCurrentSequence should be called from the Compositor thread so that the
// Mojo pipe is properly bound.
//
// Subsequent usage should only be from the Compositor thread.
class PLATFORM_EXPORT RenderFrameMetadataObserverImpl
    : public cc::RenderFrameMetadataObserver,
      public cc::mojom::blink::RenderFrameMetadataObserver {
 public:
  RenderFrameMetadataObserverImpl(
      mojo::PendingReceiver<cc::mojom::blink::RenderFrameMetadataObserver>
          receiver,
      mojo::PendingRemote<cc::mojom::blink::RenderFrameMetadataObserverClient>
          client_remote);
  ~RenderFrameMetadataObserverImpl() override;

  // cc::RenderFrameMetadataObserver:
  void BindToCurrentSequence() override;
  void OnRenderFrameSubmission(
      const cc::RenderFrameMetadata& render_frame_metadata,
      viz::CompositorFrameMetadata* compositor_frame_metadata,
      bool force_send) override;
#if BUILDFLAG(IS_ANDROID)
  void DidEndScroll() override;
#endif

  // mojom::RenderFrameMetadataObserver:
#if BUILDFLAG(IS_ANDROID)
  void UpdateRootScrollOffsetUpdateFrequency(
      cc::mojom::blink::RootScrollOffsetUpdateFrequency frequency) override;
#endif
  void ReportAllFrameSubmissionsForTesting(bool enabled) override;

 private:
  friend class RenderFrameMetadataObserverImplTest;

  // Certain fields should always have their changes reported. This will return
  // true when there is a difference between |rfm1| and |rfm2| for those fields.
  // These fields have a low frequency rate of change.
  // |needs_activation_notification| indicates whether the browser process
  // expects notification of activation of the assoicated CompositorFrame from
  // Viz.
  bool ShouldSendRenderFrameMetadata(const cc::RenderFrameMetadata& rfm1,
                                     const cc::RenderFrameMetadata& rfm2,
                                     bool* needs_activation_notification) const;

  void SendLastRenderFrameMetadata();

#if BUILDFLAG(IS_ANDROID)
  // This will determine the frequency to notify
  // |render_frame_metadata_observer_client_| of the frame submissions that
  // involve a root scroll offset change. See |RootScrollOffsetUpdateFrequency|
  // for details.
  std::optional<cc::mojom::blink::RootScrollOffsetUpdateFrequency>
      root_scroll_offset_update_frequency_;
  std::optional<gfx::PointF> last_root_scroll_offset_android_;
#endif

  // When true this will notify |render_frame_metadata_observer_client_| of all
  // frame submissions.
  bool report_all_frame_submissions_for_testing_enabled_ = false;

  uint32_t last_frame_token_ = 0;
  std::optional<cc::RenderFrameMetadata> last_render_frame_metadata_;

  // These are destroyed when BindToCurrentSequence() is called.
  mojo::PendingReceiver<cc::mojom::blink::RenderFrameMetadataObserver>
      receiver_;
  mojo::PendingRemote<cc::mojom::blink::RenderFrameMetadataObserverClient>
      client_remote_;

  mojo::Receiver<cc::mojom::blink::RenderFrameMetadataObserver>
      render_frame_metadata_observer_receiver_{this};
  mojo::Remote<cc::mojom::blink::RenderFrameMetadataObserverClient>
      render_frame_metadata_observer_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_RENDER_FRAME_METADATA_OBSERVER_IMPL_H_
