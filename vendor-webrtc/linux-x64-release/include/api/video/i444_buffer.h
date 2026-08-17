/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_I444_BUFFER_H_
#define API_VIDEO_I444_BUFFER_H_

#include <stdint.h>

#include <memory>

#include "api/scoped_refptr.h"
#include "api/video/video_frame_buffer.h"
#include "api/video/video_rotation.h"
#include "rtc_base/memory/aligned_malloc.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Plain I444 buffer in standard memory.
// I444 represents an image with in YUV format withouth any chroma subsampling.
// https://en.wikipedia.org/wiki/Chroma_subsampling#4:4:4
class RTC_EXPORT I444Buffer : public I444BufferInterface {
 public:
  static scoped_refptr<I444Buffer> Create(int width, int height);
  static scoped_refptr<I444Buffer> Create(int width,
                                          int height,
                                          int stride_y,
                                          int stride_u,
                                          int stride_v);

  // Create a new buffer and copy the pixel data.
  static scoped_refptr<I444Buffer> Copy(const I444BufferInterface& buffer);

  static scoped_refptr<I444Buffer> Copy(int width,
                                        int height,
                                        const uint8_t* data_y,
                                        int stride_y,
                                        const uint8_t* data_u,
                                        int stride_u,
                                        const uint8_t* data_v,
                                        int stride_v);

  // Returns a rotated copy of |src|.
  static scoped_refptr<I444Buffer> Rotate(const I444BufferInterface& src,
                                          VideoRotation rotation);

  scoped_refptr<I420BufferInterface> ToI420() final;
  const I420BufferInterface* GetI420() const final { return nullptr; }

  // Sets all three planes to all zeros. Used to work around for
  // quirks in memory checkers
  // (https://bugs.chromium.org/p/libyuv/issues/detail?id=377) and
  // ffmpeg (http://crbug.com/390941).
  // TODO(https://crbug.com/390941): Deprecated. Should be deleted if/when those
  // issues are resolved in a better way. Or in the mean time, use SetBlack.
  void InitializeData();

  int width() const override;
  int height() const override;
  const uint8_t* DataY() const override;
  const uint8_t* DataU() const override;
  const uint8_t* DataV() const override;

  int StrideY() const override;
  int StrideU() const override;
  int StrideV() const override;

  uint8_t* MutableDataY();
  uint8_t* MutableDataU();
  uint8_t* MutableDataV();

  // Scale the cropped area of |src| to the size of |this| buffer, and
  // write the result into |this|.
  void CropAndScaleFrom(const I444BufferInterface& src,
                        int offset_x,
                        int offset_y,
                        int crop_width,
                        int crop_height);

 protected:
  I444Buffer(int width, int height);
  I444Buffer(int width, int height, int stride_y, int stride_u, int stride_v);

  ~I444Buffer() override;

 private:
  const int width_;
  const int height_;
  const int stride_y_;
  const int stride_u_;
  const int stride_v_;
  const std::unique_ptr<uint8_t, AlignedFreeDeleter> data_;
};

}  // namespace webrtc

#endif  // API_VIDEO_I444_BUFFER_H_
