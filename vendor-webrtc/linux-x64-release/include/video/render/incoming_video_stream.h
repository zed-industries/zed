/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_RENDER_INCOMING_VIDEO_STREAM_H_
#define VIDEO_RENDER_INCOMING_VIDEO_STREAM_H_

#include <stdint.h>

#include <memory>

#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/task_queue/task_queue_factory.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "rtc_base/race_checker.h"
#include "rtc_base/thread_annotations.h"
#include "video/render/video_render_frames.h"

namespace webrtc {

class IncomingVideoStream : public VideoSinkInterface<VideoFrame> {
 public:
  IncomingVideoStream(TaskQueueFactory* task_queue_factory,
                      int32_t delay_ms,
                      VideoSinkInterface<VideoFrame>* callback);
  ~IncomingVideoStream() override;

 private:
  void OnFrame(const VideoFrame& video_frame) override;
  void Dequeue();

  SequenceChecker main_thread_checker_;
  RaceChecker decoder_race_checker_;

  VideoRenderFrames render_buffers_ RTC_GUARDED_BY(incoming_render_queue_);
  VideoSinkInterface<VideoFrame>* const callback_;
  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> incoming_render_queue_;
};

}  // namespace webrtc

#endif  // VIDEO_RENDER_INCOMING_VIDEO_STREAM_H_
