/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef MODULES_VIDEO_CODING_CODECS_VP9_VP9_FRAME_BUFFER_POOL_H_
#define MODULES_VIDEO_CODING_CODECS_VP9_VP9_FRAME_BUFFER_POOL_H_

#ifdef RTC_ENABLE_VP9

#include <cstddef>
#include <cstdint>
#include <vector>

#include "api/ref_counted_base.h"
#include "api/scoped_refptr.h"
#include "rtc_base/buffer.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

struct vpx_codec_ctx;
struct vpx_codec_frame_buffer;

namespace webrtc {

// If more buffers than this are allocated we print warnings and crash if in
// debug mode. VP9 is defined to have 8 reference buffers, of which 3 can be
// referenced by any frame, see
// https://tools.ietf.org/html/draft-grange-vp9-bitstream-00#section-2.2.2.
// Assuming VP9 holds on to at most 8 buffers, any more buffers than that
// would have to be by application code. Decoded frames should not be
// referenced for longer than necessary. If we allow ~60 additional buffers
// then the application has ~1 second to e.g. render each frame of a 60 fps
// video.
constexpr size_t kDefaultMaxNumBuffers = 68;

// This memory pool is used to serve buffers to libvpx for decoding purposes in
// VP9, which is set up in InitializeVPXUsePool. After the initialization any
// time libvpx wants to decode a frame it will use buffers provided and released
// through VpxGetFrameBuffer and VpxReleaseFrameBuffer.
// The benefit of owning the pool that libvpx relies on for decoding is that the
// decoded frames returned by libvpx (from vpx_codec_get_frame) use parts of our
// buffers for the decoded image data. By retaining ownership of this buffer
// using scoped_refptr, the image buffer can be reused by VideoFrames and no
// frame copy has to occur during decoding and frame delivery.
//
// Pseudo example usage case:
//    Vp9FrameBufferPool pool;
//    pool.InitializeVpxUsePool(decoder_ctx);
//    ...
//
//    // During decoding, libvpx will get and release buffers from the pool.
//    vpx_codec_decode(decoder_ctx, ...);
//
//    vpx_image_t* img = vpx_codec_get_frame(decoder_ctx, &iter);
//    // Important to use scoped_refptr to protect it against being recycled by
//    // the pool.
//    scoped_refptr<Vp9FrameBuffer> img_buffer = (Vp9FrameBuffer*)img->fb_priv;
//    ...
//
//    // Destroying the codec will make libvpx release any buffers it was using.
//    vpx_codec_destroy(decoder_ctx);
class Vp9FrameBufferPool {
 public:
  class Vp9FrameBuffer final : public RefCountedNonVirtual<Vp9FrameBuffer> {
   public:
    uint8_t* GetData();
    size_t GetDataSize() const;
    void SetSize(size_t size);

    using RefCountedNonVirtual<Vp9FrameBuffer>::HasOneRef;

   private:
    // Data as an easily resizable buffer.
    Buffer data_;
  };

  // Configures libvpx to, in the specified context, use this memory pool for
  // buffers used to decompress frames. This is only supported for VP9.
  bool InitializeVpxUsePool(vpx_codec_ctx* vpx_codec_context);

  // Gets a frame buffer of at least `min_size`, recycling an available one or
  // creating a new one. When no longer referenced from the outside the buffer
  // becomes recyclable.
  scoped_refptr<Vp9FrameBuffer> GetFrameBuffer(size_t min_size);
  // Gets the number of buffers currently in use (not ready to be recycled).
  int GetNumBuffersInUse() const;
  // Changes the max amount of buffers in the pool to the new value.
  // Returns true if change was successful and false if the amount of already
  // allocated buffers is bigger than new value.
  bool Resize(size_t max_number_of_buffers);
  // Releases allocated buffers, deleting available buffers. Buffers in use are
  // not deleted until they are no longer referenced.
  void ClearPool();

  // InitializeVpxUsePool configures libvpx to call this function when it needs
  // a new frame buffer. Parameters:
  // `user_priv` Private data passed to libvpx, InitializeVpxUsePool sets it up
  //             to be a pointer to the pool.
  // `min_size`  Minimum size needed by libvpx (to decompress a frame).
  // `fb`        Pointer to the libvpx frame buffer object, this is updated to
  //             use the pool's buffer.
  // Returns 0 on success. Returns < 0 on failure.
  static int32_t VpxGetFrameBuffer(void* user_priv,
                                   size_t min_size,
                                   vpx_codec_frame_buffer* fb);

  // InitializeVpxUsePool configures libvpx to call this function when it has
  // finished using one of the pool's frame buffer. Parameters:
  // `user_priv` Private data passed to libvpx, InitializeVpxUsePool sets it up
  //             to be a pointer to the pool.
  // `fb`        Pointer to the libvpx frame buffer object, its `priv` will be
  //             a pointer to one of the pool's Vp9FrameBuffer.
  static int32_t VpxReleaseFrameBuffer(void* user_priv,
                                       vpx_codec_frame_buffer* fb);

 private:
  // Protects `allocated_buffers_`.
  mutable Mutex buffers_lock_;
  // All buffers, in use or ready to be recycled.
  std::vector<scoped_refptr<Vp9FrameBuffer>> allocated_buffers_
      RTC_GUARDED_BY(buffers_lock_);
  size_t max_num_buffers_ = kDefaultMaxNumBuffers;
};

}  // namespace webrtc

#endif  // RTC_ENABLE_VP9

#endif  // MODULES_VIDEO_CODING_CODECS_VP9_VP9_FRAME_BUFFER_POOL_H_
