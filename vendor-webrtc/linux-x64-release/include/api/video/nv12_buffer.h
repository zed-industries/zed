/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_NV12_BUFFER_H_
#define API_VIDEO_NV12_BUFFER_H_

#include <cstddef>
#include <cstdint>
#include <memory>

#include "api/scoped_refptr.h"
#include "api/video/video_frame_buffer.h"
#include "rtc_base/memory/aligned_malloc.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// NV12 is a biplanar encoding format, with full-resolution Y and
// half-resolution interleved UV. More information can be found at
// http://msdn.microsoft.com/library/windows/desktop/dd206750.aspx#nv12.
class RTC_EXPORT NV12Buffer : public NV12BufferInterface {
 public:
  static scoped_refptr<NV12Buffer> Create(int width, int height);
  static scoped_refptr<NV12Buffer> Create(int width,
                                          int height,
                                          int stride_y,
                                          int stride_uv);
  static scoped_refptr<NV12Buffer> Copy(const I420BufferInterface& i420_buffer);

  scoped_refptr<I420BufferInterface> ToI420() override;

  int width() const override;
  int height() const override;

  int StrideY() const override;
  int StrideUV() const override;

  const uint8_t* DataY() const override;
  const uint8_t* DataUV() const override;

  uint8_t* MutableDataY();
  uint8_t* MutableDataUV();

  // Sets all three planes to all zeros. Used to work around for
  // quirks in memory checkers
  // (https://bugs.chromium.org/p/libyuv/issues/detail?id=377) and
  // ffmpeg (http://crbug.com/390941).
  // TODO(https://crbug.com/390941): Deprecated. Should be deleted if/when those
  // issues are resolved in a better way. Or in the mean time, use SetBlack.
  void InitializeData();

  // Scale the cropped area of `src` to the size of `this` buffer, and
  // write the result into `this`.
  void CropAndScaleFrom(const NV12BufferInterface& src,
                        int offset_x,
                        int offset_y,
                        int crop_width,
                        int crop_height);

 protected:
  NV12Buffer(int width, int height);
  NV12Buffer(int width, int height, int stride_y, int stride_uv);

  ~NV12Buffer() override;

 private:
  size_t UVOffset() const;

  const int width_;
  const int height_;
  const int stride_y_;
  const int stride_uv_;
  const std::unique_ptr<uint8_t, AlignedFreeDeleter> data_;
};

}  // namespace webrtc

#endif  // API_VIDEO_NV12_BUFFER_H_
