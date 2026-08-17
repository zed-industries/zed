/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_Y4M_FRAME_GENERATOR_H_
#define TEST_TESTSUPPORT_Y4M_FRAME_GENERATOR_H_

#include <cstddef>
#include <memory>
#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "api/test/frame_generator_interface.h"
#include "rtc_base/checks.h"
#include "test/testsupport/frame_reader.h"

namespace webrtc {
namespace test {

// Generates frames from a Y4M file. The behaviour when reaching EOF is
// configurable via RepeatMode.
class Y4mFrameGenerator : public FrameGeneratorInterface {
 public:
  enum class RepeatMode {
    // Generate frames from the input file, but it stops generating new frames
    // once EOF is reached.
    kSingle,
    // Generate frames from the input file, when EOF is reached it starts from
    // the beginning.
    kLoop,
    // Generate frames from the input file, when EOF is reached it plays frames
    // backwards from the end to the beginning of the file (and vice versa,
    // literally doing Ping/Pong between the beginning and the end of the file).
    kPingPong,
  };
  Y4mFrameGenerator(absl::string_view filename, RepeatMode repeat_mode);
  ~Y4mFrameGenerator() override = default;

  VideoFrameData NextFrame() override;

  void SkipNextFrame() override;

  void ChangeResolution(size_t width, size_t height) override;

  Resolution GetResolution() const override;

  std::optional<int> fps() const override { return fps_; }

 private:
  YuvFrameReaderImpl::RepeatMode ToYuvFrameReaderRepeatMode(
      RepeatMode repeat_mode) const;
  std::unique_ptr<webrtc::test::FrameReader> frame_reader_ = nullptr;
  std::string filename_;
  size_t width_;
  size_t height_;
  int fps_;
  const RepeatMode repeat_mode_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_Y4M_FRAME_GENERATOR_H_
