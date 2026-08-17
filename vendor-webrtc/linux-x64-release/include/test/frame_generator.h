/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_FRAME_GENERATOR_H_
#define TEST_FRAME_GENERATOR_H_

#include <memory>
#include <string>
#include <vector>

#include "api/scoped_refptr.h"
#include "api/test/frame_generator_interface.h"
#include "api/video/i420_buffer.h"
#include "api/video/nv12_buffer.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_buffer.h"
#include "api/video/video_source_interface.h"
#include "rtc_base/logging.h"
#include "rtc_base/random.h"
#include "rtc_base/synchronization/mutex.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {
namespace test {

// SquareGenerator is a FrameGenerator that draws a given amount of randomly
// sized and colored squares. Between each new generated frame, the squares
// are moved slightly towards the lower right corner.
class SquareGenerator : public FrameGeneratorInterface {
 public:
  SquareGenerator(int width, int height, OutputType type, int num_squares);

  void ChangeResolution(size_t width, size_t height) override;
  VideoFrameData NextFrame() override;
  Resolution GetResolution() const override;

  std::optional<int> fps() const override { return std::nullopt; }

 private:
  scoped_refptr<I420Buffer> CreateI420Buffer(int width, int height);

  class Square {
   public:
    Square(int width, int height, int seed);

    void Draw(const scoped_refptr<VideoFrameBuffer>& frame_buffer);

   private:
    Random random_generator_;
    int x_;
    int y_;
    const int length_;
    const uint8_t yuv_y_;
    const uint8_t yuv_u_;
    const uint8_t yuv_v_;
    const uint8_t yuv_a_;
  };

  mutable Mutex mutex_;
  const OutputType type_;
  int width_ RTC_GUARDED_BY(&mutex_);
  int height_ RTC_GUARDED_BY(&mutex_);
  std::vector<std::unique_ptr<Square>> squares_ RTC_GUARDED_BY(&mutex_);
};

class YuvFileGenerator : public FrameGeneratorInterface {
 public:
  YuvFileGenerator(std::vector<FILE*> files,
                   size_t width,
                   size_t height,
                   int frame_repeat_count);

  ~YuvFileGenerator();

  VideoFrameData NextFrame() override;
  void ChangeResolution(size_t width, size_t height) override {
    RTC_LOG(LS_WARNING) << "YuvFileGenerator::ChangeResolution not implemented";
  }
  Resolution GetResolution() const override;

  std::optional<int> fps() const override { return std::nullopt; }

 private:
  // Returns true if the new frame was loaded.
  // False only in case of a single file with a single frame in it.
  bool ReadNextFrame();

  size_t file_index_;
  size_t frame_index_;
  const std::vector<FILE*> files_;
  const size_t width_;
  const size_t height_;
  const size_t frame_size_;
  const std::unique_ptr<uint8_t[]> frame_buffer_;
  const int frame_display_count_;
  int current_display_count_;
  scoped_refptr<I420Buffer> last_read_buffer_;
};

class NV12FileGenerator : public FrameGeneratorInterface {
 public:
  NV12FileGenerator(std::vector<FILE*> files,
                    size_t width,
                    size_t height,
                    int frame_repeat_count);

  ~NV12FileGenerator();

  VideoFrameData NextFrame() override;
  void ChangeResolution(size_t width, size_t height) override {
    RTC_LOG(LS_WARNING)
        << "NV12FileGenerator::ChangeResolution not implemented";
  }
  Resolution GetResolution() const override;

  std::optional<int> fps() const override { return std::nullopt; }

 private:
  // Returns true if the new frame was loaded.
  // False only in case of a single file with a single frame in it.
  bool ReadNextFrame();

  size_t file_index_;
  size_t frame_index_;
  const std::vector<FILE*> files_;
  const size_t width_;
  const size_t height_;
  const size_t frame_size_;
  const std::unique_ptr<uint8_t[]> frame_buffer_;
  const int frame_display_count_;
  int current_display_count_;
  scoped_refptr<NV12Buffer> last_read_buffer_;
};

// SlideGenerator works similarly to YuvFileGenerator but it fills the frames
// with randomly sized and colored squares instead of reading their content
// from files.
class SlideGenerator : public FrameGeneratorInterface {
 public:
  SlideGenerator(int width, int height, int frame_repeat_count);

  VideoFrameData NextFrame() override;
  void ChangeResolution(size_t width, size_t height) override {
    RTC_LOG(LS_WARNING) << "SlideGenerator::ChangeResolution not implemented";
  }
  Resolution GetResolution() const override;

  std::optional<int> fps() const override { return std::nullopt; }

 private:
  // Generates some randomly sized and colored squares scattered
  // over the frame.
  void GenerateNewFrame();

  const int width_;
  const int height_;
  const int frame_display_count_;
  int current_display_count_;
  Random random_generator_;
  scoped_refptr<I420Buffer> buffer_;
};

class ScrollingImageFrameGenerator : public FrameGeneratorInterface {
 public:
  ScrollingImageFrameGenerator(Clock* clock,
                               const std::vector<FILE*>& files,
                               size_t source_width,
                               size_t source_height,
                               size_t target_width,
                               size_t target_height,
                               int64_t scroll_time_ms,
                               int64_t pause_time_ms);
  ~ScrollingImageFrameGenerator() override = default;

  VideoFrameData NextFrame() override;
  void ChangeResolution(size_t width, size_t height) override {
    RTC_LOG(LS_WARNING)
        << "ScrollingImageFrameGenerator::ChangeResolution not implemented";
  }
  Resolution GetResolution() const override;

  std::optional<int> fps() const override { return std::nullopt; }

 private:
  void UpdateSourceFrame(size_t frame_num);
  void CropSourceToScrolledImage(double scroll_factor);

  Clock* const clock_;
  const int64_t start_time_;
  const int64_t scroll_time_;
  const int64_t pause_time_;
  const size_t num_frames_;
  const int target_width_;
  const int target_height_;

  size_t current_frame_num_;
  bool prev_frame_not_scrolled_;
  VideoFrameData current_source_frame_;
  VideoFrameData current_frame_;
  YuvFileGenerator file_generator_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FRAME_GENERATOR_H_
