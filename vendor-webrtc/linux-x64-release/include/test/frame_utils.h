/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_FRAME_UTILS_H_
#define TEST_FRAME_UTILS_H_

#include <stdint.h>

#include "api/scoped_refptr.h"
#include "api/video/nv12_buffer.h"

namespace webrtc {
class I420Buffer;
class VideoFrame;
class VideoFrameBuffer;
namespace test {

bool EqualPlane(const uint8_t* data1,
                const uint8_t* data2,
                int stride1,
                int stride2,
                int width,
                int height);

static inline bool EqualPlane(const uint8_t* data1,
                              const uint8_t* data2,
                              int stride,
                              int width,
                              int height) {
  return EqualPlane(data1, data2, stride, stride, width, height);
}

bool FramesEqual(const webrtc::VideoFrame& f1, const webrtc::VideoFrame& f2);

bool FrameBufsEqual(const scoped_refptr<webrtc::VideoFrameBuffer>& f1,
                    const scoped_refptr<webrtc::VideoFrameBuffer>& f2);

scoped_refptr<I420Buffer> ReadI420Buffer(int width, int height, FILE*);

scoped_refptr<NV12Buffer> ReadNV12Buffer(int width, int height, FILE*);

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FRAME_UTILS_H_
