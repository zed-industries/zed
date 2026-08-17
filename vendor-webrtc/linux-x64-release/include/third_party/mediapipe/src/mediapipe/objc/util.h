// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_OBJC_UTIL_H_
#define MEDIAPIPE_OBJC_UTIL_H_

#import <Accelerate/Accelerate.h>
#import <CoreFoundation/CoreFoundation.h>
#import <CoreGraphics/CoreGraphics.h>
#import <CoreVideo/CoreVideo.h>

#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/objc/CFHolder.h"

// TODO: namespace and/or prefix these. Split up the file.

/// Returns a vImage_Buffer describing the data inside the pixel_buffer.
/// NOTE: the pixel buffer's base address must have been locked before this
/// call, and it must stay locked as long as the vImage_Buffer is in use.
inline vImage_Buffer vImageForCVPixelBuffer(CVPixelBufferRef pixel_buffer) {
  return {.data = CVPixelBufferGetBaseAddress(pixel_buffer),
          .height = CVPixelBufferGetHeight(pixel_buffer),
          .width = CVPixelBufferGetWidth(pixel_buffer),
          .rowBytes = CVPixelBufferGetBytesPerRow(pixel_buffer)};
}

/// Returns a vImage_Buffer describing the data inside the ImageFrame.
inline vImage_Buffer vImageForImageFrame(const mediapipe::ImageFrame& frame) {
  return {
      .data = reinterpret_cast<void*>(const_cast<uint8_t*>(frame.PixelData())),
      .height = static_cast<vImagePixelCount>(frame.Height()),
      .width = static_cast<vImagePixelCount>(frame.Width()),
      .rowBytes = static_cast<size_t>(frame.WidthStep())};
}

/// Converts a grayscale image without alpha to BGRA format.
vImage_Error vImageGrayToBGRA(const vImage_Buffer* src, vImage_Buffer* dst);

/// Converts a BGRA image to grayscale without alpha.
vImage_Error vImageBGRAToGray(const vImage_Buffer* src, vImage_Buffer* dst);

/// Converts an RGBA image to grayscale without alpha.
vImage_Error vImageRGBAToGray(const vImage_Buffer* src, vImage_Buffer* dst);

/// Copy from a pixel buffer to another, converting pixel format.
/// Both pixel buffers should be locked before calling this.
vImage_Error vImageConvertCVPixelBuffers(CVPixelBufferRef src,
                                         CVPixelBufferRef dst);

// Create a CVPixelBuffer without using a pool. See pixel_buffer_pool_util.h
// for creation functions that use pools.
CVReturn CreateCVPixelBufferWithoutPool(int width, int height, OSType cv_format,
                                        CVPixelBufferRef* out_buffer);
absl::StatusOr<CFHolder<CVPixelBufferRef>> CreateCVPixelBufferWithoutPool(
    int width, int height, OSType cv_format);

/// Returns a CVPixelBuffer that references the data inside the packet. The
/// packet must contain an ImageFrame. The CVPixelBuffer manages a copy of
/// the packet, so that the packet's data is kept alive as long as the
/// CVPixelBuffer is in use.
///
/// For formats which are not supported by both image types, it may be
/// necessary to convert the data. This is done by creating a new buffer.
/// If the optional can_overwrite parameter is true, the old buffer may be
/// modified instead.
absl::Status CreateCVPixelBufferForImageFramePacket(
    const mediapipe::Packet& image_frame_packet,
    CFHolder<CVPixelBufferRef>* out_buffer);
absl::Status CreateCVPixelBufferForImageFramePacket(
    const mediapipe::Packet& image_frame_packet, bool can_overwrite,
    CFHolder<CVPixelBufferRef>* out_buffer);
absl::StatusOr<CFHolder<CVPixelBufferRef>> CreateCVPixelBufferCopyingImageFrame(
    const mediapipe::ImageFrame& image_frame);
absl::StatusOr<CFHolder<CVPixelBufferRef>> CreateCVPixelBufferForImageFrame(
    std::shared_ptr<mediapipe::ImageFrame> image_frame,
    bool can_overwrite = false);

/// Creates a CVPixelBuffer with a copy of the contents of the CGImage.
absl::Status CreateCVPixelBufferFromCGImage(
    CGImageRef image, CFHolder<CVPixelBufferRef>* out_buffer);

/// Creates a CGImage with a copy of the contents of the CVPixelBuffer.
absl::Status CreateCGImageFromCVPixelBuffer(CVPixelBufferRef image_buffer,
                                            CFHolder<CGImageRef>* image);

/// DEPRECATED: use the version that returns absl::Status instead.
CVPixelBufferRef CreateCVPixelBufferForImageFramePacket(
    const mediapipe::Packet& image_frame_packet);

/// Returns an ImageFrame that references the data inside the pixel_buffer.
/// The ImageFrame retains the pixel_buffer and keeps it locked as long as it
/// is in use.
///
/// For formats which are not supported by both image types, it may be
/// necessary to convert the data. This is done by creating a new buffer.
/// If the optional can_overwrite parameter is true, the old buffer may be
/// modified instead.
///
/// ImageFrame does not have a format for BGRA data, so we normally swap the
/// channels to produce RGBA. But many graphs do not care about the order of
/// the channels; in those cases, setting the optional bgr_as_rgb parameter
/// to true skips the channel swap.
std::unique_ptr<mediapipe::ImageFrame> CreateImageFrameForCVPixelBuffer(
    CVPixelBufferRef pixel_buffer);
std::unique_ptr<mediapipe::ImageFrame> CreateImageFrameForCVPixelBuffer(
    CVPixelBufferRef pixel_buffer, bool can_overwrite, bool bgr_as_rgb);

/// Returns a CFDictionaryRef that can be passed to CVPixelBufferCreate to
/// ensure that the pixel buffer is compatible with OpenGL ES and with
/// CVOpenGLESTextureCacheCreateTextureFromImage.
/// The returned object is persistent and should not be released.
CFDictionaryRef GetCVPixelBufferAttributesForGlCompatibility();

/// Prints debug information about available CoreVideo pixel formats.
/// This prints to stdout.
void DumpCVPixelFormats();

#endif  // MEDIAPIPE_OBJC_UTIL_H_
