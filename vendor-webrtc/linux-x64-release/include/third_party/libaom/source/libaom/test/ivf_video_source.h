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
#ifndef AOM_TEST_IVF_VIDEO_SOURCE_H_
#define AOM_TEST_IVF_VIDEO_SOURCE_H_

#include <cstdio>
#include <cstdlib>
#include <new>
#include <string>

#include "aom_ports/sanitizer.h"
#include "test/video_source.h"

namespace libaom_test {
const unsigned int kCodeBufferSize = 256 * 1024 * 1024;
const unsigned int kIvfFileHdrSize = 32;
const unsigned int kIvfFrameHdrSize = 12;

static unsigned int MemGetLe32(const uint8_t *mem) {
  return (mem[3] << 24) | (mem[2] << 16) | (mem[1] << 8) | (mem[0]);
}

// This class extends VideoSource to allow parsing of ivf files,
// so that we can do actual file decodes.
class IVFVideoSource : public CompressedVideoSource {
 public:
  explicit IVFVideoSource(const std::string &file_name)
      : file_name_(file_name), input_file_(nullptr),
        compressed_frame_buf_(nullptr), frame_sz_(0), frame_(0),
        end_of_file_(false) {}

  ~IVFVideoSource() override {
    delete[] compressed_frame_buf_;

    if (input_file_) fclose(input_file_);
  }

  void Init() override {
    // Allocate a buffer for read in the compressed video frame.
    compressed_frame_buf_ = new uint8_t[kCodeBufferSize];
    ASSERT_NE(compressed_frame_buf_, nullptr) << "Allocate frame buffer failed";
    ASAN_POISON_MEMORY_REGION(compressed_frame_buf_, kCodeBufferSize);
  }

  void Begin() override {
    input_file_ = OpenTestDataFile(file_name_);
    ASSERT_NE(input_file_, nullptr)
        << "Input file open failed. Filename: " << file_name_;

    // Read file header
    uint8_t file_hdr[kIvfFileHdrSize];
    ASSERT_EQ(kIvfFileHdrSize, fread(file_hdr, 1, kIvfFileHdrSize, input_file_))
        << "File header read failed.";
    // Check file header
    ASSERT_TRUE(file_hdr[0] == 'D' && file_hdr[1] == 'K' &&
                file_hdr[2] == 'I' && file_hdr[3] == 'F')
        << "Input is not an IVF file.";

    FillFrame();
  }

  void Next() override {
    ++frame_;
    FillFrame();
  }

  void FillFrame() {
    ASSERT_NE(input_file_, nullptr);
    uint8_t frame_hdr[kIvfFrameHdrSize];
    // Check frame header and read a frame from input_file.
    if (fread(frame_hdr, 1, kIvfFrameHdrSize, input_file_) !=
        kIvfFrameHdrSize) {
      end_of_file_ = true;
    } else {
      end_of_file_ = false;

      frame_sz_ = MemGetLe32(frame_hdr);
      ASSERT_LE(frame_sz_, kCodeBufferSize)
          << "Frame is too big for allocated code buffer";
      ASAN_UNPOISON_MEMORY_REGION(compressed_frame_buf_, kCodeBufferSize);
      ASSERT_EQ(frame_sz_,
                fread(compressed_frame_buf_, 1, frame_sz_, input_file_))
          << "Failed to read complete frame";
      ASAN_POISON_MEMORY_REGION(compressed_frame_buf_ + frame_sz_,
                                kCodeBufferSize - frame_sz_);
    }
  }

  const uint8_t *cxdata() const override {
    return end_of_file_ ? nullptr : compressed_frame_buf_;
  }
  size_t frame_size() const override { return frame_sz_; }
  unsigned int frame_number() const override { return frame_; }

 protected:
  std::string file_name_;
  FILE *input_file_;
  uint8_t *compressed_frame_buf_;
  size_t frame_sz_;
  unsigned int frame_;
  bool end_of_file_;
};

}  // namespace libaom_test

#endif  // AOM_TEST_IVF_VIDEO_SOURCE_H_
