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

// A container for a dense optical flow field that provides convenient
// visualization and serialization. The flow field stores dx, dy displacement
// for each pixel in absolute pixel values.

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_MOTION_OPTICAL_FLOW_FIELD_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_MOTION_OPTICAL_FLOW_FIELD_H_

#include "absl/strings/string_view.h"
#include "mediapipe/framework/formats/location.h"
#include "mediapipe/framework/formats/motion/optical_flow_field_data.pb.h"
#include "mediapipe/framework/port/opencv_imgproc_inc.h"
#include "mediapipe/framework/port/status.h"
#include "tensorflow/core/framework/tensor.h"

namespace mediapipe {

class OpticalFlowField {
 public:
  OpticalFlowField() {}
  explicit OpticalFlowField(const cv::Mat_<cv::Point2f>& flow);
  OpticalFlowField(const OpticalFlowField&) = delete;
  OpticalFlowField& operator=(const OpticalFlowField&) = delete;

  int width() const { return flow_data_.cols; }
  int height() const { return flow_data_.rows; }

  // Returns the maximum magnitude of motion in the flow field, ignoring values
  // larger than 1e9, which are assumed to be outliers. This function can
  // return a value of 0.0f for a zero-motion flow field.
  float GetRobustMaximumMagnitude() const;

  // Computes the color wheel visualization of the flow field. Normalizes based
  // on the maximum magnitude in the field. Hue represents angle and saturation
  // corresponds to the relative magnitude. The resulting Mat is RGB.
  cv::Mat GetVisualization() const;

  // Computes the color wheel visualization as if the specified maximum
  // magnitude were the true maximum magnitude of the flow. Magnitudes outside
  // the allowed range will saturate for visualization. The actual stored flow
  // field is not affected. The resulting Mat is RGB.
  cv::Mat GetVisualizationSaturatedAt(float max_magnitude) const;

  // Allocates internal storage for optical flow field of specified size.
  // After calling this function flow_data() and mutable_flow_data() return
  // allocated cv::Mat's.
  void Allocate(int width, int height);

  // Resizes this flow field in place to [new_width, new_height]. Pixel
  // displacements are rescaled to correspond to pixels in the new image size.
  void Resize(int new_width, int new_height);

  // Returns the raw flow data.
  const cv::Mat& flow_data() const { return flow_data_; }
  cv::Mat& mutable_flow_data() { return flow_data_; }

  // Converts from a tensorflow H x W x 2 float Tensor. The internal storage
  // for the optical flow field is reallocated.
  void CopyFromTensor(const tensorflow::Tensor& tensor);

  // Converts to/from associated proto.
  void SetFromProto(const OpticalFlowFieldData& proto);
  void ConvertToProto(OpticalFlowFieldData* proto) const;

  // Propagates the point at (x,y) to the point at (new_x, new_y) based on the
  // computed flow. Uses bilinear interpolation to calculate flow values at
  // sub-pixel location. Returns false if x is not in [0, width-1] or y is not
  // in [0, height-1].
  bool FollowFlow(float x, float y, float* new_x, float* new_y) const;

  // Returns the (sub-)pixel correspondences implied by the flow field. The
  // returned cv::Mat has entries (x + dx, y + dy) at location (x, y).
  cv::Mat ConvertToCorrespondences() const;

  // Returns true if this OpticalFlowField has the same dimensions as other and
  // the values at every pixel are within the specified margin.
  bool AllWithinMargin(const OpticalFlowField& other, float margin) const;

  // Estimates occluded and disoccluded pixels between two frames based on a
  // forward-backward motion consistency check. An occluded pixel is detected
  // when following the flow to the opposite frame and then back again moves the
  // point more than the specified spatial_distance_threshold. Occluded pixels
  // in the first frame and disoccluded pixels in the second frame are marked
  // with non-zero values in the respective masks. Occluded or disoccluded may
  // be nullptr to skip the computation of the corresponding mask.
  // For a forward-backward consistency check that also considers the
  // difference in appearance of the corresponding pixels, see
  // FlowFollower::FollowFlowWithAllChecks().
  static void EstimateMotionConsistencyOcclusions(
      const OpticalFlowField& forward, const OpticalFlowField& backward,
      double spatial_distance_threshold, Location* occluded_mask,
      Location* disoccluded_mask);

 private:
  // If enforce_max_magnitude is false, max_magnitude will be reset to be
  // the actual maximum magnitude of the flow in this frame.
  cv::Mat GetVisualizationInternal(float max_magnitude,
                                   bool enforce_max_magnitude) const;

  // Computes flow at a valid sub-pixel location using bilinear interpolation.
  cv::Point2f InterpolatedFlowAt(float x, float y) const;

  // Returns a mask Location with non-zero values for pixels that fail a
  // forward-backward motion-consistency check. Pixels fail the check if
  // following flow into the next frame and back again moves a point by more
  // than the specified spatial_distance_threshold.
  static Location FindMotionInconsistentPixels(
      const OpticalFlowField& forward, const OpticalFlowField& backward,
      double spatial_distance_threshold);

  cv::Mat_<cv::Point2f> flow_data_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_MOTION_OPTICAL_FLOW_FIELD_H_
