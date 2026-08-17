// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIDEO_FRAME_SUBMITTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIDEO_FRAME_SUBMITTER_H_

#include <memory>
#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/memory/read_only_shared_memory_region.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "base/timer/timer.h"
#include "cc/metrics/frame_sequence_tracker_collection.h"
#include "cc/metrics/frame_sorter.h"
#include "cc/metrics/video_playback_roughness_reporter.h"
#include "components/viz/common/gpu/raster_context_provider.h"
#include "components/viz/common/surfaces/child_local_surface_id_allocator.h"
#include "gpu/ipc/client/gpu_channel_observer.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/system/buffer.h"
#include "services/viz/public/mojom/compositing/compositor_frame_sink.mojom-blink.h"
#include "services/viz/public/mojom/compositing/frame_timing_details.mojom-blink.h"
#include "third_party/blink/public/mojom/frame_sinks/embedded_frame_sink.mojom-blink.h"
#include "third_party/blink/public/platform/web_video_frame_submitter.h"
#include "third_party/blink/renderer/platform/graphics/video_frame_resource_provider.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// This single-threaded class facilitates the communication between the media
// stack and browser renderer, providing compositor frames containing video
// frames and corresponding resources to the |compositor_frame_sink_|.
//
// This class requires and uses a viz::ContextProvider, and thus, besides
// construction, must be consistently accessed from the same thread.
class PLATFORM_EXPORT VideoFrameSubmitter
    : public WebVideoFrameSubmitter,
      public viz::ContextLostObserver,
      public gpu::GpuChannelLostObserver,
      public viz::mojom::blink::CompositorFrameSinkClient {
 public:
  VideoFrameSubmitter(WebContextProviderCallback,
                      cc::VideoPlaybackRoughnessReporter::ReportingCallback,
                      std::unique_ptr<VideoFrameResourceProvider>);
  VideoFrameSubmitter(const VideoFrameSubmitter&) = delete;
  VideoFrameSubmitter& operator=(const VideoFrameSubmitter&) = delete;
  ~VideoFrameSubmitter() override;

  // cc::VideoFrameProvider::Client implementation.
  void StopUsingProvider() override;
  void StartRendering() override;
  void StopRendering() override;
  void DidReceiveFrame() override;
  bool IsDrivingFrameUpdates() const override;

  // WebVideoFrameSubmitter implementation.
  void Initialize(cc::VideoFrameProvider*, bool is_media_stream) override;
  void SetTransform(media::VideoTransformation) override;
  void EnableSubmission(viz::SurfaceId) override;
  void SetIsSurfaceVisible(bool is_visible) override;
  void SetIsPageVisible(bool is_visible) override;
  void SetForceBeginFrames(bool force_begin_frames) override;
  void SetForceSubmit(bool) override;

  // viz::ContextLostObserver implementation.
  void OnContextLost() override;

  // gpu::GpuChannelLostObserver implementation.
  void OnGpuChannelLost() override;

  // cc::mojom::CompositorFrameSinkClient implementation.
  void DidReceiveCompositorFrameAck(
      WTF::Vector<viz::ReturnedResource> resources) override;
  void OnBeginFrame(const viz::BeginFrameArgs&,
                    const WTF::HashMap<uint32_t, viz::FrameTimingDetails>&,
                    WTF::Vector<viz::ReturnedResource> resources) override;
  void OnBeginFramePausedChanged(bool paused) override {}
  void ReclaimResources(WTF::Vector<viz::ReturnedResource> resources) override;
  void OnCompositorFrameTransitionDirectiveProcessed(
      uint32_t sequence_id) override {}
  void OnSurfaceEvicted(const viz::LocalSurfaceId& local_surface_id) override {}

 private:
  friend class VideoFrameSubmitterTest;
  class FrameSinkBundleProxy;

  // Called during Initialize() and OnContextLost() after a new ContextGL is
  // requested.
  void OnReceivedContextProvider(
      bool use_gpu_compositing,
      scoped_refptr<viz::RasterContextProvider> context_provider,
      scoped_refptr<gpu::ClientSharedImageInterface> shared_image_interface);

  // Adopts `context_provider` if it's non-null and in a usable state. Returns
  // true on success and false on failure, implying that a new ContextProvider
  // should be requested.
  bool MaybeAcceptContextProvider(
      scoped_refptr<viz::RasterContextProvider> context_provider);

  // Starts submission and calls UpdateSubmissionState(); which may submit.
  void StartSubmitting();

  // Sets CompositorFrameSink::SetNeedsBeginFrame() state and submits a frame if
  // visible or an empty frame if not.
  void UpdateSubmissionState();

  // Will submit an empty frame to clear resource usage if it's safe.
  void SubmitEmptyFrameIfNeeded();

  // Returns whether a frame was submitted.
  bool SubmitFrame(const viz::BeginFrameAck&, scoped_refptr<media::VideoFrame>);

  // SubmitEmptyFrame() is used to force the remote CompositorFrameSink to
  // release resources for the last submission; saving a significant amount of
  // memory (~30%) when content goes off-screen. See https://crbug.com/829813.
  void SubmitEmptyFrame();

  // Pulls frame and submits it to compositor. Used in cases like
  // DidReceiveFrame(), which occurs before video rendering has started to post
  // the first frame or to submit a final frame before ending rendering.
  void SubmitSingleFrame();

  // Return whether the submitter should submit frames based on its current
  // state. It's important to only submit when this is true to save memory. See
  // comments above and in UpdateSubmissionState().
  bool ShouldSubmit() const;

  // Generates a new surface ID using using |child_local_surface_id_allocator_|.
  // Called during context loss or during a frame size change.
  void GenerateNewSurfaceId();

  // Helper method for creating viz::CompositorFrame. If |video_frame| is null
  // then the frame will be empty.
  viz::CompositorFrame CreateCompositorFrame(
      uint32_t frame_token,
      const viz::BeginFrameAck& begin_frame_ack,
      scoped_refptr<media::VideoFrame> video_frame,
      media::VideoTransformation transform);

  // Opacity state with respect to what we've told `surface_embedder_`.
  enum class Opacity {
    // We have not told the embedder anything yet.
    kNotReported,

    // We told the embedder that we have submitted an opaque frame.
    kIsOpaque,

    // We told the embedder that we have submitted a non-opaque frame.
    kIsNotOpaque
  };

  // Notify `surface_embedder_` if the opacity of the most recent video frame
  // has changed.
  void NotifyOpacityIfNeeded(Opacity new_opacity);

  void ClearFrameResources();

  raw_ptr<cc::VideoFrameProvider> video_frame_provider_ = nullptr;
  bool is_media_stream_ = false;
  scoped_refptr<viz::RasterContextProvider> context_provider_;
  scoped_refptr<gpu::ClientSharedImageInterface> shared_image_interface_;
  mojo::Remote<viz::mojom::blink::CompositorFrameSink> remote_frame_sink_;
  mojo::Remote<mojom::blink::SurfaceEmbedder> surface_embedder_;
  mojo::Receiver<viz::mojom::blink::CompositorFrameSinkClient> receiver_{this};
  WebContextProviderCallback context_provider_callback_;
  std::unique_ptr<VideoFrameResourceProvider> resource_provider_;
  int waiting_for_compositor_ack_ = 0;

  // When UseVideoFrameSinkBundle is enabled, this is initialized to a local
  // implementation which batches outgoing Viz requests with those from other
  // related VideoFrameSubmitters, rather than having each VideoFrameSubmitter
  // submit their ad hoc requests directly to Viz.
  std::unique_ptr<FrameSinkBundleProxy> bundle_proxy_;

  // Points to either `remote_frame_sink_` or `bundle_proxy_` depending
  // on whether UseVideoFrameSinkBundle is enabled.
  raw_ptr<viz::mojom::blink::CompositorFrameSink> compositor_frame_sink_ =
      nullptr;

  // Current rendering state. Set by StartRendering() and StopRendering().
  bool is_rendering_ = false;

  // If the surface is not visible within in the current view port, we should
  // not submit. Not submitting when off-screen saves significant memory.
  bool is_surface_visible_ = false;

  // Likewise, if the entire page is not visible, we should not submit. Not
  // submitting in the background causes the VideoFrameProvider to enter a
  // background rendering mode using lower frequency artificial BeginFrames.
  bool is_page_visible_ = true;

  // Whether BeginFrames should be generated regardless of visibility. Does not
  // submit unless submission is expected.
  bool force_begin_frames_ = false;

  // Whether frames should always be submitted, even if we're not visible. Used
  // by Picture-in-Picture mode to ensure submission occurs even off-screen.
  bool force_submit_ = false;

  // Needs to be initialized in implementation because media isn't a public_dep
  // of blink/platform.
  media::VideoTransformation transform_;

  viz::FrameSinkId frame_sink_id_;

  // Size of the video frame being submitted. It is set the first time a frame
  // is submitted. Every time there is a change in the video frame size, the
  // child component of the LocalSurfaceId will be updated.
  gfx::Size frame_size_;

  // Used to updated the LocalSurfaceId when detecting a change in video frame
  // size.
  viz::ChildLocalSurfaceIdAllocator child_local_surface_id_allocator_;

  viz::FrameTokenGenerator next_frame_token_;

  std::unique_ptr<cc::VideoPlaybackRoughnessReporter> roughness_reporter_;

  base::OneShotTimer empty_frame_timer_;

  std::optional<media::VideoFrame::ID> last_frame_id_;

  // We use cc::FrameSorter directly, rather than via
  // cc::CompositorFrameReportingController because video frames do not progress
  // through all of the pipeline stages that traditional CompositorFrames do.
  // Instead they are a specialized variant of compositor-only frames, submitted
  // via a batch. So track the mapping of FrameToken to viz::BeginFrameArgs in
  // `pending_frames_`, and denote their completion directly to `frame_sorter_`.
  base::flat_map<uint32_t, viz::BeginFrameArgs> pending_frames_;
  cc::FrameSequenceTrackerCollection frame_trackers_;
  cc::FrameSorter frame_sorter_;

  // The BeginFrameArgs passed to the most recent call of OnBeginFrame().
  // Required for FrameSequenceTrackerCollection::NotifySubmitFrame
  viz::BeginFrameArgs last_begin_frame_args_;

  // The token of the frames that are submitted outside OnBeginFrame(). These
  // frames should be ignored by the video tracker even if they are reported as
  // presented.
  base::flat_set<uint32_t> ignorable_submitted_frames_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // The average delta between receiving a frame and presenting it. Can be used
  // to estimate the expected display time of a frame.
  base::TimeDelta average_delta_between_receive_and_present_;

  Opacity opacity_ = Opacity::kNotReported;

  THREAD_CHECKER(thread_checker_);

  base::WeakPtrFactory<VideoFrameSubmitter> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIDEO_FRAME_SUBMITTER_H_
