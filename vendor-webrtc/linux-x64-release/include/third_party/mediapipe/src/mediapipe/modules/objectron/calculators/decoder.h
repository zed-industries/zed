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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_DECODER_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_DECODER_H_

#include <vector>

#include "Eigen/Dense"
#include "absl/status/status.h"
#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/modules/objectron/calculators/annotation_data.pb.h"
#include "mediapipe/modules/objectron/calculators/belief_decoder_config.pb.h"

namespace mediapipe {

// Decodes 3D bounding box from heatmaps and offset maps. In the future,
// if we want to develop decoder for generic skeleton, then we need to
// generalize this class, and make a few child classes.
class Decoder {
 public:
  static const int kNumOffsetmaps;

  explicit Decoder(const BeliefDecoderConfig& config) : config_(config) {
    epnp_alpha_ << 4.0f, -1.0f, -1.0f, -1.0f, 2.0f, -1.0f, -1.0f, 1.0f, 2.0f,
        -1.0f, 1.0f, -1.0f, 0.0f, -1.0f, 1.0f, 1.0f, 2.0f, 1.0f, -1.0f, -1.0f,
        0.0f, 1.0f, -1.0f, 1.0f, 0.0f, 1.0f, 1.0f, -1.0f, -2.0f, 1.0f, 1.0f,
        1.0f;
  }

  // Decodes bounding boxes from predicted heatmap and offset maps.
  // Input:
  //   heatmap: a single channel cv::Mat representing center point heatmap
  //   offsetmap: a 16 channel cv::Mat representing the 16 offset maps
  //              (2 for each of the 8 vertices)
  // Output:
  //   Outputs 3D bounding boxes 2D vertices, represented by 'point_2d' field
  //   in each 'keypoints' field of object annotations.
  FrameAnnotation DecodeBoundingBoxKeypoints(const cv::Mat& heatmap,
                                             const cv::Mat& offsetmap) const;

  // Lifts the estimated 2D projections of bounding box vertices to 3D.
  // This function uses the EPnP approach described in this paper:
  // https://icwww.epfl.ch/~lepetit/papers/lepetit_ijcv08.pdf .
  // Input:
  //   projection_matrix: the projection matrix from 3D coordinate
  //     to screen coordinate.
  //     The 2D screen coordinate is defined as: u is along the long
  //     edge of the device, pointing down; v is along the short edge
  //     of the device, pointing right.
  //   portrait: a boolen variable indicating whether our images are
  //     obtained in portrait orientation or not.
  //   estimated_box: annotation with point_2d field populated with
  //     2d vertices.
  // Output:
  //   estimated_box: annotation with point_3d field populated with
  //     3d vertices.
  absl::Status Lift2DTo3D(
      const Eigen::Matrix<float, 4, 4, Eigen::RowMajor>& projection_matrix,
      bool portrait, FrameAnnotation* estimated_box) const;

 private:
  struct BeliefBox {
    float belief;
    std::vector<std::pair<float, float>> box_2d;
  };

  std::vector<cv::Point> ExtractCenterKeypoints(
      const cv::Mat& center_heatmap) const;

  // Decodes 2D keypoints at the peak point.
  void DecodeByPeak(const cv::Mat& offsetmap, int center_x, int center_y,
                    float offset_scale_x, float offset_scale_y,
                    BeliefBox* box) const;

  // Decodes 2D keypoints by voting around the peak.
  void DecodeByVoting(const cv::Mat& heatmap, const cv::Mat& offsetmap,
                      int center_x, int center_y, float offset_scale_x,
                      float offset_scale_y, BeliefBox* box) const;

  // Returns true if it is a new box. Otherwise, it may replace an existing box
  // if the new box's belief is higher.
  bool IsNewBox(std::vector<BeliefBox>* boxes, BeliefBox* box) const;

  // Returns true if the two boxes are identical.
  bool IsIdentical(const BeliefBox& box_1, const BeliefBox& box_2) const;

  BeliefDecoderConfig config_;
  // Following equation (1) in this paper
  // https://icwww.epfl.ch/~lepetit/papers/lepetit_ijcv08.pdf,
  // this variable denotes the coefficients for the 4 control points
  // for each of the 8 3D box vertices.
  Eigen::Matrix<float, 8, 4, Eigen::RowMajor> epnp_alpha_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_DECODER_H_
