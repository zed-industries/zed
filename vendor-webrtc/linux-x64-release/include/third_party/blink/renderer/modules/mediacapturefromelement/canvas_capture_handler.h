// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_CANVAS_CAPTURE_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_CANVAS_CAPTURE_HANDLER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "gpu/GLES2/gl2extchromium.h"
#include "media/base/video_frame_pool.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/video_capture/video_capturer_source.h"
#include "third_party/skia/include/core/SkImageInfo.h"
#include "third_party/skia/include/gpu/ganesh/GrTypes.h"

class SkImage;

namespace base {
class ScopedClosureRunner;
}  // namespace base

namespace gfx {
class Size;
}  // namespace gfx

namespace blink {

class LocalFrame;
class MediaStreamComponent;

// CanvasCaptureHandler acts as the link between Blink side HTMLCanvasElement
// and Chrome side VideoCapturerSource. It is responsible for handling
// SkImage instances sent from the Blink side, convert them to
// media::VideoFrame and plug them to the MediaStreamTrack.
// CanvasCaptureHandler instance is owned by a blink::CanvasDrawListener which
// is owned by a CanvasCaptureMediaStreamTrack.
// All methods are called on the same thread as construction and destruction,
// i.e. the Main Render thread. Note that a CanvasCaptureHandlerDelegate is
// used to send back frames to |io_task_runner_|, i.e. IO thread.
class MODULES_EXPORT CanvasCaptureHandler {
 public:
  CanvasCaptureHandler(const CanvasCaptureHandler&) = delete;
  CanvasCaptureHandler& operator=(const CanvasCaptureHandler&) = delete;

  ~CanvasCaptureHandler();

  // Creates a CanvasCaptureHandler instance and updates UMA histogram.
  static std::unique_ptr<CanvasCaptureHandler> CreateCanvasCaptureHandler(
      LocalFrame* frame,
      const gfx::Size& size,
      double frame_rate,
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> io_task_runner,
      MediaStreamComponent** component);

  // Return a callback to provide a new frame. See the method
  // CanvasCaptureHandler::GetNewFrameCallback for more details.
  using NewFrameCallback =
      base::OnceCallback<void(scoped_refptr<media::VideoFrame>)>;
  NewFrameCallback GetNewFrameCallback();
  bool CanDiscardAlpha() const { return can_discard_alpha_; }
  bool NeedsNewFrame() const;

  // Functions called by VideoCapturerSource implementation.
  void StartVideoCapture(
      const media::VideoCaptureParams& params,
      const VideoCaptureDeliverFrameCB& new_frame_callback,
      const VideoCapturerSource::RunningCallback& running_callback);
  void RequestRefreshFrame();
  void StopVideoCapture();
  void SetCanDiscardAlpha(bool can_discard_alpha) {
    can_discard_alpha_ = can_discard_alpha;
  }

 private:
  // A VideoCapturerSource instance is created, which is responsible for handing
  // stop&start callbacks back to CanvasCaptureHandler. That VideoCapturerSource
  // is then plugged into a MediaStreamTrack passed as |track|, and it is owned
  // by the Blink side MediaStreamSource.
  CanvasCaptureHandler(
      LocalFrame* frame,
      const gfx::Size& size,
      double frame_rate,
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> io_task_runner,
      MediaStreamComponent** component);

  void OnNewFrameCallback(base::ScopedClosureRunner pending_call_tracker,
                          base::TimeTicks this_frame_ticks,
                          const gfx::ColorSpace& color_space,
                          scoped_refptr<media::VideoFrame> video_frame);

  void SendFrame(base::TimeTicks this_frame_ticks,
                 const gfx::ColorSpace& color_space,
                 scoped_refptr<media::VideoFrame> video_frame);

  void AddVideoCapturerSourceToVideoTrack(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      LocalFrame* frame,
      std::unique_ptr<VideoCapturerSource> source,
      MediaStreamComponent** component);

  // Send a refresh frame.
  void SendRefreshFrame();

  // Object that does all the work of running |new_frame_callback_|.
  // Destroyed on |frame_callback_task_runner_| after the class is destroyed.
  class CanvasCaptureHandlerDelegate;

  media::VideoCaptureFormat capture_format_;
  bool can_discard_alpha_ = false;
  bool ask_for_new_frame_ = false;
  std::optional<base::TimeTicks> first_frame_ticks_;
  scoped_refptr<media::VideoFrame> last_frame_;

  // The following attributes ensure that CanvasCaptureHandler emits
  // frames with monotonically increasing timestamps.
  bool deferred_request_refresh_frame_ = false;

  // The number of outsanding calls to SendNewFrame that have not made their
  // callback.
  uint32_t pending_send_new_frame_calls_ = 0;

  const scoped_refptr<base::SingleThreadTaskRunner> io_task_runner_;
  std::unique_ptr<CanvasCaptureHandlerDelegate> delegate_;

  // Bound to Main Render thread.
  THREAD_CHECKER(main_render_thread_checker_);
  base::WeakPtrFactory<CanvasCaptureHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_CANVAS_CAPTURE_HANDLER_H_
