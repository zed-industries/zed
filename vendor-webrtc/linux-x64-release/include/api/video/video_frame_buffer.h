/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_VIDEO_FRAME_BUFFER_H_
#define API_VIDEO_VIDEO_FRAME_BUFFER_H_

#include <cstdint>
#include <string>

#include "api/array_view.h"
#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class I420BufferInterface;
class I420ABufferInterface;
class I422BufferInterface;
class I444BufferInterface;
class I010BufferInterface;
class I210BufferInterface;
class I410BufferInterface;
class NV12BufferInterface;

// Base class for frame buffers of different types of pixel format and storage.
// The tag in type() indicates how the data is represented, and each type is
// implemented as a subclass. To access the pixel data, call the appropriate
// GetXXX() function, where XXX represents the type. There is also a function
// ToI420() that returns a frame buffer in I420 format, converting from the
// underlying representation if necessary. I420 is the most widely accepted
// format and serves as a fallback for video sinks that can only handle I420,
// e.g. the internal WebRTC software encoders. A special enum value 'kNative' is
// provided for external clients to implement their own frame buffer
// representations, e.g. as textures. The external client can produce such
// native frame buffers from custom video sources, and then cast it back to the
// correct subclass in custom video sinks. The purpose of this is to improve
// performance by providing an optimized path without intermediate conversions.
// Frame metadata such as rotation and timestamp are stored in
// webrtc::VideoFrame, and not here.
class RTC_EXPORT VideoFrameBuffer : public webrtc::RefCountInterface {
 public:
  // New frame buffer types will be added conservatively when there is an
  // opportunity to optimize the path between some pair of video source and
  // video sink.
  // GENERATED_JAVA_ENUM_PACKAGE: org.webrtc
  // GENERATED_JAVA_CLASS_NAME_OVERRIDE: VideoFrameBufferType
  enum class Type {
    kNative,
    kI420,
    kI420A,
    kI422,
    kI444,
    kI010,
    kI210,
    kI410,
    kNV12,
  };

  // This function specifies in what pixel format the data is stored in.
  virtual Type type() const = 0;

  // The resolution of the frame in pixels. For formats where some planes are
  // subsampled, this is the highest-resolution plane.
  virtual int width() const = 0;
  virtual int height() const = 0;

  // Returns a memory-backed frame buffer in I420 format. If the pixel data is
  // in another format, a conversion will take place. All implementations must
  // provide a fallback to I420 for compatibility with e.g. the internal WebRTC
  // software encoders.
  // Conversion may fail, for example if reading the pixel data from a texture
  // fails. If the conversion fails, nullptr is returned.
  virtual scoped_refptr<I420BufferInterface> ToI420() = 0;

  // GetI420() methods should return I420 buffer if conversion is trivial, i.e
  // no change for binary data is needed. Otherwise these methods should return
  // nullptr. One example of buffer with that property is
  // WebrtcVideoFrameAdapter in Chrome - it's I420 buffer backed by a shared
  // memory buffer. Therefore it must have type kNative. Yet, ToI420()
  // doesn't affect binary data at all. Another example is any I420A buffer.
  // TODO(https://crbug.com/webrtc/12021): Make this method non-virtual and
  // behave as the other GetXXX methods below.
  virtual const I420BufferInterface* GetI420() const;

  // A format specific scale function. Default implementation works by
  // converting to I420. But more efficient implementations may override it,
  // especially for kNative.
  // First, the image is cropped to `crop_width` and `crop_height` and then
  // scaled to `scaled_width` and `scaled_height`.
  virtual scoped_refptr<VideoFrameBuffer> CropAndScale(int offset_x,
                                                       int offset_y,
                                                       int crop_width,
                                                       int crop_height,
                                                       int scaled_width,
                                                       int scaled_height);

  // Alias for common use case.
  scoped_refptr<VideoFrameBuffer> Scale(int scaled_width, int scaled_height) {
    return CropAndScale(0, 0, width(), height(), scaled_width, scaled_height);
  }

  // These functions should only be called if type() is of the correct type.
  // Calling with a different type will result in a crash.
  const I420ABufferInterface* GetI420A() const;
  const I422BufferInterface* GetI422() const;
  const I444BufferInterface* GetI444() const;
  const I010BufferInterface* GetI010() const;
  const I210BufferInterface* GetI210() const;
  const I410BufferInterface* GetI410() const;
  const NV12BufferInterface* GetNV12() const;

  // From a kNative frame, returns a VideoFrameBuffer with a pixel format in
  // the list of types that is in the main memory with a pixel perfect
  // conversion for encoding with a software encoder. Returns nullptr if the
  // frame type is not supported, mapping is not possible, or if the kNative
  // frame has not implemented this method. Only callable if type() is kNative.
  virtual scoped_refptr<VideoFrameBuffer> GetMappedFrameBuffer(
      ArrayView<Type> types);

  // For logging: returns a textual representation of the storage.
  virtual std::string storage_representation() const;

 protected:
  ~VideoFrameBuffer() override {}
};

// Update when VideoFrameBuffer::Type is updated.
const char* VideoFrameBufferTypeToString(VideoFrameBuffer::Type type);

// This interface represents planar formats.
class PlanarYuvBuffer : public VideoFrameBuffer {
 public:
  virtual int ChromaWidth() const = 0;
  virtual int ChromaHeight() const = 0;

  // Returns the number of steps(in terms of Data*() return type) between
  // successive rows for a given plane.
  virtual int StrideY() const = 0;
  virtual int StrideU() const = 0;
  virtual int StrideV() const = 0;

 protected:
  ~PlanarYuvBuffer() override {}
};

// This interface represents 8-bit color depth formats: Type::kI420,
// Type::kI420A, Type::kI422 and Type::kI444.
class PlanarYuv8Buffer : public PlanarYuvBuffer {
 public:
  // Returns pointer to the pixel data for a given plane. The memory is owned by
  // the VideoFrameBuffer object and must not be freed by the caller.
  virtual const uint8_t* DataY() const = 0;
  virtual const uint8_t* DataU() const = 0;
  virtual const uint8_t* DataV() const = 0;

 protected:
  ~PlanarYuv8Buffer() override {}
};

class RTC_EXPORT I420BufferInterface : public PlanarYuv8Buffer {
 public:
  Type type() const override;

  int ChromaWidth() const final;
  int ChromaHeight() const final;

  scoped_refptr<I420BufferInterface> ToI420() final;
  const I420BufferInterface* GetI420() const final;

 protected:
  ~I420BufferInterface() override {}
};

class RTC_EXPORT I420ABufferInterface : public I420BufferInterface {
 public:
  Type type() const final;
  virtual const uint8_t* DataA() const = 0;
  virtual int StrideA() const = 0;

 protected:
  ~I420ABufferInterface() override {}
};

// Represents Type::kI422, 4:2:2 planar with 8 bits per pixel.
class I422BufferInterface : public PlanarYuv8Buffer {
 public:
  Type type() const final;

  int ChromaWidth() const final;
  int ChromaHeight() const final;

  scoped_refptr<VideoFrameBuffer> CropAndScale(int offset_x,
                                               int offset_y,
                                               int crop_width,
                                               int crop_height,
                                               int scaled_width,
                                               int scaled_height) override;

 protected:
  ~I422BufferInterface() override {}
};

// Represents Type::kI444, 4:4:4 planar with 8 bits per pixel.
class I444BufferInterface : public PlanarYuv8Buffer {
 public:
  Type type() const final;

  int ChromaWidth() const final;
  int ChromaHeight() const final;

  scoped_refptr<VideoFrameBuffer> CropAndScale(int offset_x,
                                               int offset_y,
                                               int crop_width,
                                               int crop_height,
                                               int scaled_width,
                                               int scaled_height) override;

 protected:
  ~I444BufferInterface() override {}
};

// This interface represents 8-bit to 16-bit color depth formats: Type::kI010,
// Type::kI210, or Type::kI410.
class PlanarYuv16BBuffer : public PlanarYuvBuffer {
 public:
  // Returns pointer to the pixel data for a given plane. The memory is owned by
  // the VideoFrameBuffer object and must not be freed by the caller.
  virtual const uint16_t* DataY() const = 0;
  virtual const uint16_t* DataU() const = 0;
  virtual const uint16_t* DataV() const = 0;

 protected:
  ~PlanarYuv16BBuffer() override {}
};

// Represents Type::kI010, allocates 16 bits per pixel and fills 10 least
// significant bits with color information.
class I010BufferInterface : public PlanarYuv16BBuffer {
 public:
  Type type() const override;

  int ChromaWidth() const final;
  int ChromaHeight() const final;

 protected:
  ~I010BufferInterface() override {}
};

// Represents Type::kI210, allocates 16 bits per pixel and fills 10 least
// significant bits with color information.
class I210BufferInterface : public PlanarYuv16BBuffer {
 public:
  Type type() const override;

  int ChromaWidth() const final;
  int ChromaHeight() const final;

 protected:
  ~I210BufferInterface() override {}
};

// Represents Type::kI410, allocates 16 bits per pixel and fills 10 least
// significant bits with color information.
class I410BufferInterface : public PlanarYuv16BBuffer {
 public:
  Type type() const override;

  int ChromaWidth() const final;
  int ChromaHeight() const final;

 protected:
  ~I410BufferInterface() override {}
};

class BiplanarYuvBuffer : public VideoFrameBuffer {
 public:
  virtual int ChromaWidth() const = 0;
  virtual int ChromaHeight() const = 0;

  // Returns the number of steps(in terms of Data*() return type) between
  // successive rows for a given plane.
  virtual int StrideY() const = 0;
  virtual int StrideUV() const = 0;

 protected:
  ~BiplanarYuvBuffer() override {}
};

class BiplanarYuv8Buffer : public BiplanarYuvBuffer {
 public:
  virtual const uint8_t* DataY() const = 0;
  virtual const uint8_t* DataUV() const = 0;

 protected:
  ~BiplanarYuv8Buffer() override {}
};

// Represents Type::kNV12. NV12 is full resolution Y and half-resolution
// interleved UV.
class RTC_EXPORT NV12BufferInterface : public BiplanarYuv8Buffer {
 public:
  Type type() const override;

  int ChromaWidth() const final;
  int ChromaHeight() const final;

  scoped_refptr<VideoFrameBuffer> CropAndScale(int offset_x,
                                               int offset_y,
                                               int crop_width,
                                               int crop_height,
                                               int scaled_width,
                                               int scaled_height) override;

 protected:
  ~NV12BufferInterface() override {}
};

// RTC_CHECKs that common values used to calculate buffer sizes are within the
// range of [1..std::numeric_limits<int>::max()].
// `width` and `height` must be > 0, `stride_y` must be >= `width` whereas
// `stride_u` and `stride_v` must be `> 0` as this is where the various yuv
// formats differ.
void CheckValidDimensions(int width,
                          int height,
                          int stride_y,
                          int stride_u,
                          int stride_v);

}  // namespace webrtc

#endif  // API_VIDEO_VIDEO_FRAME_BUFFER_H_
