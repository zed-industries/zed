// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_VIDEO_ELEMENT_CAPTURER_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_VIDEO_ELEMENT_CAPTURER_SOURCE_H_

#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "media/base/video_types.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/video_capture/video_capturer_source.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class WebMediaPlayer;

// This class is a VideoCapturerSource taking video snapshots of the ctor-passed
// blink::WebMediaPlayer on Render Main thread. The captured data is converted
// and sent back to |io_task_runner_| via the registered |new_frame_callback_|.
class MODULES_EXPORT HtmlVideoElementCapturerSource final
    : public VideoCapturerSource {
 public:
  static std::unique_ptr<HtmlVideoElementCapturerSource>
  CreateFromWebMediaPlayerImpl(
      blink::WebMediaPlayer* player,
      const scoped_refptr<base::SingleThreadTaskRunner>& io_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  HtmlVideoElementCapturerSource(
      const base::WeakPtr<blink::WebMediaPlayer>& player,
      const scoped_refptr<base::SingleThreadTaskRunner>& io_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  HtmlVideoElementCapturerSource(const HtmlVideoElementCapturerSource&) =
      delete;
  HtmlVideoElementCapturerSource& operator=(
      const HtmlVideoElementCapturerSource&) = delete;

  ~HtmlVideoElementCapturerSource() override;

  // VideoCapturerSource Implementation.
  media::VideoCaptureFormats GetPreferredFormats() override;
  void StartCapture(
      const media::VideoCaptureParams& params,
      const VideoCaptureDeliverFrameCB& new_frame_callback,
      const VideoCaptureSubCaptureTargetVersionCB&
          sub_capture_target_version_callback,
      const VideoCaptureNotifyFrameDroppedCB& frame_dropped_callback,
      const RunningCallback& running_callback) override;
  void StopCapture() override;

 private:
  friend class HTMLVideoElementCapturerSourceTest;

  // This method includes collecting data from the WebMediaPlayer and converting
  // it into a format suitable for MediaStreams.
  void sendNewFrame();

  gfx::Size natural_size_;

  const base::WeakPtr<blink::WebMediaPlayer> web_media_player_;
  const scoped_refptr<base::SingleThreadTaskRunner> io_task_runner_;
  const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // These three configuration items are passed on StartCapture();
  RunningCallback running_callback_;
  VideoCaptureDeliverFrameCB new_frame_callback_;
  double capture_frame_rate_;

  // base::TimeTicks on which the first captured VideoFrame is produced.
  base::TimeTicks start_capture_time_;

  // Target time for the next frame.
  base::TimeTicks next_capture_time_;

  // Bound to the main render thread.
  THREAD_CHECKER(thread_checker_);

  // Used on main render thread to schedule future capture events.
  base::WeakPtrFactory<HtmlVideoElementCapturerSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_VIDEO_ELEMENT_CAPTURER_SOURCE_H_
