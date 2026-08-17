/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_I420_BUFFER_H_
#define API_VIDEO_I420_BUFFER_H_

#include <stdint.h>

#include <memory>

#include "api/scoped_refptr.h"
#include "api/video/video_frame_buffer.h"
#include "api/video/video_rotation.h"
#include "rtc_base/memory/aligned_malloc.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Plain I420 buffer in standard memory.
class RTC_EXPORT I420Buffer : public I420BufferInterface {
 public:
  static scoped_refptr<I420Buffer> Create(int width, int height);
  static scoped_refptr<I420Buffer> Create(int width,
                                          int height,
                                          int stride_y,
                                          int stride_u,
                                          int stride_v);

  // Create a new buffer and copy the pixel data.
  static scoped_refptr<I420Buffer> Copy(const I420BufferInterface& buffer);
  // Deprecated.
  static scoped_refptr<I420Buffer> Copy(const VideoFrameBuffer& buffer) {
    return Copy(*buffer.GetI420());
  }

  static scoped_refptr<I420Buffer> Copy(int width,
                                        int height,
                                        const uint8_t* data_y,
                                        int stride_y,
                                        const uint8_t* data_u,
                                        int stride_u,
                                        const uint8_t* data_v,
                                        int stride_v);

  // Returns a rotated copy of `src`.
  static scoped_refptr<I420Buffer> Rotate(const I420BufferInterface& src,
                                          VideoRotation rotation);
  // Deprecated.
  static scoped_refptr<I420Buffer> Rotate(const VideoFrameBuffer& src,
                                          VideoRotation rotation) {
    return Rotate(*src.GetI420(), rotation);
  }

  // Sets the buffer to all black.
  static void SetBlack(I420Buffer* buffer);

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

  // Scale the cropped area of `src` to the size of `this` buffer, and
  // write the result into `this`.
  void CropAndScaleFrom(const I420BufferInterface& src,
                        int offset_x,
                        int offset_y,
                        int crop_width,
                        int crop_height);

  // The common case of a center crop, when needed to adjust the
  // aspect ratio without distorting the image.
  void CropAndScaleFrom(const I420BufferInterface& src);

  // Scale all of `src` to the size of `this` buffer, with no cropping.
  void ScaleFrom(const I420BufferInterface& src);

 protected:
  I420Buffer(int width, int height);
  I420Buffer(int width, int height, int stride_y, int stride_u, int stride_v);

  ~I420Buffer() override;

 private:
  const int width_;
  const int height_;
  const int stride_y_;
  const int stride_u_;
  const int stride_v_;
  const std::unique_ptr<uint8_t, AlignedFreeDeleter> data_;
};

}  // namespace webrtc

#endif  // API_VIDEO_I420_BUFFER_H_
