/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_FIXED_FPS_VIDEO_FRAME_WRITER_ADAPTER_H_
#define TEST_TESTSUPPORT_FIXED_FPS_VIDEO_FRAME_WRITER_ADAPTER_H_

#include <memory>
#include <optional>

#include "api/test/video/video_frame_writer.h"
#include "api/video/video_sink_interface.h"
#include "system_wrappers/include/clock.h"
#include "test/testsupport/video_frame_writer.h"

namespace webrtc {
namespace test {

// Writes video to the specified video writer with specified fixed frame rate.
// If at the point in time X no new frames are passed to the writer, the
// previous frame is used to fill the gap and preserve frame rate.
//
// This adaptor uses next algorithm:
// There are output "slots" at a fixed frame rate (starting at the time of the
// first received frame). Each incoming frame is assigned to the closest output
// slot. Then empty slots are filled by repeating the closest filled slot before
// empty one. If there are multiple frames closest to the same slot, the latest
// received one is used.
//
// The frames are outputted for the whole duration of the class life after the
// first frame was written or until it will be closed.
//
// For example if frames from A to F were received, then next output sequence
// will be generated:
// Received frames:  A            B       C      D  EF        Destructor called
//                   |            |       |      |  ||          |
//                   v            v       v      v  vv          v
//                   X----X----X----X----X----X----X----X----X----+----+--
//                   |    |    |    |    |    |    |    |    |
// Produced frames:  A    A    A    B    C    C    F    F    F
//
// This class is not thread safe.
class FixedFpsVideoFrameWriterAdapter : public VideoFrameWriter {
 public:
  FixedFpsVideoFrameWriterAdapter(int fps,
                                  Clock* clock,
                                  std::unique_ptr<VideoFrameWriter> delegate);
  ~FixedFpsVideoFrameWriterAdapter() override;

  bool WriteFrame(const webrtc::VideoFrame& frame) override;

  // Closes adapter and underlying delegate. User mustn't call to the WriteFrame
  // after calling this method.
  void Close() override;

 private:
  // Writes `last_frame_` for each "slot" from `last_frame_time_` up to now
  // excluding the last one.
  // Updates `last_frame_time_` to the position of the last NOT WRITTEN frame.
  // Returns true if all writes were successful, otherwise retuns false. In such
  // case it is not guranteed how many frames were actually written.
  bool WriteMissedSlotsExceptLast(Timestamp now);
  Timestamp Now() const;

  // Because `TimeDelta` stores time with microseconds precision
  // `last_frame_time_` may have a small drift and for very long streams it
  // must be updated to use double for time.
  const TimeDelta inter_frame_interval_;
  Clock* const clock_;
  std::unique_ptr<VideoFrameWriter> delegate_;
  bool is_closed_ = false;

  // Expected time slot for the last frame.
  Timestamp last_frame_time_ = Timestamp::MinusInfinity();
  std::optional<VideoFrame> last_frame_ = std::nullopt;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_FIXED_FPS_VIDEO_FRAME_WRITER_ADAPTER_H_
