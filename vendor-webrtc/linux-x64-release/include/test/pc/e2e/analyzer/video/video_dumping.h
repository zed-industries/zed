/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_DUMPING_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_DUMPING_H_

#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "api/test/video/video_frame_writer.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "test/testsupport/video_frame_writer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// `VideoSinkInterface` to dump incoming video frames into specified video
// writer.
class VideoWriter final : public VideoSinkInterface<VideoFrame> {
 public:
  // Creates video writer. Caller keeps ownership of `video_writer` and is
  // responsible for closing it after VideoWriter will be destroyed.
  VideoWriter(test::VideoFrameWriter* video_writer, int sampling_modulo);
  VideoWriter(const VideoWriter&) = delete;
  VideoWriter& operator=(const VideoWriter&) = delete;
  ~VideoWriter() override = default;

  void OnFrame(const VideoFrame& frame) override;

 private:
  test::VideoFrameWriter* const video_writer_;
  const int sampling_modulo_;

  int64_t frames_counter_ = 0;
};

// Creates a `VideoFrameWriter` to dump video frames together with their ids.
// It uses provided `video_writer_delegate` to write video itself. Frame ids
// will be logged into the specified file.
std::unique_ptr<test::VideoFrameWriter> CreateVideoFrameWithIdsWriter(
    std::unique_ptr<test::VideoFrameWriter> video_writer_delegate,
    absl::string_view frame_ids_dump_file_name);

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_DUMPING_H_
