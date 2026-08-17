/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_INCLUDE_VIDEO_FRAME_BUFFER_POOL_H_
#define COMMON_VIDEO_INCLUDE_VIDEO_FRAME_BUFFER_POOL_H_

#include <stddef.h>

#include <list>

#include "api/scoped_refptr.h"
#include "api/video/i010_buffer.h"
#include "api/video/i210_buffer.h"
#include "api/video/i410_buffer.h"
#include "api/video/i420_buffer.h"
#include "api/video/i422_buffer.h"
#include "api/video/i444_buffer.h"
#include "api/video/nv12_buffer.h"
#include "rtc_base/race_checker.h"

namespace webrtc {

// Simple buffer pool to avoid unnecessary allocations of video frame buffers.
// The pool manages the memory of the I420Buffer/NV12Buffer returned from
// Create(I420|NV12)Buffer. When the buffer is destructed, the memory is
// returned to the pool for use by subsequent calls to Create(I420|NV12)Buffer.
// If the resolution passed to Create(I420|NV12)Buffer changes or requested
// pixel format changes, old buffers will be purged from the pool.
// Note that Create(I420|NV12)Buffer will crash if more than
// kMaxNumberOfFramesBeforeCrash are created. This is to prevent memory leaks
// where frames are not returned.
class VideoFrameBufferPool {
 public:
  VideoFrameBufferPool();
  explicit VideoFrameBufferPool(bool zero_initialize);
  VideoFrameBufferPool(bool zero_initialize, size_t max_number_of_buffers);
  ~VideoFrameBufferPool();

  // Returns a buffer from the pool. If no suitable buffer exist in the pool
  // and there are less than `max_number_of_buffers` pending, a buffer is
  // created. Returns null otherwise.
  scoped_refptr<I420Buffer> CreateI420Buffer(int width, int height);
  scoped_refptr<I422Buffer> CreateI422Buffer(int width, int height);
  scoped_refptr<I444Buffer> CreateI444Buffer(int width, int height);
  scoped_refptr<I010Buffer> CreateI010Buffer(int width, int height);
  scoped_refptr<I210Buffer> CreateI210Buffer(int width, int height);
  scoped_refptr<I410Buffer> CreateI410Buffer(int width, int height);
  scoped_refptr<NV12Buffer> CreateNV12Buffer(int width, int height);

  // Changes the max amount of buffers in the pool to the new value.
  // Returns true if change was successful and false if the amount of already
  // allocated buffers is bigger than new value.
  bool Resize(size_t max_number_of_buffers);

  // Clears buffers_ and detaches the thread checker so that it can be reused
  // later from another thread.
  void Release();

 private:
  scoped_refptr<VideoFrameBuffer>
  GetExistingBuffer(int width, int height, VideoFrameBuffer::Type type);

  RaceChecker race_checker_;
  std::list<scoped_refptr<VideoFrameBuffer>> buffers_;
  // If true, newly allocated buffers are zero-initialized. Note that recycled
  // buffers are not zero'd before reuse. This is required of buffers used by
  // FFmpeg according to http://crbug.com/390941, which only requires it for the
  // initial allocation (as shown by FFmpeg's own buffer allocation code). It
  // has to do with "Use-of-uninitialized-value" on "Linux_msan_chrome".
  const bool zero_initialize_;
  // Max number of buffers this pool can have pending.
  size_t max_number_of_buffers_;
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_INCLUDE_VIDEO_FRAME_BUFFER_POOL_H_
