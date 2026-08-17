/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_FRAME_WRITER_H_
#define TEST_TESTSUPPORT_FRAME_WRITER_H_

#include <stdio.h>

#include <string>

#include "api/video/video_frame.h"

namespace webrtc {
namespace test {

// Handles writing of video files.
class FrameWriter {
 public:
  virtual ~FrameWriter() {}

  // Initializes the file handler, i.e. opens the input and output files etc.
  // This must be called before reading or writing frames has started.
  // Returns false if an error has occurred, in addition to printing to stderr.
  virtual bool Init() = 0;

  // Writes a frame of the configured frame length to the output file.
  // Returns true if the write was successful, false otherwise.
  virtual bool WriteFrame(const uint8_t* frame_buffer) = 0;

  // Closes the output file if open. Essentially makes this class impossible
  // to use anymore. Will also be invoked by the destructor.
  virtual void Close() = 0;

  // Frame length in bytes of a single frame image.
  virtual size_t FrameLength() = 0;
};

// Writes raw I420 frames in sequence.
class YuvFrameWriterImpl : public FrameWriter {
 public:
  // Creates a file handler. The input file is assumed to exist and be readable
  // and the output file must be writable.
  // Parameters:
  //   output_filename         The file to write. Will be overwritten if already
  //                           existing.
  //   width, height           Size of each frame to read.
  YuvFrameWriterImpl(std::string output_filename, int width, int height);
  ~YuvFrameWriterImpl() override;
  bool Init() override;
  bool WriteFrame(const uint8_t* frame_buffer) override;
  void Close() override;
  size_t FrameLength() override;

 protected:
  const std::string output_filename_;
  size_t frame_length_in_bytes_;
  const int width_;
  const int height_;
  FILE* output_file_;
};

// Writes raw I420 frames in sequence, but with Y4M file and frame headers for
// more convenient playback in external media players.
class Y4mFrameWriterImpl : public YuvFrameWriterImpl {
 public:
  Y4mFrameWriterImpl(std::string output_filename,
                     int width,
                     int height,
                     int frame_rate);
  ~Y4mFrameWriterImpl() override;
  bool Init() override;
  bool WriteFrame(const uint8_t* frame_buffer) override;

 private:
  const int frame_rate_;
};

// LibJpeg is not available on iOS. This class will do nothing on iOS.
class JpegFrameWriter {
 public:
  JpegFrameWriter(const std::string& output_filename);
  // Quality can be from 0 (worst) to 100 (best). Best quality is still lossy.
  // WriteFrame can be called only once. Subsequent calls will fail.
  bool WriteFrame(const VideoFrame& input_frame, int quality);

#if !defined(WEBRTC_IOS)
 private:
  bool frame_written_;
  const std::string output_filename_;
  FILE* output_file_;
#endif
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_FRAME_WRITER_H_
