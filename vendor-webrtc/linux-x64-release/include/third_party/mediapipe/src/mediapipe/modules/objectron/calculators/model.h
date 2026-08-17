// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_MODEL_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_MODEL_H_

#include "mediapipe/modules/objectron/calculators/annotation_data.pb.h"
#include "mediapipe/modules/objectron/calculators/object.pb.h"
#include "mediapipe/modules/objectron/calculators/types.h"

namespace mediapipe {

class Model {
 public:
  EIGEN_MAKE_ALIGNED_OPERATOR_NEW

  enum Type {
    kVisualizationOnly = 0,
    kBoundingBox,
    kSkeleton,
    kShape,  // Shape is a virtual object.
    kNumModes,
  };

  virtual ~Model() = default;

  virtual void SetTransformation(const Eigen::Matrix4f& transform);
  virtual void SetTranslation(const Eigen::Vector3f& translation);

  // Compute the rotation matrix from these angles and update the transformation
  // matrix accordingly
  virtual void SetRotation(float roll, float pitch, float yaw);
  virtual void SetRotation(const Eigen::Matrix3f& rotation);
  virtual void SetScale(const Eigen::Vector3f& scale);
  virtual void SetCategory(const std::string& category);
  virtual size_t GetNumberKeypoints() const { return number_keypoints_; }

  // Gets Euler angles in the order of roll, pitch, yaw.
  virtual const Eigen::Vector3f GetRotationAngles() const;
  virtual const Eigen::Matrix4f& GetTransformation() const;
  virtual const Eigen::Vector3f& GetScale() const;
  virtual const Eigen::Ref<const Eigen::Vector3f> GetTranslation() const;
  virtual const Eigen::Ref<const Eigen::Matrix3f> GetRotation() const;
  virtual const std::string& GetCategory() const;

  // Update the model's keypoints in the world-coordinate system.
  // The update includes transforming the model to the world-coordinate system
  // as well as scaling the model.
  // The user is expected to call this function after Setting the rotation,
  // orientation or the scale of the model to get an updated model.
  virtual void Update() = 0;

  // Update the model's parameters (orientation, position, and scale) from the
  // user-provided variables.
  virtual void Adjust(const std::vector<float>& variables) = 0;

  // Returns a pointer to the model's keypoints.
  // Use Eigen::Map to cast the pointer back to Vector3 or Vector4
  virtual const float* GetVertex(size_t id) const = 0;
  virtual float* GetVertex(size_t id) = 0;
  virtual void Deserialize(const Object& obj);
  virtual void Serialize(Object* obj);

  // TODO: make member variables protected, and add public apis.
  // 4x4 transformation matrix mapping the first keypoint to world coordinate
  Eigen::Matrix4f transformation_;
  Eigen::Vector3f scale_;  // width, height, depth
  Type model_type_;
  size_t number_keypoints_;
  std::string category_;

 protected:
  Model(Type type, size_t number_keypoints, const std::string& category)
      : model_type_(type),
        number_keypoints_(number_keypoints),
        category_(category) {}
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_MODEL_H_
