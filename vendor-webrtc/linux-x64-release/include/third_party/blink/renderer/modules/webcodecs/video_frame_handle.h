// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_HANDLE_H_

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/webcodecs_logger.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/skia/include/core/SkRefCnt.h"

// Note: Don't include "media/base/video_frame.h" here without good reason,
// since it includes a lot of non-blink types which can pollute the namespace.

class SkImage;

namespace media {
class VideoFrame;
}

namespace blink {

class ExecutionContext;

// Wrapper class that allows sharing a single |frame_| reference across
// multiple VideoFrames, which can be invalidated for all frames at once.
//
// If Invalidate() is not called before the handle's destructor runs, this means
// that none of the VideoFrames sharing this handle were closed, and they were
// all GC'ed instead. This can lead to stalls, since frames are not released
// fast enough through the GC to keep a pipeline running smoothly. In that case
// report an unclosed frame through |close_auditor_|.
class MODULES_EXPORT VideoFrameHandle
    : public WTF::ThreadSafeRefCounted<VideoFrameHandle> {
 public:
  VideoFrameHandle(scoped_refptr<media::VideoFrame>,
                   ExecutionContext*,
                   std::string monitoring_source_id = std::string(),
                   bool use_capture_timestamp = false);
  VideoFrameHandle(scoped_refptr<media::VideoFrame>,
                   sk_sp<SkImage> sk_image,
                   ExecutionContext*,
                   std::string monitoring_source_id = std::string(),
                   bool use_capture_timestamp = false);
  VideoFrameHandle(scoped_refptr<media::VideoFrame>,
                   sk_sp<SkImage> sk_image,
                   base::TimeDelta timestamp,
                   scoped_refptr<WebCodecsLogger::VideoFrameCloseAuditor>,
                   std::string monitoring_source_id = std::string());
  VideoFrameHandle(scoped_refptr<media::VideoFrame>,
                   sk_sp<SkImage> sk_image,
                   base::TimeDelta timestamp,
                   std::string monitoring_source_id = std::string());
  VideoFrameHandle(scoped_refptr<media::VideoFrame>,
                   sk_sp<SkImage> sk_image,
                   std::string monitoring_source_id = std::string());
  VideoFrameHandle(scoped_refptr<media::VideoFrame>,
                   sk_sp<SkImage> sk_image,
                   scoped_refptr<WebCodecsLogger::VideoFrameCloseAuditor>,
                   std::string monitoring_source_id = std::string());

  VideoFrameHandle(const VideoFrameHandle&) = delete;
  VideoFrameHandle& operator=(const VideoFrameHandle&) = delete;

  // Expire all GPUExternalTextures which are generated from
  // importExternalTexture and using VideoFrame as source. This callback should
  // be called when VideoFrameHandle is destroyed/Invalidate.
  using WebGPUExternalTextureExpireCallback = CrossThreadOnceFunction<void()>;

  // Returns a copy of |frame_|, which should be re-used throughout the scope
  // of a function call, instead of calling frame() multiple times. Otherwise
  // the frame could be destroyed between calls.
  scoped_refptr<media::VideoFrame> frame();

  // Returns a copy of |sk_image_| which may be nullptr if this isn't an SkImage
  // backed VideoFrame.
  sk_sp<SkImage> sk_image();

  // Releases the underlying media::VideoFrame reference, affecting all
  // blink::VideoFrames and blink::Planes that hold a reference to |this|.
  void Invalidate();

  // Temporary workaround for crbug.com/1182497.
  // Overrides the next call to Clone() to return |this| without adding a new
  // reference. This prevents cloning handles when internally posting frames to
  // a transferred stream, which cannot be cleared.
  void SetCloseOnClone();

  // Clones this VideoFrameHandle into a new VideoFrameHandle object.
  scoped_refptr<VideoFrameHandle> Clone();

  // Same as above, but for internal (non-garbage-collected) usage. Drops the
  // close auditor so warning messages aren't created for unclosed frames.
  scoped_refptr<VideoFrameHandle> CloneForInternalUse();

  // GPUExternalTexture generated with VideoFrame source needs to listen to
  // the VideoFrameHandle to expire themselves. Return false if the
  // VideoFrameHandle has been destroyed.
  bool WebGPURegisterExternalTextureExpireCallback(
      WebGPUExternalTextureExpireCallback
          webgpu_external_texture_expire_callback);

  base::TimeDelta timestamp() const { return timestamp_; }
  std::optional<base::TimeDelta> duration() const { return duration_; }

 private:
  friend class WTF::ThreadSafeRefCounted<VideoFrameHandle>;
  ~VideoFrameHandle();

  void InvalidateLocked() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Inform the GPUExternalTexture that VideoFrame is closed and expire the
  // external texture. |mutex_| must be held before calling into this.
  void NotifyExpiredLocked() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  void MaybeMonitorOpenFrame() EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void MaybeMonitorCloseFrame() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Flag that prevents the creation of a new handle during the next Clone()
  // call. Used as a temporary workaround for crbug.com/1182497.
  bool close_on_clone_ GUARDED_BY(lock_) = false;

  base::Lock lock_;
  sk_sp<SkImage> sk_image_ GUARDED_BY(lock_);
  scoped_refptr<media::VideoFrame> frame_ GUARDED_BY(lock_);
  scoped_refptr<WebCodecsLogger::VideoFrameCloseAuditor> close_auditor_;
  Vector<WebGPUExternalTextureExpireCallback>
      webgpu_external_texture_expire_callbacks_;
  std::string monitoring_source_id_;

  // Timestamp and duration must remain valid after close(). If the timestamp or
  // duration of the underlying media::VideoFrame are modified after
  // blink::VideoFrame construction, those updates won't appear here.
  const base::TimeDelta timestamp_;
  const std::optional<base::TimeDelta> duration_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_HANDLE_H_
