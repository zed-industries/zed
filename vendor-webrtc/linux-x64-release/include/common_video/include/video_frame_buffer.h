/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_INCLUDE_VIDEO_FRAME_BUFFER_H_
#define COMMON_VIDEO_INCLUDE_VIDEO_FRAME_BUFFER_H_

#include <stdint.h>

#include <functional>

#include "api/scoped_refptr.h"
#include "api/video/video_frame_buffer.h"

namespace webrtc {

scoped_refptr<I420BufferInterface> WrapI420Buffer(
    int width,
    int height,
    const uint8_t* y_plane,
    int y_stride,
    const uint8_t* u_plane,
    int u_stride,
    const uint8_t* v_plane,
    int v_stride,
    std::function<void()> no_longer_used);

scoped_refptr<I422BufferInterface> WrapI422Buffer(
    int width,
    int height,
    const uint8_t* y_plane,
    int y_stride,
    const uint8_t* u_plane,
    int u_stride,
    const uint8_t* v_plane,
    int v_stride,
    std::function<void()> no_longer_used);

scoped_refptr<I444BufferInterface> WrapI444Buffer(
    int width,
    int height,
    const uint8_t* y_plane,
    int y_stride,
    const uint8_t* u_plane,
    int u_stride,
    const uint8_t* v_plane,
    int v_stride,
    std::function<void()> no_longer_used);

scoped_refptr<I420ABufferInterface> WrapI420ABuffer(
    int width,
    int height,
    const uint8_t* y_plane,
    int y_stride,
    const uint8_t* u_plane,
    int u_stride,
    const uint8_t* v_plane,
    int v_stride,
    const uint8_t* a_plane,
    int a_stride,
    std::function<void()> no_longer_used);

scoped_refptr<PlanarYuvBuffer> WrapYuvBuffer(
    VideoFrameBuffer::Type type,
    int width,
    int height,
    const uint8_t* y_plane,
    int y_stride,
    const uint8_t* u_plane,
    int u_stride,
    const uint8_t* v_plane,
    int v_stride,
    std::function<void()> no_longer_used);

scoped_refptr<I010BufferInterface> WrapI010Buffer(
    int width,
    int height,
    const uint16_t* y_plane,
    int y_stride,
    const uint16_t* u_plane,
    int u_stride,
    const uint16_t* v_plane,
    int v_stride,
    std::function<void()> no_longer_used);

scoped_refptr<I210BufferInterface> WrapI210Buffer(
    int width,
    int height,
    const uint16_t* y_plane,
    int y_stride,
    const uint16_t* u_plane,
    int u_stride,
    const uint16_t* v_plane,
    int v_stride,
    std::function<void()> no_longer_used);

scoped_refptr<I410BufferInterface> WrapI410Buffer(
    int width,
    int height,
    const uint16_t* y_plane,
    int y_stride,
    const uint16_t* u_plane,
    int u_stride,
    const uint16_t* v_plane,
    int v_stride,
    std::function<void()> no_longer_used);
}  // namespace webrtc

#endif  // COMMON_VIDEO_INCLUDE_VIDEO_FRAME_BUFFER_H_
