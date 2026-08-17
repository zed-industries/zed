// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_VIDEO_RENDERER_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_VIDEO_RENDERER_SINK_H_

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_sink.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_video_renderer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "ui/gfx/geometry/size.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

// MediaStreamVideoRendererSink is a MediaStreamVideoRenderer designed
// for rendering Video MediaStreamTracks [1], MediaStreamVideoRendererSink
// implements MediaStreamVideoSink in order to render video frames provided from
// a MediaStreamVideoTrack, to which it connects itself when the
// MediaStreamVideoRenderer is Start()ed, and disconnects itself when the latter
// is Stop()ed.
//
// [1] https://dev.w3.org/2011/webrtc/editor/getusermedia.html#mediastreamtrack
//
// TODO(wuchengli): Add unit test. See the link below for reference.
// https://src.chromium.org/viewvc/chrome/trunk/src/content/renderer/media/rtc_
// video_decoder_unittest.cc?revision=180591&view=markup
class MODULES_EXPORT MediaStreamVideoRendererSink
    : public MediaStreamVideoRenderer,
      public MediaStreamVideoSink {
 public:
  MediaStreamVideoRendererSink(
      MediaStreamComponent* video_component,
      const MediaStreamVideoRenderer::RepaintCB& repaint_cb,
      scoped_refptr<base::SequencedTaskRunner> video_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> main_render_task_runner);

  MediaStreamVideoRendererSink(const MediaStreamVideoRendererSink&) = delete;
  MediaStreamVideoRendererSink& operator=(const MediaStreamVideoRendererSink&) =
      delete;

  // MediaStreamVideoRenderer implementation. Called on the main
  // thread.
  void Start() override;
  void Stop() override;
  void Resume() override;
  void Pause() override;

 protected:
  ~MediaStreamVideoRendererSink() override;

 private:
  friend class MediaStreamVideoRendererSinkTest;
  enum State {
    kStarted,
    kPaused,
    kStopped,
  };

  // MediaStreamVideoSink implementation. Called on the main thread.
  void OnReadyStateChanged(WebMediaStreamSource::ReadyState state) override;

  // Helper method used for testing.
  State GetStateForTesting();

  const RepaintCB repaint_cb_;
  Persistent<MediaStreamComponent> video_component_;

  // Inner class used for transfering frames on compositor thread and running
  // |repaint_cb_|.
  class FrameDeliverer;
  std::unique_ptr<FrameDeliverer> frame_deliverer_;

  const scoped_refptr<base::SequencedTaskRunner> video_task_runner_;
  const scoped_refptr<base::SingleThreadTaskRunner> main_render_task_runner_;

  THREAD_CHECKER(main_thread_checker_);

  base::WeakPtrFactory<MediaStreamVideoRendererSink> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_VIDEO_RENDERER_SINK_H_
