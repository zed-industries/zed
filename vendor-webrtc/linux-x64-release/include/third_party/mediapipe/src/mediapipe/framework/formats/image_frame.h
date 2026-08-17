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
//
// Get a cv::Mat view of the ImageFrame (this is efficient):
//   ::mediapipe::formats::MatView(&frame);
//
// Copying data from raw data (stored contiguously):
//   frame.CopyPixelData(format, width, height, raw_data_ptr,
//                       ImageFrame::kDefaultAlignmentBoundary);
//
// Convert an RGB ImageFrame (rgb_frame) to Grayscale:
//   ImageFrame gray_frame(ImageFormat::GRAY8, rgb_frame.Width(),
//                         rgb_frame.Height());
//   cv::Mat rgb_frame_mat = ::mediapipe::formats::MatView(&rgb_frame);
//   cv::Mat gray_frame_mat = ::mediapipe::formats::MatView(&gray_frame);
//   cv::cvtColor(rgb_frame_mat, gray_frame_mat, CV_RGB2GRAY);
//
// Resize an ImageFrame:
//   ImageFrame small_image(ImageFormat::GRAY8, 10, 10);
//   cv::Mat destination = ::mediapipe::formats::MatView(&small_image);
//   cv::resize(::mediapipe::formats::MatView(&large_image), destination,
//              destination.size(), 0, 0, cv::INTER_LINEAR);

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_FRAME_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_FRAME_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <string>

#include "absl/base/attributes.h"
#include "mediapipe/framework/formats/image_format.pb.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/tool/type_util.h"

#define IMAGE_FRAME_RAW_IMAGE MEDIAPIPE_HAS_RTTI

namespace mediapipe {

// A container for storing an image or a video frame, in one of several
// formats.  Pixels are encoded row-major in an interleaved fashion.
//
// Formats supported by ImageFrame are listed in the ImageFormat proto.
// It is the intention of ImageFormat to specify both the data format
// and the colorspace used.  For example GRAY8 and GRAY16 both use the
// same colorspace but have different formats.  Although it would be
// possible to keep HSV, linearRGB, or BGR values inside an ImageFrame
// (with format SRGB) this is an abuse of the class.  If you need a new
// format, please add one to ImageFormat::Format.
//
// Do not assume that the pixel data is stored contiguously.  It may be
// stored with row padding for alignment purposes.
class ImageFrame {
 public:
  typedef std::function<void(uint8_t*)> Deleter;

  // This class offers a few standard delete functions and retains
  // compatibility with the previous API.
  class PixelDataDeleter {
   public:
    static const Deleter kArrayDelete;
    static const Deleter kFree;
    static const Deleter kAlignedFree;
    static const Deleter kNone;
  };

  // Use a default alignment boundary of 16 because Intel SSE2 instructions may
  // incur performance penalty when accessing data not aligned on a 16-byte
  // boundary. FFmpeg requires at least this level of alignment.
  static const uint32_t kDefaultAlignmentBoundary = 16;

  // If the pixel data of an ImageFrame will be passed to an OpenGL function
  // such as glTexImage2D() or glReadPixels(), use a four-byte alignment
  // boundary because that is the initial value of the OpenGL GL_PACK_ALIGNMENT
  // and GL_UNPACK_ALIGNMENT parameters.
  static const uint32_t kGlDefaultAlignmentBoundary = 4;

  // Returns number of channels for the given image format.
  static int NumberOfChannelsForFormat(ImageFormat::Format format);
  // Returns the size of each channel in bytes for the given image format.
  static int ChannelSizeForFormat(ImageFormat::Format format);
  ABSL_DEPRECATED("Use ChannelSizeForFormat() instead")
  static int ByteDepthForFormat(ImageFormat::Format format);

  ImageFrame(const ImageFrame&) = delete;
  ImageFrame& operator=(const ImageFrame&) = delete;
  // Creates an empty ImageFrame. It will need to be initialized by some other
  // means.
  ImageFrame();

  // Allocate a frame of the appropriate size.  Does not zero it out.
  // Each row will be aligned to alignment_boundary.  alignment_boundary
  // must be a power of 2 (the number 1 is valid, and means the data will
  // be stored contiguously).
  ImageFrame(ImageFormat::Format format, int width, int height,
             uint32_t alignment_boundary);
  // Same as above, but use kDefaultAlignmentBoundary for alignment_boundary.
  ImageFrame(ImageFormat::Format format, int width, int height);

  // Acquires ownership of pixel_data.  Sets the deletion method
  // to use on pixel_data with deletion_method (which defaults
  // to using delete[]).  pixel_data must have been allocated of
  // size at least width_step*height and width_step must be at least
  // width*num_channels*depth.  Both width_step and depth are in units
  // of bytes.
  ImageFrame(ImageFormat::Format format, int width, int height, int width_step,
             uint8_t* pixel_data,
             Deleter deleter = std::default_delete<uint8_t[]>());

  ImageFrame(ImageFrame&& move_from);
  ImageFrame& operator=(ImageFrame&& move_from);

  // Returns true if the ImageFrame is unallocated.
  bool IsEmpty() const { return pixel_data_ == nullptr; }

  // Set the entire frame allocation to zero, including alignment
  // padding areas.
  void SetToZero();
  // Set the padding bytes at the end of each row (that are used for
  // alignment) to deterministic values.  This function should be called
  // to get deterministic behavior from functions that read the padding
  // areas (generally as part of highly optimized operations such as
  // those in ffmpeg).
  void SetAlignmentPaddingAreas();

  // Returns true if the data is stored contiguously (without any
  // alignment padding areas).
  bool IsContiguous() const;

  // Returns true if each row of the data is aligned to
  // alignment_boundary.  If IsAligned(16) is true then so are
  // IsAligned(8), IsAligned(4), IsAligned(2), and IsAligned(1).
  // alignment_boundary must be 1 or a power of 2.
  bool IsAligned(uint32_t alignment_boundary) const;

  // Returns the image / video format.
  ImageFormat::Format Format() const { return format_; }
  // Returns the width of the image in pixels.
  int Width() const { return width_; }
  // Returns the height of the image in pixels.
  int Height() const { return height_; }
  // Returns the number of channels.
  int NumberOfChannels() const;
  // Returns the size of each channel in bytes.
  int ChannelSize() const;
  ABSL_DEPRECATED("Use ChannelSize() instead")
  int ByteDepth() const;

  // Returns the byte offset between a pixel value and the same pixel
  // and channel in the next row.  Notice, that for alignment reasons,
  // there may be unused padding bytes at the end of each row
  // (WidthStep() - Width()*NumberOfChannels*ChannelSize() will give the
  // number of unused bytes).
  int WidthStep() const { return width_step_; }

  // Reset the current image frame and copy the data from image_frame into
  // this image frame.  The alignment_boundary must be given (and won't
  // necessarily match the alignment_boundary of the input image_frame).
  void CopyFrom(const ImageFrame& image_frame, uint32_t alignment_boundary);

  // Get a mutable pointer to the underlying image data.  The ImageFrame
  // retains ownership.
  uint8_t* MutablePixelData() { return pixel_data_.get(); }
  // Get a const pointer to the underlying image data.
  const uint8_t* PixelData() const { return pixel_data_.get(); }

  // Returns the total size of the pixel data.
  int PixelDataSize() const { return Height() * WidthStep(); }
  // Returns the total size the pixel data would take if it was stored
  // contiguously (which may not be the case).
  int PixelDataSizeStoredContiguously() const {
    return Width() * Height() * ChannelSize() * NumberOfChannels();
  }

  // Initializes ImageFrame from pixel data without copying.
  // ImageFrame takes ownership of pixel_data.  See the Constructor
  // with the same arguments for details.
  void AdoptPixelData(ImageFormat::Format format, int width, int height,
                      int width_step, uint8_t* pixel_data,
                      Deleter deleter = std::default_delete<uint8_t[]>());

  // Resets the ImageFrame and makes it a copy of the provided pixel
  // data, which is assumed to be stored contiguously.  The ImageFrame
  // will use the given alignment_boundary.
  void CopyPixelData(ImageFormat::Format format, int width, int height,
                     const uint8_t* pixel_data, uint32_t alignment_boundary);

  // Resets the ImageFrame and makes it a copy of the provided pixel
  // data, with given width_step.  The ImageFrame
  // will use the given alignment_boundary.
  void CopyPixelData(ImageFormat::Format format, int width, int height,
                     int width_step, const uint8_t* pixel_data,
                     uint32_t alignment_boundary);

  // Allocates a frame of the specified format, width, height, and alignment,
  // without clearing any current pixel data. See the constructor with the same
  // argument list.
  void Reset(ImageFormat::Format format, int width, int height,
             uint32_t alignment_boundary);

  // Relinquishes ownership of the pixel data.  Notice that the unique_ptr
  // uses a non-standard deleter.
  std::unique_ptr<uint8_t[], Deleter> Release();

  // Copy the 8-bit ImageFrame into a contiguous, pre-allocated buffer. Note
  // that ImageFrame does not necessarily store its data contiguously (i.e. do
  // not use copy_n to move image data).
  void CopyToBuffer(uint8_t* buffer, int buffer_size) const;

  // A version of CopyToBuffer for 16-bit pixel data. Note that buffer_size
  // stores the number of 16-bit elements in the buffer, not the number of
  // bytes.
  void CopyToBuffer(uint16_t* buffer, int buffer_size) const;

  // A version of CopyToBuffer for float pixel data. Note that buffer_size
  // stores the number of float elements in the buffer, not the number of
  // bytes.
  void CopyToBuffer(float* buffer, int buffer_size) const;

  // Returns an error message which prints out the format encountered.
  static std::string InvalidFormatString(ImageFormat::Format format);

 private:
  // Returns true if alignment_number is 1 or a power of 2.
  static bool IsValidAlignmentNumber(uint32_t alignment_boundary);

  // The internal implementation of copying data from the provided pixel data.
  // If width_step is 0, then calculates width_step assuming no padding.
  void InternalCopyFrom(int width, int height, int width_step, int channel_size,
                        const uint8_t* pixel_data);

  // The internal implementation of copying data to the provided buffer.
  // If width_step is 0, then calculates width_step assuming no padding.
  void InternalCopyToBuffer(int width_step, char* buffer) const;

  ImageFormat::Format format_;
  int width_;
  int height_;
  int width_step_;

  std::unique_ptr<uint8_t[], Deleter> pixel_data_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_FRAME_H_
