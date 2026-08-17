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
#ifndef AOM_TEST_WEBM_VIDEO_SOURCE_H_
#define AOM_TEST_WEBM_VIDEO_SOURCE_H_
#include <cstdarg>
#include <cstdio>
#include <cstdlib>
#include <new>
#include <string>
#include "common/tools_common.h"
#include "common/webmdec.h"
#include "test/video_source.h"

namespace libaom_test {

// This class extends VideoSource to allow parsing of WebM files,
// so that we can do actual file decodes.
class WebMVideoSource : public CompressedVideoSource {
 public:
  explicit WebMVideoSource(const std::string &file_name)
      : file_name_(file_name), aom_ctx_(new AvxInputContext()),
        webm_ctx_(new WebmInputContext()), buf_(nullptr), buf_sz_(0),
        frame_sz_(0), frame_number_(0), end_of_file_(false) {}

  ~WebMVideoSource() override {
    if (aom_ctx_->file != nullptr) fclose(aom_ctx_->file);
    webm_free(webm_ctx_);
    delete aom_ctx_;
    delete webm_ctx_;
  }

  void Init() override {
    ASSERT_NE(aom_ctx_, nullptr);
    ASSERT_NE(webm_ctx_, nullptr);
  }

  void Begin() override {
    ASSERT_NE(aom_ctx_, nullptr);
    ASSERT_NE(webm_ctx_, nullptr);
    aom_ctx_->file = OpenTestDataFile(file_name_);
    ASSERT_NE(aom_ctx_->file, nullptr)
        << "Input file open failed. Filename: " << file_name_;

    ASSERT_EQ(file_is_webm(webm_ctx_, aom_ctx_), 1) << "file is not WebM";

    FillFrame();
  }

  void Next() override {
    ++frame_number_;
    FillFrame();
  }

  void FillFrame() {
    ASSERT_NE(aom_ctx_, nullptr);
    ASSERT_NE(webm_ctx_, nullptr);
    ASSERT_NE(aom_ctx_->file, nullptr);
    const int status = webm_read_frame(webm_ctx_, &buf_, &frame_sz_, &buf_sz_);
    ASSERT_GE(status, 0) << "webm_read_frame failed";
    if (status == 1) {
      end_of_file_ = true;
    }
  }

  void SeekToNextKeyFrame() {
    ASSERT_NE(aom_ctx_, nullptr);
    ASSERT_NE(webm_ctx_, nullptr);
    ASSERT_NE(aom_ctx_->file, nullptr);
    do {
      const int status =
          webm_read_frame(webm_ctx_, &buf_, &frame_sz_, &buf_sz_);
      ASSERT_GE(status, 0) << "webm_read_frame failed";
      ++frame_number_;
      if (status == 1) {
        end_of_file_ = true;
      }
    } while (!webm_ctx_->is_key_frame && !end_of_file_);
  }

  const uint8_t *cxdata() const override {
    return end_of_file_ ? nullptr : buf_;
  }
  size_t frame_size() const override { return frame_sz_; }
  unsigned int frame_number() const override { return frame_number_; }

 protected:
  std::string file_name_;
  AvxInputContext *aom_ctx_;
  WebmInputContext *webm_ctx_;
  uint8_t *buf_;  // Owned by webm_ctx_ and freed when webm_ctx_ is freed.
  size_t buf_sz_;
  size_t frame_sz_;
  unsigned int frame_number_;
  bool end_of_file_;
};

}  // namespace libaom_test

#endif  // AOM_TEST_WEBM_VIDEO_SOURCE_H_
