// Copyright 2021 The MediaPipe Authors.
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
// A container for affine transform data
// This wrapper provides two functionalities:
//  1. Factory methods for creation of Transform objects and thus
//     AffineTransformData protocol buffers. These methods guarantee a valid
//     affine transform data and are the preferred way of creating such.
//  2. Accessors which allow for access of the data and the convertion to proto
//     format

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_AFFINE_TRANSFORM_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_AFFINE_TRANSFORM_H_

#include <memory>
#include <vector>

#include "mediapipe/framework/formats/affine_transform_data.pb.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/point2.h"

namespace mediapipe {

class AffineTransform {
 public:
  // CREATION METHODS.
  AffineTransform();

  // Constructs a affine transform wrapping the specified affine transform data.
  // Checks the validity of the input and crashes upon failure.
  explicit AffineTransform(const AffineTransformData& transform_data);

  static AffineTransform Create(const Point2_f& translation = Point2_f(0, 0),
                                const Point2_f& scale = Point2_f(1, 1),
                                float rotation = 0,
                                const Point2_f& shear = Point2_f(0, 0));

  // ACCESSORS
  // Accessor for the composition matrix
  std::vector<float> GetCompositionMatrix();

  Point2_f GetScale() const;
  Point2_f GetTranslation() const;
  Point2_f GetShear() const;
  float GetRotation() const;

  void SetScale(const Point2_f& scale);
  void SetTranslation(const Point2_f& translation);
  void SetShear(const Point2_f& shear);
  void SetRotation(float rotation);

  void AddScale(const Point2_f& scale);
  void AddTranslation(const Point2_f& translation);
  void AddShear(const Point2_f& shear);
  void AddRotation(float rotation);

  // Serializes and deserializes the affine transform object.
  void ConvertToProto(AffineTransformData* proto) const;
  AffineTransformData ConvertToProto() const;
  void SetFromProto(const AffineTransformData& proto);

  bool Equals(const AffineTransform& other, float epsilon = 0.001f) const;

  static bool Equal(const AffineTransform& lhs, const AffineTransform& rhs,
                    float epsilon = 0.001f);

 private:
  // The wrapped transform data.
  AffineTransformData affine_transform_data_;
  std::vector<float> matrix_ = {1, 0, 0, 0, 1, 0, 0, 0, 1};
  bool is_dirty_ = false;
};
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_AFFINE_TRANSFORM_H_
