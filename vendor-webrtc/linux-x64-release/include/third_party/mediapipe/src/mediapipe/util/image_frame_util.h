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
// Helper functions for working with ImageFrames.
#ifndef MEDIAPIPE_UTIL_IMAGE_FRAME_UTIL_H_
#define MEDIAPIPE_UTIL_IMAGE_FRAME_UTIL_H_

#include <cstdint>
#include <string>

#include "absl/strings/string_view.h"
#include "mediapipe/framework/formats/image_format.pb.h"
#include "mediapipe/framework/port/opencv_imgproc_inc.h"
#include "mediapipe/framework/tool/status_util.h"

namespace mediapipe {
class ImageFrame;
class YUVImage;
}  // namespace mediapipe

namespace mediapipe {
namespace image_frame_util {
// Rescale an SRGB ImageFrame.  destination_frame will be Reset() by
// this function (i.e. it will be deleted and reallocated if it already
// contained data).  The rescaling is done in 16bit LinearRGB colorspace.
// TODO Implement for other formats.
void RescaleImageFrame(const ImageFrame& source_frame, const int width,
                       const int height, const int alignment_boundary,
                       const int open_cv_interpolation_algorithm,
                       ImageFrame* destination_frame);

// Rescale the source image to the destination.  Following OpenCV
// conventions, destination will be reallocated only if it isn't
// the correct width, height, and format (i.e. channel and depth).
// The rescaling is done in 16bit LinearRGB colorspace.
void RescaleSrgbImage(const cv::Mat& source, const int width, const int height,
                      const int open_cv_interpolation_algorithm,
                      cv::Mat* destination);

// Convert an SRGB ImageFrame to an I420 YUVImage.
void ImageFrameToYUVImage(const ImageFrame& image_frame, YUVImage* yuv_image);

// Convert an SRGB ImageFrame to a 420p NV12 YUVImage.
void ImageFrameToYUVNV12Image(const ImageFrame& image_frame,
                              YUVImage* yuv_nv12_image);

// Convert a YUVImage to an SRGB ImageFrame. If use_bt709 is set to false, this
// function will assume that the YUV is as defined in BT.601 (standard from the
// 1980s). Most content is using BT.709 (as of 2019), but it's likely that this
// will no longer the case in the future, when BT.2100 will likely be dominant.
// This function needs to be changed significantly once YUVImage starts
// supporting ICtCp.
void YUVImageToImageFrame(const YUVImage& yuv_image, ImageFrame* image_frame,
                          bool use_bt709 = false);

// Converts a YUV image to an image frame, based on the yuv_image.fourcc()
// format.  Fails if no format is provided.
void YUVImageToImageFrameFromFormat(const YUVImage& yuv_image,
                                    ImageFrame* image_frame);

// Convert sRGB values into MPEG YCbCr values.  Notice that MPEG YCbCr
// values use a smaller range of values than JPEG YCbCr.  The conversion
// values used are those from ITU-R BT.601 (which are the same as ITU-R
// BT.709).  The conversion values are taken from wikipedia and cross
// checked with other sources.
void SrgbToMpegYCbCr(const uint8_t r, const uint8_t g, const uint8_t b,  //
                     uint8_t* y, uint8_t* cb, uint8_t* cr);
// Convert MPEG YCbCr values into sRGB values.  See the SrgbToMpegYCbCr()
// for more notes.  Many MPEG YCbCr values do not correspond directly
// to an sRGB value.  If the value is invalid it will be clipped to the
// closest valid value on a per channel basis.
void MpegYCbCrToSrgb(const uint8_t y, const uint8_t cb, const uint8_t cr,  //
                     uint8_t* r, uint8_t* g, uint8_t* b);

// Conversion functions to and from srgb and linear RGB in 16 bits-per-pixel
// channel.
void SrgbToLinearRgb16(const cv::Mat& source, cv::Mat* destination);
void LinearRgb16ToSrgb(const cv::Mat& source, cv::Mat* destination);

}  // namespace image_frame_util
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_IMAGE_FRAME_UTIL_H_
