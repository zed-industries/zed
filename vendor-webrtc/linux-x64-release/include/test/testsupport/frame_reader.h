/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_FRAME_READER_H_
#define TEST_TESTSUPPORT_FRAME_READER_H_

#include <stdio.h>

#include <memory>
#include <optional>
#include <string>

#include "api/scoped_refptr.h"
#include "api/video/i420_buffer.h"
#include "api/video/resolution.h"

namespace webrtc {
namespace test {

// Handles reading of I420 frames from video files.
class FrameReader {
 public:
  struct Ratio {
    int num = 1;
    int den = 1;
  };

  static constexpr Ratio kNoScale = Ratio({.num = 1, .den = 1});

  virtual ~FrameReader() {}

  // Reads and returns next frame. Returns `nullptr` if reading failed or end of
  // stream is reached.
  virtual scoped_refptr<I420Buffer> PullFrame() = 0;

  // Reads and returns next frame. `frame_num` stores unwrapped frame number
  // which can be passed to `ReadFrame` to re-read this frame later. Returns
  // `nullptr` if reading failed or end of stream is reached.
  virtual scoped_refptr<I420Buffer> PullFrame(int* frame_num) = 0;

  // Reads and returns frame specified by `frame_num`. Returns `nullptr` if
  // reading failed.
  virtual scoped_refptr<I420Buffer> ReadFrame(int frame_num) = 0;

  // Reads next frame, resizes and returns it. `frame_num` stores unwrapped
  // frame number which can be passed to `ReadFrame` to re-read this frame
  // later. `resolution` specifies resolution of the returned frame.
  // `framerate_scale` specifies frame rate scale factor. Frame rate scaling is
  // done by skipping or repeating frames.
  virtual scoped_refptr<I420Buffer> PullFrame(int* frame_num,
                                              Resolution resolution,
                                              Ratio framerate_scale) = 0;

  // Reads frame specified by `frame_num`, resizes and returns it. Returns
  // `nullptr` if reading failed.
  virtual scoped_refptr<I420Buffer> ReadFrame(int frame_num,
                                              Resolution resolution) = 0;

  // Total number of retrievable frames.
  virtual int num_frames() const = 0;
};

class YuvFrameReaderImpl : public FrameReader {
 public:
  enum class RepeatMode { kSingle, kRepeat, kPingPong };

  // Creates the frame reader for a YUV file specified by `filepath`.
  // `resolution` specifies width and height of frames in pixels. `repeat_mode`
  // specifies behaviour of the reader at reaching the end of file (stop, read
  // it over from the beginning or read in reverse order). The file is assumed
  // to exist, be readable and to contain at least 1 frame.
  YuvFrameReaderImpl(std::string filepath,
                     Resolution resolution,
                     RepeatMode repeat_mode);

  ~YuvFrameReaderImpl() override;

  virtual void Init();

  scoped_refptr<I420Buffer> PullFrame() override;

  scoped_refptr<I420Buffer> PullFrame(int* frame_num) override;

  scoped_refptr<I420Buffer> PullFrame(int* frame_num,
                                      Resolution resolution,
                                      Ratio framerate_scale) override;

  scoped_refptr<I420Buffer> ReadFrame(int frame_num) override;

  scoped_refptr<I420Buffer> ReadFrame(int frame_num,
                                      Resolution resolution) override;

  int num_frames() const override { return num_frames_; }

 protected:
  class RateScaler {
   public:
    int Skip(Ratio framerate_scale);

   private:
    std::optional<int> ticks_;
  };

  const std::string filepath_;
  Resolution resolution_;
  const RepeatMode repeat_mode_;
  int num_frames_;
  int frame_num_;
  int frame_size_bytes_;
  int header_size_bytes_;
  FILE* file_;
  RateScaler framerate_scaler_;
};

class Y4mFrameReaderImpl : public YuvFrameReaderImpl {
 public:
  // Creates the frame reader for a Y4M file specified by `filepath`.
  // `repeat_mode` specifies behaviour of the reader at reaching the end of file
  // (stop, read it over from the beginning or read in reverse order). The file
  // is assumed to exist, be readable and to contain at least 1 frame.
  Y4mFrameReaderImpl(std::string filepath, RepeatMode repeat_mode);

  void Init() override;
};

std::unique_ptr<FrameReader> CreateYuvFrameReader(std::string filepath,
                                                  Resolution resolution);

std::unique_ptr<FrameReader> CreateYuvFrameReader(
    std::string filepath,
    Resolution resolution,
    YuvFrameReaderImpl::RepeatMode repeat_mode);

std::unique_ptr<FrameReader> CreateY4mFrameReader(std::string filepath);

std::unique_ptr<FrameReader> CreateY4mFrameReader(
    std::string filepath,
    YuvFrameReaderImpl::RepeatMode repeat_mode);

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_FRAME_READER_H_
