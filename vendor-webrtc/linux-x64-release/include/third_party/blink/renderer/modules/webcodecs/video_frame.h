// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_H_

#include <stdint.h>

#include <optional>

#include "base/feature_list.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_pixel_format.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_image_source.h"
#include "third_party/blink/renderer/core/imagebitmap/image_bitmap_source.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/canvas_image_source_util.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/array_buffer_util.h"
#include "third_party/blink/renderer/modules/webcodecs/video_frame_handle.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/predefined_color_space.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/skia/include/core/SkImage.h"
#include "third_party/skia/include/core/SkRefCnt.h"

// Note: Don't include "media/base/video_frame.h" here without good reason,
// since it includes a lot of non-blink types which can pollute the namespace.

namespace media {
class VideoFrame;
}

namespace blink {

class CanvasImageSource;
class DOMRectReadOnly;
class ExceptionState;
class ExecutionContext;
class PlaneLayout;
class ScriptState;
class VideoColorSpace;
class VideoFrameBufferInit;
class VideoFrameCopyToOptions;
class VideoFrameInit;
class VideoFrameLayout;
class VideoFrameMetadata;

class MODULES_EXPORT VideoFrame final : public ScriptWrappable,
                                        public CanvasImageSource,
                                        public ImageBitmapSource {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Creates a VideoFrame with a new VideoFrameHandle wrapping |frame|, and
  // monitored using |monitoring_source_id|.
  VideoFrame(scoped_refptr<media::VideoFrame> frame,
             ExecutionContext*,
             std::string monitoring_source_id = std::string(),
             sk_sp<SkImage> sk_image = nullptr,
             bool use_capture_timestamp = false);

  // Creates a VideoFrame from an existing handle.
  // All frames sharing |handle| will have their |handle_| invalidated if any of
  // the frames receives a call to close().
  explicit VideoFrame(scoped_refptr<VideoFrameHandle> handle);

  ~VideoFrame() override;

  // video_frame.idl implementation.
  static VideoFrame* Create(ScriptState* script_state,
                            const V8CanvasImageSource* source,
                            const VideoFrameInit* init,
                            ExceptionState& exception_state);
  static VideoFrame* Create(ScriptState*,
                            const AllowSharedBufferSource*,
                            const VideoFrameBufferInit*,
                            ExceptionState&);

  std::optional<V8VideoPixelFormat> format() const;

  int64_t timestamp() const;
  std::optional<uint64_t> duration() const;

  uint32_t codedWidth() const;
  uint32_t codedHeight() const;

  DOMRectReadOnly* codedRect();
  DOMRectReadOnly* visibleRect();

  uint32_t rotation() const;
  bool flip() const;

  uint32_t displayWidth() const;
  uint32_t displayHeight() const;

  VideoColorSpace* colorSpace();

  VideoFrameMetadata* metadata(ExceptionState&);

  uint32_t allocationSize(VideoFrameCopyToOptions* options, ExceptionState&);

  ScriptPromise<IDLSequence<PlaneLayout>> copyTo(
      ScriptState* script_state,
      const AllowSharedBufferSource* destination,
      VideoFrameCopyToOptions* options,
      ExceptionState& exception_state);

  // Invalidates |handle_|, releasing underlying media::VideoFrame references.
  // This effectively "destroys" all frames sharing the same Handle.
  void close();

  // Creates a clone of |this|, with a new Handle, referencing the same
  // media::VideoFrame. The cloned frame will not be closed when |this| is,
  // and its lifetime should be independently managed.
  VideoFrame* clone(ExceptionState&);

  // Convenience functions
  scoped_refptr<VideoFrameHandle> handle() const { return handle_; }
  scoped_refptr<media::VideoFrame> frame() const { return handle_->frame(); }

  bool WouldTaintOrigin() const override;

  // GarbageCollected override
  void Trace(Visitor*) const override;

 private:
  // CanvasImageSource implementation
  scoped_refptr<Image> GetSourceImageForCanvas(FlushReason,
                                               SourceImageStatus*,
                                               const gfx::SizeF&) override;

  gfx::SizeF ElementSize(const gfx::SizeF&,
                         const RespectImageOrientationEnum) const override;
  bool IsVideoFrame() const override;
  bool IsOpaque() const override;
  bool IsAccelerated() const override;

  void ConvertAndCopyToRGB(scoped_refptr<media::VideoFrame> frame,
                           const gfx::Rect& src_rect,
                           const VideoFrameLayout& dest_layout,
                           base::span<uint8_t> buffer,
                           PredefinedColorSpace target_color_space);

  bool CopyToAsync(ScriptPromiseResolver<IDLSequence<PlaneLayout>>*,
                   scoped_refptr<media::VideoFrame> frame,
                   gfx::Rect src_rect,
                   const AllowSharedBufferSource* destination,
                   const VideoFrameLayout& dest_layout);

  // ImageBitmapSource implementation
  static constexpr uint64_t kCpuEfficientFrameSize = 320u * 240u;
  ImageBitmapSourceStatus CheckUsability() const override;
  ScriptPromise<ImageBitmap> CreateImageBitmap(
      ScriptState*,
      std::optional<gfx::Rect> crop_rect,
      const ImageBitmapOptions*,
      ExceptionState&) override;

  // Underlying frame
  scoped_refptr<VideoFrameHandle> handle_;

  // Caches
  Member<DOMRectReadOnly> coded_rect_;
  Member<DOMRectReadOnly> visible_rect_;
  Member<VideoColorSpace> color_space_;
  Member<VideoColorSpace> empty_color_space_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_H_
