// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_VIDEO_FRAME_SUBMITTER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_VIDEO_FRAME_SUBMITTER_H_

#include "cc/layers/video_frame_provider.h"
#include "cc/metrics/video_playback_roughness_reporter.h"
#include "components/viz/common/surfaces/frame_sink_id.h"
#include "components/viz/common/surfaces/surface_id.h"
#include "third_party/blink/public/platform/web_common.h"

namespace cc {
class LayerTreeSettings;
}

namespace media {
enum VideoRotation : int;
}

namespace viz {
class RasterContextProvider;
}  // namespace viz

namespace gpu {
class ClientSharedImageInterface;
}  // namespace gpu

namespace blink {

// Sets the proper context_provider and compositing mode onto the Submitter.
using WebSubmitterConfigurationCallback =
    base::OnceCallback<void(bool,
                            scoped_refptr<viz::RasterContextProvider>,
                            scoped_refptr<gpu::ClientSharedImageInterface>)>;

// Callback to obtain the media RasterContextProvider and a bool indicating
// whether we are in software compositing mode.
using WebContextProviderCallback =
    base::RepeatingCallback<void(scoped_refptr<viz::RasterContextProvider>,
                                 WebSubmitterConfigurationCallback)>;

// Exposes the VideoFrameSubmitter, which submits CompositorFrames containing
// decoded VideoFrames from the VideoFrameProvider to the compositor for
// display.
class BLINK_PLATFORM_EXPORT WebVideoFrameSubmitter
    : public cc::VideoFrameProvider::Client {
 public:
  static std::unique_ptr<WebVideoFrameSubmitter> Create(
      WebContextProviderCallback,
      cc::VideoPlaybackRoughnessReporter::ReportingCallback,
      const cc::LayerTreeSettings&,
      bool use_sync_primitives);
  ~WebVideoFrameSubmitter() override = default;

  // Intialize must be called before submissions occur, pulled out of
  // StartSubmitting() to enable tests without the full mojo statck running.
  virtual void Initialize(cc::VideoFrameProvider*, bool is_media_stream) = 0;

  // Set the rotation state of the video to be used while appending frames.
  //
  // TODO(dalecurtis): This could be removed in favor of getting it from each
  // VideoFrame, but today that information isn't set everywhere.
  virtual void SetTransform(media::VideoTransformation) = 0;

  // Prepares the compositor frame sink to accept frames by providing
  // a SurfaceId.
  virtual void EnableSubmission(viz::SurfaceId) = 0;

  // Set whether the surface is visible within the current view port. Stops
  // submission if not unless SetForceSubmit(true) has been called.
  virtual void SetIsSurfaceVisible(bool) = 0;

  // Set whether the page containing the video element is visible. Stops
  // submission if not unless SetForceSubmit(true) has been called.
  virtual void SetIsPageVisible(bool) = 0;

  // Set whether BeginFrames should be generated regardless of visibility. Does
  // not submit unless submission is expected.
  virtual void SetForceBeginFrames(bool) = 0;

  // Set whether frames should always be submitted regardless of visibility.
  virtual void SetForceSubmit(bool) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_VIDEO_FRAME_SUBMITTER_H_
