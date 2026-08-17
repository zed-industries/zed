/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_TEST_Y4M_VIDEO_SOURCE_H_
#define AOM_TEST_Y4M_VIDEO_SOURCE_H_
#include <algorithm>
#include <memory>
#include <string>

#include "common/y4minput.h"
#include "test/video_source.h"

namespace libaom_test {

// This class extends VideoSource to allow parsing of raw yv12
// so that we can do actual file encodes.
class Y4mVideoSource : public VideoSource {
 public:
  Y4mVideoSource(const std::string &file_name, unsigned int start, int limit)
      : file_name_(file_name), input_file_(nullptr), img_(new aom_image_t()),
        start_(start), limit_(limit), frame_(0), framerate_numerator_(0),
        framerate_denominator_(0), y4m_() {}

  ~Y4mVideoSource() override {
    aom_img_free(img_.get());
    CloseSource();
  }

  virtual void OpenSource() {
    CloseSource();
    input_file_ = OpenTestDataFile(file_name_);
    ASSERT_NE(input_file_, nullptr)
        << "Input file open failed. Filename: " << file_name_;
  }

  virtual void ReadSourceToStart() {
    ASSERT_NE(input_file_, nullptr);
    ASSERT_FALSE(
        y4m_input_open(&y4m_, input_file_, nullptr, 0, AOM_CSP_UNKNOWN, 0));
    framerate_numerator_ = y4m_.fps_n;
    framerate_denominator_ = y4m_.fps_d;
    frame_ = 0;
    for (unsigned int i = 0; i < start_; i++) {
      Next();
    }
    FillFrame();
  }

  void Begin() override {
    OpenSource();
    ReadSourceToStart();
  }

  void Next() override {
    ++frame_;
    FillFrame();
  }

  aom_image_t *img() const override {
    return (frame_ < limit_) ? img_.get() : nullptr;
  }

  // Models a stream where Timebase = 1/FPS, so pts == frame.
  aom_codec_pts_t pts() const override { return frame_; }

  unsigned long duration() const override { return 1; }

  aom_rational_t timebase() const override {
    const aom_rational_t t = { framerate_denominator_, framerate_numerator_ };
    return t;
  }

  unsigned int frame() const override { return frame_; }

  unsigned int limit() const override { return limit_; }

  virtual void FillFrame() {
    ASSERT_NE(input_file_, nullptr);
    // Read a frame from input_file.
    y4m_input_fetch_frame(&y4m_, input_file_, img_.get());
  }

  // Swap buffers with another y4m source. This allows reading a new frame
  // while keeping the old frame around. A whole Y4mSource is required and
  // not just a aom_image_t because of how the y4m reader manipulates
  // aom_image_t internals,
  void SwapBuffers(Y4mVideoSource *other) {
    std::swap(other->y4m_.dst_buf, y4m_.dst_buf);
    aom_image_t *tmp;
    tmp = other->img_.release();
    other->img_.reset(img_.release());
    img_.reset(tmp);
  }

 protected:
  void CloseSource() {
    y4m_input_close(&y4m_);
    y4m_ = y4m_input();
    if (input_file_ != nullptr) {
      fclose(input_file_);
      input_file_ = nullptr;
    }
  }

  std::string file_name_;
  FILE *input_file_;
  std::unique_ptr<aom_image_t> img_;
  unsigned int start_;
  unsigned int limit_;
  unsigned int frame_;
  int framerate_numerator_;
  int framerate_denominator_;
  y4m_input y4m_;
};

}  // namespace libaom_test

#endif  // AOM_TEST_Y4M_VIDEO_SOURCE_H_
