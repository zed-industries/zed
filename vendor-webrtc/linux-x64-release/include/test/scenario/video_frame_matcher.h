/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_VIDEO_FRAME_MATCHER_H_
#define TEST_SCENARIO_VIDEO_FRAME_MATCHER_H_

#include <deque>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "api/units/timestamp.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "rtc_base/ref_counted_object.h"
#include "rtc_base/task_queue_for_test.h"
#include "system_wrappers/include/clock.h"
#include "test/scenario/performance_stats.h"

namespace webrtc {
namespace test {

class VideoFrameMatcher {
 public:
  explicit VideoFrameMatcher(
      std::vector<std::function<void(const VideoFramePair&)>>
          frame_pair_handlers);
  ~VideoFrameMatcher();
  void RegisterLayer(int layer_id);
  void OnCapturedFrame(const VideoFrame& frame, Timestamp at_time);
  void OnDecodedFrame(const VideoFrame& frame,
                      int layer_id,
                      Timestamp render_time,
                      Timestamp at_time);
  bool Active() const;

 private:
  struct DecodedFrameBase {
    int id;
    Timestamp decoded_time = Timestamp::PlusInfinity();
    Timestamp render_time = Timestamp::PlusInfinity();
    scoped_refptr<VideoFrameBuffer> frame;
    scoped_refptr<VideoFrameBuffer> thumb;
    int repeat_count = 0;
  };
  using DecodedFrame = FinalRefCountedObject<DecodedFrameBase>;
  struct CapturedFrame {
    int id;
    Timestamp capture_time = Timestamp::PlusInfinity();
    scoped_refptr<VideoFrameBuffer> frame;
    scoped_refptr<VideoFrameBuffer> thumb;
    double best_score = INFINITY;
    scoped_refptr<DecodedFrame> best_decode;
    bool matched = false;
  };
  struct VideoLayer {
    int layer_id;
    std::deque<CapturedFrame> captured_frames;
    scoped_refptr<DecodedFrame> last_decode;
    int next_decoded_id = 1;
  };
  void HandleMatch(CapturedFrame captured, int layer_id);
  void Finalize();
  int next_capture_id_ = 1;
  std::vector<std::function<void(const VideoFramePair&)>> frame_pair_handlers_;
  std::map<int, VideoLayer> layers_;
  TaskQueueForTest task_queue_;
};

class CapturedFrameTap : public VideoSinkInterface<VideoFrame> {
 public:
  CapturedFrameTap(Clock* clock, VideoFrameMatcher* matcher);
  CapturedFrameTap(CapturedFrameTap&) = delete;
  CapturedFrameTap& operator=(CapturedFrameTap&) = delete;

  void OnFrame(const VideoFrame& frame) override;
  void OnDiscardedFrame() override;

 private:
  Clock* const clock_;
  VideoFrameMatcher* const matcher_;
  int discarded_count_ = 0;
};

class ForwardingCapturedFrameTap : public VideoSinkInterface<VideoFrame>,
                                   public VideoSourceInterface<VideoFrame> {
 public:
  ForwardingCapturedFrameTap(Clock* clock,
                             VideoFrameMatcher* matcher,
                             VideoSourceInterface<VideoFrame>* source);
  ForwardingCapturedFrameTap(ForwardingCapturedFrameTap&) = delete;
  ForwardingCapturedFrameTap& operator=(ForwardingCapturedFrameTap&) = delete;

  // VideoSinkInterface interface
  void OnFrame(const VideoFrame& frame) override;
  void OnDiscardedFrame() override;

  // VideoSourceInterface interface
  void AddOrUpdateSink(VideoSinkInterface<VideoFrame>* sink,
                       const VideoSinkWants& wants) override;
  void RemoveSink(VideoSinkInterface<VideoFrame>* sink) override;

 private:
  Clock* const clock_;
  VideoFrameMatcher* const matcher_;
  VideoSourceInterface<VideoFrame>* const source_;
  VideoSinkInterface<VideoFrame>* sink_ = nullptr;
  int discarded_count_ = 0;
};

class DecodedFrameTap : public VideoSinkInterface<VideoFrame> {
 public:
  DecodedFrameTap(Clock* clock, VideoFrameMatcher* matcher, int layer_id);
  // VideoSinkInterface interface
  void OnFrame(const VideoFrame& frame) override;

 private:
  Clock* const clock_;
  VideoFrameMatcher* const matcher_;
  int layer_id_;
};
}  // namespace test
}  // namespace webrtc
#endif  // TEST_SCENARIO_VIDEO_FRAME_MATCHER_H_
