// Copyright 2022 The MediaPipe Authors.
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
// A collection of functions operating on MediaPipe::Location that require
// OpenCV to either convert between formats, or apply OpenCV transformations.

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_LOCATION_OPENCV_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_LOCATION_OPENCV_H_
#include "mediapipe/framework/formats/location.h"
#include "mediapipe/framework/port/opencv_core_inc.h"

namespace mediapipe {
// Creates a location of type BOUNDING_BOX from an OpenCV rectangle.
Location CreateBBoxLocation(const cv::Rect& rect);

// Creates a location of type MASK from a single-channel uint8 or float
// cv::Mat_ (type is CV_8UC1 or CV_32FC1). Check fails if the mat is not
// single channel. Pixels with positive values are treated as the foreground.
template <typename T>
Location CreateCvMaskLocation(const cv::Mat_<T>& mask);

// Enlarges the location by the given factor. This operation keeps the center
// of the location fixed, while enlarging its dimensions by the given factor.
// Note that the location may partially lie outside the image after this
// operation.
void EnlargeLocation(Location& location, float factor);

// Same as Location::GetMask() with the difference that the return value is a
// cv::Mat of type CV_8UC1. Background pixels are set to 0 and foreground pixels
// are set to 255.
std::unique_ptr<cv::Mat> GetCvMask(const Location& location);

// Returns the provided location's RELATIVE_BOUNDING_BOX or MASK location
// data as an OpenCV Mat. If the location data is in a format not directly
// convertible to the specified return type the following conversion principles
// are used:
//   - Rectangle -> Mask: the rectangle is converted to a mask with all
//      pixels inside the rectangle being foreground pixels.
std::unique_ptr<cv::Mat> ConvertToCvMask(const Location& location,
                                         int image_width, int image_height);
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_LOCATION_OPENCV_H_
