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
// A container for location data, representing location information in an image.
// This wrapper provides two functionalities:
//  1. Factory methods for creation of Location objects and thus LocationData
//     protocol buffers. These methods guarantee a valid location data and are
//     the prefer way of creating such.
//  2. Accessors which allow for extracting location information in various
//     formats. If necessary, the location information is converted.

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_LOCATION_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_LOCATION_H_

#include <memory>

#include "mediapipe/framework/formats/location_data.pb.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/point2.h"
#include "mediapipe/framework/port/rectangle.h"

namespace mediapipe {
class BoundingBox;
}  // namespace mediapipe

namespace mediapipe {

class Location {
 public:
  // CREATION METHODS.
  Location();
  // Constructs a location wrapping the specified location data. Checks the
  // validity of the input and crashes upon failure.
  explicit Location(const LocationData& location_data);
  // Creates a location of type GLOBAL, i.e. a location representing the full
  // image.
  static Location CreateGlobalLocation();
  // Creates a location of type BOUNDING_BOX, i.e. it is based on a bounding box
  // defined by its upper left corner (xmin, ymin) and its width and height.
  static Location CreateBBoxLocation(int xmin, int ymin, int width, int height);
  // Creates a location of type BOUNDING_BOX from bounding boxes in various
  // formats.
  static Location CreateBBoxLocation(const Rectangle_i& rect);
  static Location CreateBBoxLocation(const ::mediapipe::BoundingBox& bbox);
  // Creates a location of type RELATIVE_BOUNDING_BOX, i.e. it is based on a
  // bounding box defined by its upper left corner (xmin, ymin) and its width
  // and height, all relative to the image dimensions.
  static Location CreateRelativeBBoxLocation(float relative_xmin,
                                             float relative_ymin,
                                             float relative_width,
                                             float relative_height);
  // Creates a location of type RELATIVE_BOUNDING_BOX from bounding boxes in
  // various formats.
  static Location CreateRelativeBBoxLocation(const Rectangle_f& relative_rect);

  // Returns the location type describing the type of data it contains. This
  // type is set at creation time based on the one of the above factory methods.
  LocationData::Format GetFormat() const;
  // Checks the validity of the specified location_data, i.e. whether all the
  // necessary fields for the location data type are set.
  static bool IsValidLocationData(const LocationData& location_data);

  // MODIFIERS
  // Scales the current bounding box (x,y,width,height) by the "scale" amount
  // (1.0f means no scale, <1.0f means scale down, and >1.0f means scale up).
  // Returns *this.
  //
  // NOTE: it does not handle masks.
  Location& Scale(float scale);

  // Resizes the location such that it is the tighest square location containing
  // centered the original location. It supports locations of type GLOBAL,
  // BOUNDING_BOX and RELATIVE_BOUNDING_BOX, otherwise it CHECK-fails. The user
  // must specify the image dimensions. Returns *this.
  Location& Square(int image_width, int image_height);

  // If the location is larger than the image, then in some cases it is
  // beneficial to translate the location such that the image is centered
  // within the location. The following method achieves this translation.
  // Returns *this.
  Location& ShiftToFitBestIntoImage(int image_width, int image_height);

  // Replaces the location with the intersection of the current location the
  // specified crop rectangle. Locations of type GLOBAL remain
  // unmodified. The image size is used for locations of type
  // BOUNDARY_BOX and MASK. Using this override for locations of type
  // RELATIVE_BOUNDING_BOX is an error. Use the Rectangle_f override of this
  // function instead.
  // This operation is useful when one needs to make sure that the location is
  // fully contained within the specified image. Returns *this.
  Location& Crop(const Rectangle_i& crop_rectangle);

  // Replaces the location with the intersection of the current location and the
  // specified crop rectangle. Locations of type GLOBAL remain
  // unmodified. This override is only for locations of type
  // RELATIVE_BOUNDING_BOX. Use the Rectangle_i override of this
  // function for other types of locations.
  // This operation is useful when one needs to make sure that the location is
  // fully contained within the specified image. Returns *this.
  Location& Crop(const Rectangle_f& crop_rectangle);

  // ACCESSORS.
  // Non-type converting accessor: returns the requested data only if the output
  // format is consistent with the location data type. E.g. if one requests a
  // rectangle, then the wrapped location data should be of type BOUNDING_BOX.
  // Accessor for location data type BOUNDING_BOX with two possible return types
  // Rectangle_i and mediapipe.::mediapipe::BoundingBox.
  template <typename T>
  T GetBBox() const;
  // Accessor for location data type RELATIVE_BOUNDING_BOX.
  Rectangle_f GetRelativeBBox() const;

  // Accessor for relative_keypoints in location data. Relative keypoints are
  // specified with x and y coordinates, where both x and y are relative to the
  // image width and height, respectively, and are in the range [0, 1]. Fails if
  // location data is not of type RELATIVE_BOUNDING_BOX.
  std::vector<Point2_f> GetRelativeKeypoints() const;

  // Type converting accessor: returns the requested data in the specified
  // output type. If the location data is in a format not directly convertible
  // to the specified return type the following conversion principles are used:
  //   - Rectangle -> Mask: the rectangle is converted to a mask with all
  //      pixels inside the rectangle being foreground pixels.
  //   - Mask -> Rectangle: the tightest enclosing rectangle to the mask is the
  //      rectangle representation of a mask.
  //   - Global -> Rectangle, Mask: all the pixels in the image are considered
  //      foreground. Thus, the equivalent rectangle and mask are a rectangle
  //      covering the full image.
  // The supported output types are the same as for GetBBox() above: Rectangle_i
  // and mediapipe.::mediapipe::BoundingBox.
  template <typename T>
  T ConvertToBBox(int image_width, int image_height) const;
  Rectangle_f ConvertToRelativeBBox(int image_width, int image_height) const;
  // Returns keypoints in absolute pixel coordinates.
  std::vector<Point2_i> ConvertToKeypoints(int image_width,
                                           int image_height) const;

  // Sets the keypoints.
  void SetRelativeKeypoints(const std::vector<Point2_f>& keypoints);

  // Serializes and deserializes the location object.
  void ConvertToProto(LocationData* proto) const;
  LocationData ConvertToProto() const;
  void SetFromProto(const LocationData& proto);

#if !defined(MEDIAPIPE_MOBILE) && !defined(MEDIAPIPE_LITE)
  // Deep equality comparison.
  bool operator==(const Location& other) const;
  bool operator!=(const Location& other) const;
#endif  // !defined(MEDIAPIPE_MOBILE) && !defined(MEDIAPIPE_LITE)

 private:
  // The wrapped location data.
  LocationData location_data_;
};
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_LOCATION_H_
